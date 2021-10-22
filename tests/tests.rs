use near_sdk::env;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk_sim::{
    call, deploy, init_simulator, to_yocto, ContractAccount, UserAccount, DEFAULT_GAS,
    STORAGE_AMOUNT,
};

extern crate near_apps;
use near_apps::{ContractArgs, NearAppsContract, Tags};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONTRACT_BYTES => "res/near_apps.wasm",
    TEST_FILE_BYTES => "tests/status_message.wasm",
}

fn init() -> (UserAccount, ContractAccount<NearAppsContract>, UserAccount) {
    let mut genesis = near_sdk_sim::runtime::GenesisConfig::default();
    genesis.gas_limit = u64::MAX;
    genesis.gas_price = 0;
    let master_account = init_simulator(Some(genesis));
    let contract_account = deploy! {
        contract: NearAppsContract,
        contract_id: "contract",
        bytes: &CONTRACT_BYTES,
        signer_account: master_account
    };

    let alice = master_account.create_user(
        AccountId::new_unchecked("alice".to_string()),
        to_yocto("10000"),
    );
    alice.deploy(&TEST_FILE_BYTES, "status".parse().unwrap(), to_yocto("35"));
    (master_account, contract_account, alice)
}

#[test]
fn simulate_successful_call() {
    let (master_account, near_apps, _alice) = init();
    let status_id: near_sdk::AccountId = "status".parse().unwrap();
    let status_amt = to_yocto("35");
    /*let res = call!(
        master_account,
        near_apps.deploy_status_message(status_id.clone(), status_amt.into()),
        STORAGE_AMOUNT,
        DEFAULT_GAS
    );
    let promise_outcomes = res.get_receipt_results();
    println!("{:#?}\n{:#?}", promise_outcomes, res);
    */
    let message = "hello world";
    let res = call!(
        master_account,
        near_apps.call(
            Some(
            Tags::new(
                "Mike".to_string(),
                "Near.org".to_string(),
                "testing".to_string(),
            )),
            status_id.clone(),
            ContractArgs::new("set_status".to_string(), message.to_string(),)
        ),
        gas = DEFAULT_GAS * 3
    );
    println!("COMPLEX CALL: {:#?}", res.promise_results());
}
