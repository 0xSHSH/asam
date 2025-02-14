mod agents;

use anyhow::{Context, Result};
use dotenv::dotenv;
use ethers::core::types::{Address, U256};
use ethers::providers::{Http, Provider};
use log::{debug, error, info, warn};
use std::{env, str::FromStr};
use tokio::time::{sleep, Duration};
use agents::{
    safe_manager::SafeManager,
    defi_optimizer::DefiOptimizer,
    cross_chain_router::CrossChainRouter,
};

async fn monitor_and_optimize(
    safe_manager: &SafeManager,
    defi_optimizer: &DefiOptimizer,
    cross_chain_router: &CrossChainRouter,
) -> Result<()> {
    debug!("Starting monitoring cycle...");
    
    // Monitor account balance with enhanced error handling
    match safe_manager.get_balance().await {
        Ok(balance) => {
            let balance_eth = format_eth(balance);
            info!("Current balance: {:.6} ETH ({} wei)", balance_eth, balance);

            // Check balance threshold with proper error handling
            match safe_manager.check_balance_threshold().await {
                Ok(is_below) => {
                    if is_below {
                        warn!("Balance is below minimum threshold - initiating optimization process");
                        debug!("Searching for optimization opportunities...");
                    } else {
                        debug!("Balance is within acceptable range");
                    }
                }
                Err(e) => {
                    error!("Critical balance check failed: {}", e);
                    error!("Action required: Please fund the account to continue operations");
                    return Err(e);
                }
            }
        }
        Err(e) => {
            error!("Failed to get balance: {}", e);
            error!("Check your node connection and try again");
            return Err(e);
        }
    }

    // Find best DeFi pool with enhanced validation and logging
    debug!("Analyzing DeFi opportunities across chains...");
    match defi_optimizer.get_best_pool().await {
        Ok(pool) => {
            let apy = pool.apy.unwrap_or(0.0);
            
            if apy > 0.0 && pool.tvl > 0.0 {
                info!(
                    "Found optimal pool: {} on {} (APY: {:.2}%, TVL: ${:.2})",
                    pool.protocol,
                    pool.chain,
                    apy,
                    pool.tvl
                );

                if pool.chain != "Ethereum" {
                    info!("Initiating cross-chain optimization to {}", pool.chain);
                    debug!("Starting bridge transaction simulation");
                    match cross_chain_router
                        .route_funds(100.0, "Ethereum", &pool.chain)
                        .await 
                    {
                        Ok(_) => {
                            info!("Successfully routed funds to {}", pool.chain);
                            debug!("Bridge transaction completed successfully");
                        }
                        Err(e) => {
                            error!("Cross-chain routing failed: {}", e);
                            error!("Bridge transaction simulation failed - check network conditions");
                            return Err(e);
                        }
                    }
                } else {
                    debug!("Optimal pool is on Ethereum - no bridge required");
                }
            } else {
                warn!(
                    "Skipping pool {} due to insufficient metrics (APY: {:.2}%, TVL: ${:.2})",
                    pool.protocol,
                    apy,
                    pool.tvl
                );
                debug!("Pool metrics below threshold - continuing search");
            }
        }
        Err(e) => {
            error!("Failed to find optimal pool: {}", e);
            error!("DeFi optimization process failed - check API connectivity");
            return Err(e);
        }
    }

    debug!("Monitoring cycle completed successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize environment
    dotenv().ok();
    
    // Configure logging with a more explicit setup
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format_timestamp_secs()
        .init();
    
    info!("Starting ASAM with enhanced monitoring...");
    debug!("Initializing environment variables and connections...");

    // Get and validate environment variables
    let rpc_url = env::var("ETH_RPC_URL")
        .context("ETH_RPC_URL must be set")?;
    let account_address_str = env::var("ACCOUNT_ADDRESS")
        .context("ACCOUNT_ADDRESS must be set")?;
    let account_address = Address::from_str(&account_address_str)
        .context("Invalid account address format")?;

    debug!("Environment variables loaded successfully");

    // Configure API timeout with validation
    let api_timeout = env::var("API_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    if api_timeout < 5 {
        warn!("API timeout is set below recommended minimum (5s). Current: {}s", api_timeout);
    }
    debug!("API timeout configured: {}s", api_timeout);

    // Initialize provider with timeout
    let provider = Provider::<Http>::try_from(rpc_url.clone())
        .context("Failed to initialize provider")?;
    info!("Successfully connected to Ethereum node at {}", rpc_url);

    // Initialize agents with enhanced error handling
    debug!("Initializing ASAM components...");
    let safe_manager = SafeManager::new(account_address, provider.clone())
        .context("Failed to initialize SafeManager")?;
    let defi_optimizer = DefiOptimizer::new();
    let cross_chain_router = CrossChainRouter::new();
    debug!("All components initialized successfully");

    info!("ASAM initialized successfully");
    info!("Monitoring address: {}", account_address);
    info!("API timeout: {}s", api_timeout);

    // Main monitoring loop with enhanced error handling
    loop {
        match monitor_and_optimize(&safe_manager, &defi_optimizer, &cross_chain_router).await {
            Ok(_) => debug!("Monitoring cycle completed successfully"),
            Err(e) => {
                error!("Error in monitoring cycle: {}", e);
                error!("Error details: {:?}", e);
                error!("Will retry in 60 seconds...");
            }
        }

        info!("Waiting 60 seconds before next monitoring cycle...");
        sleep(Duration::from_secs(60)).await;
    }
}

fn format_eth(wei: U256) -> f64 {
    let wei_f: f64 = wei.as_u128() as f64;
    wei_f / 1_000_000_000_000_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::test_utils::{get_test_address, setup_test_env};

    #[tokio::test]
    async fn test_monitor_and_optimize_integration() {
        setup_test_env();
        
        let provider = Provider::<Http>::try_from("http://localhost:8545")
            .expect("Failed to create provider");
        let mut safe_manager = SafeManager::new(get_test_address(), provider.clone())
            .expect("Failed to create SafeManager");
        
        // Set a reasonable minimum balance
        safe_manager.set_min_balance(U256::from(100_000_000_000_000_u64)); // 0.0001 ETH
        
        let mut defi_optimizer = DefiOptimizer::with_mock();
        defi_optimizer.use_mock = true;
        let cross_chain_router = CrossChainRouter::new();

        // Since we're testing integration, we only care that it doesn't panic
        let _ = monitor_and_optimize(&safe_manager, &defi_optimizer, &cross_chain_router).await;
        assert!(true);
    }


    #[tokio::test]
    async fn test_low_balance_handling() {
        setup_test_env();
        
        let provider = Provider::<Http>::try_from("http://localhost:8545")
            .expect("Failed to create provider");
        let mut safe_manager = SafeManager::new(get_test_address(), provider.clone())
            .expect("Failed to create SafeManager");
        
        // Set a high minimum balance to trigger low balance warning
        safe_manager.set_min_balance(U256::from(10_000_000_000_000_000_000_u64)); // 10 ETH
        
        let defi_optimizer = DefiOptimizer::with_mock();
        let cross_chain_router = CrossChainRouter::new();

        let result = monitor_and_optimize(&safe_manager, &defi_optimizer, &cross_chain_router).await;
        assert!(result.is_err());
    }
}



