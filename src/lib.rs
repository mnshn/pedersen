//---------------
// public modules
//---------------

// the main commitment module
pub mod commit;

//----------------
// private modules
//----------------

// the module that handles the steps of the Pedersen protocol
pub(crate) mod errors;
pub(crate) mod protocol;

//----------------
// external crates
//----------------

// the elliptic curve module
extern crate curve25519_dalek;
// randum number generator
extern crate rand_core;
