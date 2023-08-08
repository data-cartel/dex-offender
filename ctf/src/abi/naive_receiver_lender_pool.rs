pub use naive_receiver_lender_pool::*;
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
pub mod naive_receiver_lender_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ETH"),
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
                    ::std::borrow::ToOwned::to_owned("flashFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("flashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IERC3156FlashBorrower",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxFlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxFlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallbackFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallbackFailed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RepayFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RepayFailed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsupportedCurrency"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnsupportedCurrency",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static NAIVERECEIVERLENDERPOOL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01`\0Ua\x04\xE9\x80a\0%`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80c\\\xFF\xE9\xDE\x14a\0OW\x80ca2U\xAB\x14a\0\x84W\x80c\x83\"\xFF\xF2\x14a\0\xB2W\x80c\xD9\xD9\x8C\xE4\x14a\0\xF2W`\0\x80\xFD[6a\0JW\0[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[Pa\0oa\0j6`\x04a\x03.V[a\x01\x12V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x90W`\0\x80\xFD[Pa\0\xA4a\0\x9F6`\x04a\x03\xCDV[a\x02pV[`@Q\x90\x81R` \x01a\0{V[4\x80\x15a\0\xBEW`\0\x80\xFD[Pa\0\xDAs\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0{V[4\x80\x15a\0\xFEW`\0\x80\xFD[Pa\0\xA4a\x01\r6`\x04a\x03\xF1V[a\x02\xA6V[`\0`\x01`\x01`\xA0\x1B\x03\x85\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\x01QW`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Ga\x01\\\x87\x86a\x02\xF6V[`@Qc#\xE3\x0C\x8B`\xE0\x1B\x81R\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xE3\x0C\x8B\x90a\x01\xD0\x903\x90s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x90\x8B\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x8C\x90\x8C\x90`\x04\x01a\x04\x1DV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x13\x91\x90a\x04yV[\x14a\x021W`@Qc\x04C\xEC+`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02Cg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x04\x92V[G\x10\x15a\x02cW`@Qc\x9Ep:\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x96\x95PPPPPPV[`\0s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xED\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\x02\x9EWPG\x91\x90PV[P`\0\x91\x90PV[`\0`\x01`\x01`\xA0\x1B\x03\x83\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\x02\xE5W`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0[\x92\x91PPV[`\0\x80`\0\x80\x84\x86Z\xF1a\x03\x12Wc\xB1-\x13\xEB`\0R`\x04`\x1C\xFD[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03+W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x03FW`\0\x80\xFD[\x855a\x03Q\x81a\x03\x16V[\x94P` \x86\x015a\x03a\x81a\x03\x16V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\x85W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x03\x99W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x03\xA8W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x03\xBAW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\xDFW`\0\x80\xFD[\x815a\x03\xEA\x81a\x03\x16V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\x04W`\0\x80\xFD[\x825a\x04\x0F\x81a\x03\x16V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`@\x81\x01\x85\x90R``\x81\x01\x84\x90R`\xA0`\x80\x82\x01\x81\x90R\x81\x01\x82\x90R`\0\x82\x84`\xC0\x84\x017`\0`\xC0\x84\x84\x01\x01R`\xC0`\x1F\x19`\x1F\x85\x01\x16\x83\x01\x01\x90P\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x04\x8BW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02\xF0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA1\xD5~\x1F\x83@\xC8M3\x01\x9C$\x90\x9E\x8Agt\xEF\xC4o\x0Bl\x96?\xC7\xA1\x87_2d\x81\xD2dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static NAIVERECEIVERLENDERPOOL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80c\\\xFF\xE9\xDE\x14a\0OW\x80ca2U\xAB\x14a\0\x84W\x80c\x83\"\xFF\xF2\x14a\0\xB2W\x80c\xD9\xD9\x8C\xE4\x14a\0\xF2W`\0\x80\xFD[6a\0JW\0[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[Pa\0oa\0j6`\x04a\x03.V[a\x01\x12V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x90W`\0\x80\xFD[Pa\0\xA4a\0\x9F6`\x04a\x03\xCDV[a\x02pV[`@Q\x90\x81R` \x01a\0{V[4\x80\x15a\0\xBEW`\0\x80\xFD[Pa\0\xDAs\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0{V[4\x80\x15a\0\xFEW`\0\x80\xFD[Pa\0\xA4a\x01\r6`\x04a\x03\xF1V[a\x02\xA6V[`\0`\x01`\x01`\xA0\x1B\x03\x85\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\x01QW`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Ga\x01\\\x87\x86a\x02\xF6V[`@Qc#\xE3\x0C\x8B`\xE0\x1B\x81R\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c#\xE3\x0C\x8B\x90a\x01\xD0\x903\x90s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x90\x8B\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x8C\x90\x8C\x90`\x04\x01a\x04\x1DV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x13\x91\x90a\x04yV[\x14a\x021W`@Qc\x04C\xEC+`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02Cg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x04\x92V[G\x10\x15a\x02cW`@Qc\x9Ep:\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x96\x95PPPPPPV[`\0s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xED\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\x02\x9EWPG\x91\x90PV[P`\0\x91\x90PV[`\0`\x01`\x01`\xA0\x1B\x03\x83\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\x02\xE5W`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0[\x92\x91PPV[`\0\x80`\0\x80\x84\x86Z\xF1a\x03\x12Wc\xB1-\x13\xEB`\0R`\x04`\x1C\xFD[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03+W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x03FW`\0\x80\xFD[\x855a\x03Q\x81a\x03\x16V[\x94P` \x86\x015a\x03a\x81a\x03\x16V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\x85W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x03\x99W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x03\xA8W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x03\xBAW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\xDFW`\0\x80\xFD[\x815a\x03\xEA\x81a\x03\x16V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\x04W`\0\x80\xFD[\x825a\x04\x0F\x81a\x03\x16V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`@\x81\x01\x85\x90R``\x81\x01\x84\x90R`\xA0`\x80\x82\x01\x81\x90R\x81\x01\x82\x90R`\0\x82\x84`\xC0\x84\x017`\0`\xC0\x84\x84\x01\x01R`\xC0`\x1F\x19`\x1F\x85\x01\x16\x83\x01\x01\x90P\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x04\x8BW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02\xF0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA1\xD5~\x1F\x83@\xC8M3\x01\x9C$\x90\x9E\x8Agt\xEF\xC4o\x0Bl\x96?\xC7\xA1\x87_2d\x81\xD2dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static NAIVERECEIVERLENDERPOOL_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct NaiveReceiverLenderPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NaiveReceiverLenderPool<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for NaiveReceiverLenderPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for NaiveReceiverLenderPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for NaiveReceiverLenderPool<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NaiveReceiverLenderPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NaiveReceiverLenderPool<M> {
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
                NAIVERECEIVERLENDERPOOL_ABI.clone(),
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
                NAIVERECEIVERLENDERPOOL_ABI.clone(),
                NAIVERECEIVERLENDERPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ETH` (0x8322fff2) function
        pub fn eth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([131, 34, 255, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashFee` (0xd9d98ce4)
        /// function
        pub fn flash_fee(
            &self,
            token: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([217, 217, 140, 228], (token, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoan` (0x5cffe9de)
        /// function
        pub fn flash_loan(
            &self,
            receiver: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [92, 255, 233, 222],
                    (receiver, token, amount, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxFlashLoan` (0x613255ab)
        /// function
        pub fn max_flash_loan(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([97, 50, 85, 171], token)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for NaiveReceiverLenderPool<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallbackFailed` with signature
    /// `CallbackFailed()` and selector `0x221f6158`
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
    #[etherror(name = "CallbackFailed", abi = "CallbackFailed()")]
    pub struct CallbackFailed;
    ///Custom Error type `RepayFailed` with signature
    /// `RepayFailed()` and selector `0x9e703a05`
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
    #[etherror(name = "RepayFailed", abi = "RepayFailed()")]
    pub struct RepayFailed;
    ///Custom Error type `UnsupportedCurrency` with
    /// signature `UnsupportedCurrency()` and selector
    /// `0x2263f4e2`
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
    #[etherror(name = "UnsupportedCurrency", abi = "UnsupportedCurrency()")]
    pub struct UnsupportedCurrency;
    ///Container type for all of the contract's custom
    /// errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum NaiveReceiverLenderPoolErrors {
        CallbackFailed(CallbackFailed),
        RepayFailed(RepayFailed),
        UnsupportedCurrency(UnsupportedCurrency),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for NaiveReceiverLenderPoolErrors {
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
                <CallbackFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CallbackFailed(decoded));
            }
            if let Ok(decoded) =
                <RepayFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RepayFailed(decoded));
            }
            if let Ok(decoded) =
                <UnsupportedCurrency as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UnsupportedCurrency(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NaiveReceiverLenderPoolErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallbackFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsupportedCurrency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => {
                    ::ethers::core::abi::AbiEncode::encode(s)
                }
            }
        }
    }
    impl ::ethers::contract::ContractRevert for NaiveReceiverLenderPoolErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallbackFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RepayFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnsupportedCurrency as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for NaiveReceiverLenderPoolErrors {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::CallbackFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsupportedCurrency(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
        for NaiveReceiverLenderPoolErrors
    {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<CallbackFailed> for NaiveReceiverLenderPoolErrors {
        fn from(value: CallbackFailed) -> Self { Self::CallbackFailed(value) }
    }
    impl ::core::convert::From<RepayFailed> for NaiveReceiverLenderPoolErrors {
        fn from(value: RepayFailed) -> Self { Self::RepayFailed(value) }
    }
    impl ::core::convert::From<UnsupportedCurrency>
        for NaiveReceiverLenderPoolErrors
    {
        fn from(value: UnsupportedCurrency) -> Self {
            Self::UnsupportedCurrency(value)
        }
    }
    ///Container type for all input parameters for the
    /// `ETH` function with signature `ETH()` and selector
    /// `0x8322fff2`
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
    #[ethcall(name = "ETH", abi = "ETH()")]
    pub struct EthCall;
    ///Container type for all input parameters for the
    /// `flashFee` function with signature
    /// `flashFee(address,uint256)` and selector
    /// `0xd9d98ce4`
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
    #[ethcall(name = "flashFee", abi = "flashFee(address,uint256)")]
    pub struct FlashFeeCall {
        pub token: ::ethers::core::types::Address,
        pub p1: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `flashLoan` function with signature
    /// `flashLoan(address,address,uint256,bytes)` and
    /// selector `0x5cffe9de`
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
        name = "flashLoan",
        abi = "flashLoan(address,address,uint256,bytes)"
    )]
    pub struct FlashLoanCall {
        pub receiver: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `maxFlashLoan` function with signature
    /// `maxFlashLoan(address)` and selector `0x613255ab`
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
    #[ethcall(name = "maxFlashLoan", abi = "maxFlashLoan(address)")]
    pub struct MaxFlashLoanCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum NaiveReceiverLenderPoolCalls {
        Eth(EthCall),
        FlashFee(FlashFeeCall),
        FlashLoan(FlashLoanCall),
        MaxFlashLoan(MaxFlashLoanCall),
    }
    impl ::ethers::core::abi::AbiDecode for NaiveReceiverLenderPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <EthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Eth(decoded));
            }
            if let Ok(decoded) =
                <FlashFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FlashFee(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <MaxFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::MaxFlashLoan(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NaiveReceiverLenderPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Eth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for NaiveReceiverLenderPoolCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Eth(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashLoan(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFlashLoan(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EthCall> for NaiveReceiverLenderPoolCalls {
        fn from(value: EthCall) -> Self { Self::Eth(value) }
    }
    impl ::core::convert::From<FlashFeeCall> for NaiveReceiverLenderPoolCalls {
        fn from(value: FlashFeeCall) -> Self { Self::FlashFee(value) }
    }
    impl ::core::convert::From<FlashLoanCall> for NaiveReceiverLenderPoolCalls {
        fn from(value: FlashLoanCall) -> Self { Self::FlashLoan(value) }
    }
    impl ::core::convert::From<MaxFlashLoanCall> for NaiveReceiverLenderPoolCalls {
        fn from(value: MaxFlashLoanCall) -> Self { Self::MaxFlashLoan(value) }
    }
    ///Container type for all return fields from the `ETH`
    /// function with signature `ETH()` and selector
    /// `0x8322fff2`
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
    pub struct EthReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `flashFee` function with signature
    /// `flashFee(address,uint256)` and selector
    /// `0xd9d98ce4`
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
    pub struct FlashFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `flashLoan` function with signature
    /// `flashLoan(address,address,uint256,bytes)` and
    /// selector `0x5cffe9de`
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
    pub struct FlashLoanReturn(pub bool);
    ///Container type for all return fields from the
    /// `maxFlashLoan` function with signature
    /// `maxFlashLoan(address)` and selector `0x613255ab`
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
    pub struct MaxFlashLoanReturn(pub ::ethers::core::types::U256);
}
