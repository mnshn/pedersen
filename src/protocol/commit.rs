use crate::errors::error::CommitError;
use crate::protocol::setup::{setup, Setup, SetupResult};
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;

type CommitResult<Commit> = Result<Commit, CommitError>;

#[derive(Copy, Clone, Debug)]
pub struct Commit {
    setup: Setup,
}

impl Commit {
    fn setup_commit() -> Self {
        let setup = setup();
        let mut rng = OsRng;
        let blinding: Scalar = Scalar::random(&mut rng);
        Commit { setup }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit() {
        println!("{:?}", Commit::setup_commit())
    }
}
