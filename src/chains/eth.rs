use std::str::FromStr;
use crate::chains::Balance;
use alloy::{primitives::{Address as AlloyAddress, U256}, providers::{Provider, ProviderBuilder}};
use colored::Colorize;
use rust_decimal::Decimal;
use crate::{Address, Net};


const WEI_PER_ETH_STR: &str = "1000000000000000000";

#[derive(Debug,PartialEq,Clone)]
pub struct EthAccount {
    address: String,
    net: Net
}

fn wei_to_eth(wei: U256) -> anyhow::Result<Decimal> {
    let wei_dec = Decimal::from_str(&wei.to_string())?;
    let divisor = Decimal::from_str(WEI_PER_ETH_STR)?;
    Ok(wei_dec / divisor)
}

impl TryFrom<(Address,Net)> for EthAccount {
    type Error = anyhow::Error;
    fn try_from((addr,net): (Address,Net)) -> anyhow::Result<Self> {
        match addr {
            Address::EvmFormat(s) => Ok(EthAccount {
                address: s,
                net
            }),
            Address::SolFormat(_) => anyhow::bail!("Not evm/eth type".red()),
        }
    }  
}

impl EthAccount {
    fn rpc_address(&self) -> &'static str {
        match self.net {
           Net::DevNet => "https://ethereum-sepolia-rpc.publicnode.com",
           Net::MainNet => "https://ethereum-rpc.publicnode.com",
           Net::TestNet => "https://ethereum.publicnode.com",
        }
    }
    
    fn check_address_validity(&self) -> anyhow::Result<AlloyAddress> {
        self.address.parse::<AlloyAddress>().map_err(|_| anyhow::anyhow!("invalid evm address"))
    }
    
    pub async fn get_eth_balance(&self) -> anyhow::Result<Balance> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_address().parse()?);
        let address = self.check_address_validity()?;
        let weis: U256 = provider.get_balance(address).await?;
        Ok(Balance {
            public_address: address.to_string(),
            balance: wei_to_eth(weis)?,
            net: self.net.to_string()
        })
    }

}
