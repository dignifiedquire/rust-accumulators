//! Prime Group Backend for our Accumulstors

//
//Classgroup Backend
//
#[cfg(feature = "class_group")]
mod classgroup;

#[cfg(feature = "class_group")]
pub use self::classgroup::ClassGroup;

//
//RSA Backend
//

#[cfg(feature = "rsa_group")]
mod rsa;
#[cfg(feature = "rsa_group")]
pub use self::rsa::RSAGroup;
