#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

pub use pallet::*;


#[cfg(test)]
mod tests;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::{pallet_prelude::*,Blake2_128Concat} ;
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type MaxClaimLength:Get<u32>;
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }


    #[pallet::storage]
    pub type Proofs<T:Config> = StorageMap<
    _, 
    Blake2_128Concat,
    BoundedVec<u8,T::MaxClaimLength>,
    (T::AccountId,BlockNumberFor<T>)>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user has successfully set a new value.
        ClaimCreated {
            /// The new value set.
            owner: T::AccountId,
            claim: BoundedVec<u8,T::MaxClaimLength>,
        },
        ClaimTransfer {
            from: T::AccountId,
            to: T::AccountId,
            claim: BoundedVec<u8,T::MaxClaimLength>,
        },
    }

  
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ProofNotExist,
        OwnError
    }

 
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>, 
            claim: BoundedVec<u8,T::MaxClaimLength>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;
              
            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExist
            );
            
            Proofs::<T>::insert(&claim,(
                who.clone(), frame_system::Pallet::<T>::block_number()
           ));

            // Emit an event.
            Self::deposit_event(
                Event::ClaimCreated {owner: who, claim }
            );

            // Return a successful `DispatchResult`
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn transfer_claim(
            from: OriginFor<T>, 
            claim: BoundedVec<u8,T::MaxClaimLength>,
            to: T::AccountId, 
            ) -> DispatchResult {

            let who = ensure_signed(from)?;

            // key 必须存在
            ensure!(
                Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofNotExist
            );

            // 资产必须属于 from
            let (owner,block_number) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotExist)?;
            ensure!(owner==who, Error::<T>::OwnError);

            // from 不能是 to 
            ensure!(who!= to, Error::<T>::OwnError);
            
            // 转移资产
            Proofs::<T>::insert(&claim,(
                to.clone(), frame_system::Pallet::<T>::block_number()
           ));
           
            // Emit an event.
            Self::deposit_event(
              Event::ClaimTransfer {from:who ,to , claim }
            );

           Ok(())
        }
       
    }
}
