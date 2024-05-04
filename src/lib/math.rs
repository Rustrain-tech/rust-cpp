use num::traits;

/// Returns the greatest common divisor of two numbers.
pub fn gcd<T: traits::PrimInt>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Returns the least common multiple of two numbers.
pub fn lcm<T: traits::PrimInt>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

/// Return (gcd(a, b), x, y) such that ax + by = gcd(a, b).
pub fn extgcd<T: traits::PrimInt>(a: T, b: T) -> (T, T, T) {
    if b == T::zero() {
        (a, T::one(), T::zero())
    } else {
        let (d, x, y) = extgcd(b, a % b);
        (d, y, x - a / b * y)
    }
}

/// Returns a^b.
pub fn pow<T: traits::PrimInt>(mut a: T, mut b: T) -> T {
    let mut ret = T::one();
    while b > T::zero() {
        if b & T::one() == T::one() {
            ret = ret * a;
        }
        a = a * a;
        b = b >> 1;
    }
    ret
}

/// Returns a^b mod m.
pub fn pow_mod<T: traits::PrimInt>(mut a: T, mut b: T, m: T) -> T {
    let mut ret = T::one();
    a = a % m;
    while b > T::zero() {
        if b & T::one() == T::one() {
            ret = (ret * a) % m;
        }
        a = (a * a) % m;
        b = b >> 1;
    }
    ret
}

/// Returns the modular inverse of a mod m.
/// The result x satisfies 0 <= x < m and a * x ≡ 1 (mod m).
pub fn inv_mod<T: traits::PrimInt>(a: T, m: T) -> T {
    let (_, x, _) = extgcd(a, m);
    (x % m + m) % m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3, 7), 1);
        assert_eq!(gcd(12, 18), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(3, 7), 21);
        assert_eq!(lcm(12, 18), 36);
    }

    #[test]
    fn test_extgcd() {
        assert_eq!(extgcd(3, 7), (1, -2, 1));
        assert_eq!(extgcd(12, 18), (6, -1, 1));
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 10), 1024);
        assert_eq!(pow(3, 3), 27);
    }
}
