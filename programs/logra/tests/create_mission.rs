use anchor_lang::{
    prelude::Pubkey,
    solana_program::system_program,
    InstructionData,
    ToAccountMetas,
};
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::{Instruction, Message};
use solana_signer::Signer;
use solana_transaction::Transaction;

#[test]
fn test_create_mission() {
    // ARRANGE: preparar el entorno
    let mut svm = LiteSVM::new();

    // Cargar el programa compilado
    svm.add_program(logra::ID, include_bytes!("../../../target/deploy/logra.so"))
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

    // Data para la instrucción de Crear Misión
    let instruction_data = logra::instruction::CreateMission {
        name: "Primera misión".to_string(),
        description: "Completar mi primera misión en Logra".to_string(),
        start_at: None,
        end_at: None,
    };

    // Cuentas para la instrucción de Crear Misión
    let accounts = logra::accounts::CreateMission {
        creator: creator_pubkey,
        mission_counter: mission_counter_pda,
        mission: mission_pda,
        system_program: system_program::ID,
    };

    // Construir la instrucción
    let instruction = Instruction {
        program_id: logra::ID,
        accounts: accounts.to_account_metas(None),
        data: instruction_data.data(),
    };
}
