use anchor_lang::prelude::*;

use super::SeasonStep;

#[event]
pub struct SeasonStepEvent {
    pub season_number: u8,
    pub current_step: SeasonStep,
}

#[event]
pub struct SeasonMintEvent {
    pub season_number: u8,
    pub sold_art_amount: u8,
}
