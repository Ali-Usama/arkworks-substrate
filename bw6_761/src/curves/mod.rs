use sp_ark_bw6_761::{BW6_761 as BW6_761Host, curves::Config as ConfigHost};

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub type Config = ConfigHost<crate::Host>;
pub type BW6_761 = BW6_761Host<crate::Host>;
