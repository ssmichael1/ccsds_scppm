use crate::types::*;

pub struct Interleaver {
    pub il: [usize; CODED_LEN_BITS],
    pub dl: [usize; CODED_LEN_BITS],
}

/// CCSDS SC-PPM Convolutional Encoder
/// See:  https://public.ccsds.org/Pubs/142x0b1.pdf
///       Section 3.8.3
impl Interleaver {
    pub fn new() -> Interleaver {
        const A_ILV: u64 = 11;
        const B_ILV: u64 = 210;

        // Create the interleaver
        let mut il = [0usize; CODED_LEN_BITS];
        for idx in 0..CODED_LEN_BITS {
            il[idx] = ((A_ILV * idx as u64 + B_ILV * idx as u64 * idx as u64)
                % CODED_LEN_BITS as u64) as usize;
        }

        Interleaver {
            il: il,
            dl: {
                let mut dl = [0usize; CODED_LEN_BITS];
                for idx in 0..CODED_LEN_BITS {
                    dl[il[idx]] = idx;
                }
                dl
            },
        }
    }
}
