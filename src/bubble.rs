use time::PreciseTime;
use array::swap;

// 冒泡
fn bubble(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    for i in 0..value.len() {
        for m in 0..value.len() - 1 - i {
            if value[m] > value[m + 1] {
                swap(&mut value, m, m + 1);
            }
        }
    }
    value
}

// 优化1
fn bubble1(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    for i in 0..value.len() {
        let mut flag = 0;
        for m in 0..value.len() - 1 - i {
            if value[m] > value[m + 1] {
                swap(&mut value, m, m + 1);
                flag = 1;
            }
        }
        if flag == 0 {
            return value
        }
    }
    value
}

// 优化1+2
fn bubble2(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    let mut last_change = value.len() - 1;
    for _ in 0..value.len() {
        let mut flag = 0;
        for m in 0..last_change {
            if value[m] > value[m + 1] {
                swap(&mut value, m, m + 1);
                flag = 1;
                last_change = m + 1;
            }
        }
        if flag == 0 {
            return value
        }
    }
    value
}

pub fn test(array_value: &Vec<i32>) {
    println!("Start test bubble sort");

    let start_bubble = PreciseTime::now();
    let a = bubble(&array_value);
    let end_bubble = PreciseTime::now();
    println!("bubble take time {}", start_bubble.to(end_bubble));

    let start_bubble = PreciseTime::now();
    let b = bubble1(&array_value);
    let end_bubble = PreciseTime::now();
    println!("bubble1 take time {}", start_bubble.to(end_bubble));

    let start_bubble = PreciseTime::now();
    let c = bubble2(&array_value);
    let end_bubble = PreciseTime::now();
    println!("bubble2 take time {}", start_bubble.to(end_bubble));

//    println!("{:?}", a);
//    println!("{:?}", b);
//    println!("{:?}", c);
    println!("{}, {}", a == b, b == c);
}