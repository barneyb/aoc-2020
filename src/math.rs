/// Finds the multiplicative inverse of `a mod m`.
pub fn mult_inv(a: usize, m: usize) -> usize {
    bin_pow(a, m - 2, m)
}

/// Finds `a ^ b mod m` using binary exponentiation.
pub fn bin_pow(mut a: usize, mut b: usize, m: usize) -> usize {
    a %= m;
    let mut res = 1;
    while b > 0 {
        if b & 1 != 0 {
            res = mult_mod(res, a, m);
        }
        a = mult_mod(a, a, m);
        b >>= 1;
    }
    return res;
}

/// Finds `a * b mod m` while avoiding overflow.
pub fn mult_mod(mut a: usize, mut b: usize, m: usize) -> usize {
    if let Some(r) = a.checked_mul(b) {
        return r % m;
    }
    let mut res = 0;
    a = a % m;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m;
        }
        a = (a * 2) % m;
        b /= 2;
    }
    res % m
}
