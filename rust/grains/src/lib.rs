pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    } else if s == 1 {
        return u64::from(s);
    }

    let mut previous = 1 as u64;
    for _ in 2..=s {
        previous *= 2 as u64;
    }

    previous
}

pub fn total() -> u64 {
    let mut result = 0 as u64;
    for i in 1..=64 {
        result += square(i);
    }
    result
}
