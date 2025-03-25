use std::fmt::Display;

use std::ops::{Add, Div, Mul, Neg, Sub};

/// x^8 + x^4 + x^3 + x + 1
const MOD: u16 = 0b100011011;

/// An element of the galois field GF(2^8) modulo the polynomial x^8 + x^4 + x^3 + x + 1
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Polynomial(u16);

impl Polynomial {
    /// Create a polynomial modulo x^8 + x^4 + x^3 + x + 1 (we do the modulo operation for you!)
    fn new(mut n: u16) -> Polynomial {
        if n == 0 {
            return Polynomial(0);
        }

        while 15 - n.leading_zeros() > 7 {
            let msb = 15 - n.leading_zeros();
            n ^= MOD << (msb - 8);
        }
        Polynomial(n)
    }

    fn degree(self) -> i32 {
        15 - self.0.leading_zeros() as i32
    }

    /// Computes the quotient (left) and remainder (right) of self by rhs.
    fn divalg(mut self, rhs: Polynomial) -> (Polynomial, Polynomial) {
        assert!(rhs != Polynomial(0)); // TODO ?
        let divord = rhs.degree();
        let mut ret = 0;

        // This terminates since divord > 0
        while self.degree() >= divord {
            let curord = self.degree();

            self = Polynomial(self.0 ^ (rhs.0 << (curord - divord)));
            ret ^= 1 << (curord - divord);
        }

        (Polynomial::new(ret), Polynomial::new(self.0))
    }

    fn inverse(self) -> Polynomial {
        let mut vals = Vec::new();
        let mut a = Polynomial(MOD);
        let mut b = self;

        while b.0 != 1 {
            let (div, rem) = a.divalg(b);
            vals.push(div);
            a = b;
            b = rem;
        }

        let mut a = Polynomial(0);
        let mut b = Polynomial(1);

        while let Some(c) = vals.pop() {
            let tmp = b;
            b = c * b + a;
            a = tmp;
        }

        b
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;

        let mut p = self.0;
        if p == 0 {
            write!(f, "0")?;
        } else {
            while p != 0 {
                let msb = 15 - p.leading_zeros();

                if first {
                    write!(f, "x^{msb}")?;
                } else {
                    write!(f, " + x^{msb}")?;
                }
                first = false;

                p ^= 1 << msb;
            }
        }
        Ok(())
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 ^ rhs.0)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 ^ rhs.0)
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        self
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut r = 0;

        let mut b = rhs.0;

        while b != 0 {
            let lsb = b.trailing_zeros();

            r ^= self.0 << lsb;

            b &= b - 1;
        }

        Polynomial::new(r)
    }
}

impl Div for Polynomial {
    type Output = Polynomial;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mul() {
        let p = Polynomial::new(0b10000110);
        let q = Polynomial::new(0b10111110);
        println!("{}", p);
        println!("{}", q);
        println!("{}", p + q);
        println!("{}", p * q);
        assert!(false);
    }

    #[test]
    fn div() {
        let p = Polynomial::new(0x37);
        let q = Polynomial::new(0xca);
        println!("{}", p);
        println!("{}", q);
        println!("{}", p + q);
        println!("{}", p * q);
        println!("{}", p.inverse());
        println!("{}", q.inverse());
        println!("{}", p / q);
        println!("{}", q / p);
        assert!((p/q).inverse() == q/p);
        assert!((q/p).inverse() == p/q);
        assert!((p/p).inverse() == Polynomial(1));
        assert!((q/q).inverse() == Polynomial(1));
        assert!(false);
    }
}
