use num_bigint::{BigUint, BigInt};
use num_bigint_dig as num_bigint;
use num_bigint_dig::ModInverse;
use num_traits;

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