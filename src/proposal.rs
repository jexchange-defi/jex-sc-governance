multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopDecode, TopEncode, TypeAbi)]
pub struct Proposal<M: ManagedTypeApi> {
    id: u64,
    label: ManagedBuffer<M>,
    nft_nonce: u64,
    start_vote_timestamp: u64,
    end_vote_timestamp: u64,
    nb_choices: u8,
    content_tx_hash: ManagedByteArray<M, 32>,
}

const LABEL_MIN_LENGTH: usize = 10usize;
const LABEL_MAX_LENGTH: usize = 64usize;

const NFT_NAME_PREFIX: &[u8] = b"JIP-";

#[multiversx_sc::module]
pub trait ProposalModule: crate::locker::LockerModule {
    fn do_create_proposal(
        &self,
        id: u64,
        label: ManagedBuffer,
        start_vote_timestamp: u64,
        end_vote_timestamp: u64,
        nb_choices: u8,
        content_tx_hash: ManagedByteArray<Self::Api, 32>,
    ) -> u64 {
        require!(self.proposal(id).is_empty(), "ID already used");

        require!(
            label.len() >= LABEL_MIN_LENGTH && label.len() <= LABEL_MAX_LENGTH,
            "Invalid label"
        );

        require!(
            start_vote_timestamp > self.blockchain().get_block_timestamp(),
            "Invalid start timestamp"
        );

        require!(
            end_vote_timestamp > start_vote_timestamp,
            "Invalid end timestamp"
        );

        let nft_nonce = self.mint_proposal_nft(id, &content_tx_hash);

        let proposal = Proposal::<Self::Api> {
            id,
            label,
            content_tx_hash,
            end_vote_timestamp,
            nft_nonce,
            start_vote_timestamp,
            nb_choices,
        };

        self.proposal(id).set(&proposal);

        nft_nonce
    }

    fn do_vote_proposal(&self, proposal_id: u64, choice: u8) {
        let proposal = self.require_proposal_exists(proposal_id);

        // require vote is open
        let timestamp = self.blockchain().get_block_timestamp();
        require!(
            timestamp >= proposal.start_vote_timestamp && timestamp <= proposal.end_vote_timestamp,
            "Vote is not open"
        );

        // require user did not vote this proposal already
        let caller = self.blockchain().get_caller();
        require!(
            !self.voters(proposal_id).contains(&caller),
            "User has already voted"
        );

        // require valid choice
        require!(
            choice > 0 && choice <= proposal.nb_choices,
            "Invalid choice"
        );

        // require user has voting power
        let voting_power = self.get_reward_power(&caller);
        require!(voting_power > 0, "User has no voting power");

        self.vote_results(proposal_id, choice)
            .update(|x| *x += voting_power);

        self.voters(proposal_id).insert(caller);
    }

    fn mint_proposal_nft(&self, id: u64, content_tx_hash: &ManagedByteArray<Self::Api, 32>) -> u64 {
        let collection_id = self.proposal_nft_collection_id().get();

        let name = sc_format!("{}{}", NFT_NAME_PREFIX, id);
        let uri = sc_format!("mvx://{:x}/data", content_tx_hash.as_managed_buffer());

        let big_zero = BigUint::zero();
        let empty_buffer = ManagedBuffer::new();
        let uris = ManagedVec::from_single_item(uri);

        let nft_nonce = self.send().esdt_nft_create(
            &collection_id,
            &BigUint::from(1u32),
            &name,
            &big_zero,
            &empty_buffer,
            &empty_buffer,
            &uris,
        );

        nft_nonce
    }

    fn require_proposal_exists(&self, proposal_id: u64) -> Proposal<Self::Api> {
        require!(!self.proposal(proposal_id).is_empty(), "Proposal not found");

        self.proposal(proposal_id).get()
    }

    #[view(getProposal)]
    #[storage_mapper("proposal")]
    fn proposal(&self, id: u64) -> SingleValueMapper<Proposal<Self::Api>>;

    #[view(getProposalNftCollectionId)]
    #[storage_mapper("proposal_nft_collection_id")]
    fn proposal_nft_collection_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getVoteResults)]
    #[storage_mapper("vote_results")]
    fn vote_results(&self, proposal_id: u64, choice: u8) -> SingleValueMapper<BigUint>;

    #[view(getVoters)]
    #[storage_mapper("voters")]
    fn voters(&self, proposal_id: u64) -> UnorderedSetMapper<ManagedAddress>;
}
