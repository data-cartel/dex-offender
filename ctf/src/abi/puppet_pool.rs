pub use puppet_pool::*;
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
pub mod puppet_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrow"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("calculateDepositRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDepositRequired",
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
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract DamnValuableToken",
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
                    ::std::borrow::ToOwned::to_owned("uniswapPair"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uniswapPair"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Borrowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Borrowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotEnoughCollateral",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TransferFailed"),
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
    pub static PUPPETPOOL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\x928\x03\x80a\x07\x92\x839\x81\x01`@\x81\x90Ra\0/\x91a\0gV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\x80Ra\0\x9AV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0bW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0zW`\0\x80\xFD[a\0\x83\x83a\0KV[\x91Pa\0\x91` \x84\x01a\0KV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x06\xB8a\0\xDA`\09`\0\x81\x81a\x01\x15\x01R\x81\x81a\x01\xE9\x01Ra\x04\xC5\x01R`\0\x81\x81`\xC9\x01R\x81\x81a\x04\x9A\x01Ra\x05?\x01Ra\x06\xB8`\0\xF3\xFE`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80cK?\xD1H\x14a\0ZW\x80c]H\xE2U\x14a\0oW\x80c\xBCUL(\x14a\0\x97W\x80c\xC8\x16\x84\x1B\x14a\0\xB7W\x80c\xFC\x0CTj\x14a\x01\x03W\x80c\xFC~(m\x14a\x017W[`\0\x80\xFD[a\0ma\0h6`\x04a\x05\x99V[a\x01dV[\0[4\x80\x15a\0{W`\0\x80\xFD[Pa\0\x84`\x02\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xA3W`\0\x80\xFD[Pa\0\x84a\0\xB26`\x04a\x05\xC5V[a\x02\xCEV[4\x80\x15a\0\xC3W`\0\x80\xFD[Pa\0\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x8EV[4\x80\x15a\x01\x0FW`\0\x80\xFD[Pa\0\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01CW`\0\x80\xFD[Pa\0\x84a\x01R6`\x04a\x05\xDEV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01la\x03\x07V[`\0a\x01w\x83a\x02\xCEV[\x90P\x804\x10\x15a\x01\x99W`@Qb\xD1\x1D\xF3`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x804\x11\x15a\x01\xAEWa\x01\xAE34\x83\x90\x03a\x03eV[3`\0\x90\x81R`\x01` R`@\x90\x81\x90 \x80T\x83\x01\x90UQc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x022W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02V\x91\x90a\x06\0V[a\x02sW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R3\x90\x7F\xC1N\x9EkM\x98\xA5B\xB0Z\x0Fkd\xAC\xD5\xF4\xCB\xDB\x91K\xE8\x842\xD8t\x86\xB6d\xE9pq\xA7\x90``\x01`@Q\x80\x91\x03\x90\xA2Pa\x02\xCA`\x01`\0UV[PPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02a\x02\xE3a\x04\x83V[a\x02\xED\x90\x85a\x06\"V[a\x02\xF7\x91\x90a\x06\"V[a\x03\x01\x91\x90a\x06GV[\x92\x91PPV[`\x02`\0T\x03a\x03^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[\x80G\x10\x15a\x03\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x03UV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\x02W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\x07V[``\x91P[PP\x90P\x80a\x04~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03UV[PPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x052\x91\x90a\x06iV[a\x05n`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x161g\r\xE0\xB6\xB3\xA7d\0\0a\x06\"V[a\x05x\x91\x90a\x06GV[\x90P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x94W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\xACW`\0\x80\xFD[\x825\x91Pa\x05\xBC` \x84\x01a\x05}V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xD7W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xF0W`\0\x80\xFD[a\x05\xF9\x82a\x05}V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x12W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\xF9W`\0\x80\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x01WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x06dWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x06{W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \xABlb\x93J\xBF\xCB\xEA\xA9'\xB4\xF7\x95>\xB5\x8E\\\xCB\xAA\xF3C\x980[\x19YOr\x16\xDC\xBD\xD0dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static PUPPETPOOL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80cK?\xD1H\x14a\0ZW\x80c]H\xE2U\x14a\0oW\x80c\xBCUL(\x14a\0\x97W\x80c\xC8\x16\x84\x1B\x14a\0\xB7W\x80c\xFC\x0CTj\x14a\x01\x03W\x80c\xFC~(m\x14a\x017W[`\0\x80\xFD[a\0ma\0h6`\x04a\x05\x99V[a\x01dV[\0[4\x80\x15a\0{W`\0\x80\xFD[Pa\0\x84`\x02\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xA3W`\0\x80\xFD[Pa\0\x84a\0\xB26`\x04a\x05\xC5V[a\x02\xCEV[4\x80\x15a\0\xC3W`\0\x80\xFD[Pa\0\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x8EV[4\x80\x15a\x01\x0FW`\0\x80\xFD[Pa\0\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01CW`\0\x80\xFD[Pa\0\x84a\x01R6`\x04a\x05\xDEV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01la\x03\x07V[`\0a\x01w\x83a\x02\xCEV[\x90P\x804\x10\x15a\x01\x99W`@Qb\xD1\x1D\xF3`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x804\x11\x15a\x01\xAEWa\x01\xAE34\x83\x90\x03a\x03eV[3`\0\x90\x81R`\x01` R`@\x90\x81\x90 \x80T\x83\x01\x90UQc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x022W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02V\x91\x90a\x06\0V[a\x02sW`@Qc\x12\x17\x1D\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R3\x90\x7F\xC1N\x9EkM\x98\xA5B\xB0Z\x0Fkd\xAC\xD5\xF4\xCB\xDB\x91K\xE8\x842\xD8t\x86\xB6d\xE9pq\xA7\x90``\x01`@Q\x80\x91\x03\x90\xA2Pa\x02\xCA`\x01`\0UV[PPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x02a\x02\xE3a\x04\x83V[a\x02\xED\x90\x85a\x06\"V[a\x02\xF7\x91\x90a\x06\"V[a\x03\x01\x91\x90a\x06GV[\x92\x91PPV[`\x02`\0T\x03a\x03^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[\x80G\x10\x15a\x03\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x03UV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\x02W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\x07V[``\x91P[PP\x90P\x80a\x04~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03UV[PPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x052\x91\x90a\x06iV[a\x05n`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x161g\r\xE0\xB6\xB3\xA7d\0\0a\x06\"V[a\x05x\x91\x90a\x06GV[\x90P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x94W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\xACW`\0\x80\xFD[\x825\x91Pa\x05\xBC` \x84\x01a\x05}V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xD7W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xF0W`\0\x80\xFD[a\x05\xF9\x82a\x05}V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x12W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\xF9W`\0\x80\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x01WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x06dWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x06{W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \xABlb\x93J\xBF\xCB\xEA\xA9'\xB4\xF7\x95>\xB5\x8E\\\xCB\xAA\xF3C\x980[\x19YOr\x16\xDC\xBD\xD0dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static PUPPETPOOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PuppetPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PuppetPool<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for PuppetPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for PuppetPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for PuppetPool<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PuppetPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PuppetPool<M> {
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
                PUPPETPOOL_ABI.clone(),
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
                PUPPETPOOL_ABI.clone(),
                PUPPETPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEPOSIT_FACTOR`
        /// (0x5d48e255) function
        pub fn deposit_factor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([93, 72, 226, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrow` (0x4b3fd148)
        /// function
        pub fn borrow(
            &self,
            amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 63, 209, 72], (amount, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDepositRequired`
        /// (0xbc554c28) function
        pub fn calculate_deposit_required(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([188, 85, 76, 40], amount)
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
        ///Calls the contract's `uniswapPair` (0xc816841b)
        /// function
        pub fn uniswap_pair(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 22, 132, 27], ())
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
        From<::ethers::contract::Contract<M>> for PuppetPool<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotEnoughCollateral` with
    /// signature `NotEnoughCollateral()` and selector
    /// `0x34477cc0`
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
    #[etherror(name = "NotEnoughCollateral", abi = "NotEnoughCollateral()")]
    pub struct NotEnoughCollateral;
    ///Custom Error type `TransferFailed` with signature
    /// `TransferFailed()` and selector `0x90b8ec18`
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
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
    ///Container type for all of the contract's custom
    /// errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum PuppetPoolErrors {
        NotEnoughCollateral(NotEnoughCollateral),
        TransferFailed(TransferFailed),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PuppetPoolErrors {
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
                <NotEnoughCollateral as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NotEnoughCollateral(decoded));
            }
            if let Ok(decoded) =
                <TransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PuppetPoolErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NotEnoughCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => {
                    ::ethers::core::abi::AbiEncode::encode(s)
                }
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PuppetPoolErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <NotEnoughCollateral as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PuppetPoolErrors {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::NotEnoughCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PuppetPoolErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<NotEnoughCollateral> for PuppetPoolErrors {
        fn from(value: NotEnoughCollateral) -> Self {
            Self::NotEnoughCollateral(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for PuppetPoolErrors {
        fn from(value: TransferFailed) -> Self { Self::TransferFailed(value) }
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
        abi = "Borrowed(address,address,uint256,uint256)"
    )]
    pub struct BorrowedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub deposit_required: ::ethers::core::types::U256,
        pub borrow_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `DEPOSIT_FACTOR` function with signature
    /// `DEPOSIT_FACTOR()` and selector `0x5d48e255`
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
    #[ethcall(name = "DEPOSIT_FACTOR", abi = "DEPOSIT_FACTOR()")]
    pub struct DepositFactorCall;
    ///Container type for all input parameters for the
    /// `borrow` function with signature
    /// `borrow(uint256,address)` and selector `0x4b3fd148`
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
    #[ethcall(name = "borrow", abi = "borrow(uint256,address)")]
    pub struct BorrowCall {
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `calculateDepositRequired` function with signature
    /// `calculateDepositRequired(uint256)` and selector
    /// `0xbc554c28`
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
        name = "calculateDepositRequired",
        abi = "calculateDepositRequired(uint256)"
    )]
    pub struct CalculateDepositRequiredCall {
        pub amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the
    /// `uniswapPair` function with signature
    /// `uniswapPair()` and selector `0xc816841b`
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
    #[ethcall(name = "uniswapPair", abi = "uniswapPair()")]
    pub struct UniswapPairCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum PuppetPoolCalls {
        DepositFactor(DepositFactorCall),
        Borrow(BorrowCall),
        CalculateDepositRequired(CalculateDepositRequiredCall),
        Deposits(DepositsCall),
        Token(TokenCall),
        UniswapPair(UniswapPairCall),
    }
    impl ::ethers::core::abi::AbiDecode for PuppetPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DepositFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DepositFactor(decoded));
            }
            if let Ok(decoded) =
                <BorrowCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Borrow(decoded));
            }
            if let Ok(decoded)
                = <CalculateDepositRequiredCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateDepositRequired(decoded));
            }
            if let Ok(decoded) =
                <DepositsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Deposits(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded) =
                <UniswapPairCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UniswapPair(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PuppetPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Borrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDepositRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapPair(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PuppetPoolCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::DepositFactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Borrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDepositRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposits(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapPair(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DepositFactorCall> for PuppetPoolCalls {
        fn from(value: DepositFactorCall) -> Self { Self::DepositFactor(value) }
    }
    impl ::core::convert::From<BorrowCall> for PuppetPoolCalls {
        fn from(value: BorrowCall) -> Self { Self::Borrow(value) }
    }
    impl ::core::convert::From<CalculateDepositRequiredCall> for PuppetPoolCalls {
        fn from(value: CalculateDepositRequiredCall) -> Self {
            Self::CalculateDepositRequired(value)
        }
    }
    impl ::core::convert::From<DepositsCall> for PuppetPoolCalls {
        fn from(value: DepositsCall) -> Self { Self::Deposits(value) }
    }
    impl ::core::convert::From<TokenCall> for PuppetPoolCalls {
        fn from(value: TokenCall) -> Self { Self::Token(value) }
    }
    impl ::core::convert::From<UniswapPairCall> for PuppetPoolCalls {
        fn from(value: UniswapPairCall) -> Self { Self::UniswapPair(value) }
    }
    ///Container type for all return fields from the
    /// `DEPOSIT_FACTOR` function with signature
    /// `DEPOSIT_FACTOR()` and selector `0x5d48e255`
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
    pub struct DepositFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `calculateDepositRequired` function with signature
    /// `calculateDepositRequired(uint256)` and selector
    /// `0xbc554c28`
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
    pub struct CalculateDepositRequiredReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the
    /// `uniswapPair` function with signature
    /// `uniswapPair()` and selector `0xc816841b`
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
    pub struct UniswapPairReturn(pub ::ethers::core::types::Address);
}
