pub use puppet_v2_pool::*;
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
pub mod puppet_v2_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("wethAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("uniswapPairAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("uniswapFactoryAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowAmount"),
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
                    ::std::borrow::ToOwned::to_owned("calculateDepositOfWETHRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDepositOfWETHRequired",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
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
                    ::std::borrow::ToOwned::to_owned("deposits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposits"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Borrowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Borrowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("depositRequired"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrowAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PUPPETV2POOL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\xD28\x03\x80a\x07\xD2\x839\x81\x81\x01`@R`\x80\x81\x10\x15a\x003W`\0\x80\xFD[P\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x02\x80T\x93\x85\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\0\x80T\x94\x84\x16\x94\x83\x16\x94\x90\x94\x17\x90\x93U`\x01\x80T\x92\x90\x93\x16\x91\x16\x17\x90Ua\x07/\x80a\0\xA3`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\xC4\xBD\x83\xFA\x14a\0FW\x80c\xC5\xEB\xEA\xEC\x14a\0uW\x80c\xFC~(m\x14a\0\x94W[`\0\x80\xFD[a\0c`\x04\x806\x03` \x81\x10\x15a\0\\W`\0\x80\xFD[P5a\0\xBAV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\x92`\x04\x806\x03` \x81\x10\x15a\0\x8BW`\0\x80\xFD[P5a\0\xEFV[\0[a\0c`\x04\x806\x03` \x81\x10\x15a\0\xAAW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x02\x9CV[`\0`\x03g\r\xE0\xB6\xB3\xA7d\0\0a\0\xE0\x82a\0\xD4\x86a\x02\xAEV[\x90c\xFF\xFF\xFF\xFFa\x03\x06\x16V[\x81a\0\xE7W\xFE[\x04\x93\x92PPPV[`\0a\0\xFA\x82a\0\xBAV[`\x03T`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R\x90Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a\x01WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01kW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x01\x81W`\0\x80\xFD[PP3`\0\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x80T\x87\x01\x90U`\x02T\x81Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x93\x84\x01\x95\x90\x95R`$\x83\x01\x87\x90RQ`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93c\xA9\x05\x9C\xBB\x93`D\x80\x85\x01\x94\x83\x90\x03\x01\x90\x82\x90\x87\x80;\x15\x80\x15a\x01\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02\x11W`\0\x80\xFD[PQa\x02VW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`\x8A\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q\x82\x81R` \x81\x01\x84\x90RB\x81\x83\x01R\x90Q3\x91\x7F\xC48\x1C\x9C{P\x89y\x89lU8\x1C\x1C\xB8\xC2\xAE\xF2\x8Du<\xFFva\xF9\xCD\x87\xB9\xA1E\x84\xF8\x91\x90\x81\x90\x03``\x01\x90\xA2PPV[`\x04` R`\0\x90\x81R`@\x90 T\x81V[`\x01T`\x03T`\x02T`\0\x92\x83\x92\x83\x92a\x02\xD7\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x81\x16\x91\x16a\x03oV[\x90\x92P\x90Pa\x02\xFEa\x02\xF7\x85g\r\xE0\xB6\xB3\xA7d\0\0c\xFF\xFF\xFF\xFFa\x03\x06\x16V[\x82\x84a\x04=V[\x94\x93PPPPV[`\0\x81\x15\x80a\x03!WPP\x80\x82\x02\x82\x82\x82\x81a\x03\x1EW\xFE[\x04\x14[a\x03iW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92\x91PPV[`\0\x80`\0a\x03~\x85\x85a\x04\xE9V[P\x90P`\0\x80a\x03\x8F\x88\x88\x88a\x05\xC7V[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\xC7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x03\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x03\xF1W`\0\x80\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14a\x04+W\x80\x82a\x04.V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x84\x11a\x04}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x06\xD5`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a\x04\x8DWP`\0\x82\x11[a\x04\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a\x06\xAD`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82a\x04\xD9\x85\x84c\xFF\xFF\xFF\xFFa\x03\x06\x16V[\x81a\x04\xE0W\xFE[\x04\x94\x93PPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x05=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x06\x88`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x05]W\x82\x84a\x05`V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05\xC0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0\x80`\0a\x05\xD6\x85\x85a\x04\xE9V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x96\xE8\xACBw\x19\x8F\xF8\xB6\xF7\x85G\x8A\xA9\xA3\x9F@<\xB7h\xDD\x02\xCB\xEE2l>}\xA3H\x84_`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV\xFEUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Library: INSUFFICIENT_LIQUIDITYUniswapV2Library: INSUFFICIENT_AMOUNT\xA2dipfsX\"\x12 \x95Nj\xECx\x8EkvbcW\x92F\xABv( \xB1p=4\x11\xCE,\\\xA6\xB6%\xD0\x84\xAAXdsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static PUPPETV2POOL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\xC4\xBD\x83\xFA\x14a\0FW\x80c\xC5\xEB\xEA\xEC\x14a\0uW\x80c\xFC~(m\x14a\0\x94W[`\0\x80\xFD[a\0c`\x04\x806\x03` \x81\x10\x15a\0\\W`\0\x80\xFD[P5a\0\xBAV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\x92`\x04\x806\x03` \x81\x10\x15a\0\x8BW`\0\x80\xFD[P5a\0\xEFV[\0[a\0c`\x04\x806\x03` \x81\x10\x15a\0\xAAW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x02\x9CV[`\0`\x03g\r\xE0\xB6\xB3\xA7d\0\0a\0\xE0\x82a\0\xD4\x86a\x02\xAEV[\x90c\xFF\xFF\xFF\xFFa\x03\x06\x16V[\x81a\0\xE7W\xFE[\x04\x93\x92PPPV[`\0a\0\xFA\x82a\0\xBAV[`\x03T`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R\x90Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a\x01WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01kW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x01\x81W`\0\x80\xFD[PP3`\0\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x80T\x87\x01\x90U`\x02T\x81Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x93\x84\x01\x95\x90\x95R`$\x83\x01\x87\x90RQ`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93c\xA9\x05\x9C\xBB\x93`D\x80\x85\x01\x94\x83\x90\x03\x01\x90\x82\x90\x87\x80;\x15\x80\x15a\x01\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02\x11W`\0\x80\xFD[PQa\x02VW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`\x8A\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q\x82\x81R` \x81\x01\x84\x90RB\x81\x83\x01R\x90Q3\x91\x7F\xC48\x1C\x9C{P\x89y\x89lU8\x1C\x1C\xB8\xC2\xAE\xF2\x8Du<\xFFva\xF9\xCD\x87\xB9\xA1E\x84\xF8\x91\x90\x81\x90\x03``\x01\x90\xA2PPV[`\x04` R`\0\x90\x81R`@\x90 T\x81V[`\x01T`\x03T`\x02T`\0\x92\x83\x92\x83\x92a\x02\xD7\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x81\x16\x91\x16a\x03oV[\x90\x92P\x90Pa\x02\xFEa\x02\xF7\x85g\r\xE0\xB6\xB3\xA7d\0\0c\xFF\xFF\xFF\xFFa\x03\x06\x16V[\x82\x84a\x04=V[\x94\x93PPPPV[`\0\x81\x15\x80a\x03!WPP\x80\x82\x02\x82\x82\x82\x81a\x03\x1EW\xFE[\x04\x14[a\x03iW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92\x91PPV[`\0\x80`\0a\x03~\x85\x85a\x04\xE9V[P\x90P`\0\x80a\x03\x8F\x88\x88\x88a\x05\xC7V[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\xC7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x03\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a\x03\xF1W`\0\x80\xFD[P\x80Q` \x90\x91\x01Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14a\x04+W\x80\x82a\x04.V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV[`\0\x80\x84\x11a\x04}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x06\xD5`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a\x04\x8DWP`\0\x82\x11[a\x04\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80a\x06\xAD`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82a\x04\xD9\x85\x84c\xFF\xFF\xFF\xFFa\x03\x06\x16V[\x81a\x04\xE0W\xFE[\x04\x94\x93PPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x05=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\x06\x88`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x05]W\x82\x84a\x05`V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05\xC0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`\0\x80`\0a\x05\xD6\x85\x85a\x04\xE9V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x96\xE8\xACBw\x19\x8F\xF8\xB6\xF7\x85G\x8A\xA9\xA3\x9F@<\xB7h\xDD\x02\xCB\xEE2l>}\xA3H\x84_`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV\xFEUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Library: INSUFFICIENT_LIQUIDITYUniswapV2Library: INSUFFICIENT_AMOUNT\xA2dipfsX\"\x12 \x95Nj\xECx\x8EkvbcW\x92F\xABv( \xB1p=4\x11\xCE,\\\xA6\xB6%\xD0\x84\xAAXdsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static PUPPETV2POOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PuppetV2Pool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PuppetV2Pool<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for PuppetV2Pool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for PuppetV2Pool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for PuppetV2Pool<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PuppetV2Pool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PuppetV2Pool<M> {
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
                PUPPETV2POOL_ABI.clone(),
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
                PUPPETV2POOL_ABI.clone(),
                PUPPETV2POOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `borrow` (0xc5ebeaec)
        /// function
        pub fn borrow(
            &self,
            borrow_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 235, 234, 236], borrow_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's
        /// `calculateDepositOfWETHRequired` (0xc4bd83fa)
        /// function
        pub fn calculate_deposit_of_weth_required(
            &self,
            token_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([196, 189, 131, 250], token_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposits` (0xfc7e286d)
        /// function
        pub fn deposits(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([252, 126, 40, 109], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Borrowed` event
        pub fn borrowed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BorrowedFilter,
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
            BorrowedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for PuppetV2Pool<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
        name = "Borrowed",
        abi = "Borrowed(address,uint256,uint256,uint256)"
    )]
    pub struct BorrowedFilter {
        #[ethevent(indexed)]
        pub borrower: ::ethers::core::types::Address,
        pub deposit_required: ::ethers::core::types::U256,
        pub borrow_amount: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `borrow` function with signature `borrow(uint256)`
    /// and selector `0xc5ebeaec`
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
    #[ethcall(name = "borrow", abi = "borrow(uint256)")]
    pub struct BorrowCall {
        pub borrow_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `calculateDepositOfWETHRequired` function with
    /// signature `calculateDepositOfWETHRequired(uint256)`
    /// and selector `0xc4bd83fa`
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
    #[ethcall(
        name = "calculateDepositOfWETHRequired",
        abi = "calculateDepositOfWETHRequired(uint256)"
    )]
    pub struct CalculateDepositOfWETHRequiredCall {
        pub token_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `deposits` function with signature
    /// `deposits(address)` and selector `0xfc7e286d`
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
    #[ethcall(name = "deposits", abi = "deposits(address)")]
    pub struct DepositsCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum PuppetV2PoolCalls {
        Borrow(BorrowCall),
        CalculateDepositOfWETHRequired(CalculateDepositOfWETHRequiredCall),
        Deposits(DepositsCall),
    }
    impl ::ethers::core::abi::AbiDecode for PuppetV2PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BorrowCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Borrow(decoded));
            }
            if let Ok(decoded)
                = <CalculateDepositOfWETHRequiredCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateDepositOfWETHRequired(decoded));
            }
            if let Ok(decoded) =
                <DepositsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Deposits(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PuppetV2PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Borrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDepositOfWETHRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PuppetV2PoolCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Borrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDepositOfWETHRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposits(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BorrowCall> for PuppetV2PoolCalls {
        fn from(value: BorrowCall) -> Self { Self::Borrow(value) }
    }
    impl ::core::convert::From<CalculateDepositOfWETHRequiredCall>
        for PuppetV2PoolCalls
    {
        fn from(value: CalculateDepositOfWETHRequiredCall) -> Self {
            Self::CalculateDepositOfWETHRequired(value)
        }
    }
    impl ::core::convert::From<DepositsCall> for PuppetV2PoolCalls {
        fn from(value: DepositsCall) -> Self { Self::Deposits(value) }
    }
    ///Container type for all return fields from the
    /// `calculateDepositOfWETHRequired` function with
    /// signature `calculateDepositOfWETHRequired(uint256)`
    /// and selector `0xc4bd83fa`
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
    pub struct CalculateDepositOfWETHRequiredReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the
    /// `deposits` function with signature
    /// `deposits(address)` and selector `0xfc7e286d`
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
    pub struct DepositsReturn(pub ::ethers::core::types::U256);
}
