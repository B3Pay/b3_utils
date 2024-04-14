use ic_bls12_381::{G1Affine, G2Affine, G2Prepared, Gt};

use self::error::PairingError;

mod error;

const G1AFFINE_BYTES: usize = 48; // Size of compressed form
const G2AFFINE_BYTES: usize = 96; // Size of compressed form

pub fn gt_multipairing(terms: &[(&G1Affine, &G2Prepared)]) -> Gt {
    ic_bls12_381::multi_miller_loop(terms).final_exponentiation()
}

pub fn option_from_ctoption<T>(ctoption: subtle::CtOption<T>) -> Option<T> {
    if bool::from(ctoption.is_some()) {
        Some(ctoption.unwrap())
    } else {
        None
    }
}

pub fn deserialize_g1(bytes: &[u8]) -> Result<G1Affine, PairingError> {
    let bytes: &[u8; G1AFFINE_BYTES] = bytes.try_into().map_err(|_| PairingError::InvalidLength)?;

    let pt = G1Affine::from_compressed(bytes);
    if bool::from(pt.is_some()) {
        Ok(pt.unwrap())
    } else {
        Err(PairingError::InvalidCurve)
    }
}

pub fn deserialize_g2(bytes: &[u8]) -> Result<G2Affine, PairingError> {
    let bytes: &[u8; G2AFFINE_BYTES] = bytes.try_into().map_err(|_| PairingError::InvalidLength)?;

    let pt = G2Affine::from_compressed(bytes);
    if bool::from(pt.is_some()) {
        Ok(pt.unwrap())
    } else {
        Err(PairingError::InvalidCurve)
    }
}