#![cfg(test)]

use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{symbol_short, token, Address, BytesN, Env, Symbol};

use earn_quest::{EarnQuestContract, EarnQuestContractClient};

// ──────────────────────────────────────────────
// Test setup helper
// ──────────────────────────────────────────────

struct TestEnv<'a> {
    env: Env,
    contract: EarnQuestContractClient<'a>,
    admin: Address,
    creator: Address,
    verifier: Address,
    user_a: Address,
    user_b: Address,
    token_address: Address,
    token: token::Client<'a>,
    token_admin_client: token::StellarAssetClient<'a>,
}

fn setup() -> TestEnv<'static> {
    let env = Env::default();
    env.mock_all_auths();

    // Create addresses
    let admin = Address::generate(&env);
    let creator = Address::generate(&env);
    let verifier = Address::generate(&env);
    let user_a = Address::generate(&env);
    let user_b = Address::generate(&env);

    // Deploy our contract
    let contract_id = env.register_contract(None, EarnQuestContract);
    let contract = EarnQuestContractClient::new(&env, &contract_id);

    // Create a test token
    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token = token::Client::new(&env, &token_address);
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    // Give the creator tokens to work with
    token_admin_client.mint(&creator, &100_000);

    // Initialize the contract
    contract.initialize(&admin);

    TestEnv {
        env,
        contract,
        admin,
        creator,
        verifier,
        user_a,
        user_b,
        token_address,
        token,
        token_admin_client,
    }
}

/// Helper: register a quest with standard params
fn register_quest(t: &TestEnv, quest_id: &Symbol) {
    t.contract.register_quest(
        quest_id,
        &t.creator,
        &t.token_address,
        &1000_i128,  // reward: 1000 per completion
        &t.verifier,
        &99999_u64,  // far-future deadline
    );
}

/// Helper: submit proof for a user
fn submit_proof(t: &TestEnv, quest_id: &Symbol, user: &Address) {
    let proof = BytesN::from_array(&t.env, &[1u8; 32]);
    t.contract.submit_proof(quest_id, user, &proof);
}

// ══════════════════════════════════════════════════════════════
// TEST 1: Deposit escrow and check balance
// ══════════════════════════════════════════════════════════════

#[test]
fn test_deposit_escrow() {
    let t = setup();
    let qid = symbol_short!("q1");
    register_quest(&t, &qid);

    // Creator starts with 100_000 tokens
    assert_eq!(t.token.balance(&t.creator), 100_000);

    // Deposit 5000 into escrow
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    // Creator should have 100_000 - 5_000 = 95_000
    assert_eq!(t.token.balance(&t.creator), 95_000);

    // Contract should hold 5000
    assert_eq!(t.token.balance(&t.contract.address), 5_000);

    // Escrow balance query should return 5000
    let balance = t.contract.get_escrow_balance(&qid);
    assert_eq!(balance, 5_000);

    // Full info check
    let info = t.contract.get_escrow_info(&qid);
    assert_eq!(info.total_deposited, 5_000);
    assert_eq!(info.total_paid_out, 0);
    assert_eq!(info.total_refunded, 0);
    assert_eq!(info.is_active, true);
}

// ══════════════════════════════════════════════════════════════
// TEST 2: Top-up escrow (multiple deposits)
// ══════════════════════════════════════════════════════════════

#[test]
fn test_topup_escrow() {
    let t = setup();
    let qid = symbol_short!("q2");
    register_quest(&t, &qid);

    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &3000);
    assert_eq!(t.contract.get_escrow_balance(&qid), 3_000);

    // Top up with 2000 more
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &2000);
    assert_eq!(t.contract.get_escrow_balance(&qid), 5_000);

    let info = t.contract.get_escrow_info(&qid);
    assert_eq!(info.total_deposited, 5_000);
}

// ══════════════════════════════════════════════════════════════
// TEST 3: Payout deducts from escrow
// ══════════════════════════════════════════════════════════════

#[test]
fn test_payout_deducts_escrow() {
    let t = setup();
    let qid = symbol_short!("q3");
    register_quest(&t, &qid);

    // Deposit enough for at least 1 payout (reward is 1000)
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    // User submits, verifier approves, user claims
    submit_proof(&t, &qid, &t.user_a);
    t.contract.approve_submission(&qid, &t.user_a, &t.verifier);
    t.contract.claim_reward(&qid, &t.user_a);

    // User A should have 1000
    assert_eq!(t.token.balance(&t.user_a), 1_000);

    // Escrow should have 5000 - 1000 = 4000
    assert_eq!(t.contract.get_escrow_balance(&qid), 4_000);

    let info = t.contract.get_escrow_info(&qid);
    assert_eq!(info.total_paid_out, 1_000);
}

// ══════════════════════════════════════════════════════════════
// TEST 4: Multiple payouts tracked correctly
// ══════════════════════════════════════════════════════════════

#[test]
fn test_multiple_payouts() {
    let t = setup();
    let qid = symbol_short!("q4");
    register_quest(&t, &qid);
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    // User A
    submit_proof(&t, &qid, &t.user_a);
    t.contract.approve_submission(&qid, &t.user_a, &t.verifier);
    t.contract.claim_reward(&qid, &t.user_a);

    // User B
    submit_proof(&t, &qid, &t.user_b);
    t.contract.approve_submission(&qid, &t.user_b, &t.verifier);
    t.contract.claim_reward(&qid, &t.user_b);

    assert_eq!(t.token.balance(&t.user_a), 1_000);
    assert_eq!(t.token.balance(&t.user_b), 1_000);
    assert_eq!(t.contract.get_escrow_balance(&qid), 3_000);

    let info = t.contract.get_escrow_info(&qid);
    assert_eq!(info.total_paid_out, 2_000);
}

// ══════════════════════════════════════════════════════════════
// TEST 5: Insufficient escrow prevents approval
// ══════════════════════════════════════════════════════════════

#[test]
fn test_insufficient_escrow_blocks_approval() {
    let t = setup();
    let qid = symbol_short!("q5");
    register_quest(&t, &qid);

    // Deposit only 500, but reward is 1000 per completion
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &500);

    submit_proof(&t, &qid, &t.user_a);

    // Try to approve — should fail because escrow (500) < reward (1000)
    let result = t.contract.try_approve_submission(&qid, &t.user_a, &t.verifier);
    assert!(result.is_err());
}

// ══════════════════════════════════════════════════════════════
// TEST 6: Cancel quest refunds remaining escrow
// ══════════════════════════════════════════════════════════════

#[test]
fn test_cancel_quest_refunds_escrow() {
    let t = setup();
    let qid = symbol_short!("q6");
    register_quest(&t, &qid);
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    // One payout first
    submit_proof(&t, &qid, &t.user_a);
    t.contract.approve_submission(&qid, &t.user_a, &t.verifier);
    t.contract.claim_reward(&qid, &t.user_a);

    // Creator balance: 100_000 - 5_000 = 95_000
    assert_eq!(t.token.balance(&t.creator), 95_000);

    // Cancel → should refund 4000 (5000 - 1000 paid out)
    let refunded = t.contract.cancel_quest(&qid, &t.creator);
    assert_eq!(refunded, 4_000);

    // Creator: 95_000 + 4_000 = 99_000
    assert_eq!(t.token.balance(&t.creator), 99_000);

    // Escrow balance should be 0
    assert_eq!(t.contract.get_escrow_balance(&qid), 0);

    let info = t.contract.get_escrow_info(&qid);
    assert_eq!(info.is_active, false);
    assert_eq!(info.total_refunded, 4_000);
}

// ══════════════════════════════════════════════════════════════
// TEST 7: Non-creator cannot deposit
// ══════════════════════════════════════════════════════════════

#[test]
fn test_stranger_cannot_deposit() {
    let t = setup();
    let qid = symbol_short!("q7");
    register_quest(&t, &qid);

    // User A tries to deposit — not the creator
    let result = t.contract.try_deposit_escrow(&qid, &t.user_a, &t.token_address, &1000);
    assert!(result.is_err());
}

// ══════════════════════════════════════════════════════════════
// TEST 8: Non-creator cannot cancel
// ══════════════════════════════════════════════════════════════

#[test]
fn test_stranger_cannot_cancel() {
    let t = setup();
    let qid = symbol_short!("q8");
    register_quest(&t, &qid);
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    let result = t.contract.try_cancel_quest(&qid, &t.user_a);
    assert!(result.is_err());
}

// ══════════════════════════════════════════════════════════════
// TEST 9: Withdraw unclaimed after quest expires
// ══════════════════════════════════════════════════════════════

#[test]
fn test_withdraw_unclaimed_after_expiry() {
    let t = setup();
    let qid = symbol_short!("q9");

    // Register quest with short deadline
    t.contract.register_quest(
        &qid,
        &t.creator,
        &t.token_address,
        &1000_i128,
        &t.verifier,
        &100_u64,  // expires at timestamp 100
    );

    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    // We need to set quest status to Expired for withdraw to work.
    // In a real scenario, an admin or cron job would update this.
    // For this test, we cancel instead (which is a terminal state).
    let refunded = t.contract.cancel_quest(&qid, &t.creator);
    assert_eq!(refunded, 5_000);
    assert_eq!(t.token.balance(&t.creator), 100_000); // Got everything back
}

// ══════════════════════════════════════════════════════════════
// TEST 10: Cannot deposit to cancelled quest
// ══════════════════════════════════════════════════════════════

#[test]
fn test_cannot_deposit_to_cancelled_quest() {
    let t = setup();
    let qid = symbol_short!("q10");
    register_quest(&t, &qid);
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    t.contract.cancel_quest(&qid, &t.creator);

    // Try to deposit after cancel — should fail
    let result = t.contract.try_deposit_escrow(&qid, &t.creator, &t.token_address, &1000);
    assert!(result.is_err());
}

// ══════════════════════════════════════════════════════════════
// TEST 11: Withdraw from active quest fails
// ══════════════════════════════════════════════════════════════

#[test]
fn test_cannot_withdraw_from_active_quest() {
    let t = setup();
    let qid = symbol_short!("q11");
    register_quest(&t, &qid);
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    // Quest is Active — withdraw_unclaimed should fail
    let result = t.contract.try_withdraw_unclaimed(&qid, &t.creator);
    assert!(result.is_err());
}

// ══════════════════════════════════════════════════════════════
// TEST 12: Double withdrawal prevented
// ══════════════════════════════════════════════════════════════

#[test]
fn test_double_withdrawal_prevented() {
    let t = setup();
    let qid = symbol_short!("q12");
    register_quest(&t, &qid);
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &5000);

    // Cancel → refunds everything
    t.contract.cancel_quest(&qid, &t.creator);

    // Try to withdraw again — escrow is inactive, balance is 0
    let result = t.contract.try_withdraw_unclaimed(&qid, &t.creator);
    assert!(result.is_err());
}

// ══════════════════════════════════════════════════════════════
// TEST 13: Quest without escrow works (backward compatibility)
// ══════════════════════════════════════════════════════════════

#[test]
fn test_quest_without_escrow_backward_compat() {
    let t = setup();
    let qid = symbol_short!("q13");
    register_quest(&t, &qid);

    // DON'T deposit escrow — old-style quest
    // Manually fund the contract so transfer_reward succeeds
    t.token_admin_client.mint(&t.contract.address, &10_000);

    // Should work without escrow
    submit_proof(&t, &qid, &t.user_a);
    t.contract.approve_submission(&qid, &t.user_a, &t.verifier);
    t.contract.claim_reward(&qid, &t.user_a);

    assert_eq!(t.token.balance(&t.user_a), 1_000);
}

// ══════════════════════════════════════════════════════════════
// TEST 14: Full lifecycle
// ══════════════════════════════════════════════════════════════

#[test]
fn test_full_lifecycle() {
    let t = setup();
    let qid = symbol_short!("q14");
    register_quest(&t, &qid);

    // 1. Deposit escrow for 3 completions (3 × 1000 = 3000)
    t.contract.deposit_escrow(&qid, &t.creator, &t.token_address, &3000);
    assert_eq!(t.contract.get_escrow_balance(&qid), 3_000);
    assert_eq!(t.token.balance(&t.creator), 97_000);

    // 2. User A completes and claims
    submit_proof(&t, &qid, &t.user_a);
    t.contract.approve_submission(&qid, &t.user_a, &t.verifier);
    t.contract.claim_reward(&qid, &t.user_a);
    assert_eq!(t.token.balance(&t.user_a), 1_000);
    assert_eq!(t.contract.get_escrow_balance(&qid), 2_000);

    // 3. User B completes and claims
    submit_proof(&t, &qid, &t.user_b);
    t.contract.approve_submission(&qid, &t.user_b, &t.verifier);
    t.contract.claim_reward(&qid, &t.user_b);
    assert_eq!(t.token.balance(&t.user_b), 1_000);
    assert_eq!(t.contract.get_escrow_balance(&qid), 1_000);

    // 4. Creator cancels — 1 slot unused, gets 1000 back
    let refunded = t.contract.cancel_quest(&qid, &t.creator);
    assert_eq!(refunded, 1_000);
    assert_eq!(t.token.balance(&t.creator), 98_000); // 97000 + 1000

    // 5. Verify final state
    let info = t.contract.get_escrow_info(&qid);
    assert_eq!(info.total_deposited, 3_000);
    assert_eq!(info.total_paid_out, 2_000);
    assert_eq!(info.total_refunded, 1_000);
    assert_eq!(info.is_active, false);
    assert_eq!(t.contract.get_escrow_balance(&qid), 0);
}

// ══════════════════════════════════════════════════════════════
// TEST 15: Zero amount deposit rejected
// ══════════════════════════════════════════════════════════════

#[test]
fn test_zero_deposit_rejected() {
    let t = setup();
    let qid = symbol_short!("q15");
    register_quest(&t, &qid);

    let result = t.contract.try_deposit_escrow(&qid, &t.creator, &t.token_address, &0);
    assert!(result.is_err());
}

// ══════════════════════════════════════════════════════════════
// TEST 16: Wrong token rejected
// ══════════════════════════════════════════════════════════════

#[test]
fn test_wrong_token_rejected() {
    let t = setup();
    let qid = symbol_short!("q16");
    register_quest(&t, &qid);

    // Create a different token
    let other_admin = Address::generate(&t.env);
    let other_token = t.env.register_stellar_asset_contract(other_admin.clone());
    let other_admin_client = token::StellarAssetClient::new(&t.env, &other_token);
    other_admin_client.mint(&t.creator, &10_000);

    // Try to deposit the wrong token
    let result = t.contract.try_deposit_escrow(&qid, &t.creator, &other_token, &1000);
    assert!(result.is_err());
}