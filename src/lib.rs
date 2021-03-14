//---------------
// public modules
//---------------

// the main commitment module
// pub mod commit;

//----------------
// private modules
//----------------

// the module that handles the steps of the Pedersen protocol
pub(crate) mod errors;
pub(crate) mod protocol;
pub(crate) mod traits;

//----------------
// external crates for TESTING
//----------------

// an elliptic curve module
extern crate curve25519_dalek;
// a random nr generator
extern crate rand_core;
// a hash
extern crate sha2;
