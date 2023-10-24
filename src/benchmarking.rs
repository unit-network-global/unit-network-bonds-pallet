//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Bond;
use frame_benchmarking::v2::*;
use frame_system::{ RawOrigin };
use frame_support::traits::{ fungibles::{ Create, Mutate, Inspect }, Get, Hooks };
use traits::profile::ProfileInterface;
use sp_runtime::SaturatedConversion;
const UNIT_ID: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks(
	where
	T: Config<AssetId = u32, AssetBalance = u128>,
	T::Fungibles: Create<T::AccountId> + Mutate<T::AccountId> + Inspect<T::AccountId>,
	T::BenchmarkHelper: ProfileInterface<T::AccountId>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn asset_bond_and_stake() -> Result<(), BenchmarkError> {
		let token_amount: AssetBalanceOf<T> = (100_000_000_000u128).try_into().ok().unwrap();
		let unit_fee_to_register_asset: AssetBalanceOf<T> = T::AssetRegistrationFee::get();
		let unbonding_period: u32 = 20;
		let apr: u8 = 10;
		let asset_id: u32 = 1000;

		let caller: T::AccountId = account("account", 0, 0);

		// Create profile for the caller
		let _ = T::BenchmarkHelper::create_user(
			caller.clone(),
			"email".as_bytes().to_vec(),
			"first_name".as_bytes().to_vec(),
			"last_name".as_bytes().to_vec(),
			"date_of_birth".as_bytes().to_vec(),
			"bio".as_bytes().to_vec(),
			"username".as_bytes().to_vec(),
			"website".as_bytes().to_vec(),
			"linkedin".as_bytes().to_vec(),
			"twitter".as_bytes().to_vec(),
			"instagram".as_bytes().to_vec(),
			"telegram".as_bytes().to_vec(),
			"youtube_url".as_bytes().to_vec(),
			"facebook".as_bytes().to_vec(),
			"vision".as_bytes().to_vec(),
			"tag_line".as_bytes().to_vec(),
			0u32, // exchange_volume
			"current_time".as_bytes().to_vec(),
			caller.clone(), // inviter
			"city_token_symbol".as_bytes().to_vec(),
			"industry_token_symbol".as_bytes().to_vec(),
			"personal_token_symbol".as_bytes().to_vec(),
			"project_token_symbol".as_bytes().to_vec(),
			"profile_picture_colour_hex".as_bytes().to_vec(),
			"profile_picture_initials".as_bytes().to_vec()
		);

		// Create asset to be staked
		T::Fungibles::create(asset_id, caller.clone(), true, (100u32).try_into().ok().unwrap())?;

		// Mint assets into the caller
		T::Fungibles::mint_into(asset_id, &caller, AssetBalanceOf::<T>::from(token_amount).into())?;

		// Mint Unit asset to the caller. (Used to pay fees)
		T::Fungibles::mint_into(UNIT_ID, &caller, unit_fee_to_register_asset * 2)?;

		// Register asset for staking and transfer funds to the reward pool
		Bond::<T>::register_asset(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			token_amount / 10,
			unbonding_period,
			apr
		)?;

		#[extrinsic_call]
		asset_bond_and_stake(RawOrigin::Signed(caller.clone()), asset_id.into(), token_amount / 2);

		assert_eq!(Bond::<T>::asset_ledger(asset_id, caller.clone()).locked, token_amount / 2);

		assert_eq!(
			Bond::<T>::asset_staker_info(asset_id, caller).stakes[0].staked,
			token_amount / 2
		);

		Ok(())
	}

	#[benchmark]
	fn asset_unbond_and_unstake() -> Result<(), BenchmarkError> {
		let token_amount: AssetBalanceOf<T> = (100_000_000_000u128).try_into().ok().unwrap();
		let unit_fee_to_register_asset: AssetBalanceOf<T> = T::AssetRegistrationFee::get();
		let unbonding_period: u32 = 20;
		let apr: u8 = 10;
		let asset_id: u32 = 1000;

		let caller: T::AccountId = whitelisted_caller();

		// Create profile for the caller
		let _ = T::BenchmarkHelper::create_user(
			caller.clone(),
			"email".as_bytes().to_vec(),
			"first_name".as_bytes().to_vec(),
			"last_name".as_bytes().to_vec(),
			"date_of_birth".as_bytes().to_vec(),
			"bio".as_bytes().to_vec(),
			"username".as_bytes().to_vec(),
			"website".as_bytes().to_vec(),
			"linkedin".as_bytes().to_vec(),
			"twitter".as_bytes().to_vec(),
			"instagram".as_bytes().to_vec(),
			"telegram".as_bytes().to_vec(),
			"youtube_url".as_bytes().to_vec(),
			"facebook".as_bytes().to_vec(),
			"vision".as_bytes().to_vec(),
			"tag_line".as_bytes().to_vec(),
			0u32, // exchange_volume
			"current_time".as_bytes().to_vec(),
			caller.clone(), // inviter
			"city_token_symbol".as_bytes().to_vec(),
			"industry_token_symbol".as_bytes().to_vec(),
			"personal_token_symbol".as_bytes().to_vec(),
			"project_token_symbol".as_bytes().to_vec(),
			"profile_picture_colour_hex".as_bytes().to_vec(),
			"profile_picture_initials".as_bytes().to_vec()
		);

		// Create asset to be staked
		T::Fungibles::create(asset_id, caller.clone(), true, (1u32).try_into().ok().unwrap())?;

		// Mint assets into the caller
		let _ = T::Fungibles::mint_into(
			asset_id,
			&caller,
			AssetBalanceOf::<T>::from(token_amount).into()
		);

		// Mint Unit asset to the caller. (Used to pay fees)
		T::Fungibles::mint_into(UNIT_ID, &caller, unit_fee_to_register_asset * 2)?;

		// Register asset for staking and transfer funds to the reward pool
		Bond::<T>::register_asset(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			token_amount / 2,
			unbonding_period,
			apr
		)?;

		let max_stakes = T::MaxEraStakeValues::get();

		for _ in 0..max_stakes {
			Bond::<T>::asset_bond_and_stake(
				RawOrigin::Signed(caller.clone()).into(),
				asset_id.into(),
				100
			)?;
		}

		#[extrinsic_call]
		asset_unbond_and_unstake(RawOrigin::Signed(caller.clone()), asset_id.into(), 100);

		assert_eq!(
			Bond::<T>::asset_ledger(asset_id, caller.clone()).locked,
			100 * (max_stakes as u128)
		);

		assert_eq!(
			Bond::<T>::asset_ledger(asset_id, caller.clone()).unbonding_info.unlocking_chunks
				[0].amount,
			100
		);

		assert_eq!(
			Bond::<T>::asset_staker_info(asset_id, caller).latest_staked_value(),
			100 * (max_stakes as u128) - 100
		);

		Ok(())
	}

	#[benchmark]
	fn withdraw_unbonded_asset() -> Result<(), BenchmarkError> {
		let token_amount: AssetBalanceOf<T> = (1_000_000_000u128).try_into().ok().unwrap();
		let unit_fee_to_register_asset: AssetBalanceOf<T> = T::AssetRegistrationFee::get();
		let unbonding_period: u32 = 1;
		let apr: u8 = 10;
		let asset_id: u32 = 1000;

		let caller: T::AccountId = whitelisted_caller();

		// Create profile for the caller
		let _ = T::BenchmarkHelper::create_user(
			caller.clone(),
			"email".as_bytes().to_vec(),
			"first_name".as_bytes().to_vec(),
			"last_name".as_bytes().to_vec(),
			"date_of_birth".as_bytes().to_vec(),
			"bio".as_bytes().to_vec(),
			"username".as_bytes().to_vec(),
			"website".as_bytes().to_vec(),
			"linkedin".as_bytes().to_vec(),
			"twitter".as_bytes().to_vec(),
			"instagram".as_bytes().to_vec(),
			"telegram".as_bytes().to_vec(),
			"youtube_url".as_bytes().to_vec(),
			"facebook".as_bytes().to_vec(),
			"vision".as_bytes().to_vec(),
			"tag_line".as_bytes().to_vec(),
			0u32, // exchange_volume
			"current_time".as_bytes().to_vec(),
			caller.clone(), // inviter
			"city_token_symbol".as_bytes().to_vec(),
			"industry_token_symbol".as_bytes().to_vec(),
			"personal_token_symbol".as_bytes().to_vec(),
			"project_token_symbol".as_bytes().to_vec(),
			"profile_picture_colour_hex".as_bytes().to_vec(),
			"profile_picture_initials".as_bytes().to_vec()
		);

		// Create asset to be staked
		T::Fungibles::create(asset_id, caller.clone(), true, (1u32).try_into().ok().unwrap())?;

		// Mint assets into the caller
		let _ = T::Fungibles::mint_into(
			asset_id,
			&caller,
			AssetBalanceOf::<T>::from(token_amount).into()
		);

		// Mint Unit asset to the caller. (Used to pay fees)
		T::Fungibles::mint_into(UNIT_ID, &caller, unit_fee_to_register_asset * 2)?;

		// Register asset for staking and transfer funds to the reward pool
		Bond::<T>::register_asset(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			token_amount / 2,
			unbonding_period,
			apr
		)?;

		Bond::<T>::asset_bond_and_stake(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			token_amount / 3
		)?;

		let max_unlocking_items = T::MaxUnlockingChunks::get();

		for _ in 0..max_unlocking_items {
			Bond::<T>::asset_unbond_and_unstake(
				RawOrigin::Signed(caller.clone()).into(),
				asset_id.into(),
				100
			)?;
		}

		// advance blocks executing on_initialize
		while frame_system::Pallet::<T>::block_number() < (20u32).into() {
			Bond::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
			frame_system::Pallet::<T>::set_block_number(
				frame_system::Pallet::<T>::block_number() + (1u32).into()
			);

			Bond::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		}

		#[extrinsic_call]
		withdraw_unbonded_asset(RawOrigin::Signed(caller.clone()), asset_id.into());

		assert_eq!(
			Bond::<T>::asset_ledger(asset_id, caller.clone()).locked,
			token_amount / 3 - 100 * (max_unlocking_items as u128)
		);

		assert!(
			Bond::<T>
				::asset_ledger(asset_id, caller.clone())
				.unbonding_info.unlocking_chunks.is_empty()
		);

		Ok(())
	}

	#[benchmark]
	fn claim_asset_rewards() -> Result<(), BenchmarkError> {
		let token_amount: AssetBalanceOf<T> = (1_000_000_000u128).try_into().ok().unwrap();
		let unit_fee_to_register_asset: AssetBalanceOf<T> = T::AssetRegistrationFee::get();
		let unbonding_period: u32 = 1;
		let apr: u8 = 10;
		let asset_id: u32 = 1000;

		let caller: T::AccountId = whitelisted_caller();

		// Create profile for the caller
		let _ = T::BenchmarkHelper::create_user(
			caller.clone(),
			"email".as_bytes().to_vec(),
			"first_name".as_bytes().to_vec(),
			"last_name".as_bytes().to_vec(),
			"date_of_birth".as_bytes().to_vec(),
			"bio".as_bytes().to_vec(),
			"username".as_bytes().to_vec(),
			"website".as_bytes().to_vec(),
			"linkedin".as_bytes().to_vec(),
			"twitter".as_bytes().to_vec(),
			"instagram".as_bytes().to_vec(),
			"telegram".as_bytes().to_vec(),
			"youtube_url".as_bytes().to_vec(),
			"facebook".as_bytes().to_vec(),
			"vision".as_bytes().to_vec(),
			"tag_line".as_bytes().to_vec(),
			0u32, // exchange_volume
			"current_time".as_bytes().to_vec(),
			caller.clone(), // inviter
			"city_token_symbol".as_bytes().to_vec(),
			"industry_token_symbol".as_bytes().to_vec(),
			"personal_token_symbol".as_bytes().to_vec(),
			"project_token_symbol".as_bytes().to_vec(),
			"profile_picture_colour_hex".as_bytes().to_vec(),
			"profile_picture_initials".as_bytes().to_vec()
		);

		// Create asset to be staked
		T::Fungibles::create(asset_id, caller.clone(), true, (1u32).try_into().ok().unwrap())?;

		// Mint assets into the caller
		let _ = T::Fungibles::mint_into(
			asset_id,
			&caller,
			AssetBalanceOf::<T>::from(token_amount).into()
		);

		// Mint Unit asset to the caller. (Used to pay fees)
		T::Fungibles::mint_into(UNIT_ID, &caller, unit_fee_to_register_asset * 2)?;

		// Register asset for staking and transfer funds to the reward pool
		Bond::<T>::register_asset(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			token_amount / 2,
			unbonding_period,
			apr
		)?;

		let max_stakes = T::MaxEraStakeValues::get();

		for _ in 0..max_stakes {
			Bond::<T>::asset_bond_and_stake(
				RawOrigin::Signed(caller.clone()).into(),
				asset_id.into(),
				100
			)?;
		}

		// advance blocks executing on_initialize
		while frame_system::Pallet::<T>::block_number() < (20u32).into() {
			Bond::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
			frame_system::Pallet::<T>::set_block_number(
				frame_system::Pallet::<T>::block_number() + (1u32).into()
			);

			Bond::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		}

		#[extrinsic_call]
		claim_asset_rewards(RawOrigin::Signed(caller.clone()), asset_id.into());

		assert!(Bond::<T>::user_actions(asset_id, caller).len() > 0);

		Ok(())
	}

	#[benchmark]
	fn unregister_asset() -> Result<(), BenchmarkError> {
		let token_amount: AssetBalanceOf<T> = (1_000_000_000_000_000u128).try_into().ok().unwrap();
		let unbonding_period: u32 = 1;
		let apr: u8 = 10;
		let asset_id: u32 = 1000;

		let caller: T::AccountId = whitelisted_caller();

		// Create profile for the caller
		let _ = T::BenchmarkHelper::create_user(
			caller.clone(),
			"email".as_bytes().to_vec(),
			"first_name".as_bytes().to_vec(),
			"last_name".as_bytes().to_vec(),
			"date_of_birth".as_bytes().to_vec(),
			"bio".as_bytes().to_vec(),
			"username".as_bytes().to_vec(),
			"website".as_bytes().to_vec(),
			"linkedin".as_bytes().to_vec(),
			"twitter".as_bytes().to_vec(),
			"instagram".as_bytes().to_vec(),
			"telegram".as_bytes().to_vec(),
			"youtube_url".as_bytes().to_vec(),
			"facebook".as_bytes().to_vec(),
			"vision".as_bytes().to_vec(),
			"tag_line".as_bytes().to_vec(),
			0u32, // exchange_volume
			"current_time".as_bytes().to_vec(),
			caller.clone(), // inviter
			"city_token_symbol".as_bytes().to_vec(),
			"industry_token_symbol".as_bytes().to_vec(),
			"personal_token_symbol".as_bytes().to_vec(),
			"project_token_symbol".as_bytes().to_vec(),
			"profile_picture_colour_hex".as_bytes().to_vec(),
			"profile_picture_initials".as_bytes().to_vec()
		);

		// Create asset to be staked
		T::Fungibles::create(asset_id, caller.clone(), true, (1u32).try_into().ok().unwrap())?;

		// Mint assets into the caller
		let _ = T::Fungibles::mint_into(
			asset_id,
			&caller,
			AssetBalanceOf::<T>::from(token_amount).into()
		);

		// Mint Unit asset to the caller. (Used to pay fees)
		let _ = T::Fungibles::mint_into(
			UNIT_ID,
			&caller,
			AssetBalanceOf::<T>::from(token_amount).into()
		);

		// Register asset for staking and transfer funds to the reward pool
		Bond::<T>::register_asset(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			10,
			unbonding_period,
			apr
		)?;

		Bond::<T>::asset_bond_and_stake(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			100_000_000_000
		)?;

		// advance blocks executing on_initialize
		while frame_system::Pallet::<T>::block_number() < (20u32).into() {
			Bond::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
			frame_system::Pallet::<T>::set_block_number(
				frame_system::Pallet::<T>::block_number() + (1u32).into()
			);

			Bond::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		}

		Bond::<T>::claim_asset_rewards(RawOrigin::Signed(caller.clone()).into(), asset_id)?;

		#[extrinsic_call]
		unregister_asset(RawOrigin::Signed(caller.clone()), asset_id.into());

		assert_eq!(RegisteredAssets::<T>::get(asset_id), None);

		Ok(())
	}

	#[benchmark]
	fn register_asset() -> Result<(), BenchmarkError> {
		let token_amount: AssetBalanceOf<T> = (100_000_000_000u128).try_into().ok().unwrap();
		let unit_fee_to_register_asset: AssetBalanceOf<T> = T::AssetRegistrationFee::get();
		let unbonding_period: u32 = 20;
		let apr: u8 = 10;
		let asset_id: u32 = 1000;
		let caller: T::AccountId = whitelisted_caller();
		let _ = T::BenchmarkHelper::create_user(
			caller.clone(),
			"email".as_bytes().to_vec(),
			"first_name".as_bytes().to_vec(),
			"last_name".as_bytes().to_vec(),
			"date_of_birth".as_bytes().to_vec(),
			"bio".as_bytes().to_vec(),
			"username".as_bytes().to_vec(),
			"website".as_bytes().to_vec(),
			"linkedin".as_bytes().to_vec(),
			"twitter".as_bytes().to_vec(),
			"instagram".as_bytes().to_vec(),
			"telegram".as_bytes().to_vec(),
			"youtube_url".as_bytes().to_vec(),
			"facebook".as_bytes().to_vec(),
			"vision".as_bytes().to_vec(),
			"tag_line".as_bytes().to_vec(),
			0u32, // exchange_volume
			"current_time".as_bytes().to_vec(),
			caller.clone(), // inviter
			"city_token_symbol".as_bytes().to_vec(),
			"industry_token_symbol".as_bytes().to_vec(),
			"personal_token_symbol".as_bytes().to_vec(),
			"project_token_symbol".as_bytes().to_vec(),
			"profile_picture_colour_hex".as_bytes().to_vec(),
			"profile_picture_initials".as_bytes().to_vec()
		);

		T::Fungibles::create(asset_id, caller.clone(), true, (1u32).try_into().ok().unwrap())?;

		let _ = T::Fungibles::mint_into(
			asset_id,
			&caller,
			AssetBalanceOf::<T>::from(token_amount).into()
		);

		T::Fungibles::mint_into(UNIT_ID, &caller, unit_fee_to_register_asset * 2)?;

		#[extrinsic_call]
		register_asset(
			RawOrigin::Signed(caller.clone()),
			asset_id.into(),
			token_amount,
			unbonding_period,
			apr
		);

		let asset_info = AssetInfo { registrar: caller, state: AssetState::Active };

		assert_eq!(RegisteredAssets::<T>::get(asset_id), Some(asset_info));
		Ok(())
	}

	#[benchmark]
	fn cancel_unbond() -> Result<(), BenchmarkError> {
		let token_amount: AssetBalanceOf<T> = (1_000_000_000u128).try_into().ok().unwrap();
		let unit_fee_to_register_asset: AssetBalanceOf<T> = T::AssetRegistrationFee::get();
		let unbonding_period: u32 = 20;
		let apr: u8 = 10;
		let asset_id: u32 = 1000;
		let blocks_per_era = T::BlockPerEra::get();

		let caller: T::AccountId = whitelisted_caller();

		// Create profile for the caller
		let _ = T::BenchmarkHelper::create_user(
			caller.clone(),
			"email".as_bytes().to_vec(),
			"first_name".as_bytes().to_vec(),
			"last_name".as_bytes().to_vec(),
			"date_of_birth".as_bytes().to_vec(),
			"bio".as_bytes().to_vec(),
			"username".as_bytes().to_vec(),
			"website".as_bytes().to_vec(),
			"linkedin".as_bytes().to_vec(),
			"twitter".as_bytes().to_vec(),
			"instagram".as_bytes().to_vec(),
			"telegram".as_bytes().to_vec(),
			"youtube_url".as_bytes().to_vec(),
			"facebook".as_bytes().to_vec(),
			"vision".as_bytes().to_vec(),
			"tag_line".as_bytes().to_vec(),
			0u32, // exchange_volume
			"current_time".as_bytes().to_vec(),
			caller.clone(), // inviter
			"city_token_symbol".as_bytes().to_vec(),
			"industry_token_symbol".as_bytes().to_vec(),
			"personal_token_symbol".as_bytes().to_vec(),
			"project_token_symbol".as_bytes().to_vec(),
			"profile_picture_colour_hex".as_bytes().to_vec(),
			"profile_picture_initials".as_bytes().to_vec()
		);

		// Create asset to be staked
		T::Fungibles::create(asset_id, caller.clone(), true, (1u32).try_into().ok().unwrap())?;

		// Mint assets into the caller
		let _ = T::Fungibles::mint_into(
			asset_id,
			&caller,
			AssetBalanceOf::<T>::from(token_amount).into()
		);

		// Mint Unit asset to the caller. (Used to pay fees)
		T::Fungibles::mint_into(UNIT_ID, &caller, unit_fee_to_register_asset * 2)?;

		// Register asset for staking and transfer funds to the reward pool
		Bond::<T>::register_asset(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			token_amount / 2,
			unbonding_period,
			apr
		)?;

		let max_unlocking_items = T::MaxUnlockingChunks::get();

		Bond::<T>::asset_bond_and_stake(
			RawOrigin::Signed(caller.clone()).into(),
			asset_id.into(),
			100 * (max_unlocking_items as u128)
		)?;

		for i in 0..max_unlocking_items {
			Bond::<T>::asset_unbond_and_unstake(
				RawOrigin::Signed(caller.clone()).into(),
				asset_id.into(),
				1
			)?;

			// advance blocks executing on_initialize
			while
				frame_system::Pallet::<T>::block_number() <
				(((i as u32) + 1) * blocks_per_era.saturated_into::<u32>() + 1).into()
			{
				Bond::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
				frame_system::Pallet::<T>::set_block_number(
					frame_system::Pallet::<T>::block_number() + (1u32).into()
				);

				Bond::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
			}
		}

		#[extrinsic_call]
		cancel_unbond(RawOrigin::Signed(caller.clone()), asset_id.into(), max_unlocking_items - 1);

		assert_last_event::<T>(Event::CancelUnbond(caller, asset_id).into());

		Ok(())
	}

	impl_benchmark_test_suite!(Bond, crate::mock::ExternalityBuilder::build(), crate::mock::Test);
}
