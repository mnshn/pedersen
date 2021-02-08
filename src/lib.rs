//---------------
// public modules
//---------------

// the main commitment module
pub mod commit;

//----------------
// private modules
//----------------

// the module that handles the steps of the Pedersen protocol
pub(crate) mod protocol;

pub(crate) mod errors;

// the elliptic curve module
use curve25519_dalek;
use digest;
use sha3;
