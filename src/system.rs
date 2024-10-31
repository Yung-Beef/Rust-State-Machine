use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + Copy;
	type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Pallet { block_number: Zero::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number = self.block_number + One::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let prev = *self.nonce.get(who).unwrap_or(&Zero::zero());
		self.nonce.insert(who.clone(), prev + One::one());
	}
}

#[cfg(test)]
mod test {
	struct TestConfig {
		block_number: u32,
		nonce: super::BTreeMap<String, u32>,
	}

	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut sys = super::Pallet::<TestConfig>::new();

		sys.inc_block_number();
		sys.inc_nonce(&"Alice".to_string());

		assert_eq!(sys.block_number(), 1);
		assert_eq!(sys.nonce.get("Alice").unwrap(), &1);
		assert_eq!(sys.nonce.get("Bob").unwrap_or(&0), &0);
	}
}
