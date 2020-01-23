// Recording all of these as constants for now (and only uncommenting ones currently in use in the
// simulation). In future, we can refactor this such that these can be passed as parameters to spec
// types (versus hard-coding as constants)

// Phase 0 Spec Here:
// https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#constants

//// Misc
//pub const MAX_COMMITTEES_PER_SLOT: usize = 64;
//pub const TARGET_COMMITTEE_SIZE: usize = 128;
//pub const MAX_VALIDATORS_PER_COMMITTEE: usize = 2048;
//pub const MIN_PER_EPOCH_CHURN_LIMIT: usize = 4;
//pub const CHURN_LIMIT_QUOTIENT: usize = 65536;
//pub const SHUFFLE_ROUND_COUNT: usize = 90;
//pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: usize = 16384;
//pub const MIN_GENESIS_TIME: u64 = 1578009600;

//// Gwei Values
//pub const MIN_DEPOSIT_AMOUNT: u64 = 1_000_000_000;
//pub const MAX_EFFECTIVE_BALANCE: u64 = 32_000_000_000;
//pub const EJECTION_BALANCE: u64 = 16_000_000_000;
//pub const EFFECTIVE_BALANCE_INCREMENT: u64 = 1_000_000_000;

//// Initial Values
//GENESIS_FORK_VERSION = Version('0x00000000')
//BLS_WITHDRAWAL_PREFIX = Bytes1('0x00')

//// Time Parameters
//pub const MIN_GENESIS_DELAY: u64 = 86400;
//pub const SECONDS_PER_SLOT: u64 = 12;
//pub const MIN_ATTESTATION_INCLUSION_DELAY: u64 = 1;
//pub const SLOTS_PER_EPOCH: u64 = 32;
//pub const MIN_SEED_LOOKAHEAD: u64 = 1;
//pub const MAX_SEED_LOOKAHEAD: u64 = 4;
//pub const MIN_EPOCHS_TO_INACTIVITY_PENALTY: u64 = 4;
//pub const SLOTS_PER_ETH1_VOTING_PERIOD: u64 = 1024;
//pub const SLOTS_PER_HISTORICAL_ROOT: u64 = 8192;
//pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: u64 = 256;
//pub const PERSISTENT_COMMITTEE_PERIOD: u64 = 2048;

//// State List Lengths
//pub const EPOCHS_PER_HISTORICAL_VECTOR: u64 = 65536;
//pub const EPOCHS_PER_SLASHINGS_VECTOR: u64 = 8192;
//pub const HISTORICAL_ROOTS_LIMIT: u64 = 16_777_216;
//pub const VALIDATOR_REGISTRY_LIMIT: u64 = 1_099_511_627_776;

//// Rewards and Penalties
//pub const BASE_REWARD_FACTOR: u64 = 64;
//pub const WHISTLEBLOWER_REWARD_QUOTIENT: u64 = 512;
//pub const PROPOSER_REWARD_QUOTIENT: u64 = 8;
//pub const INACTIVITY_PENALTY_QUOTIENT: u64 = 33_554_432;
//pub const MIN_SLASHING_PENALTY_QUOTIENT: u64 = 32;

//// Max Operations Per Block
//pub const MAX_PROPOSER_SLASHINGS: u64 = 16;
//pub const MAX_ATTESTER_SLASHINGS: u64 = 1;
//pub const MAX_ATTESTATIONS: u64 = 128;
//pub const MAX_DEPOSITS: u64 = 16;
//pub const MAX_VOLUNTARY_EXITS: u64 = 16;

//// Domain Types
//DOMAIN_BEACON_PROPOSER = DomainType('0x00000000')
//DOMAIN_BEACON_ATTESTER = DomainType('0x01000000')
//DOMAIN_RANDAO = DomainType('0x02000000')
//DOMAIN_DEPOSIT = DomainType('0x03000000')
//DOMAIN_VOLUNTARY_EXIT = DomainType('0x04000000')

// Phase 1 Spec Here:
// https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase1/shard-data-chains.md#configuration

//// Misc
//// Differs from spec link above bc new proposal is to have 64, not 1024, shards
//pub const SHARD_COUNT: usize = 64;
//pub const MIN_BLOCK_BODY_PRICE: u64 = 1;
//pub const MAX_PERIOD_COMMITTEE_SIZE: u64 = 128;
//pub const SHARD_HEADER_SIZE: u64 = 1024;
//pub const SHARD_BLOCK_SIZE_TARGET: u64 = 16_384;
//pub const MAX_SHARD_BLOCK_SIZE: u64 = 65_536;

//// Time Parameters
//pub const SHARD_SLOTS_PER_EPOCH: u64 = 128;
//pub const EPOCHS_PER_SHARD_PERIOD: u64 = 256;
//
//// State List Lengths
//pub const HISTORY_ACCUMULATOR_DEPTH: usize = 64;
//
//// Rewards and Penalties
//pub const BLOCK_BODY_PRICE_QUOTIENT: u64 = 8;
//
//// Signature Domain Types
//pub const DOMAIN_SHARD_PROPOSER = DomainType('0x80000000')
//pub const DOMAIN_SHARD_ATTESTER = DomainType('0x81000000')
