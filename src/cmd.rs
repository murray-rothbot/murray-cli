use clap::command;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct MurrayCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Returns the data associated with a given trasnaction
    GetTx{
        /// The transaction's hash
        tx_id: String
    },
    /// Returns the data associated with a bitcoin block
    GetBlock{
        block_hash: String
    },
    /// Some useful data about the mempool
    GetMempool,
    /// Submits a transaction to the bitcoin network
    BroadcastTransaction {
        /// The hex-encode serialized transaction
        tx_hex: String
    },
    /// Returns the time in which a block was mined or will be mined
    Height2Time {
        /// The block height
        height: Option<u32> 
    },
    /// Returns the time a block was mined
    HashToTime{
        hash: Option<String>,
    },
    /// Returns the fee recomendation for the next n blocks
    Fees,
    AddressDetails {
        address: String
    },
    /// Get current mempool as projected blocks
    MempoolBlocks,
}
