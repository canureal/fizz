use rust_decimal::Decimal;

pub mod solana;
pub mod eth;


#[derive(Debug,PartialEq,Clone)]
pub struct Balance {
    pub public_address: String,
    pub balance: Decimal,
    pub net: String 
}


