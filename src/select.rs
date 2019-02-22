use time::PreciseTime;

fn swap(val: &mut Vec<i32>, a: usize, b: usize) {
    val[a] = val[a] ^ val[b];
    val[b] = val[a] ^ val[b];
    val[a] = val[a] ^ val[b];
}

// 选择
fn select(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    for i in 0..value.len() - 1 {
        let mut min = i;
        for n in i+1..value.len() {
            if value[n] < value[min] {
                min = n;
            }
        }
        if min != i {
            swap(&mut value, min, i);
        }
    }
   value
}

// 优化1
fn select1(val: &Vec<i32>) -> Vec<i32> {
    let mut value = val.clone();
    let mut left = 0;
    let mut right = value.len() - 1;

    while left < right {
        let mut min = left;
        let mut max = right;

        for n in left..right+1 { // 需要循环到right下标,因此需要加1
            if value[min] > value[n] {
                min = n;
            }
            if value[max] < value[n] {
                max = n;
            }
        }
        if min == right && max == left {
            swap(&mut value, left, right);
        } else {
            if min != left {
                swap(&mut value, min, left);
            }
            if max != right {
                swap(&mut value, if max == left { min } else { max }, right);
            }
        }
        left+=1;
        right-=1;
    }
    value
}

pub fn test(array_value: &Vec<i32>) {
    println!("Start test select sort");

    let start_select = PreciseTime::now();
    let a = select(&array_value);
    let end_select = PreciseTime::now();
    println!("select take time {}", start_select.to(end_select));

    let start_select = PreciseTime::now();
    let b = select1(&array_value);
    let end_select = PreciseTime::now();
    println!("select1 take time {}", start_select.to(end_select));

//    println!("{:?}", a);
//    println!("{:?}", b);
    println!("{}", a == b);
}
