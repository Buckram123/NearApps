use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Promise, PromiseResult};

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

#[derive(Serialize, Deserialize, Debug)]
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

    #[payable]
    pub fn call(&mut self, tags: Option<Tags>, contract_name: AccountId, args: ContractArgs) {
        self.verify_tags(&tags);
        if self.verify_contract(&contract_name) {
            let p0 = env::promise_create(
                contract_name,
                &args.function_name,
                &args.params.into_bytes(),
                env::attached_deposit(),
                env::prepaid_gas() / 3,
            );
            env::promise_then(
                p0,
                env::current_account_id(),
                "check_promise",
                json!({ "tags": tags }).to_string().as_bytes(),
                0,
                env::prepaid_gas() / 3,
            );
        }
    }

    fn verify_contract(&self, contract_name: &AccountId) -> bool {
        match self.any_contracts {
            true => true,
            false => self.approved_contracts.contains(contract_name),
        }
    }

    fn verify_tags(&self, tags: &Option<Tags>) {
        if self.any_tags {
            return;
        } else if tags.is_none() {
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
    pub fn check_promise(&mut self, tags: Option<Tags>) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                if let Some(tags) = tags {
                    let output = format!(
                        "Person: {}\nCompany: {}\nPurpose: {}",
                        tags.person, tags.company, tags.purpose
                    );
                    env::log_str(&output);
                }
            }
            _ => env::panic_str("Promise with index 0 failed"),
        };
    }
}
