mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	fn new() -> Self {
		Runtime {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "Alice".to_string();
	let bob = "Bob".to_string();
	let charlie = "Charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

    // start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

    // first transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("{}", e));

    // second transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice, charlie, 20)
		.map_err(|e| eprintln!("{}", e));

	println!("{:?}", runtime);
}

#[test]
fn init_balances() {
	let mut balances = balances::Pallet::new();

	assert_eq!(balances.balance(&"alice".to_string()), 0);
	balances.set_balance(&"alice".to_string(), 100);
	assert_eq!(balances.balance(&"alice".to_string()), 100);
	assert_eq!(balances.balance(&"bob".to_string()), 0);
	assert_eq!(balances.transfer(String::from("alice"), String::from("bob"), 50), Ok(()))
}

#[test]
fn transfer_balance() {
	let mut balances = balances::Pallet::new();
	balances.set_balance(&"alice".to_string(), 100);

	assert_eq!(balances.transfer(String::from("alice"), String::from("bob"), 150), Err("Error"));
	assert_eq!(balances.transfer(String::from("alice"), String::from("bob"), 50), Ok(()));
	assert_eq!(balances.balance(&"alice".to_string()), 50);
	assert_eq!(balances.balance(&"bob".to_string()), 50);
}
