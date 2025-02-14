pub mod safe_manager;
pub mod defi_optimizer;
pub mod cross_chain_router;

#[cfg(test)]
pub(crate) mod test_utils {
	use ethers::core::types::{Address, U256};
	use std::str::FromStr;

	pub fn get_test_address() -> Address {
		Address::from_str("0x0000000000000000000000000000000000000000").unwrap()
	}

	pub fn get_test_value(eth_amount: f64) -> U256 {
		let wei = eth_amount * 1_000_000_000_000_000_000.0;
		U256::from(wei as u64)
	}

	pub fn setup_test_env() {
		std::env::set_var("RUST_LOG", "debug");
		std::env::set_var("ETH_RPC_URL", "http://localhost:8545");
		std::env::set_var("ACCOUNT_ADDRESS", "0x0000000000000000000000000000000000000000");
	}
}
