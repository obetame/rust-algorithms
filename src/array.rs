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

pub fn swap(val: &mut Vec<i32>, a: usize, b: usize) {
    val[a] = val[a] ^ val[b];
    val[b] = val[a] ^ val[b];
    val[a] = val[a] ^ val[b];
}