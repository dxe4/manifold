use lazy_static::lazy_static;
use rug::Integer;

lazy_static! {
    // TODO use array instead of vec?
    static ref EMPTY_BASES: Vec<Integer> = Vec::new();
    static ref BASES_2: Vec<Integer> = vec![Integer::from(2)];
    static ref BASES_2_3: Vec<Integer> = vec![Integer::from(2), Integer::from(3)];
    static ref BASES_31_73: Vec<Integer> = vec![Integer::from(31), Integer::from(73)];
    static ref BASES_2_3_5: Vec<Integer> =
        vec![Integer::from(2), Integer::from(3), Integer::from(5)];
    static ref BASES_2_3_5_7: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
    ];
    static ref BASES_2_7_61: Vec<Integer> =
        vec![Integer::from(2), Integer::from(7), Integer::from(61)];
    static ref BASES_2_13_23_1662803: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(13),
        Integer::from(23),
        Integer::from(1662803),
    ];
    static ref BASES_2_3_5_7_11: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
    ];
    static ref BASES_2_3_5_7_11_13: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
    ];
    static ref BASES_2_3_5_7_11_13_17: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
        Integer::from(17),
    ];
    static ref BASES_2_3_5_7_11_13_17_19_23: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
        Integer::from(17),
        Integer::from(19),
        Integer::from(23),
    ];
    static ref BASES_ALL: Vec<Integer> = vec![
        Integer::from(2),
        Integer::from(3),
        Integer::from(5),
        Integer::from(7),
        Integer::from(11),
        Integer::from(13),
        Integer::from(17),
        Integer::from(19),
        Integer::from(23),
        Integer::from(29),
        Integer::from(31),
        Integer::from(37),
    ];
}

pub fn get_miller_rabin_bases(n: &Integer) -> &'static Vec<Integer> {
    match n.cmp0() {
        std::cmp::Ordering::Less => &EMPTY_BASES,
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
