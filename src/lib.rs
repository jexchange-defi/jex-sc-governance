#![no_std]

multiversx_sc::imports!();

mod locker;
mod locker_sc_proxy;
mod proposal;

#[multiversx_sc::contract]
pub trait JexScGovernanceContract: locker::LockerModule + proposal::ProposalModule {
    #[init]
    fn init(&self, sc_locker_address: ManagedAddress, proposal_nft_collection_id: TokenIdentifier) {
        self.sc_locker_address().set_if_empty(&sc_locker_address);
        self.proposal_nft_collection_id()
            .set_if_empty(&proposal_nft_collection_id);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(createProposal)]
    fn create_proposal(
        &self,
        id: u64,
        label: ManagedBuffer,
        start_vote_timestamp: u64,
        end_vote_timestamp: u64,
        nb_choices: u8,
        content_tx_hash: ManagedByteArray<Self::Api, 32>,
    ) -> u64 {
        self.require_is_caller_admin();

        self.do_create_proposal(
            id,
            label,
            start_vote_timestamp,
            end_vote_timestamp,
            nb_choices,
            content_tx_hash,
        )
    }

    /// Cleanup data for a past proposal.
    /// Endpoint should be called until it returns *true* when the cleanup is complete.
    /// Use *max_voters* to limit the gas.
    ///
    /// Return *true* if cleanup is finished, *false* otherwise.
    #[endpoint(cleanup)]
    #[only_owner]
    fn cleanup(&self, proposal_id: u64, max_voters: usize) -> bool {
        self.do_cleanup_voters(proposal_id, max_voters)
    }

    #[endpoint(setAdmin)]
    #[only_owner]
    fn set_admin(&self, address: ManagedAddress) {
        self.admin_address().set(&address);
    }

    /// Note: *choice* starting index = 1
    #[endpoint(vote)]
    fn vote(&self, proposal_id: u64, choice: u8) {
        self.do_vote_proposal(proposal_id, choice);
    }

    fn require_is_caller_admin(&self) {
        require!(
            self.admin_address().get() == self.blockchain().get_caller(),
            "Not admin"
        );
    }

    #[view(getProposals)]
    fn get_proposals(
        &self,
        first: u64,
        last: u64,
    ) -> MultiValueEncoded<proposal::ProposalAndVotes<Self::Api>> {
        let mut res = ManagedVec::<Self::Api, proposal::ProposalAndVotes<Self::Api>>::new();

        for proposal_id in first..last {
            match self.do_get_proposal_and_votes(proposal_id) {
                None => (),
                Some(proposal_and_votes) => res.push(proposal_and_votes),
            }
        }

        res.into()
    }

    #[view(getVotingPower)]
    fn get_voting_power(&self, address: ManagedAddress) -> BigUint {
        self.get_reward_power(&address)
    }

    #[view(getAdminAddress)]
    #[storage_mapper("admin_address")]
    fn admin_address(&self) -> SingleValueMapper<ManagedAddress>;
}
