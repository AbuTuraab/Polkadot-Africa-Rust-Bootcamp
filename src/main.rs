mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
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

    // create a block
    // increase block number
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), faith.clone(), 50)
        .map_err(|e| println!("error: {}", e));

    // second transaction
    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), nathaniel.clone(), 70)
        .map_err(|e| println!("error: {}", e));

    // Create block 2
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 2);

    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), femi.clone(), 100)
        .map_err(|e| println!("error: {}", e));

    runtime.system.inc_nonce(&femi);
    let _res = runtime
        .balances
        .transfer(femi.clone(), temi.clone(), 100)
        .map_err(|e| println!("error: {}", e));

    // block 3 : should fail
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 3);

    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), nathaniel.clone(), 1200)
        .map_err(|e| println!("error: {}", e));

    println!("{:#?}", runtime);
}
