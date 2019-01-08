


//Because we want the accumulator to be secure without trusted setup, 
//it must also be the case that in the underlying security assumptions 
//the adversary can see the coins used while selecting the concrete module.

//In particular, an accumulator must have a public key divided into two parts, 
//one of which (say, the RSA modulus n) is generated by using a public randomness known by the adversary, 
//and another one (say, a generator of a large subgroup in Z∗n) can be chosen by using a non-public randomness. 

pub mod create_discriminant;
mod cgroup;
pub use cgroup::ClassGroup;