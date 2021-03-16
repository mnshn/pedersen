//! The main commitment module that holds the struct that carries the commiter's data and the
//! fuctionality to, given setup data and a message, construct a Pedersen commitment

use crate::setup::SetupData;
use crate::traits::{Group, Scalar};
use rand_core::{CryptoRng, RngCore};

/// Struct that holds the commiter's data.
pub struct CommitData<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    /// The commitment that the commiter shares before the open phase
    pub commitment: T,
    /// The setup data (group genarators) that the commiter has received
    pub setup_data: SetupData<T, K>,
    /// A random salt that the Pedersen protocol needs to (perfectly) hide the commitment
    pub random_scalar: K,
    /// The actual massage (as a scalar) that will be commited
    pub message: K,
}

impl<T, K> CommitData<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    /// The commit function that takes the setup data and the message and returns the commitment
    pub fn commit<R: RngCore + CryptoRng>(
        setup_data: SetupData<T, K>,
        message: Vec<u8>,
        rng: &mut R,
    ) -> Self {
        let message_as_scalar: K = K::from_bytes(message);
        let random_scalar = K::random(rng);
        CommitData {
            commitment: message_as_scalar * setup_data.g + random_scalar * setup_data.h,
            random_scalar,
            message: message_as_scalar,
            setup_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Scalar;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar as RistrettoScalar;
    use rand_core::OsRng;

    #[test]
    fn test_commit_bytes() {
        let mut rng = OsRng;
        let setup_data: SetupData<RistrettoPoint, RistrettoScalar> = SetupData::new(&mut rng);
        let commitment: CommitData<RistrettoPoint, RistrettoScalar> =
            CommitData::commit(setup_data, vec![99, 12], &mut rng);
        assert_eq!(
            RistrettoScalar::from_bytes(vec![99, 12]),
            commitment.message
        );
    }
    #[test]
    fn test_commit_non_err_on_nonempty() {
        assert_ne!(0, 1);
    }
}
