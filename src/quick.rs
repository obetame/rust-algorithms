use time::PreciseTime;
use array::swap;

fn quick(val: &Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    let mut value = vec![6, 2, 6, 7, 4, 7, 7, 3, 1, 6];
    let mut mid = (left + right) / 2;
    let mut left_index = left;
    let mut right_index = right;
    if value.len() == 0 {
        return value;
    }

    while left_index <= right_index {
        while value[left_index] < value[mid] {
            left_index += 1;
        }
        while value[right_index] > value[mid] {
            right_index -= 1;
        }
        println!("left: {}, right: {}, array: {:?}", left_index, right_index, value);
        if left_index <= right_index  {
            swap(&mut value, left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }
    }

    value
}

pub fn test(array_value: &Vec<i32>) {
    println!("Start test quick sort");
    if array_value.len() < 100 {
        println!("{:?}", array_value);
    }

    let start_quick = PreciseTime::now();
    let a = quick(&array_value, 0, array_value.len() - 1);
    let end_quick = PreciseTime::now();
    println!("quick take time {}", start_quick.to(end_quick));
    println!("{:?}", a);
}