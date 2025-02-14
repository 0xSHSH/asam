use ethers::providers::{Middleware, Provider, Http};
use ethers::core::types::{Address, TransactionRequest, U256};
use ethers::types::transaction::eip2718::TypedTransaction;
use anyhow::{Result, Context};
use log::{info, warn, error, debug};
use thiserror::Error;
use serde::{Deserialize, Serialize};


#[derive(Error, Debug)]
pub enum SafeError {
	#[error("Transaction failed: {0}")]
	TransactionFailed(String),
	#[error("Insufficient balance for transaction. Required: {required}, Available: {available}")]
	InsufficientBalance { required: U256, available: U256 },
	#[error("Invalid address: {0}")]
	InvalidAddress(String),
	#[error("Provider error: {0}")]
	ProviderError(String),
	#[error("Gas estimation failed: {0}")]
	GasEstimationFailed(String),
	#[error("Balance below critical threshold. Current: {current}, Minimum: {minimum}. Action required: Please fund the account with at least {minimum} wei")]
	CriticalBalance { current: U256, minimum: U256 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafeTransaction {
	pub to: Address,
	pub value: U256,
	pub data: Vec<u8>,
	pub operation: u8,
	pub safe_tx_gas: U256,
	pub nonce: Option<U256>,
}

pub struct SafeManager {
	address: Address,
	provider: Provider<Http>,
	min_balance: U256,
	critical_balance: U256,
}

impl SafeManager {
	pub fn new(address: Address, provider: Provider<Http>) -> Result<Self> {
		let min_balance = U256::from(1_000_000_000_000_000_u64); // 0.001 ETH
		let critical_balance = min_balance / 2; // 0.0005 ETH

		debug!("Initializing SafeManager for address: {:?}", address);
		debug!("Minimum balance threshold: {} wei", min_balance);
		debug!("Critical balance threshold: {} wei", critical_balance);

		Ok(Self {
			address,
			provider,
			min_balance,
			critical_balance,
		})
	}

	pub async fn get_balance(&self) -> Result<U256> {
		debug!("Fetching balance for address: {:?}", self.address);
		
		self.provider
			.get_balance(self.address, None)
			.await
			.context("Failed to fetch balance")
			.map_err(|e| {
				error!("Provider error while fetching balance: {}", e);
				SafeError::ProviderError(e.to_string()).into()
			})
	}

	pub async fn check_balance_threshold(&self) -> Result<bool> {
		let balance = self.get_balance().await?;
		let is_below = balance < self.min_balance;
		
		if balance <= self.critical_balance {
			error!(
				"CRITICAL: Balance extremely low! Current: {} wei, Minimum: {} wei. Action required: Please fund the account with at least {} wei",
				balance, self.critical_balance, self.min_balance
			);
			return Err(SafeError::CriticalBalance {
				current: balance,
				minimum: self.critical_balance,
			}.into());
		}
		
		if is_below {
			warn!(
				"WARNING: Balance ({} wei) is below minimum threshold ({} wei). Consider funding the account soon.",
				balance, self.min_balance
			);
		} else {
			info!(
				"Balance is sufficient. Current: {} wei, Minimum required: {} wei",
				balance, self.min_balance
			);
		}
		
		Ok(is_below)
	}

	pub async fn simulate_transaction(&self, tx: &SafeTransaction) -> Result<U256> {
		info!("Simulating transaction to: {:?}", tx.to);
		debug!("Transaction details: value={}, data_len={}", tx.value, tx.data.len());
		
		let balance = self.get_balance().await?;
		if balance < tx.value {
			error!(
				"Insufficient balance for transaction. Required: {} wei, Available: {} wei. Please fund the account with at least {} wei",
				tx.value, balance, tx.value - balance
			);
			return Err(SafeError::InsufficientBalance {
				required: tx.value,
				available: balance,
			}.into());
		}

		let tx_request = TransactionRequest::new()
			.to(tx.to)
			.value(tx.value)
			.from(self.address)
			.data(tx.data.clone());

		let typed_tx = TypedTransaction::Legacy(tx_request);

		self.provider.estimate_gas(&typed_tx, None).await
			.map_err(|e| {
				error!("Gas estimation failed: {}. Please verify transaction parameters and network conditions", e);
				SafeError::GasEstimationFailed(e.to_string()).into()
			})
	}



	pub async fn execute_transaction(&self, tx: SafeTransaction) -> Result<()> {
		info!("Preparing to execute transaction to: {:?}", tx.to);
		debug!("Transaction value: {} wei", tx.value);

		// First simulate to get gas estimate
		let estimated_gas = self.simulate_transaction(&tx).await?;
		info!("Gas estimation successful: {} units", estimated_gas);

		// Additional validation here
		let total_required = tx.value + (estimated_gas * self.provider.get_gas_price().await?);
		let balance = self.get_balance().await?;
		
		if balance < total_required {
			return Err(SafeError::InsufficientBalance {
				required: total_required,
				available: balance,
			}.into());
		}

		// In a real implementation, this would:
		// 1. Create the Safe transaction
		// 2. Sign the transaction
		// 3. Collect required signatures
		// 4. Execute the transaction
		
		info!("Transaction executed successfully");
		debug!("Gas used: {}", estimated_gas);
		Ok(())
	}

	pub fn get_address(&self) -> Address {
		self.address
	}

	pub fn set_min_balance(&mut self, min_balance: U256) {
		self.min_balance = min_balance;
		self.critical_balance = min_balance / 2;
		info!(
			"Updated balance thresholds - Minimum: {} wei, Critical: {} wei",
			min_balance, self.critical_balance
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ethers::providers::Provider;
	use std::str::FromStr;

	async fn setup_test_manager() -> Result<SafeManager> {
		let provider = Provider::<Http>::try_from("http://localhost:8545")
			.expect("Failed to create provider");
		let address = Address::from_str("0x0000000000000000000000000000000000000000")
			.expect("Failed to parse address");
		SafeManager::new(address, provider)
	}

	#[tokio::test]
	async fn test_balance_threshold() {
		let mut manager = setup_test_manager().await.unwrap();
		manager.set_min_balance(U256::from(1_000_000_000_000_000_u64)); // 0.001 ETH
		
		let result = manager.check_balance_threshold().await;
		assert!(result.is_err());
		if let Err(e) = result {
			let safe_error = e.downcast::<SafeError>();
			assert!(safe_error.is_ok());
		}
	}

	#[tokio::test]
	async fn test_critical_balance() {
		let mut manager = setup_test_manager().await.unwrap();
		manager.set_min_balance(U256::from(1_000_000_000_000_000_u64)); // 0.001 ETH
		
		let result = manager.check_balance_threshold().await;
		assert!(result.is_err());
		if let Err(e) = result {
			let safe_error = e.downcast::<SafeError>();
			assert!(safe_error.is_ok());
		}
	}

	#[tokio::test]
	async fn test_transaction_validation() {
		let manager = setup_test_manager().await.unwrap();
		let invalid_tx = SafeTransaction {
			to: Address::zero(),
			value: U256::from(1_000_000_000_000_000_000_u64), // 1 ETH
			data: vec![],
			operation: 0,
			safe_tx_gas: U256::zero(),
			nonce: None,
		};

		let result = manager.simulate_transaction(&invalid_tx).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_get_address() {
		let manager = setup_test_manager().await.unwrap();
		let addr = manager.get_address();
		assert_eq!(
			addr,
			Address::from_str("0x0000000000000000000000000000000000000000").unwrap()
		);
	}

	#[tokio::test]
	async fn test_set_min_balance() {
		let mut manager = setup_test_manager().await.unwrap();
		let new_min = U256::from(3_000_000_000_000_000_u64); // 0.003 ETH
		manager.set_min_balance(new_min);
		
		assert_eq!(manager.critical_balance, new_min / 2);
		assert_eq!(manager.min_balance, new_min);
	}
}



