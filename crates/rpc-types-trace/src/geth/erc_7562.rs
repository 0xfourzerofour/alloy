//! Erc7562 tracer types. This module contains structures for tracing and validating Erc7562 transactions.

use std::collections::{HashMap, HashSet};

use alloy_primitives::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

use super::CallLogFrame;

/// Represents a call frame in the EVM with additional opcode information.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallFrameWithOpCodes {
    /// Type of the call frame. Renamed to 'ty' for serialization due to 'type' being a reserved keyword in Rust.
    #[serde(default, rename = "type")]
    pub ty: u8,

    /// Address of the caller.
    pub from: Address,

    /// Amount of gas provided for this call.
    pub gas: u64,

    /// Gas consumed by this call. Renamed from 'gasUsed' for serialization.
    #[serde(default, rename = "gasUsed")]
    pub gas_used: u64,

    /// Address of the callee.
    pub to: Address,

    /// Input data for the call.
    pub input: Bytes,

    /// Output of the call if successful, can be `None` if not applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Bytes>,

    /// Error message if the call failed.
    pub error: String,

    /// Reason for reversion if the call was reverted.
    #[serde(default, rename = "revertReason")]
    pub revert_reason: String,

    /// Logs emitted during this call, serialized only if not empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<CallLogFrame>,

    /// Value transferred in wei.
    pub value: U256,

    /// Whether a snapshot was reverted during this call.
    #[serde(default, rename = "revertedSnapshot")]
    pub reverted_snapshot: bool,

    /// Information about storage slots that were accessed.
    #[serde(default, rename = "accessedSlots")]
    pub accessed_slots: AccessedSlots,

    /// List of addresses for which external code was accessed.
    #[serde(default, rename = "extCodeAccessInfo")]
    pub ext_code_access_info: Vec<Address>,

    /// Counts of opcodes used during this call.
    #[serde(default, rename = "usedOpcodes")]
    pub used_opcodes: HashMap<u8, u64>,

    /// Size of contracts and the opcode used to create them.
    #[serde(default, rename = "contractSize")]
    pub contract_size: HashMap<Address, ContractSizeWithOpCode>,

    /// Indicates if the call ran out of gas.
    #[serde(default, rename = "outOfGas")]
    pub out_of_gas: bool,

    /// Child calls made from this call frame.
    pub calls: Vec<CallFrameWithOpCodes>,
}

/// Structure to hold information about accessed storage slots.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessedSlots {
    /// Mapping of address to a list of slots read from.
    pub reads: HashMap<String, Vec<String>>,

    /// Mapping of address to the number of writes to slots.
    pub writes: HashMap<String, u64>,

    /// Mapping of address to the number of transient reads.
    #[serde(default, rename = "transientReads")]
    pub transient_reads: HashMap<String, u64>,

    /// Mapping of address to the number of transient writes.
    #[serde(default, rename = "transientWrites")]
    pub transient_writes: HashMap<String, u64>,
}

/// Structure to store contract size along with the opcode used for deployment.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractSizeWithOpCode {
    /// Size of the contract in bytes.
    #[serde(default, rename = "contractSize")]
    pub contract_size: usize,

    /// Opcode used for contract deployment.
    pub opcode: u8,
}

/// Configuration for the Erc7562 validation tracer.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Erc7562ValidationTracerConfig {
    /// Number of items from the stack top to include in the trace.
    #[serde(default, rename = "stackTopItemsSize")]
    pub stack_top_items_size: usize,

    /// Set of opcodes to ignore during tracing.
    #[serde(default, rename = "ignoredOpcodes")]
    pub ignored_opcodes: HashSet<u8>,

    /// Flag to include logs in the trace.
    #[serde(default, rename = "withLog")]
    pub with_log: bool,
}

impl Erc7562ValidationTracerConfig {
    /// Creates a new `Erc7562ValidationTracerConfig` with default values.
    pub fn new() -> Self {
        Self { stack_top_items_size: 3, with_log: true, ignored_opcodes: HashSet::new() }
    }
}
