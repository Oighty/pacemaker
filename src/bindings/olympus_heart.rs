pub use olympus_heart::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod olympus_heart {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("kernel_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Kernel"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("operator_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IOperator"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("rewardToken_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("maxReward_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("auctionDuration_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint48"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ROLES"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ROLES"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ROLESv1"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("activate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("activate"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("active"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("active"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("auctionDuration"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("auctionDuration"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint48"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beat"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("beat"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeKernel"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("changeKernel"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newKernel_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract Kernel"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("configureDependencies"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("configureDependencies",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dependencies"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(5usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("Keycode[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentReward"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("currentReward"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deactivate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deactivate"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("frequency"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("frequency"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint48"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isActive"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isActive"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("kernel"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("kernel"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract Kernel"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastBeat"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lastBeat"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint48"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxReward"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("maxReward"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operator"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IOperator"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestPermissions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestPermissions"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("permissions"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(5usize),
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Permissions[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resetBeat"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("resetBeat"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rewardToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ERC20"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOperator"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRewardAuctionParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setRewardAuctionParams",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maxReward_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("auctionDuration_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint48"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawUnspentRewards"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawUnspentRewards",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ERC20"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Beat"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Beat"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("timestamp_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardIssued"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RewardIssued"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rewardAmount_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RewardUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("maxRewardAmount_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("auctionDuration_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Heart_BeatAvailable"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Heart_BeatAvailable",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Heart_BeatStopped"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Heart_BeatStopped"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Heart_InvalidParams"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Heart_InvalidParams",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Heart_OutOfCycle"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Heart_OutOfCycle"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("KernelAdapter_OnlyKernel"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("KernelAdapter_OnlyKernel",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("caller_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Policy_ModuleDoesNotExist"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Policy_ModuleDoesNotExist",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("keycode_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(5usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("Keycode"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Policy_WrongModuleVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Policy_WrongModuleVersion",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("expected_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OLYMPUSHEART_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01`\x02U4\x80\x15b\0\0\x16W`\0\x80\xFD[P`@Qb\0\x18\x138\x03\x80b\0\x18\x13\x839\x81\x01`@\x81\x90Rb\0\09\x91b\0\x01+V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x91\x90\x91\x17\x90\x92U`\x06\x80T\x90\x91\x16\x86\x83\x16\x17\x90U`\x05\x80T`\xFF\x19\x16`\x01\x17\x90U`\x03\x80TBe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\x01``\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x82\x02\x17`\x01`\x01``\x1B\x03\x16l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x93\x87\x16\x93\x84\x02\x17\x90\x91U`\x04\x84\x90U`@\x80Q\x92\x83R` \x83\x01\x85\x90R\x82\x01R\x7Fe\xFCG\xE07\xDF\xFB\xFE\x0F,v\xBF,\xA9l!\x04\xC1\x04s\xC3\x86(x\xB7\x19\x04I\xB6Ao\xD6\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPPPb\0\x01\xA9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01(W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01DW`\0\x80\xFD[\x85Qb\0\x01Q\x81b\0\x01\x12V[` \x87\x01Q\x90\x95Pb\0\x01d\x81b\0\x01\x12V[`@\x87\x01Q\x90\x94Pb\0\x01w\x81b\0\x01\x12V[``\x87\x01Q`\x80\x88\x01Q\x91\x94P\x92Pe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x01\x9BW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[a\x16Z\x80b\0\x01\xB9`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x07W`\x005`\xE0\x1C\x80c\x02\xFB\x0C^\x14a\x01\x0CW\x80c\x07b\x1E\xCA\x14a\x01.W\x80c\x0C\xBFT\xC8\x14a\x01DW\x80c\x0F\x15\xF4\xC0\x14a\x01tW\x80c\"\xF3\xE2\xD4\x14a\x01~W\x80cFW\xB3l\x14a\x01\x86W\x80cQ\xB4+\0\x14a\x01\x99W\x80cW\x0C\xA75\x14a\x01\xA1W\x80cY$\xBEp\x14a\x01\xC1W\x80cf\xA7\x8El\x14a\x01\xD6W\x80ci\x11\xBBX\x14a\x01\xDFW\x80cz%\x9D\xBA\x14a\x01\xF2W\x80c\x92<\xB9R\x14a\x01\xFAW\x80c\x94Y\xB8u\x14a\x02\rW\x80c\x9A\xB7\xD7\xE7\x14a\x02\"W\x80c\xB3\xAB\x15\xFB\x14a\x025W\x80c\xD3\xA7\xB7\xD3\x14a\x02HW\x80c\xD4\xAA\xE0\xC4\x14a\x02ZW\x80c\xEA\xD5\r\xA3\x14a\x02mW\x80c\xEFh\xB8}\x14a\x02uW\x80c\xF7\xC6\x18\xC1\x14a\x02}W[`\0\x80\xFD[`\x05Ta\x01\x19\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x016a\x02\x97V[`@Q\x90\x81R` \x01a\x01%V[`\x03Ta\x01]\x90`\x01`0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01%V[a\x01|a\x04\x15V[\0[a\x01\x19a\x04\xA0V[a\x01|a\x01\x946`\x04a\x12\x1AV[a\x05\x17V[a\x01|a\x05oV[`\x06Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x01%\x91\x90a\x12>V[a\x01\xC9a\x05\xEEV[`@Qa\x01%\x91\x90a\x12RV[a\x016`\x04T\x81V[a\x01|a\x01\xED6`\x04a\x12\xC9V[a\x06\xEBV[a\x01|a\x08iV[`\x01Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x15a\n\xDAV[`@Qa\x01%\x91\x90a\x13\x0BV[a\x01|a\x0206`\x04a\x12\x1AV[a\rPV[a\x01|a\x02C6`\x04a\x12\x1AV[a\x0E\x94V[`\x03Ta\x01]\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01]a\x0F*V[a\x01|a\x0F\xA3V[`\x03Ta\x01\xB4\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80a\x02\xA2a\x0F*V[`\x03T\x90\x91P`\0\x90a\x02\xBE\x90\x83\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[`\x03T\x90\x91PB\x90`\0\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16`\x01`0\x1B\x90\x92\x04\x16\x11a\x02\xF8W`\x03T`\x01`0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xFAV[\x83[\x90P\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x03\x1EW`\0\x94PPPPP\x90V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x033\x85\x85a\x13\x99V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x03xW`\x04Te\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\x03W\x86\x86a\x13\x99V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03i\x91\x90a\x13\xC0V[a\x03s\x91\x90a\x13\xF5V[a\x03|V[`\x04T[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R\x91\x92P`\0\x91`\x01``\x1B\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90a\x03\xB8\x900\x90`\x04\x01a\x12>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF9\x91\x90a\x14\tV[\x90P\x80\x82\x11a\x04\x08W\x81a\x04\nV[\x80[\x96PPPPPPP\x90V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x04U\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x83W=`\0\x80>=`\0\xFD[PP`\x05\x80T`\xFF\x19\x16`\x01\x17\x90UPa\x04\x9D\x90Pa\x10\x19V[PV[`\0\x80T`@Qc\xE5\"#\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE5\"#\xBB\x90a\x04\xD1\x900\x90`\x04\x01a\x12>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x12\x91\x90a\x149V[\x90P\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05MW3`@Qc\x05>\x90\x0F`\xE2\x1B\x81R`\x04\x01a\x05D\x91\x90a\x12>V[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x05\xAF\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xC9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xDDW=`\0\x80>=`\0\xFD[PP`\x05\x80T`\xFF\x19\x16\x90UPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\x05W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\x05`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1A\xE7\xEC.`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB0\x91\x90a\x14[V[`\x01`\x01`\xD8\x1B\x03\x19\x16\x81Rc\x08m\xB7\xDF`\xE4\x1B` \x90\x91\x01R\x81Q\x82\x90`\0\x90a\x06\xDDWa\x06\xDDa\x14\x85V[` \x02` \x01\x01\x81\x90RP\x90V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x07+\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07YW=`\0\x80>=`\0\xFD[PPPPa\x07ea\x0F*V[`\x03Ta\x07z\x91\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16Be\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x07\xA9W`@Qc\x0B\xCD\xBE)`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xB1a\x0F*V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x07\xE1W`@Qc%\xD1\x03'`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T`\x04\x85\x90Ue\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x01``\x1B`\x01`\x01`\xA0\x1B\x03\x88\x16\x90\x81\x02e\xFF\xFF\xFF\xFF\xFF\xFF`0\x1B\x19\x16\x91\x90\x91\x17`\x01`0\x1B\x92\x86\x16\x92\x83\x02\x17\x90\x92U`@\x80Q\x92\x83R` \x83\x01\x86\x90R\x82\x01R\x7Fe\xFCG\xE07\xDF\xFB\xFE\x0F,v\xBF,\xA9l!\x04\xC1\x04s\xC3\x86(x\xB7\x19\x04I\xB6Ao\xD6\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x02T`\x01\x14a\x08\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RiREENTRANCY`\xB0\x1B`D\x82\x01R`d\x01a\x05DV[`\x02\x80U`\x05T`\xFF\x16a\x08\xCFW`@Qc\x017\xEF\xE3`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Ba\x08\xD8a\x0F*V[`\x03Ta\x08\xED\x91\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\t\x1DW`@QcF\x1B\xBB\x83`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x86\xDB}\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tmW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x81W=`\0\x80>=`\0\xFD[PPPP`\x06`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cqY\xA6\x18`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xE9W=`\0\x80>=`\0\xFD[PPPP`\0a\t\xF7a\x02\x97V[\x90Pa\n\x01a\x0F*V[`\x03Ta\n\x16\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84a\x13\x99V[a\n \x91\x90a\x14\x9BV[a\n*\x90\x83a\x13\x99V[`\x03\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ua\ne\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x83a\x10KV[\x7F\xF9\xB6\xECw\x0C\x96\xC3\xA6\xF9\"[\xF3\xA8F:\x8A\xD0>\x88DH\xAF\x07\xB4\xEC\xD4\xD1\x9Db\x89F\xEE3\x82`@Qa\n\x96\x92\x91\x90a\x14\xC0V[`@Q\x80\x91\x03\x90\xA1`@QB\x81R\x7F\xD5!^\x06a\x83\x14!\xA9\xEB\xC7\x9F\x9D(H\xDB\xFCa!<\xF6\xAE<\x92\xB4^\x17\xEAP\xBE\x19\x10\x90` \x01`@Q\x80\x91\x03\x90\xA1PP`\x01`\x02UV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90PdPRICE`\xD8\x1B\x81`\0\x81Q\x81\x10a\x0B\x16Wa\x0B\x16a\x14\x85V[`\x01`\x01`\xD8\x1B\x03\x19\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x0B>dROLES`\xD8\x1B\x90V[\x81`\x01\x81Q\x81\x10a\x0BQWa\x0BQa\x14\x85V[` \x02` \x01\x01\x90`\x01`\x01`\xD8\x1B\x03\x19\x16\x90\x81`\x01`\x01`\xD8\x1B\x03\x19\x16\x81RPPa\x0B\x96\x81`\0\x81Q\x81\x10a\x0B\x89Wa\x0B\x89a\x14\x85V[` \x02` \x01\x01Qa\x11bV[`\x05`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPa\x0B\xD2\x81`\x01\x81Q\x81\x10a\x0B\x89Wa\x0B\x89a\x14\x85V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x05T`@\x80Q`\x01b\x17\x94\xA3`\xE2\x1B\x03\x19\x81R\x81Q`\0\x94a\x01\0\x90\x94\x04\x90\x93\x16\x92c\xFF\xA1\xADt\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0C9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C]\x91\x90a\x14\xEFV[P\x90P`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFF\xA1\xADt`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD8\x91\x90a\x14\xEFV[P`@\x80Q\x80\x82\x01\x82R`\x01\x80\x82R` \x80\x83\x01\x91\x90\x91R\x91Q\x92\x93P`\0\x92a\r\x02\x92\x01a\x15\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82`\xFF\x16`\x01\x14\x15\x80a\r*WP\x81`\xFF\x16`\x01\x14\x15[\x15a\rJW\x80`@Qc\xDB>\xA69`\xE0\x1B\x81R`\x04\x01a\x05D\x91\x90a\x15\x86V[PPP\x90V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\r\x90\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xBEW=`\0\x80>=`\0\xFD[PPPPa\r\xCAa\x0F*V[`\x03Ta\r\xDF\x91\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16Be\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x0E\x0EW`@Qc\x0B\xCD\xBE)`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x903\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E>\x91\x90a\x12>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x7F\x91\x90a\x14\tV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90a\x10KV[PPV[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x0E\xD4\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xEEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x02W=`\0\x80>=`\0\xFD[PP`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x94\x90\x94\x17\x90\x93UPPPV[`\0`\x05`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cs!\xF1\0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x12\x91\x90a\x15\xB9V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x0F\xE3\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x11W=`\0\x80>=`\0\xFD[PPPPa\x04\x9D[a\x10!a\x0F*V[a\x10+\x90Ba\x13\x99V[`\x03\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x85\x85`@Q`$\x01a\x10s\x92\x91\x90a\x14\xC0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x10\xB1\x91\x90a\x15\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x10\xEEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10\xF3V[``\x91P[P\x91P\x91P\x81\x80\x15a\x11\x1DWP\x80Q\x15\x80a\x11\x1DWP\x80\x80` \x01\x90Q\x81\x01\x90a\x11\x1D\x91\x90a\x149V[a\x11[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x05DV[PPPPPV[`\0\x80T`@Qc-7\0-`\xE2\x1B\x81R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB4\xDC\0\xB4\x90a\x11\x93\x90\x86\x90`\x04\x01a\x15\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD4\x91\x90a\x16\x07V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xFFW\x82`@Qc\\?\xA9\xCD`\xE0\x1B\x81R`\x04\x01a\x05D\x91\x90a\x15\xF2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x9DW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12,W`\0\x80\xFD[\x815a\x127\x81a\x12\x05V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x12\xA8W\x81Q\x80Q`\x01`\x01`\xD8\x1B\x03\x19\x16\x85R\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x12oV[P\x91\x97\x96PPPPPPPV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x9DW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xDEW`\0\x80\xFD[\x835a\x12\xE9\x81a\x12\x05V[\x92P` \x84\x015\x91P`@\x84\x015a\x13\0\x81a\x12\xB5V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x13MW\x83Q`\x01`\x01`\xD8\x1B\x03\x19\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x13'V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x13\x90Wa\x13\x90a\x13YV[\x01\x94\x93PPPPV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x13\xB8Wa\x13\xB8a\x13YV[\x03\x93\x92PPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x13\xDAWa\x13\xDAa\x13YV[P\x02\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x14\x04Wa\x14\x04a\x13\xDFV[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x14\x1BW`\0\x80\xFD[PQ\x91\x90PV[\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x14KW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x127W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x14mW`\0\x80\xFD[\x81Q`\x01`\x01`\xD8\x1B\x03\x19\x81\x16\x81\x14a\x127W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x14\xB4Wa\x14\xB4a\x13\xDFV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[\x80Q`\xFF\x81\x16\x81\x14a\x14\xEAW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x02W`\0\x80\xFD[a\x15\x0B\x83a\x14\xD9V[\x91Pa\x15\x19` \x84\x01a\x14\xD9V[\x90P\x92P\x92\x90PV[`@\x81\x01\x81\x83`\0[`\x02\x81\x10\x15a\x15MW\x81Q`\xFF\x16\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x15+V[PPP\x92\x91PPV[`\0[\x83\x81\x10\x15a\x15qW\x81\x81\x01Q\x83\x82\x01R` \x01a\x15YV[\x83\x81\x11\x15a\x15\x80W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x15\xA5\x81`@\x85\x01` \x87\x01a\x15VV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xCBW`\0\x80\xFD[\x81Qa\x127\x81a\x12\xB5V[`\0\x82Qa\x15\xE8\x81\x84` \x87\x01a\x15VV[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xD8\x1B\x03\x19\x91\x90\x91\x16\x81R` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\x16\x19W`\0\x80\xFD[\x81Qa\x127\x81a\x12\x05V\xFE\xA2dipfsX\"\x12 \xE0\x0Et$\x8A\xC9T\xC9zH\xE3\xE4H\xD7\xDF\x81:\x94\x1Ev\xFE\\\xEF\xD4\xEC\x91\xA5\xB4|\xFB\x0FydsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static OLYMPUSHEART_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x07W`\x005`\xE0\x1C\x80c\x02\xFB\x0C^\x14a\x01\x0CW\x80c\x07b\x1E\xCA\x14a\x01.W\x80c\x0C\xBFT\xC8\x14a\x01DW\x80c\x0F\x15\xF4\xC0\x14a\x01tW\x80c\"\xF3\xE2\xD4\x14a\x01~W\x80cFW\xB3l\x14a\x01\x86W\x80cQ\xB4+\0\x14a\x01\x99W\x80cW\x0C\xA75\x14a\x01\xA1W\x80cY$\xBEp\x14a\x01\xC1W\x80cf\xA7\x8El\x14a\x01\xD6W\x80ci\x11\xBBX\x14a\x01\xDFW\x80cz%\x9D\xBA\x14a\x01\xF2W\x80c\x92<\xB9R\x14a\x01\xFAW\x80c\x94Y\xB8u\x14a\x02\rW\x80c\x9A\xB7\xD7\xE7\x14a\x02\"W\x80c\xB3\xAB\x15\xFB\x14a\x025W\x80c\xD3\xA7\xB7\xD3\x14a\x02HW\x80c\xD4\xAA\xE0\xC4\x14a\x02ZW\x80c\xEA\xD5\r\xA3\x14a\x02mW\x80c\xEFh\xB8}\x14a\x02uW\x80c\xF7\xC6\x18\xC1\x14a\x02}W[`\0\x80\xFD[`\x05Ta\x01\x19\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x016a\x02\x97V[`@Q\x90\x81R` \x01a\x01%V[`\x03Ta\x01]\x90`\x01`0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01%V[a\x01|a\x04\x15V[\0[a\x01\x19a\x04\xA0V[a\x01|a\x01\x946`\x04a\x12\x1AV[a\x05\x17V[a\x01|a\x05oV[`\x06Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x01%\x91\x90a\x12>V[a\x01\xC9a\x05\xEEV[`@Qa\x01%\x91\x90a\x12RV[a\x016`\x04T\x81V[a\x01|a\x01\xED6`\x04a\x12\xC9V[a\x06\xEBV[a\x01|a\x08iV[`\x01Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x15a\n\xDAV[`@Qa\x01%\x91\x90a\x13\x0BV[a\x01|a\x0206`\x04a\x12\x1AV[a\rPV[a\x01|a\x02C6`\x04a\x12\x1AV[a\x0E\x94V[`\x03Ta\x01]\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01]a\x0F*V[a\x01|a\x0F\xA3V[`\x03Ta\x01\xB4\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80a\x02\xA2a\x0F*V[`\x03T\x90\x91P`\0\x90a\x02\xBE\x90\x83\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[`\x03T\x90\x91PB\x90`\0\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16`\x01`0\x1B\x90\x92\x04\x16\x11a\x02\xF8W`\x03T`\x01`0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xFAV[\x83[\x90P\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x03\x1EW`\0\x94PPPPP\x90V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x033\x85\x85a\x13\x99V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x03xW`\x04Te\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\x03W\x86\x86a\x13\x99V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03i\x91\x90a\x13\xC0V[a\x03s\x91\x90a\x13\xF5V[a\x03|V[`\x04T[`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R\x91\x92P`\0\x91`\x01``\x1B\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90a\x03\xB8\x900\x90`\x04\x01a\x12>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF9\x91\x90a\x14\tV[\x90P\x80\x82\x11a\x04\x08W\x81a\x04\nV[\x80[\x96PPPPPPP\x90V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x04U\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x83W=`\0\x80>=`\0\xFD[PP`\x05\x80T`\xFF\x19\x16`\x01\x17\x90UPa\x04\x9D\x90Pa\x10\x19V[PV[`\0\x80T`@Qc\xE5\"#\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE5\"#\xBB\x90a\x04\xD1\x900\x90`\x04\x01a\x12>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x12\x91\x90a\x149V[\x90P\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05MW3`@Qc\x05>\x90\x0F`\xE2\x1B\x81R`\x04\x01a\x05D\x91\x90a\x12>V[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x05\xAF\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xC9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xDDW=`\0\x80>=`\0\xFD[PP`\x05\x80T`\xFF\x19\x16\x90UPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\x05W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\x05`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1A\xE7\xEC.`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB0\x91\x90a\x14[V[`\x01`\x01`\xD8\x1B\x03\x19\x16\x81Rc\x08m\xB7\xDF`\xE4\x1B` \x90\x91\x01R\x81Q\x82\x90`\0\x90a\x06\xDDWa\x06\xDDa\x14\x85V[` \x02` \x01\x01\x81\x90RP\x90V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x07+\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07YW=`\0\x80>=`\0\xFD[PPPPa\x07ea\x0F*V[`\x03Ta\x07z\x91\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16Be\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x07\xA9W`@Qc\x0B\xCD\xBE)`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xB1a\x0F*V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x07\xE1W`@Qc%\xD1\x03'`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T`\x04\x85\x90Ue\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x01``\x1B`\x01`\x01`\xA0\x1B\x03\x88\x16\x90\x81\x02e\xFF\xFF\xFF\xFF\xFF\xFF`0\x1B\x19\x16\x91\x90\x91\x17`\x01`0\x1B\x92\x86\x16\x92\x83\x02\x17\x90\x92U`@\x80Q\x92\x83R` \x83\x01\x86\x90R\x82\x01R\x7Fe\xFCG\xE07\xDF\xFB\xFE\x0F,v\xBF,\xA9l!\x04\xC1\x04s\xC3\x86(x\xB7\x19\x04I\xB6Ao\xD6\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x02T`\x01\x14a\x08\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01RiREENTRANCY`\xB0\x1B`D\x82\x01R`d\x01a\x05DV[`\x02\x80U`\x05T`\xFF\x16a\x08\xCFW`@Qc\x017\xEF\xE3`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Ba\x08\xD8a\x0F*V[`\x03Ta\x08\xED\x91\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\t\x1DW`@QcF\x1B\xBB\x83`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x86\xDB}\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tmW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x81W=`\0\x80>=`\0\xFD[PPPP`\x06`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cqY\xA6\x18`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xE9W=`\0\x80>=`\0\xFD[PPPP`\0a\t\xF7a\x02\x97V[\x90Pa\n\x01a\x0F*V[`\x03Ta\n\x16\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84a\x13\x99V[a\n \x91\x90a\x14\x9BV[a\n*\x90\x83a\x13\x99V[`\x03\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ua\ne\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x83a\x10KV[\x7F\xF9\xB6\xECw\x0C\x96\xC3\xA6\xF9\"[\xF3\xA8F:\x8A\xD0>\x88DH\xAF\x07\xB4\xEC\xD4\xD1\x9Db\x89F\xEE3\x82`@Qa\n\x96\x92\x91\x90a\x14\xC0V[`@Q\x80\x91\x03\x90\xA1`@QB\x81R\x7F\xD5!^\x06a\x83\x14!\xA9\xEB\xC7\x9F\x9D(H\xDB\xFCa!<\xF6\xAE<\x92\xB4^\x17\xEAP\xBE\x19\x10\x90` \x01`@Q\x80\x91\x03\x90\xA1PP`\x01`\x02UV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90PdPRICE`\xD8\x1B\x81`\0\x81Q\x81\x10a\x0B\x16Wa\x0B\x16a\x14\x85V[`\x01`\x01`\xD8\x1B\x03\x19\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x0B>dROLES`\xD8\x1B\x90V[\x81`\x01\x81Q\x81\x10a\x0BQWa\x0BQa\x14\x85V[` \x02` \x01\x01\x90`\x01`\x01`\xD8\x1B\x03\x19\x16\x90\x81`\x01`\x01`\xD8\x1B\x03\x19\x16\x81RPPa\x0B\x96\x81`\0\x81Q\x81\x10a\x0B\x89Wa\x0B\x89a\x14\x85V[` \x02` \x01\x01Qa\x11bV[`\x05`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPa\x0B\xD2\x81`\x01\x81Q\x81\x10a\x0B\x89Wa\x0B\x89a\x14\x85V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x05T`@\x80Q`\x01b\x17\x94\xA3`\xE2\x1B\x03\x19\x81R\x81Q`\0\x94a\x01\0\x90\x94\x04\x90\x93\x16\x92c\xFF\xA1\xADt\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0C9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C]\x91\x90a\x14\xEFV[P\x90P`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFF\xA1\xADt`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD8\x91\x90a\x14\xEFV[P`@\x80Q\x80\x82\x01\x82R`\x01\x80\x82R` \x80\x83\x01\x91\x90\x91R\x91Q\x92\x93P`\0\x92a\r\x02\x92\x01a\x15\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82`\xFF\x16`\x01\x14\x15\x80a\r*WP\x81`\xFF\x16`\x01\x14\x15[\x15a\rJW\x80`@Qc\xDB>\xA69`\xE0\x1B\x81R`\x04\x01a\x05D\x91\x90a\x15\x86V[PPP\x90V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\r\x90\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xBEW=`\0\x80>=`\0\xFD[PPPPa\r\xCAa\x0F*V[`\x03Ta\r\xDF\x91\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13oV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16Be\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x0E\x0EW`@Qc\x0B\xCD\xBE)`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x903\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E>\x91\x90a\x12>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x7F\x91\x90a\x14\tV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90a\x10KV[PPV[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x0E\xD4\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xEEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x02W=`\0\x80>=`\0\xFD[PP`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x94\x90\x94\x17\x90\x93UPPPV[`\0`\x05`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cs!\xF1\0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x12\x91\x90a\x15\xB9V[`\x01T`@Qc\xD0\x9A \xC5`\xE0\x1B\x81Rj42\xB0\xB9:/\xB0\xB26\xB4\xB7`\xA9\x1B\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\x9A \xC5\x90a\x0F\xE3\x90\x84\x903\x90`\x04\x01a\x14\"V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x11W=`\0\x80>=`\0\xFD[PPPPa\x04\x9D[a\x10!a\x0F*V[a\x10+\x90Ba\x13\x99V[`\x03\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x85\x85`@Q`$\x01a\x10s\x92\x91\x90a\x14\xC0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x10\xB1\x91\x90a\x15\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x10\xEEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10\xF3V[``\x91P[P\x91P\x91P\x81\x80\x15a\x11\x1DWP\x80Q\x15\x80a\x11\x1DWP\x80\x80` \x01\x90Q\x81\x01\x90a\x11\x1D\x91\x90a\x149V[a\x11[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x05DV[PPPPPV[`\0\x80T`@Qc-7\0-`\xE2\x1B\x81R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB4\xDC\0\xB4\x90a\x11\x93\x90\x86\x90`\x04\x01a\x15\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD4\x91\x90a\x16\x07V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xFFW\x82`@Qc\\?\xA9\xCD`\xE0\x1B\x81R`\x04\x01a\x05D\x91\x90a\x15\xF2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x9DW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12,W`\0\x80\xFD[\x815a\x127\x81a\x12\x05V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x12\xA8W\x81Q\x80Q`\x01`\x01`\xD8\x1B\x03\x19\x16\x85R\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x12oV[P\x91\x97\x96PPPPPPPV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x9DW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xDEW`\0\x80\xFD[\x835a\x12\xE9\x81a\x12\x05V[\x92P` \x84\x015\x91P`@\x84\x015a\x13\0\x81a\x12\xB5V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x13MW\x83Q`\x01`\x01`\xD8\x1B\x03\x19\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x13'V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x13\x90Wa\x13\x90a\x13YV[\x01\x94\x93PPPPV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x13\xB8Wa\x13\xB8a\x13YV[\x03\x93\x92PPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x13\xDAWa\x13\xDAa\x13YV[P\x02\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x14\x04Wa\x14\x04a\x13\xDFV[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x14\x1BW`\0\x80\xFD[PQ\x91\x90PV[\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x14KW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x127W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x14mW`\0\x80\xFD[\x81Q`\x01`\x01`\xD8\x1B\x03\x19\x81\x16\x81\x14a\x127W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x14\xB4Wa\x14\xB4a\x13\xDFV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[\x80Q`\xFF\x81\x16\x81\x14a\x14\xEAW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x02W`\0\x80\xFD[a\x15\x0B\x83a\x14\xD9V[\x91Pa\x15\x19` \x84\x01a\x14\xD9V[\x90P\x92P\x92\x90PV[`@\x81\x01\x81\x83`\0[`\x02\x81\x10\x15a\x15MW\x81Q`\xFF\x16\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x15+V[PPP\x92\x91PPV[`\0[\x83\x81\x10\x15a\x15qW\x81\x81\x01Q\x83\x82\x01R` \x01a\x15YV[\x83\x81\x11\x15a\x15\x80W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x15\xA5\x81`@\x85\x01` \x87\x01a\x15VV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xCBW`\0\x80\xFD[\x81Qa\x127\x81a\x12\xB5V[`\0\x82Qa\x15\xE8\x81\x84` \x87\x01a\x15VV[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xD8\x1B\x03\x19\x91\x90\x91\x16\x81R` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\x16\x19W`\0\x80\xFD[\x81Qa\x127\x81a\x12\x05V\xFE\xA2dipfsX\"\x12 \xE0\x0Et$\x8A\xC9T\xC9zH\xE3\xE4H\xD7\xDF\x81:\x94\x1Ev\xFE\\\xEF\xD4\xEC\x91\xA5\xB4|\xFB\x0FydsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static OLYMPUSHEART_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct OlympusHeart<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OlympusHeart<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OlympusHeart<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OlympusHeart<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OlympusHeart<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OlympusHeart))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OlympusHeart<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                OLYMPUSHEART_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                OLYMPUSHEART_ABI.clone(),
                OLYMPUSHEART_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ROLES` (0x923cb952) function
        pub fn roles(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([146, 60, 185, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `activate` (0x0f15f4c0) function
        pub fn activate(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 21, 244, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `active` (0x02fb0c5e) function
        pub fn active(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([2, 251, 12, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `auctionDuration` (0x0cbf54c8) function
        pub fn auction_duration(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([12, 191, 84, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beat` (0x7a259dba) function
        pub fn beat(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 37, 157, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeKernel` (0x4657b36c) function
        pub fn change_kernel(
            &self,
            new_kernel: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 87, 179, 108], new_kernel)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configureDependencies` (0x9459b875) function
        pub fn configure_dependencies(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 5]>> {
            self.0
                .method_hash([148, 89, 184, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentReward` (0x07621eca) function
        pub fn current_reward(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 98, 30, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deactivate` (0x51b42b00) function
        pub fn deactivate(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 180, 43, 0], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `frequency` (0xead50da3) function
        pub fn frequency(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([234, 213, 13, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isActive` (0x22f3e2d4) function
        pub fn is_active(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([34, 243, 226, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kernel` (0xd4aae0c4) function
        pub fn kernel(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([212, 170, 224, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastBeat` (0xd3a7b7d3) function
        pub fn last_beat(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([211, 167, 183, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxReward` (0x66a78e6c) function
        pub fn max_reward(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 167, 142, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operator` (0x570ca735) function
        pub fn operator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([87, 12, 167, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestPermissions` (0x5924be70) function
        pub fn request_permissions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Permissions>> {
            self.0
                .method_hash([89, 36, 190, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetBeat` (0xef68b87d) function
        pub fn reset_beat(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 104, 184, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardToken` (0xf7c618c1) function
        pub fn reward_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([247, 198, 24, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperator` (0xb3ab15fb) function
        pub fn set_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 171, 21, 251], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRewardAuctionParams` (0x6911bb58) function
        pub fn set_reward_auction_params(
            &self,
            token: ::ethers::core::types::Address,
            max_reward: ::ethers::core::types::U256,
            auction_duration: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 17, 187, 88], (token, max_reward, auction_duration))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawUnspentRewards` (0x9ab7d7e7) function
        pub fn withdraw_unspent_rewards(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 183, 215, 231], token)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Beat` event
        pub fn beat_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BeatFilter> {
            self.0.event()
        }
        ///Gets the contract's `RewardIssued` event
        pub fn reward_issued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardIssuedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RewardUpdated` event
        pub fn reward_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardUpdatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OlympusHeartEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for OlympusHeart<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Heart_BeatAvailable` with signature `Heart_BeatAvailable()` and selector `0x2f36f8a4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Heart_BeatAvailable", abi = "Heart_BeatAvailable()")]
    pub struct Heart_BeatAvailable;
    ///Custom Error type `Heart_BeatStopped` with signature `Heart_BeatStopped()` and selector `0x9bf7f180`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Heart_BeatStopped", abi = "Heart_BeatStopped()")]
    pub struct Heart_BeatStopped;
    ///Custom Error type `Heart_InvalidParams` with signature `Heart_InvalidParams()` and selector `0x97440c9c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Heart_InvalidParams", abi = "Heart_InvalidParams()")]
    pub struct Heart_InvalidParams;
    ///Custom Error type `Heart_OutOfCycle` with signature `Heart_OutOfCycle()` and selector `0x8c377706`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Heart_OutOfCycle", abi = "Heart_OutOfCycle()")]
    pub struct Heart_OutOfCycle;
    ///Custom Error type `KernelAdapter_OnlyKernel` with signature `KernelAdapter_OnlyKernel(address)` and selector `0x14fa403c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "KernelAdapter_OnlyKernel",
        abi = "KernelAdapter_OnlyKernel(address)"
    )]
    pub struct KernelAdapter_OnlyKernel {
        pub caller: ::ethers::core::types::Address,
    }
    ///Custom Error type `Policy_ModuleDoesNotExist` with signature `Policy_ModuleDoesNotExist(bytes5)` and selector `0x5c3fa9cd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Policy_ModuleDoesNotExist",
        abi = "Policy_ModuleDoesNotExist(bytes5)"
    )]
    pub struct Policy_ModuleDoesNotExist {
        pub keycode: [u8; 5],
    }
    ///Custom Error type `Policy_WrongModuleVersion` with signature `Policy_WrongModuleVersion(bytes)` and selector `0xdb3ea639`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Policy_WrongModuleVersion",
        abi = "Policy_WrongModuleVersion(bytes)"
    )]
    pub struct Policy_WrongModuleVersion {
        pub expected: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum OlympusHeartErrors {
        Heart_BeatAvailable(Heart_BeatAvailable),
        Heart_BeatStopped(Heart_BeatStopped),
        Heart_InvalidParams(Heart_InvalidParams),
        Heart_OutOfCycle(Heart_OutOfCycle),
        KernelAdapter_OnlyKernel(KernelAdapter_OnlyKernel),
        Policy_ModuleDoesNotExist(Policy_ModuleDoesNotExist),
        Policy_WrongModuleVersion(Policy_WrongModuleVersion),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for OlympusHeartErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <Heart_BeatAvailable as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Heart_BeatAvailable(decoded));
            }
            if let Ok(decoded) = <Heart_BeatStopped as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Heart_BeatStopped(decoded));
            }
            if let Ok(decoded) =
                <Heart_InvalidParams as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Heart_InvalidParams(decoded));
            }
            if let Ok(decoded) = <Heart_OutOfCycle as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Heart_OutOfCycle(decoded));
            }
            if let Ok(decoded) =
                <KernelAdapter_OnlyKernel as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::KernelAdapter_OnlyKernel(decoded));
            }
            if let Ok(decoded) =
                <Policy_ModuleDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Policy_ModuleDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <Policy_WrongModuleVersion as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Policy_WrongModuleVersion(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OlympusHeartErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Heart_BeatAvailable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Heart_BeatStopped(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Heart_InvalidParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Heart_OutOfCycle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::KernelAdapter_OnlyKernel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Policy_ModuleDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Policy_WrongModuleVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for OlympusHeartErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Heart_BeatAvailable as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Heart_BeatStopped as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Heart_InvalidParams as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Heart_OutOfCycle as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <KernelAdapter_OnlyKernel as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Policy_ModuleDoesNotExist as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Policy_WrongModuleVersion as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for OlympusHeartErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Heart_BeatAvailable(element) => ::core::fmt::Display::fmt(element, f),
                Self::Heart_BeatStopped(element) => ::core::fmt::Display::fmt(element, f),
                Self::Heart_InvalidParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Heart_OutOfCycle(element) => ::core::fmt::Display::fmt(element, f),
                Self::KernelAdapter_OnlyKernel(element) => ::core::fmt::Display::fmt(element, f),
                Self::Policy_ModuleDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::Policy_WrongModuleVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for OlympusHeartErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Heart_BeatAvailable> for OlympusHeartErrors {
        fn from(value: Heart_BeatAvailable) -> Self {
            Self::Heart_BeatAvailable(value)
        }
    }
    impl ::core::convert::From<Heart_BeatStopped> for OlympusHeartErrors {
        fn from(value: Heart_BeatStopped) -> Self {
            Self::Heart_BeatStopped(value)
        }
    }
    impl ::core::convert::From<Heart_InvalidParams> for OlympusHeartErrors {
        fn from(value: Heart_InvalidParams) -> Self {
            Self::Heart_InvalidParams(value)
        }
    }
    impl ::core::convert::From<Heart_OutOfCycle> for OlympusHeartErrors {
        fn from(value: Heart_OutOfCycle) -> Self {
            Self::Heart_OutOfCycle(value)
        }
    }
    impl ::core::convert::From<KernelAdapter_OnlyKernel> for OlympusHeartErrors {
        fn from(value: KernelAdapter_OnlyKernel) -> Self {
            Self::KernelAdapter_OnlyKernel(value)
        }
    }
    impl ::core::convert::From<Policy_ModuleDoesNotExist> for OlympusHeartErrors {
        fn from(value: Policy_ModuleDoesNotExist) -> Self {
            Self::Policy_ModuleDoesNotExist(value)
        }
    }
    impl ::core::convert::From<Policy_WrongModuleVersion> for OlympusHeartErrors {
        fn from(value: Policy_WrongModuleVersion) -> Self {
            Self::Policy_WrongModuleVersion(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Beat", abi = "Beat(uint256)")]
    pub struct BeatFilter {
        pub timestamp: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "RewardIssued", abi = "RewardIssued(address,uint256)")]
    pub struct RewardIssuedFilter {
        pub to: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "RewardUpdated", abi = "RewardUpdated(address,uint256,uint48)")]
    pub struct RewardUpdatedFilter {
        pub token: ::ethers::core::types::Address,
        pub max_reward_amount: ::ethers::core::types::U256,
        pub auction_duration: u64,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum OlympusHeartEvents {
        BeatFilter(BeatFilter),
        RewardIssuedFilter(RewardIssuedFilter),
        RewardUpdatedFilter(RewardUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for OlympusHeartEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BeatFilter::decode_log(log) {
                return Ok(OlympusHeartEvents::BeatFilter(decoded));
            }
            if let Ok(decoded) = RewardIssuedFilter::decode_log(log) {
                return Ok(OlympusHeartEvents::RewardIssuedFilter(decoded));
            }
            if let Ok(decoded) = RewardUpdatedFilter::decode_log(log) {
                return Ok(OlympusHeartEvents::RewardUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OlympusHeartEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BeatFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardIssuedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BeatFilter> for OlympusHeartEvents {
        fn from(value: BeatFilter) -> Self {
            Self::BeatFilter(value)
        }
    }
    impl ::core::convert::From<RewardIssuedFilter> for OlympusHeartEvents {
        fn from(value: RewardIssuedFilter) -> Self {
            Self::RewardIssuedFilter(value)
        }
    }
    impl ::core::convert::From<RewardUpdatedFilter> for OlympusHeartEvents {
        fn from(value: RewardUpdatedFilter) -> Self {
            Self::RewardUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ROLES` function with signature `ROLES()` and selector `0x923cb952`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "ROLES", abi = "ROLES()")]
    pub struct RolesCall;
    ///Container type for all input parameters for the `activate` function with signature `activate()` and selector `0x0f15f4c0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "activate", abi = "activate()")]
    pub struct ActivateCall;
    ///Container type for all input parameters for the `active` function with signature `active()` and selector `0x02fb0c5e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "active", abi = "active()")]
    pub struct ActiveCall;
    ///Container type for all input parameters for the `auctionDuration` function with signature `auctionDuration()` and selector `0x0cbf54c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "auctionDuration", abi = "auctionDuration()")]
    pub struct AuctionDurationCall;
    ///Container type for all input parameters for the `beat` function with signature `beat()` and selector `0x7a259dba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "beat", abi = "beat()")]
    pub struct BeatCall;
    ///Container type for all input parameters for the `changeKernel` function with signature `changeKernel(address)` and selector `0x4657b36c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "changeKernel", abi = "changeKernel(address)")]
    pub struct ChangeKernelCall {
        pub new_kernel: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `configureDependencies` function with signature `configureDependencies()` and selector `0x9459b875`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "configureDependencies", abi = "configureDependencies()")]
    pub struct ConfigureDependenciesCall;
    ///Container type for all input parameters for the `currentReward` function with signature `currentReward()` and selector `0x07621eca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "currentReward", abi = "currentReward()")]
    pub struct CurrentRewardCall;
    ///Container type for all input parameters for the `deactivate` function with signature `deactivate()` and selector `0x51b42b00`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deactivate", abi = "deactivate()")]
    pub struct DeactivateCall;
    ///Container type for all input parameters for the `frequency` function with signature `frequency()` and selector `0xead50da3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "frequency", abi = "frequency()")]
    pub struct FrequencyCall;
    ///Container type for all input parameters for the `isActive` function with signature `isActive()` and selector `0x22f3e2d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isActive", abi = "isActive()")]
    pub struct IsActiveCall;
    ///Container type for all input parameters for the `kernel` function with signature `kernel()` and selector `0xd4aae0c4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "kernel", abi = "kernel()")]
    pub struct KernelCall;
    ///Container type for all input parameters for the `lastBeat` function with signature `lastBeat()` and selector `0xd3a7b7d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lastBeat", abi = "lastBeat()")]
    pub struct LastBeatCall;
    ///Container type for all input parameters for the `maxReward` function with signature `maxReward()` and selector `0x66a78e6c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "maxReward", abi = "maxReward()")]
    pub struct MaxRewardCall;
    ///Container type for all input parameters for the `operator` function with signature `operator()` and selector `0x570ca735`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "operator", abi = "operator()")]
    pub struct OperatorCall;
    ///Container type for all input parameters for the `requestPermissions` function with signature `requestPermissions()` and selector `0x5924be70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "requestPermissions", abi = "requestPermissions()")]
    pub struct RequestPermissionsCall;
    ///Container type for all input parameters for the `resetBeat` function with signature `resetBeat()` and selector `0xef68b87d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "resetBeat", abi = "resetBeat()")]
    pub struct ResetBeatCall;
    ///Container type for all input parameters for the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "rewardToken", abi = "rewardToken()")]
    pub struct RewardTokenCall;
    ///Container type for all input parameters for the `setOperator` function with signature `setOperator(address)` and selector `0xb3ab15fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setOperator", abi = "setOperator(address)")]
    pub struct SetOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRewardAuctionParams` function with signature `setRewardAuctionParams(address,uint256,uint48)` and selector `0x6911bb58`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setRewardAuctionParams",
        abi = "setRewardAuctionParams(address,uint256,uint48)"
    )]
    pub struct SetRewardAuctionParamsCall {
        pub token: ::ethers::core::types::Address,
        pub max_reward: ::ethers::core::types::U256,
        pub auction_duration: u64,
    }
    ///Container type for all input parameters for the `withdrawUnspentRewards` function with signature `withdrawUnspentRewards(address)` and selector `0x9ab7d7e7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "withdrawUnspentRewards",
        abi = "withdrawUnspentRewards(address)"
    )]
    pub struct WithdrawUnspentRewardsCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum OlympusHeartCalls {
        Roles(RolesCall),
        Activate(ActivateCall),
        Active(ActiveCall),
        AuctionDuration(AuctionDurationCall),
        Beat(BeatCall),
        ChangeKernel(ChangeKernelCall),
        ConfigureDependencies(ConfigureDependenciesCall),
        CurrentReward(CurrentRewardCall),
        Deactivate(DeactivateCall),
        Frequency(FrequencyCall),
        IsActive(IsActiveCall),
        Kernel(KernelCall),
        LastBeat(LastBeatCall),
        MaxReward(MaxRewardCall),
        Operator(OperatorCall),
        RequestPermissions(RequestPermissionsCall),
        ResetBeat(ResetBeatCall),
        RewardToken(RewardTokenCall),
        SetOperator(SetOperatorCall),
        SetRewardAuctionParams(SetRewardAuctionParamsCall),
        WithdrawUnspentRewards(WithdrawUnspentRewardsCall),
    }
    impl ::ethers::core::abi::AbiDecode for OlympusHeartCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RolesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Roles(decoded));
            }
            if let Ok(decoded) = <ActivateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Activate(decoded));
            }
            if let Ok(decoded) = <ActiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Active(decoded));
            }
            if let Ok(decoded) =
                <AuctionDurationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AuctionDuration(decoded));
            }
            if let Ok(decoded) = <BeatCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Beat(decoded));
            }
            if let Ok(decoded) = <ChangeKernelCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChangeKernel(decoded));
            }
            if let Ok(decoded) =
                <ConfigureDependenciesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConfigureDependencies(decoded));
            }
            if let Ok(decoded) = <CurrentRewardCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CurrentReward(decoded));
            }
            if let Ok(decoded) = <DeactivateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deactivate(decoded));
            }
            if let Ok(decoded) = <FrequencyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Frequency(decoded));
            }
            if let Ok(decoded) = <IsActiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsActive(decoded));
            }
            if let Ok(decoded) = <KernelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Kernel(decoded));
            }
            if let Ok(decoded) = <LastBeatCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastBeat(decoded));
            }
            if let Ok(decoded) = <MaxRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxReward(decoded));
            }
            if let Ok(decoded) = <OperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Operator(decoded));
            }
            if let Ok(decoded) =
                <RequestPermissionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestPermissions(decoded));
            }
            if let Ok(decoded) = <ResetBeatCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ResetBeat(decoded));
            }
            if let Ok(decoded) = <RewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardToken(decoded));
            }
            if let Ok(decoded) = <SetOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOperator(decoded));
            }
            if let Ok(decoded) =
                <SetRewardAuctionParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetRewardAuctionParams(decoded));
            }
            if let Ok(decoded) =
                <WithdrawUnspentRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawUnspentRewards(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OlympusHeartCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Roles(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Activate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Active(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AuctionDuration(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Beat(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChangeKernel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ConfigureDependencies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentReward(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deactivate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Frequency(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsActive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Kernel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastBeat(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxReward(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Operator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestPermissions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResetBeat(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRewardAuctionParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawUnspentRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OlympusHeartCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Roles(element) => ::core::fmt::Display::fmt(element, f),
                Self::Activate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Active(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuctionDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::Beat(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeKernel(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfigureDependencies(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deactivate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Frequency(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kernel(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastBeat(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestPermissions(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetBeat(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRewardAuctionParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawUnspentRewards(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RolesCall> for OlympusHeartCalls {
        fn from(value: RolesCall) -> Self {
            Self::Roles(value)
        }
    }
    impl ::core::convert::From<ActivateCall> for OlympusHeartCalls {
        fn from(value: ActivateCall) -> Self {
            Self::Activate(value)
        }
    }
    impl ::core::convert::From<ActiveCall> for OlympusHeartCalls {
        fn from(value: ActiveCall) -> Self {
            Self::Active(value)
        }
    }
    impl ::core::convert::From<AuctionDurationCall> for OlympusHeartCalls {
        fn from(value: AuctionDurationCall) -> Self {
            Self::AuctionDuration(value)
        }
    }
    impl ::core::convert::From<BeatCall> for OlympusHeartCalls {
        fn from(value: BeatCall) -> Self {
            Self::Beat(value)
        }
    }
    impl ::core::convert::From<ChangeKernelCall> for OlympusHeartCalls {
        fn from(value: ChangeKernelCall) -> Self {
            Self::ChangeKernel(value)
        }
    }
    impl ::core::convert::From<ConfigureDependenciesCall> for OlympusHeartCalls {
        fn from(value: ConfigureDependenciesCall) -> Self {
            Self::ConfigureDependencies(value)
        }
    }
    impl ::core::convert::From<CurrentRewardCall> for OlympusHeartCalls {
        fn from(value: CurrentRewardCall) -> Self {
            Self::CurrentReward(value)
        }
    }
    impl ::core::convert::From<DeactivateCall> for OlympusHeartCalls {
        fn from(value: DeactivateCall) -> Self {
            Self::Deactivate(value)
        }
    }
    impl ::core::convert::From<FrequencyCall> for OlympusHeartCalls {
        fn from(value: FrequencyCall) -> Self {
            Self::Frequency(value)
        }
    }
    impl ::core::convert::From<IsActiveCall> for OlympusHeartCalls {
        fn from(value: IsActiveCall) -> Self {
            Self::IsActive(value)
        }
    }
    impl ::core::convert::From<KernelCall> for OlympusHeartCalls {
        fn from(value: KernelCall) -> Self {
            Self::Kernel(value)
        }
    }
    impl ::core::convert::From<LastBeatCall> for OlympusHeartCalls {
        fn from(value: LastBeatCall) -> Self {
            Self::LastBeat(value)
        }
    }
    impl ::core::convert::From<MaxRewardCall> for OlympusHeartCalls {
        fn from(value: MaxRewardCall) -> Self {
            Self::MaxReward(value)
        }
    }
    impl ::core::convert::From<OperatorCall> for OlympusHeartCalls {
        fn from(value: OperatorCall) -> Self {
            Self::Operator(value)
        }
    }
    impl ::core::convert::From<RequestPermissionsCall> for OlympusHeartCalls {
        fn from(value: RequestPermissionsCall) -> Self {
            Self::RequestPermissions(value)
        }
    }
    impl ::core::convert::From<ResetBeatCall> for OlympusHeartCalls {
        fn from(value: ResetBeatCall) -> Self {
            Self::ResetBeat(value)
        }
    }
    impl ::core::convert::From<RewardTokenCall> for OlympusHeartCalls {
        fn from(value: RewardTokenCall) -> Self {
            Self::RewardToken(value)
        }
    }
    impl ::core::convert::From<SetOperatorCall> for OlympusHeartCalls {
        fn from(value: SetOperatorCall) -> Self {
            Self::SetOperator(value)
        }
    }
    impl ::core::convert::From<SetRewardAuctionParamsCall> for OlympusHeartCalls {
        fn from(value: SetRewardAuctionParamsCall) -> Self {
            Self::SetRewardAuctionParams(value)
        }
    }
    impl ::core::convert::From<WithdrawUnspentRewardsCall> for OlympusHeartCalls {
        fn from(value: WithdrawUnspentRewardsCall) -> Self {
            Self::WithdrawUnspentRewards(value)
        }
    }
    ///Container type for all return fields from the `ROLES` function with signature `ROLES()` and selector `0x923cb952`
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
    pub struct RolesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `active` function with signature `active()` and selector `0x02fb0c5e`
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
    pub struct ActiveReturn(pub bool);
    ///Container type for all return fields from the `auctionDuration` function with signature `auctionDuration()` and selector `0x0cbf54c8`
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
    pub struct AuctionDurationReturn(pub u64);
    ///Container type for all return fields from the `configureDependencies` function with signature `configureDependencies()` and selector `0x9459b875`
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
    pub struct ConfigureDependenciesReturn {
        pub dependencies: ::std::vec::Vec<[u8; 5]>,
    }
    ///Container type for all return fields from the `currentReward` function with signature `currentReward()` and selector `0x07621eca`
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
    pub struct CurrentRewardReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `frequency` function with signature `frequency()` and selector `0xead50da3`
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
    pub struct FrequencyReturn(pub u64);
    ///Container type for all return fields from the `isActive` function with signature `isActive()` and selector `0x22f3e2d4`
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
    pub struct IsActiveReturn(pub bool);
    ///Container type for all return fields from the `kernel` function with signature `kernel()` and selector `0xd4aae0c4`
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
    pub struct KernelReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `lastBeat` function with signature `lastBeat()` and selector `0xd3a7b7d3`
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
    pub struct LastBeatReturn(pub u64);
    ///Container type for all return fields from the `maxReward` function with signature `maxReward()` and selector `0x66a78e6c`
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
    pub struct MaxRewardReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operator` function with signature `operator()` and selector `0x570ca735`
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
    pub struct OperatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `requestPermissions` function with signature `requestPermissions()` and selector `0x5924be70`
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
    pub struct RequestPermissionsReturn {
        pub permissions: ::std::vec::Vec<Permissions>,
    }
    ///Container type for all return fields from the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
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
    pub struct RewardTokenReturn(pub ::ethers::core::types::Address);
}
