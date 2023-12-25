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

    #[endpoint(setAdmin)]
    #[only_owner]
    fn set_admin(&self, address: ManagedAddress) {
        self.admin_address().set(&address);
    }

    fn require_is_caller_admin(&self) {
        require!(
            self.admin_address().get() == self.blockchain().get_caller(),
            "Not admin"
        );
    }

    #[view(getVotingPower)]
    fn get_voting_power(&self, address: ManagedAddress) -> BigUint {
        let voting_power = self.get_reward_power(&address);

        voting_power
    }

    #[view(getAdminAddress)]
    #[storage_mapper("admin_address")]
    fn admin_address(&self) -> SingleValueMapper<ManagedAddress>;
}
