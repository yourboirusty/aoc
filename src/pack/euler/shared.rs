use std::collections::HashMap;

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

pub fn get_prime_factorization(n: u64, primes: &Vec<u64>) -> HashMap<u64, u64> {
    let mut ans = HashMap::new();
    let mut d = n;
    for p in primes {
        if p >= &n {
            break;
        }
        if d % p != 0 {
            continue;
        }
        let mut p_count = 0;
        while d % p == 0 {
            p_count += 1;
            d /= p;
        }
        ans.insert(*p, p_count);
    }
    ans
}

// https://mathschallenge.net/index.php?section=faq&ref=number/sum_of_divisors
pub fn sum_of_divisors(n: u64, primes: &Vec<u64>) -> u64 {
    assert!(primes[primes.len() - 1] >= n);
    let prime_factorization = get_prime_factorization(n, primes);
    prime_factorization
        .iter()
        .fold(1, |acc, (p, k)| acc * (p.pow((k + 1) as u32) - 1) / (p - 1))
}

#[test]
fn test_get_primes_under() {
    assert_eq!(get_primes_under(10), vec![2, 3, 5, 7]);
}

#[test]
fn test_get_prime_factorization() {
    let primes = get_primes_under(10000);
    assert_eq!(
        get_prime_factorization(28, &primes),
        HashMap::from([(2, 2), (7, 1)])
    );
    assert_eq!(
        get_prime_factorization(100, &primes),
        HashMap::from([(2, 2), (5, 2)])
    );
}

#[test]
fn test_get_sum_of_divisors() {
    let primes = get_primes_under(10000);
    assert_eq!(sum_of_divisors(28, &primes), 56);
    assert_eq!(sum_of_divisors(100, &primes), 217);
}
