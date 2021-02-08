use crate::errors::error::Error;

use curve25519_dalek::constants::{RISTRETTO_BASEPOINT_COMPRESSED, RISTRETTO_BASEPOINT_POINT};
use curve25519_dalek::ristretto::RistrettoPoint;
use digest::Input;
use sha3::Sha3_512;

pub type SetupResult<Setup> = Result<Setup, Error>;

#[derive(Copy, Clone)]
pub struct Setup {
    pub generator: RistrettoPoint,
    pub blinding: RistrettoPoint,
}

impl Default for Setup {
    fn default() -> Self {
        Setup {
            generator: RISTRETTO_BASEPOINT_POINT,
            blinding: RistrettoPoint::hash_from_bytes::<Sha3_512>(
                RISTRETTO_BASEPOINT_COMPRESSED.as_bytes(),
            ),
        }
    }
}

pub fn setup<'a>() -> SetupResult<Setup> {
    Ok(Setup::default())
}
