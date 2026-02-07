use aln_quantumsynergy_pos_vm_amazon::aln::sync_cmd::SyncController;
use aln_quantumsynergy_pos_vm_amazon::aln::fs_vfs::PetabyteVfs;
use aln_quantumsynergy_pos_vm_amazon::aln::quantum_image_term::QuantumImageTerminal;
use aln_quantumsynergy_pos_vm_amazon::aln::amazon_vm::AmazonVmBridge;
use aln_quantumsynergy_pos_vm_amazon::aln::merchant_terminal::MerchantTerminal;
use aln_quantumsynergy_pos_vm_amazon::aln::connection_framework::ConnectionFramework;

#[tokio::main]
async fn main() {
    let vfs = PetabyteVfs::new("V://System".into(), 1);
    let quantum_term = QuantumImageTerminal::new("QuantumSynergyPOS".into(), "V://System".into());
    let vm_bridge = AmazonVmBridge::from_config("config/vm_amazon.yaml").unwrap();
    let merchant = MerchantTerminal::from_config("config/quantum_pos_terminal.yaml").unwrap();
    let framework = ConnectionFramework::new("Alien_Language_Network".into(), 8800, 10);

    let mut controller = SyncController::new(
        "a7b9c3d2-5e6f-4a1b-9c2d-3e4f5a6b7c8d".to_string(),
        vfs,
        quantum_term,
        vm_bridge,
        merchant,
        framework,
    );

    controller.remove_sims_and_barriers();
    controller.load_dependencies("branch0.AMPM.merch.aln.pos");
    controller.bootstrap().await.unwrap();
}
