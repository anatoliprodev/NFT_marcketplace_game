use anchor_lang::prelude::*;

////////////////////////////////
/// To-Do
/// check metaplex's normal sell, or auction and open auction in SC or game server
/// key is the nft is in pda, if it is in admin wallet, gameserver can do
/// copy code from admin_airdrop and use same feature with it.
/// 
/// param:
/// - candymachine pubkeys(2)
/// - season_number
/// - admin wallet pub key
/// action:
/// //// make utils for this auth ////
/// - check admin auth
/// - check step if it's FREELYMINT
/// /////////////////////////////////////
/// Choose one in following cases
/// - allow normal sell
/// - open auction
/// 
////////////////////////////////////////////////////////////////