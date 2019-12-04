fn check(num:i64) -> bool {
    let mut has_adjanced = false;
    let mut has_decrease = false;

    let mut buf = num;
    let mut dig;
    let mut prev_dig = 10;
    while buf > 0 {
        dig = buf % 10;
        if dig == prev_dig {
            has_adjanced = true;
        }
        if dig > prev_dig {
            has_decrease = true;
        }
        buf = buf / 10;
        prev_dig = dig;
    }
    
    return has_adjanced && !has_decrease;
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
