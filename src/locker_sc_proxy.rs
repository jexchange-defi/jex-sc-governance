multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopDecode, TopEncode, TypeAbi)]
pub struct Lock<M: ManagedTypeApi> {
    amount: BigUint<M>,
    unlock_epoch: u64,
}

#[derive(ManagedVecItem, NestedDecode, NestedEncode, TopDecode, TopEncode, TypeAbi)]
pub struct LockOut<M: ManagedTypeApi> {
    amount: BigUint<M>,
    unlock_epoch: u64,
    remaining_epochs: u64,
    pub reward_power: BigUint<M>,
}

#[multiversx_sc::proxy]
pub trait LockerScProxy {
    #[view(getLockOf)]
    fn get_lock_of(&self, address: &ManagedAddress) -> LockOut<Self::Api>;
}
