pub use the_rewarder_pool::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod the_rewarder_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("REWARDS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REWARDS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("accountingToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accountingToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract AccountingToken"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("distributeRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distributeRewards"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isNewRewardsRound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isNewRewardsRound"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastRecordedSnapshotTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastRecordedSnapshotTimestamp",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastRewardTimestamps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastRewardTimestamps",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastSnapshotIdForRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastSnapshotIdForRewards",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidityToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidityToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rewardToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract RewardToken"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("roundNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("roundNumber"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDepositAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidDepositAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static THEREWARDERPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa6\x0B8\x03\x80a6\x0B\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x8DV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R`@Qa\0H\x90a\x01sV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\0dW=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\xA0R`@Qa\0}\x90a\x01\x80V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\0\x99W=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\xC0Ra\0\xAEa\0\xB4V[Pa\x01\xD6V[`\xA0Q`\x01`\x01`\xA0\x1B\x03\x16c\x97\x11qZ`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x1A\x91\x90a\x01\xBDV[`\0\x80T`\x01`\xC0\x1B`\x01`\x01`@\x1B\x03B\x81\x16`\x01`\x80\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16`\x01`\x01`\x80\x1B\x03\x95\x90\x95\x16\x94\x90\x94\x17\x91\x90\x91\x17\x81\x81\x04\x84\x16`\x01\x01\x90\x93\x16\x02`\x01`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x17\x1B\x80a\x0Bd\x839\x01\x90V[a\x13\x8C\x80a\"\x7F\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x01\x9FW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xB6W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x01\xCFW`\0\x80\xFD[PQ\x91\x90PV[`\x80Q`\xA0Q`\xC0Qa\t'a\x02=`\09`\0\x81\x81a\x023\x01Ra\x04\xDF\x01R`\0\x81\x81a\x02\x0C\x01R\x81\x81a\x02\x9C\x01R\x81\x81a\x03h\x01R\x81\x81a\x04\x08\x01R\x81\x81a\x05\xB0\x01Ra\x06\x91\x01R`\0\x81\x81a\x01N\x01R\x81\x81a\x03\x05\x01Ra\x06\"\x01Ra\t'`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80coJ,\xD0\x11a\0qW\x80coJ,\xD0\x14a\x01\xA3W\x80cpv\xB7\xCD\x14a\x01\xB9W\x80c\xB6\xB5_%\x14a\x01\xE4W\x80c\xC0\x03N\x0C\x14a\x01\xF7W\x80c\xDAh\xCF\x8B\x14a\x02\x07W\x80c\xF7\xC6\x18\xC1\x14a\x02.W`\0\x80\xFD[\x80c\x1AF]#\x14a\0\xB9W\x80c+\x7F\x81\xFE\x14a\0\xD6W\x80c.\x1A}M\x14a\x01\nW\x80c;c+%\x14a\x01\x1FW\x80cC\xCD\x8F~\x14a\x01IW\x80cN'\x86\xFB\x14a\x01\x88W[`\0\x80\xFD[a\0\xC1a\x02UV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xF1\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xCDV[a\x01\x1Da\x01\x186`\x04a\x08nV[a\x02\x80V[\0[a\0\xF1a\x01-6`\x04a\x08\x87V[`\x01` R`\0\x90\x81R`@\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCDV[`\0Ta\0\xF1\x90`\x01`\xC0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xABa\x03.V[`@Q\x90\x81R` \x01a\0\xCDV[`\0Ta\x01\xCC\x90`\x01`\x01`\x80\x1B\x03\x16\x81V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCDV[a\x01\x1Da\x01\xF26`\x04a\x08nV[a\x05sV[a\x01\xABh\x05k\xC7^-c\x10\0\0\x81V[a\x01p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80Ta\x02x\x90b\x06\x97\x80\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xB7V[B\x10\x15\x90P\x90V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xFCW=`\0\x80>=`\0\xFD[PPPPa\x03+\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x83a\x06IV[PV[`\0a\x038a\x02UV[\x15a\x03EWa\x03Ea\x06\x8FV[`\0\x80T`@Qc\t\x81\xB2M`\xE4\x1B\x81R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x98\x1B$\xD0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x08\xD8V[`\0\x80T`@Qc'qf\xBF`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`$\x82\x01R\x91\x92P\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cN\xE2\xCD~\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04{\x91\x90a\x08\xD8V[\x90P`\0\x81\x11\x80\x15a\x04\x8DWP`\0\x82\x11[\x15a\x05nWa\x04\xA6\x81h\x05k\xC7^-c\x10\0\0\x84a\x07mV[\x92P`\0\x83\x11\x80\x15a\x04\xBEWPa\x04\xBC3a\x07\x94V[\x15[\x15a\x05nW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05?W=`\0\x80>=`\0\xFD[PP3`\0\x90\x81R`\x01` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPP[PP\x90V[\x80`\0\x03a\x05\x94W`@Qc\xFE\x9B\xA5\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x10W=`\0\x80>=`\0\xFD[PPPPa\x06\x1Ca\x03.V[Pa\x03+\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0030\x84a\x08\x1AV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\x06\x85Wc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x97\x11qZ`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x13\x91\x90a\x08\xD8V[`\0\x80T`\x01`\xC0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16`\x01`\x80\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16`\x01`\x01`\x80\x1B\x03\x95\x90\x95\x16\x94\x90\x94\x17\x91\x90\x91\x17\x81\x81\x04\x84\x16`\x01\x01\x90\x93\x16\x02`\x01`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x07\x8DWc\xAD%\x1C'`\0R`\x04`\x1C\xFD[P\x91\x02\x04\x90V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16\x82R`\x01` R`@\x82 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x90\x92\x04\x82\x16\x91\x16\x10\x80\x15\x90a\x08\x14WP`\0Ta\x07\xEE\x90b\x06\x97\x80\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xB7V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x92\x91PPV[`@Q\x81``R\x82`@R\x83``\x1B`,Rc#\xB8r\xDD``\x1B`\x0CR` `\0`d`\x1C`\0\x89Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\x08`Wcy9\xF4$`\0R`\x04`\x1C\xFD[`\0``R`@RPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x80W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\x99W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xB0W`\0\x80\xFD[\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x08\x14WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xEAW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x91\xA9N\xEB\x9D\xFE\x8B0\x93#\\\x88\x07\xBC\x0F\xFDT\x94\x11\xC68\xB1\xC5\x8Ez\x90\xEF\x8F\x17;\xFAidsolcC\0\x08\x14\x003`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e9*7\xB5\xB2\xB7`\xD1\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c9*%\xA7`\xE1\x1B\x81RP\x81`\x03\x90\x81b\0\0_\x91\x90b\0\x01\xC1V[P`\x04b\0\0n\x82\x82b\0\x01\xC1V[PPPb\0\0\x823b\0\0\x95` \x1B` \x1CV[b\0\0\x8F3`\x07b\0\0\xD1V[b\0\x02\x8DV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80`\0\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01hWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xBCW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\x97WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\xB8W\x82\x81U`\x01\x01b\0\x01\xA3V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01\xDDWb\0\x01\xDDb\0\x01\x1CV[b\0\x01\xF5\x81b\0\x01\xEE\x84Tb\0\x012V[\x84b\0\x01nV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02-W`\0\x84\x15b\0\x02\x14WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\xB8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02^W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02=V[P\x85\x82\x10\x15b\0\x02}W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x14~\x80b\0\x02\x9D`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x04W`\x005`\xE0\x1C\x80cT\xD1\xF1=\x11a\x01\x18W\x80c\x9D\xC2\x9F\xAC\x11a\0\xA0W\x80c\xD7S?\x02\x11a\0oW\x80c\xD7S?\x02\x14a\x05\x8FW\x80c\xDDb\xED>\x14a\x05\xADW\x80c\xF0N(>\x14a\x05\xCDW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE0W\x80c\xFE\xE8\x1C\xF4\x14a\x05\xF3W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x14a\x05:W\x80c\xA4W\xC2\xD7\x14a\x05ZW\x80c\xA9\x05\x9C\xBB\x14a\x024W\x80c\xD59\x13\x93\x14a\x05zW`\0\x80\xFD[\x80csY\xE4\x1F\x11a\0\xE7W\x80csY\xE4\x1F\x14a\x04\x97W\x80c\x8D\xA5\xCB[\x14a\x04\xC4W\x80c\x95\xD8\x9BA\x14a\x04\xF0W\x80c\x97\x11qZ\x14a\x05\x05W\x80c\x98\x1B$\xD0\x14a\x05\x1AW`\0\x80\xFD[\x80cT\xD1\xF1=\x14a\x04<W\x80cp(\xE2\xCD\x14a\x04DW\x80cp\xA0\x821\x14a\x04YW\x80cqP\x18\xA6\x14a\x04\x8FW`\0\x80\xFD[\x80c%i)b\x11a\x01\x9BW\x80c9P\x93Q\x11a\x01jW\x80c9P\x93Q\x14a\x03\x92W\x80c@\xC1\x0F\x19\x14a\x03\xB2W\x80cJN\xE7\xB1\x14a\x03\xD2W\x80cN\xE2\xCD~\x14a\x03\xE5W\x80cQNb\xFC\x14a\x04\x05W`\0\x80\xFD[\x80c%i)b\x14a\x03&W\x80c(,Q\xF3\x14a\x03.W\x80c-\xE9H\x07\x14a\x03CW\x80c1<\xE5g\x14a\x03vW`\0\x80\xFD[\x80c\x18:On\x11a\x01\xD7W\x80c\x18:On\x14a\x02\xA7W\x80c\x1C\x10\x89?\x14a\x02\xBCW\x80c\x1C\xD6M\xF4\x14a\x02\xCFW\x80c#\xB8r\xDD\x14a\x03\x06W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x02\tW\x80c\t^\xA7\xB3\x14a\x024W\x80c\x13\xA6a\xED\x14a\x02dW\x80c\x18\x16\r\xDD\x14a\x02\x92W[`\0\x80\xFD[4\x80\x15a\x02\x15W`\0\x80\xFD[Pa\x02\x1Ea\x06&V[`@Qa\x02+\x91\x90a\x11;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02@W`\0\x80\xFD[Pa\x02Ta\x02O6`\x04a\x11\xA0V[a\x06\xB8V[`@Q\x90\x15\x15\x81R` \x01a\x02+V[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x02\x84a\x02\x7F6`\x04a\x11\xF1V[a\x06\xD2V[`@Q\x90\x81R` \x01a\x02+V[4\x80\x15a\x02\x9EW`\0\x80\xFD[P`\x02Ta\x02\x84V[a\x02\xBAa\x02\xB56`\x04a\x12\xB6V[a\x06\xFBV[\0[a\x02\xBAa\x02\xCA6`\x04a\x11\xA0V[a\x07\x08V[4\x80\x15a\x02\xDBW`\0\x80\xFD[Pa\x02Ta\x02\xEA6`\x04a\x11\xA0V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x03\x12W`\0\x80\xFD[Pa\x02Ta\x03!6`\x04a\x12\xCFV[a\x07\x1EV[a\x02\xBAa\x07BV[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x02\x84`\x04\x81V[4\x80\x15a\x03OW`\0\x80\xFD[Pa\x02\x84a\x03^6`\x04a\x13\x0BV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03\x82W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x02+V[4\x80\x15a\x03\x9EW`\0\x80\xFD[Pa\x02Ta\x03\xAD6`\x04a\x11\xA0V[a\x07\x92V[4\x80\x15a\x03\xBEW`\0\x80\xFD[Pa\x02\xBAa\x03\xCD6`\x04a\x11\xA0V[a\x07\xB4V[a\x02\xBAa\x03\xE06`\x04a\x11\xA0V[a\x07\xCEV[4\x80\x15a\x03\xF1W`\0\x80\xFD[Pa\x02\x84a\x04\x006`\x04a\x11\xA0V[a\x07\xE0V[4\x80\x15a\x04\x11W`\0\x80\xFD[Pa\x02Ta\x04 6`\x04a\x11\xA0V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02\xBAa\x089V[4\x80\x15a\x04PW`\0\x80\xFD[Pa\x02\x84`\x02\x81V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x02\x84a\x04t6`\x04a\x13\x0BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02\xBAa\x08uV[4\x80\x15a\x04\xA3W`\0\x80\xFD[Pa\x04\xB7a\x04\xB26`\x04a\x12\xB6V[a\x08\x89V[`@Qa\x02+\x91\x90a\x13&V[4\x80\x15a\x04\xD0W`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[4\x80\x15a\x04\xFCW`\0\x80\xFD[Pa\x02\x1Ea\x08\xC2V[4\x80\x15a\x05\x11W`\0\x80\xFD[Pa\x02\x84a\x08\xD1V[4\x80\x15a\x05&W`\0\x80\xFD[Pa\x02\x84a\x0556`\x04a\x12\xB6V[a\x08\xECV[4\x80\x15a\x05FW`\0\x80\xFD[Pa\x02\xBAa\x05U6`\x04a\x11\xA0V[a\t\x17V[4\x80\x15a\x05fW`\0\x80\xFD[Pa\x02Ta\x05u6`\x04a\x11\xA0V[a\t,V[4\x80\x15a\x05\x86W`\0\x80\xFD[Pa\x02\x84`\x01\x81V[4\x80\x15a\x05\x9BW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x02+V[4\x80\x15a\x05\xB9W`\0\x80\xFD[Pa\x02\x84a\x05\xC86`\x04a\x13mV[a\t\xACV[a\x02\xBAa\x05\xDB6`\x04a\x13\x0BV[a\t\xD7V[a\x02\xBAa\x05\xEE6`\x04a\x13\x0BV[a\n\x14V[4\x80\x15a\x05\xFFW`\0\x80\xFD[Pa\x02\x84a\x06\x0E6`\x04a\x13\x0BV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[```\x03\x80Ta\x065\x90a\x13\xA0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06a\x90a\x13\xA0V[\x80\x15a\x06\xAEW\x80`\x1F\x10a\x06\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\xC6\x81\x85\x85a\n;V[`\x01\x91PP[\x92\x91PPV[`\0\x81Q`\x05\x1B[\x80\x15a\x06\xF5W\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x06\xDAV[P\x91\x90PV[a\x07\x053\x82a\nTV[PV[a\x07\x10a\n\xA3V[a\x07\x1A\x82\x82a\n\xBEV[PPV[`\x003a\x07,\x85\x82\x85a\x0B\tV[a\x077\x85\x85\x85a\n;V[P`\x01\x94\x93PPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\x003a\x06\xC6\x81\x85\x85a\x07\xA5\x83\x83a\t\xACV[a\x07\xAF\x91\x90a\x13\xEAV[a\n;V[`\x01a\x07\xBF\x81a\x0B\x83V[a\x07\xC9\x83\x83a\x0B\xA9V[PPPV[a\x07\xD6a\n\xA3V[a\x07\x1A\x82\x82a\nTV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x05` R`@\x81 \x81\x90\x81\x90a\x08\x07\x90\x85\x90a\x0CtV[\x91P\x91P\x81a\x08.W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 Ta\x080V[\x80[\x95\x94PPPPPV[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x08}a\n\xA3V[a\x08\x87`\0a\rjV[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\x08\x94WPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x04\x80Ta\x065\x90a\x13\xA0V[`\0`\x02a\x08\xDE\x81a\x0B\x83V[a\x08\xE6a\r\xA8V[\x91PP\x90V[`\0\x80`\0a\x08\xFC\x84`\x06a\x0CtV[\x91P\x91P\x81a\t\rW`\x02Ta\t\x0FV[\x80[\x94\x93PPPPV[`\x04a\t\"\x81a\x0B\x83V[a\x07\xC9\x83\x83a\x0E\x02V[`\x003\x81a\t:\x82\x86a\t\xACV[\x90P\x83\x81\x10\x15a\t\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x077\x82\x86\x86\x84\x03a\n;V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\t\xDFa\n\xA3V[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\n\x07Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x07\x05\x81a\rjV[a\n\x1Ca\n\xA3V[\x80``\x1Ba\n2WctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x07\x05\x81a\rjV[`@Qc\xD6#G%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\x08\x87Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0a\x0B\x15\x84\x84a\t\xACV[\x90P`\0\x19\x81\x14a\x0B}W\x81\x81\x10\x15a\x0BpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\t\x96V[a\x0B}\x84\x84\x84\x84\x03a\n;V[PPPPV[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x07\x05Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\t\x96V[a\x0C\x0B`\0\x83\x83a\x0F@V[\x80`\x02`\0\x82\x82Ta\x0C\x1D\x91\x90a\x13\xEAV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80`\0\x84\x11a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x04U$3#\x056\xE6\x17\x076\x86\xF7C\xA2\x06\x96B\x06\x972\x03`T\x1B`D\x82\x01R`d\x01a\t\x96V[a\x0C\xC8a\x0F\x88V[\x84\x11\x15a\r\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Snapshot: nonexistent id\0\0\0`D\x82\x01R`d\x01a\t\x96V[`\0a\r#\x84\x86a\x0F\x98V[\x84T\x90\x91P\x81\x03a\r;W`\0\x80\x92P\x92PPa\rcV[`\x01\x84`\x01\x01\x82\x81T\x81\x10a\rRWa\rRa\x13\xFDV[\x90`\0R` `\0 \x01T\x92P\x92PP[\x92P\x92\x90PV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\0a\r\xB8`\x08\x80T`\x01\x01\x90UV[`\0a\r\xC2a\x0F\x88V[\x90P\x7F\x800\xE8;\x04\xD8{\xEFSH\x0E&&2f\xD6\xCAf\x86:\xA8Pj\xCAo%Y\xD1\x8A\xA1\xCBg\x81`@Qa\r\xF5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0EbW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\t\x96V[a\x0En\x82`\0\x83a\x0F@V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0E\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\t\x96V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x86\x86\x03\x90U`\x02\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0F_Wa\x0FW\x82a\x10EV[a\x07\xC9a\x10wV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0FvWa\x0FW\x83a\x10EV[a\x0F\x7F\x83a\x10EV[a\x07\xC9\x82a\x10EV[`\0a\x0F\x93`\x08T\x90V[\x90P\x90V[\x81T`\0\x90\x81\x03a\x0F\xABWP`\0a\x06\xCCV[\x82T`\0\x90[\x80\x82\x10\x15a\x0F\xF8W`\0a\x0F\xC5\x83\x83a\x10\x85V[`\0\x87\x81R` \x90 \x90\x91P\x85\x90\x82\x01T\x11\x15a\x0F\xE4W\x80\x91Pa\x0F\xF2V[a\x0F\xEF\x81`\x01a\x13\xEAV[\x92P[Pa\x0F\xB1V[`\0\x82\x11\x80\x15a\x10$WP\x83a\x10!\x86a\x10\x13`\x01\x86a\x14\x13V[`\0\x91\x82R` \x90\x91 \x01\x90V[T\x14[\x15a\x10=Wa\x104`\x01\x83a\x14\x13V[\x92PPPa\x06\xCCV[P\x90Pa\x06\xCCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x91\x83\x90R\x90\x91 Ta\x07\x05\x91\x90a\x10\xA7V[a\x10\xA7V[a\x08\x87`\x06a\x10r`\x02T\x90V[`\0a\x10\x94`\x02\x84\x84\x18a\x14&V[a\x10\xA0\x90\x84\x84\x16a\x13\xEAV[\x93\x92PPPV[`\0a\x10\xB1a\x0F\x88V[\x90P\x80a\x10\xBD\x84a\x10\xF1V[\x10\x15a\x07\xC9W\x82T`\x01\x80\x82\x01\x85U`\0\x85\x81R` \x80\x82 \x90\x93\x01\x93\x90\x93U\x93\x84\x01\x80T\x94\x85\x01\x81U\x82R\x90 \x90\x91\x01UV[\x80T`\0\x90\x81\x03a\x11\x04WP`\0\x91\x90PV[\x81T\x82\x90a\x11\x14\x90`\x01\x90a\x14\x13V[\x81T\x81\x10a\x11$Wa\x11$a\x13\xFDV[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x11hW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x11LV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x116W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x11\xB3W`\0\x80\xFD[a\x11\xBC\x83a\x11\x89V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\xFF\x81\x16\x81\x14a\x116W`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x12\x04W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\x1CW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x120W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12BWa\x12Ba\x11\xCAV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x12gWa\x12ga\x11\xCAV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x12\x85W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x12\xAAWa\x12\x9B\x85a\x11\xE0V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x12\x8AV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x12\xC8W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xE4W`\0\x80\xFD[a\x12\xED\x84a\x11\x89V[\x92Pa\x12\xFB` \x85\x01a\x11\x89V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x13\x1DW`\0\x80\xFD[a\x10\xA0\x82a\x11\x89V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x13aW\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x13BV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x80W`\0\x80\xFD[a\x13\x89\x83a\x11\x89V[\x91Pa\x13\x97` \x84\x01a\x11\x89V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\xB4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xF5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xCCWa\x06\xCCa\x13\xD4V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xCCWa\x06\xCCa\x13\xD4V[`\0\x82a\x14CWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 ]\x91f\x0E\xBF\xC4\xAA\xBF\xA2\x07\x93l\xED\x9F\x86\x80\xA8=\x89;\x81\x9A\x16c\xAEa\xB3\xDC\xCFy\x82\xC8dsolcC\0\x08\x14\x003`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k)2\xBB\xB0\xB92\x10*7\xB5\xB2\xB7`\xA1\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x14\x95\xD5`\xEA\x1B\x81RP\x81`\x03\x90\x81b\0\0d\x91\x90b\0\x01\xC6V[P`\x04b\0\0s\x82\x82b\0\x01\xC6V[PPPb\0\0\x873b\0\0\x9A` \x1B` \x1CV[b\0\0\x943`\x01b\0\0\xD6V[b\0\x02\x92V[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80`\0\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01LW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01mWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xC1W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\x9CWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\xBDW\x82\x81U`\x01\x01b\0\x01\xA8V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01\xE2Wb\0\x01\xE2b\0\x01!V[b\0\x01\xFA\x81b\0\x01\xF3\x84Tb\0\x017V[\x84b\0\x01sV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x022W`\0\x84\x15b\0\x02\x19WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\xBDV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02cW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02BV[P\x85\x82\x10\x15b\0\x02\x82W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x10\xEA\x80b\0\x02\xA2`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xC2W`\x005`\xE0\x1C\x80cQNb\xFC\x11a\0\xF7W\x80c\xA4W\xC2\xD7\x11a\0\x95W\x80c\xDDb\xED>\x11a\0dW\x80c\xDDb\xED>\x14a\x04\xECW\x80c\xF0N(>\x14a\x05\x0CW\x80c\xF2\xFD\xE3\x8B\x14a\x05\x1FW\x80c\xFE\xE8\x1C\xF4\x14a\x052W`\0\x80\xFD[\x80c\xA4W\xC2\xD7\x14a\x04yW\x80c\xA9\x05\x9C\xBB\x14a\x04\x99W\x80c\xD59\x13\x93\x14a\x04\xB9W\x80c\xD7S?\x02\x14a\x04\xCEW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\0\xD1W\x80cqP\x18\xA6\x14a\x04\x03W\x80csY\xE4\x1F\x14a\x04\x0BW\x80c\x8D\xA5\xCB[\x14a\x048W\x80c\x95\xD8\x9BA\x14a\x04dW`\0\x80\xFD[\x80cQNb\xFC\x14a\x03\x8EW\x80cT\xD1\xF1=\x14a\x03\xC5W\x80cp\xA0\x821\x14a\x03\xCDW`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01dW\x80c1<\xE5g\x11a\x01>W\x80c1<\xE5g\x14a\x03\x1FW\x80c9P\x93Q\x14a\x03;W\x80c@\xC1\x0F\x19\x14a\x03[W\x80cJN\xE7\xB1\x14a\x03{W`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x02\xC4W\x80c%i)b\x14a\x02\xE4W\x80c-\xE9H\x07\x14a\x02\xECW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x01\xA0W\x80c\x18\x16\r\xDD\x14a\x02PW\x80c\x18:On\x14a\x02eW\x80c\x1C\x10\x89?\x14a\x02zW\x80c\x1C\xD6M\xF4\x14a\x02\x8DW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\xC7W\x80c\t^\xA7\xB3\x14a\x01\xF2W\x80c\x13\xA6a\xED\x14a\x02\"W[`\0\x80\xFD[4\x80\x15a\x01\xD3W`\0\x80\xFD[Pa\x01\xDCa\x05eV[`@Qa\x01\xE9\x91\x90a\r\xEEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFEW`\0\x80\xFD[Pa\x02\x12a\x02\r6`\x04a\x0EXV[a\x05\xF7V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE9V[4\x80\x15a\x02.W`\0\x80\xFD[Pa\x02Ba\x02=6`\x04a\x0E\xA9V[a\x06\x11V[`@Q\x90\x81R` \x01a\x01\xE9V[4\x80\x15a\x02\\W`\0\x80\xFD[P`\x02Ta\x02BV[a\x02xa\x02s6`\x04a\x0FnV[a\x06:V[\0[a\x02xa\x02\x886`\x04a\x0EXV[a\x06GV[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\x12a\x02\xA86`\x04a\x0EXV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x02\xD0W`\0\x80\xFD[Pa\x02\x12a\x02\xDF6`\x04a\x0F\x87V[a\x06]V[a\x02xa\x06\x81V[4\x80\x15a\x02\xF8W`\0\x80\xFD[Pa\x02Ba\x03\x076`\x04a\x0F\xC3V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03+W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x01\xE9V[4\x80\x15a\x03GW`\0\x80\xFD[Pa\x02\x12a\x03V6`\x04a\x0EXV[a\x06\xD1V[4\x80\x15a\x03gW`\0\x80\xFD[Pa\x02xa\x03v6`\x04a\x0EXV[a\x06\xF3V[a\x02xa\x03\x896`\x04a\x0EXV[a\x07\rV[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x02\x12a\x03\xA96`\x04a\x0EXV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02xa\x07\x1FV[4\x80\x15a\x03\xD9W`\0\x80\xFD[Pa\x02Ba\x03\xE86`\x04a\x0F\xC3V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02xa\x07[V[4\x80\x15a\x04\x17W`\0\x80\xFD[Pa\x04+a\x04&6`\x04a\x0FnV[a\x07oV[`@Qa\x01\xE9\x91\x90a\x0F\xE5V[4\x80\x15a\x04DW`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE9V[4\x80\x15a\x04pW`\0\x80\xFD[Pa\x01\xDCa\x07\xA8V[4\x80\x15a\x04\x85W`\0\x80\xFD[Pa\x02\x12a\x04\x946`\x04a\x0EXV[a\x07\xB7V[4\x80\x15a\x04\xA5W`\0\x80\xFD[Pa\x02\x12a\x04\xB46`\x04a\x0EXV[a\x087V[4\x80\x15a\x04\xC5W`\0\x80\xFD[Pa\x02B`\x01\x81V[4\x80\x15a\x04\xDAW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x01\xE9V[4\x80\x15a\x04\xF8W`\0\x80\xFD[Pa\x02Ba\x05\x076`\x04a\x10,V[a\x08EV[a\x02xa\x05\x1A6`\x04a\x0F\xC3V[a\x08pV[a\x02xa\x05-6`\x04a\x0F\xC3V[a\x08\xADV[4\x80\x15a\x05>W`\0\x80\xFD[Pa\x02Ba\x05M6`\x04a\x0F\xC3V[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[```\x03\x80Ta\x05t\x90a\x10_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xA0\x90a\x10_V[\x80\x15a\x05\xEDW\x80`\x1F\x10a\x05\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xEDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\x05\x81\x85\x85a\x08\xD4V[`\x01\x91PP[\x92\x91PPV[`\0\x81Q`\x05\x1B[\x80\x15a\x064W\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x06\x19V[P\x91\x90PV[a\x06D3\x82a\t\xF8V[PV[a\x06Oa\nGV[a\x06Y\x82\x82a\nbV[PPV[`\x003a\x06k\x85\x82\x85a\n\xADV[a\x06v\x85\x85\x85a\x0B'V[P`\x01\x94\x93PPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\x003a\x06\x05\x81\x85\x85a\x06\xE4\x83\x83a\x08EV[a\x06\xEE\x91\x90a\x10\x93V[a\x08\xD4V[`\x01a\x06\xFE\x81a\x0C\xCBV[a\x07\x08\x83\x83a\x0C\xF1V[PPPV[a\x07\x15a\nGV[a\x06Y\x82\x82a\t\xF8V[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x07ca\nGV[a\x07m`\0a\r\xB0V[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\x07zWPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x04\x80Ta\x05t\x90a\x10_V[`\x003\x81a\x07\xC5\x82\x86a\x08EV[\x90P\x83\x81\x10\x15a\x08*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06v\x82\x86\x86\x84\x03a\x08\xD4V[`\x003a\x06\x05\x81\x85\x85a\x0B'V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x08xa\nGV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x08\xA0Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x06D\x81a\r\xB0V[a\x08\xB5a\nGV[\x80``\x1Ba\x08\xCBWctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x06D\x81a\r\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\t6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\x07mWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0a\n\xB9\x84\x84a\x08EV[\x90P`\0\x19\x81\x14a\x0B!W\x81\x81\x10\x15a\x0B\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x08!V[a\x0B!\x84\x84\x84\x84\x03a\x08\xD4V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0B\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0CeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x87\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x92Q\x85\x81R\x90\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x0B!V[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x06DWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\rGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x08!V[\x80`\x02`\0\x82\x82Ta\rY\x91\x90a\x10\x93V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x0E\x1BW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\r\xFFV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0ESW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0EkW`\0\x80\xFD[a\x0Et\x83a\x0E<V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\xFF\x81\x16\x81\x14a\x0ESW`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x0E\xBCW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xD4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\xE8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0E\xFAWa\x0E\xFAa\x0E\x82V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x0F\x1FWa\x0F\x1Fa\x0E\x82V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x0F=W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x0FbWa\x0FS\x85a\x0E\x98V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x0FBV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\x80W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x9CW`\0\x80\xFD[a\x0F\xA5\x84a\x0E<V[\x92Pa\x0F\xB3` \x85\x01a\x0E<V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\xD5W`\0\x80\xFD[a\x0F\xDE\x82a\x0E<V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10 W\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10\x01V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10?W`\0\x80\xFD[a\x10H\x83a\x0E<V[\x91Pa\x10V` \x84\x01a\x0E<V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x10sW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x064WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\x0BWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x94\x81\xCC\x19\xA6\xCFwI\xD2M\xF6\xD5\r\x8A\xE8\x0FaCR>\x7F\xABZ\x0C\x92\xEA\xEF\x1F\xFFU\xFC\x9FdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static THEREWARDERPOOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80coJ,\xD0\x11a\0qW\x80coJ,\xD0\x14a\x01\xA3W\x80cpv\xB7\xCD\x14a\x01\xB9W\x80c\xB6\xB5_%\x14a\x01\xE4W\x80c\xC0\x03N\x0C\x14a\x01\xF7W\x80c\xDAh\xCF\x8B\x14a\x02\x07W\x80c\xF7\xC6\x18\xC1\x14a\x02.W`\0\x80\xFD[\x80c\x1AF]#\x14a\0\xB9W\x80c+\x7F\x81\xFE\x14a\0\xD6W\x80c.\x1A}M\x14a\x01\nW\x80c;c+%\x14a\x01\x1FW\x80cC\xCD\x8F~\x14a\x01IW\x80cN'\x86\xFB\x14a\x01\x88W[`\0\x80\xFD[a\0\xC1a\x02UV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xF1\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xCDV[a\x01\x1Da\x01\x186`\x04a\x08nV[a\x02\x80V[\0[a\0\xF1a\x01-6`\x04a\x08\x87V[`\x01` R`\0\x90\x81R`@\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCDV[`\0Ta\0\xF1\x90`\x01`\xC0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xABa\x03.V[`@Q\x90\x81R` \x01a\0\xCDV[`\0Ta\x01\xCC\x90`\x01`\x01`\x80\x1B\x03\x16\x81V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCDV[a\x01\x1Da\x01\xF26`\x04a\x08nV[a\x05sV[a\x01\xABh\x05k\xC7^-c\x10\0\0\x81V[a\x01p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80Ta\x02x\x90b\x06\x97\x80\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xB7V[B\x10\x15\x90P\x90V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xFCW=`\0\x80>=`\0\xFD[PPPPa\x03+\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x83a\x06IV[PV[`\0a\x038a\x02UV[\x15a\x03EWa\x03Ea\x06\x8FV[`\0\x80T`@Qc\t\x81\xB2M`\xE4\x1B\x81R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x98\x1B$\xD0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x08\xD8V[`\0\x80T`@Qc'qf\xBF`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`$\x82\x01R\x91\x92P\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cN\xE2\xCD~\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04{\x91\x90a\x08\xD8V[\x90P`\0\x81\x11\x80\x15a\x04\x8DWP`\0\x82\x11[\x15a\x05nWa\x04\xA6\x81h\x05k\xC7^-c\x10\0\0\x84a\x07mV[\x92P`\0\x83\x11\x80\x15a\x04\xBEWPa\x04\xBC3a\x07\x94V[\x15[\x15a\x05nW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05?W=`\0\x80>=`\0\xFD[PP3`\0\x90\x81R`\x01` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPP[PP\x90V[\x80`\0\x03a\x05\x94W`@Qc\xFE\x9B\xA5\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x10W=`\0\x80>=`\0\xFD[PPPPa\x06\x1Ca\x03.V[Pa\x03+\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0030\x84a\x08\x1AV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\x06\x85Wc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x97\x11qZ`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x13\x91\x90a\x08\xD8V[`\0\x80T`\x01`\xC0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16`\x01`\x80\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16`\x01`\x01`\x80\x1B\x03\x95\x90\x95\x16\x94\x90\x94\x17\x91\x90\x91\x17\x81\x81\x04\x84\x16`\x01\x01\x90\x93\x16\x02`\x01`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x82`\0\x19\x04\x84\x11\x83\x02\x15\x82\x02a\x07\x8DWc\xAD%\x1C'`\0R`\x04`\x1C\xFD[P\x91\x02\x04\x90V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16\x82R`\x01` R`@\x82 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x90\x92\x04\x82\x16\x91\x16\x10\x80\x15\x90a\x08\x14WP`\0Ta\x07\xEE\x90b\x06\x97\x80\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xB7V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x92\x91PPV[`@Q\x81``R\x82`@R\x83``\x1B`,Rc#\xB8r\xDD``\x1B`\x0CR` `\0`d`\x1C`\0\x89Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\x08`Wcy9\xF4$`\0R`\x04`\x1C\xFD[`\0``R`@RPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x80W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\x99W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xB0W`\0\x80\xFD[\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x08\x14WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xEAW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x91\xA9N\xEB\x9D\xFE\x8B0\x93#\\\x88\x07\xBC\x0F\xFDT\x94\x11\xC68\xB1\xC5\x8Ez\x90\xEF\x8F\x17;\xFAidsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static THEREWARDERPOOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TheRewarderPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TheRewarderPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TheRewarderPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TheRewarderPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TheRewarderPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TheRewarderPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TheRewarderPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    THEREWARDERPOOL_ABI.clone(),
                    client,
                ),
            )
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
                THEREWARDERPOOL_ABI.clone(),
                THEREWARDERPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `REWARDS` (0xc0034e0c) function
        pub fn rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 3, 78, 12], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accountingToken` (0xda68cf8b) function
        pub fn accounting_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([218, 104, 207, 139], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xb6b55f25) function
        pub fn deposit(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 181, 95, 37], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributeRewards` (0x6f4a2cd0) function
        pub fn distribute_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([111, 74, 44, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isNewRewardsRound` (0x1a465d23) function
        pub fn is_new_rewards_round(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([26, 70, 93, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastRecordedSnapshotTimestamp` (0x2b7f81fe) function
        pub fn last_recorded_snapshot_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([43, 127, 129, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastRewardTimestamps` (0x3b632b25) function
        pub fn last_reward_timestamps(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([59, 99, 43, 37], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastSnapshotIdForRewards` (0x7076b7cd) function
        pub fn last_snapshot_id_for_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 118, 183, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidityToken` (0x43cd8f7e) function
        pub fn liquidity_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([67, 205, 143, 126], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardToken` (0xf7c618c1) function
        pub fn reward_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([247, 198, 24, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roundNumber` (0x4e2786fb) function
        pub fn round_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([78, 39, 134, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TheRewarderPool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidDepositAmount` with signature `InvalidDepositAmount()` and selector `0xfe9ba5cd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidDepositAmount", abi = "InvalidDepositAmount()")]
    pub struct InvalidDepositAmount;
    ///Container type for all input parameters for the `REWARDS` function with signature `REWARDS()` and selector `0xc0034e0c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "REWARDS", abi = "REWARDS()")]
    pub struct RewardsCall;
    ///Container type for all input parameters for the `accountingToken` function with signature `accountingToken()` and selector `0xda68cf8b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "accountingToken", abi = "accountingToken()")]
    pub struct AccountingTokenCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256)` and selector `0xb6b55f25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256)")]
    pub struct DepositCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `distributeRewards` function with signature `distributeRewards()` and selector `0x6f4a2cd0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "distributeRewards", abi = "distributeRewards()")]
    pub struct DistributeRewardsCall;
    ///Container type for all input parameters for the `isNewRewardsRound` function with signature `isNewRewardsRound()` and selector `0x1a465d23`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isNewRewardsRound", abi = "isNewRewardsRound()")]
    pub struct IsNewRewardsRoundCall;
    ///Container type for all input parameters for the `lastRecordedSnapshotTimestamp` function with signature `lastRecordedSnapshotTimestamp()` and selector `0x2b7f81fe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "lastRecordedSnapshotTimestamp",
        abi = "lastRecordedSnapshotTimestamp()"
    )]
    pub struct LastRecordedSnapshotTimestampCall;
    ///Container type for all input parameters for the `lastRewardTimestamps` function with signature `lastRewardTimestamps(address)` and selector `0x3b632b25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lastRewardTimestamps", abi = "lastRewardTimestamps(address)")]
    pub struct LastRewardTimestampsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `lastSnapshotIdForRewards` function with signature `lastSnapshotIdForRewards()` and selector `0x7076b7cd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lastSnapshotIdForRewards", abi = "lastSnapshotIdForRewards()")]
    pub struct LastSnapshotIdForRewardsCall;
    ///Container type for all input parameters for the `liquidityToken` function with signature `liquidityToken()` and selector `0x43cd8f7e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "liquidityToken", abi = "liquidityToken()")]
    pub struct LiquidityTokenCall;
    ///Container type for all input parameters for the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rewardToken", abi = "rewardToken()")]
    pub struct RewardTokenCall;
    ///Container type for all input parameters for the `roundNumber` function with signature `roundNumber()` and selector `0x4e2786fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "roundNumber", abi = "roundNumber()")]
    pub struct RoundNumberCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TheRewarderPoolCalls {
        Rewards(RewardsCall),
        AccountingToken(AccountingTokenCall),
        Deposit(DepositCall),
        DistributeRewards(DistributeRewardsCall),
        IsNewRewardsRound(IsNewRewardsRoundCall),
        LastRecordedSnapshotTimestamp(LastRecordedSnapshotTimestampCall),
        LastRewardTimestamps(LastRewardTimestampsCall),
        LastSnapshotIdForRewards(LastSnapshotIdForRewardsCall),
        LiquidityToken(LiquidityTokenCall),
        RewardToken(RewardTokenCall),
        RoundNumber(RoundNumberCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for TheRewarderPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <RewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rewards(decoded));
            }
            if let Ok(decoded)
                = <AccountingTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AccountingToken(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <DistributeRewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DistributeRewards(decoded));
            }
            if let Ok(decoded)
                = <IsNewRewardsRoundCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsNewRewardsRound(decoded));
            }
            if let Ok(decoded)
                = <LastRecordedSnapshotTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastRecordedSnapshotTimestamp(decoded));
            }
            if let Ok(decoded)
                = <LastRewardTimestampsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastRewardTimestamps(decoded));
            }
            if let Ok(decoded)
                = <LastSnapshotIdForRewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastSnapshotIdForRewards(decoded));
            }
            if let Ok(decoded)
                = <LiquidityTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LiquidityToken(decoded));
            }
            if let Ok(decoded)
                = <RewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardToken(decoded));
            }
            if let Ok(decoded)
                = <RoundNumberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RoundNumber(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TheRewarderPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Rewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AccountingToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DistributeRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsNewRewardsRound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastRecordedSnapshotTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastRewardTimestamps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastSnapshotIdForRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidityToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoundNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TheRewarderPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Rewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountingToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DistributeRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsNewRewardsRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastRecordedSnapshotTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastRewardTimestamps(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastSnapshotIdForRewards(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidityToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RewardsCall> for TheRewarderPoolCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<AccountingTokenCall> for TheRewarderPoolCalls {
        fn from(value: AccountingTokenCall) -> Self {
            Self::AccountingToken(value)
        }
    }
    impl ::core::convert::From<DepositCall> for TheRewarderPoolCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DistributeRewardsCall> for TheRewarderPoolCalls {
        fn from(value: DistributeRewardsCall) -> Self {
            Self::DistributeRewards(value)
        }
    }
    impl ::core::convert::From<IsNewRewardsRoundCall> for TheRewarderPoolCalls {
        fn from(value: IsNewRewardsRoundCall) -> Self {
            Self::IsNewRewardsRound(value)
        }
    }
    impl ::core::convert::From<LastRecordedSnapshotTimestampCall>
    for TheRewarderPoolCalls {
        fn from(value: LastRecordedSnapshotTimestampCall) -> Self {
            Self::LastRecordedSnapshotTimestamp(value)
        }
    }
    impl ::core::convert::From<LastRewardTimestampsCall> for TheRewarderPoolCalls {
        fn from(value: LastRewardTimestampsCall) -> Self {
            Self::LastRewardTimestamps(value)
        }
    }
    impl ::core::convert::From<LastSnapshotIdForRewardsCall> for TheRewarderPoolCalls {
        fn from(value: LastSnapshotIdForRewardsCall) -> Self {
            Self::LastSnapshotIdForRewards(value)
        }
    }
    impl ::core::convert::From<LiquidityTokenCall> for TheRewarderPoolCalls {
        fn from(value: LiquidityTokenCall) -> Self {
            Self::LiquidityToken(value)
        }
    }
    impl ::core::convert::From<RewardTokenCall> for TheRewarderPoolCalls {
        fn from(value: RewardTokenCall) -> Self {
            Self::RewardToken(value)
        }
    }
    impl ::core::convert::From<RoundNumberCall> for TheRewarderPoolCalls {
        fn from(value: RoundNumberCall) -> Self {
            Self::RoundNumber(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for TheRewarderPoolCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `REWARDS` function with signature `REWARDS()` and selector `0xc0034e0c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RewardsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `accountingToken` function with signature `accountingToken()` and selector `0xda68cf8b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AccountingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `distributeRewards` function with signature `distributeRewards()` and selector `0x6f4a2cd0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DistributeRewardsReturn {
        pub rewards: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `isNewRewardsRound` function with signature `isNewRewardsRound()` and selector `0x1a465d23`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsNewRewardsRoundReturn(pub bool);
    ///Container type for all return fields from the `lastRecordedSnapshotTimestamp` function with signature `lastRecordedSnapshotTimestamp()` and selector `0x2b7f81fe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastRecordedSnapshotTimestampReturn(pub u64);
    ///Container type for all return fields from the `lastRewardTimestamps` function with signature `lastRewardTimestamps(address)` and selector `0x3b632b25`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastRewardTimestampsReturn(pub u64);
    ///Container type for all return fields from the `lastSnapshotIdForRewards` function with signature `lastSnapshotIdForRewards()` and selector `0x7076b7cd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastSnapshotIdForRewardsReturn(pub u128);
    ///Container type for all return fields from the `liquidityToken` function with signature `liquidityToken()` and selector `0x43cd8f7e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidityTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RewardTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `roundNumber` function with signature `roundNumber()` and selector `0x4e2786fb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RoundNumberReturn(pub u64);
}
