use litesvm::LiteSVM;
use logra::state::mission_counter;
use solana_keypair::Keypair;
use solana_signer::Signer;

use anchor_lang::prelude::Pubkey;
use anchor_lang::{InstructionData, ToAccountMetas};
use solana_message::Message;
use solana_transaction::Transaction;

#[test]
fn test_create_mission() {
    // ARRANGE: preparar el entorno
    let mut svm = LiteSVM::new();

    // Cargar el programa compilado
    svm.add_program(
        logra::ID,
        include_bytes!("../../../target/deploy/logra.so"))
        .unwrap();

    // Crear wallet de prueba
    let creator = Keypair::new();
    // Financiar wallet con SOL de prueba
    svm.airdrop(&creator.pubkey(), 2_000_000_000).unwrap();
    // Verificar que la wallet tiene fondos
    assert!(svm.get_account(&creator.pubkey()).is_some());

    // Derivar las cuentas PDA
    let creator_pubkey = creator.pubkey();
    let mission_id = 0u64;
    let mission_id_bytes = mission_id.to_le_bytes();

    let (mission_counter_pda, _) = Pubkey::find_program_address(
        &[
            logra::constants::MISSION_COUNTER_SEED,
            creator_pubkey.as_ref(),
        ],
        &logra::ID,
    );

    let (mission_pda, _) = Pubkey::find_program_address(
        &[
            logra::constants::MISSION_SEED,
            creator_pubkey.as_ref(),
            mission_id_bytes.as_ref(),
        ],
        &logra::ID,
    );
}
