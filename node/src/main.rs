//! Substrate Parachain Node Template CLI
// Code based on the Parachain Template provided by Parity Technologies.
// Modified by Group (Nguyễn Đức Hoàng, Trần Đình Việt Huy, Hồ Quốc Long, Nguyễn Bá Thiều Khôi Nguyên)
// for academic purposes (Đồ án Cơ sở – Đại học Đà Lạt, 2025).

#![warn(missing_docs)]

use polkadot_sdk::*;

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

fn main() -> sc_cli::Result<()> {
	command::run()
}
