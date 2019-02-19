use rand::Rng;

pub fn init() -> Vec<i32> {
    let n = 10000;
    let mut array = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..n {
        array.push(rng.gen_range(0, n));
    }

    array
}