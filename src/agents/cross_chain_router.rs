use anyhow::Result;
use log::{debug, info, error};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
use tokio::time::{sleep, Duration};

#[derive(Error, Debug)]
pub enum CrossChainError {
	#[error("Invalid chain '{0}'. Supported chains: {1}")]
	InvalidChain(String, String),
	#[error("Insufficient liquidity for transfer. Required: {required}, Available: {available}")]
	InsufficientLiquidity { required: f64, available: f64 },
	#[error("Amount {amount} is below minimum {minimum}")]
	AmountTooLow { amount: f64, minimum: f64 },
	#[error("Bridge error: {0}")]
	BridgeError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainInfo {
	pub name: String,
	pub chain_id: u64,
	pub is_active: bool,
	pub min_transfer: f64,
}

pub struct CrossChainRouter {
	supported_chains: HashSet<String>,
	min_amount: f64,
}

impl CrossChainRouter {
	pub fn new() -> Self {
		let mut supported_chains = HashSet::new();
		supported_chains.insert("Ethereum".to_string());
		supported_chains.insert("Arbitrum".to_string());
		supported_chains.insert("Optimism".to_string());
		supported_chains.insert("Polygon".to_string());
		supported_chains.insert("Fantom".to_string());
		
		Self {
			supported_chains,
			min_amount: 0.1,
		}
	}

	pub async fn route_funds(&self, amount: f64, source_chain: &str, target_chain: &str) -> Result<()> {
		debug!("Starting cross-chain transfer validation");
		debug!("Validating source chain: {}", source_chain);
		
		self.validate_chain(source_chain)
			.map_err(|e| {
				error!("Source chain validation failed: {}", e);
				error!("Supported chains: {}", self.supported_chains.iter().cloned().collect::<Vec<_>>().join(", "));
				e
			})?;
			
		debug!("Validating target chain: {}", target_chain);
		self.validate_chain(target_chain)
			.map_err(|e| {
				error!("Target chain validation failed: {}", e);
				error!("Supported chains: {}", self.supported_chains.iter().cloned().collect::<Vec<_>>().join(", "));
				e
			})?;

		debug!("Validating transfer amount: {} tokens", amount);
		if amount < self.min_amount {
			let error = CrossChainError::AmountTooLow {
				amount,
				minimum: self.min_amount,
			};
			error!("Transfer amount too low: {}", error);
			error!("Please increase the transfer amount to at least {} tokens", self.min_amount);
			return Err(error.into());
		}

		debug!("Checking bridge liquidity...");
		self.check_liquidity(amount, source_chain, target_chain)
			.map_err(|e| {
				error!("Liquidity check failed: {}", e);
				error!("Please try again with a smaller amount or wait for liquidity to increase");
				e
			})?;

		info!(
			"Initiating cross-chain transfer: {} tokens from {} to {}",
			amount, source_chain, target_chain
		);
		debug!("All validations passed, proceeding with bridge transaction");

		self.simulate_bridge_transaction(amount, source_chain, target_chain)
			.await
			.map_err(|e| {
				error!("Bridge transaction failed: {}", e);
				error!("Transaction simulation encountered an error - please check network conditions");
				e
			})?;

		info!(
			"Successfully routed {} tokens from {} to {}",
			amount, source_chain, target_chain
		);
		debug!("Cross-chain transfer completed successfully");
		
		Ok(())
	}

	fn validate_chain(&self, chain: &str) -> Result<()> {
		if !self.supported_chains.contains(chain) {
			let supported = self.supported_chains
				.iter()
				.cloned()
				.collect::<Vec<_>>()
				.join(", ");
				
			return Err(CrossChainError::InvalidChain(
				chain.to_string(),
				supported
			).into());
		}
		Ok(())
	}

	fn check_liquidity(&self, amount: f64, _source_chain: &str, _target_chain: &str) -> Result<()> {
		let simulated_liquidity = 1000.0;
		
		if amount > simulated_liquidity {
			return Err(CrossChainError::InsufficientLiquidity {
				required: amount,
				available: simulated_liquidity,
			}.into());
		}
		
		debug!(
			"Liquidity check passed. Required: {}, Available: {}",
			amount, simulated_liquidity
		);
		Ok(())
	}

	async fn simulate_bridge_transaction(&self, amount: f64, source_chain: &str, target_chain: &str) -> Result<()> {
		debug!("Starting bridge transaction simulation");
		debug!("Simulating lock transaction on source chain");
		
		info!("Step 1: Locking {} tokens on {}", amount, source_chain);
		debug!("Waiting for lock transaction confirmation...");
		sleep(Duration::from_secs(1)).await;
		
		info!("Step 2: Generating proof for {} tokens {} -> {}", amount, source_chain, target_chain);
		debug!("Computing merkle proof for bridge transaction...");
		sleep(Duration::from_secs(1)).await;
		
		info!("Step 3: Releasing {} tokens on {}", amount, target_chain);
		debug!("Simulating release transaction on target chain...");
		sleep(Duration::from_secs(1)).await;
		
		debug!("Bridge transaction simulation completed successfully");
		debug!("All bridge steps executed without errors");
		Ok(())
	}

	pub fn get_supported_chains(&self) -> Vec<String> {
		self.supported_chains.iter().cloned().collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_unsupported_chain() {
		let router = CrossChainRouter::new();
		let result = router.route_funds(100.0, "Ethereum", "Unsupported").await;
		assert!(matches!(
			result.unwrap_err().downcast::<CrossChainError>(),
			Ok(CrossChainError::InvalidChain(_, _))
		));
	}

	#[tokio::test]
	async fn test_amount_too_low() {
		let router = CrossChainRouter::new();
		let result = router.route_funds(0.05, "Ethereum", "Arbitrum").await;
		assert!(matches!(
			result.unwrap_err().downcast::<CrossChainError>(),
			Ok(CrossChainError::AmountTooLow { amount: 0.05, minimum: 0.1 })
		));
	}

	#[tokio::test]
	async fn test_insufficient_liquidity() {
		let router = CrossChainRouter::new();
		let result = router.route_funds(2000.0, "Ethereum", "Arbitrum").await;
		assert!(matches!(
			result.unwrap_err().downcast::<CrossChainError>(),
			Ok(CrossChainError::InsufficientLiquidity { required: 2000.0, available: 1000.0 })
		));
	}

	#[tokio::test]
	async fn test_supported_chains() {
		let router = CrossChainRouter::new();
		let chains = router.get_supported_chains();
		assert!(chains.contains(&"Ethereum".to_string()));
		assert!(chains.contains(&"Arbitrum".to_string()));
		assert!(chains.contains(&"Optimism".to_string()));
		assert!(chains.contains(&"Polygon".to_string()));
		assert!(chains.contains(&"Fantom".to_string()));
	}

	#[tokio::test]
	async fn test_successful_transfer() {
		let router = CrossChainRouter::new();
		let result = router.route_funds(100.0, "Ethereum", "Arbitrum").await;
		assert!(result.is_ok());
	}
}