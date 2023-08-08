pub use selfie_pool::*;
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
pub mod selfie_pool {
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_governance"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("emergencyExit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emergencyExit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                    ::std::borrow::ToOwned::to_owned("flashFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                    name: ::std::borrow::ToOwned::to_owned("_receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IERC3156FlashBorrower",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
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
                    ::std::borrow::ToOwned::to_owned("governance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("governance"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract SimpleGovernance",
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
                    ::std::borrow::ToOwned::to_owned("maxFlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxFlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
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
                                        ::std::borrow::ToOwned::to_owned("contract ERC20Snapshot"),
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
                    ::std::borrow::ToOwned::to_owned("FundsDrained"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FundsDrained"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("CallerNotGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CallerNotGovernance",
                            ),
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SELFIEPOOL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t\xC48\x03\x80a\t\xC4\x839\x81\x01`@\x81\x90Ra\0/\x91a\0gV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x9AV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0bW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0zW`\0\x80\xFD[a\0\x83\x83a\0KV[\x91Pa\0\x91` \x84\x01a\0KV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x08\xC7a\0\xFD`\09`\0\x81\x81`l\x01Ra\x04\x90\x01R`\0\x81\x81a\x01\x1C\x01R\x81\x81a\x01J\x01R\x81\x81a\x01\xC0\x01R\x81\x81a\x03\x14\x01R\x81\x81a\x03\xC2\x01R\x81\x81a\x04\x04\x01R\x81\x81a\x04\xE6\x01R\x81\x81a\x05\x82\x01Ra\x06G\x01Ra\x08\xC7`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80cZ\xA6\xE6u\x14a\0gW\x80c\\\xFF\xE9\xDE\x14a\0\xABW\x80ca2U\xAB\x14a\0\xCEW\x80c\xA4A\xD0g\x14a\0\xEFW\x80c\xD9\xD9\x8C\xE4\x14a\x01\x04W\x80c\xFC\x0CTj\x14a\x01\x17W[`\0\x80\xFD[a\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBEa\0\xB96`\x04a\x07\x0BV[a\x01>V[`@Q\x90\x15\x15\x81R` \x01a\0\xA2V[a\0\xE1a\0\xDC6`\x04a\x07\xAAV[a\x03\xB4V[`@Q\x90\x81R` \x01a\0\xA2V[a\x01\x02a\0\xFD6`\x04a\x07\xAAV[a\x04\x85V[\0[a\0\xE1a\x01\x126`\x04a\x07\xCEV[a\x069V[a\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x01Ha\x06\x96V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01\x9AW`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02-\x91\x90a\x07\xFAV[P`@Qc#\xE3\x0C\x8B`\xE0\x1B\x81R\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c#\xE3\x0C\x8B\x90a\x02\x87\x903\x90\x8A\x90\x8A\x90`\0\x90\x8B\x90\x8B\x90`\x04\x01a\x08\x1CV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\x08xV[\x14a\x02\xE8W`@Qc\x04C\xEC+`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x86\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x81\x91\x90a\x07\xFAV[a\x03\x9EW`@Qc\x9Ep:\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01a\x03\xAB`\x01`\0UV[\x95\x94PPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x03a\x04}W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04w\x91\x90a\x08xV[\x92\x91PPV[P`\0\x91\x90PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xCEW`@Qc\xF2\xBE0\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x055W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05Y\x91\x90a\x08xV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF1\x91\x90a\x07\xFAV[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x01)\x9B\xD5\xAF\x84\xE2<L\x82\xC1\xD4\xED\xDF*\x9C\0\xE7\x0B\x8B\x88`v\xD0l\xDB\xE7\xC6\xFC\x80\xA8}\x82`@Qa\x06-\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x8DW`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\0\x92\x91PPV[`\x02`\0T\x03a\x06\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x07#W`\0\x80\xFD[\x855a\x07.\x81a\x06\xF3V[\x94P` \x86\x015a\x07>\x81a\x06\xF3V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07bW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x07vW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\x85W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x07\x97W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07\xBCW`\0\x80\xFD[\x815a\x07\xC7\x81a\x06\xF3V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xE1W`\0\x80\xFD[\x825a\x07\xEC\x81a\x06\xF3V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x0CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xC7W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`@\x81\x01\x85\x90R``\x81\x01\x84\x90R`\xA0`\x80\x82\x01\x81\x90R\x81\x01\x82\x90R`\0\x82\x84`\xC0\x84\x017`\0`\xC0\x84\x84\x01\x01R`\xC0`\x1F\x19`\x1F\x85\x01\x16\x83\x01\x01\x90P\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x8AW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \xE7\x93\xBE\xF9e\xF2D}\xCEY\xFA\xE5\xEAz&\xD5\xCBqB\x81\xA3\xFB9\x8E\x8Dk`\xE2\xAC\xE6\xDE\xBBdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static SELFIEPOOL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80cZ\xA6\xE6u\x14a\0gW\x80c\\\xFF\xE9\xDE\x14a\0\xABW\x80ca2U\xAB\x14a\0\xCEW\x80c\xA4A\xD0g\x14a\0\xEFW\x80c\xD9\xD9\x8C\xE4\x14a\x01\x04W\x80c\xFC\x0CTj\x14a\x01\x17W[`\0\x80\xFD[a\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBEa\0\xB96`\x04a\x07\x0BV[a\x01>V[`@Q\x90\x15\x15\x81R` \x01a\0\xA2V[a\0\xE1a\0\xDC6`\x04a\x07\xAAV[a\x03\xB4V[`@Q\x90\x81R` \x01a\0\xA2V[a\x01\x02a\0\xFD6`\x04a\x07\xAAV[a\x04\x85V[\0[a\0\xE1a\x01\x126`\x04a\x07\xCEV[a\x069V[a\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x01Ha\x06\x96V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01\x9AW`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02-\x91\x90a\x07\xFAV[P`@Qc#\xE3\x0C\x8B`\xE0\x1B\x81R\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c#\xE3\x0C\x8B\x90a\x02\x87\x903\x90\x8A\x90\x8A\x90`\0\x90\x8B\x90\x8B\x90`\x04\x01a\x08\x1CV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\x08xV[\x14a\x02\xE8W`@Qc\x04C\xEC+`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x86\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x81\x91\x90a\x07\xFAV[a\x03\x9EW`@Qc\x9Ep:\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01a\x03\xAB`\x01`\0UV[\x95\x94PPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x03a\x04}W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04w\x91\x90a\x08xV[\x92\x91PPV[P`\0\x91\x90PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xCEW`@Qc\xF2\xBE0\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x055W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05Y\x91\x90a\x08xV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF1\x91\x90a\x07\xFAV[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x01)\x9B\xD5\xAF\x84\xE2<L\x82\xC1\xD4\xED\xDF*\x9C\0\xE7\x0B\x8B\x88`v\xD0l\xDB\xE7\xC6\xFC\x80\xA8}\x82`@Qa\x06-\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x8DW`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\0\x92\x91PPV[`\x02`\0T\x03a\x06\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x07#W`\0\x80\xFD[\x855a\x07.\x81a\x06\xF3V[\x94P` \x86\x015a\x07>\x81a\x06\xF3V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07bW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x07vW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\x85W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x07\x97W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07\xBCW`\0\x80\xFD[\x815a\x07\xC7\x81a\x06\xF3V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xE1W`\0\x80\xFD[\x825a\x07\xEC\x81a\x06\xF3V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x0CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xC7W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`@\x81\x01\x85\x90R``\x81\x01\x84\x90R`\xA0`\x80\x82\x01\x81\x90R\x81\x01\x82\x90R`\0\x82\x84`\xC0\x84\x017`\0`\xC0\x84\x84\x01\x01R`\xC0`\x1F\x19`\x1F\x85\x01\x16\x83\x01\x01\x90P\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x8AW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \xE7\x93\xBE\xF9e\xF2D}\xCEY\xFA\xE5\xEAz&\xD5\xCBqB\x81\xA3\xFB9\x8E\x8Dk`\xE2\xAC\xE6\xDE\xBBdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static SELFIEPOOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SelfiePool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SelfiePool<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for SelfiePool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for SelfiePool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for SelfiePool<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SelfiePool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SelfiePool<M> {
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
                SELFIEPOOL_ABI.clone(),
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
                SELFIEPOOL_ABI.clone(),
                SELFIEPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `emergencyExit`
        /// (0xa441d067) function
        pub fn emergency_exit(
            &self,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 65, 208, 103], receiver)
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
        ///Calls the contract's `governance` (0x5aa6e675)
        /// function
        pub fn governance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([90, 166, 230, 117], ())
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
        ///Calls the contract's `token` (0xfc0c546a)
        /// function
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
        ///Gets the contract's `FundsDrained` event
        pub fn funds_drained_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FundsDrainedFilter,
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
            FundsDrainedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for SelfiePool<M>
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
    ///Custom Error type `CallerNotGovernance` with
    /// signature `CallerNotGovernance()` and selector
    /// `0xf2be30fb`
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
    #[etherror(name = "CallerNotGovernance", abi = "CallerNotGovernance()")]
    pub struct CallerNotGovernance;
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
    pub enum SelfiePoolErrors {
        CallbackFailed(CallbackFailed),
        CallerNotGovernance(CallerNotGovernance),
        RepayFailed(RepayFailed),
        UnsupportedCurrency(UnsupportedCurrency),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SelfiePoolErrors {
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
                <CallerNotGovernance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CallerNotGovernance(decoded));
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
    impl ::ethers::core::abi::AbiEncode for SelfiePoolErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallbackFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotGovernance(element) => {
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
    impl ::ethers::contract::ContractRevert for SelfiePoolErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallbackFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CallerNotGovernance as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for SelfiePoolErrors {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::CallbackFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotGovernance(element) => {
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
    impl ::core::convert::From<::std::string::String> for SelfiePoolErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<CallbackFailed> for SelfiePoolErrors {
        fn from(value: CallbackFailed) -> Self { Self::CallbackFailed(value) }
    }
    impl ::core::convert::From<CallerNotGovernance> for SelfiePoolErrors {
        fn from(value: CallerNotGovernance) -> Self {
            Self::CallerNotGovernance(value)
        }
    }
    impl ::core::convert::From<RepayFailed> for SelfiePoolErrors {
        fn from(value: RepayFailed) -> Self { Self::RepayFailed(value) }
    }
    impl ::core::convert::From<UnsupportedCurrency> for SelfiePoolErrors {
        fn from(value: UnsupportedCurrency) -> Self {
            Self::UnsupportedCurrency(value)
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
    #[ethevent(name = "FundsDrained", abi = "FundsDrained(address,uint256)")]
    pub struct FundsDrainedFilter {
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `emergencyExit` function with signature
    /// `emergencyExit(address)` and selector `0xa441d067`
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
    #[ethcall(name = "emergencyExit", abi = "emergencyExit(address)")]
    pub struct EmergencyExitCall {
        pub receiver: ::ethers::core::types::Address,
    }
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
    /// `governance` function with signature `governance()`
    /// and selector `0x5aa6e675`
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
    #[ethcall(name = "governance", abi = "governance()")]
    pub struct GovernanceCall;
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
    ///Container type for all input parameters for the
    /// `token` function with signature `token()` and
    /// selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum SelfiePoolCalls {
        EmergencyExit(EmergencyExitCall),
        FlashFee(FlashFeeCall),
        FlashLoan(FlashLoanCall),
        Governance(GovernanceCall),
        MaxFlashLoan(MaxFlashLoanCall),
        Token(TokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for SelfiePoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <EmergencyExitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::EmergencyExit(decoded));
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
                <GovernanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Governance(decoded));
            }
            if let Ok(decoded) =
                <MaxFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::MaxFlashLoan(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Token(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SelfiePoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EmergencyExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Governance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SelfiePoolCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::EmergencyExit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashLoan(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Governance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFlashLoan(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EmergencyExitCall> for SelfiePoolCalls {
        fn from(value: EmergencyExitCall) -> Self { Self::EmergencyExit(value) }
    }
    impl ::core::convert::From<FlashFeeCall> for SelfiePoolCalls {
        fn from(value: FlashFeeCall) -> Self { Self::FlashFee(value) }
    }
    impl ::core::convert::From<FlashLoanCall> for SelfiePoolCalls {
        fn from(value: FlashLoanCall) -> Self { Self::FlashLoan(value) }
    }
    impl ::core::convert::From<GovernanceCall> for SelfiePoolCalls {
        fn from(value: GovernanceCall) -> Self { Self::Governance(value) }
    }
    impl ::core::convert::From<MaxFlashLoanCall> for SelfiePoolCalls {
        fn from(value: MaxFlashLoanCall) -> Self { Self::MaxFlashLoan(value) }
    }
    impl ::core::convert::From<TokenCall> for SelfiePoolCalls {
        fn from(value: TokenCall) -> Self { Self::Token(value) }
    }
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
    /// `governance` function with signature `governance()`
    /// and selector `0x5aa6e675`
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
    pub struct GovernanceReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the
    /// `token` function with signature `token()` and
    /// selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
}
