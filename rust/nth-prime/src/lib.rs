pub fn nth(n: u32) -> u32 {
    let mut prime_counter = 0;
    let mut cur_prime = 2;

    fn is_prime(x: u32) -> bool {
        for i in 2..x {
            if x % i == 0 {
                return false;
            }
        }
        true
    }

    let mut index = 2;
    while prime_counter != n {
        index += 1;
        if is_prime(index){
            cur_prime = index;
            prime_counter = prime_counter + 1;
        }
    }

    cur_prime
}
