use anchor_lang::solana_program;

// pub struct SeasonScheduleEntry {
//     ////////////////////////////////
//     /// Schedule season step(hour unit)
//     ////////////////////////////////
//     pub countdown_duration: u8, // 7 x 24
//     pub premint_duration: u8, // 1 x 24
//     pub premint_wave1_duration: u8,
//     pub premint_blocking_duration: u8, // 7 x 24
//     pub mint_wave3_duration: u8,       // 1 x 24
// }

// pub struct CollectionQuantityLimitationEntry {
//     ////////////////////////////////
//     /// Quantity Limitation
//     ////////////////////////////////
//     pub premint_wave1_amount: u8, // 1000 NFTs
//     pub whitelist_user_max_premint_quantity: u8, // 2 NFTs
//     pub max_freely_mint_quantity: u8,            // 1 NFTs
// }

// pub fn get_current_step(
//     season_opened_timestamp: i64,
//     schedules: SeasonScheduleEntry,
//     quantity_limitation: CollectionQuantityLimitationEntry,
//     current_timestamp: i64,
// ) -> SeasonStep {
//     return SeasonStep::PREMINT;
// }

/// Returns true if a `leaf` can be proved to be a part of a Merkle tree
/// defined by `root`. For this, a `proof` must be provided, containing
/// sibling hashes on the branch from the leaf to the root of the tree. Each
/// pair of leaves and each pair of pre-images are assumed to be sorted.
pub fn merkle_tree_verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash = solana_program::keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash = solana_program::keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}
