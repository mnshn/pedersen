#![warn(missing_docs)]
//---------------
// public modules
//---------------

// the main commitment module
// pub mod commit;
//
// the traits module
pub mod traits;

//----------------
// internal modules
//----------------

// the module that handles the steps of the Pedersen protocol
pub(crate) mod errors;
pub(crate) mod protocol;

//----------------
// external crates
//----------------

extern crate rand_core;
