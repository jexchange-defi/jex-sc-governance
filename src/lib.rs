#![no_std]

multiversx_sc::imports!();

mod locker;
mod locker_sc_proxy;

#[multiversx_sc::contract]
pub trait JexScGovernanceContract: locker::LockerModule {
    #[init]
    fn init(&self, sc_locker_address: ManagedAddress) {
        self.sc_locker_address().set_if_empty(&sc_locker_address);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[view(getVotingPower)]
    fn get_voting_power(&self, address: ManagedAddress) -> BigUint {
        let voting_power = self.get_reward_power(&address);

        voting_power
    }
}
