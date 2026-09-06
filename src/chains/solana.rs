use crate::Net;
use crate::Address;

#[derive(Debug,PartialEq,Clone)]
pub struct SolanaAccount {
    address: String,
    net: String,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Balance {
    pub public_address: String,
    pub balance: u64,
}

impl TryFrom<(Address,Net)> for SolanaAccount {
    type Error = anyhow::Error; 
    fn try_from((addr, net): (Address, Net)) -> anyhow::Result<Self> {
        match addr {
            Address::SolFormat(s) => Ok(SolanaAccount {
                address: s.to_string(),
                net: net.to_string(), 
            }),
            Address::EvmFormat(_) => anyhow::bail!("emv type! not solana!"),
        }
    }
}

impl SolanaAccount {
    pub async fn get_balance(&self) -> anyhow::Result<Balance>  {
        // placeholder
        let balance: u64 = 12;
        Ok(Balance {
            public_address: self.address.to_string(),
            balance: balance 
        })  
    }       
}
