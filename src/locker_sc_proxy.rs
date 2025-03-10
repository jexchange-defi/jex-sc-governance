multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopDecode, TopEncode)]
pub struct Lock<M: ManagedTypeApi> {
    amount: BigUint<M>,
    unlock_epoch: u64,
}

#[type_abi]
#[derive(ManagedVecItem, NestedDecode, NestedEncode, TopDecode, TopEncode)]
pub struct LockOut<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    unlock_epoch: u64,
    pub remaining_epochs: u64,
    pub reward_power: BigUint<M>,
}

#[multiversx_sc::proxy]
pub trait LockerScProxy {
    #[view(getLockOf)]
    fn get_lock_of(&self, address: &ManagedAddress) -> LockOut<Self::Api>;
}
