//! A crate for making and verifying commitments

#![warn(missing_docs)]

//----------------|
// public modules |
//----------------|

// The commitment module: this is the only module the commiter needs
pub mod commit;
// The setup module. Either the trusted party or the verifier needs this
pub mod setup;
// The verify module that the verfier uses to check the commitment after open
pub mod verify;

//------------------|
// internal modules |
//------------------|

// The module that handles the steps of the Pedersen protocol
pub(crate) mod errors;
// The scalar and group traits
pub(crate) mod traits;

//-----------------------|
// external dependencies |
//-----------------------|
extern crate rand_core;
