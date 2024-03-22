use clap::command;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct MurrayCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Debug, ValueEnum, Default)]
pub enum Currency {
    #[default]
    BTC,
    SAT,
    USD,
    BRL,
}

impl From<Currency> for murray_rs::Currency {
    fn from(value: Currency) -> Self {
        match value {
            Currency::BTC => murray_rs::Currency::BTC,
            Currency::USD => murray_rs::Currency::USD,
            Currency::BRL => murray_rs::Currency::BRL,
            Currency::SAT => murray_rs::Currency::SATS,
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    // blockchain
    /// Returns the data associated with a given trasnaction
    GetTx {
        /// The transaction's hash
        tx_id: String,
    },
    /// Returns the data associated with a bitcoin block
    GetBlock {
        /// the block hash
        block_hash: String,
    },
    /// Some useful data about the mempool
    GetMempool,
    /// Submits a transaction to the bitcoin network
    BroadcastTransaction {
        /// The hex-encode serialized transaction
        tx_hex: String,
    },
    /// Returns the time in which a block was mined or will be mined
    Height2Time {
        /// The block height
        height: Option<u32>,
    },
    /// Returns the time a block was mined
    HashToTime {
        /// the block hash
        hash: Option<String>,
    },
    /// Returns the fee recomendation for the next n blocks
    Fees,
    /// Get some general information about an address
    AddressDetails {
        /// the address you want to find out about
        address: String,
    },
    /// Get current mempool as projected blocks
    MempoolBlocks,
    /// Returns the transactions associated with an address
    AddressTransactions {
        /// the address you want to find out about
        address: String,
    },
    /// Returns the current hashrate
    HashRate,
    /// Returns a transaction, given it's id
    GetTransaction {
        /// the hex-encoded transaction id
        tx_id: String,
    },
    /// Returns useful information about a lightning node
    GetNodeInfo {
        /// the hex-encoded node id
        pub_key: String,
    },
    /// Get general stats about the lightning network
    LightningStats,
    /// Get the top lightning nodes
    LightningTopNodes,
    /// Converts values between different currencies
    Convert { currency: Currency, amount: i64 },
}
