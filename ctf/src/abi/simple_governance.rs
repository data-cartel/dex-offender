pub use simple_governance::*;
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
pub mod simple_governance {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("governanceToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("executeAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeAction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISimpleGovernance.GovernanceAction",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getActionCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActionCounter"),
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
                    ::std::borrow::ToOwned::to_owned("getActionDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActionDelay"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGovernanceToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGovernanceToken"),
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
                    ::std::borrow::ToOwned::to_owned("queueAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queueAction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActionExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ActionExecuted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ActionQueued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ActionQueued"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActionFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ActionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotExecute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CannotExecute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTarget"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTarget"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughVotes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("who"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TargetMustHaveCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TargetMustHaveCode"),
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
    pub static SIMPLEGOVERNANCE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0CS8\x03\x80a\x0CS\x839\x81\x01`@\x81\x90Ra\0/\x91a\0XV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x01\x80Ua\0\x88V[`\0` \x82\x84\x03\x12\x15a\0jW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x81W`\0\x80\xFD[\x93\x92PPPV[a\x0B\xBC\x80a\0\x97`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80c\x12\x05z\x14\x14a\0ZW\x80c?\x8A\x03}\x14a\0\x7FW\x80cR\xEC\xB9\n\x14a\0\xA7W\x80c\x9A\xCA\x08\xD4\x14a\0\xC7W\x80c\xB6\xE7hs\x14a\0\xDCW\x80c\xC0\xC1\xCFU\x14a\x01\tW[`\0\x80\xFD[4\x80\x15a\0fW`\0\x80\xFD[Pb\x02\xA3\0[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x8BW`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0vV[4\x80\x15a\0\xB3W`\0\x80\xFD[Pa\0la\0\xC26`\x04a\x07\xEFV[a\x01)V[4\x80\x15a\0\xD3W`\0\x80\xFD[P`\x01Ta\0lV[4\x80\x15a\0\xE8W`\0\x80\xFD[Pa\0\xFCa\0\xF76`\x04a\x08\x99V[a\x03#V[`@Qa\0v\x91\x90a\x08\xF8V[a\x01\x1Ca\x01\x176`\x04a\x08\x99V[a\x04UV[`@Qa\0v\x91\x90a\t\\V[`\0a\x0143a\x05\x97V[a\x01XW`@Qc}\x89%u`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[0`\x01`\x01`\xA0\x1B\x03\x86\x16\x03a\x01\x81W`@QcAj\xEB\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15\x80\x15\x90a\x01\x98WP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15[\x15a\x01\xB6W`@Qcm\xD4\xAAe`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x90P`@Q\x80`\xA0\x01`@R\x80\x85`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP\x83\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x85Q\x81T\x92\x87\x01Q\x94\x87\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17`\x01`\x80\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16\x02\x17`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1B\x94\x90\x92\x16\x93\x90\x93\x02\x17\x82U``\x84\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\x80\x84\x01Q\x91\x92P\x82\x01\x90a\x02\xDB\x90\x82a\n\x15V[PP`\x01\x80T\x81\x01\x90UP`@Q\x81\x81R3\x90\x7FM\xFD\x92\xF6\x9E\x02\xF8,\x8Fg\x05\xB2\xE4\xA3dF[X\x8F\xA4w<\xDA&xl<\xA8\xDFC\xA1\x95\x90` \x01`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`\0\x82\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xC0\x1B\x90\x04\x90\x92\x16\x93\x82\x01\x93\x90\x93R`\x01\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R\x90\x82\x01\x80T\x91\x92\x91`\x80\x84\x01\x91\x90a\x03\xCC\x90a\t\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xF8\x90a\t\x8CV[\x80\x15a\x04EW\x80`\x1F\x10a\x04\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``a\x04`\x82a\x06\x95V[a\x04\x80W`@Qc\xB4R\xFA\xAF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x01OV[`\0\x82\x81R`\x02` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1BBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x81U\x91Q\x84\x81R3\x91\x7Fx\xA7\xD6Od\xEFh\x91\xBE\xDF\xC1\xB6\x85DG\x11<\xCA\x07\xAC\xE8t{\x8Al\xF3*\x0F\xD5\xA8\x0B\x17\x91\x01`@Q\x80\x91\x03\x90\xA2`\x01\x81\x01T\x81T`@Q`\0\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x90a\x05\x18\x90`\x02\x87\x01\x90a\n\xD5V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05UW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05ZV[``\x91P[P\x91P\x91P\x81a\x05\x8FW\x80Q\x15a\x05sW\x80Q\x81` \x01\xFD[`@Qc\xA6\xA7\xDB\xBD`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x01OV[\x94\x93PPPPV[`\0\x80T`@Qc\"\xC8\x8B'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x92\x16\x90cE\x91\x16N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x06\x91\x90a\x0BKV[\x90P`\0`\x02`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xECs\xFB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x81\x91\x90a\x0BKV[a\x06\x8B\x91\x90a\x0BdV[\x90\x91\x11\x93\x92PPPV[`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\xC0\x1B\x90\x04\x90\x93\x16\x91\x83\x01\x91\x90\x91R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01R\x91\x82\x01\x80T\x84\x93\x91`\x80\x84\x01\x91a\x07\x0F\x90a\t\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07;\x90a\t\x8CV[\x80\x15a\x07\x88W\x80`\x1F\x10a\x07]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x07\xB2WP`\0\x92\x91PPV[`\0\x81` \x01QB\x03\x90P\x81`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14\x80\x15a\x05\x8FWPb\x02\xA3\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x94\x93PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x08\x05W`\0\x80\xFD[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x1CW`\0\x80\xFD[\x93P` \x85\x015`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x088W`\0\x80\xFD[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08UW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x08iW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08xW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x08\x8AW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xABW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x08\xD8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x08\xBCV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\x01`\x01`\x80\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R\x80`@\x86\x01Q\x16``\x85\x01RPP`\x01\x80`\xA0\x1B\x03``\x84\x01Q\x16`\x80\x83\x01R`\x80\x83\x01Q`\xA0\x80\x84\x01Ra\x05\x8F`\xC0\x84\x01\x82a\x08\xB2V[` \x81R`\0a\to` \x83\x01\x84a\x08\xB2V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\t\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\xC0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\n\x10W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\t\xEDWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\n\x0CW\x82\x81U`\x01\x01a\t\xF9V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n/Wa\n/a\tvV[a\nC\x81a\n=\x84Ta\t\x8CV[\x84a\t\xC6V[` \x80`\x1F\x83\x11`\x01\x81\x14a\nxW`\0\x84\x15a\n`WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\n\x0CV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\n\xA7W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\n\x88V[P\x85\x82\x10\x15a\n\xC5W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Ta\n\xE3\x81a\t\x8CV[`\x01\x82\x81\x16\x80\x15a\n\xFBW`\x01\x81\x14a\x0B\x10Wa\x0B?V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x0B?V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x0B6W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x0B\x1DV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B]W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82a\x0B\x81WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9E5\x05\xD0\x10\xCDv$\x16\x8A\xD8\x8C\x9D\xD9\x98\xAE\xEB\xDFlo3p\xB6\xCA\xF6\x85\xA7\xED\xDB\xA9\nQdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static SIMPLEGOVERNANCE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80c\x12\x05z\x14\x14a\0ZW\x80c?\x8A\x03}\x14a\0\x7FW\x80cR\xEC\xB9\n\x14a\0\xA7W\x80c\x9A\xCA\x08\xD4\x14a\0\xC7W\x80c\xB6\xE7hs\x14a\0\xDCW\x80c\xC0\xC1\xCFU\x14a\x01\tW[`\0\x80\xFD[4\x80\x15a\0fW`\0\x80\xFD[Pb\x02\xA3\0[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x8BW`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0vV[4\x80\x15a\0\xB3W`\0\x80\xFD[Pa\0la\0\xC26`\x04a\x07\xEFV[a\x01)V[4\x80\x15a\0\xD3W`\0\x80\xFD[P`\x01Ta\0lV[4\x80\x15a\0\xE8W`\0\x80\xFD[Pa\0\xFCa\0\xF76`\x04a\x08\x99V[a\x03#V[`@Qa\0v\x91\x90a\x08\xF8V[a\x01\x1Ca\x01\x176`\x04a\x08\x99V[a\x04UV[`@Qa\0v\x91\x90a\t\\V[`\0a\x0143a\x05\x97V[a\x01XW`@Qc}\x89%u`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[0`\x01`\x01`\xA0\x1B\x03\x86\x16\x03a\x01\x81W`@QcAj\xEB\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15\x80\x15\x90a\x01\x98WP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15[\x15a\x01\xB6W`@Qcm\xD4\xAAe`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x90P`@Q\x80`\xA0\x01`@R\x80\x85`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP\x83\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x85Q\x81T\x92\x87\x01Q\x94\x87\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17`\x01`\x80\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16\x02\x17`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1B\x94\x90\x92\x16\x93\x90\x93\x02\x17\x82U``\x84\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\x80\x84\x01Q\x91\x92P\x82\x01\x90a\x02\xDB\x90\x82a\n\x15V[PP`\x01\x80T\x81\x01\x90UP`@Q\x81\x81R3\x90\x7FM\xFD\x92\xF6\x9E\x02\xF8,\x8Fg\x05\xB2\xE4\xA3dF[X\x8F\xA4w<\xDA&xl<\xA8\xDFC\xA1\x95\x90` \x01`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`\0\x82\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xC0\x1B\x90\x04\x90\x92\x16\x93\x82\x01\x93\x90\x93R`\x01\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R\x90\x82\x01\x80T\x91\x92\x91`\x80\x84\x01\x91\x90a\x03\xCC\x90a\t\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xF8\x90a\t\x8CV[\x80\x15a\x04EW\x80`\x1F\x10a\x04\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``a\x04`\x82a\x06\x95V[a\x04\x80W`@Qc\xB4R\xFA\xAF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x01OV[`\0\x82\x81R`\x02` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1BBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x81U\x91Q\x84\x81R3\x91\x7Fx\xA7\xD6Od\xEFh\x91\xBE\xDF\xC1\xB6\x85DG\x11<\xCA\x07\xAC\xE8t{\x8Al\xF3*\x0F\xD5\xA8\x0B\x17\x91\x01`@Q\x80\x91\x03\x90\xA2`\x01\x81\x01T\x81T`@Q`\0\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x90a\x05\x18\x90`\x02\x87\x01\x90a\n\xD5V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05UW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05ZV[``\x91P[P\x91P\x91P\x81a\x05\x8FW\x80Q\x15a\x05sW\x80Q\x81` \x01\xFD[`@Qc\xA6\xA7\xDB\xBD`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x01OV[\x94\x93PPPPV[`\0\x80T`@Qc\"\xC8\x8B'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x92\x16\x90cE\x91\x16N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x06\x91\x90a\x0BKV[\x90P`\0`\x02`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xECs\xFB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x81\x91\x90a\x0BKV[a\x06\x8B\x91\x90a\x0BdV[\x90\x91\x11\x93\x92PPPV[`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T`\x01`\x01`\x80\x1B\x03\x81\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\xC0\x1B\x90\x04\x90\x93\x16\x91\x83\x01\x91\x90\x91R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01R\x91\x82\x01\x80T\x84\x93\x91`\x80\x84\x01\x91a\x07\x0F\x90a\t\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07;\x90a\t\x8CV[\x80\x15a\x07\x88W\x80`\x1F\x10a\x07]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x07\xB2WP`\0\x92\x91PPV[`\0\x81` \x01QB\x03\x90P\x81`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14\x80\x15a\x05\x8FWPb\x02\xA3\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x94\x93PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x08\x05W`\0\x80\xFD[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x1CW`\0\x80\xFD[\x93P` \x85\x015`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x088W`\0\x80\xFD[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08UW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x08iW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08xW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x08\x8AW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xABW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x08\xD8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x08\xBCV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\x01`\x01`\x80\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R\x80`@\x86\x01Q\x16``\x85\x01RPP`\x01\x80`\xA0\x1B\x03``\x84\x01Q\x16`\x80\x83\x01R`\x80\x83\x01Q`\xA0\x80\x84\x01Ra\x05\x8F`\xC0\x84\x01\x82a\x08\xB2V[` \x81R`\0a\to` \x83\x01\x84a\x08\xB2V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\t\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\xC0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\n\x10W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\t\xEDWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\n\x0CW\x82\x81U`\x01\x01a\t\xF9V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n/Wa\n/a\tvV[a\nC\x81a\n=\x84Ta\t\x8CV[\x84a\t\xC6V[` \x80`\x1F\x83\x11`\x01\x81\x14a\nxW`\0\x84\x15a\n`WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\n\x0CV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\n\xA7W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\n\x88V[P\x85\x82\x10\x15a\n\xC5W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Ta\n\xE3\x81a\t\x8CV[`\x01\x82\x81\x16\x80\x15a\n\xFBW`\x01\x81\x14a\x0B\x10Wa\x0B?V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x0B?V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x0B6W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x0B\x1DV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B]W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82a\x0B\x81WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9E5\x05\xD0\x10\xCDv$\x16\x8A\xD8\x8C\x9D\xD9\x98\xAE\xEB\xDFlo3p\xB6\xCA\xF6\x85\xA7\xED\xDB\xA9\nQdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static SIMPLEGOVERNANCE_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SimpleGovernance<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleGovernance<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for SimpleGovernance<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for SimpleGovernance<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for SimpleGovernance<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SimpleGovernance))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleGovernance<M> {
        /// Creates a new contract instance with the
        /// specified `ethers` client at `address`.
        /// The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SIMPLEGOVERNANCE_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer`
        /// instance based on the provided constructor
        /// arguments and sends it. Returns a new
        /// instance of a deployer that returns an instance
        /// of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you
        ///   should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1
        ///   block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and
        /// deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi`
        /// object in the `greeter.json` artifact.
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
                SIMPLEGOVERNANCE_ABI.clone(),
                SIMPLEGOVERNANCE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `executeAction`
        /// (0xc0c1cf55) function
        pub fn execute_action(
            &self,
            action_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([192, 193, 207, 85], action_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAction` (0xb6e76873)
        /// function
        pub fn get_action(
            &self,
            action_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, GovernanceAction>
        {
            self.0
                .method_hash([182, 231, 104, 115], action_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionCounter`
        /// (0x9aca08d4) function
        pub fn get_action_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([154, 202, 8, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionDelay`
        /// (0x12057a14) function
        pub fn get_action_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([18, 5, 122, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGovernanceToken`
        /// (0x3f8a037d) function
        pub fn get_governance_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 138, 3, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queueAction` (0x52ecb90a)
        /// function
        pub fn queue_action(
            &self,
            target: ::ethers::core::types::Address,
            value: u128,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([82, 236, 185, 10], (target, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ActionExecuted` event
        pub fn action_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ActionExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ActionQueued` event
        pub fn action_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ActionQueuedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of
        /// this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SimpleGovernanceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for SimpleGovernance<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActionFailed` with signature
    /// `ActionFailed(uint256)` and selector `0xa6a7dbbd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ActionFailed", abi = "ActionFailed(uint256)")]
    pub struct ActionFailed {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `CannotExecute` with signature
    /// `CannotExecute(uint256)` and selector `0xb452faaf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CannotExecute", abi = "CannotExecute(uint256)")]
    pub struct CannotExecute {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidTarget` with signature
    /// `InvalidTarget()` and selector `0x82d5d76a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidTarget", abi = "InvalidTarget()")]
    pub struct InvalidTarget;
    ///Custom Error type `NotEnoughVotes` with signature
    /// `NotEnoughVotes(address)` and selector `0xfb124aea`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotEnoughVotes", abi = "NotEnoughVotes(address)")]
    pub struct NotEnoughVotes {
        pub who: ::ethers::core::types::Address,
    }
    ///Custom Error type `TargetMustHaveCode` with
    /// signature `TargetMustHaveCode()` and selector
    /// `0x6dd4aa65`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TargetMustHaveCode", abi = "TargetMustHaveCode()")]
    pub struct TargetMustHaveCode;
    ///Container type for all of the contract's custom
    /// errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum SimpleGovernanceErrors {
        ActionFailed(ActionFailed),
        CannotExecute(CannotExecute),
        InvalidTarget(InvalidTarget),
        NotEnoughVotes(NotEnoughVotes),
        TargetMustHaveCode(TargetMustHaveCode),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleGovernanceErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <ActionFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ActionFailed(decoded));
            }
            if let Ok(decoded) =
                <CannotExecute as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotExecute(decoded));
            }
            if let Ok(decoded) =
                <InvalidTarget as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidTarget(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughVotes as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotEnoughVotes(decoded));
            }
            if let Ok(decoded) =
                <TargetMustHaveCode as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TargetMustHaveCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleGovernanceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActionFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotExecute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTarget(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetMustHaveCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => {
                    ::ethers::core::abi::AbiEncode::encode(s)
                }
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SimpleGovernanceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ActionFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <CannotExecute as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTarget as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughVotes as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TargetMustHaveCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SimpleGovernanceErrors {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ActionFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotExecute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTarget(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughVotes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetMustHaveCode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SimpleGovernanceErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<ActionFailed> for SimpleGovernanceErrors {
        fn from(value: ActionFailed) -> Self { Self::ActionFailed(value) }
    }
    impl ::core::convert::From<CannotExecute> for SimpleGovernanceErrors {
        fn from(value: CannotExecute) -> Self { Self::CannotExecute(value) }
    }
    impl ::core::convert::From<InvalidTarget> for SimpleGovernanceErrors {
        fn from(value: InvalidTarget) -> Self { Self::InvalidTarget(value) }
    }
    impl ::core::convert::From<NotEnoughVotes> for SimpleGovernanceErrors {
        fn from(value: NotEnoughVotes) -> Self { Self::NotEnoughVotes(value) }
    }
    impl ::core::convert::From<TargetMustHaveCode> for SimpleGovernanceErrors {
        fn from(value: TargetMustHaveCode) -> Self {
            Self::TargetMustHaveCode(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ActionExecuted",
        abi = "ActionExecuted(uint256,address)"
    )]
    pub struct ActionExecutedFilter {
        pub action_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ActionQueued", abi = "ActionQueued(uint256,address)")]
    pub struct ActionQueuedFilter {
        pub action_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum SimpleGovernanceEvents {
        ActionExecutedFilter(ActionExecutedFilter),
        ActionQueuedFilter(ActionQueuedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SimpleGovernanceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ActionExecutedFilter::decode_log(log) {
                return Ok(SimpleGovernanceEvents::ActionExecutedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ActionQueuedFilter::decode_log(log) {
                return Ok(SimpleGovernanceEvents::ActionQueuedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SimpleGovernanceEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ActionExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ActionQueuedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ActionExecutedFilter> for SimpleGovernanceEvents {
        fn from(value: ActionExecutedFilter) -> Self {
            Self::ActionExecutedFilter(value)
        }
    }
    impl ::core::convert::From<ActionQueuedFilter> for SimpleGovernanceEvents {
        fn from(value: ActionQueuedFilter) -> Self {
            Self::ActionQueuedFilter(value)
        }
    }
    ///Container type for all input parameters for the
    /// `executeAction` function with signature
    /// `executeAction(uint256)` and selector `0xc0c1cf55`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "executeAction", abi = "executeAction(uint256)")]
    pub struct ExecuteActionCall {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `getAction` function with signature
    /// `getAction(uint256)` and selector `0xb6e76873`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAction", abi = "getAction(uint256)")]
    pub struct GetActionCall {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `getActionCounter` function with signature
    /// `getActionCounter()` and selector `0x9aca08d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getActionCounter", abi = "getActionCounter()")]
    pub struct GetActionCounterCall;
    ///Container type for all input parameters for the
    /// `getActionDelay` function with signature
    /// `getActionDelay()` and selector `0x12057a14`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getActionDelay", abi = "getActionDelay()")]
    pub struct GetActionDelayCall;
    ///Container type for all input parameters for the
    /// `getGovernanceToken` function with signature
    /// `getGovernanceToken()` and selector `0x3f8a037d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getGovernanceToken", abi = "getGovernanceToken()")]
    pub struct GetGovernanceTokenCall;
    ///Container type for all input parameters for the
    /// `queueAction` function with signature
    /// `queueAction(address,uint128,bytes)` and selector
    /// `0x52ecb90a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "queueAction", abi = "queueAction(address,uint128,bytes)")]
    pub struct QueueActionCall {
        pub target: ::ethers::core::types::Address,
        pub value: u128,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum SimpleGovernanceCalls {
        ExecuteAction(ExecuteActionCall),
        GetAction(GetActionCall),
        GetActionCounter(GetActionCounterCall),
        GetActionDelay(GetActionDelayCall),
        GetGovernanceToken(GetGovernanceTokenCall),
        QueueAction(QueueActionCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleGovernanceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ExecuteActionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ExecuteAction(decoded));
            }
            if let Ok(decoded) =
                <GetActionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAction(decoded));
            }
            if let Ok(decoded) =
                <GetActionCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetActionCounter(decoded));
            }
            if let Ok(decoded) =
                <GetActionDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetActionDelay(decoded));
            }
            if let Ok(decoded)
                = <GetGovernanceTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetGovernanceToken(decoded));
            }
            if let Ok(decoded) =
                <QueueActionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::QueueAction(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleGovernanceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExecuteAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActionCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActionDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGovernanceToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QueueAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SimpleGovernanceCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ExecuteAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetActionCounter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetActionDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGovernanceToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QueueAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ExecuteActionCall> for SimpleGovernanceCalls {
        fn from(value: ExecuteActionCall) -> Self { Self::ExecuteAction(value) }
    }
    impl ::core::convert::From<GetActionCall> for SimpleGovernanceCalls {
        fn from(value: GetActionCall) -> Self { Self::GetAction(value) }
    }
    impl ::core::convert::From<GetActionCounterCall> for SimpleGovernanceCalls {
        fn from(value: GetActionCounterCall) -> Self {
            Self::GetActionCounter(value)
        }
    }
    impl ::core::convert::From<GetActionDelayCall> for SimpleGovernanceCalls {
        fn from(value: GetActionDelayCall) -> Self {
            Self::GetActionDelay(value)
        }
    }
    impl ::core::convert::From<GetGovernanceTokenCall> for SimpleGovernanceCalls {
        fn from(value: GetGovernanceTokenCall) -> Self {
            Self::GetGovernanceToken(value)
        }
    }
    impl ::core::convert::From<QueueActionCall> for SimpleGovernanceCalls {
        fn from(value: QueueActionCall) -> Self { Self::QueueAction(value) }
    }
    ///Container type for all return fields from the
    /// `executeAction` function with signature
    /// `executeAction(uint256)` and selector `0xc0c1cf55`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExecuteActionReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the
    /// `getAction` function with signature
    /// `getAction(uint256)` and selector `0xb6e76873`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetActionReturn(pub GovernanceAction);
    ///Container type for all return fields from the
    /// `getActionCounter` function with signature
    /// `getActionCounter()` and selector `0x9aca08d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetActionCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getActionDelay` function with signature
    /// `getActionDelay()` and selector `0x12057a14`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetActionDelayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getGovernanceToken` function with signature
    /// `getGovernanceToken()` and selector `0x3f8a037d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetGovernanceTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `queueAction` function with signature
    /// `queueAction(address,uint128,bytes)` and selector
    /// `0x52ecb90a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct QueueActionReturn {
        pub action_id: ::ethers::core::types::U256,
    }
}
