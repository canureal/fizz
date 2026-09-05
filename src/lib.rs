use clap::Parser;    


#[derive(Debug,PartialEq,Clone,clap::ValueEnum)]
pub enum Chain {
    Solana,
    Evm,
}

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
pub struct Args {
    #[arg(short,long)]
    pub chain: Chain,
}

