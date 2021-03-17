use crate::setup::SetupData;
use crate::traits::{Group, Scalar};

pub struct VerifyData<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    commitment: T,
    setup_data: SetupData<T, K>,
    random_scalar: K,
    message: K,
    open: bool,
}

impl<T, K> VerifyData<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    fn receive_commit(commitment: T, setup_data: SetupData<T, K>) -> Self {
        VerifyData {
            commitment,
            setup_data,
            random_scalar: K::default(),
            message: K::default(),
            open: false,
        }
    }
    fn receive_open(commit_data: Self, random_scalar: K, message: K) -> Result<bool, String> {
        if commit_data.open {
            return Err("Already opened".into());
        }
        let verify_data = VerifyData {
            random_scalar,
            message,
            open: true,
            ..commit_data
        };

        return Ok(
            message * verify_data.setup_data.g + random_scalar * verify_data.setup_data.h
                == verify_data.commitment,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commit::CommitData;
    use crate::traits::Scalar;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar as RistrettoScalar;
    use rand_core::OsRng;

    #[test]
    fn test_verify() {
        let mut rng = OsRng;
        let setup_data: SetupData<RistrettoPoint, RistrettoScalar> = SetupData::new(&mut rng);
        let commitment: CommitData<RistrettoPoint, RistrettoScalar> =
            CommitData::commit(setup_data, vec![99, 12], &mut rng);
        let receive_commit =
            VerifyData::receive_commit(commitment.commitment, commitment.setup_data);
        let receive_open =
            VerifyData::receive_open(receive_commit, commitment.random_scalar, commitment.message)
                .unwrap();
        assert_eq!(receive_open, true);
    }
}
