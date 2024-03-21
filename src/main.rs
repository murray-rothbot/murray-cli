use clap::Parser;
use cmd::{Commands, MurrayCli};
use murray_rs::{BlockchainError, GetAddressParams, GetBlockParams, GetTransactionParams, Murray, PostTransactionParams};
use serde_json::Value;

mod cmd;

fn main() {
    let cli =MurrayCli::parse();
    let murray = Client::default();
    
    let response = match cli.command {
        Commands::GetTx { tx_id } => murray.get_tx(tx_id),
        Commands::GetMempool => murray.get_mempool(),
        Commands::BroadcastTransaction { tx_hex } => murray.broadcast_transaction(tx_hex),
        Commands::GetBlock { block_hash } => murray.get_block(block_hash),
        Commands::Height2Time { height } => murray.block_to_time(None, height),
        Commands::HashToTime { hash } => murray.block_to_time(hash, None),
        Commands::Fees => murray.get_fees(),
        Commands::AddressDetails { address } => murray.address_details(address),
        Commands::MempoolBlocks => murray.get_fees_mempool_blocks(),
    };

    match response {
        Ok(response) =>  println!("{}", ::serde_json::to_string_pretty(&response).unwrap()),
        Err(error) => println!("Something went wrong while processing the request: {error:?}")
    }
}

#[derive(Default)]
struct Client {
    client: Murray,
}

impl Client{
    pub fn get_fees_mempool_blocks(&self) -> Result<Value, Error>{
        call_murray!(self.client.blockchain.get_fees_mempool_blocks())
    }

    pub fn get_tx(&self, txid: String) -> Result<Value, Error>{
        call_murray!(self.client.blockchain.get_transaction(GetTransactionParams { txid }))
    }
    
    pub fn address_details(&self, address: String) -> Result<Value, Error>{
        call_murray!(self.client.blockchain.get_address_details(GetAddressParams { address }))
    }

    pub fn get_block(&self, hash: String) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.get_block(GetBlockParams{hash: Some(hash), height: None}))
    }
    
    pub fn get_mempool(&self) -> Result<Value, Error> {
       call_murray!(self.client.blockchain.get_mempool())
    }
    
    pub fn broadcast_transaction(&self, tx_hex: String) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.post_transaction(PostTransactionParams{tx_hex}))
    }

    pub fn block_to_time(&self, hash: Option<String>, height: Option<u32>) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.get_block2time(GetBlockParams{hash, height}))
    }

    pub fn get_fees(&self) -> Result<Value, Error>{
        call_murray!(self.client.blockchain.get_fees_recommended())
    }
}

#[derive(Debug)]
enum Error {
    Serde,
    Blockchain(BlockchainError),
}

impl From<serde_json::Error> for Error {
    fn from(_value: serde_json::Error) -> Self {
        Error::Serde   
    }
}

impl From<BlockchainError> for Error {
    fn from(value: BlockchainError) -> Self {
        Error::Blockchain(value)
    }
}

macro_rules! call_murray {
    ($cmd: expr) => {
       Ok(::serde_json::to_value($cmd?)?) 
    };
}

use call_murray;
