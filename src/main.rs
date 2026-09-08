use clap::Parser;
use anyhow::Result;
use fizz::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let table = args.run().await?;
    println!("{table}"); 
    Ok(())
}
