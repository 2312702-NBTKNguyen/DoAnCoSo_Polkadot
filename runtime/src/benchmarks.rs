// Code based on the Parachain Template provided by Parity Technologies.
// Modified by Group (Nguyễn Đức Hoàng, Trần Đình Việt Huy, Hồ Quốc Long, Nguyễn Bá Thiều Khôi Nguyên)
// for academic purposes (Đồ án Cơ sở – Đại học Đà Lạt, 2025).

polkadot_sdk::frame_benchmarking::define_benchmarks!(
	[frame_system, SystemBench::<Runtime>]
	[pallet_balances, Balances]
	[pallet_session, SessionBench::<Runtime>]
	[pallet_timestamp, Timestamp]
	[pallet_message_queue, MessageQueue]
	[pallet_sudo, Sudo]
	[pallet_collator_selection, CollatorSelection]
	[cumulus_pallet_parachain_system, ParachainSystem]
	[cumulus_pallet_xcmp_queue, XcmpQueue]
	[cumulus_pallet_weight_reclaim, WeightReclaim]
	[blog_pallet, Blog]
);
