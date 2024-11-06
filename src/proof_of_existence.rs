use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::<T::Content, T::AccountId>::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err(&"Claim already exists");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.get_claim(&claim) == None {
            return Err(&"Claim does not exist");
        } else if self.get_claim(&claim) != Some(&caller) {
            return Err(&"Invalid owner");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut pallet = super::Pallet::<TestConfig>::new();

        assert_eq!(pallet.get_claim(&"abc"), None);
        assert_eq!(pallet.create_claim(&"alice", &"abc"), Ok(()));
        assert_eq!(pallet.get_claim(&"abc"), Some(&"alice"));
        assert_eq!(
            pallet.create_claim(&"bob", &"abc"),
            Err("Claim already exists")
        );

        assert_eq!(pallet.revoke_claim(&"bob", &"abc"), Err("Invalid owner"));
        assert_eq!(
            pallet.revoke_claim(&"bob", &"abcdef"),
            Err("Claim does not exist")
        );
        assert_eq!(pallet.revoke_claim(&"alice", &"abc"), Ok(()))
    }
}
