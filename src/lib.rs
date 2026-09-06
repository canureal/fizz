mod chains;
use chains::solana::SolanaAccount;
use clap::Parser;


#[derive(Debug,PartialEq,Clone,clap::ValueEnum)]
pub enum Chain {
    Solana,
    Evm,
}

#[derive(PartialEq,Clone,)]
pub enum Address {
    EvmFormat(String),
    SolFormat(String),
}

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
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let valid = matches!(
            (&self.chain, &self.addr),
            (Chain::Solana, Address::SolFormat(_)) | (Chain::Evm, Address::EvmFormat(_))
        );
        if !valid {
            anyhow::bail!("chain and address does not comply");
        }
        match self.chain {
            Chain::Solana => self.run_sol().await,
            Chain::Evm => self.run_evm().await,
        }
    }

    async fn run_sol(self) -> anyhow::Result<()> {
        let acc = SolanaAccount::try_from((self.addr, self.net))?;
        let balance = acc.get_balance().await?;
    
        println!("Address: {}\nBalance: {}\nNet: {}", balance.public_address, balance.balance, balance.net);
        Ok(())
    }

    async fn run_evm(&self) -> anyhow::Result<()> {
        println!("on {}, {}", &self.chain, &self.addr); 
        Ok(())
    }
}


// to, get rid of forced debug outputs
impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Solana => write!(f, "solana"),
            Chain::Evm => write!(f, "evm"),
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
