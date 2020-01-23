use crate::newtypes::{Root, Slot};
use crate::constants::{SHARD_COUNT};
use crate::cross_link::{CrossLink};
use crate::execution_environment::{ExecutionEnvironment};

// TODO: switch from arrays and vecs to the SSZ-spec compliant FixedVector and VariableList

/// The state of the `BeaconChain` at some slot.
/// Full spec is here: https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#beaconstate
/// SSZ spec is here: https://github.com/ethereum/eth2.0-specs/blob/dev/ssz/simple-serialize.md
struct BeaconState {
    // Versioning
    genesis_time: u64,
    slot: Slot,
    fork: Fork,
    // History
    latest_block_header: BeaconBlockHeader,
    block_roots: [Root; SLOTS_PER_HISTORICAL_ROOT],
    state_roots: [Root; SLOTS_PER_HISTORICAL_ROOT],
    historical_roots: Vec<Root, HISTORICAL_ROOTS_LIMIT>,



    // Specced fields

    // Unspecced fields
    cross_links: [CrossLink; SHARD_COUNT],
    execution_environments: Vec<ExecutionEnvironment>,

}
