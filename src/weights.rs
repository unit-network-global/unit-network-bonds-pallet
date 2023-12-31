
//! Autogenerated weights for `pallet_bonds`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro-de-Mateo.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-unitchain
// benchmark
// pallet
// --chain
// dev
// --wasm-execution
// compiled
// --pallet
// pallet-bonds
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/bonds/src/weights.rs
// --template
// frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_bonds`.
pub trait WeightInfo {
	fn asset_bond_and_stake() -> Weight;
	fn asset_unbond_and_unstake() -> Weight;
	fn withdraw_unbonded_asset() -> Weight;
	fn claim_asset_rewards() -> Weight;
	fn unregister_asset() -> Weight;
	fn register_asset() -> Weight;
	fn cancel_unbond() -> Weight;
}

/// Weights for `pallet_bonds` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::RegisteredAssets` (r:1 w:0)
	/// Proof: `Bonds::RegisteredAssets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn asset_bond_and_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2736`
		//  Estimated: `6208`
		// Minimum execution time: 97_000_000 picoseconds.
		Weight::from_parts(98_000_000, 6208)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetUnbondingPeriod` (r:1 w:0)
	/// Proof: `Bonds::AssetUnbondingPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn asset_unbond_and_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1826`
		//  Estimated: `5291`
		// Minimum execution time: 51_000_000 picoseconds.
		Weight::from_parts(55_000_000, 5291)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw_unbonded_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3173`
		//  Estimated: `6638`
		// Minimum execution time: 88_000_000 picoseconds.
		Weight::from_parts(93_000_000, 6638)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetAPR` (r:1 w:0)
	/// Proof: `Bonds::AssetAPR` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Account` (r:2 w:0)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::UserRewards` (r:0 w:1)
	/// Proof: `Bonds::UserRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn claim_asset_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3365`
		//  Estimated: `6830`
		// Minimum execution time: 91_000_000 picoseconds.
		Weight::from_parts(93_000_000, 6830)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Bonds::RegisteredAssets` (r:1 w:1)
	/// Proof: `Bonds::RegisteredAssets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetUnbondingPeriod` (r:0 w:1)
	/// Proof: `Bonds::AssetUnbondingPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unregister_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2187`
		//  Estimated: `6208`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(64_000_000, 6208)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::RegisteredAssets` (r:1 w:1)
	/// Proof: `Bonds::RegisteredAssets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetUnbondingPeriod` (r:0 w:1)
	/// Proof: `Bonds::AssetUnbondingPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:0 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetAPR` (r:0 w:1)
	/// Proof: `Bonds::AssetAPR` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2926`
		//  Estimated: `11426`
		// Minimum execution time: 127_000_000 picoseconds.
		Weight::from_parts(130_000_000, 11426)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(12_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn cancel_unbond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1909`
		//  Estimated: `5374`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(60_000_000, 5374)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::RegisteredAssets` (r:1 w:0)
	/// Proof: `Bonds::RegisteredAssets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn asset_bond_and_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2736`
		//  Estimated: `6208`
		// Minimum execution time: 97_000_000 picoseconds.
		Weight::from_parts(98_000_000, 6208)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetUnbondingPeriod` (r:1 w:0)
	/// Proof: `Bonds::AssetUnbondingPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn asset_unbond_and_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1826`
		//  Estimated: `5291`
		// Minimum execution time: 51_000_000 picoseconds.
		Weight::from_parts(55_000_000, 5291)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw_unbonded_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3173`
		//  Estimated: `6638`
		// Minimum execution time: 88_000_000 picoseconds.
		Weight::from_parts(93_000_000, 6638)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetAPR` (r:1 w:0)
	/// Proof: `Bonds::AssetAPR` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Account` (r:2 w:0)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::UserRewards` (r:0 w:1)
	/// Proof: `Bonds::UserRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn claim_asset_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3365`
		//  Estimated: `6830`
		// Minimum execution time: 91_000_000 picoseconds.
		Weight::from_parts(93_000_000, 6830)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Bonds::RegisteredAssets` (r:1 w:1)
	/// Proof: `Bonds::RegisteredAssets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetUnbondingPeriod` (r:0 w:1)
	/// Proof: `Bonds::AssetUnbondingPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unregister_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2187`
		//  Estimated: `6208`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(64_000_000, 6208)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::RegisteredAssets` (r:1 w:1)
	/// Proof: `Bonds::RegisteredAssets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetUnbondingPeriod` (r:0 w:1)
	/// Proof: `Bonds::AssetUnbondingPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:0 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetAPR` (r:0 w:1)
	/// Proof: `Bonds::AssetAPR` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2926`
		//  Estimated: `11426`
		// Minimum execution time: 127_000_000 picoseconds.
		Weight::from_parts(130_000_000, 11426)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(12_u64))
	}
	/// Storage: `SubAccount::SubAccount` (r:1 w:0)
	/// Proof: `SubAccount::SubAccount` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Profile::UserItem` (r:1 w:0)
	/// Proof: `Profile::UserItem` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bonds::AssetLedger` (r:1 w:1)
	/// Proof: `Bonds::AssetLedger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::CurrentEra` (r:1 w:0)
	/// Proof: `Bonds::CurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralStakerInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralStakerInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::AssetGeneralEraInfo` (r:1 w:1)
	/// Proof: `Bonds::AssetGeneralEraInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Bonds::UserActions` (r:1 w:1)
	/// Proof: `Bonds::UserActions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn cancel_unbond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1909`
		//  Estimated: `5374`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(60_000_000, 5374)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}