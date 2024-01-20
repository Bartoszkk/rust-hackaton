#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod forumRoles {
    use ink::storage::Mapping;
    use ink_env::AccountId;
    use parity_scale_codec::{Decode, Encode};



    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Role {
        SuperAdmin,
        Admin,
        Moderator,
        User,
    }

    #[ink(storage)]
    pub struct ForumRoles {
        roles: ink::storage::Mapping<AccountId, Role>,
        super_admin: AccountId,
    }

    impl ForumRoles {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut roles = HashMap::new();
            roles.insert(caller, Role::SuperAdmin);
            Self {
                roles,
                super_admin: caller,
            }
        }

        #[ink(message)]
        pub fn set_role(&mut self, user: AccountId, role: Role) -> bool {
            let caller = self.env().caller();
            if caller == self.super_admin || 
               (role != Role::SuperAdmin && self.roles.get(&caller) == Some(&Role::Admin)) {
                self.roles.insert(user, role);
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn get_role(&self, user: AccountId) -> Role {
            *self.roles.get(&user).unwrap_or(&Role::User)
        }
    }
}
