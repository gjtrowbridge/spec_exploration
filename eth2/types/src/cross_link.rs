use crate::newtypes::{Root, Slot};

pub struct CrossLink {
    shard_roots: Vec<(Slot, Root)>
}
