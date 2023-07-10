pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3, 5];
    let mut is_prime;
    let mut count: u32 = 3;

    for x in (7..u32::MAX).step_by(2) {
        is_prime = true;
        for prime in &primes {
            if x % prime == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            primes.push(x);
            count += 1;
        }
        if count >= (n + 1) {
            break;
        }
    }
    primes[n as usize]
}
