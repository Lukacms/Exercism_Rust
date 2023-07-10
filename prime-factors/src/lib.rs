pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];
    let mut divisor: u64 = 2;
    let mut init_nb: u64 = n;

    while init_nb > 1 {
        while init_nb % divisor == 0 {
            primes.push(divisor);
            init_nb /= divisor;
        }
        divisor += 1;
    }
    primes
}
