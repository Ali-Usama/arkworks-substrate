use ark_ec::short_weierstrass::Affine;
use sp_ark_bls12_381::{
    g1::{endomorphism as endomorphism_host, Config as ConfigHost},
    G1Affine as G1AffineHost, G1Projective as G1ProjectiveHost,
};

pub type Config = ConfigHost<crate::Host>;

pub type G1Affine = G1AffineHost<crate::Host>;
pub type G1Projective = G1ProjectiveHost<crate::Host>;

pub use sp_ark_bls12_381::g1::{BETA, G1_GENERATOR_X, G1_GENERATOR_Y};

pub fn endomorphism(p: &Affine<Config>) -> Affine<Config> {
    endomorphism_host::<crate::Host>(p)
}
