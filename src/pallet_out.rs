#![cfg_attr(not(feature = "std"), no_std)]
#[doc = " Edit this file to define custom logic or remove it if it is not needed."]
#[doc = " Learn more about FRAME and the core library of Substrate FRAME pallets:"]
#[doc = " <https://docs.substrate.io/reference/frame-pallets/>"]
pub use pallet::*;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::*;
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    #[pallet::pallet]
    pub struct Pallet<T>(_);
    struct Student {
        age: i32,
        name: String,
    }
    struct MyStruct {
        field1: i32,
        field2: String,
    }
    #[doc = " Configure the pallet by specifying the parameters and types on which it depends."]
    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[doc = " Because this pallet emits events, it depends on the runtime's definition of an event."]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[doc = " Type representing the weight of this pallet"]
        type WeightInfo: WeightInfo;
    }
    #[pallet::storage]
    # [pallet :: getter (fn something)]
    pub type Something<T> = StorageValue<_, u32>;
    #[pallet::event]
    # [pallet :: generate_deposit (pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        #[doc = " Event documentation should end with an array that provides descriptive names for event"]
        #[doc = " parameters. [something, who]"]
        SomethingStored { something: u32, who: T::AccountId },
    }
    #[pallet::error]
    pub enum Error<T> {
        #[doc = " Error names should be descriptive."]
        NoneValue,
        #[doc = " Errors should have helpful documentation associated with them."]
        StorageOverflow,
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[doc = " An example dispatchable that takes a singles value as a parameter, writes the value to"]
        #[doc = " storage and emits an event. This function must be dispatched by a signed extrinsic."]
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            <Something<T>>::put(something);
            Self::deposit_event(Event::SomethingStored { something, who });
            Ok(())
        }
        #[doc = " An example dispatchable that may throw a custom error."]
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::cause_error())]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            match <Something<T>>::get() {
                None => return Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    <Something<T>>::put(new);
                    Ok(())
                }
            }
        }
    }
}
