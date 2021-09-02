#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use primitives::v1::Id as ParaId;
use runtime_common::traits::Registrar;
use runtime_parachains::{paras, ParaLifeCycle};

//#[cfg(test)]
//mod mock;

//#[cfg(test)]
//mod tests;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

// Types of slots which paras can be registered to
#[derive(Default, Encode, Decode)]
enum SlotType {
    LongTerm,
    ShortTerm,
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Parachain registrar type. We use this to ensure that only the manager of a para
        /// is able to register it for its slots.
        type Registrar: Registrar<AccountId = Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn short_term_slots)]
    // Number of short term slots. This is the number of parathreads that will be upgraded to
    // parachains every rotation.
    pub(super) type ShortTermSlots<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn short_term_slot_duration)]
    // Number of leases assigned to a short term slot
    pub(super) type ShortTermSlotDuration<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn long_term_slot_duration)]
    // Number of leases assigned to a long term slot
    pub(super) type ShortTermSlotDuration<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn long_term_paras)]
    // Paras registered for a long term slot
    pub(super) type LongTermParas<T> = StorageValue<_, Vec<ParaId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn short_term_paras)]
    // Paras registered for a short term slot
    pub(super) type ShortTermParas<T> = StorageValue<_, Vec<ParaId>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A para has been registered for long term slots.\[ParaId\]
        LongTermParaRegistered(ParaId),
        /// A para has been registered for short term slots.\[ParaId\]
        ShortTermParaRegistered(ParaId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The specified parachain or parathread is not registered.
        ParaDoesntExist,
        /// The specified parachain or parathread is already registered.
        ParaAlreadyExists,
        /// Not a parathread.
        NotParathread,
        /// Not a parachain.
        NotParachain,
        /// Invalid para index.
        InvalidParaId,
        /// The origin of this call is invalid.
        InvalidOrigin,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a parathread as a long term para.
        /// Fails if para is not registered as parathread.
        ///
        ///  ## Arguments
        ///  - `origin`: Must be called by `root` origin.
        ///  - `id`: The para ID.
        ///
        /// ## Events
        /// `LongTermParaRegistered` event is emitted in case of success.
        ///
        /// Root can always do this.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn register_long_term_para(
            origin: OriginFor<T>,
            id: ParaId
        ) -> DispatchResult {
            // Long term paras will be registered by root
            ensure_root(origin)?;

            Self::do_register(id, SlotType::LongTerm)
        }

        /// Register a parathread as a short term para.
        /// Fails if para is not registered as parathread.
        ///
        /// ## Arguments
        /// - `origin`: Must be called by `Signed` origin.
        /// - `id` The para ID. Must be owned/managed by the `origin` signing account.
        ///
        /// ## Events
        /// `ShortTermParaRegistered` event is emitted in case of success.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn register_short_term_para(
            origin: OriginFor<T>,
            id: ParaId
        ) -> DispatchResult {
            // Check for signed origin.
            let who = ensure_signed(origin)?;
            let manager = T::Registrar::manager_of(id).ok_or(Error::<T>::InvalidParaId);
            ensure!(who == manager, Error::<T>::InvalidOrigin);

            Self::do_register(id, SlotType::ShortTerm)
        }
    }
}

impl<T: Config> Pallet<T> {
    fn do_register(
        id: ParaId,
        slot_type: SlotType
    ) -> DispatchResult {
        // Para must be a parathread
        ensure!(
            paras::Pallet::<T>::lifecycle(id) == Some(ParaLifeCycle::Parathread),
            Error::<T>::NotParathread,
        );

        // Insert id according to the slot type
        match slot_type {
            SlotType::LongTerm => {
                <LongTermParas<T>>::insert(id);
                Self::deposit_event(Event::<T>::LongTermParaRegistered(id));
                Ok(())
            }
            SlotType::ShortTerm => {
                <ShortTermParas<T>>::insert(id);
                Self::deposit_event(Event::<T>::ShortTermParaRegistered(id));
                Ok(())
            }
        }
    }
}
