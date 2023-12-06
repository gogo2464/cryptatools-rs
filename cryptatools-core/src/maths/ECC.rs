use num_bigint::{BigUint, BigInt};
use num_bigint_dig as num_bigint;
use num_bigint_dig::ModInverse;
use num_traits;
use num_bigint_dig::prime::probably_prime;
use num_bigint_dig::ToBigInt;

#[cfg(feature = "progress-bar")]
use indicatif::ProgressBar;
use primal::Primes;
use std::collections::HashMap;




struct ECC {

}

impl ECC {

}

#[derive(Debug, Clone, Default)]
pub struct Point {
    /// X coordinate of the Point
    pub x_cord: BigInt,
    /// Z coordinate of the Point
    pub z_cord: BigInt,
    /// Parameter of the elliptic curve in Montgomery form
    pub a_24: BigInt,
    /// modulus
    pub modulus: BigInt,
}

impl Point {
    /// Initial parameters for the Point struct.
    ///
    /// # Parameters
    ///
    /// - `x_cord`: X coordinate of the Point
    /// - `z_cord`: Z coordinate of the Point
    /// - `a_24`: Parameter of the elliptic curve in Montgomery form
    /// - `mod`: modulus
    pub fn new(x_cord: BigInt, z_cord: BigInt, a_24: BigInt, modulus: BigInt) -> Point {
        Point {
            x_cord,
            z_cord,
            a_24,
            modulus,
        }
    }

    /// Adds two points `self` and `Q` where `diff = self - Q`.
    ///
    /// This algorithm requires 6 multiplications. The assumption is that `self.x_cord * Q.x_cord * (self.x_cord - Q.x_cord) != 0`.
    /// Using this algorithm speeds up the addition by reducing the number of multiplications required.
    ///
    /// The `mont_ladder` algorithm is constructed in a way that the difference between intermediate points is always equal to the initial point.
    /// So, we always know what the difference between the point is.
    ///
    /// # Parameters
    ///
    /// - `Q`: Point on the curve in Montgomery form.
    /// - `diff`: `self - Q`
    ///
    /// ```
    /// use num_bigint::{BigUint, BigInt};
    /// use num_bigint_dig as num_bigint;
    /// use num_bigint_dig::ModInverse;
    /// use num_traits;
    /// use num_traits::cast::FromPrimitive;
    /// 
    /// use cryptatools_core::maths::ECC::*;
    /// let p1 = Point::new(BigInt::from_u64(11).unwrap(), BigInt::from_u64(16).unwrap(), BigInt::from_u64(7).unwrap(), BigInt::from_u64(29).unwrap());
    /// let p2 = Point::new(BigInt::from_u64(13).unwrap(), BigInt::from_u64(10).unwrap(), BigInt::from_u64(7).unwrap(), BigInt::from_u64(29).unwrap());
    /// let p3 = p2.add(&p1, &p1);
    /// assert_eq!(p3.x_cord, BigInt::from_u64(23).unwrap());
    /// assert_eq!(p3.z_cord, BigInt::from_u64(17).unwrap());
    /// ```
    pub fn add(&self, q: &Point, diff: &Point) -> Point {
        let u = BigInt::from(&self.x_cord - &self.z_cord) * BigInt::from(&q.x_cord + &q.z_cord);
        let v = BigInt::from(&self.x_cord + &self.z_cord) * BigInt::from(&q.x_cord - &q.z_cord);
        let add = BigInt::from(&u + &v);
        let subt = u - v;
        let x_cord = BigInt::from(&diff.z_cord * &add) * &add % &self.modulus;
        let z_cord = BigInt::from(&diff.x_cord * &subt) * &subt % &self.modulus;

        Point::new(x_cord, z_cord, self.a_24.clone(), self.modulus.clone())
    }

    /// Doubles a point in an elliptic curve in Montgomery form.
    ///
    /// ```
    /// use num_bigint::{BigUint, BigInt};
    /// use num_bigint_dig as num_bigint;
    /// use num_bigint_dig::ModInverse;
    /// use num_traits;
    /// use num_traits::cast::FromPrimitive;
    /// use cryptatools_core::maths::ECC::*;
    /// 
    /// let p1 = Point::new(BigInt::from_u64(11).unwrap(), BigInt::from_u64(16).unwrap(), BigInt::from_u64(7).unwrap(), BigInt::from_u64(29).unwrap());
    /// let p2 = p1.double();
    /// assert_eq!(p2.x_cord, BigInt::from(13));
    /// assert_eq!(p2.z_cord, BigInt::from(10));
    /// ```
    pub fn double(&self) -> Point {
        let u = BigInt::from(&self.x_cord + &self.z_cord).sqrt();
        let v = BigInt::from(&self.x_cord - &self.z_cord).sqrt();
        let diff = BigInt::from(&u - &v);
        let x_cord = (u * &v) % &self.modulus;
        let z_cord = ((v + &self.a_24 * &diff) * diff) % &self.modulus;

        Point::new(x_cord, z_cord, self.a_24.clone(), self.modulus.clone())
    }

    /// Scalar multiplication of a point in Montgomery form
    /// using Montgomery Ladder Algorithm.
    /// A total of 11 multiplications are required in each step of this
    /// algorithm.
    ///
    /// # Parameters
    ///
    /// - `k`: The positive integer multiplier
    pub fn mont_ladder(&self, k: &BigInt) -> Point {
        let mut q = self.clone();
        let mut r = self.double();

        for i in format!("{:b}", k)[1..].chars() {
            if i == '1' {
                q = r.add(&q, self);
                r = r.double();
            } else {
                r = q.add(&r, self);
                q = q.double();
            }
        }
        q
    }
}

impl PartialEq for Point {
    /// Two points are equal if X/Z of both points are equal.
    fn eq(&self, other: &Self) -> bool {
        if self.a_24 != other.a_24 || self.modulus != other.modulus {
            false
        } else {
            self.z_cord.clone().mod_inverse(&self.modulus).unwrap() * &self.x_cord % &self.modulus
                == other.z_cord.clone().mod_inverse(&self.modulus).unwrap() * &other.x_cord
                    % &self.modulus
        }
    }
}








/// Error occured during ecm factorization.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Bounds should be an even integer.
    #[error("Bounds should be an even integer")]
    BoundsNotEven,
    /// Too small bounds.
    #[error("Too small bounds")]
    BoundsTooSmall,
    /// The factorization failed.
    #[error("The factorization failed")]
    ECMFailed,
    /// The number is prime.
    #[error("The number is prime")]
    NumberIsPrime,
}

/// Returns one factor of n using Lenstra's 2 Stage Elliptic curve Factorization
/// with Suyama's Parameterization. Here Montgomery arithmetic is used for fast
/// computation of addition and doubling of points in elliptic curve.
///
/// This ECM method considers elliptic curves in Montgomery form (E : b*y^2*z = x^3 + a*x^2*z + x*z^2)
/// and involves elliptic curve operations (mod N), where the elements in Z are reduced (mod N).
/// Since N is not a prime, E over FF(N) is not really an elliptic curve but we can still do point additions
/// and doubling as if FF(N) was a field.
///
/// Stage 1: The basic algorithm involves taking a random point (P) on an elliptic curve in FF(N).
/// The compute k*P using Montgomery ladder algorithm.
/// Let q be an unknown factor of N. Then the order of the curve E, |E(FF(q))|,
/// might be a smooth number that divides k. Then we have k = l * |E(FF(q))|
/// for some l. For any point belonging to the curve E, |E(FF(q))|*P = O,
/// hence k*P = l*|E(FF(q))|*P. Thus kP.z_cord = 0 (mod q), and the unknown factor of N (q)
/// can be recovered by taking gcd(kP.z_cord, N).
///
/// Stage 2: This is a continuation of Stage 1 if k*P != O. The idea is to utilize
/// the fact that even if kP != 0, the value of k might miss just one large prime divisor
/// of |E(FF(q))|. In this case, we only need to compute the scalar multiplication by p
/// to get p*k*P = O. Here a second bound B2 restricts the size of possible values of p.
///
/// Parameters:
///
/// - `n`: Number to be factored.
/// - `B1`: Stage 1 Bound.
/// - `B2`: Stage 2 Bound.
/// - `max_curve`: Maximum number of curves generated.
/// - `rgen`: Random number generator.
pub fn ecm_one_factor(
    n: &BigUint,
    b1: usize,
    b2: usize,
    max_curve: usize,
    rgen: u32,
    #[cfg(feature = "progress-bar")] pb: Option<&ProgressBar>,
) -> Result<BigInt, Error> {
    if b1 % 2 != 0 || b2 % 2 != 0 {
        return Err(Error::BoundsNotEven);
    }

    if probably_prime(n, 0) == true {
        return Err(Error::NumberIsPrime);
    }

    #[cfg(feature = "progress-bar")]
    if let Some(pb) = pb {
        pb.set_length(max_curve as u64);
        pb.set_position(0);
    }

    let mut curve = 0;
    let d = (b2 as f64).sqrt() as usize;
    let two_d = 2 * d;
    let mut beta: Vec<BigInt> = vec![BigInt::default(); d + 1];
    let mut s: Vec<Point> = vec![Point::default(); d + 1];
    let mut k = BigInt::from(1);

    for p in Primes::all().take_while(|&p| p <= b1) {
        k *= p.pow(b1.ilog(p));
    }

    while curve <= max_curve {
        curve += 1;

        #[cfg(feature = "progress-bar")]
        if let Some(pb) = pb {
            pb.inc(1);
        }

        // Suyama's Parametrization
        let sigma = (n - BigInt::from(1).to_biguint().unwrap()).to_bigint().unwrap().random_below(rgen);
        let u = (&sigma * &sigma - BigInt::from(5)) % n;
        let v = (BigInt::from(4) * sigma) % n;
        let diff = v.clone() - u.clone();
        let u_3 = u.clone().pow(3) % n;

        let c = match (BigInt::from(4) * &u_3 * &v).invert(n) {
            Ok(c) => {
                (diff.pow_mod(&BigInt::from(3), n).unwrap() * (BigInt::from(4) * &u + &v) * c
                    - BigInt::from(2))
                    % n
            }
            _ => return Ok((BigInt::from(4) * u_3 * v).gcd(n)),
        };

        let a24 = (c + 2) * BigInt::from(4).invert(n).unwrap() % n;
        let q = Point::new(u_3, v.pow(3) % n, a24, n.clone());
        let q = q.mont_ladder(&k);
        let g = q.z_cord.clone().gcd(n);

        // Stage 1 factor
        if &g != n && g != 1 {
            return Ok(g);
        }

        // Stage 1 failure. Q.z = 0, Try another curve
        if &g == n {
            continue;
        }

        // Stage 2 - Improved Standard Continuation
        s[1] = q.double();
        s[2] = s[1].double();
        beta[1] = BigInt::from(&s[1].x_cord * &s[1].z_cord) % n;
        beta[2] = BigInt::from(&s[2].x_cord * &s[2].z_cord) % n;

        for d in 3..=(d) {
            s[d] = s[d - 1].add(&s[1], &s[d - 2]);
            beta[d] = BigInt::from(&s[d].x_cord * &s[d].z_cord) % n;
        }

        let mut g = BigInt::from(1);
        let b = b1 - 1;
        let mut t = q.mont_ladder(&BigInt::from(b - two_d));
        let mut r = q.mont_ladder(&BigInt::from(b));

        let mut primes = Primes::all().skip_while(|&q| q < b);
        for rr in (b..b2).step_by(two_d) {
            let alpha = BigInt::from(&r.x_cord * &r.z_cord) % n;
            for q in primes.by_ref().take_while(|&q| q <= rr + two_d) {
                let delta = (q - rr) / 2;
                let f = BigInt::from(&r.x_cord - &s[d].x_cord)
                    * BigInt::from(&r.z_cord + &s[d].z_cord)
                    - &alpha
                    + &beta[delta];
                g = (g * f) % n;
            }
            // Swap
            std::mem::swap(&mut t, &mut r);
            r = r.add(&s[d], &t);
        }
        g = g.gcd(n);

        // Stage 2 Factor found
        if &g != n && g != 1 {
            return Ok(g);
        }
    }

    // ECM failed, Increase the bounds
    Err(Error::ECMFailed)
}

fn optimal_b1(digits: usize) -> usize {
    match digits {
        1..=15 => 2000,
        16..=20 => 11000,
        21..=25 => 50000,
        26..=30 => 250000,
        31..=35 => 1000000,
        36..=40 => 3000000,
        41..=45 => 11000000,
        46..=50 => 44000000,
        51..=55 => 110000000,
        56..=60 => 260000000,
        61..=65 => 850000000,
        66..=70 => 2900000000,
        _ => 2900000000,
    }
}

/// Performs factorization using Lenstra's Elliptic curve method.
///
/// This function repeatedly calls `ecm_one_factor` to compute the factors
/// of n. First all the small factors are taken out using trial division.
/// Then `ecm_one_factor` is used to compute one factor at a time.
///
/// # Parameters
///
/// - `n`: Number to be factored.
pub fn ecm(
    n: &BigUint,
    #[cfg(feature = "progress-bar")] pb: Option<&ProgressBar>,
) -> Result<HashMap<BigInt, usize>, Error> {
    ecm_with_params(
        n,
        optimal_b1(n.to_string().len()),
        100000,
        200,
        1234,
        #[cfg(feature = "progress-bar")]
        pb,
    )
}

/// Performs factorization using Lenstra's Elliptic curve method.
///
/// This function repeatedly calls `ecm_one_factor` to compute the factors
/// of n. First all the small factors are taken out using trial division.
/// Then `ecm_one_factor` is used to compute one factor at a time.
///
/// # Parameters
///
/// - `n`: Number to be factored.
/// - `B1`: Stage 1 Bound.
/// - `B2`: Stage 2 Bound.
/// - `max_curve`: Maximum number of curves generated.
/// - `seed`: Initialize pseudorandom generator.
pub fn ecm_with_params(
    n: &BigUint,
    b1: usize,
    b2: usize,
    max_curve: usize,
    seed: usize,
    #[cfg(feature = "progress-bar")] pb: Option<&ProgressBar>,
) -> Result<HashMap<BigInt, usize>, Error> {
    let mut factors = HashMap::new();

    let mut n: BigInt = n.clone();
    for prime in Primes::all().take(100_000) {
        if n.is_divisible_u(prime as u32) {
            let prime = BigInt::from(prime);
            while n.is_divisible(&prime) {
                n /= &prime;
                *factors.entry(prime.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut rand_state = RandState::new();
    rand_state.seed(&seed.into());

    while n != 1 {
        let factor = ecm_one_factor(
            &n,
            b1,
            b2,
            max_curve,
            16,// 16 is an hardcoded random generated number
            #[cfg(feature = "progress-bar")]
            pb,
        )
        .unwrap_or(n.clone());

        while n.is_divisible(&factor) {
            n /= &factor;
            *factors.entry(factor.clone()).or_insert(0) += 1;
        }
    }

    Ok(factors)
}