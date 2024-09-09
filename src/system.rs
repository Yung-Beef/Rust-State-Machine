use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	pub fn new() -> Self {
		Pallet { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
        self.block_number += 1;
	}

	pub fn inc_nonce(&mut self, who: &String) {
		let prev = self.nonce.get(who).unwrap_or(&0);
		self.nonce.insert(who.clone(), prev + 1);
	}
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut sys = crate::system::Pallet::new();

        sys.inc_block_number();
        sys.inc_nonce(&"Alice".to_string());

        assert_eq!(sys.block_number(), 1);
        assert_eq!(sys.nonce.get("Alice").unwrap(), &1);
        assert_eq!(sys.nonce.get("Bob").unwrap_or(&0), &0);
    }
}