#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod nft_marketplace {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Nft {
        owner: AccountId,
        price: u32,
        url: String,
        sold_tokens: Mapping<u32, AccountId>,
        total_sold_tokens: u32,
    }

    #[ink(event)]
    pub struct TransferOwnership {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
    }
    #[ink(event)]
    pub struct Transaction {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
    }

    impl Nft{
        #[ink(constructor)]
        pub fn new(url: String, price: u32) -> Self {
            Self {
                owner: Self::env().caller(),
                price:price,
                url:url,
                sold_tokens: Mapping::new(),
                total_sold_tokens: 0,
            }
        }

        // Read Only functions
        #[ink(message)]
        pub fn show_url(&self) -> String {
            self.url.clone()
        }
        #[ink(message)]
        pub fn show_price(&self) -> u32 {
            self.price
        }
        #[ink(message)]
        pub fn show_total_issued_tokens(&self) -> u32 {
            self.total_sold_tokens
        }
        #[ink(message)]
        pub fn show_owner(&self) -> AccountId {
            self.owner.clone()
        }
        #[ink(message)]
        pub fn show_nft_sold_owner(&self, id: u32) -> Option<AccountId> {
            self.sold_tokens.get(id)
        }
        #[ink(message)]
        pub fn is_valid_token(&self, id: u32) -> bool {
            self.sold_tokens.contains(id)
        }

        //Write functions
        #[ink(message)]
        pub fn change_price(&mut self, new_price: u32) {
            if Self::env().caller() == self.owner {
                self.price = new_price;
            } else {
                panic!("You don't own this NFT");
            }
        }
        

        #[ink(message)]
        #[ink(payable)]
        pub fn sell(&mut self, token_owner: AccountId) {
            if Self::env().caller() == self.owner {
                self.sold_tokens
                    .insert(self.total_sold_tokens, &token_owner);
                self.total_sold_tokens += 1;
                self.env().emit_event(Transaction {
                    from: Self::env().caller(),
                    to: token_owner,
                });
            } else {
                panic!("You need to own this NFT to sell it");
            }
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn transfer(&mut self, new_owner: AccountId) {
            if Self::env().caller() == self.owner {
                self.owner = new_owner;
                self.env().emit_event(TransferOwnership {
                    from: Self::env().caller(),
                    to: new_owner,
                })
            } 
            else {
                panic!("You need to be the owner to transfer ownership");
            }
        }
    }
}