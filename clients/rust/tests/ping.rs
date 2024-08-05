#![cfg(feature = "test-sbf")]

use ping::instructions::PingBuilder;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{signature::Signer, transaction::Transaction};

#[tokio::test]
async fn ping() {
    let mut context = ProgramTest::new("ping_program", ping::ID, None)
        .start_with_context()
        .await;

    let ix = PingBuilder::new().instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();
}
