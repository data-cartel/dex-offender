pub use damn_valuable_token_snapshot::*;
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
pub mod damn_valuable_token_snapshot {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialSupply"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOfAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOfAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("snapshotId"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subtractedValue"),
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
                    ::std::borrow::ToOwned::to_owned("getBalanceAtLastSnapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBalanceAtLastSnapshot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalSupplyAtLastSnapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalSupplyAtLastSnapshot",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addedValue"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("snapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("snapshot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastSnapshotId"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupplyAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupplyAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("snapshotId"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Snapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Snapshot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
    pub static DAMNVALUABLETOKENSNAPSHOT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x12,8\x03\x80b\0\x12,\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02\xD3V[`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\"0\xB6\xB7+0\xB6:\xB0\xB162\xAA7\xB5\xB2\xB7`y\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\x15\x95`\xEA\x1B\x81RP\x81`\x03\x90\x81b\0\0\x8B\x91\x90b\0\x03\x91V[P`\x04b\0\0\x9A\x82\x82b\0\x03\x91V[PPPb\0\0\xAF3\x82b\0\0\xB6` \x1B` \x1CV[Pb\0\x04\xBBV[`\x01`\x01`\xA0\x1B\x03\x82\x16b\0\x01\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[b\0\x01\x1F`\0\x83\x83b\0\x01\x8AV[\x80`\x02`\0\x82\x82Tb\0\x013\x91\x90b\0\x04sV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16b\0\x01\xB3Wb\0\x01\xA4\x82b\0\x01\xDEV[b\0\x01\xAEb\0\x02\x16V[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16b\0\x01\xCDWb\0\x01\xA4\x83b\0\x01\xDEV[b\0\x01\xD8\x83b\0\x01\xDEV[b\0\x01\xAE\x82[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x91\x83\x90R\x90\x91 Tb\0\x02\x13\x91\x90b\0\x02(V[b\0\x02(V[PV[b\0\x02&`\x06b\0\x02\r`\x02T\x90V[V[`\0b\0\x024b\0\x02wV[\x90P\x80b\0\x02B\x84b\0\x02\x88V[\x10\x15b\0\x01\xAEW\x82T`\x01\x80\x82\x01\x85U`\0\x85\x81R` \x80\x82 \x90\x93\x01\x93\x90\x93U\x93\x84\x01\x80T\x94\x85\x01\x81U\x82R\x90 \x90\x91\x01UV[`\0b\0\x02\x83`\x08T\x90V[\x90P\x90V[\x80T`\0\x90\x81\x03b\0\x02\x9CWP`\0\x91\x90PV[\x81T\x82\x90b\0\x02\xAE\x90`\x01\x90b\0\x04\x8FV[\x81T\x81\x10b\0\x02\xC1Wb\0\x02\xC1b\0\x04\xA5V[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x02\xE6W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x039WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xAEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x03hWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x03\x89W\x82\x81U`\x01\x01b\0\x03tV[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\xADWb\0\x03\xADb\0\x02\xEDV[b\0\x03\xC5\x81b\0\x03\xBE\x84Tb\0\x03\x03V[\x84b\0\x03?V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03\xFDW`\0\x84\x15b\0\x03\xE4WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x03\x89V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x04.W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x04\rV[P\x85\x82\x10\x15b\0\x04MW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15b\0\x04\x89Wb\0\x04\x89b\0\x04]V[\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15b\0\x04\x89Wb\0\x04\x89b\0\x04]V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[a\ra\x80b\0\x04\xCB`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\x97W\x80c\xA3\xECs\xFB\x11a\0fW\x80c\xA3\xECs\xFB\x14a\x01\xFFW\x80c\xA4W\xC2\xD7\x14a\x02\x07W\x80c\xA9\x05\x9C\xBB\x14a\x02\x1AW\x80c\xDDb\xED>\x14a\x02-W`\0\x80\xFD[\x80cp\xA0\x821\x14a\x01\xB3W\x80c\x95\xD8\x9BA\x14a\x01\xDCW\x80c\x97\x11qZ\x14a\x01\xE4W\x80c\x98\x1B$\xD0\x14a\x01\xECW`\0\x80\xFD[\x80c1<\xE5g\x11a\0\xD3W\x80c1<\xE5g\x14a\x01kW\x80c9P\x93Q\x14a\x01zW\x80cE\x91\x16N\x14a\x01\x8DW\x80cN\xE2\xCD~\x14a\x01\xA0W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\x05W\x80c\t^\xA7\xB3\x14a\x01#W\x80c\x18\x16\r\xDD\x14a\x01FW\x80c#\xB8r\xDD\x14a\x01XW[`\0\x80\xFD[a\x01\ra\x02@V[`@Qa\x01\x1A\x91\x90a\x0BKV[`@Q\x80\x91\x03\x90\xF3[a\x016a\x0116`\x04a\x0B\xB0V[a\x02\xD2V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1AV[`\x02T[`@Q\x90\x81R` \x01a\x01\x1AV[a\x016a\x01f6`\x04a\x0B\xDAV[a\x02\xECV[`@Q`\x12\x81R` \x01a\x01\x1AV[a\x016a\x01\x886`\x04a\x0B\xB0V[a\x03\x10V[a\x01Ja\x01\x9B6`\x04a\x0C\x16V[a\x032V[a\x01Ja\x01\xAE6`\x04a\x0B\xB0V[a\x03<V[a\x01Ja\x01\xC16`\x04a\x0C\x16V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\ra\x03\x95V[a\x01Ja\x03\xA4V[a\x01Ja\x01\xFA6`\x04a\x0C1V[a\x03\xB8V[a\x01Ja\x03\xE3V[a\x016a\x02\x156`\x04a\x0B\xB0V[a\x03\xF5V[a\x016a\x02(6`\x04a\x0B\xB0V[a\x04uV[a\x01Ja\x02;6`\x04a\x0CJV[a\x04\x83V[```\x03\x80Ta\x02O\x90a\x0C}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02{\x90a\x0C}V[\x80\x15a\x02\xC8W\x80`\x1F\x10a\x02\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x02\xE0\x81\x85\x85a\x04\xAEV[`\x01\x91PP[\x92\x91PPV[`\x003a\x02\xFA\x85\x82\x85a\x05\xD2V[a\x03\x05\x85\x85\x85a\x06LV[P`\x01\x94\x93PPPPV[`\x003a\x02\xE0\x81\x85\x85a\x03#\x83\x83a\x04\x83V[a\x03-\x91\x90a\x0C\xCDV[a\x04\xAEV[`\0a\x02\xE6\x82`\tT[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x05` R`@\x81 \x81\x90\x81\x90a\x03c\x90\x85\x90a\x07\xFBV[\x91P\x91P\x81a\x03\x8AW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 Ta\x03\x8CV[\x80[\x95\x94PPPPPV[```\x04\x80Ta\x02O\x90a\x0C}V[`\0a\x03\xAEa\x08\xF1V[`\t\x81\x90U\x91\x90PV[`\0\x80`\0a\x03\xC8\x84`\x06a\x07\xFBV[\x91P\x91P\x81a\x03\xD9W`\x02Ta\x03\xDBV[\x80[\x94\x93PPPPV[`\0a\x03\xF0`\tTa\x03\xB8V[\x90P\x90V[`\x003\x81a\x04\x03\x82\x86a\x04\x83V[\x90P\x83\x81\x10\x15a\x04hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x05\x82\x86\x86\x84\x03a\x04\xAEV[`\x003a\x02\xE0\x81\x85\x85a\x06LV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0a\x05\xDE\x84\x84a\x04\x83V[\x90P`\0\x19\x81\x14a\x06FW\x81\x81\x10\x15a\x069W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x04_V[a\x06F\x84\x84\x84\x84\x03a\x04\xAEV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x06\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x07\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x04_V[a\x07\x1D\x83\x83\x83a\tKV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x07\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x87\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x92Q\x85\x81R\x90\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x06FV[`\0\x80`\0\x84\x11a\x08GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x04U$3#\x056\xE6\x17\x076\x86\xF7C\xA2\x06\x96B\x06\x972\x03`T\x1B`D\x82\x01R`d\x01a\x04_V[a\x08Oa\t\x98V[\x84\x11\x15a\x08\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Snapshot: nonexistent id\0\0\0`D\x82\x01R`d\x01a\x04_V[`\0a\x08\xAA\x84\x86a\t\xA3V[\x84T\x90\x91P\x81\x03a\x08\xC2W`\0\x80\x92P\x92PPa\x08\xEAV[`\x01\x84`\x01\x01\x82\x81T\x81\x10a\x08\xD9Wa\x08\xD9a\x0C\xE0V[\x90`\0R` `\0 \x01T\x92P\x92PP[\x92P\x92\x90PV[`\0a\t\x01`\x08\x80T`\x01\x01\x90UV[`\0a\t\x0Ba\t\x98V[\x90P\x7F\x800\xE8;\x04\xD8{\xEFSH\x0E&&2f\xD6\xCAf\x86:\xA8Pj\xCAo%Y\xD1\x8A\xA1\xCBg\x81`@Qa\t>\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\toWa\tb\x82a\nPV[a\tja\n\x85V[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\x86Wa\tb\x83a\nPV[a\t\x8F\x83a\nPV[a\tj\x82a\nPV[`\0a\x03\xF0`\x08T\x90V[\x81T`\0\x90\x81\x03a\t\xB6WP`\0a\x02\xE6V[\x82T`\0\x90[\x80\x82\x10\x15a\n\x03W`\0a\t\xD0\x83\x83a\n\x95V[`\0\x87\x81R` \x90 \x90\x91P\x85\x90\x82\x01T\x11\x15a\t\xEFW\x80\x91Pa\t\xFDV[a\t\xFA\x81`\x01a\x0C\xCDV[\x92P[Pa\t\xBCV[`\0\x82\x11\x80\x15a\n/WP\x83a\n,\x86a\n\x1E`\x01\x86a\x0C\xF6V[`\0\x91\x82R` \x90\x91 \x01\x90V[T\x14[\x15a\nHWa\n?`\x01\x83a\x0C\xF6V[\x92PPPa\x02\xE6V[P\x90Pa\x02\xE6V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x91\x83\x90R\x90\x91 Ta\n\x82\x91\x90a\n\xB7V[a\n\xB7V[PV[a\n\x93`\x06a\n}`\x02T\x90V[V[`\0a\n\xA4`\x02\x84\x84\x18a\r\tV[a\n\xB0\x90\x84\x84\x16a\x0C\xCDV[\x93\x92PPPV[`\0a\n\xC1a\t\x98V[\x90P\x80a\n\xCD\x84a\x0B\x01V[\x10\x15a\tjW\x82T`\x01\x80\x82\x01\x85U`\0\x85\x81R` \x80\x82 \x90\x93\x01\x93\x90\x93U\x93\x84\x01\x80T\x94\x85\x01\x81U\x82R\x90 \x90\x91\x01UV[\x80T`\0\x90\x81\x03a\x0B\x14WP`\0\x91\x90PV[\x81T\x82\x90a\x0B$\x90`\x01\x90a\x0C\xF6V[\x81T\x81\x10a\x0B4Wa\x0B4a\x0C\xE0V[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x0BxW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0B\\V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xC3W`\0\x80\xFD[a\x0B\xCC\x83a\x0B\x99V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xEFW`\0\x80\xFD[a\x0B\xF8\x84a\x0B\x99V[\x92Pa\x0C\x06` \x85\x01a\x0B\x99V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0C(W`\0\x80\xFD[a\n\xB0\x82a\x0B\x99V[`\0` \x82\x84\x03\x12\x15a\x0CCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C]W`\0\x80\xFD[a\x0Cf\x83a\x0B\x99V[\x91Pa\x0Ct` \x84\x01a\x0B\x99V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\x91W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xB1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xE6Wa\x02\xE6a\x0C\xB7V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xE6Wa\x02\xE6a\x0C\xB7V[`\0\x82a\r&WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC2\xBF_\xE0\x15-\x85\xEC\"\xC9\x04\xB1\x1B\x15\x8848\xDF\x1B\x8D&\x82\0\x0F\xA6\\\xD4\xF6\x93\xE4\x9D\xF5dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static DAMNVALUABLETOKENSNAPSHOT_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\x97W\x80c\xA3\xECs\xFB\x11a\0fW\x80c\xA3\xECs\xFB\x14a\x01\xFFW\x80c\xA4W\xC2\xD7\x14a\x02\x07W\x80c\xA9\x05\x9C\xBB\x14a\x02\x1AW\x80c\xDDb\xED>\x14a\x02-W`\0\x80\xFD[\x80cp\xA0\x821\x14a\x01\xB3W\x80c\x95\xD8\x9BA\x14a\x01\xDCW\x80c\x97\x11qZ\x14a\x01\xE4W\x80c\x98\x1B$\xD0\x14a\x01\xECW`\0\x80\xFD[\x80c1<\xE5g\x11a\0\xD3W\x80c1<\xE5g\x14a\x01kW\x80c9P\x93Q\x14a\x01zW\x80cE\x91\x16N\x14a\x01\x8DW\x80cN\xE2\xCD~\x14a\x01\xA0W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\x05W\x80c\t^\xA7\xB3\x14a\x01#W\x80c\x18\x16\r\xDD\x14a\x01FW\x80c#\xB8r\xDD\x14a\x01XW[`\0\x80\xFD[a\x01\ra\x02@V[`@Qa\x01\x1A\x91\x90a\x0BKV[`@Q\x80\x91\x03\x90\xF3[a\x016a\x0116`\x04a\x0B\xB0V[a\x02\xD2V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1AV[`\x02T[`@Q\x90\x81R` \x01a\x01\x1AV[a\x016a\x01f6`\x04a\x0B\xDAV[a\x02\xECV[`@Q`\x12\x81R` \x01a\x01\x1AV[a\x016a\x01\x886`\x04a\x0B\xB0V[a\x03\x10V[a\x01Ja\x01\x9B6`\x04a\x0C\x16V[a\x032V[a\x01Ja\x01\xAE6`\x04a\x0B\xB0V[a\x03<V[a\x01Ja\x01\xC16`\x04a\x0C\x16V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\ra\x03\x95V[a\x01Ja\x03\xA4V[a\x01Ja\x01\xFA6`\x04a\x0C1V[a\x03\xB8V[a\x01Ja\x03\xE3V[a\x016a\x02\x156`\x04a\x0B\xB0V[a\x03\xF5V[a\x016a\x02(6`\x04a\x0B\xB0V[a\x04uV[a\x01Ja\x02;6`\x04a\x0CJV[a\x04\x83V[```\x03\x80Ta\x02O\x90a\x0C}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02{\x90a\x0C}V[\x80\x15a\x02\xC8W\x80`\x1F\x10a\x02\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x02\xE0\x81\x85\x85a\x04\xAEV[`\x01\x91PP[\x92\x91PPV[`\x003a\x02\xFA\x85\x82\x85a\x05\xD2V[a\x03\x05\x85\x85\x85a\x06LV[P`\x01\x94\x93PPPPV[`\x003a\x02\xE0\x81\x85\x85a\x03#\x83\x83a\x04\x83V[a\x03-\x91\x90a\x0C\xCDV[a\x04\xAEV[`\0a\x02\xE6\x82`\tT[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x05` R`@\x81 \x81\x90\x81\x90a\x03c\x90\x85\x90a\x07\xFBV[\x91P\x91P\x81a\x03\x8AW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 Ta\x03\x8CV[\x80[\x95\x94PPPPPV[```\x04\x80Ta\x02O\x90a\x0C}V[`\0a\x03\xAEa\x08\xF1V[`\t\x81\x90U\x91\x90PV[`\0\x80`\0a\x03\xC8\x84`\x06a\x07\xFBV[\x91P\x91P\x81a\x03\xD9W`\x02Ta\x03\xDBV[\x80[\x94\x93PPPPV[`\0a\x03\xF0`\tTa\x03\xB8V[\x90P\x90V[`\x003\x81a\x04\x03\x82\x86a\x04\x83V[\x90P\x83\x81\x10\x15a\x04hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x05\x82\x86\x86\x84\x03a\x04\xAEV[`\x003a\x02\xE0\x81\x85\x85a\x06LV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0a\x05\xDE\x84\x84a\x04\x83V[\x90P`\0\x19\x81\x14a\x06FW\x81\x81\x10\x15a\x069W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x04_V[a\x06F\x84\x84\x84\x84\x03a\x04\xAEV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x06\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x07\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x04_V[a\x07\x1D\x83\x83\x83a\tKV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x07\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x04_V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x87\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x92Q\x85\x81R\x90\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x06FV[`\0\x80`\0\x84\x11a\x08GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x04U$3#\x056\xE6\x17\x076\x86\xF7C\xA2\x06\x96B\x06\x972\x03`T\x1B`D\x82\x01R`d\x01a\x04_V[a\x08Oa\t\x98V[\x84\x11\x15a\x08\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Snapshot: nonexistent id\0\0\0`D\x82\x01R`d\x01a\x04_V[`\0a\x08\xAA\x84\x86a\t\xA3V[\x84T\x90\x91P\x81\x03a\x08\xC2W`\0\x80\x92P\x92PPa\x08\xEAV[`\x01\x84`\x01\x01\x82\x81T\x81\x10a\x08\xD9Wa\x08\xD9a\x0C\xE0V[\x90`\0R` `\0 \x01T\x92P\x92PP[\x92P\x92\x90PV[`\0a\t\x01`\x08\x80T`\x01\x01\x90UV[`\0a\t\x0Ba\t\x98V[\x90P\x7F\x800\xE8;\x04\xD8{\xEFSH\x0E&&2f\xD6\xCAf\x86:\xA8Pj\xCAo%Y\xD1\x8A\xA1\xCBg\x81`@Qa\t>\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\toWa\tb\x82a\nPV[a\tja\n\x85V[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\x86Wa\tb\x83a\nPV[a\t\x8F\x83a\nPV[a\tj\x82a\nPV[`\0a\x03\xF0`\x08T\x90V[\x81T`\0\x90\x81\x03a\t\xB6WP`\0a\x02\xE6V[\x82T`\0\x90[\x80\x82\x10\x15a\n\x03W`\0a\t\xD0\x83\x83a\n\x95V[`\0\x87\x81R` \x90 \x90\x91P\x85\x90\x82\x01T\x11\x15a\t\xEFW\x80\x91Pa\t\xFDV[a\t\xFA\x81`\x01a\x0C\xCDV[\x92P[Pa\t\xBCV[`\0\x82\x11\x80\x15a\n/WP\x83a\n,\x86a\n\x1E`\x01\x86a\x0C\xF6V[`\0\x91\x82R` \x90\x91 \x01\x90V[T\x14[\x15a\nHWa\n?`\x01\x83a\x0C\xF6V[\x92PPPa\x02\xE6V[P\x90Pa\x02\xE6V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x91\x83\x90R\x90\x91 Ta\n\x82\x91\x90a\n\xB7V[a\n\xB7V[PV[a\n\x93`\x06a\n}`\x02T\x90V[V[`\0a\n\xA4`\x02\x84\x84\x18a\r\tV[a\n\xB0\x90\x84\x84\x16a\x0C\xCDV[\x93\x92PPPV[`\0a\n\xC1a\t\x98V[\x90P\x80a\n\xCD\x84a\x0B\x01V[\x10\x15a\tjW\x82T`\x01\x80\x82\x01\x85U`\0\x85\x81R` \x80\x82 \x90\x93\x01\x93\x90\x93U\x93\x84\x01\x80T\x94\x85\x01\x81U\x82R\x90 \x90\x91\x01UV[\x80T`\0\x90\x81\x03a\x0B\x14WP`\0\x91\x90PV[\x81T\x82\x90a\x0B$\x90`\x01\x90a\x0C\xF6V[\x81T\x81\x10a\x0B4Wa\x0B4a\x0C\xE0V[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x0BxW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0B\\V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xC3W`\0\x80\xFD[a\x0B\xCC\x83a\x0B\x99V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xEFW`\0\x80\xFD[a\x0B\xF8\x84a\x0B\x99V[\x92Pa\x0C\x06` \x85\x01a\x0B\x99V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0C(W`\0\x80\xFD[a\n\xB0\x82a\x0B\x99V[`\0` \x82\x84\x03\x12\x15a\x0CCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C]W`\0\x80\xFD[a\x0Cf\x83a\x0B\x99V[\x91Pa\x0Ct` \x84\x01a\x0B\x99V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\x91W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xB1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xE6Wa\x02\xE6a\x0C\xB7V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xE6Wa\x02\xE6a\x0C\xB7V[`\0\x82a\r&WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC2\xBF_\xE0\x15-\x85\xEC\"\xC9\x04\xB1\x1B\x15\x8848\xDF\x1B\x8D&\x82\0\x0F\xA6\\\xD4\xF6\x93\xE4\x9D\xF5dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static DAMNVALUABLETOKENSNAPSHOT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DamnValuableTokenSnapshot<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DamnValuableTokenSnapshot<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for DamnValuableTokenSnapshot<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for DamnValuableTokenSnapshot<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for DamnValuableTokenSnapshot<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DamnValuableTokenSnapshot))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DamnValuableTokenSnapshot<M> {
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
                DAMNVALUABLETOKENSNAPSHOT_ABI.clone(),
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
                DAMNVALUABLETOKENSNAPSHOT_ABI.clone(),
                DAMNVALUABLETOKENSNAPSHOT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowance` (0xdd62ed3e)
        /// function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3)
        /// function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231)
        /// function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfAt` (0x4ee2cd7e)
        /// function
        pub fn balance_of_at(
            &self,
            account: ::ethers::core::types::Address,
            snapshot_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([78, 226, 205, 126], (account, snapshot_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567)
        /// function
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance`
        /// (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalanceAtLastSnapshot`
        /// (0x4591164e) function
        pub fn get_balance_at_last_snapshot(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([69, 145, 22, 78], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's
        /// `getTotalSupplyAtLastSnapshot` (0xa3ec73fb)
        /// function
        pub fn get_total_supply_at_last_snapshot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([163, 236, 115, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance`
        /// (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03)
        /// function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `snapshot` (0x9711715a)
        /// function
        pub fn snapshot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([151, 17, 113, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41)
        /// function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd)
        /// function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupplyAt`
        /// (0x981b24d0) function
        pub fn total_supply_at(
            &self,
            snapshot_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([152, 27, 36, 208], snapshot_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb)
        /// function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd)
        /// function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Snapshot` event
        pub fn snapshot_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SnapshotFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
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
            DamnValuableTokenSnapshotEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for DamnValuableTokenSnapshot<M>
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Snapshot", abi = "Snapshot(uint256)")]
    pub struct SnapshotFilter {
        pub id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum DamnValuableTokenSnapshotEvents {
        ApprovalFilter(ApprovalFilter),
        SnapshotFilter(SnapshotFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for DamnValuableTokenSnapshotEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(DamnValuableTokenSnapshotEvents::ApprovalFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SnapshotFilter::decode_log(log) {
                return Ok(DamnValuableTokenSnapshotEvents::SnapshotFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(DamnValuableTokenSnapshotEvents::TransferFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DamnValuableTokenSnapshotEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SnapshotFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for DamnValuableTokenSnapshotEvents {
        fn from(value: ApprovalFilter) -> Self { Self::ApprovalFilter(value) }
    }
    impl ::core::convert::From<SnapshotFilter> for DamnValuableTokenSnapshotEvents {
        fn from(value: SnapshotFilter) -> Self { Self::SnapshotFilter(value) }
    }
    impl ::core::convert::From<TransferFilter> for DamnValuableTokenSnapshotEvents {
        fn from(value: TransferFilter) -> Self { Self::TransferFilter(value) }
    }
    ///Container type for all input parameters for the
    /// `allowance` function with signature
    /// `allowance(address,address)` and selector
    /// `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `approve` function with signature
    /// `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `balanceOf` function with signature
    /// `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `balanceOfAt` function with signature
    /// `balanceOfAt(address,uint256)` and selector
    /// `0x4ee2cd7e`
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
    #[ethcall(name = "balanceOfAt", abi = "balanceOfAt(address,uint256)")]
    pub struct BalanceOfAtCall {
        pub account: ::ethers::core::types::Address,
        pub snapshot_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `decimals` function with signature `decimals()` and
    /// selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the
    /// `decreaseAllowance` function with signature
    /// `decreaseAllowance(address,uint256)` and selector
    /// `0xa457c2d7`
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
        name = "decreaseAllowance",
        abi = "decreaseAllowance(address,uint256)"
    )]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `getBalanceAtLastSnapshot` function with signature
    /// `getBalanceAtLastSnapshot(address)` and selector
    /// `0x4591164e`
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
        name = "getBalanceAtLastSnapshot",
        abi = "getBalanceAtLastSnapshot(address)"
    )]
    pub struct GetBalanceAtLastSnapshotCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `getTotalSupplyAtLastSnapshot` function with
    /// signature `getTotalSupplyAtLastSnapshot()` and
    /// selector `0xa3ec73fb`
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
        name = "getTotalSupplyAtLastSnapshot",
        abi = "getTotalSupplyAtLastSnapshot()"
    )]
    pub struct GetTotalSupplyAtLastSnapshotCall;
    ///Container type for all input parameters for the
    /// `increaseAllowance` function with signature
    /// `increaseAllowance(address,uint256)` and selector
    /// `0x39509351`
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
        name = "increaseAllowance",
        abi = "increaseAllowance(address,uint256)"
    )]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `name` function with signature `name()` and selector
    /// `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the
    /// `snapshot` function with signature `snapshot()` and
    /// selector `0x9711715a`
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
    #[ethcall(name = "snapshot", abi = "snapshot()")]
    pub struct SnapshotCall;
    ///Container type for all input parameters for the
    /// `symbol` function with signature `symbol()` and
    /// selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the
    /// `totalSupply` function with signature
    /// `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the
    /// `totalSupplyAt` function with signature
    /// `totalSupplyAt(uint256)` and selector `0x981b24d0`
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
    #[ethcall(name = "totalSupplyAt", abi = "totalSupplyAt(uint256)")]
    pub struct TotalSupplyAtCall {
        pub snapshot_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `transfer` function with signature
    /// `transfer(address,uint256)` and selector
    /// `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `transferFrom` function with signature
    /// `transferFrom(address,address,uint256)` and selector
    /// `0x23b872dd`
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
        name = "transferFrom",
        abi = "transferFrom(address,address,uint256)"
    )]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum DamnValuableTokenSnapshotCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BalanceOfAt(BalanceOfAtCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        GetBalanceAtLastSnapshot(GetBalanceAtLastSnapshotCall),
        GetTotalSupplyAtLastSnapshot(GetTotalSupplyAtLastSnapshotCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Name(NameCall),
        Snapshot(SnapshotCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        TotalSupplyAt(TotalSupplyAtCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for DamnValuableTokenSnapshotCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfAtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::BalanceOfAt(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <GetBalanceAtLastSnapshotCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetBalanceAtLastSnapshot(decoded));
            }
            if let Ok(decoded)
                = <GetTotalSupplyAtLastSnapshotCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTotalSupplyAtLastSnapshot(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <NameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) =
                <SnapshotCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Snapshot(decoded));
            }
            if let Ok(decoded) =
                <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyAtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TotalSupplyAt(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DamnValuableTokenSnapshotCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOfAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBalanceAtLastSnapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalSupplyAtLastSnapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Snapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupplyAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DamnValuableTokenSnapshotCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceOfAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Decimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBalanceAtLastSnapshot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalSupplyAtLastSnapshot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Snapshot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupplyAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Transfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: AllowanceCall) -> Self { Self::Allowance(value) }
    }
    impl ::core::convert::From<ApproveCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: ApproveCall) -> Self { Self::Approve(value) }
    }
    impl ::core::convert::From<BalanceOfCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: BalanceOfCall) -> Self { Self::BalanceOf(value) }
    }
    impl ::core::convert::From<BalanceOfAtCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: BalanceOfAtCall) -> Self { Self::BalanceOfAt(value) }
    }
    impl ::core::convert::From<DecimalsCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: DecimalsCall) -> Self { Self::Decimals(value) }
    }
    impl ::core::convert::From<DecreaseAllowanceCall>
        for DamnValuableTokenSnapshotCalls
    {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<GetBalanceAtLastSnapshotCall>
        for DamnValuableTokenSnapshotCalls
    {
        fn from(value: GetBalanceAtLastSnapshotCall) -> Self {
            Self::GetBalanceAtLastSnapshot(value)
        }
    }
    impl ::core::convert::From<GetTotalSupplyAtLastSnapshotCall>
        for DamnValuableTokenSnapshotCalls
    {
        fn from(value: GetTotalSupplyAtLastSnapshotCall) -> Self {
            Self::GetTotalSupplyAtLastSnapshot(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall>
        for DamnValuableTokenSnapshotCalls
    {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<NameCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: NameCall) -> Self { Self::Name(value) }
    }
    impl ::core::convert::From<SnapshotCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: SnapshotCall) -> Self { Self::Snapshot(value) }
    }
    impl ::core::convert::From<SymbolCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: SymbolCall) -> Self { Self::Symbol(value) }
    }
    impl ::core::convert::From<TotalSupplyCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: TotalSupplyCall) -> Self { Self::TotalSupply(value) }
    }
    impl ::core::convert::From<TotalSupplyAtCall>
        for DamnValuableTokenSnapshotCalls
    {
        fn from(value: TotalSupplyAtCall) -> Self { Self::TotalSupplyAt(value) }
    }
    impl ::core::convert::From<TransferCall> for DamnValuableTokenSnapshotCalls {
        fn from(value: TransferCall) -> Self { Self::Transfer(value) }
    }
    impl ::core::convert::From<TransferFromCall>
        for DamnValuableTokenSnapshotCalls
    {
        fn from(value: TransferFromCall) -> Self { Self::TransferFrom(value) }
    }
    ///Container type for all return fields from the
    /// `allowance` function with signature
    /// `allowance(address,address)` and selector
    /// `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `approve` function with signature
    /// `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the
    /// `balanceOf` function with signature
    /// `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `balanceOfAt` function with signature
    /// `balanceOfAt(address,uint256)` and selector
    /// `0x4ee2cd7e`
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
    pub struct BalanceOfAtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `decimals` function with signature `decimals()` and
    /// selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the
    /// `decreaseAllowance` function with signature
    /// `decreaseAllowance(address,uint256)` and selector
    /// `0xa457c2d7`
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
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the
    /// `getBalanceAtLastSnapshot` function with signature
    /// `getBalanceAtLastSnapshot(address)` and selector
    /// `0x4591164e`
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
    pub struct GetBalanceAtLastSnapshotReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getTotalSupplyAtLastSnapshot` function with
    /// signature `getTotalSupplyAtLastSnapshot()` and
    /// selector `0xa3ec73fb`
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
    pub struct GetTotalSupplyAtLastSnapshotReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the
    /// `increaseAllowance` function with signature
    /// `increaseAllowance(address,uint256)` and selector
    /// `0x39509351`
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
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `name`
    /// function with signature `name()` and selector
    /// `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the
    /// `snapshot` function with signature `snapshot()` and
    /// selector `0x9711715a`
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
    pub struct SnapshotReturn {
        pub last_snapshot_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the
    /// `symbol` function with signature `symbol()` and
    /// selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the
    /// `totalSupply` function with signature
    /// `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `totalSupplyAt` function with signature
    /// `totalSupplyAt(uint256)` and selector `0x981b24d0`
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
    pub struct TotalSupplyAtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `transfer` function with signature
    /// `transfer(address,uint256)` and selector
    /// `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the
    /// `transferFrom` function with signature
    /// `transferFrom(address,address,uint256)` and selector
    /// `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
