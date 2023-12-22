use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "file:locker-mock/output/locker-mock.wasm",
        locker_mock::ContractBuilder,
    );
    blockchain.register_contract(
        "file:output/jex-sc-governance.wasm",
        jex_sc_governance::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_rs() {
    world().run("scenarios/create_proposal.scen.json");
}
