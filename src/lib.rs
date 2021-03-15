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
// external crates
//----------------

extern crate rand_core;
