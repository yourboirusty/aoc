pub fn get_primes_under(n: u64) -> Vec<u64> {
    let mut sieve = vec![true; n as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..sieve.len() {
        if !sieve[i] {
            continue;
        }
        let mut j = i + i;
        while j < sieve.len() {
            sieve[j] = false;
            j += i;
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter(|(_, prime)| **prime)
        .map(|(idx, _)| idx as u64)
        .collect()
}