use crate::utils::setup::{Auction, AuctionAsset, EnglishAuction};
use fuels::types::Identity;

pub(crate) async fn auction_info(auction_id: u64, contract: &EnglishAuction) -> Option<Auction> {
    contract
        .methods()
        .auction_info(auction_id)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn deposit_balance(
    auction_id: u64,
    contract: &EnglishAuction,
    identity: Identity,
) -> Option<AuctionAsset> {
    contract
        .methods()
        .deposit_balance(auction_id, identity)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn total_auctions(contract: &EnglishAuction) -> u64 {
    contract
        .methods()
        .total_auctions()
        .call()
        .await
        .unwrap()
        .value
}
