use std::cmp::min;

fn check(num:i64) -> bool {
    let mut has_adjanced = false;
    let mut has_decrease = false;
    let mut min_tail_length = 10;

    let mut buf = num;
    let mut dig;
    let mut prev_dig = 10;
    let mut tail_length = 1;
    while buf > 0 {
        dig = buf % 10;
        if dig == prev_dig {
            has_adjanced = true;
            tail_length += 1;
        } else {
            if tail_length > 1 {
                min_tail_length = min(tail_length, min_tail_length);
            }
            tail_length = 1;
        }
        if dig > prev_dig {
            has_decrease = true;
        }
        buf = buf / 10;
        prev_dig = dig;
    }

    if tail_length > 1 {
        min_tail_length = min(tail_length, min_tail_length);
    }
    
    return has_adjanced && !has_decrease && (min_tail_length <= 2);
}

fn main() {
    let mut counter = 0;
    for i in 145852..616942 {
        if check(i) {
            counter += 1;
        }
    }

    println!("Counter: {}", counter);
}
