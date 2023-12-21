#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(ManagedVecItem, NestedDecode, NestedEncode, TopDecode, TopEncode, TypeAbi)]
pub struct Lock<M: ManagedTypeApi> {
    amount: BigUint<M>,
    unlock_epoch: u64,
}

#[derive(ManagedVecItem, NestedDecode, NestedEncode, TopDecode, TopEncode, TypeAbi)]
pub struct LockOut<M: ManagedTypeApi> {
    amount: BigUint<M>,
    unlock_epoch: u64,
    remaining_epochs: u64,
    reward_power: BigUint<M>,
}

#[multiversx_sc::contract]
pub trait LockerMockContract {
    #[init]
    fn init(&self) {}

    #[view(getLockOf)]
    fn get_lock_of(&self, address: &ManagedAddress) -> LockOut<Self::Api> {
        let lock = self.lock_of(address).get();
        let current_epoch = self.blockchain().get_block_epoch();

        let remaining_epochs = if lock.unlock_epoch > current_epoch {
            lock.unlock_epoch - current_epoch
        } else {
            0u64
        };
        let out = LockOut::<Self::Api> {
            amount: lock.amount.clone(),
            unlock_epoch: lock.unlock_epoch,
            remaining_epochs,
            reward_power: self.calculate_reward_power(&lock.amount, remaining_epochs),
        };

        out
    }

    fn calculate_reward_power(&self, amount: &BigUint, remaining_epochs: u64) -> BigUint {
        let rp = ((amount * remaining_epochs * 3u32) / 180u32) + amount;

        rp.max(amount.clone())
    }

    #[storage_mapper("lock_of")]
    fn lock_of(&self, address: &ManagedAddress) -> SingleValueMapper<Lock<Self::Api>>;
}
