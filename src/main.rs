use clap::Parser;
use cmd::{Commands, MurrayCli};
use murray_rs::{
    BlockchainError, ConvertCurrencyParams, Currency, GetAddressParams, GetBlockParams,
    GetNodeDetailsParams, GetTransactionParams, LightningError, Murray, PostTransactionParams,
    PriceError,
};
use serde_json::Value;

mod cmd;

fn main() {
    let cli = MurrayCli::parse();
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
        Commands::HashRate => murray.get_hash_rate(),
        Commands::GetTransaction { tx_id } => murray.get_transaction(tx_id),
        Commands::AddressTransactions { address } => murray.get_address_transaction(address),
        Commands::Convert { amount, currency } => murray.convert(currency.into(), amount),
        Commands::LightningStats => murray.lightning_stats(),
        Commands::GetNodeInfo { pub_key } => murray.get_node_info(pub_key),
        Commands::LightningTopNodes => murray.get_lightning_top_nodes(),
    };

    match response {
        Ok(response) => println!("{}", ::serde_json::to_string_pretty(&response).unwrap()),
        Err(error) => println!("Something went wrong while processing the request: {error:?}"),
    }
}

#[derive(Default)]
struct Client {
    client: Murray,
}

impl Client {
    pub fn convert(&self, currency: Currency, value: i64) -> Result<Value, Error> {
        call_murray!(self
            .client
            .prices
            .convert_currency(ConvertCurrencyParams { currency, value }))
    }

    pub fn get_lightning_top_nodes(&self) -> Result<Value, Error> {
        call_murray!(self.client.lightning.get_top_nodes())
    }

    pub fn lightning_stats(&self) -> Result<Value, Error> {
        call_murray!(self.client.lightning.get_statistics())
    }

    pub fn get_node_info(&self, public_key: String) -> Result<Value, Error> {
        call_murray!(self
            .client
            .lightning
            .get_node_details(GetNodeDetailsParams { public_key }))
    }

    pub fn get_hash_rate(&self) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.get_hashrate())
    }

    pub fn get_transaction(&self, txid: String) -> Result<Value, Error> {
        call_murray!(self
            .client
            .blockchain
            .get_transaction(GetTransactionParams { txid }))
    }

    pub fn get_address_transaction(&self, address: String) -> Result<Value, Error> {
        call_murray!(self
            .client
            .blockchain
            .get_address_transactions(GetAddressParams { address }))
    }

    pub fn get_fees_mempool_blocks(&self) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.get_fees_mempool_blocks())
    }

    pub fn get_tx(&self, txid: String) -> Result<Value, Error> {
        call_murray!(self
            .client
            .blockchain
            .get_transaction(GetTransactionParams { txid }))
    }

    pub fn address_details(&self, address: String) -> Result<Value, Error> {
        call_murray!(self
            .client
            .blockchain
            .get_address_details(GetAddressParams { address }))
    }

    pub fn get_block(&self, hash: String) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.get_block(GetBlockParams {
            hash: Some(hash),
            height: None
        }))
    }

    pub fn get_mempool(&self) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.get_mempool())
    }

    pub fn broadcast_transaction(&self, tx_hex: String) -> Result<Value, Error> {
        call_murray!(self
            .client
            .blockchain
            .post_transaction(PostTransactionParams { tx_hex }))
    }

    pub fn block_to_time(&self, hash: Option<String>, height: Option<u32>) -> Result<Value, Error> {
        call_murray!(self
            .client
            .blockchain
            .get_block2time(GetBlockParams { hash, height }))
    }

    pub fn get_fees(&self) -> Result<Value, Error> {
        call_murray!(self.client.blockchain.get_fees_recommended())
    }
}

#[derive(Debug)]
enum Error {
    Serde,
    Blockchain(BlockchainError),
    Lightning(LightningError),
    Price(PriceError),
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

impl From<LightningError> for Error {
    fn from(value: LightningError) -> Self {
        Error::Lightning(value)
    }
}

impl From<PriceError> for Error {
    fn from(value: PriceError) -> Self {
        Error::Price(value)
    }
}

macro_rules! call_murray {
    ($cmd: expr) => {
        Ok(::serde_json::to_value($cmd?)?)
    };
}

use call_murray;
