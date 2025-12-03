use ark_bn254::{Bn254, Fr, G1Projective, G2Projective};
use ark_ec::{CurveGroup, PrimeGroup, pairing::Pairing}; // Need pairing::Pairing trait
use std::ops::Mul;

fn main() {
    let g1 = G1Projective::generator();
    let g2 = G2Projective::generator();

    println!("G1: {:?}", g1);
    println!("G2: {:?}", g2);

    let x = Fr::from(10u64);

    let _p1 = g1 * x;
    let _p2 = g2 * x;

    let zero = Fr::from(0u64); 
    assert_eq!(g1 * (x + zero), g1 * x);

    let two = Fr::from(2u64);

    // Check G1 addition vs multiplication
    assert_eq!(g1 + g1, g1 * two);
    println!("G1 arithmetic matches!");

    // Check G2 addition vs multiplication
    assert_eq!(g2 + g2, g2 * two);
    println!("G2 arithmetic matches!");


    let s3 = Fr::from(3u64);
    let s8 = Fr::from(8u64);
    let s24 = Fr::from(24u64);

    let p = g1 * s3;
    let q = g2 * s8;
    let r = g1 * s24;

    // Calculate pairings
    let lhs = Bn254::pairing(p, q);
    let rhs = Bn254::pairing(r, g2);

    assert_eq!(lhs, rhs);
    println!("Pairing verified!");

    let p1 = g1 * Fr::from(3u64);
    let p2 = g2 * Fr::from(8u64);

    let q1 = g1 * Fr::from(6u64);
    let q2 = g2 * Fr::from(4u64);

    assert_eq!(
        Bn254::pairing(p1, p2), 
        Bn254::pairing(q1, q2)
    );
    println!("Product equality verified!");

    let p1 = g1 * Fr::from(2u64);
    let p2 = g2 * Fr::from(3u64);

    let q1 = g1 * Fr::from(4u64);
    let q2 = g2 * Fr::from(5u64);

    let r1 = g1 * Fr::from(13u64);
    let r2 = g2 * Fr::from(2u64);

    // Calculate Pairings (Result is PairingOutput wrapper)
    let pair1 = Bn254::pairing(p1, p2); // Result 6
    let pair2 = Bn254::pairing(q1, q2); // Result 20
    let pair3 = Bn254::pairing(r1, r2); // Result 26

    // Verify: pair1 * pair2 == pair3
    assert_eq!(pair1.0 * pair2.0, pair3.0);

    println!("GT multiplication verified!");
}