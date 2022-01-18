use anchor_lang::prelude::*;
use std::fmt;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, PartialOrd)]
pub enum SeasonStep {
    PREPARING = 0,
    COUNTDOWN = 1,
    PREMINT = 2,
    BLOCKING = 3,
    FREELYMINT = 4,
    PUBLISHED = 5,
}

// impl fmt::Debug for SeasonStep {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             SeasonStep::PREPARING => write!(f, "{:?}", "PREPARING"),
//             SeasonStep::COUNTDOWN => write!(f, "{:?}", "COUNTDOWN"),
//             SeasonStep::PREMINT => write!(f, "{:?}", "PREMINT"),
//             SeasonStep::BLOCKING => write!(f, "{:?}", "BLOCKING"),
//             SeasonStep::FREELYMINT => write!(f, "{:?}", "FREELYMINT"),
//             SeasonStep::PUBLISHED => write!(f, "{:?}", "PUBLISHED"),
//         }
//     }
// }

impl fmt::Display for SeasonStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SeasonStep::PREPARING => write!(f, "PREPARING"),
            SeasonStep::COUNTDOWN => write!(f, "COUNTDOWN"),
            SeasonStep::PREMINT => write!(f, "PREMINT"),
            SeasonStep::BLOCKING => write!(f, "BLOCKING"),
            SeasonStep::FREELYMINT => write!(f, "FREELYMINT"),
            SeasonStep::PUBLISHED => write!(f, "PUBLISHED"),
        }
    }
}
