use multiversx_sc::api::{const_handles::MBUF_TEMPORARY_1, use_raw_handle};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait LockerModule {
    fn get_reward_power(&self, address: &ManagedAddress) -> BigUint {
        let voting_power;

        if self.has_lock(address) {
            let lock = self
                .locker_sc_proxy(self.sc_locker_address().get())
                .get_lock_of(address)
                .execute_on_dest_context::<crate::locker_sc_proxy::LockOut<Self::Api>>();

            voting_power = lock.reward_power;
        } else {
            voting_power = BigUint::zero();
        }

        voting_power
    }

    /// Check if the given address has deposit funds in the locker.
    /// Using the storage read API because the locker SC does not have a view for that.
    fn has_lock(&self, address: &ManagedAddress) -> bool {
        let mut storage_key = ManagedBuffer::from("lock_of");
        storage_key.append(address.as_managed_buffer());

        let result_buffer = ManagedBuffer::from_handle(use_raw_handle(MBUF_TEMPORARY_1));

        use multiversx_sc::api::{StorageReadApi, StorageReadApiImpl};

        Self::Api::storage_read_api_impl().storage_load_from_address(
            self.sc_locker_address().get().get_handle(),
            storage_key.get_handle(),
            result_buffer.get_handle(),
        );

        let res = crate::locker_sc_proxy::Lock::<Self::Api>::top_decode(result_buffer);

        res.is_ok()
    }

    #[view(getLockerScAddress)]
    #[storage_mapper("sc_locker_address")]
    fn sc_locker_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[proxy]
    fn locker_sc_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> crate::locker_sc_proxy::Proxy<Self::Api>;
}
