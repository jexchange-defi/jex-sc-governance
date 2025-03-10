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
fn cleanup_rs() {
    world().run("scenarios/cleanup.scen.json");
}

#[test]
fn cleanup_incomplete_rs() {
    world().run("scenarios/cleanup_incomplete.scen.json");
}

#[test]
fn cleanup_proposal_not_ended_rs() {
    world().run("scenarios/cleanup_proposal_not_ended.scen.json");
}

#[test]
fn cleanup_proposal_not_found_rs() {
    world().run("scenarios/cleanup_proposal_not_found.scen.json");
}

#[test]
fn cleanup_proposal_not_owner_rs() {
    world().run("scenarios/cleanup_proposal_not_owner.scen.json");
}

#[test]
fn create_proposal_rs() {
    world().run("scenarios/create_proposal.scen.json");
}

#[test]
fn create_proposal_id_already_used_rs() {
    world().run("scenarios/create_proposal_id_already_used.scen.json");
}

#[test]
fn create_proposal_invalid_end_timestamp_rs() {
    world().run("scenarios/create_proposal_invalid_end_timestamp.scen.json");
}

#[test]
fn create_proposal_invalid_label_rs() {
    world().run("scenarios/create_proposal_invalid_label.scen.json");
}

#[test]
fn create_proposal_invalid_start_timestamp_rs() {
    world().run("scenarios/create_proposal_invalid_start_timestamp.scen.json");
}

#[test]
fn create_proposal_not_admin_rs() {
    world().run("scenarios/create_proposal_not_admin.scen.json");
}

#[test]
fn deploy_rs() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn get_proposals_rs() {
    world().run("scenarios/get_proposals.scen.json");
}

#[test]
fn get_proposals_empty_rs() {
    world().run("scenarios/get_proposals_empty.scen.json");
}

#[test]
fn get_voting_power_rs() {
    world().run("scenarios/get_voting_power.scen.json");
}

#[test]
fn get_voting_power_unknown_user_rs() {
    world().run("scenarios/get_voting_power_unknown_user.scen.json");
}

#[test]
fn set_admin_rs() {
    world().run("scenarios/set_admin.scen.json");
}

#[test]
fn set_admin_not_owner_rs() {
    world().run("scenarios/set_admin_not_owner.scen.json");
}

#[test]
fn vote_proposal_rs() {
    world().run("scenarios/vote_proposal.scen.json");
}

#[test]
fn vote_proposal_already_voted_rs() {
    world().run("scenarios/vote_proposal_already_voted.scen.json");
}

#[test]
fn vote_proposal_invalid_choice_rs() {
    world().run("scenarios/vote_proposal_invalid_choice.scen.json");
}

#[test]
fn vote_proposal_no_voting_power_rs() {
    world().run("scenarios/vote_proposal_no_voting_power.scen.json");
}

#[test]
fn vote_proposal_not_found_rs() {
    world().run("scenarios/vote_proposal_not_found.scen.json");
}

#[test]
fn vote_proposal_not_open_rs() {
    world().run("scenarios/vote_proposal_not_open.scen.json");
}
