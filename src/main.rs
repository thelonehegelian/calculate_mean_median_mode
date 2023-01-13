use rand::Rng;
fn main() {
    /************
     * FIND MEAN
     ************/
    // get a vec of random integers between 100 and 10000
    let mut data = (0..100)
        .map(|_| rand::random::<u32>() % 10000 + 100)
        .collect::<Vec<u32>>();

    let sum = data.iter().fold(0, |acc, x| acc + x);
    let len = data.len();
    let mean = sum as f64 / len as f64;
    println!("mean: {}", mean);

    /************
     * FIND MEDIAN
     ************/
}
