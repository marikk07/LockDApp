use anchor_lang::prelude::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};
use escrow_program::{initialize, release, Escrow};  // Import your program logic

#[tokio::test]
async fn test_initialize() -> Result<(), TransportError> {
    let program_test = ProgramTest::new("escrow_program", escrow_program::id(), None);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create necessary accounts
    let escrow_keypair = Keypair::new();
    let authority = Keypair::new();

    // Create the context for the initialize instruction
    let condition = String::from("secret_condition");
    let amount = 100u64;

    // Initialize the escrow account
    let initialize_context = Initialize {
        escrow: Account::new(escrow_keypair.pubkey()),
        authority: Account::new(authority.pubkey()),
        system_program: Account::new(solana_sdk::system_program::id()),
    };

    let tx = initialize(Context::new(&initialize_context), amount, condition).unwrap();

    // Add logic to verify the Escrow account has the correct condition hash and amount
    // Fetch the escrow account and verify its state
    let escrow_account: Escrow = banks_client
        .get_account_data_with_borsh(escrow_keypair.pubkey())
        .await
        .unwrap();
    assert_eq!(escrow_account.amount, amount);
    assert_eq!(escrow_account.is_released, false);

    Ok(())
}

#[tokio::test]
async fn test_release() -> Result<(), TransportError> {
    let program_test = ProgramTest::new("escrow_program", escrow_program::id(), None);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create necessary accounts and initialize escrow
    let escrow_keypair = Keypair::new();
    let authority = Keypair::new();
    let condition = String::from("secret_condition");
    let amount = 100u64;

    let initialize_context = Initialize {
        escrow: Account::new(escrow_keypair.pubkey()),
        authority: Account::new(authority.pubkey()),
        system_program: Account::new(solana_sdk::system_program::id()),
    };

    // Call initialize to set up the escrow
    let tx_initialize = initialize(Context::new(&initialize_context), amount, condition).unwrap();

    // Test releasing the escrow with correct and incorrect inputs
    let correct_input = String::from("secret_condition");
    let incorrect_input = String::from("wrong_condition");

    // Create the release context
    let release_context = Release {
        escrow: Account::new(escrow_keypair.pubkey()),
        authority: Account::new(authority.pubkey()),
    };

    // Test with correct input
    let tx_release_correct = release(Context::new(&release_context), correct_input).unwrap();

    // Fetch the escrow account and verify its state
    let escrow_account: Escrow = banks_client
        .get_account_data_with_borsh(escrow_keypair.pubkey())
        .await
        .unwrap();
    assert!(escrow_account.is_released);

    // Test with incorrect input
    let tx_release_incorrect = release(Context::new(&release_context), incorrect_input);
    assert!(tx_release_incorrect.is_err());

    Ok(())
}

