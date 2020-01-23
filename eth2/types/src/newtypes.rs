use fixed_hash::{construct_fixed_hash};


// Necessary for impl_common macro
use serde::{Deserialize, Serialize};
use slog;
use std::cmp::{Ord, Ordering};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};
use ssz::{ssz_encode, Decode, DecodeError, Encode};

#[derive(Eq, Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Slot(u64);

#[derive(Eq, Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Epoch(u64);

#[derive(Eq, Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Shard(u64);

#[derive(Eq, Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShardSlot(u64);

impl_common!(Slot);
impl_common!(Epoch);
impl_common!(Shard);
impl_common!(ShardSlot);

construct_fixed_hash! {
    pub struct Root(32);
}