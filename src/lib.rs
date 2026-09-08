mod chains;
use colored::Colorize;
use comfy_table::{Table, Cell, Color};
use chains::solana::SolanaAccount;
use clap::Parser;

use crate::chains::eth::EthAccount;


#[derive(Debug,PartialEq,Clone,clap::ValueEnum)]
pub enum Chain {
    Solana,
    Eth,
}

#[derive(PartialEq,Clone,)]
pub enum Address {
    EvmFormat(String),
    SolFormat(String),
}

#[derive(Debug,PartialEq,Clone,clap::ValueEnum)]
pub enum Net {
    #[value(name = "devnet")]
    DevNet,
    #[value(name = "testnet")]
    TestNet,
    #[value(name = "mainnet")]
    MainNet,
}

#[derive(Parser)]
#[command(version,about,long_about = None)]
pub struct Args {
    #[arg(short,long)]
    pub chain: Chain,
    pub addr: Address,
    #[arg(short,long, default_value = "mainnet")]
    pub net: Net  
    // work as fizz --chain <address> --net(optional actually) mainnet
}

impl Args {
    pub async fn run(self) -> anyhow::Result<Table> {
        let valid = matches!(
            (&self.chain, &self.addr),
            (Chain::Solana, Address::SolFormat(_)) | (Chain::Eth, Address::EvmFormat(_))
        );
        if !valid {
            anyhow::bail!("chain and address does not comply".red());
        }
        match self.chain {
            Chain::Solana => self.run_sol().await, 
            Chain::Eth => self.run_evm().await,
        }
    }

    async fn run_sol(self) -> anyhow::Result<Table> {
        let acc = SolanaAccount::try_from((self.addr, self.net))?;
        let balance = acc.get_balance().await?;
        let mut table = Table::new();
        table
            .set_header(vec!["Address", "Balance(SOL)", "network"])
            .add_row(vec![
                Cell::new(balance.public_address).fg(Color::Magenta),
                Cell::new(balance.balance.to_string()).fg(Color::Magenta),
                Cell::new(balance.net).fg(Color::Magenta),
            ]);

        Ok(table)
    }

    async fn run_evm(self) -> anyhow::Result<Table> {
        let acc = EthAccount::try_from((self.addr,self.net))?;
        let balance = acc.get_eth_balance().await?;
        let mut table = Table::new();
        
        table
            .set_header(vec!["Address", "Balance(Eth)","network"])
            .add_row(vec![
                Cell::new(balance.public_address).fg(Color::Blue),
                Cell::new(balance.balance.to_string()).fg(Color::Blue),
                Cell::new(balance.net).fg(Color::Blue),                
            ]); 

        Ok(table)
    }
}


// to, get rid of forced debug outputs
impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Solana => write!(f, "solana"),
            Chain::Eth => write!(f, "evm"),
        }
    }
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::SolFormat(addr) => write!(f, "{}", addr),
            Address::EvmFormat(addr) => write!(f, "{}", addr)
        }
    }
}
impl std::fmt::Display for Net {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Net::DevNet => write!(f, "devnet", ),
            Net::MainNet => write!(f, "mainnet"),
            Net::TestNet => write!(f, "testnet") 
        }
    }
}

//
impl std::str::FromStr for Address {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            Ok(Address::EvmFormat(s.to_string()))
        } else {
            // this ain't a good format testing but for now, we can tolerate.
            Ok(Address::SolFormat(s.to_string()))
        }
    }
}



