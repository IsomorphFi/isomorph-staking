use fuels::{
    prelude::*,
    accounts::wallet::WalletUnlocked,
    tx::{ContractId, StorageSlot},
};

abigen!(Contract(
    name = "LiquidStaking",
    abi = "out/debug/liquid_staking-abi.json"
));

abigen!(Contract(
    name = "LSToken",
    abi = "out/debug/ls_token-abi.json"
));

// function to test constants
const INITIAL_BALANCE: u64 = 1_000_000;
const STAKE_AMOUNT: u64 = 100_000;

struct TestContext {
    wallet: WalletUnlocked,
    staking_contract: LiquidStaking<WalletUnlocked>,
    ls_token_contract: LSToken<WalletUnlocked>,
}

async fn setup_test() -> TestContext {
    // Create a wallet
    let wallet = launch_provider_and_get_wallet().await;

    // Deploy the LS Token contract first
    let ls_token_id = Contract::deploy(
        "./out/debug/ls_token.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(Vec::new())),
    )
    .await
    .unwrap();

    let staking_id = Contract::deploy(
        "./out/debug/liquid_staking.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(Vec::new())),
    )
    .await
    .unwrap();

    // Create contract instances
    let staking_contract = LiquidStaking::new(staking_id.clone(), wallet.clone());
    let ls_token_contract = LSToken::new(ls_token_id.clone(), wallet.clone());

    // Initialize the staking contract with the LS token address
    staking_contract.methods()
        .initialize(ContractId::from(ls_token_id))
        .call()
        .await
        .unwrap();

    TestContext {
        wallet,
        staking_contract,
        ls_token_contract,
    }
}

#[tokio::test]
async fn test_stake_and_receive_ls_tokens() {
    let TestContext { wallet, staking_contract, ls_token_contract } = setup_test().await;

    // Perform stake
    staking_contract.methods()
        .stake()
        .call_params(CallParameters::default().with_amount(STAKE_AMOUNT))
        .call()
        .await
        .unwrap();

    // Check staked amount
    let staked_amount = staking_contract.methods()
        .get_staked_amount(wallet.address())
        .call()
        .await
        .unwrap()
        .value;

    // Check LS token balance
    let ls_token_balance = ls_token_contract.methods()
        .balance_of(wallet.address())
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(staked_amount, ls_token_balance, "LS token amount should match staked amount");
}

#[tokio::test]
async fn test_unstake_and_burn_ls_tokens() {
    let TestContext { wallet, staking_contract, ls_token_contract } = setup_test().await;

    // First stake
    staking_contract.methods()
        .stake()
        .call_params(CallParameters::default().with_amount(STAKE_AMOUNT))
        .call()
        .await
        .unwrap();

    // Wait for minimum staking period
    std::thread::sleep(std::time::Duration::from_secs(86400));

    // Record initial LS token balance
    let initial_ls_balance = ls_token_contract.methods()
        .balance_of(wallet.address())
        .call()
        .await
        .unwrap()
        .value;

    // Unstake half
    let unstake_amount = STAKE_AMOUNT / 2;
    staking_contract.methods()
        .unstake(unstake_amount)
        .call()
        .await
        .unwrap();

    // Check final LS token balance
    let final_ls_balance = ls_token_contract.methods()
        .balance_of(wallet.address())
        .call()
        .await
        .unwrap()
        .value;

    // Verify LS tokens were burned
    assert_eq!(
        final_ls_balance,
        initial_ls_balance - unstake_amount,
        "LS tokens should be burned on unstake"
    );
}

// Previous test cases remain the same...
[Previous test cases from the earlier artifact...]

// Helper function remains the same...
async fn launch_provider_and_get_wallet() -> WalletUnlocked {
    // Same implementation as before...
}