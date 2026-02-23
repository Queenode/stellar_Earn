use soroban_sdk::{contracttype, Address, Symbol, BytesN, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quest {
    pub id: Symbol,
    pub creator: Address,
    pub reward_asset: Address,
    pub reward_amount: i128,
    pub verifier: Address,
    pub deadline: u64,
    pub status: QuestStatus,
    pub total_claims: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Submission {
    pub quest_id: Symbol,
    pub submitter: Address,
    pub proof_hash: BytesN<32>,
    pub status: SubmissionStatus,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuestStatus {
    Active,
    Paused,
    Completed,
    Expired,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Rejected,
    Paid,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserStats {
    pub xp: u64,
    pub level: u32,
    pub quests_completed: u32,
    pub badges: Vec<Badge>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Badge {
    Rookie,
    Explorer,
    Veteran,
    Master,
    Legend,
}

//================================================================================
// Batch operation input types (gas-optimized multi-item operations)
//================================================================================

/// Single quest registration input for batch registration.
/// Creator is implied from auth in register_quests_batch.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchQuestInput {
    pub id: Symbol,
    pub reward_asset: Address,
    pub reward_amount: i128,
    pub verifier: Address,
    pub deadline: u64,
}

/// Single approval input for batch approval.
/// Verifier is implied from auth in approve_submissions_batch.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchApprovalInput {
    pub quest_id: Symbol,
    pub submitter: Address,
}
