use rand_core::{CryptoRng, RngCore};
use std::ops::{Add, Mul};

pub trait Scalar<T>: Default + Copy + Mul<T, Output = T> + Mul<Output = Self> {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self;
    fn from_bytes(bytes: Vec<u8>) -> Self;
}
pub trait Group: Default + PartialEq + Copy + Add<Output = Self> {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self;
    fn generator() -> Self;
}
