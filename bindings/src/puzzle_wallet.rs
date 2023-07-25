pub use puzzle_wallet::*;
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
pub mod puzzle_wallet {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addToWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addToWhitelist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("balances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balances"),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxBalance"),
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
                    ::std::borrow::ToOwned::to_owned("maxBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxBalance"),
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
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("setMaxBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaxBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxBalance"),
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
                    ::std::borrow::ToOwned::to_owned("whitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("whitelisted"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PUZZLEWALLET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\t\xA7\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x91W`\x005`\xE0\x1C\x80c\xB6\x1D'\xF6\x11a\0YW\x80c\xB6\x1D'\xF6\x14a\x01YW\x80c\xB7\xB0B-\x14a\x01lW\x80c\xD0\xE3\r\xB0\x14a\x01\x8CW\x80c\xD96T~\x14a\x01\x94W\x80c\xE42R\xD7\x14a\x01\xD4W`\0\x80\xFD[\x80c'\xE25\xE3\x14a\0\x96W\x80cs\xADF\x8A\x14a\0\xD6W\x80c\x8D\xA5\xCB[\x14a\0\xECW\x80c\x9DQ\xD9\xB7\x14a\x01$W\x80c\xAC\x96P\xD8\x14a\x01FW[`\0\x80\xFD[4\x80\x15a\0\xA2W`\0\x80\xFD[Pa\0\xC3a\0\xB16`\x04a\x07BV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xE2W`\0\x80\xFD[Pa\0\xC3`\x01T\x81V[4\x80\x15a\0\xF8W`\0\x80\xFD[P`\0Ta\x01\x0C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCDV[4\x80\x15a\x010W`\0\x80\xFD[Pa\x01Da\x01?6`\x04a\x07dV[a\x01\xF4V[\0[a\x01Da\x01T6`\x04a\x07}V[a\x02\x7FV[a\x01Da\x01g6`\x04a\x07\xF2V[a\x04iV[4\x80\x15a\x01xW`\0\x80\xFD[Pa\x01Da\x01\x876`\x04a\x07dV[a\x05\xBEV[a\x01Da\x06\x1BV[4\x80\x15a\x01\xA0W`\0\x80\xFD[Pa\x01\xC4a\x01\xAF6`\x04a\x07BV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xCDV[4\x80\x15a\x01\xE0W`\0\x80\xFD[Pa\x01Da\x01\xEF6`\x04a\x07BV[a\x06\xB8V[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x02,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[`@Q\x80\x91\x03\x90\xFD[G\x15a\x02zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FContract balance is not 0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02#V[`\x01UV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x02\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[`\0\x80[\x82\x81\x10\x15a\x04cW`\0\x84\x84\x83\x81\x81\x10a\x02\xCEWa\x02\xCEa\x08\xA2V[\x90P` \x02\x81\x01\x90a\x02\xE0\x91\x90a\x08\xB8V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP` \x82\x01Q\x91\x92PPc\x02\xF1\xCF%`\xE4\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x03\x87W\x83\x15a\x03\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FDeposit can only be called once\0`D\x82\x01R`d\x01a\x02#V[`\x01\x93P[`\x000\x87\x87\x86\x81\x81\x10a\x03\x9CWa\x03\x9Ca\x08\xA2V[\x90P` \x02\x81\x01\x90a\x03\xAE\x91\x90a\x08\xB8V[`@Qa\x03\xBC\x92\x91\x90a\t\x06V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03\xF7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xFCV[``\x91P[PP\x90P\x80a\x04MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FError while delegating call\0\0\0\0\0`D\x82\x01R`d\x01a\x02#V[PPP\x80\x80a\x04[\x90a\t,V[\x91PPa\x02\xB2V[PPPPV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x04\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[3`\0\x90\x81R`\x03` R`@\x90 T\x83\x11\x15a\x04\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsInsufficient balance``\x1B`D\x82\x01R`d\x01a\x02#V[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x05\r\x90\x84\x90a\tEV[\x92PP\x81\x90UP`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84\x84\x84`@Qa\x051\x92\x91\x90a\t\x06V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05sV[``\x91P[PP\x90P\x80a\x05\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x11^\x19X\xDD]\x1A[\xDB\x88\x19\x98Z[\x19Y`\x82\x1B`D\x82\x01R`d\x01a\x02#V[PPPPPV[`\x01T\x15a\x06\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x02#V[`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90UV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x06JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[`\x01TG\x11\x15a\x06\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x13X^\x08\x18\x98[\x18[\x98\xD9H\x1C\x99XX\xDA\x19Y`j\x1B`D\x82\x01R`d\x01a\x02#V[3`\0\x90\x81R`\x03` R`@\x81 \x80T4\x92\x90a\x06\xB1\x90\x84\x90a\t^V[\x90\x91UPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl'7\xBA\x10:42\x907\xBB\xB72\xB9`\x99\x1B`D\x82\x01R`d\x01a\x02#V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07=W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07TW`\0\x80\xFD[a\x07]\x82a\x07&V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07vW`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x07\x90W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xA8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07\xBCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\xCBW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x07\xE0W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x08\x08W`\0\x80\xFD[a\x08\x11\x85a\x07&V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x085W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x08IW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08XW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x08jW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[` \x80\x82R`\x0F\x90\x82\x01Rn\x13\x9B\xDD\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x8A\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\xCFW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xEAW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xFFW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t>Wa\t>a\t\x16V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\tXWa\tXa\t\x16V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\tXWa\tXa\t\x16V\xFE\xA2dipfsX\"\x12 \x88\xBF\x08\x81\xB6\xDC\xF3\xA3G\xF3\x8B\xD6\xF1\xCA\xD3\xD5\xDE\x8DkD\x1E\xE2\xA1\xC7\xC0/L\x9B\xD4S7hdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static PUZZLEWALLET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x91W`\x005`\xE0\x1C\x80c\xB6\x1D'\xF6\x11a\0YW\x80c\xB6\x1D'\xF6\x14a\x01YW\x80c\xB7\xB0B-\x14a\x01lW\x80c\xD0\xE3\r\xB0\x14a\x01\x8CW\x80c\xD96T~\x14a\x01\x94W\x80c\xE42R\xD7\x14a\x01\xD4W`\0\x80\xFD[\x80c'\xE25\xE3\x14a\0\x96W\x80cs\xADF\x8A\x14a\0\xD6W\x80c\x8D\xA5\xCB[\x14a\0\xECW\x80c\x9DQ\xD9\xB7\x14a\x01$W\x80c\xAC\x96P\xD8\x14a\x01FW[`\0\x80\xFD[4\x80\x15a\0\xA2W`\0\x80\xFD[Pa\0\xC3a\0\xB16`\x04a\x07BV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xE2W`\0\x80\xFD[Pa\0\xC3`\x01T\x81V[4\x80\x15a\0\xF8W`\0\x80\xFD[P`\0Ta\x01\x0C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCDV[4\x80\x15a\x010W`\0\x80\xFD[Pa\x01Da\x01?6`\x04a\x07dV[a\x01\xF4V[\0[a\x01Da\x01T6`\x04a\x07}V[a\x02\x7FV[a\x01Da\x01g6`\x04a\x07\xF2V[a\x04iV[4\x80\x15a\x01xW`\0\x80\xFD[Pa\x01Da\x01\x876`\x04a\x07dV[a\x05\xBEV[a\x01Da\x06\x1BV[4\x80\x15a\x01\xA0W`\0\x80\xFD[Pa\x01\xC4a\x01\xAF6`\x04a\x07BV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xCDV[4\x80\x15a\x01\xE0W`\0\x80\xFD[Pa\x01Da\x01\xEF6`\x04a\x07BV[a\x06\xB8V[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x02,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[`@Q\x80\x91\x03\x90\xFD[G\x15a\x02zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FContract balance is not 0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02#V[`\x01UV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x02\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[`\0\x80[\x82\x81\x10\x15a\x04cW`\0\x84\x84\x83\x81\x81\x10a\x02\xCEWa\x02\xCEa\x08\xA2V[\x90P` \x02\x81\x01\x90a\x02\xE0\x91\x90a\x08\xB8V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP` \x82\x01Q\x91\x92PPc\x02\xF1\xCF%`\xE4\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x03\x87W\x83\x15a\x03\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FDeposit can only be called once\0`D\x82\x01R`d\x01a\x02#V[`\x01\x93P[`\x000\x87\x87\x86\x81\x81\x10a\x03\x9CWa\x03\x9Ca\x08\xA2V[\x90P` \x02\x81\x01\x90a\x03\xAE\x91\x90a\x08\xB8V[`@Qa\x03\xBC\x92\x91\x90a\t\x06V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03\xF7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xFCV[``\x91P[PP\x90P\x80a\x04MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FError while delegating call\0\0\0\0\0`D\x82\x01R`d\x01a\x02#V[PPP\x80\x80a\x04[\x90a\t,V[\x91PPa\x02\xB2V[PPPPV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x04\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[3`\0\x90\x81R`\x03` R`@\x90 T\x83\x11\x15a\x04\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsInsufficient balance``\x1B`D\x82\x01R`d\x01a\x02#V[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x05\r\x90\x84\x90a\tEV[\x92PP\x81\x90UP`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84\x84\x84`@Qa\x051\x92\x91\x90a\t\x06V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05sV[``\x91P[PP\x90P\x80a\x05\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x11^\x19X\xDD]\x1A[\xDB\x88\x19\x98Z[\x19Y`\x82\x1B`D\x82\x01R`d\x01a\x02#V[PPPPPV[`\x01T\x15a\x06\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x02#V[`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90UV[3`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x06JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02#\x90a\x08yV[`\x01TG\x11\x15a\x06\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x13X^\x08\x18\x98[\x18[\x98\xD9H\x1C\x99XX\xDA\x19Y`j\x1B`D\x82\x01R`d\x01a\x02#V[3`\0\x90\x81R`\x03` R`@\x81 \x80T4\x92\x90a\x06\xB1\x90\x84\x90a\t^V[\x90\x91UPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl'7\xBA\x10:42\x907\xBB\xB72\xB9`\x99\x1B`D\x82\x01R`d\x01a\x02#V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07=W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07TW`\0\x80\xFD[a\x07]\x82a\x07&V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07vW`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x07\x90W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xA8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07\xBCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\xCBW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x07\xE0W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x08\x08W`\0\x80\xFD[a\x08\x11\x85a\x07&V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x085W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x08IW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08XW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x08jW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[` \x80\x82R`\x0F\x90\x82\x01Rn\x13\x9B\xDD\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x8A\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\xCFW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xEAW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xFFW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t>Wa\t>a\t\x16V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\tXWa\tXa\t\x16V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\tXWa\tXa\t\x16V\xFE\xA2dipfsX\"\x12 \x88\xBF\x08\x81\xB6\xDC\xF3\xA3G\xF3\x8B\xD6\xF1\xCA\xD3\xD5\xDE\x8DkD\x1E\xE2\xA1\xC7\xC0/L\x9B\xD4S7hdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static PUZZLEWALLET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PuzzleWallet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PuzzleWallet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PuzzleWallet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PuzzleWallet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PuzzleWallet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PuzzleWallet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PuzzleWallet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PUZZLEWALLET_ABI.clone(),
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
                PUZZLEWALLET_ABI.clone(),
                PUZZLEWALLET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addToWhitelist` (0xe43252d7) function
        pub fn add_to_whitelist(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 50, 82, 215], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xd0e30db0) function
        pub fn deposit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0xb61d27f6) function
        pub fn execute(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 29, 39, 246], (to, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0xb7b0422d) function
        pub fn init(
            &self,
            max_balance: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 176, 66, 45], max_balance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxBalance` (0x73ad468a) function
        pub fn max_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([115, 173, 70, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxBalance` (0x9d51d9b7) function
        pub fn set_max_balance(
            &self,
            max_balance: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 81, 217, 183], max_balance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelisted` (0xd936547e) function
        pub fn whitelisted(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 54, 84, 126], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PuzzleWallet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addToWhitelist` function with signature `addToWhitelist(address)` and selector `0xe43252d7`
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
    #[ethcall(name = "addToWhitelist", abi = "addToWhitelist(address)")]
    pub struct AddToWhitelistCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `deposit` function with signature `deposit()` and selector `0xd0e30db0`
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
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(address,uint256,bytes)` and selector `0xb61d27f6`
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
    #[ethcall(name = "execute", abi = "execute(address,uint256,bytes)")]
    pub struct ExecuteCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `init` function with signature `init(uint256)` and selector `0xb7b0422d`
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
    #[ethcall(name = "init", abi = "init(uint256)")]
    pub struct InitCall {
        pub max_balance: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `maxBalance` function with signature `maxBalance()` and selector `0x73ad468a`
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
    #[ethcall(name = "maxBalance", abi = "maxBalance()")]
    pub struct MaxBalanceCall;
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `setMaxBalance` function with signature `setMaxBalance(uint256)` and selector `0x9d51d9b7`
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
    #[ethcall(name = "setMaxBalance", abi = "setMaxBalance(uint256)")]
    pub struct SetMaxBalanceCall {
        pub max_balance: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `whitelisted` function with signature `whitelisted(address)` and selector `0xd936547e`
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
    #[ethcall(name = "whitelisted", abi = "whitelisted(address)")]
    pub struct WhitelistedCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PuzzleWalletCalls {
        AddToWhitelist(AddToWhitelistCall),
        Balances(BalancesCall),
        Deposit(DepositCall),
        Execute(ExecuteCall),
        Init(InitCall),
        MaxBalance(MaxBalanceCall),
        Multicall(MulticallCall),
        Owner(OwnerCall),
        SetMaxBalance(SetMaxBalanceCall),
        Whitelisted(WhitelistedCall),
    }
    impl ::ethers::core::abi::AbiDecode for PuzzleWalletCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddToWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddToWhitelist(decoded));
            }
            if let Ok(decoded)
                = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <InitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded)
                = <MaxBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxBalance(decoded));
            }
            if let Ok(decoded)
                = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <SetMaxBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMaxBalance(decoded));
            }
            if let Ok(decoded)
                = <WhitelistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Whitelisted(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PuzzleWalletCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddToWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Balances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMaxBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Whitelisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PuzzleWalletCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddToWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaxBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Whitelisted(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddToWhitelistCall> for PuzzleWalletCalls {
        fn from(value: AddToWhitelistCall) -> Self {
            Self::AddToWhitelist(value)
        }
    }
    impl ::core::convert::From<BalancesCall> for PuzzleWalletCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<DepositCall> for PuzzleWalletCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for PuzzleWalletCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<InitCall> for PuzzleWalletCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<MaxBalanceCall> for PuzzleWalletCalls {
        fn from(value: MaxBalanceCall) -> Self {
            Self::MaxBalance(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for PuzzleWalletCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PuzzleWalletCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetMaxBalanceCall> for PuzzleWalletCalls {
        fn from(value: SetMaxBalanceCall) -> Self {
            Self::SetMaxBalance(value)
        }
    }
    impl ::core::convert::From<WhitelistedCall> for PuzzleWalletCalls {
        fn from(value: WhitelistedCall) -> Self {
            Self::Whitelisted(value)
        }
    }
    ///Container type for all return fields from the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    pub struct BalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxBalance` function with signature `maxBalance()` and selector `0x73ad468a`
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
    pub struct MaxBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `whitelisted` function with signature `whitelisted(address)` and selector `0xd936547e`
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
    pub struct WhitelistedReturn(pub bool);
}
