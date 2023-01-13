use rand::Rng;
fn main() {
    // get a vec of random integers between 100 and 10000
    let mut v = (0..100)
        .map(|_| rand::random::<u32>() % 10000 + 100)
        .collect::<Vec<u32>>();
}
