use crate::{config, RedPacket, RedPacketError};
use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

pub fn calculate_random_amount(red_packet: &mut RedPacket, claimer: Pubkey) -> Result<u64> {
    let remaining_amount = red_packet.remaining_amount;
    let remaining_packets = red_packet.packet_count - red_packet.claimed_count;

    if remaining_packets == 0 {
        return Ok(0);
    }

    if remaining_packets == 1 {
        return Ok(remaining_amount);
    }

    let mut hasher = Sha256::new();
    hasher.update(red_packet.random_seed.to_le_bytes());
    hasher.update(claimer.as_ref());
    hasher.update(red_packet.claimed_count.to_le_bytes());
    let result = hasher.finalize();

    let randomness = u64::from_le_bytes(
        result[0..8]
            .try_into()
            .map_err(|_| RedPacketError::RandomnessError)?,
    );
    let max_amount = remaining_amount.min((remaining_amount / remaining_packets as u64) * 2);
    Ok((randomness % max_amount) + 1)
}

pub fn verify_merkle_proof(
    claimer: Pubkey,
    amount: u64,
    proof: &[[u8; 32]],
    merkle_root: &[u8; 32],
) -> Result<()> {
    require!(
        proof.len() <= config::MAX_PROOF_LENGTH,
        RedPacketError::MerkleProofTooLong
    );

    let mut hasher = Sha256::new();
    hasher.update(claimer.as_ref());
    hasher.update(amount.to_le_bytes());
    let mut computed_hash = hasher.finalize();

    for proof_element in proof.iter() {
        let mut hasher = Sha256::new();
        if computed_hash.as_slice() <= proof_element.as_slice() {
            hasher.update(computed_hash.as_slice());
            hasher.update(proof_element.as_slice());
        } else {
            hasher.update(proof_element.as_slice());
            hasher.update(computed_hash.as_slice());
        }
        computed_hash = hasher.finalize();
    }

    require!(
        computed_hash.as_slice() == merkle_root,
        RedPacketError::MerkleProofInvalid
    );

    Ok(())
}
