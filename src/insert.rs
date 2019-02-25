use time::PreciseTime;
use array::swap;

// 插入
fn insert(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    for n in 1..value.len() {
        let mut base = n;
        while base > 0 && value[base] < value[base - 1] {
            swap(&mut value, base, base - 1);
            base -= 1;
        }
    }
    value
}

pub fn test(array_value: &Vec<i32>) {
    println!("Start test insert sort");

    let start_insert = PreciseTime::now();
    insert(&array_value);
    let end_insert = PreciseTime::now();
    println!("insert take time {}", start_insert.to(end_insert));
}