use super::*;
use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{
		Get,
		LockableCurrency,
		ReservableCurrency,
		fungibles::{ Inspect, roles::Inspect as RolesInspect, Mutate },
		Time,
		tokens::{ Balance, Preservation },
	},
	PalletId,
};
use frame_system::{ ensure_signed, pallet_prelude::* };
use sp_runtime::{
	traits::{ AccountIdConversion, Saturating, Zero },
	FixedU128,
	SaturatedConversion,
	FixedPointNumber,
	FixedPointOperand,
};
use traits::{ profile::{ ProfileInspect }, subaccounts::SubAccounts };
use pallet_profile::Users;
pub use weights::WeightInfo;

pub type EncryptionLength = u32;	
pub type TimeLength = u32;	
pub type SymbolMaxLength = u32;	
pub type EmailMaxLength = u32;
pub type NameLength = u32;	
pub type YoutubeLength = u32;
pub type SocialLength = u32;
pub type HexLength = u32;
pub type InitialsLength = u32;
pub type BioLength = u32;
pub type WebsiteLength = u32;
pub type DobLength = u32;
pub type LocationLength = u32;
pub type TagLineLength = u32;
pub type LanguageCodeLength = u32;
pub type ChatPubkeyLength = u32;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The balance type of this pallet.
	pub type AssetBalanceOf<T> = <T as Config>::AssetBalance;

	pub type AssetIdOf<T> = <T as Config>::AssetId;

	pub type MomentOf<T> = <<T as Config>::Time as Time>::Moment;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The staking balance.
		type Currency: LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>> +
			ReservableCurrency<Self::AccountId>;

		/// Unit staking pallet Id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Rewards pool account id
		#[pallet::constant]
		type RewardsPoolId: Get<PalletId>;

		/// Max number of unique `EraStake` values that can exist.
		/// When stakers claims rewards, they will either keep the number of `EraStake` values the same or they will reduce them by one.
		/// Stakers cannot add an additional `EraStake` value by calling `bond&stake` or `unbond&unstake` if they've reached the max number of values.
		///
		/// This ensures that history doesn't grow indefinitely - if there are too many chunks, stakers should first claim their former rewards
		/// before adding additional `EraStake` values.
		#[pallet::constant]
		type MaxEraStakeValues: Get<u32>;

		/// Minimum amount user must have staked.
		#[pallet::constant]
		type MinimumStakingAmount: Get<AssetBalanceOf<Self>>;

		/// Number of blocks per era.
		#[pallet::constant]
		type BlockPerEra: Get<BlockNumberFor<Self>>;

		/// Max number of unlocking chunks per account Id.
		/// If value is zero, unlocking becomes impossible.
		#[pallet::constant]
		type MaxUnlockingChunks: Get<u32>;

		type AssetId: Member +
			Parameter +
			Copy +
			MaybeSerializeDeserialize +
			MaxEncodedLen +
			Default +
			Zero +
			From<u32>;

		type Fungibles: Inspect<
			Self::AccountId,
			AssetId = Self::AssetId,
			Balance = Self::AssetBalance
		> +
			Mutate<Self::AccountId> +
			RolesInspect<Self::AccountId>; // type to check if an asset exists

		type AssetBalance: Balance +
			FixedPointOperand +
			MaxEncodedLen +
			MaybeSerializeDeserialize +
			TypeInfo;

		/// Amount of UNIT that will be reserved when register a new asset.
		#[pallet::constant]
		type AssetRegistrationFee: Get<AssetBalanceOf<Self>>;

		/// Time provider
		type Time: Time;

		/// Unit asset id.
		#[pallet::constant]
		type UnitId: Get<u32>;

		#[pallet::constant]
		type StringLimit: Get<u32>;

		/// Type to access the profile pallet
		type Profile: ProfileInspect<Self::AccountId, Users<Self::AccountId, Self:: StringLimit>>;

		/// Type to access the sub account pallet
		type SubAccounts: SubAccounts<Self::AccountId>;

		// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// Helper trait for benchmarks.
		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper;
	}

	#[pallet::storage]
	/// Number of eras that need to pass until unstaked funds can be withdrawn.
	/// Current era is always counted as full era (regardless how much blocks are remaining).
	/// When set to `0`, it's equal to having no unbonding period.
	#[pallet::getter(fn asset_unbonding_period)]
	pub type AssetUnbondingPeriod<T> = StorageMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		EraIndex,
		ValueQuery
	>;

	#[pallet::storage]
	/// Interest rate per year for an asset.
	#[pallet::getter(fn asset_apr)]
	pub type AssetAPR<T> = StorageMap<_, Blake2_128Concat, AssetIdOf<T>, u8, ValueQuery>;

	/// Assets where the users can stake if active.
	#[pallet::storage]
	#[pallet::getter(fn registered_assets)]
	pub type RegisteredAssets<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		AssetInfo<T::AccountId>
	>;

	/// Information about the staker for an asset.
	#[pallet::storage]
	#[pallet::getter(fn asset_ledger)]
	pub type AssetLedger<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		Blake2_128Concat,
		T::AccountId,
		AccountLedger<AssetBalanceOf<T>>,
		ValueQuery
	>;

	/// The current era index.
	#[pallet::storage]
	#[pallet::getter(fn current_era)]
	pub type CurrentEra<T> = StorageValue<_, EraIndex, ValueQuery>;

	#[pallet::storage]
	/// General information about an era like TVL, total staked value for an asset.
	#[pallet::getter(fn asset_general_era_info)]
	pub type AssetGeneralEraInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		Blake2_128Concat,
		EraIndex,
		EraInfo<AssetBalanceOf<T>>
	>;

	/// Info about stakers stakes.
	#[pallet::storage]
	#[pallet::getter(fn asset_staker_info)]
	pub type AssetGeneralStakerInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		Blake2_128Concat,
		T::AccountId,
		StakerInfo<AssetBalanceOf<T>, MomentOf<T>>,
		ValueQuery
	>;

	/// Stores the block number of when the next era starts
	#[pallet::storage]
	#[pallet::getter(fn next_era_starting_block)]
	pub type NextEraStartingBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

	#[pallet::type_value]
	pub fn ForceEraOnEmpty() -> Forcing {
		Forcing::NotForcing
	}

	/// Mode of era forcing.
	#[pallet::storage]
	#[pallet::getter(fn force_era)]
	pub type ForceEra<T> = StorageValue<_, Forcing, ValueQuery, ForceEraOnEmpty>;

	/* ***************************
        START TRAIL DATA 
        ***************************
    */
	// Map to save trail data for users rewards
	#[pallet::storage]
	#[pallet::getter(fn user_rewards)]
	pub type UserRewards<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, AssetIdOf<T>>,
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Blake2_128Concat, EraIndex>,
		),
		RewardData<AssetBalanceOf<T>, BlockNumberFor<T>, T::AccountId, MomentOf<T>>
	>;

	#[pallet::storage]
	#[pallet::getter(fn user_actions)]
	pub type UserActions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		Blake2_128Concat,
		T::AccountId,
		Vec<ActionInfo<AssetBalanceOf<T>, BlockNumberFor<T>, MomentOf<T>>>,
		ValueQuery
	>;

	/* ***************************
        END TRAIL DATA 
        ***************************
    */

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Account has staked some funds in an asset.
		AssetBondAndStake(T::AccountId, AssetIdOf<T>, AssetBalanceOf<T>),
		/// Account has unbonded & unstaked some funds in an asset. Unbonding process begins.
		AssetUnbondAndUnstake(T::AccountId, AssetIdOf<T>, AssetBalanceOf<T>),
		/// New staking era.
		NewEra(EraIndex),
		/// Account has withdrawn unbonded funds in an asset.
		WithdrawnFromAsset(T::AccountId, AssetIdOf<T>, AssetBalanceOf<T>),
		/// Reward paid to staker for staking an asset.
		AssetReward(T::AccountId, AssetIdOf<T>, EraIndex, AssetBalanceOf<T>),
		/// New asset registered.
		AssetRegistered(AssetIdOf<T>, AssetBalanceOf<T>, EraIndex, u8),
		/// Asset unregistered.
		AssetUnregistered(AssetIdOf<T>),
		/// Staking is set to unactive because cannot paid rewards.
		StakingSttoped(AssetIdOf<T>),
		/// Cancel unbonding chunk
		CancelUnbond(T::AccountId, AssetIdOf<T>),
	}
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Can not stake with value less than minimum staking value
		InsufficientValue,
		/// Can not stake with zero value.
		StakingWithNoValue,
		/// Too many active `EraStake` values for staker.
		/// Claim existing rewards to fix this problem.
		TooManyEraStakeValues,
		/// If this error its emitted it means a bug.
		UnexpectedStakeInfoEra,
		/// Unstaking with zero value
		UnstakingWithNoValue,
		/// User has too many unlocking chunks. Withdraw the existing chunks if possible
		/// or wait for current chunks to complete unlocking process to withdraw them.
		TooManyUnlockingChunks,
		/// User isn't staked.
		NoStake,
		/// There are no previously unbonded funds that can be unstaked and withdrawn.
		NothingToWithdraw,
		/// Era parameter is out of bounds
		EraOutOfBounds,
		/// APR should be greater than zero
		InvalidAPR,
		/// The asset does not exist
		AssetDoesNotExist,
		/// Cannot register an asset for staking twice
		AssetAlreadyRegistered,
		/// The caller is not the owner or admin of the asset
		NotAuthorized,
		/// Cannot register an asset without giving rewards to the pallet
		InvalidRewardAmount,
		/// The asset is not registered
		AssetNotRegistered,
		/// The staking for the assets its still active
		StakingActive,
		/// The staking for the assets its not active
		StakingNotActive,
		/// There is no unlocking chunks for the user
		NoUnbondingInfo,
		/// Index out of bonds
		InvalidIndex,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: BlockNumberFor<T>) -> Weight {
			let force_new_era = Self::force_era().eq(&Forcing::ForceNew);
			let previous_era = Self::current_era();
			let next_era_starting_block = Self::next_era_starting_block();

			// Value is compared to 1 since genesis block is ignored
			if now >= next_era_starting_block || force_new_era || previous_era.is_zero() {
				let blocks_per_era = T::BlockPerEra::get();
				let next_era = previous_era + 1;
				CurrentEra::<T>::put(next_era);

				NextEraStartingBlock::<T>::put(now + blocks_per_era);

				Self::rotate_staking_info(previous_era);

				if force_new_era {
					ForceEra::<T>::put(Forcing::NotForcing);
				}

				Self::deposit_event(Event::<T>::NewEra(next_era));

				T::DbWeight::get().reads_writes(5, 3)
			} else {
				T::DbWeight::get().reads(4)
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call(weight(<T as Config>::WeightInfo))]
	impl<T: Config> Pallet<T> {
		/// The origin can stake balance for a particular asset.
		///
		/// The origin must be Signed.
		///
		/// The asset should be registered for staking.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the asset to stake the funds.
		/// - `amount`: The amount of tokens to stake.
		///
		/// Emits `AssetBondAndStake` event when successful.
		///
		#[pallet::call_index(0)]
		pub fn asset_bond_and_stake(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			#[pallet::compact] amount: AssetBalanceOf<T>
		) -> DispatchResultWithPostInfo {
			let mut staker = ensure_signed(origin)?;
			// Mutate the origin to transfer funds from the main account
			staker = T::SubAccounts::get_main_account(staker)?;

			//ensure that the asset is registered an active
			if let Some(asset) = Self::registered_assets(&asset_id) {
				ensure!(asset.state == AssetState::Active, Error::<T>::StakingNotActive);
			} else {
				return Err(Error::<T>::AssetNotRegistered.into());
			}

			// Get the staking ledger for the asset or create an entry if it doesn't exist.
			let mut ledger = Self::asset_ledger(&asset_id, &staker);
			let available_balance = T::Fungibles::balance(asset_id.clone(), &staker);
			let value_to_stake = amount.min(available_balance);
			ensure!(value_to_stake > Zero::zero(), Error::<T>::StakingWithNoValue);

			let current_era = Self::current_era();
			let mut staker_info = Self::asset_staker_info(&asset_id, &staker);

			let now = T::Time::now();

			Self::asset_stake(&mut staker_info, value_to_stake.clone(), current_era, now.clone())?;

			ledger.locked = ledger.locked.saturating_add(value_to_stake);

			// Update storage
			AssetGeneralEraInfo::<T>::mutate(&asset_id, &current_era, |value| {
				if let Some(x) = value {
					x.staked = x.staked.saturating_add(value_to_stake);
					x.locked = x.locked.saturating_add(value_to_stake);
				}
			});

			Self::update_asset_ledger(&staker, ledger, asset_id.clone(), value_to_stake)?;
			Self::update_asset_staker_info(&staker, staker_info, asset_id);

			//Trail info
			UserActions::<T>::mutate(&asset_id, &staker, |vec| {
				vec.push(ActionInfo {
					action: Action::Stake,
					amount: value_to_stake,
					time: <frame_system::Pallet<T>>::block_number(),
					index: vec.len() as u32,
					timestamp: now,
				})
			});
			Self::deposit_event(Event::<T>::AssetBondAndStake(staker, asset_id, value_to_stake));

			Ok(().into())
		}

		/// The origin can start unbonding process and unstake balance for a particular asset.
		///
		/// The user should have tokens staked.
		///
		/// The unstaked amount will no longer be eligible for rewards but still won't be unlocked.
		/// User needs to wait for the unbonding period to finish before being able to withdraw
		/// the funds via `withdraw_unbonded` call.
		///
		/// In case remaining staked balance is below minimum staking amount,
		/// entire stake will be unstaked.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the asset to unstake the funds.
		/// - `amount`: The amount of tokens to start the unbonding period.
		///
		/// Emits `AssetUnbondAndUnstake` event when successful.
		///
		#[pallet::call_index(1)]
		pub fn asset_unbond_and_unstake(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			#[pallet::compact] amount: AssetBalanceOf<T>
		) -> DispatchResultWithPostInfo {
			let mut staker = ensure_signed(origin)?;
			// Mutate the origin to transfer funds from the main account
			staker = T::SubAccounts::get_main_account(staker)?;

			ensure!(amount > Zero::zero(), Error::<T>::UnstakingWithNoValue);

			let now = T::Time::now();

			let current_era = Self::current_era();
			let mut staker_info = Self::asset_staker_info(&asset_id, &staker);

			let value_to_unstake = Self::asset_unstake(
				&mut staker_info,
				amount,
				current_era,
				now.clone()
			)?;

			// Update the chunks and write them to storage
			let mut ledger = Self::asset_ledger(&asset_id, &staker);
			ledger.unbonding_info.add(UnlockingChunk {
				amount: value_to_unstake,
				unlock_era: current_era + AssetUnbondingPeriod::<T>::get(&asset_id),
			});

			// This should be done AFTER insertion since it's possible for chunks to merge
			ensure!(
				ledger.unbonding_info.len() <= T::MaxUnlockingChunks::get(),
				Error::<T>::TooManyUnlockingChunks
			);

			AssetLedger::<T>::insert(asset_id.clone(), &staker, ledger);

			// Update total staked value in era for the asset.
			AssetGeneralEraInfo::<T>::mutate(&asset_id, &current_era, |value| {
				if let Some(x) = value {
					x.staked = x.staked.saturating_sub(value_to_unstake);
				}
			});

			Self::update_asset_staker_info(&staker, staker_info, asset_id.clone());

			Self::deposit_event(
				Event::<T>::AssetUnbondAndUnstake(staker.clone(), asset_id, value_to_unstake)
			);

			//Trail info
			UserActions::<T>::mutate(&asset_id, &staker, |vec| {
				vec.push(ActionInfo {
					action: Action::Unstake,
					amount: value_to_unstake,
					time: <frame_system::Pallet<T>>::block_number(),
					index: vec.len() as u32,
					timestamp: now,
				})
			});

			Ok(().into())
		}

		/// The origin can withdraw all funds that have completed the unbonding process for a particular asset.
		///
		/// If there are unbonding chunks which will be fully unbonded in future eras,
		/// they will remain and can be withdrawn later.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the asset to withdraw the unbonded the funds.
		///
		/// Emits `WithdrawnFromAsset` event when successful.
		///
		#[pallet::call_index(2)]
		pub fn withdraw_unbonded_asset(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>
		) -> DispatchResultWithPostInfo {
			let mut staker = ensure_signed(origin)?;
			// Mutate the origin to transfer funds from the main account
			staker = T::SubAccounts::get_main_account(staker)?;

			let now = T::Time::now();

			let mut ledger = Self::asset_ledger(&asset_id, &staker);
			let current_era = Self::current_era();

			let (valid_chunks, future_chunks) = ledger.unbonding_info.partition(current_era);
			let withdraw_amount = valid_chunks.sum();

			ensure!(!withdraw_amount.is_zero(), Error::<T>::NothingToWithdraw);

			// Get the staking ledger and update it
			ledger.locked = ledger.locked.saturating_sub(withdraw_amount);
			ledger.unbonding_info = future_chunks;

			if ledger.is_empty() {
				AssetLedger::<T>::remove(asset_id, &staker);
			} else {
				AssetLedger::<T>::insert(asset_id, &staker, ledger);
			}

			T::Fungibles::transfer(
				asset_id.clone(),
				&Self::account_id(),
				&staker,
				withdraw_amount,
				Preservation::Expendable
			)?;

			AssetGeneralEraInfo::<T>::mutate(&asset_id, &current_era, |value| {
				if let Some(x) = value {
					x.locked = x.locked.saturating_sub(withdraw_amount);
				}
			});

			Self::deposit_event(
				Event::<T>::WithdrawnFromAsset(staker.clone(), asset_id, withdraw_amount)
			);

			//Trail info
			UserActions::<T>::mutate(&asset_id, &staker, |vec| {
				vec.push(ActionInfo {
					action: Action::WithdrawUnbonded,
					amount: withdraw_amount,
					time: <frame_system::Pallet<T>>::block_number(),
					index: vec.len() as u32,
					timestamp: now,
				})
			});

			Ok(().into())
		}

		/// The origin can claim earned staker rewards for the oldest unclaimed era for a particular asset.
		///
		/// In order to claim multiple eras, this call has to be called multiple times.
		///
		/// The rewards are always immediately re-staked.
		///
		/// The users can only claim rewards of the 7 past days, rewards older than that will be discarded.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the asset to claim the rewards.
		///
		/// Emits `AssetBondAndStake` and `AssetReward` events when successful.
		///
		#[pallet::call_index(3)]
		pub fn claim_asset_rewards(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>
		) -> DispatchResultWithPostInfo {
			let mut staker = ensure_signed(origin)?;
			// Mutate the origin to transfer funds from the main account
			staker = T::SubAccounts::get_main_account(staker)?;

			let now = T::Time::now();

			let current_era = Self::current_era();

			// Ensure we have something to claim
			let mut staker_info = Self::asset_staker_info(&asset_id, &staker);
			let (era, staked) = staker_info.claim(current_era.clone(), now.clone());

			ensure!(staked > Zero::zero(), Error::<T>::NoStake);

			ensure!(era < current_era, Error::<T>::EraOutOfBounds);

			let reward = Self::compute_asset_reward(asset_id.clone(), staked.clone());

			if reward > T::Fungibles::balance(asset_id.clone(), &Self::reward_pool()) {
				RegisteredAssets::<T>::mutate(&asset_id, |value| {
					if let Some(x) = value {
						x.state = AssetState::Inactive;
					}
				});
				Self::deposit_event(Event::<T>::StakingSttoped(asset_id));
				return Ok(().into());
			}

			let mut ledger = Self::asset_ledger(&asset_id, &staker);

			let should_restake_reward = Self::should_restake_asset_reward(
				ledger.reward_destination,
				staker_info.latest_staked_value()
			);

			if should_restake_reward {
				staker_info
					.stake(current_era, reward.clone(), now.clone())
					.map_err(|_| Error::<T>::UnexpectedStakeInfoEra)?;

				// Restaking will, in the worst case, remove one, and add one record,
				// so it's fine if the vector is full
				ensure!(
					staker_info.len() <= T::MaxEraStakeValues::get(),
					Error::<T>::TooManyEraStakeValues
				);
			}

			if should_restake_reward {
				ledger.locked = ledger.locked.saturating_add(reward);

				AssetLedger::<T>::insert(asset_id.clone(), staker.clone(), ledger);

				// Update storage
				AssetGeneralEraInfo::<T>::mutate(&asset_id, &current_era, |value| {
					if let Some(x) = value {
						x.staked = x.staked.saturating_add(reward);
						x.locked = x.locked.saturating_add(reward);
					}
				});

				Self::deposit_event(
					Event::<T>::AssetBondAndStake(staker.clone(), asset_id.clone(), reward)
				);

				//Trail info
				UserActions::<T>::mutate(&asset_id, &staker, |vec| {
					vec.push(ActionInfo {
						action: Action::Stake,
						amount: reward,
						time: <frame_system::Pallet<T>>::block_number(),
						index: vec.len() as u32,
						timestamp: now,
					})
				});

				T::Fungibles::transfer(
					asset_id.clone(),
					&Self::reward_pool(),
					&Self::account_id(),
					reward.clone(),
					Preservation::Expendable
				)?;
			} else {
				T::Fungibles::transfer(
					asset_id.clone(),
					&Self::reward_pool(),
					&staker,
					reward.clone(),
					Preservation::Expendable
				)?;
			}

			Self::update_asset_staker_info(&staker, staker_info, asset_id.clone());
			Self::deposit_event(Event::<T>::AssetReward(staker.clone(), asset_id, era, reward));

			// Add trail data
			UserRewards::<T>::insert((asset_id, staker.clone(), era), RewardData {
				time: <frame_system::Pallet<T>>::block_number(),
				amount: reward,
				era: current_era,
				account: staker.clone(),
				timestamp: now,
			});
			//Trail info
			UserActions::<T>::mutate(&asset_id, &staker, |vec| {
				vec.push(ActionInfo {
					action: Action::RewardsClaimed,
					amount: reward,
					time: <frame_system::Pallet<T>>::block_number(),
					index: vec.len() as u32,
					timestamp: now,
				})
			});

			Ok(().into())
		}

		/// The origin can register a particular asset for staking.
		///
		/// The origin must be Signed and the sender should be the owner of the asset.
		///
		/// The sender will send tokens to the reward pool in order for stakers to claim their rewards.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the asset to register.
		/// - `reward_amount`: The amount of tokens that the owner will send to the reward pool.
		/// - `unbonding_period`: The amount of eras that the users should wait to withdraw the tokens.
		/// - `asset_apr`: The fixed annual percentage rate that the pallet will pay to stakers.
		///
		/// Emits `AssetRegistered` event when successful.
		///
		#[pallet::call_index(4)]
		pub fn register_asset(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			#[pallet::compact] reward_amount: AssetBalanceOf<T>,
			unbonding_period: EraIndex,
			asset_apr: u8
		) -> DispatchResultWithPostInfo {
			let mut sender = ensure_signed(origin)?;
			// Mutate the origin to transfer funds from the main account
			sender = T::SubAccounts::get_main_account(sender)?;

			// check if the asset its already registered
			ensure!(
				!RegisteredAssets::<T>::contains_key(&asset_id),
				Error::<T>::AssetAlreadyRegistered
			);
			// check if sender has permissions to register the asset
			ensure!(
				T::Fungibles::owner(asset_id.clone()) == Some(sender.clone()) ||
					T::Fungibles::admin(asset_id.clone()) == Some(sender.clone()),
				Error::<T>::NotAuthorized
			);
			// check if reward_amount is greater than 0
			ensure!(reward_amount > Zero::zero(), Error::<T>::InvalidRewardAmount);
			// check that apr is greater than 0 and below 30%
			ensure!(asset_apr > Zero::zero() && asset_apr < 30u8, Error::<T>::InvalidAPR);

			// reserve UNIT to register the asset
			T::Fungibles::transfer(
				T::UnitId::get().into(),
				&sender,
				&Self::account_id(),
				T::AssetRegistrationFee::get(),
				Preservation::Expendable
			)?;

			//TODO: transfer assets from the bank to the reward pool
			// transfer reward_amount to the pallet account
			T::Fungibles::transfer(
				asset_id.clone(),
				&sender,
				&Self::reward_pool(),
				reward_amount,
				Preservation::Expendable
			)?;

			// update storage
			RegisteredAssets::<T>::insert(&asset_id, AssetInfo {
				registrar: sender,
				state: AssetState::Active,
			});

			AssetUnbondingPeriod::<T>::insert(&asset_id, unbonding_period.clone());

			let current_era = Self::current_era();
			AssetGeneralEraInfo::<T>::insert(&asset_id, &current_era, EraInfo {
				staked: Zero::zero(),
				locked: Zero::zero(),
			});

			AssetAPR::<T>::insert(&asset_id, asset_apr.clone());
			Self::deposit_event(
				Event::<T>::AssetRegistered(asset_id, reward_amount, unbonding_period, asset_apr)
			);
			Ok(().into())
		}

		/// The origin can unregister a particular asset from staking.
		///
		/// The origin must be Signed.
		///
		/// The state of the asset should be inactive. This happens when the reward pool run out of tokens.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the asset to unregister.
		///
		/// Emits `AssetUnregistered` event when successful.
		///
		#[pallet::call_index(5)]
		pub fn unregister_asset(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;

			if let Some(asset_info) = RegisteredAssets::<T>::get(&asset_id) {
				ensure!(asset_info.state == AssetState::Inactive, Error::<T>::StakingActive);

				// unreserve UNIT
				T::Fungibles::transfer(
					T::UnitId::get().into(),
					&Self::account_id(),
					&asset_info.registrar,
					T::AssetRegistrationFee::get(),
					Preservation::Expendable
				)?;

				// update storage
				RegisteredAssets::<T>::remove(&asset_id);
				AssetUnbondingPeriod::<T>::remove(&asset_id);

				Self::deposit_event(Event::<T>::AssetUnregistered(asset_id));
				Ok(().into())
			} else {
				Err(Error::<T>::AssetNotRegistered.into())
			}
		}

		/// The origin can cancel the unbonding period for their tokens for an asset.
		///
		/// The origin must be Signed.
		///
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the asset to unregister.
		/// - `chunk_index`: The identifier of the unlocking chunk to be staked again.
		///
		/// Emits `CancelUnbond` event when successful.
		///
		/// Weight: `O(1)` TODO: Add correct weight
		#[pallet::call_index(6)]
		pub fn cancel_unbond(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			chunk_index: u32
		) -> DispatchResultWithPostInfo {
			let mut staker = ensure_signed(origin)?;
			// Mutate the origin to transfer funds from the main account
			staker = T::SubAccounts::get_main_account(staker)?;

			let now = T::Time::now();

			let mut ledger = AssetLedger::<T>::get(asset_id.clone(), staker.clone());
			ensure!(
				!ledger.unbonding_info.unlocking_chunks.is_empty(),
				Error::<T>::NoUnbondingInfo
			);

			ensure!(
				ledger.unbonding_info.unlocking_chunks.len() > chunk_index.try_into().unwrap(),
				Error::<T>::InvalidIndex
			);

			let unlocking_chunk = ledger.unbonding_info.unlocking_chunks.remove(
				chunk_index.try_into().unwrap()
			);

			let current_era = Self::current_era();
			let mut staker_info = Self::asset_staker_info(&asset_id, &staker);

			Self::asset_stake(&mut staker_info, unlocking_chunk.amount.clone(), current_era, now)?;

			// Update storage
			AssetGeneralEraInfo::<T>::mutate(&asset_id, &current_era, |value| {
				if let Some(x) = value {
					x.staked = x.staked.saturating_add(unlocking_chunk.amount);
				}
			});

			if ledger.is_empty() {
				AssetLedger::<T>::remove(asset_id, &staker);
			} else {
				AssetLedger::<T>::insert(asset_id, &staker, ledger);
			}

			Self::update_asset_staker_info(&staker, staker_info, asset_id);

			Self::deposit_event(
				Event::<T>::AssetBondAndStake(staker.clone(), asset_id, unlocking_chunk.amount)
			);

			Self::deposit_event(Event::<T>::CancelUnbond(staker.clone(), asset_id));

			//Trail info
			UserActions::<T>::mutate(&asset_id, &staker, |vec| {
				vec.push(ActionInfo {
					action: Action::CancelUnbond,
					amount: unlocking_chunk.amount,
					time: <frame_system::Pallet<T>>::block_number(),
					index: vec.len() as u32,
					timestamp: now,
				})
			});

			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Get AccountId assigned to the pallet.
	pub(crate) fn account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}

	/// Get reward pool account assigned to the pallet.
	pub(crate) fn reward_pool() -> T::AccountId {
		T::RewardsPoolId::get().into_account_truncating()
	}

	/// An utility method used to stake specified amount.
	///
	/// `StakerInfo` is provided and all checks are made to ensure that it's possible to
	/// complete staking operation.
	///
	/// # Arguments
	///
	/// * `staker_info` - info about staker's stakes up to current moment
	/// * `value` - value which is being bonded & staked
	/// * `current_era` - current era
	///
	/// # Returns
	///
	/// If stake operation was successful, given structs are properly modified.
	/// If not, an error is returned and structs are left in an undefined state.
	///
	fn asset_stake(
		staker_info: &mut StakerInfo<AssetBalanceOf<T>, MomentOf<T>>,
		value: AssetBalanceOf<T>,
		current_era: EraIndex,
		now: MomentOf<T>
	) -> Result<(), Error<T>> {
		staker_info.stake(current_era, value, now).map_err(|_| Error::<T>::UnexpectedStakeInfoEra)?;
		ensure!(
			// One spot should remain for compounding reward claim call
			staker_info.len() < T::MaxEraStakeValues::get(),
			Error::<T>::TooManyEraStakeValues
		);
		ensure!(
			staker_info.latest_staked_value() >= T::MinimumStakingAmount::get(),
			Error::<T>::InsufficientValue
		);

		return Ok(());
	}

	/// An utility method used to unstake specified amount.
	///
	/// The amount unstaked can be different in case staked amount would fall bellow `MinimumStakingAmount`.
	/// In that case, entire staked amount will be unstaked.
	///
	/// `StakerInfo` is provided and all checks are made to ensure that it's possible to
	/// complete unstake operation.
	///
	/// # Arguments
	///
	/// * `staker_info` - info about staker's stakes up to current moment
	/// * `value` - value which should be unstaked
	/// * `current_era` - current era
	///
	/// # Returns
	///
	/// If unstake operation was successful, given structs are properly modified and total unstaked value is returned.
	/// If not, an error is returned and structs are left in an undefined state.
	///
	fn asset_unstake(
		staker_info: &mut StakerInfo<AssetBalanceOf<T>, MomentOf<T>>,
		value: AssetBalanceOf<T>,
		current_era: EraIndex,
		now: MomentOf<T>
	) -> Result<AssetBalanceOf<T>, Error<T>> {
		let staked_value = staker_info.latest_staked_value();
		ensure!(staked_value > Zero::zero(), Error::<T>::NoStake);

		// Calculate the value which will be unstaked.
		let remaining = staked_value.saturating_sub(value);
		let value_to_unstake = if remaining < T::MinimumStakingAmount::get() {
			staked_value
		} else {
			value
		};

		// Sanity check
		ensure!(value_to_unstake > Zero::zero(), Error::<T>::UnstakingWithNoValue);

		staker_info
			.unstake(current_era, value_to_unstake, now)
			.map_err(|_| Error::<T>::UnexpectedStakeInfoEra)?;
		ensure!(
			// One spot should remain for compounding reward claim call
			staker_info.len() < T::MaxEraStakeValues::get(),
			Error::<T>::TooManyEraStakeValues
		);

		return Ok(value_to_unstake);
	}

	/// Update the staked info for the asset and transfer funds.
	fn update_asset_ledger(
		staker: &T::AccountId,
		ledger: AccountLedger<AssetBalanceOf<T>>,
		asset_id: AssetIdOf<T>,
		amount_staked: AssetBalanceOf<T>
	) -> Result<(), DispatchError> {
		if ledger.is_empty() {
			AssetLedger::<T>::remove(asset_id, &staker);
			T::Fungibles::transfer(
				asset_id.clone(),
				&Self::account_id(),
				staker,
				amount_staked,
				Preservation::Expendable
			)?;
			Ok(())
		} else {
			T::Fungibles::transfer(
				asset_id.clone(),
				staker,
				&Self::account_id(),
				amount_staked,
				Preservation::Expendable
			)?;
			AssetLedger::<T>::insert(asset_id, staker, ledger);
			Ok(())
		}
	}

	/// Update the staker info for the staker.
	/// If staker_info is empty, remove it from the DB. Otherwise, store it.
	fn update_asset_staker_info(
		staker: &T::AccountId,
		staker_info: StakerInfo<AssetBalanceOf<T>, MomentOf<T>>,
		asset_id: AssetIdOf<T>
	) {
		if staker_info.is_empty() {
			AssetGeneralStakerInfo::<T>::remove(asset_id, staker)
		} else {
			AssetGeneralStakerInfo::<T>::insert(asset_id, staker, staker_info)
		}
	}

	/// Returns the reward for the staked amount
	pub(crate) fn compute_asset_reward(
		asset_id: AssetIdOf<T>,
		staked_value: AssetBalanceOf<T>
	) -> AssetBalanceOf<T> {
		let apr = AssetAPR::<T>::get(&asset_id);
		let interest = FixedU128::from_rational(apr.into(), 36500u128);
		let new_balance: u128 = interest.saturating_mul_int(
			TryInto::<u128>::try_into(staked_value).ok().unwrap().into()
		);
		new_balance.saturated_into::<AssetBalanceOf<T>>()
	}

	pub(crate) fn should_restake_asset_reward(
		reward_destination: RewardDestination,
		latest_staked_value: AssetBalanceOf<T>
	) -> bool {
		reward_destination == RewardDestination::StakeBalance && latest_staked_value > Zero::zero()
	}

	/// Used to copy the asset staking info from the ending era over to the next era.
	fn rotate_staking_info(current_era: EraIndex) -> Weight {
		let next_era = current_era + 1;

		let mut consumed_weight = Weight::zero();

		for (asset_id, asset_info) in RegisteredAssets::<T>::iter() {
			// Ignore asset if staking is unactive
			consumed_weight = consumed_weight.saturating_add(T::DbWeight::get().reads(1));
			if let AssetState::Inactive = asset_info.state {
				continue;
			}

			// Copy data from era `X` to era `X + 1`
			if let Some(asset_era_info) = Self::asset_general_era_info(&asset_id, current_era) {
				AssetGeneralEraInfo::<T>::insert(&asset_id, next_era, asset_era_info);

				consumed_weight = consumed_weight.saturating_add(
					T::DbWeight::get().reads_writes(1, 1)
				);
			} else {
				consumed_weight = consumed_weight.saturating_add(T::DbWeight::get().reads(1));
			}
		}
		consumed_weight
	}
}
