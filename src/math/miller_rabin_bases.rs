use rug::Integer;

const EMPTY_BASES: [u32; 0] = [];
const BASES_2: [u32; 1] = [2];
const BASES_2_3: [u32; 2] = [2, 3];
const BASES_31_73: [u32; 2] = [31, 73];
const BASES_2_3_5: [u32; 3] = [2, 3, 5];
const BASES_2_3_5_7: [u32; 4] = [2, 3, 5, 7];
const BASES_2_7_61: [u32; 3] = [2, 7, 61];
const BASES_2_13_23_1662803: [u32; 4] = [2, 13, 23, 1662803];
const BASES_2_3_5_7_11: [u32; 5] = [2, 3, 5, 7, 11];
const BASES_2_3_5_7_11_13: [u32; 6] = [2, 3, 5, 7, 11, 13];
const BASES_2_3_5_7_11_13_17: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];
const BASES_2_3_5_7_11_13_17_19_23: [u32; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];
const BASES_ALL: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

pub fn get_miller_rabin_bases(n: &Integer) -> &'static [u32] {
    match n.cmp0() {
        std::cmp::Ordering::Less => &EMPTY_BASES,
        // 2^18 - 1 = 262143
        _ => match n.to_u64().unwrap_or(u64::MAX) {
            0..=2046 => &BASES_2,
            2047..=1373652 => &BASES_2_3,
            1373653..=9080190 => &BASES_31_73,
            9080191..=25326000 => &BASES_2_3_5,
            25326001..=3215031750 => &BASES_2_3_5_7,
            3215031751..=4759123140 => &BASES_2_7_61,
            4759123141..=1122004669632 => &BASES_2_13_23_1662803,
            1122004669633..=2152302898746 => &BASES_2_3_5_7_11,
            2152302898747..=3474749660382 => &BASES_2_3_5_7_11_13,
            3474749660383..=341550071728320 => &BASES_2_3_5_7_11_13_17,
            341550071728321..=3825123056546413050 => &BASES_2_3_5_7_11_13_17_19_23,
            _ => &BASES_ALL,
        },
    }
}
