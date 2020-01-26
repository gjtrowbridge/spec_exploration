use crate::cross_link::{CrossLink};
use crate::eth_spec::EthSpec;
use crate::execution_environment::{ExecutionEnvironment};
use crate::newtypes::{Root, Slot};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, FixedVector, VariableList};


/// The state of the `BeaconChain` at some slot.
/// Full spec is here: https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#beaconstate
/// SSZ spec is here: https://github.com/ethereum/eth2.0-specs/blob/dev/ssz/simple-serialize.md

#[derive(
Debug,
PartialEq,
Clone,
Deserialize,
Serialize,
Encode,
Decode,
)]
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
//    block_roots: FixedVector<Root, T::SlotsPerHistoricalRoot>,
//    state_roots: FixedVector<Root, T::SlotsPerHistoricalRoot>,
//    historical_roots: VariableList<Root, T::HistoricalRootsLimit>,

    // Eth1
//    eth1_data: Eth1Data,
//    eth1_data_votes: VariableList<Eth1Data, T::ValidatorRegistryLimit>,
//    eth1_deposit_index: u64,

    // Registry
//    validators: VariableList<Validator, T::ValidatorRegistryLimit>,
//    balances: VariableList<Gwei, T::ValidatorRegistryLimit>,

    // Randomness
//    randao_mixes: FixedVector<Gwei, T::EpochsPerHistoricalVector>,

    // Slashings
//    slashings: FixedVector<Gwei, T::EpochsPerSlashingsVector>,

    // Attestations
//    previous_epoch_attestations: VariableList<PendingAttestation, T::MaxPendingAttestations>,

    // Finality
//    justification_bits: BitVector<T::JustificationBitsLength>,
//    previous_justified_checkpoint: Checkpoint,
//    current_justified_checkpoint: Checkpoint,
//    finalized_checkpoint: Checkpoint,

    // Unspecced fields
    cross_links: FixedVector<CrossLink, T::ShardCount>,
    execution_environments: VariableList<ExecutionEnvironment, T::MaxExecutionEnvironments>,

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn can_encode_and_decode_ssz() {
        assert_eq!(1, 2);
    }
}

