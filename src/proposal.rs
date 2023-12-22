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
pub trait ProposalModule {
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

    fn mint_proposal_nft(&self, id: u64, content_tx_hash: &ManagedByteArray<Self::Api, 32>) -> u64 {
        let collection_id = self.proposal_nft_collection_id().get();

        let name = sc_format!("{}{}", NFT_NAME_PREFIX, id);

        let nft_nonce = self.send().esdt_nft_create_compact_named(
            &collection_id,
            &BigUint::from(1u32),
            &name,
            &content_tx_hash,
        );

        nft_nonce
    }

    #[view(getProposal)]
    #[storage_mapper("proposal")]
    fn proposal(&self, id: u64) -> SingleValueMapper<Proposal<Self::Api>>;

    #[view(getProposalNftCollectionId)]
    #[storage_mapper("proposal_nft_collection_id")]
    fn proposal_nft_collection_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
