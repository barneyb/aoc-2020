/// I return a `Vec` with all the number's prime factors, in ascending order. If a prime number is
/// passed, the `Vec` will be empty.
///
/// # Examples
///
/// ```
/// use aoc_2020::prime::prime_factorization;
///
/// assert!(prime_factorization(17).is_empty());
/// assert_eq!(vec![2, 2, 5], prime_factorization(20));
/// ```
pub fn prime_factorization(number: usize) -> Vec<usize> {
    prime_factors(number).collect()
}

/// I iterate over the prime factors of the passed composite number, in ascending order. If a prime
/// number is passed, the returned `Factors` will be empty.
///
/// ## Examples
///
/// ```
/// use aoc_2020::prime::prime_factors;
///
/// assert_eq!(None, prime_factors(17).next());
///
/// let mut factors = prime_factors(20);
/// assert_eq!(Some(2), factors.next());
/// assert_eq!(Some(2), factors.next());
/// assert_eq!(Some(5), factors.next());
/// ```
pub fn prime_factors(number: usize) -> PrimeFactors {
    PrimeFactors {
        number,
        n: number,
        sqrt: (number as f64).sqrt() as usize,
        f: 2,
    }
}

pub struct PrimeFactors {
    number: usize,
    n: usize,
    sqrt: usize,
    f: usize,
}

impl Iterator for PrimeFactors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.f <= self.sqrt {
            if self.n % self.f == 0 {
                self.n /= self.f;
                return Some(self.f);
            }
            self.f += 1;
        }
        if self.n > 1 {
            if self.n == self.number {
                self.n = 1;
            } else {
                let n = self.n;
                self.n = 1;
                return Some(n);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prime_factorization() {
        assert!(prime_factorization(0).is_empty());
        assert!(prime_factorization(1).is_empty());
        assert!(prime_factorization(2).is_empty());
        assert!(prime_factorization(3).is_empty());
        assert_eq!(vec![2, 2], prime_factorization(4));
        assert!(prime_factorization(17).is_empty());
        assert_eq!(vec![2, 2, 5], prime_factorization(20));
        assert_eq!(vec![2, 2, 2, 3], prime_factorization(24));
        assert_eq!(vec![5, 5], prime_factorization(25));
        assert_eq!(vec![2, 13], prime_factorization(26));
        assert_eq!(vec![2, 5, 5], prime_factorization(50));
        assert_eq!(vec![3, 3, 3, 5, 13], prime_factorization(1755));
        assert_eq!(
            vec![3, 3, 5, 87887, 4443619],
            prime_factorization(17574135437385)
        );
        // just exercise...
        for n in 0..1005 {
            prime_factorization(n);
        }
    }
}
