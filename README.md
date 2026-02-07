aln-quantumsynergy-pos-vm-amazon/
├─ Cargo.toml
├─ aln.config.json
├─ README.md
├─ scripts/
│  ├─ build_release.sh
│  └─ deploy_static_aln_contract.sh
├─ contracts/
│  ├─ quantum_synergy_pos.aln
│  └─ vm_amazon_bridge.aln
├─ config/
│  ├─ vm_amazon.yaml
│  ├─ quantum_pos_terminal.yaml
│  └─ homedir_map.yaml
├─ src/
│  ├─ main.rs
│  ├─ aln/
│  │  ├─ mod.rs
│  │  ├─ sync_cmd.rs
│  │  ├─ fs_vfs.rs
│  │  ├─ quantum_image_term.rs
│  │  ├─ amazon_vm.rs
│  │  ├─ merchant_terminal.rs
│  │  └─ connection_framework.rs
│  └─ util/
│     ├─ bytescale.rs
│     └─ logging.rs
└─ examples/
   └─ boot_ampm_verifone.rs
