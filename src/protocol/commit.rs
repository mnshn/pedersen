use crate::errors::error::CommitError;
use crate::protocol::setup::{setup, Setup, SetupResult};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;
use sha2::Sha512;

type CommitResult<Commit> = Result<Commit, CommitError>;

#[derive(Copy, Clone, Debug)]
pub struct Commit {
    commitment: RistrettoPoint,
}

#[allow(non_snake_case)]
impl Commit {
    fn setup_commit(message: &str) -> CommitResult<Commit> {
        let setup = setup();
        let mut rng = OsRng;
        let blinding: Scalar = Scalar::random(&mut rng);
        let generator_G = setup.generator_G;
        let generator_H = setup.generator_H;
        let messages_bytes = message.as_bytes();
        let message_scalar = Scalar::hash_from_bytes::<Sha512>(messages_bytes);
        let commitment = message_scalar * generator_G + blinding * generator_H;
        Ok(Commit { commitment })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit() {
        println!("{:?}", Commit::setup_commit("bak"))
    }
}
