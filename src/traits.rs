use rand_core::{CryptoRng, RngCore};
use std::ops::{Add, Mul};

/// Scalar trait that enforces the minimal structure we need
pub trait Scalar<T>: Default + Copy + Mul<T, Output = T> + Mul<Output = Self> {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self;
    fn from_bytes(bytes: Vec<u8>) -> Self;
}
/// Group trait that enforces the minimal structure we need
pub trait Group: Default + PartialEq + Copy + Add<Output = Self> {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self;
    fn generator() -> Self;
}
