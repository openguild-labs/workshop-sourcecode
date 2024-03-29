#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Burnable, PSP22Mintable)]
#[openbrush::contract]
pub mod psp22_example {
	use openbrush::traits::Storage;

	#[ink(storage)]
	#[derive(Default, Storage)]
	pub struct Contract {
		#[storage_field]
		psp22: psp22::Data,
	}

	impl Contract {
		#[ink(constructor)]
		pub fn new(initial_supply: Balance) -> Self {
			let mut _instance = Self::default();
			psp22::Internal::_mint_to(&mut _instance, Self::env().caller(), initial_supply)
				.expect("Should mint");
			_instance
		}
	}
}
