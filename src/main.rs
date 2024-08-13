use rand::Rng;
use rand::seq::SliceRandom;

fn insertion_sort(arr: &mut [u32]) {
    // Put your code here...
}

#[test]
fn test_insertion_sort() {
    let mut vec: Vec<u32> = (1..100).collect();
    let mut rng = rand::thread_rng();
    vec.shuffle(&mut rng);
    insertion_sort(&mut vec);
    assert_eq!(vec, (1..100).collect::<Vec<u32>>());
}

fn main() {
}