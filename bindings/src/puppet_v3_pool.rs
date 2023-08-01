pub use puppet_v3_pool::*;
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
pub mod puppet_v3_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_weth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC20Minimal"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC20Minimal"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_uniswapV3Pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IUniswapV3Pool"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEPOSIT_FACTOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEPOSIT_FACTOR"),
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
                    ::std::borrow::ToOwned::to_owned("TWAP_PERIOD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TWAP_PERIOD"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                (
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20Minimal"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3Pool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uniswapV3Pool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IUniswapV3Pool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weth"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20Minimal"),
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
                                    name: ::std::borrow::ToOwned::to_owned("depositAmount"),
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
    pub static PUPPETV3POOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0EL8\x03\x80a\x0EL\x839\x81\x81\x01`@R``\x81\x10\x15a\x003W`\0\x80\xFD[P\x80Q` \x82\x01Q`@\x90\x92\x01Q`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x92\x82\x1B\x83\x16`\xA0R\x90\x1B\x16`\xC0R`\x80Q``\x1C`\xA0Q``\x1C`\xC0Q``\x1Ca\r\x9Ca\0\xB0`\09\x80a\x02\xDER\x80a\x03TRP\x80a\x02xR\x80a\x03\x02R\x80a\x03\x85RP\x80a\x01`R\x80a\x01\xE8R\x80a\x03\xA6RPa\r\x9C`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\xC5\xEB\xEA\xEC\x11a\0[W\x80c\xC5\xEB\xEA\xEC\x14a\x01\tW\x80c\xF5^\xBD*\x14a\x01(W\x80c\xFC\x0CTj\x14a\x010W\x80c\xFC~(m\x14a\x018Wa\0\x88V[\x80c?\xC8\xCE\xF3\x14a\0\x8DW\x80c]H\xE2U\x14a\0\xB1W\x80c|\xA2Q\x84\x14a\0\xCBW\x80c\xC4\xBD\x83\xFA\x14a\0\xECW[`\0\x80\xFD[a\0\x95a\x01^V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xB9a\x01\x82V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xD3a\x01\x87V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xB9`\x04\x806\x03` \x81\x10\x15a\x01\x02W`\0\x80\xFD[P5a\x01\x8DV[a\x01&`\x04\x806\x03` \x81\x10\x15a\x01\x1FW`\0\x80\xFD[P5a\x01\xADV[\0[a\0\x95a\x02\xDCV[a\0\x95a\x03\0V[a\0\xB9`\x04\x806\x03` \x81\x10\x15a\x01NW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x03$V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x03\x81V[a\x02X\x81V[`\0\x80a\x01\xA1a\x01\x9C\x84a\x036V[a\x03LV[`\x03\x02\x91PP[\x91\x90PV[`\0a\x01\xB8\x82a\x01\x8DV[`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a\x021W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02EW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02[W`\0\x80\xFD[PP3`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x83\x01\x90Ua\x02\x9E\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x84a\x03\xD1V[`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x81Q3\x92\x7F\xEA\xE9\xCF\xBCw\xFD\xD4\x0C\xA8\x99\xF3k`\x82V\x06;+\xC9\xD8\x17\x8B\x02 \xF7\xADQ>\x17\x8Dg0\x92\x82\x90\x03\x01\x90\xA2PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[\x80`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x01\xA8W`\0\x80\xFD[`\0\x80a\x03{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02Xa\x05\x1FV[P\x90Pa\x03\xCA\x81\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x08\x8EV[\x93\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x04MW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x04.V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x04\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xB4V[``\x91P[P\x91P\x91P\x81\x80\x15a\x04\xE2WP\x80Q\x15\x80a\x04\xE2WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x04\xDFW`\0\x80\xFD[PQ[a\x05\x18W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra*#`\xF1\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0\x80c\xFF\xFF\xFF\xFF\x83\x16a\x05_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04%`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x05\x8EW\xFE[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPP`\0\x81`\x01\x81Q\x81\x10a\x05\xB7W\xFE[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`@Qc\x88;\xDB\xFD`\xE0\x1B\x81R`\x04\x81\x01\x82\x81R\x83Q`$\x83\x01R\x83Q`\0\x93\x84\x93`\x01`\x01`\xA0\x1B\x03\x8B\x16\x93c\x88;\xDB\xFD\x93\x88\x93\x91\x92\x83\x92`D\x90\x91\x01\x91\x85\x82\x01\x91\x02\x80\x83\x83\x8B[\x83\x81\x10\x15a\x06,W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\x14V[PPPP\x90P\x01\x92PPP`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06OW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x06cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@\x90\x81R\x81\x10\x15a\x06\x8CW`\0\x80\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\x06\xACW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x06\xC1W`\0\x80\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x06\xDEW`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x07\x0BW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xF3V[PPPP\x90P\x01`@R` \x01\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\x074W`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x07IW`\0\x80\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x07fW`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x07\x93W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07{V[PPPP\x90P\x01`@RPPP\x91P\x91P`\0\x82`\0\x81Q\x81\x10a\x07\xB3W\xFE[` \x02` \x01\x01Q\x83`\x01\x81Q\x81\x10a\x07\xC8W\xFE[` \x02` \x01\x01Q\x03\x90P`\0\x82`\0\x81Q\x81\x10a\x07\xE2W\xFE[` \x02` \x01\x01Q\x83`\x01\x81Q\x81\x10a\x07\xF7W\xFE[` \x02` \x01\x01Q\x03\x90P\x87c\xFF\xFF\xFF\xFF\x16\x82`\x06\x0B\x81a\x08\x14W\xFE[\x05\x96P`\0\x82`\x06\x0B\x12\x80\x15a\x08>WP\x87c\xFF\xFF\xFF\xFF\x16\x82`\x06\x0B\x81a\x087W\xFE[\x07`\x06\x0B\x15\x15[\x15a\x08KW`\0\x19\x90\x96\x01\x95[c\xFF\xFF\xFF\xFF\x88\x16`\x01`\x01`\xA0\x1B\x03\x02d\x01\0\0\0\0`\x01`\xC0\x1B\x03` \x83\x90\x1B\x16`\x01`\x01`\xC0\x1B\x03\x82\x16\x81a\x08~W\xFE[\x04\x96PPPPPPP\x92P\x92\x90PV[`\0\x80a\x08\x9A\x86a\t\x85V[\x90P`\x01`\x01`\x80\x1B\x03`\x01`\x01`\xA0\x1B\x03\x82\x16\x11a\t\tW`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x02\x90\x84\x81\x16\x90\x86\x16\x10a\x08\xE9Wa\x08\xE4`\x01`\xC0\x1B\x87`\x01`\x01`\x80\x1B\x03\x16\x83a\x0C\xB7V[a\t\x01V[a\t\x01\x81\x87`\x01`\x01`\x80\x1B\x03\x16`\x01`\xC0\x1Ba\x0C\xB7V[\x92PPa\t|V[`\0a\t(`\x01`\x01`\xA0\x1B\x03\x83\x16\x80h\x01\0\0\0\0\0\0\0\0a\x0C\xB7V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\t`Wa\t[`\x01`\x80\x1B\x87`\x01`\x01`\x80\x1B\x03\x16\x83a\x0C\xB7V[a\txV[a\tx\x81\x87`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba\x0C\xB7V[\x92PP[P\x94\x93PPPPV[`\0\x80`\0\x83`\x02\x0B\x12a\t\x9CW\x82`\x02\x0Ba\t\xA4V[\x82`\x02\x0B`\0\x03[\x90Pb\r\x89\xE8\x81\x11\x15a\t\xE2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x15`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x01\x82\x16a\t\xF6W`\x01`\x80\x1Ba\n\x08V[o\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02\x82\x16\x15a\n<Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a\n[Wo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a\nzWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a\n\x99Wo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a\n\xB8Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a\n\xD7Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a\n\xF6Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a\x0B\x16Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a\x0B6Wo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a\x0BVWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a\x0BvWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a\x0B\x96Wo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a\x0B\xB6Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a\x0B\xD6Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a\x0B\xF6Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a\x0C\x17Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a\x0C7Wn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a\x0CVWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a\x0CsWk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[`\0\x84`\x02\x0B\x13\x15a\x0C\x8EW\x80`\0\x19\x81a\x0C\x8AW\xFE[\x04\x90P[d\x01\0\0\0\0\x81\x06\x15a\x0C\xA2W`\x01a\x0C\xA5V[`\0[`\xFF\x16` \x82\x90\x1C\x01\x92PPP\x91\x90PV[`\0\x80\x80`\0\x19\x85\x87\t\x86\x86\x02\x92P\x82\x81\x10\x90\x83\x90\x03\x03\x90P\x80a\x0C\xEDW`\0\x84\x11a\x0C\xE2W`\0\x80\xFD[P\x82\x90\x04\x90Pa\x03\xCAV[\x80\x84\x11a\x0C\xF9W`\0\x80\xFD[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xA1\xC2\xFF\xC0\xE8\xC8c\x87\r%.\x0E\xFF\x1B\x18N\x13\xEB~\x84ik\xE8S\x946\xC9\x1E\xAB\xC1\xAA<dsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static PUPPETV3POOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\xC5\xEB\xEA\xEC\x11a\0[W\x80c\xC5\xEB\xEA\xEC\x14a\x01\tW\x80c\xF5^\xBD*\x14a\x01(W\x80c\xFC\x0CTj\x14a\x010W\x80c\xFC~(m\x14a\x018Wa\0\x88V[\x80c?\xC8\xCE\xF3\x14a\0\x8DW\x80c]H\xE2U\x14a\0\xB1W\x80c|\xA2Q\x84\x14a\0\xCBW\x80c\xC4\xBD\x83\xFA\x14a\0\xECW[`\0\x80\xFD[a\0\x95a\x01^V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xB9a\x01\x82V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xD3a\x01\x87V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xB9`\x04\x806\x03` \x81\x10\x15a\x01\x02W`\0\x80\xFD[P5a\x01\x8DV[a\x01&`\x04\x806\x03` \x81\x10\x15a\x01\x1FW`\0\x80\xFD[P5a\x01\xADV[\0[a\0\x95a\x02\xDCV[a\0\x95a\x03\0V[a\0\xB9`\x04\x806\x03` \x81\x10\x15a\x01NW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x03$V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x03\x81V[a\x02X\x81V[`\0\x80a\x01\xA1a\x01\x9C\x84a\x036V[a\x03LV[`\x03\x02\x91PP[\x91\x90PV[`\0a\x01\xB8\x82a\x01\x8DV[`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a\x021W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02EW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02[W`\0\x80\xFD[PP3`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x83\x01\x90Ua\x02\x9E\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x84a\x03\xD1V[`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x81Q3\x92\x7F\xEA\xE9\xCF\xBCw\xFD\xD4\x0C\xA8\x99\xF3k`\x82V\x06;+\xC9\xD8\x17\x8B\x02 \xF7\xADQ>\x17\x8Dg0\x92\x82\x90\x03\x01\x90\xA2PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[\x80`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x01\xA8W`\0\x80\xFD[`\0\x80a\x03{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02Xa\x05\x1FV[P\x90Pa\x03\xCA\x81\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x08\x8EV[\x93\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x04MW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x04.V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x04\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xB4V[``\x91P[P\x91P\x91P\x81\x80\x15a\x04\xE2WP\x80Q\x15\x80a\x04\xE2WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x04\xDFW`\0\x80\xFD[PQ[a\x05\x18W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra*#`\xF1\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0\x80c\xFF\xFF\xFF\xFF\x83\x16a\x05_W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04%`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x05\x8EW\xFE[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPP`\0\x81`\x01\x81Q\x81\x10a\x05\xB7W\xFE[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`@Qc\x88;\xDB\xFD`\xE0\x1B\x81R`\x04\x81\x01\x82\x81R\x83Q`$\x83\x01R\x83Q`\0\x93\x84\x93`\x01`\x01`\xA0\x1B\x03\x8B\x16\x93c\x88;\xDB\xFD\x93\x88\x93\x91\x92\x83\x92`D\x90\x91\x01\x91\x85\x82\x01\x91\x02\x80\x83\x83\x8B[\x83\x81\x10\x15a\x06,W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\x14V[PPPP\x90P\x01\x92PPP`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06OW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x06cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@\x90\x81R\x81\x10\x15a\x06\x8CW`\0\x80\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\x06\xACW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x06\xC1W`\0\x80\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x06\xDEW`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x07\x0BW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xF3V[PPPP\x90P\x01`@R` \x01\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\x074W`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x07IW`\0\x80\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x07fW`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x07\x93W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07{V[PPPP\x90P\x01`@RPPP\x91P\x91P`\0\x82`\0\x81Q\x81\x10a\x07\xB3W\xFE[` \x02` \x01\x01Q\x83`\x01\x81Q\x81\x10a\x07\xC8W\xFE[` \x02` \x01\x01Q\x03\x90P`\0\x82`\0\x81Q\x81\x10a\x07\xE2W\xFE[` \x02` \x01\x01Q\x83`\x01\x81Q\x81\x10a\x07\xF7W\xFE[` \x02` \x01\x01Q\x03\x90P\x87c\xFF\xFF\xFF\xFF\x16\x82`\x06\x0B\x81a\x08\x14W\xFE[\x05\x96P`\0\x82`\x06\x0B\x12\x80\x15a\x08>WP\x87c\xFF\xFF\xFF\xFF\x16\x82`\x06\x0B\x81a\x087W\xFE[\x07`\x06\x0B\x15\x15[\x15a\x08KW`\0\x19\x90\x96\x01\x95[c\xFF\xFF\xFF\xFF\x88\x16`\x01`\x01`\xA0\x1B\x03\x02d\x01\0\0\0\0`\x01`\xC0\x1B\x03` \x83\x90\x1B\x16`\x01`\x01`\xC0\x1B\x03\x82\x16\x81a\x08~W\xFE[\x04\x96PPPPPPP\x92P\x92\x90PV[`\0\x80a\x08\x9A\x86a\t\x85V[\x90P`\x01`\x01`\x80\x1B\x03`\x01`\x01`\xA0\x1B\x03\x82\x16\x11a\t\tW`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x02\x90\x84\x81\x16\x90\x86\x16\x10a\x08\xE9Wa\x08\xE4`\x01`\xC0\x1B\x87`\x01`\x01`\x80\x1B\x03\x16\x83a\x0C\xB7V[a\t\x01V[a\t\x01\x81\x87`\x01`\x01`\x80\x1B\x03\x16`\x01`\xC0\x1Ba\x0C\xB7V[\x92PPa\t|V[`\0a\t(`\x01`\x01`\xA0\x1B\x03\x83\x16\x80h\x01\0\0\0\0\0\0\0\0a\x0C\xB7V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\t`Wa\t[`\x01`\x80\x1B\x87`\x01`\x01`\x80\x1B\x03\x16\x83a\x0C\xB7V[a\txV[a\tx\x81\x87`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba\x0C\xB7V[\x92PP[P\x94\x93PPPPV[`\0\x80`\0\x83`\x02\x0B\x12a\t\x9CW\x82`\x02\x0Ba\t\xA4V[\x82`\x02\x0B`\0\x03[\x90Pb\r\x89\xE8\x81\x11\x15a\t\xE2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x15`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x01\x82\x16a\t\xF6W`\x01`\x80\x1Ba\n\x08V[o\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02\x82\x16\x15a\n<Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a\n[Wo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a\nzWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a\n\x99Wo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a\n\xB8Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a\n\xD7Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a\n\xF6Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a\x0B\x16Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a\x0B6Wo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a\x0BVWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a\x0BvWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a\x0B\x96Wo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a\x0B\xB6Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a\x0B\xD6Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a\x0B\xF6Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a\x0C\x17Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a\x0C7Wn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a\x0CVWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a\x0CsWk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[`\0\x84`\x02\x0B\x13\x15a\x0C\x8EW\x80`\0\x19\x81a\x0C\x8AW\xFE[\x04\x90P[d\x01\0\0\0\0\x81\x06\x15a\x0C\xA2W`\x01a\x0C\xA5V[`\0[`\xFF\x16` \x82\x90\x1C\x01\x92PPP\x91\x90PV[`\0\x80\x80`\0\x19\x85\x87\t\x86\x86\x02\x92P\x82\x81\x10\x90\x83\x90\x03\x03\x90P\x80a\x0C\xEDW`\0\x84\x11a\x0C\xE2W`\0\x80\xFD[P\x82\x90\x04\x90Pa\x03\xCAV[\x80\x84\x11a\x0C\xF9W`\0\x80\xFD[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xA1\xC2\xFF\xC0\xE8\xC8c\x87\r%.\x0E\xFF\x1B\x18N\x13\xEB~\x84ik\xE8S\x946\xC9\x1E\xAB\xC1\xAA<dsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static PUPPETV3POOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PuppetV3Pool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PuppetV3Pool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PuppetV3Pool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PuppetV3Pool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PuppetV3Pool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PuppetV3Pool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PuppetV3Pool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PUPPETV3POOL_ABI.clone(),
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
                PUPPETV3POOL_ABI.clone(),
                PUPPETV3POOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEPOSIT_FACTOR` (0x5d48e255) function
        pub fn deposit_factor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([93, 72, 226, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TWAP_PERIOD` (0x7ca25184) function
        pub fn twap_period(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([124, 162, 81, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrow` (0xc5ebeaec) function
        pub fn borrow(
            &self,
            borrow_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 235, 234, 236], borrow_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDepositOfWETHRequired` (0xc4bd83fa) function
        pub fn calculate_deposit_of_weth_required(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 189, 131, 250], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposits` (0xfc7e286d) function
        pub fn deposits(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([252, 126, 40, 109], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3Pool` (0xf55ebd2a) function
        pub fn uniswap_v3_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([245, 94, 189, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weth` (0x3fc8cef3) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 200, 206, 243], ())
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
        /// Returns an `Event` builder for all the events of this contract.
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PuppetV3Pool<M> {
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
        Hash
    )]
    #[ethevent(name = "Borrowed", abi = "Borrowed(address,uint256,uint256)")]
    pub struct BorrowedFilter {
        #[ethevent(indexed)]
        pub borrower: ::ethers::core::types::Address,
        pub deposit_amount: ::ethers::core::types::U256,
        pub borrow_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `DEPOSIT_FACTOR` function with signature `DEPOSIT_FACTOR()` and selector `0x5d48e255`
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
    #[ethcall(name = "DEPOSIT_FACTOR", abi = "DEPOSIT_FACTOR()")]
    pub struct DepositFactorCall;
    ///Container type for all input parameters for the `TWAP_PERIOD` function with signature `TWAP_PERIOD()` and selector `0x7ca25184`
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
    #[ethcall(name = "TWAP_PERIOD", abi = "TWAP_PERIOD()")]
    pub struct TwapPeriodCall;
    ///Container type for all input parameters for the `borrow` function with signature `borrow(uint256)` and selector `0xc5ebeaec`
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
    #[ethcall(name = "borrow", abi = "borrow(uint256)")]
    pub struct BorrowCall {
        pub borrow_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDepositOfWETHRequired` function with signature `calculateDepositOfWETHRequired(uint256)` and selector `0xc4bd83fa`
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
        name = "calculateDepositOfWETHRequired",
        abi = "calculateDepositOfWETHRequired(uint256)"
    )]
    pub struct CalculateDepositOfWETHRequiredCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deposits` function with signature `deposits(address)` and selector `0xfc7e286d`
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
    #[ethcall(name = "deposits", abi = "deposits(address)")]
    pub struct DepositsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `uniswapV3Pool` function with signature `uniswapV3Pool()` and selector `0xf55ebd2a`
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
    #[ethcall(name = "uniswapV3Pool", abi = "uniswapV3Pool()")]
    pub struct UniswapV3PoolCall;
    ///Container type for all input parameters for the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PuppetV3PoolCalls {
        DepositFactor(DepositFactorCall),
        TwapPeriod(TwapPeriodCall),
        Borrow(BorrowCall),
        CalculateDepositOfWETHRequired(CalculateDepositOfWETHRequiredCall),
        Deposits(DepositsCall),
        Token(TokenCall),
        UniswapV3Pool(UniswapV3PoolCall),
        Weth(WethCall),
    }
    impl ::ethers::core::abi::AbiDecode for PuppetV3PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DepositFactorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositFactor(decoded));
            }
            if let Ok(decoded)
                = <TwapPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TwapPeriod(decoded));
            }
            if let Ok(decoded)
                = <BorrowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Borrow(decoded));
            }
            if let Ok(decoded)
                = <CalculateDepositOfWETHRequiredCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateDepositOfWETHRequired(decoded));
            }
            if let Ok(decoded)
                = <DepositsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposits(decoded));
            }
            if let Ok(decoded)
                = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded)
                = <UniswapV3PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UniswapV3Pool(decoded));
            }
            if let Ok(decoded)
                = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PuppetV3PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TwapPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Borrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CalculateDepositOfWETHRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3Pool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PuppetV3PoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::TwapPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::Borrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDepositOfWETHRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositFactorCall> for PuppetV3PoolCalls {
        fn from(value: DepositFactorCall) -> Self {
            Self::DepositFactor(value)
        }
    }
    impl ::core::convert::From<TwapPeriodCall> for PuppetV3PoolCalls {
        fn from(value: TwapPeriodCall) -> Self {
            Self::TwapPeriod(value)
        }
    }
    impl ::core::convert::From<BorrowCall> for PuppetV3PoolCalls {
        fn from(value: BorrowCall) -> Self {
            Self::Borrow(value)
        }
    }
    impl ::core::convert::From<CalculateDepositOfWETHRequiredCall>
    for PuppetV3PoolCalls {
        fn from(value: CalculateDepositOfWETHRequiredCall) -> Self {
            Self::CalculateDepositOfWETHRequired(value)
        }
    }
    impl ::core::convert::From<DepositsCall> for PuppetV3PoolCalls {
        fn from(value: DepositsCall) -> Self {
            Self::Deposits(value)
        }
    }
    impl ::core::convert::From<TokenCall> for PuppetV3PoolCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<UniswapV3PoolCall> for PuppetV3PoolCalls {
        fn from(value: UniswapV3PoolCall) -> Self {
            Self::UniswapV3Pool(value)
        }
    }
    impl ::core::convert::From<WethCall> for PuppetV3PoolCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    ///Container type for all return fields from the `DEPOSIT_FACTOR` function with signature `DEPOSIT_FACTOR()` and selector `0x5d48e255`
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
    pub struct DepositFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `TWAP_PERIOD` function with signature `TWAP_PERIOD()` and selector `0x7ca25184`
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
    pub struct TwapPeriodReturn(pub u32);
    ///Container type for all return fields from the `calculateDepositOfWETHRequired` function with signature `calculateDepositOfWETHRequired(uint256)` and selector `0xc4bd83fa`
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
    pub struct CalculateDepositOfWETHRequiredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `deposits` function with signature `deposits(address)` and selector `0xfc7e286d`
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
    pub struct DepositsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `uniswapV3Pool` function with signature `uniswapV3Pool()` and selector `0xf55ebd2a`
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
    pub struct UniswapV3PoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
}
