use time::PreciseTime;
use array::swap;

fn adjust(val: &mut Vec<i32>, i: usize, len: usize) {
    let left_index = i * 2 + 1;
    let right_index = i * 2 + 2;
    let mut big_index = i;

    if left_index < len && val[left_index] > val[big_index] {
        big_index = left_index;
    }
    if right_index < len && val[right_index] > val[big_index] {
        big_index = right_index;
    }
    if big_index != i {
        swap(val, big_index,  i);
        adjust(val, big_index, len);
    }
}

fn adjust1(val: &mut Vec<i32>, i: usize, len: usize) {
    let mut parent_index = i;

    while parent_index * 2 <= len {
        let left_index = parent_index * 2 + 1;
        let right_index = parent_index * 2 + 2;
        let mut big_index = parent_index;

        if left_index < len && val[left_index] > val[big_index] {
            big_index = left_index;
        }
        if right_index < len && val[right_index] > val[big_index] {
            big_index = right_index;
        }
        if big_index != parent_index {
            swap(val,big_index, parent_index);
            parent_index = big_index;
        } else {
            break;
        }
    }
}

// 堆
fn head(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    let len = value.len();

    for i in (0..len / 2).rev() {
        adjust(&mut value, i, len);
    }

    for i in (0..len).rev() {
        swap(&mut value, i, 0);
        adjust(&mut value, 0, i);
    }
    value
}

fn head1(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    let len = value.len();

    for i in (0..len / 2).rev() {
        adjust1(&mut value, i, len);
    }

    for i in (0..len).rev() {
        swap(&mut value, i, 0);
        adjust1(&mut value, 0, i);
    }
    value
}

pub fn test(array_value: &Vec<i32>) {
    println!("Start test head sort");

    let start_head = PreciseTime::now();
    let a = head(&array_value);
    let end_head = PreciseTime::now();
    println!("head take time {}", start_head.to(end_head));

    let start_head = PreciseTime::now();
    let b = head1(&array_value);
    let end_head = PreciseTime::now();
    println!("head take time {}", start_head.to(end_head));

    println!("a == b ? {}", a == b);
}