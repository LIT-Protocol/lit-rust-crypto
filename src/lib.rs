#[cfg(any(feature = "bls12_381_plus", feature = "rust"))]
pub use bls12_381_plus;
#[cfg(any(feature = "blst", feature = "rust"))]
pub use blsful;
#[cfg(any(feature = "blstrs_plus", feature = "blst"))]
pub use blstrs_plus;
#[cfg(feature = "curve25519-dalek")]
pub use curve25519_dalek;
#[cfg(feature = "decaf377")]
pub use decaf377;
#[cfg(feature = "ed448-goldilocks")]
pub use ed448_goldilocks;
#[cfg(feature = "jubjub")]
pub use jubjub;
#[cfg(feature = "k256")]
pub use k256;
#[cfg(feature = "p256")]
pub use p256;
#[cfg(feature = "p384")]
pub use p384;
#[cfg(feature = "pasta")]
pub use pasta::{self, pallas, vesta};
#[cfg(feature = "vsss-rs")]
pub use vsss_rs::{self, curve25519};

pub use elliptic_curve;
pub use elliptic_curve::ff;
pub use elliptic_curve::group;
#[cfg(feature = "hash2curve")]
pub use elliptic_curve::hash2curve;

#[cfg(feature = "arithmetic")]
pub use elliptic_curve::ops as ec_ops;

#[cfg(feature = "jubjub")]
pub fn red_jubjub_signing_generator() -> jubjub::SubgroupPoint {
    // use group::cofactor::CofactorGroup;
    //
    // const SPEND_AUTH_SIG_BASEPOINT_BYTES: [u8; 32] = [
    //     48, 181, 242, 170, 173, 50, 86, 48, 188, 221, 219, 206, 77, 103, 101, 109, 5, 253, 28, 194,
    //     208, 55, 187, 83, 117, 182, 233, 109, 158, 1, 161, 215,
    // ];
    // let pt: jubjub::ExtendedPoint = jubjub::AffinePoint::from_bytes(&SPEND_AUTH_SIG_BASEPOINT_BYTES)
    //     .expect("to create extended point")
    //     .into();
    // pt.into_subgroup().expect("to convert to subgroup")
    jubjub::SubgroupPoint::new_unchecked(
        [
            0x9fea675eb63e8cf6,
            0x15ba8508eb7f13c5,
            0x87a02da79c8b7ef8,
            0xaf4897169c1851e,
        ],
        [
            0xfb63146264e65a56,
            0x77f3f8c6fd45d5e5,
            0x8770a243986a6eb9,
            0x6dde055ca112d037,
        ],
        [
            0x00000001fffffffe,
            0x5884b7fa00034802,
            0x998c4fefecbc4ff5,
            0x1824b159acc5056f,
        ],
        [
            0x9fea675eb63e8cf6,
            0x15ba8508eb7f13c5,
            0x87a02da79c8b7ef8,
            0x0af4897169c1851e,
        ],
        [
            0xfb63146264e65a56,
            0x77f3f8c6fd45d5e5,
            0x8770a243986a6eb9,
            0x6dde055ca112d037,
        ],
    )
}

#[cfg(feature = "pasta")]
pub fn red_pallas_signing_generator() -> pallas::Point {
    // use group::GroupEncoding;
    // const SPEND_AUTH_SIG_BASEPOINT_BYTES: [u8; 32] = [
    //     99, 201, 117, 184, 132, 114, 26, 141, 12, 161, 112, 123, 227, 12, 127, 12, 95, 68, 95, 62, 124,
    //     24, 141, 59, 6, 214, 241, 40, 179, 35, 85, 183,
    // ];
    // pallas::Affine::from_bytes(&SPEND_AUTH_SIG_BASEPOINT_BYTES.into())
    //     .expect("to affine point")
    //     .into()
    pallas::Point::new_unchecked(
        [
            0x490f248c0abc8698,
            0x8f66e9e53afc5a4a,
            0x8a4b3d0435fe217d,
            0x1088a0c632c10d0b,
        ],
        [
            0xe6900acd3de44a97,
            0x24ecf6ed2f7b5e04,
            0x79d25b841a7b961d,
            0x0cfbbe1efe9c580b,
        ],
        [
            0x34786d38fffffffd,
            0x992c350be41914ad,
            0xffffffffffffffff,
            0x3fffffffffffffff,
        ],
    )
}
