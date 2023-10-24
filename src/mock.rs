use crate::{ self as pallet_bonds, * };
use frame_support::{
	assert_ok,
	construct_runtime,
	parameter_types,
	traits::{ ConstU16, ConstU32, ConstU64, OnFinalize, OnInitialize, AsEnsureOriginWithArg },
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{ BlakeTwo256, IdentityLookup },
	BoundedVec,
	BuildStorage,
};
use sp_io::TestExternalities;
use pallet_assets::{ AssetMetadataParameter, TokenType };

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Bonds: pallet_bonds,
        Assets: pallet_assets,
        Profile: pallet_profile,
		PalletTimestamp: pallet_timestamp,
		Bank: pallet_bank,
		Treasury: pallet_treasury,
		Pool: pallet_pool,
		Oracle: pallet_oracle,
		SubAccounts: pallet_subaccount,
	}
);

impl pallet_subaccount::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Profile = Profile;
}

parameter_types! {
	pub const PoolPalletId: PalletId = PalletId(*b"unitpool");
}

impl pallet_pool::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Fungibles = Assets;
	type PalletId = PoolPalletId;
	type Time = PalletTimestamp;
}

// ORACLE CONFIG
use pallet_oracle::DefaultCombineData;
use frame_support::traits::{ Time, SortedMembers };
use sp_std::cell::RefCell;

thread_local! {
	static TIME: RefCell<u32> = RefCell::new(0);
}

pub struct Timestamp;
impl Time for Timestamp {
	type Moment = u32;

	fn now() -> Self::Moment {
		TIME.with(|v| *v.borrow())
	}
}

impl Timestamp {
	pub fn set_timestamp(val: u32) {
		TIME.with(|v| {
			*v.borrow_mut() = val;
		});
	}
}

pub struct Members;

impl SortedMembers<AccountId> for Members {
	fn sorted_members() -> Vec<AccountId> {
		OracleMembers::get()
	}
}

pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 1;
pub const CAROL: AccountId = 2;

parameter_types! {
	pub static OracleMembers: Vec<AccountId> = vec![ALICE, BOB, CAROL];
}

impl pallet_oracle::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CombineData = DefaultCombineData<Self, ConstU32<1>, ConstU32<600>>;
	type Time = Timestamp;
	type Members = Members;
	type OracleKey = AssetId;
	type OracleValue = Balance;
	type OracleSupply = u64;
	type Fungibles = Assets;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Block = Block;
	type Nonce = u32;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxLocks: u32 = 4;
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Test {
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
	type FreezeIdentifier = ();
	type RuntimeHoldReason = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub(crate) type BlockNumber = u64;
pub(crate) type Balance = u128;
pub(crate) type EraIndex = u32;
pub(crate) type AccountId = u64;
pub(crate) type AssetId = u32;
pub(crate) type Moment = u64;

/// Value shouldn't be less than 2 for testing purposes, otherwise we cannot test certain corner cases.
pub(crate) const EXISTENTIAL_DEPOSIT: Balance = 2;
pub(crate) const MINIMUM_STAKING_AMOUNT: Balance = 10;
pub(crate) const MINIMUM_REMAINING_AMOUNT: Balance = 1;
pub(crate) const MAX_UNLOCKING_CHUNKS: u32 = 29;
pub(crate) const UNBONDING_PERIOD: EraIndex = 28;
pub(crate) const MAX_ERA_STAKE_VALUES: u32 = 60;
// Do note that this needs to at least be 3 for tests to be valid. It can be greater but not smaller.
pub(crate) const BLOCKS_PER_ERA: BlockNumber = 3;
pub(crate) const ASSET_APR: u8 = 8;
pub(crate) const REGISTRATION_DEPOSIT: Balance = 1_000;
pub(crate) const ASSET_ID: AssetId = 0;

parameter_types! {
	pub const UnitStakingPalletId: PalletId = PalletId(*b"unitstke");
	pub const RewardsPoolId: PalletId = PalletId(*b"rewardpo");
	pub const MinimumStakingAmount: Balance = MINIMUM_STAKING_AMOUNT;
	pub const MinimumRemainingAmount: Balance = MINIMUM_REMAINING_AMOUNT;
	pub const MaxEraStakeValues: u32 = MAX_ERA_STAKE_VALUES;
	pub const BlockPerEra: BlockNumber = BLOCKS_PER_ERA;
	pub const MaxUnlockingChunks: u32 = MAX_UNLOCKING_CHUNKS;
	pub const RegistrationDeposit: Balance = REGISTRATION_DEPOSIT;
	pub const UnitId: u32 = 0;
}

/// Configure the pallet-bonds in pallets/bonds.
impl pallet_bonds::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type PalletId = UnitStakingPalletId;
	type RewardsPoolId = RewardsPoolId;
	type MaxEraStakeValues = MaxEraStakeValues;
	type MinimumStakingAmount = MinimumStakingAmount;
	type BlockPerEra = BlockPerEra;
	type MaxUnlockingChunks = MaxUnlockingChunks;
	type AssetId = u32;
	type AssetBalance = u128;
	type Fungibles = Assets;
	type AssetRegistrationFee = RegistrationDeposit;
	type Time = PalletTimestamp;
	type UnitId = UnitId;
	type Profile = Profile;
	type SubAccounts = SubAccounts;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = Profile;
	//type WeightInfo = ();
}

parameter_types! {
	pub const BankPalletId: PalletId = PalletId(*b"py/banka");
}

impl pallet_bank::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Fungibles = Assets;
	type PalletId = BankPalletId;
	type TimeProvider = PalletTimestamp;
	type TreasuryInterface = Treasury;
	type SubAccounts = SubAccounts;
}

parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/treas");
}

impl pallet_treasury::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Fungibles = Assets;
	type PalletId = TreasuryPalletId;
	type TimeProvider = PalletTimestamp;
	type PoolInterface = Pool;
	type OracleInterface = Oracle;
	type SubAccounts = SubAccounts;
	type AssetId = u32;
	type AssetBalance = u128;
	type WeightInfo = ();
}

parameter_types! {
	pub const DefaultSupply: u32 = 100_000;
	pub const MinSupply: Balance = 100;
	pub const DefaultDecimals: u8 = 10;
	pub const SymbolMaxLength: u32 = 6;
	pub const NameMaxLength: u32 = 25;
	pub const PalletBank: PalletId = PalletId(*b"bank____");
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = ();
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type AssetDeposit = ConstU32<1>;
	type AssetAccountDeposit = ConstU32<10>;
	type MetadataDepositBase = ConstU32<1>;
	type MetadataDepositPerByte = ConstU32<1>;
	type ApprovalDeposit = ConstU32<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	type Extra = ();
	type RemoveItemsLimit = ConstU32<5>;
	type DefaultSupply = DefaultSupply;
	type MinSupply = MinSupply;
	type DefaultDecimals = DefaultDecimals;
	type SymbolMaxLength = SymbolMaxLength;
	type AssetPalletId = PalletBank;
	type NameMaxLength = NameMaxLength;
	type TimeProvider = PalletTimestamp;
	type Bank = Bank;
	type Treasury = Treasury;
	type SubAccounts = SubAccounts;
}

impl pallet_profile::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FollowIndex = u64;
	type SubAccounts = SubAccounts;
	type TimeProvider = PalletTimestamp;
	type AssetsFungibles = Assets;
}

pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = ();
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let mut storage = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

		let config: pallet_assets::GenesisConfig<Test> = pallet_assets::GenesisConfig {
			assets: vec![
				// id, owner, is_sufficient, min_balance
				(0, true, 1)
			],
			metadata: vec![(
				ASSET_ID,
				"Token Name".as_bytes().to_vec(),
				"Symbol".as_bytes().to_vec(),
				10,
			)],

			accounts: vec![
				// id, account_id, balance
				(ASSET_ID, 0, 100_000_000_000),
				(ASSET_ID, 1, 100_000_000),
				(ASSET_ID, 2, 100_000_000),
				(ASSET_ID, 3, 100_000_000),
				(ASSET_ID, Bonds::account_id(), 100_000_000_000)
			],
		};

		config.assimilate_storage(&mut storage).unwrap();
		(pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(0, 1000000),
				(1, 10000000),
				(2, 100000),
				(3, 10000),
				(4, 4900),
				(5, 3800),
				(6, 10),
				(7, 1000),
				(8, 2000),
				(9, 10000),
				(10, 300),
				(11, 400),
				(20, 10),
				(540, EXISTENTIAL_DEPOSIT),
				(Bonds::account_id(), 1_000_000_000_000)
			],
		})
			.assimilate_storage(&mut storage)
			.ok();

		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| {
			System::set_block_number(1);
			assert_ok!(
				Profile::create_user(
					RuntimeOrigin::signed(0),
					"email".as_bytes().to_vec(),
					"name".as_bytes().to_vec(),
					"lastName".as_bytes().to_vec(),
					"dateOfbirth".as_bytes().to_vec(),
					"bio".as_bytes().to_vec(),
					"User1".as_bytes().to_vec(),
					"website".as_bytes().to_vec(),
					"linkedin".as_bytes().to_vec(),
					"twitter".as_bytes().to_vec(),
					"instagram".as_bytes().to_vec(),
					"telegram".as_bytes().to_vec(),
					"youtube".as_bytes().to_vec(),
					"facebook".as_bytes().to_vec(),
					"vision".as_bytes().to_vec(),
					"tagLIne".as_bytes().to_vec(),
					0,
					"currentTime".as_bytes().to_vec(),
					0,
					"cityToken".as_bytes().to_vec(),
					"industyToken".as_bytes().to_vec(),
					"personalTOken".as_bytes().to_vec(),
					"projectToken".as_bytes().to_vec(),
					"profileColor".as_bytes().to_vec(),
					"profileInitials".as_bytes().to_vec()
				)
			);
			assert_ok!(
				Profile::create_user(
					RuntimeOrigin::signed(1),
					"email".as_bytes().to_vec(),
					"name".as_bytes().to_vec(),
					"lastName".as_bytes().to_vec(),
					"dateOfbirth".as_bytes().to_vec(),
					"bio".as_bytes().to_vec(),
					"User2".as_bytes().to_vec(),
					"website".as_bytes().to_vec(),
					"linkedin".as_bytes().to_vec(),
					"twitter".as_bytes().to_vec(),
					"instagram".as_bytes().to_vec(),
					"telegram".as_bytes().to_vec(),
					"youtube".as_bytes().to_vec(),
					"facebook".as_bytes().to_vec(),
					"vision".as_bytes().to_vec(),
					"tagLIne".as_bytes().to_vec(),
					0,
					"currentTime".as_bytes().to_vec(),
					0,
					"cityToken".as_bytes().to_vec(),
					"industyToken".as_bytes().to_vec(),
					"personalTOken".as_bytes().to_vec(),
					"projectToken".as_bytes().to_vec(),
					"profileColor".as_bytes().to_vec(),
					"profileInitials".as_bytes().to_vec()
				)
			);
			assert_ok!(
				Profile::create_user(
					RuntimeOrigin::signed(2),
					"email".as_bytes().to_vec(),
					"name".as_bytes().to_vec(),
					"lastName".as_bytes().to_vec(),
					"dateOfbirth".as_bytes().to_vec(),
					"bio".as_bytes().to_vec(),
					"User3".as_bytes().to_vec(),
					"website".as_bytes().to_vec(),
					"linkedin".as_bytes().to_vec(),
					"twitter".as_bytes().to_vec(),
					"instagram".as_bytes().to_vec(),
					"telegram".as_bytes().to_vec(),
					"youtube".as_bytes().to_vec(),
					"facebook".as_bytes().to_vec(),
					"vision".as_bytes().to_vec(),
					"tagLIne".as_bytes().to_vec(),
					0,
					"currentTime".as_bytes().to_vec(),
					1,
					"cityToken".as_bytes().to_vec(),
					"industyToken".as_bytes().to_vec(),
					"personalTOken".as_bytes().to_vec(),
					"projectToken".as_bytes().to_vec(),
					"profileColor".as_bytes().to_vec(),
					"profileInitials".as_bytes().to_vec()
				)
			);
			assert_ok!(
				Profile::create_user(
					RuntimeOrigin::signed(3),
					"email".as_bytes().to_vec(),
					"name".as_bytes().to_vec(),
					"lastName".as_bytes().to_vec(),
					"dateOfbirth".as_bytes().to_vec(),
					"bio".as_bytes().to_vec(),
					"User3".as_bytes().to_vec(),
					"website".as_bytes().to_vec(),
					"linkedin".as_bytes().to_vec(),
					"twitter".as_bytes().to_vec(),
					"instagram".as_bytes().to_vec(),
					"telegram".as_bytes().to_vec(),
					"youtube".as_bytes().to_vec(),
					"facebook".as_bytes().to_vec(),
					"vision".as_bytes().to_vec(),
					"tagLIne".as_bytes().to_vec(),
					0,
					"currentTime".as_bytes().to_vec(),
					2,
					"cityToken".as_bytes().to_vec(),
					"industyToken".as_bytes().to_vec(),
					"personalTOken".as_bytes().to_vec(),
					"projectToken".as_bytes().to_vec(),
					"profileColor".as_bytes().to_vec(),
					"profileInitials".as_bytes().to_vec()
				)
			);
		});
		ext
	}
}

/// Used to run to the specified block number
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		Bonds::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);

		Bonds::on_initialize(System::block_number());
	}
}

/// Used to run the specified number of blocks
pub fn run_for_blocks(n: u64) {
	run_to_block(System::block_number() + n);
}

/// Advance blocks to the beginning of an era.
///
/// Function has no effect if era is already passed.
pub fn advance_to_era(n: EraIndex) {
	while Bonds::current_era() < n {
		run_for_blocks(1);
	}
}

/// Initialize first block.
/// This method should only be called once in a UT otherwise the first block will get initialized multiple times.
pub fn initialize_first_block() {
	// This assert prevents method misuse
	assert_eq!(System::block_number(), 1 as BlockNumber);

	Bonds::on_initialize(System::block_number());
	run_to_block(2);
}

// Used to get a vec of all events
pub fn events() -> Vec<crate::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let RuntimeEvent::Bonds(inner) = e { Some(inner) } else { None }
		})
		.collect()
}
