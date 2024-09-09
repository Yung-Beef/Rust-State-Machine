use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		let c = self.balance(&caller);
		let t = self.balance(&to);

		let new_caller_balance = c.checked_sub(amount).ok_or("Error")?;
		let new_to_balance = t.checked_add(amount).ok_or("Error")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}
