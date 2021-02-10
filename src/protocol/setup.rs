use crate::errors::error::CommitError;

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;

pub type SetupResult<Setup> = Result<Setup, CommitError>;

#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Setup {
    generator_G: RistrettoPoint,
    generator_H: RistrettoPoint,
    factor: Scalar,
}

impl Setup {
    fn default() -> SetupResult<Self> {
        let mut rng = OsRng;
        let factor: Scalar = Scalar::random(&mut rng);
        match factor {
            factor if factor == Scalar::one() => Err(CommitError::Setup),
            _ => Ok(Setup {
                generator_G: RISTRETTO_BASEPOINT_POINT,
                generator_H: factor * RISTRETTO_BASEPOINT_POINT,
                factor,
            }),
        }
    }
}

pub fn setup() -> Setup {
    Setup::default().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        let setup = setup();
        assert_eq!(setup.generator_H, setup.factor * setup.generator_G)
    }
}
