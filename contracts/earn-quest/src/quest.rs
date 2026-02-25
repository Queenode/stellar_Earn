use crate::errors::Error;
use crate::events;
use crate::storage;
use crate::types::{BatchQuestInput, MetadataDescription, Quest, QuestMetadata, QuestStatus};
use crate::validation;
use soroban_sdk::{Address, Env, Symbol, Vec};

const MAX_METADATA_TITLE_LEN: u32 = 80;
const MAX_METADATA_CATEGORY_LEN: u32 = 40;
const MAX_METADATA_TAG_LEN: u32 = 32;
const MAX_METADATA_REQUIREMENT_LEN: u32 = 200;
const MAX_METADATA_INLINE_DESCRIPTION_LEN: u32 = 1200;
const MAX_METADATA_TAGS: u32 = 15;
const MAX_METADATA_REQUIREMENTS: u32 = 20;

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

/// Register a new quest and store metadata in the same transaction.
pub fn register_quest_with_metadata(
    env: &Env,
    id: &Symbol,
    creator: &Address,
    reward_asset: &Address,
    reward_amount: i128,
    verifier: &Address,
    deadline: u64,
    metadata: &QuestMetadata,
) -> Result<(), Error> {
    register_quest(
        env,
        id,
        creator,
        reward_asset,
        reward_amount,
        verifier,
        deadline,
    )?;
    validate_metadata(metadata)?;
    storage::set_quest_metadata(env, id, metadata);
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

/// Update metadata for an existing quest.
/// Only the quest creator or an admin can update metadata.
pub fn update_quest_metadata(
    env: &Env,
    quest_id: &Symbol,
    updater: &Address,
    metadata: &QuestMetadata,
) -> Result<(), Error> {
    let quest = storage::get_quest(env, quest_id)?;
    if &quest.creator != updater && !storage::is_admin(env, updater) {
        return Err(Error::Unauthorized);
    }

    validate_metadata(metadata)?;
    storage::set_quest_metadata(env, quest_id, metadata);
    Ok(())
}

fn validate_metadata(metadata: &QuestMetadata) -> Result<(), Error> {
    validate_string_len(&metadata.title, MAX_METADATA_TITLE_LEN)?;
    validate_string_len(&metadata.category, MAX_METADATA_CATEGORY_LEN)?;

    validation::validate_array_length(metadata.tags.len(), MAX_METADATA_TAGS)?;
    for i in 0..metadata.tags.len() {
        validate_string_len(&metadata.tags.get(i).unwrap(), MAX_METADATA_TAG_LEN)?;
    }

    validation::validate_array_length(metadata.requirements.len(), MAX_METADATA_REQUIREMENTS)?;
    for i in 0..metadata.requirements.len() {
        validate_string_len(
            &metadata.requirements.get(i).unwrap(),
            MAX_METADATA_REQUIREMENT_LEN,
        )?;
    }

    if let MetadataDescription::Inline(desc) = &metadata.description {
        validate_string_len(desc, MAX_METADATA_INLINE_DESCRIPTION_LEN)?;
    }

    Ok(())
}

fn validate_string_len(value: &soroban_sdk::String, max: u32) -> Result<(), Error> {
    if value.len() > max {
        return Err(Error::StringTooLong);
    }
    Ok(())
}
