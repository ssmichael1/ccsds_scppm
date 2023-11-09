use crate::types::*;
use bitvec::prelude::*;

/// CCSDS SC-PPM Convolutional Encoder
/// See:  https://public.ccsds.org/Pubs/142x0b1.pdf
///
pub fn encode(rate: CodeRate, input: &[u8]) -> Coded {
    let mut g = bitarr![0u8;1];

    let mut c: Coded = Coded::ZERO;
    let mut bitcount: usize = 8;
    let mut input_idx: usize = 0;
    let mut output_idx: usize = 0;

    // Table 3-2
    const PUNCTURE_PATTERNS: [[bool; 6]; 3] = [
        [true, true, true, true, true, true],
        [true, true, false, true, true, false],
        [true, true, false, false, true, false],
    ];

    // Get the puncture pattern for the desired rate
    let puncture_pattern = PUNCTURE_PATTERNS[rate as usize];
    let mut puncture_idx: usize = 0;

    // See section 3.8.2.1
    while output_idx < CODED_LEN_BITS {
        // Get the 3 inputs
        let g0: bool = g[1];
        let g1: bool = g[2];
        let g2: bool = ((input[input_idx] as u16 >> bitcount) & 0x1) == 1;

        g.set(0, g0);
        g.set(1, g1);
        g.set(2, g2);

        // Decrement bit count, and wrap, if needed
        bitcount -= 1;
        if bitcount == 0 {
            bitcount = 8;
            input_idx += 1;
        }

        let h0: bool = g0 ^ g2;
        let h1: bool = g0 ^ g1 ^ g2;

        if puncture_pattern[puncture_idx] == true {
            c.set(output_idx, h0);
            output_idx += 1;
        }
        puncture_idx += 1;
        if puncture_pattern[puncture_idx] == true {
            c.set(output_idx, h1);
            output_idx += 1;
        }
        puncture_idx += 1;
        if puncture_pattern[puncture_idx] == true {
            c.set(output_idx, h1);
            output_idx += 1;
        }
        puncture_idx += 1;
        if puncture_idx == 6 {
            puncture_idx = 0;
        }
    }
    c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mytest() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let vals12: Vec<u8> = (0..UNCODED_R12_LEN_BYTES)
            .map(|_| rng.gen_range(0..=255))
            .collect();

        let enc12 = encode(CodeRate::Rate1_2, vals12.as_ref());
        assert!(enc12.len() == CODED_LEN_BITS);

        let vals13: Vec<u8> = (0..UNCODED_R13_LEN_BYTES)
            .map(|_| rng.gen_range(0..=255))
            .collect();
        let enc13 = encode(CodeRate::Rate1_3, vals13.as_ref());
        assert!(enc13.len() == CODED_LEN_BITS);

        let vals23: Vec<u8> = (0..UNCODED_R23_LEN_BYTES)
            .map(|_| rng.gen_range(0..=255))
            .collect();
        let enc23 = encode(CodeRate::Rate2_3, vals23.as_ref());
        assert!(enc23.len() == CODED_LEN_BITS);
    }
}
