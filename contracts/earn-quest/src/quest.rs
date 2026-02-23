use crate::errors::Error;
use crate::events;
use crate::storage;
use crate::types::{BatchQuestInput, Quest, QuestStatus};
use crate::validation;
use soroban_sdk::{Address, Env, Symbol, Vec};

/// Register a new quest with full input validation.
///
/// Validates:
/// - Quest does not already exist
/// - Reward amount is within valid range (> 0 and <= MAX)
/// - Deadline is in the future
/// - Creator and verifier are distinct addresses
pub fn register_quest(
    env: &Env,
    id: &Symbol,
    creator: &Address,
    reward_asset: &Address,
    reward_amount: i128,
    verifier: &Address,
    deadline: u64,
) -> Result<(), Error> {
    // Validate quest ID symbol length
    validation::validate_symbol_length(id)?;

    // Check quest doesn't already exist
    if storage::has_quest(env, id) {
        return Err(Error::QuestAlreadyExists);
    }

    // Validate reward amount range (min/max)
    validation::validate_reward_amount(reward_amount)?;

    // Validate deadline is in the future
    validation::validate_deadline(env, deadline)?;

    // Validate creator and verifier are different addresses
    validation::validate_addresses_distinct(creator, verifier)?;

    let quest = Quest {
        id: id.clone(),
        creator: creator.clone(),
        reward_asset: reward_asset.clone(),
        reward_amount,
        verifier: verifier.clone(),
        deadline,
        status: QuestStatus::Active,
        total_claims: 0,
    };

    storage::set_quest(env, id, &quest);

    // EMIT EVENT: QuestRegistered
    events::quest_registered(
        env,
        id.clone(),
        creator.clone(),
        reward_asset.clone(),
        reward_amount,
        verifier.clone(),
        deadline,
    );

    Ok(())
}

//================================================================================
// Batch registration (gas-optimized)
//================================================================================

/// Register multiple quests in a single transaction.
///
/// Validates batch size, then processes each item in order. On first validation
/// or storage error, the entire batch is reverted (no partial state). Events are
/// emitted for each successfully processed quest before the next is applied.
///
/// # Arguments
/// * `env` - Contract environment
/// * `creator` - Must match auth; creator for all quests in the batch
/// * `quests` - List of quest inputs (id, reward_asset, reward_amount, verifier, deadline)
///
/// # Returns
/// * `Ok(())` if all quests were registered
/// * `Err(Error)` on first failure (e.g. QuestAlreadyExists, ArrayTooLong)
pub fn register_quests_batch(
    env: &Env,
    creator: &Address,
    quests: &Vec<BatchQuestInput>,
) -> Result<(), Error> {
    let len = quests.len();
    validation::validate_batch_quest_size(len)?;

    for i in 0u32..len {
        let q = quests.get(i).unwrap();
        register_quest(
            env,
            &q.id,
            creator,
            &q.reward_asset,
            q.reward_amount,
            &q.verifier,
            q.deadline,
        )?;
    }

    Ok(())
}
