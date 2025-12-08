//! Mock runtime cho testing blog pallet

use crate as blog_pallet;
use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU32, ConstU64},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Cấu hình mock runtime
frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		Balances: pallet_balances,
		Blog: blog_pallet,
	}
);

// Cấu hình System pallet
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type RuntimeCall = RuntimeCall;
	type RuntimeTask = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type ExtensionsWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

// Cấu hình Balances pallet
impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ConstU32<50>;
	type DoneSlashHandler = ();
}

// Các tham số cho Blog pallet
parameter_types! {
	pub const BlogPalletId: PalletId = PalletId(*b"blogpall");
	pub const PostCreationFee: u128 = 10;
	pub const CommentCreationFee: u128 = 1;
	pub const MaxTitleLength: u32 = 200;
	pub const MaxContentLength: u32 = 10_000;
	pub const MaxCommentLength: u32 = 1_000;
	pub const MaxCommentsPerPost: u32 = 100;
	pub const MaxTagsPerPost: u32 = 10;
	pub const MaxTagLength: u32 = 64;
}

// Cấu hình Blog pallet
impl blog_pallet::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type PalletId = BlogPalletId;
	type PostCreationFee = PostCreationFee;
	type CommentCreationFee = CommentCreationFee;
	type MaxTitleLength = MaxTitleLength;
	type MaxContentLength = MaxContentLength;
	type MaxCommentLength = MaxCommentLength;
	type MaxCommentsPerPost = MaxCommentsPerPost;
	type MaxTagsPerPost = MaxTagsPerPost;
	type MaxTagLength = MaxTagLength;
	type WeightInfo = ();
}

// Build test externalities
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.expect("Frame system builds valid default genesis config");

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 1000),
			(2, 1000),
			(3, 1000),
		],
		dev_accounts: Default::default(),
		..Default::default()
	}
	.assimilate_storage(&mut storage)
	.expect("Pallet balances storage can be assimilated");

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
