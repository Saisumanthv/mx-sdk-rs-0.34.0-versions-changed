#![no_std]

dharitri_wasm::imports!();

#[dharitri_wasm::contract]
pub trait SendTxRepeat {
    #[init]
    fn init(&self) {}

    #[payable("MOAX")]
    #[endpoint]
    fn repeat(&self, to: ManagedAddress, amount: BigUint, times: usize) {
        for _ in 0..times {
            self.send().direct_moax(&to, &amount);
        }
    }
}
