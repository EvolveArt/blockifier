use alloc::string::String;

use starknet_api::api_core::{ChainId, ContractAddress};
use starknet_api::block::{BlockNumber, BlockTimestamp};

use crate::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BlockContext {
    pub chain_id: ChainId,
    pub block_number: BlockNumber,
    pub block_timestamp: BlockTimestamp,

    // Fee-related.
    pub sequencer_address: ContractAddress,
    pub fee_token_address: ContractAddress,
    pub vm_resource_fee_cost: HashMap<String, f64>,
    pub gas_price: u128, // In wei.

    // Limits.
    pub invoke_tx_max_n_steps: u32,
    pub validate_max_n_steps: u32,
}
