use time::PreciseTime;

fn quick_sort(value: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let mut left_index = left;
        let mut right_index = right;
        let base = value[left_index];

        while left_index < right_index {
            while left_index < right_index && value[right_index] >= base {
                right_index -= 1;
            }
            if left_index < right_index {
                value[left_index] = value[right_index];
                left_index += 1;
            }
            while left_index < right_index && value[left_index] < base {
                left_index += 1;
            }
            if left_index < right_index {
                value[right_index] = value[left_index];
                right_index -= 1;
            }
        }
        value[left_index] = base;
        if left_index != 0 {
            // make sure right params more than 0
            quick_sort(value, left, left_index - 1);
        }
        quick_sort(value, left_index + 1, right);
    }
}

fn quick(val: &Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    let mut value = val.clone();

    quick_sort(&mut value, left, right);

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
//    println!("{:?}", a);
}