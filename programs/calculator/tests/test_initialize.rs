use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    std::{path::PathBuf, process::Command},
};

fn read_program_bytes() -> Vec<u8> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../../target/deploy/calculator.so");
    if !path.exists() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let status = Command::new("cargo")
            .args(["build-sbf", "--manifest-path"])
            .arg(&manifest)
            .status()
            .expect("failed to run cargo build-sbf");
        assert!(status.success(), "cargo build-sbf failed");
    }
    std::fs::read(path).expect("failed to read built calculator program")
}

#[test]
fn test_initialize() {
    let program_id = calculator::id();
    let payer = Keypair::new();
    let new_account = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = read_program_bytes();
    svm.add_program(program_id, &bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &calculator::instruction::Initialize {}.data(),
        calculator::accounts::Initialize {
            new_account: new_account.pubkey(),
            signer: payer.pubkey(),
            system_program: anchor_lang::solana_program::system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer, &new_account])
            .unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());
}
