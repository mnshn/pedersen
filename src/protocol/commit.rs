use crate::errors::error::CommitError;
use crate::protocol::setup::{setup, Setup, SetupResult};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;
use sha2::Sha512;

type CommitResult<Commit> = Result<Commit, CommitError>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Commit {
    commitment: RistrettoPoint,
}

#[allow(non_snake_case)]
impl Commit {
    fn setup_commit(message: String) -> CommitResult<Commit> {
        match message {
            message if message.len() == 0 => Err(CommitError::Commit),
            _ => {
                let setup = setup();
                let mut rng = OsRng;
                let blinding: Scalar = Scalar::random(&mut rng);
                let generator_G = setup.generator_G;
                let generator_H = setup.generator_H;
                let messages_bytes = message.as_bytes();
                let message_scalar = Scalar::hash_from_bytes::<Sha512>(messages_bytes);
                let commitment = message_scalar * generator_G + blinding * generator_H;
                return Ok(Commit { commitment });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_fails_empty() {
        let commit = Commit::setup_commit("".into());
        assert_eq!(commit, Err(CommitError::Commit));
    }
    #[test]
    fn test_commit_non_err_on_nonempty() {
        let commit = Commit::setup_commit("bongo".into());
        assert_ne!(commit, Err(CommitError::Commit));
    }
}
