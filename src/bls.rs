use crate::error::{Result, Error};
use blsful::{Bls12381G2Impl, PublicKey, Signature, SignatureSchemes, SignatureShare, TimeCryptCiphertext};

pub fn encrypt(
    public_key: PublicKey<Bls12381G2Impl>,
    message: &[u8],
    identity: &[u8],
) -> Result<TimeCryptCiphertext<Bls12381G2Impl>> {
    let ciphertext = public_key.encrypt_time_lock(SignatureSchemes::ProofOfPossession, message, &identity)?;
    Ok(ciphertext)
}

pub fn verify_and_decrypt(
    public_key: &PublicKey<Bls12381G2Impl>,
    identity: &[u8],
    ciphertext: &TimeCryptCiphertext<Bls12381G2Impl>,
    shares: &[SignatureShare<Bls12381G2Impl>],
) -> Result<Vec<u8>> {
    if shares.len() < 2 {
        return Err(Error::InsufficientSignatureShares(shares.len()));
    }
    let signature = Signature::from_shares(shares)?;
    signature.verify(public_key, identity)?;

    Option::<Vec<u8>>::from(ciphertext.decrypt(&signature))
        .ok_or_else(|| Error::Decryption)
}
