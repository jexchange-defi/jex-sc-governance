use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn cleanup_go() {
    world().run("scenarios/cleanup.scen.json");
}

#[test]
fn cleanup_incomplete_go() {
    world().run("scenarios/cleanup_incomplete.scen.json");
}

#[test]
fn cleanup_proposal_not_ended_go() {
    world().run("scenarios/cleanup_proposal_not_ended.scen.json");
}

#[test]
fn cleanup_proposal_not_found_go() {
    world().run("scenarios/cleanup_proposal_not_found.scen.json");
}

#[test]
fn cleanup_proposal_not_owner_go() {
    world().run("scenarios/cleanup_proposal_not_owner.scen.json");
}

#[test]
fn create_proposal_go() {
    world().run("scenarios/create_proposal.scen.json");
}

#[test]
fn create_proposal_id_already_used_go() {
    world().run("scenarios/create_proposal_id_already_used.scen.json");
}

#[test]
fn create_proposal_invalid_end_timestamp_go() {
    world().run("scenarios/create_proposal_invalid_end_timestamp.scen.json");
}

#[test]
fn create_proposal_invalid_label_go() {
    world().run("scenarios/create_proposal_invalid_label.scen.json");
}

#[test]
fn create_proposal_invalid_start_timestamp_go() {
    world().run("scenarios/create_proposal_invalid_start_timestamp.scen.json");
}

#[test]
fn create_proposal_not_admin_go() {
    world().run("scenarios/create_proposal_not_admin.scen.json");
}

#[test]
fn deploy_go() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn get_proposals_go() {
    world().run("scenarios/get_proposals.scen.json");
}

#[test]
fn get_proposals_empty_go() {
    world().run("scenarios/get_proposals_empty.scen.json");
}

#[test]
fn get_voting_power_go() {
    world().run("scenarios/get_voting_power.scen.json");
}

#[test]
fn get_voting_power_unknown_user_go() {
    world().run("scenarios/get_voting_power_unknown_user.scen.json");
}

#[test]
fn set_admin_go() {
    world().run("scenarios/set_admin.scen.json");
}

#[test]
fn set_admin_not_owner_go() {
    world().run("scenarios/set_admin_not_owner.scen.json");
}

#[test]
fn vote_proposal_go() {
    world().run("scenarios/vote_proposal.scen.json");
}

#[test]
fn vote_proposal_already_voted_go() {
    world().run("scenarios/vote_proposal_already_voted.scen.json");
}

#[test]
fn vote_proposal_invalid_choice_go() {
    world().run("scenarios/vote_proposal_invalid_choice.scen.json");
}

#[test]
fn vote_proposal_lock_too_short_go() {
    world().run("scenarios/vote_proposal_lock_too_short.scen.json");
}

#[test]
fn vote_proposal_no_voting_power_go() {
    world().run("scenarios/vote_proposal_no_voting_power.scen.json");
}

#[test]
fn vote_proposal_not_found_go() {
    world().run("scenarios/vote_proposal_not_found.scen.json");
}

#[test]
fn vote_proposal_not_open_go() {
    world().run("scenarios/vote_proposal_not_open.scen.json");
}
