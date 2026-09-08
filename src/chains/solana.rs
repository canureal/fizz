use colored::Colorize;
use rust_decimal::Decimal;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use bs58;
use crate::{
    Address,
    Net,
    chains::{Balance}
};

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;


#[derive(Debug,PartialEq,Clone)]
pub struct SolanaAccount {
    address: String,
    net: Net,
}


impl TryFrom<(Address,Net)> for SolanaAccount {
    type Error = anyhow::Error; 
    fn try_from((addr, net): (Address, Net)) -> anyhow::Result<Self> {
        match addr {
            Address::SolFormat(s) => Ok(SolanaAccount {
                address: s,
                net, 
            }),
            Address::EvmFormat(_) => anyhow::bail!("evm type! not solana!".magenta()),
        }
    }
}

fn convert_to_sol(lamport: u64) -> anyhow::Result<Decimal> {
    Ok(Decimal::from(lamport) / Decimal::from(LAMPORTS_PER_SOL))
}

impl SolanaAccount {
    fn rpc_endpoint(&self) -> &'static str {
        match self.net {
            Net::DevNet => "https://api.devnet.solana.com",
            Net::TestNet => "https://api.testnet.solana.com",
            Net::MainNet => "https://api.mainnet-beta.solana.com", 
        }
    }
    
    fn pubkey(&self) -> anyhow::Result<Pubkey> {
        let decoded_addrs = bs58::decode(&self.address).into_vec()?;
        let pubkey = Pubkey::try_from(decoded_addrs.as_slice()).map_err(|_| anyhow::anyhow!("invalid pubkey bytes"))?;
        Ok(pubkey)
    }

    pub async fn get_balance(&self) -> anyhow::Result<Balance>  {
        let rpc_client = RpcClient::new(self.rpc_endpoint().to_string());  
        let pubkey = self.pubkey()?;
        let lamports = rpc_client.get_account(&pubkey).await?.lamports;

        Ok(Balance {
            public_address: self.address.to_string(),
            balance: convert_to_sol(lamports)?,
            net: self.net.to_string()
        })  
    }       
}
