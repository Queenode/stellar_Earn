#![no_std]

mod admin;
pub mod errors;
mod escrow;
mod events;
mod payout;
mod quest;
mod reputation;
mod security;
pub mod storage;
mod submission;
pub mod types;
pub mod validation;

use crate::errors::Error;
use crate::types::{
    Badge, BatchApprovalInput, BatchQuestInput, EscrowInfo, QuestMetadata, UserStats,
};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, Vec};
mod init;


#[contract]
pub struct EarnQuestContract;

#[contractimpl]
impl EarnQuestContract {
    /// Initialize contract with admin, version, and config
    pub fn initialize(
        env: Env,
        admin: Address,
        version: u32,
        config_params: Vec<(String, String)>
    ) -> Result<(), Error> {
        admin.require_auth();
        if storage::is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }
        let config = init::InitConfig {
            admin: admin.clone(),
            version,
            config_params,
        };
        init::initialize(&env, config);
        Ok(())
    }

    /// Authorize contract upgrade (admin only)
    pub fn authorize_upgrade(env: Env, caller: Address) -> Result<(), Error> {
        caller.require_auth();
        if !init::upgrade_authorize(&env, &caller) {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    /// Get contract version
    pub fn get_version(env: Env) -> u32 {
        storage::get_version(&env)
    }

    /// Get contract admin
    pub fn get_admin(env: Env) -> Address {
        storage::get_admin(&env)
    }

    /// Get contract config
    pub fn get_config(env: Env) -> Vec<(String, String)> {
        storage::get_config(&env)
    }

    /// Add a new admin (admin only)
    pub fn add_admin(env: Env, caller: Address, new_admin: Address) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        admin::add_admin(&env, &caller, &new_admin)
    }

    /// Remove an admin (admin only)
    pub fn remove_admin(env: Env, caller: Address, admin_to_remove: Address) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        admin::remove_admin(&env, &caller, &admin_to_remove)
    }

    /// Check if an address is an admin
    pub fn is_admin(env: Env, address: Address) -> bool {
        admin::is_admin(&env, &address)
    }

    /// Register a new quest with full input validation
    pub fn register_quest(
        env: Env,
        id: Symbol,
        creator: Address,
        reward_asset: Address,
        reward_amount: i128,
        verifier: Address,
        deadline: u64,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        creator.require_auth();

        quest::register_quest(
            &env,
            &id,
            &creator,
            &reward_asset,
            reward_amount,
            &verifier,
            deadline,
        )
    }

    /// Register a new quest and attach metadata during creation.
    pub fn register_quest_with_metadata(
        env: Env,
        id: Symbol,
        creator: Address,
        reward_asset: Address,
        reward_amount: i128,
        verifier: Address,
        deadline: u64,
        metadata: QuestMetadata,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        creator.require_auth();

        quest::register_quest_with_metadata(
            &env,
            &id,
            &creator,
            &reward_asset,
            reward_amount,
            &verifier,
            deadline,
            &metadata,
        )
    }

    /// Register multiple quests in one transaction (gas-optimized).
    /// Creator must authorize; all quests are registered to that creator.
    /// Batch size is limited; on first error the entire batch reverts.
    pub fn register_quests_batch(
        env: Env,
        creator: Address,
        quests: Vec<BatchQuestInput>,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        creator.require_auth();

        quest::register_quests_batch(&env, &creator, &quests)
    }

    /// Submit proof with input validation
    pub fn submit_proof(
        env: Env,
        quest_id: Symbol,
        submitter: Address,
        proof_hash: BytesN<32>,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        submitter.require_auth();

        submission::submit_proof(&env, &quest_id, &submitter, &proof_hash)
    }

    /// Approve submission with status transition validation
    pub fn approve_submission(
        env: Env,
        quest_id: Symbol,
        submitter: Address,
        verifier: Address,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        verifier.require_auth();

        submission::approve_submission(&env, &quest_id, &submitter, &verifier)
    }

    /// Approve multiple submissions in one transaction (gas-optimized).
    /// Verifier must authorize; all items must be for quests where this address is verifier.
    /// Batch size is limited; on first error the entire batch reverts.
    pub fn approve_submissions_batch(
        env: Env,
        verifier: Address,
        submissions: Vec<BatchApprovalInput>,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        verifier.require_auth();

        submission::approve_submissions_batch(&env, &verifier, &submissions)
    }

    /// Claim approved reward with full validation
    pub fn claim_reward(env: Env, quest_id: Symbol, submitter: Address) -> Result<(), Error> {
        // 1. Auth
        security::require_not_paused(&env)?;
        submitter.require_auth();

        // 2. Validate claim (status transitions, limits)
        submission::validate_claim(&env, &quest_id, &submitter)?;

        // 3. Data Retrieval for payout
        let quest = storage::get_quest(&env, &quest_id)?;

        // 4. Payout
        payout::transfer_reward_from_escrow(
            &env,
            &quest_id,
            &quest.reward_asset,
            &submitter,
            quest.reward_amount,
        )?;
        // 5. State Update
        storage::update_submission_status(
            &env,
            &quest_id,
            &submitter,
            types::SubmissionStatus::Paid,
        )?;
        storage::increment_quest_claims(&env, &quest_id)?;

        // EMIT EVENT: RewardClaimed
        events::reward_claimed(
            &env,
            quest_id.clone(),
            submitter.clone(),
            quest.reward_asset,
            quest.reward_amount,
        );

        // 6. Award XP for quest completion
        reputation::award_xp(&env, &submitter, 100)?;

        Ok(())
    }

    /// Get user reputation stats
    pub fn get_user_stats(env: Env, user: Address) -> UserStats {
        reputation::get_user_stats(&env, &user)
    }

    /// Grant a badge to a user (admin only) with array length validation
    pub fn grant_badge(env: Env, admin: Address, user: Address, badge: Badge) -> Result<(), Error> {
        security::require_not_paused(&env)?;

        // Validate badge count before granting
        let stats = storage::get_user_stats_or_default(&env, &user);
        validation::validate_badge_count(stats.badges.len())?;

        reputation::grant_badge(&env, &admin, &user, badge)
    }

    /// Emergency: pause contract immediately (admin only)
    pub fn emergency_pause(env: Env, caller: Address) -> Result<(), Error> {
        security::emergency_pause(&env, &caller)
    }

    /// Emergency: approve unpause (admin multisig approval)
    pub fn emergency_approve_unpause(env: Env, caller: Address) -> Result<(), Error> {
        security::emergency_approve_unpause(&env, &caller)
    }

    /// Emergency: unpause contract after approvals and timelock
    pub fn emergency_unpause(env: Env, caller: Address) -> Result<(), Error> {
        security::emergency_unpause(&env, &caller)
    }

    /// Emergency withdrawal when paused (admin only) with amount validation
    pub fn emergency_withdraw(
        env: Env,
        caller: Address,
        asset: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        // Validate withdraw amount range
        validation::validate_reward_amount(amount)?;
        security::emergency_withdraw(&env, &caller, &asset, &to, amount)
    }

    /// Deposit tokens into escrow for a quest.
    ///
    /// The creator sends tokens to the contract, earmarked for this quest.
    /// Can be called multiple times to add more funds (top-up).
    ///
    /// # Who can call: Quest creator only
    /// # Token flow: Creator wallet → Contract
    pub fn deposit_escrow(
        env: Env,
        quest_id: Symbol,
        depositor: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        depositor.require_auth();
        escrow::deposit(&env, &quest_id, &depositor, &token, amount)
    }

    /// Cancel a quest and refund remaining escrow to creator.
    ///
    /// # Who can call: Quest creator only
    /// # Requires: Quest is Active or Paused
    /// # Token flow: Contract → Creator wallet (remaining balance)
    /// # Returns: Amount refunded
    pub fn cancel_quest(env: Env, quest_id: Symbol, creator: Address) -> Result<i128, Error> {
        security::require_not_paused(&env)?;
        creator.require_auth();
        escrow::cancel_quest(&env, &quest_id, &creator)
    }

    /// Withdraw unclaimed escrow from a finished quest.
    ///
    /// # Who can call: Quest creator only
    /// # Requires: Quest is Completed, Expired, or Cancelled
    /// # Token flow: Contract → Creator wallet (remaining balance)
    /// # Returns: Amount withdrawn
    pub fn withdraw_unclaimed(env: Env, quest_id: Symbol, creator: Address) -> Result<i128, Error> {
        security::require_not_paused(&env)?;
        creator.require_auth();
        escrow::withdraw_unclaimed(&env, &quest_id, &creator)
    }

    /// Expire a quest whose deadline has passed and refund remaining escrow.
    ///
    /// # Who can call: Quest creator only
    /// # Requires: Quest is Active or Paused AND deadline has passed
    /// # Token flow: Contract → Creator wallet (remaining balance)
    /// # Returns: Amount refunded
    pub fn expire_quest(env: Env, quest_id: Symbol, creator: Address) -> Result<i128, Error> {
        security::require_not_paused(&env)?;
        creator.require_auth();
        escrow::expire_quest(&env, &quest_id, &creator)
    /// Update quest metadata (quest creator or admin).
    pub fn update_quest_metadata(
        env: Env,
        quest_id: Symbol,
        updater: Address,
        metadata: QuestMetadata,
    ) -> Result<(), Error> {
        security::require_not_paused(&env)?;
        updater.require_auth();
        quest::update_quest_metadata(&env, &quest_id, &updater, &metadata)
    }

    /// Query quest metadata.
    pub fn get_quest_metadata(env: Env, quest_id: Symbol) -> Result<QuestMetadata, Error> {
        storage::get_quest_metadata(&env, &quest_id)
    }

    /// Check whether metadata exists for a quest.
    pub fn has_quest_metadata(env: Env, quest_id: Symbol) -> bool {
        storage::has_quest_metadata(&env, &quest_id)
    }

    /// Query the available escrow balance for a quest.
    pub fn get_escrow_balance(env: Env, quest_id: Symbol) -> Result<i128, Error> {
        escrow::get_balance(&env, &quest_id)
    }

    /// Query the full escrow info for a quest.
    pub fn get_escrow_info(env: Env, quest_id: Symbol) -> Result<EscrowInfo, Error> {
        escrow::get_info(&env, &quest_id)
    }

    /// Admin: set unpause approvals threshold
    pub fn set_unpause_threshold(env: Env, caller: Address, threshold: u32) -> Result<(), Error> {
        security::set_unpause_threshold(&env, &caller, threshold)
    }

    /// Admin: set unpause timelock seconds
    pub fn set_unpause_timelock(env: Env, caller: Address, seconds: u64) -> Result<(), Error> {
        security::set_unpause_timelock(&env, &caller, seconds)
    }
}
