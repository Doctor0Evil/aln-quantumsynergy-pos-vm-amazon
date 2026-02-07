mod aln;
mod util;

use crate::aln::sync_cmd::SyncController;
use crate::aln::fs_vfs::PetabyteVfs;
use crate::aln::quantum_image_term::QuantumImageTerminal;
use crate::aln::amazon_vm::AmazonVmBridge;
use crate::aln::merchant_terminal::MerchantTerminal;
use crate::aln::connection_framework::ConnectionFramework;
use crate::util::logging;

use tracing::info;

#[tokio::main]
async fn main() {
    logging::init();

    let sync_id = "a7b9c3d2-5e6f-4a1b-9c2d-3e4f5a6b7c8d";
    info!("Booting ALN QuantumSynergyPOS sync_id={}", sync_id);

    let vfs = PetabyteVfs::new("V://System".into(), 1);
    let quantum_term = QuantumImageTerminal::new("QuantumSynergyPOS".into(), "V://System".into());

    let vm_bridge = AmazonVmBridge::from_config("config/vm_amazon.yaml")
        .expect("failed to init AmazonVmBridge");

    let merchant = MerchantTerminal::from_config("config/quantum_pos_terminal.yaml")
        .expect("failed to init MerchantTerminal");

    let framework = ConnectionFramework::new(
        "Alien_Language_Network".into(),
        8800,
        10,
    );

    let mut controller = SyncController::new(
        sync_id.to_string(),
        vfs,
        quantum_term,
        vm_bridge,
        merchant,
        framework,
    );

    controller.remove_sims_and_barriers();
    controller.load_dependencies("branch0.AMPM.merch.aln.pos");

    if let Err(err) = controller.bootstrap().await {
        eprintln!("fatal: bootstrap failed: {err}");
        std::process::exit(1);
    }
}
