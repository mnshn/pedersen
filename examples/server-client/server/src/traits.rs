use pedersen::traits::{Group, Scalar};
use std::ops::Deref;

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar as RistrettoScalar;
use derive_more::Add;
use rand_core::OsRng;
use rand_core::{CryptoRng, RngCore};
use sha2::Sha512;
use std::ops::Mul;

#[derive(Default, Clone, Copy, PartialEq, Eq, Add)]
struct WrapPoint(RistrettoPoint);
#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct WrapScalar(RistrettoScalar);

impl Mul<WrapPoint> for WrapScalar {
    type Output = WrapPoint;
    fn mul(self, rhs: WrapPoint) -> WrapPoint {
        WrapPoint(self.0 * rhs.0)
    }
}

impl Deref for WrapPoint {
    type Target = RistrettoPoint;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for WrapScalar {
    type Target = RistrettoScalar;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Scalar<WrapPoint> for WrapScalar {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        Self(RistrettoScalar::random(rng))
    }
    fn from_bytes(input: Vec<u8>) -> Self {
        Self(RistrettoScalar::hash_from_bytes::<Sha512>(&input))
    }
}
impl Group for WrapPoint {
    fn generator() -> Self {
        Self(RISTRETTO_BASEPOINT_POINT)
    }
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        Self(RistrettoPoint::random(rng))
    }
}
