///`ExitPoolRequest(address[],uint256[],bytes,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ExitPoolRequest {
    pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
    pub min_amounts_out: ::std::vec::Vec<::ethers::core::types::U256>,
    pub user_data: ::ethers::core::types::Bytes,
    pub to_internal_balance: bool,
}
///`Config(uint32,uint32,uint32,uint32,uint32,uint32,uint32,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Config {
    pub cushion_factor: u32,
    pub cushion_duration: u32,
    pub cushion_debt_buffer: u32,
    pub cushion_deposit_interval: u32,
    pub reserve_factor: u32,
    pub regen_wait: u32,
    pub regen_threshold: u32,
    pub regen_observe: u32,
}
///`Regen(uint32,uint48,uint32,bool[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Regen {
    pub count: u32,
    pub last_regen: u64,
    pub next_observation: u32,
    pub observations: ::std::vec::Vec<bool>,
}
///`Status((uint32,uint48,uint32,bool[]),(uint32,uint48,uint32,bool[]))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Status {
    pub low: Regen,
    pub high: Regen,
}
///`Instruction(uint8,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Instruction {
    pub action: u8,
    pub target: ::ethers::core::types::Address,
}
///`JoinPoolRequest(address[],uint256[],bytes,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct JoinPoolRequest {
    pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
    pub max_amounts_in: ::std::vec::Vec<::ethers::core::types::U256>,
    pub user_data: ::ethers::core::types::Bytes,
    pub from_internal_balance: bool,
}
///`Permissions(bytes5,bytes4)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Permissions {
    pub keycode: [u8; 5],
    pub func_selector: [u8; 4],
}
///`Band((uint256),(uint256),uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Band {
    pub high: Line,
    pub low: Line,
    pub spread: ::ethers::core::types::U256,
}
///`Line(uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Line {
    pub price: ::ethers::core::types::U256,
}
///`Range((bool,uint48,uint256,uint256,uint256),(bool,uint48,uint256,uint256,uint256),((uint256),(uint256),uint256),((uint256),(uint256),uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Range {
    pub low: Side,
    pub high: Side,
    pub cushion: Band,
    pub wall: Band,
}
///`Side(bool,uint48,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Side {
    pub active: bool,
    pub last_active: u64,
    pub capacity: ::ethers::core::types::U256,
    pub threshold: ::ethers::core::types::U256,
    pub market: ::ethers::core::types::U256,
}
///`RewardsData(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RewardsData {
    pub reward_token: ::ethers::core::types::Address,
    pub outstanding_rewards: ::ethers::core::types::U256,
}
