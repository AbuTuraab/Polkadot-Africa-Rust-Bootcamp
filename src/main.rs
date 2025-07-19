use support::Dispatch;
use types::{AccountId, Balance};

mod balances;
mod support;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

// This is an enum that contains all the calls available to our runtime
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

/// This is our runtime, it allows us to interact with all logic in the system.
#[derive(Debug)]
pub struct Runtime {
    pub system: system::Pallet<Self>,
    pub balances: balances::Pallet<Self>,
}

impl Runtime {
    // Create a new instance of the runtime
    fn new() -> Self {
        Runtime {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if self.system.block_number() != block.header.block_number {
            return Err("block number does not match what is expected");
        }

        // (index, item)
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            // call the dispatch function here
            let _res = self.dispatch(caller, call).map_err(|e| {
                // display the error
                // extrinsic number
                // block number
                eprintln!(
                    "Extrinsic Error\n\t Block Number: {}\n\t Extrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    // This is a dispatch call on behalf of the caller
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    // Users
    let femi = String::from("Femi");
    let temi = String::from("temi");
    let cheryl = String::from("cheryl");
    let nathaniel = String::from("nathaniel");
    let faith = String::from("faith");

    // give some money - GENSIS Block
    runtime.balances.set_balance(&cheryl, 1000);

    // Create block 1
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: cheryl.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: faith.clone(),
                    amount: 50,
                }),
            },
            support::Extrinsic {
                caller: cheryl.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: nathaniel.clone(),
                    amount: 70,
                }),
            },
        ],
    };

    // execute block one
    runtime.execute_block(block_1).expect("invalid block");

    // Create block 2
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: cheryl.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: femi.clone(),
                    amount: 100,
                }),
            },
            support::Extrinsic {
                caller: femi.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: temi.clone(),
                    amount: 100,
                }),
            },
        ],
    };

    runtime.execute_block(block_2).expect("invalid block");

    // block 3 : should fail
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![support::Extrinsic {
            caller: cheryl.clone(),
            call: RuntimeCall::Balances(balances::Call::Transfer {
                to: nathaniel.clone(),
                amount: 1200,
            }),
        }],
    };

    runtime.execute_block(block_3).expect("invalid block");

    println!("{:#?}", runtime);
}
