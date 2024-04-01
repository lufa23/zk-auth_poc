use num_bigint::{BigUint, RandBigInt};
use rand;

/// output = n^exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)
}

/// output = s = k - c * x mod p
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q)
    }
    return q - (c * x -k).modpow(&BigUint::from(1u32), q);

}


/// condition 1: r1 = alpha^s * y1^c
/// condition 2: r2 = beta^s * y2^c
pub fn verify(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c:&BigUint, s: &BigUint, p: &BigUint) -> bool {
    let cond1 = *r1 == (alpha.modpow(s, p) * y1.modpow(c, p)).modpow(&BigUint::from(1u32), &p);
    let cond2 = *r2 == (beta.modpow(s, p) * y2.modpow(c, p)).modpow(&BigUint::from(1u32), &p);
    cond1 && cond2
}


pub fn generate_random_below(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();

    rng.gen_biguint_below(bound)

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_with_random_numbers() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let x = BigUint::from(6u32);
        let k = generate_random_below(&q);

        let c = generate_random_below(&q);

        let y1 = exponentiate(&alpha, &x, &p);
        let y2 = exponentiate(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = exponentiate(&alpha, &k, &p);
        let r2 = exponentiate(&beta, &k, &p);


        
        let s = solve(&k, &c , &x, &q);

        let result = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s, &p);
        assert!(result);


    }
}
