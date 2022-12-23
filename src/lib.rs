extern crate near_sdk;

use near_sdk::{AccountId, Balance, VMContext};

#[near_sdk::contract]
pub struct Marketplace {
    // Store the account IDs of the sellers in the marketplace
    sellers: Vec<AccountId>,
    // Store the account IDs of the buyers in the marketplace
    buyers: Vec<AccountId>,
    // Store the listings in the marketplace
    listings: Vec<Listing>,
}

#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
struct Listing {
    // The seller of the listing
    seller: AccountId,
    // The item being sold
    item: String,
    // The price of the item
    price: Balance,
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            sellers: Vec::new(),
            buyers: Vec::new(),
            listings: Vec::new(),
        }
    }
}

#[near_sdk::extern_trait]
pub trait MarketplaceInterface {
    fn add_seller(&mut self, seller: AccountId);
    fn add_buyer(&mut self, buyer: AccountId);
    fn create_listing(&mut self, seller: AccountId, item: String, price: Balance);
    fn get_listings(&self) -> Vec<Listing>;
    fn purchase_item(&mut self, buyer: AccountId, listing_id: usize);
}

#[near_sdk::extern_trait]
impl MarketplaceInterface for Marketplace {
    fn add_seller(&mut self, seller: AccountId) {
        self.sellers.push(seller);
    }

    fn add_buyer(&mut self, buyer: AccountId) {
        self.buyers.push(buyer);
    }

    fn create_listing(&mut self, seller: AccountId, item: String, price: Balance) {
        self.listings.push(Listing { seller, item, price });
    }

    fn get_listings(&self) -> Vec<Listing> {
        self.listings.clone()
    }

    fn purchase_item(&mut self, buyer: AccountId, listing_id: usize) {
        let listing = self.listings.remove(listing_id);
        near_sdk::env::transfer_from_contract_to_account(
            listing.seller,
            listing.price,
            near_sdk::env::current_account_id(),
        );
    }
}