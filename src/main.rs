mod balances;
mod proof_of_existence;
mod support;
mod system;
use crate::support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type Block = crate::support::Block<Header, Extrinsic>;
	pub type BlockNumber = u32;
	pub type Content = &'static str;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Nonce = u32;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

fn main() {
	// Create a new instance of the Runtime.
	// It will instantiate with it all the modules it uses.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&alice, 100);

	// Create block 1
	let block_1 = crate::types::Block {
		header: crate::types::Header { block_number: 1 },
		extrinsics: vec![
			// First transaction
			crate::types::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: 30 }),
			},
			// Second transaction
			crate::types::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 20 }),
			},
		],
	};

	// Execute block 1
	runtime.execute_block(block_1).expect("Invalid block");

	// Create block 2
	let block_2 = crate::types::Block {
		header: crate::types::Header { block_number: 2 },
		extrinsics: vec![
			// First transaction
			crate::types::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "Alice's claim" })
			},
			// Second transaction
			crate::types::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "Bob's claim" })
			},
			// Third transaction
			crate::types::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim { claim: "Alice's claim" })
			},
		],
	};

	// Execute block 2
	runtime.execute_block(block_2).expect("Invalid block");

	// Create block 3
	let block_3 = crate::types::Block {
		header: crate::types::Header { block_number: 3 },
		extrinsics: vec![
			// First transaction
			crate::types::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim { claim: "Bob's claim" })
			},
		],
	};

	// Execute block 3
	runtime.execute_block(block_3).expect("Invalid block");

	// Simply print the debug format of our runtime state.
	println!("{:?}", runtime);
}
