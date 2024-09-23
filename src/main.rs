use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

fn insertion_sort(arr: &mut [u32]) {
    // Put your code here...
}

#[test]
fn test_insertion_sort() {
    let state = 0xdeadbeef;
    let mut vec: Vec<u32> = (1..100).collect();
    let mut rng = <StdRng as SeedableRng>::seed_from_u64(state);
    vec.shuffle(&mut rng);
    insertion_sort(&mut vec);
    assert_eq!(vec, (1..100).collect::<Vec<u32>>());
}

fn main() {}
