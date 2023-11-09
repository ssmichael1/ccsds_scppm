use bitvec::prelude::*;

pub const UNCODED_R12_LEN_BITS: usize = 7560;
pub const UNCODED_R12_LEN_BYTES: usize = UNCODED_R12_LEN_BITS >> 3;

pub const UNCODED_R13_LEN_BITS: usize = 5040;
pub const UNCODED_R13_LEN_BYTES: usize = UNCODED_R13_LEN_BITS >> 3;

pub const UNCODED_R23_LEN_BITS: usize = 10080;
pub const UNCODED_R23_LEN_BYTES: usize = UNCODED_R23_LEN_BITS >> 3;

pub const CODED_LEN_BITS: usize = 15120;
pub const CODED_LEN_BYTES: usize = CODED_LEN_BITS >> 3;

pub type UncodedR12 = [u8; UNCODED_R12_LEN_BYTES];
pub type Coded = BitArray<[u8; CODED_LEN_BYTES]>;

#[derive(PartialEq, Eq)]
pub enum CodeRate {
    Rate1_3 = 0,
    Rate1_2 = 1,
    Rate2_3 = 2,
}
