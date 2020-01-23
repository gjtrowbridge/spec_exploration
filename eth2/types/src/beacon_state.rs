use crate::constants::{
    SHARD_COUNT,
};
use crate::cross_link::{CrossLink};
use crate::eth_spec::EthSpec;
use crate::execution_environment::{ExecutionEnvironment};
use crate::newtypes::{Root, Slot};
use ssz_types::{FixedVector, VariableList};

/// The state of the `BeaconChain` at some slot.
/// Full spec is here: https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#beaconstate
/// SSZ spec is here: https://github.com/ethereum/eth2.0-specs/blob/dev/ssz/simple-serialize.md
pub struct BeaconState<T>
where
    T: EthSpec,
{
    // Versioning
//    genesis_time: u64,
    slot: Slot,
//    fork: Fork,

    // History
//    latest_block_header: BeaconBlockHeader,
    block_roots: FixedVector<Root, T::SlotsPerHistoricalRoot>,
//    state_roots: FixedVector<Root, u64>,
//    historical_roots: VariableList<Root, u64>,

    // Eth1
//    eth1_data: Eth1Data,
//    eth1_data_votes: VariableList<Eth1Data, u64>,
//    eth1_deposit_index: u64,

    // Registry
//    validators: VariableList<Validator, u64>,
//    balances: VariableList<Gwei, u64>,

    // Randomness
//    randao_mixes: FixedVector<Gwei, u64>,

    // Slashings
//    slashings: FixedVector<Gwei, u64>,

    // Attestations
//    previous_epoch_attestations: VariableList<PendingAttestation, >

    // Finality

    // Unspecced fields
//    cross_links: [CrossLink; SHARD_COUNT],
//    execution_environments: Vec<ExecutionEnvironment>,

}
