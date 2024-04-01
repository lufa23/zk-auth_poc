use num_bigint::{BigUint, RandBigInt};
use rand;


pub struct ZKP {
    p: BigUint,
    q: BigUint,
    alpha: BigUint,
    beta: BigUint,

}
impl ZKP {

/// output = n^exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)
}

/// output = s = k - c * x mod p
pub fn solve(&self,k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), &self.q)
    }
    return &self.q - (c * x -k).modpow(&BigUint::from(1u32), &self.q);

}


/// condition 1: r1 = alpha^s * y1^c
/// condition 2: r2 = beta^s * y2^c
pub fn verify(&self, r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, c:&BigUint, s: &BigUint) -> bool {
    let cond1 = *r1 == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p)).modpow(&BigUint::from(1u32), &self.p);
    let cond2 = *r2 == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p)).modpow(&BigUint::from(1u32), &self.p);
    cond1 && cond2
}


pub fn generate_random_below(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();

    rng.gen_biguint_below(bound)

}
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
        let zkp = ZKP{p: p.clone(), q: q.clone(),alpha: alpha.clone(), beta: beta.clone()};

        let x = BigUint::from(6u32);
        let k = ZKP::generate_random_below(&q);

        let c = ZKP::generate_random_below(&q);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);


        
        let s = zkp.solve(&k, &c , &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);


    }
}
