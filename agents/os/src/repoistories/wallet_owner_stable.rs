use std::cell::RefCell;

use crate::{domain::WalletOwner, error::Error, WalletOwnerStable};

use super::WalletOwnerRepository;

pub struct WalletOwnerStableRepositoy<'a> {
    pub owners: &'a RefCell<WalletOwnerStable>,
}

impl<'a> WalletOwnerRepository for WalletOwnerStableRepositoy<'a> {
    fn insert_wallet_owner(
        &mut self,
        owner: candid::Principal,
        canister_id: candid::Principal,
        created_at: u64,
    ) -> Result<WalletOwner, Error> {
        if self.owners.borrow().contains_key(&canister_id) {
            Err(Error::AlreadyExists)
        } else {
            let wallet_owner = WalletOwner {
                canister_id,
                owner,
                created_at,
            };

            self.owners
                .borrow_mut()
                .insert(canister_id, wallet_owner)
                .ok_or(Error::Unknown)
        }
    }

    fn count_wallet(&self) -> u64 {
        self.owners.borrow().len()
    }
}
