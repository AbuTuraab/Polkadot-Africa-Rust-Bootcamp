// A representation of a block in our blockchain
pub struct Block<Header, Extrinsic> {
    // The block header contains metadata about the block.
    pub header: Header,
    // The extrinsics are a collection of transactions to execute.
    pub extrinsics: Vec<Extrinsic>,
}

// Header struct that contains metadata of the block
pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
    // pub parent_block_hash: String,
}

// Extrinsic struct that contains information about the transaction to execute
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

// Result of the runtime
pub type DispatchResult = Result<(), &'static str>;

// A trait for handling incoming extrinsics
pub trait Dispatch {
    // Who is calling the function.
    type Caller; // these are to be defined by the implementor
    // What function or transaction is being called.
    type Call;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
