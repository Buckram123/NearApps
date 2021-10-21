use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PanicOnDefault, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearApps {
    any_contracts: bool,
    any_tags: bool,
    approved_contracts: LookupSet<AccountId>,
}

impl Default for NearApps {
    fn default() -> Self {
        Self {
            any_contracts: false,
            any_tags: false,
            approved_contracts: LookupSet::new(b"c"),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Tags {
    person: String,
    company: String,
    purpose: String,
}

impl Tags {
    pub fn new(person: String, company: String, purpose: String) -> Self {
        Self {
            person,
            company,
            purpose,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractArgs {
    function_name: String,
    params: String,
}

impl ContractArgs {
    pub fn new(function_name: String, params: String) -> Self {
        Self {
            function_name,
            params,
        }
    }
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_verify_contract_name(#[callback] val: bool, tags: Tags) -> bool;
}

#[near_bindgen]
impl NearApps {
    // #[init]
    // pub fn init() -> Self {
    //     Self {
    //         any_contracts: false,
    //         any_tags: false,
    //         approved_contracts: LookupSet::new(b"c"),
    //     }
    // }

    pub fn call(&self, tags: Tags, contract_name: AccountId, args: ContractArgs) {
        self.verify_tags(&tags);
        if self.verify_contract(&contract_name) {
            Promise::new(contract_name)
                .function_call(
                    args.function_name,
                    args.params.into_bytes(),
                    0,
                    env::prepaid_gas() / 3,
                )
                .then(ext_self::callback_verify_contract_name(
                    tags,
                    env::current_account_id(),
                    0,
                    env::prepaid_gas() / 3,
                ));
        }
    }

    fn verify_contract(&self, contract_name: &AccountId) -> bool {
        match self.any_contracts {
            true => true,
            false => self.approved_contracts.contains(contract_name),
        }
    }

    fn verify_tags(&self, _tags: &Tags) {
        if self.any_tags {
            return;
        } else if false {
            env::panic_str("bad tags");
        }
    }

    #[private]
    pub fn add_contract(&mut self, contract_name: AccountId) {
        self.approved_contracts.insert(&contract_name);
    }

    #[private]
    pub fn remove_contract(&mut self, contract_name: AccountId) {
        self.approved_contracts.remove(&contract_name);
    }

    #[private]
    pub fn any_contracts_allowed(&mut self, any: bool) {
        self.any_contracts = any;
    }

    #[private]
    pub fn any_tags_allowed(&mut self, any: bool) {
        self.any_tags = any;
    }

    #[private]
    pub fn callback_verify_contract_name(&mut self, #[callback] val: bool, tags: Tags) -> bool {
        let tags = format!(
            "Person: {}\nCompany: {}\nPurpose: {}",
            tags.person, tags.company, tags.purpose
        );
        env::log_str(&tags);
        val
    }

}
