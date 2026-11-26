use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_signer::Signer;

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
}
