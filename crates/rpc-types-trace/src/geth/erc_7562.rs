//! Erc7562 tracer types.                                                                                                                                                                                         
use super::CallLogFrame;
use alloy_primitives::{map::HashMap, Address, Bytes, U256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallFrameWithOpCodes {
    #[serde(default, rename = "type")]
    ty: u8,
    from: Address,
    gas: u64,
    #[serde(default, rename = "gasUsed")]
    gas_used: u64,
    to: Address,
    input: Bytes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    output: Option<Bytes>,
    error: String,
    #[serde(default, rename = "revertReason")]
    revert_reason: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    logs: Vec<CallLogFrame>,
    value: U256,
    #[serde(default, rename = "revertedSnapshot")]
    reverted_snapshot: bool,
    #[serde(default, rename = "accessedSlots")]
    accessed_slots: AccessedSlots,
    #[serde(default, rename = "extCodeAccessInfo")]
    ext_code_access_info: Vec<Address>,
    #[serde(default, rename = "usedOpcodes")]
    used_opcodes: HashMap<u8, u64>,
    #[serde(default, rename = "contractSize")]
    contract_size: HashMap<Address, ContractSizeWithOpCode>,
    #[serde(default, rename = "outOfGas")]
    out_of_gas: bool,
    calls: Vec<CallFrameWithOpCodes>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessedSlots {
    reads: HashMap<String, Vec<String>>,
    writes: HashMap<String, u64>,
    #[serde(default, rename = "transientReads")]
    transient_reads: HashMap<String, u64>,
    #[serde(default, rename = "transientWrites")]
    transient_writes: HashMap<String, u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractSizeWithOpCode {
    #[serde(default, rename = "contractSize")]
    contract_size: usize,
    opcode: u8,
}
