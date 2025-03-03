use divan::bench;
use manifold_rs::math::witness_accuracy::track_witness_accuracy;
use rug::Integer;

fn main() {
    divan::main();
}

#[divan::bench(args = [0, 1, 2])]
fn track_witness_accuracy_integer(n: u32) {
    track_witness_accuracy(10_u32.pow(n), 10_u32.pow(n + 1));
}
