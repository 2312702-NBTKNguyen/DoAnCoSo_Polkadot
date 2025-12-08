// Code based on the Parachain Template provided by Parity Technologies.
// Modified by Group (Nguyễn Đức Hoàng, Trần Đình Việt Huy, Hồ Quốc Long, Nguyễn Bá Thiều Khôi Nguyên)
// for academic purposes (Đồ án Cơ sở – Đại học Đà Lạt, 2025).

use polkadot_sdk::substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
	generate_cargo_keys();

	rerun_if_git_head_changed();
}
