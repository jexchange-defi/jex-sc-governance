multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait LockerModule {
    fn get_reward_power(&self, address: &ManagedAddress) -> (BigUint, u64) {
        let lock_mapper = self.lock_of(self.sc_locker_address().get(), address);

        if !lock_mapper.is_empty() {
            let lock = self
                .locker_sc_proxy(self.sc_locker_address().get())
                .get_lock_of(address)
                .execute_on_dest_context::<crate::locker_sc_proxy::LockOut<Self::Api>>();

            (lock.reward_power, lock.remaining_epochs)
        } else {
            (BigUint::zero(), 0u64)
        }
    }

    /// Check if the given address has deposit funds in the locker.
    /// Using the storage read API because the locker SC does not have a view for that.
    fn has_lock(&self, address: &ManagedAddress) -> bool {
        let lock_mapper = self.lock_of(self.sc_locker_address().get(), address);

        !lock_mapper.is_empty()
    }

    #[view(getLockerScAddress)]
    #[storage_mapper("sc_locker_address")]
    fn sc_locker_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Using the storage because locker SC does not have a view for that.
    #[storage_mapper_from_address("lock_of")]
    fn lock_of(
        &self,
        sc_address: ManagedAddress,
        account: &ManagedAddress,
    ) -> SingleValueMapper<crate::locker_sc_proxy::Lock<Self::Api>, ManagedAddress>;

    #[proxy]
    fn locker_sc_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> crate::locker_sc_proxy::Proxy<Self::Api>;
}
