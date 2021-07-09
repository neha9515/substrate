#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_system::Config;
use frame_system::Module;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage

decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	
	trait Store for Module<T: Config> as BookInfo {
		StoredValue get(fn stored_value): u32;
		StoredAccount get(fn stored_account): T::AccountId;
	}
}
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
	trait #[weight = 10_000]
	fn set_value(origin, value:u32)->DispatchResult
	{
	let _=ensure_signed(origin)?;
	StoredValue::put(value);
	ok(())
	}
	fn set_account(origin)-> DispatchResult{
	let who= ensure_signed(origin)?;
	<StoredAccount<T>>::put($who);				
	Ok(())
	}
	}
			
}

