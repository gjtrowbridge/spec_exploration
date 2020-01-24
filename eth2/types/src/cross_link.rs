use crate::newtypes::{Root, Slot};
use ssz_types::{VariableList};
// TODO: Replace this with the actual max # of slots back each crosslink can hold
use typenum::U64;

pub struct CrossLink {
    shard_roots: VariableList<(Slot, Root), U64>
}
