pub use double_entry_point::*;
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
pub mod double_entry_point {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("legacyToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("vaultAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("fortaAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("playerAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("cryptoVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cryptoVault"),
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
                    ::std::borrow::ToOwned::to_owned("delegateTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegateTransfer"),
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
                                    name: ::std::borrow::ToOwned::to_owned("origSender"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegatedFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegatedFrom"),
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
                    ::std::borrow::ToOwned::to_owned("forta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("forta"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Forta"),
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
                    ::std::borrow::ToOwned::to_owned("player"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("player"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
    pub static DOUBLEENTRYPOINT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x12^8\x03\x80b\0\x12^\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02hV[`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7FDoubleEntryPointToken\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\x11U`\xEA\x1B\x81RP\x81`\x03\x90\x81b\0\0\x97\x91\x90b\0\x03iV[P`\x04b\0\0\xA6\x82\x82b\0\x03iV[PPPb\0\0\xC3b\0\0\xBDb\0\x01*` \x1B` \x1CV[b\0\x01.V[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\t\x80T\x85\x84\x16\x90\x83\x16\x17\x90U`\x07\x80T\x84\x84\x16\x90\x83\x16\x17\x90U`\x06\x80T\x92\x86\x16\x92\x90\x91\x16\x82\x17\x90Ub\0\x01 \x90h\x05k\xC7^-c\x10\0\0b\0\x01\x80V[PPPPb\0\x04]V[3\x90V[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16b\0\x01\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02`\0\x82\x82Tb\0\x01\xEF\x91\x90b\0\x045V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[PPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02cW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02\x7FW`\0\x80\xFD[b\0\x02\x8A\x85b\0\x02KV[\x93Pb\0\x02\x9A` \x86\x01b\0\x02KV[\x92Pb\0\x02\xAA`@\x86\x01b\0\x02KV[\x91Pb\0\x02\xBA``\x86\x01b\0\x02KV[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02\xF0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\x11WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02FW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x03@WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x03aW\x82\x81U`\x01\x01b\0\x03LV[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\x85Wb\0\x03\x85b\0\x02\xC5V[b\0\x03\x9D\x81b\0\x03\x96\x84Tb\0\x02\xDBV[\x84b\0\x03\x17V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03\xD5W`\0\x84\x15b\0\x03\xBCWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x03aV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x04\x06W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03\xE5V[P\x85\x82\x10\x15b\0\x04%W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x80\x82\x01\x80\x82\x11\x15b\0\x04WWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[a\r\xF1\x80b\0\x04m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xADW\x80c\xA9\x05\x9C\xBB\x11a\0qW\x80c\xA9\x05\x9C\xBB\x14a\x02^W\x80c\xD4D\xD8\xA0\x14a\x02qW\x80c\xD8g\x0E\x1F\x14a\x02\x84W\x80c\xDDb\xED>\x14a\x02\x97W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xAAW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x15W\x80c\x8D\xA5\xCB[\x14a\x02\x1FW\x80c\x95\xD8\x9BA\x14a\x020W\x80c\x9C\xD1\xA1!\x14a\x028W\x80c\xA4W\xC2\xD7\x14a\x02KW`\0\x80\xFD[\x80c&\xFE\x99Q\x11a\0\xF4W\x80c&\xFE\x99Q\x14a\x01\x8CW\x80c1<\xE5g\x14a\x01\xB7W\x80c9P\x93Q\x14a\x01\xC6W\x80cH\xDB_\x89\x14a\x01\xD9W\x80cp\xA0\x821\x14a\x01\xECW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01&W\x80c\t^\xA7\xB3\x14a\x01DW\x80c\x18\x16\r\xDD\x14a\x01gW\x80c#\xB8r\xDD\x14a\x01yW[`\0\x80\xFD[a\x01.a\x02\xBDV[`@Qa\x01;\x91\x90a\x0B{V[`@Q\x80\x91\x03\x90\xF3[a\x01Wa\x01R6`\x04a\x0B\xDEV[a\x03OV[`@Q\x90\x15\x15\x81R` \x01a\x01;V[`\x02T[`@Q\x90\x81R` \x01a\x01;V[a\x01Wa\x01\x876`\x04a\x0C\nV[a\x03iV[`\x08Ta\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01;V[`@Q`\x12\x81R` \x01a\x01;V[a\x01Wa\x01\xD46`\x04a\x0B\xDEV[a\x03\x8DV[`\x07Ta\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ka\x01\xFA6`\x04a\x0CKV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02\x1Da\x03\xAFV[\0[`\x05T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x9FV[a\x01.a\x03\xC3V[a\x01Wa\x02F6`\x04a\x0CoV[a\x03\xD2V[a\x01Wa\x02Y6`\x04a\x0B\xDEV[a\x06`V[a\x01Wa\x02l6`\x04a\x0B\xDEV[a\x06\xDBV[`\x06Ta\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\tTa\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ka\x02\xA56`\x04a\x0C\xB1V[a\x06\xE9V[a\x02\x1Da\x02\xB86`\x04a\x0CKV[a\x07\x14V[```\x03\x80Ta\x02\xCC\x90a\x0C\xEAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xF8\x90a\x0C\xEAV[\x80\x15a\x03EW\x80`\x1F\x10a\x03\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x03]\x81\x85\x85a\x07\x8DV[`\x01\x91PP[\x92\x91PPV[`\x003a\x03w\x85\x82\x85a\x08\xB1V[a\x03\x82\x85\x85\x85a\t+V[P`\x01\x94\x93PPPPV[`\x003a\x03]\x81\x85\x85a\x03\xA0\x83\x83a\x06\xE9V[a\x03\xAA\x91\x90a\r$V[a\x07\x8DV[a\x03\xB7a\n\xCFV[a\x03\xC1`\0a\x0B)V[V[```\x04\x80Ta\x02\xCC\x90a\x0C\xEAV[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x13\x9B\xDD\x08\x1B\x19Y\xD8X\xDEH\x18\xDB\xDB\x9D\x1C\x98X\xDD`j\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\tT`\x07T`@QcO\x08A\xE5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\0\x92\x91\x90\x91\x16\x90c\x9E\x10\x83\xCA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9F\x91\x90a\rEV[`\tT`@Qc\x06\xE3\x81}`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xDCp/\xA0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x13\x91\x90a\rbV[`\tT`\x07T`@Qc>\x87\xF4\xA3`\xE2\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92c\xFA\x1F\xD2\x8C\x92a\x05N\x92\x16\x90`\0\x906\x90`\x04\x01a\r{V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05hW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05|W=`\0\x80>=`\0\xFD[PPPPa\x05\x8B\x84\x87\x87a\t+V[`\tT`@Qc\x06\xE3\x81}`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`\x01\x95P\x83\x92\x16\x90c\xDCp/\xA0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFD\x91\x90a\rbV[\x11\x15a\x06WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FAlert has been triggered, revert`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x04!V[PP\x93\x92PPPV[`\x003\x81a\x06n\x82\x86a\x06\xE9V[\x90P\x83\x81\x10\x15a\x06\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x04!V[a\x03\x82\x82\x86\x86\x84\x03a\x07\x8DV[`\x003a\x03]\x81\x85\x85a\t+V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x07\x1Ca\n\xCFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x04!V[a\x07\x8A\x81a\x0B)V[PV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x07\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0a\x08\xBD\x84\x84a\x06\xE9V[\x90P`\0\x19\x81\x14a\t%W\x81\x81\x10\x15a\t\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x04!V[a\t%\x84\x84\x84\x84\x03a\x07\x8DV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\t\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\niW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x87\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x92Q\x85\x81R\x90\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\t%V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04!V[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x0B\xA8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0B\x8CV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x8AW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xF1W`\0\x80\xFD[\x825a\x0B\xFC\x81a\x0B\xC9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x1FW`\0\x80\xFD[\x835a\x0C*\x81a\x0B\xC9V[\x92P` \x84\x015a\x0C:\x81a\x0B\xC9V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x0C]W`\0\x80\xFD[\x815a\x0Ch\x81a\x0B\xC9V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x84W`\0\x80\xFD[\x835a\x0C\x8F\x81a\x0B\xC9V[\x92P` \x84\x015\x91P`@\x84\x015a\x0C\xA6\x81a\x0B\xC9V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xC4W`\0\x80\xFD[\x825a\x0C\xCF\x81a\x0B\xC9V[\x91P` \x83\x015a\x0C\xDF\x81a\x0B\xC9V[\x80\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xFEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\x1EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x03cWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\rWW`\0\x80\xFD[\x81Qa\x0Ch\x81a\x0B\xC9V[`\0` \x82\x84\x03\x12\x15a\rtW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x88\xBF\xBE\x8F,]\xE9*;\xE8\xC0E-\x05\xC6B\xD2\x82|\x06\xF2Gfu&\xE7MhU\x9E\xE8#dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static DOUBLEENTRYPOINT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xADW\x80c\xA9\x05\x9C\xBB\x11a\0qW\x80c\xA9\x05\x9C\xBB\x14a\x02^W\x80c\xD4D\xD8\xA0\x14a\x02qW\x80c\xD8g\x0E\x1F\x14a\x02\x84W\x80c\xDDb\xED>\x14a\x02\x97W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xAAW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x15W\x80c\x8D\xA5\xCB[\x14a\x02\x1FW\x80c\x95\xD8\x9BA\x14a\x020W\x80c\x9C\xD1\xA1!\x14a\x028W\x80c\xA4W\xC2\xD7\x14a\x02KW`\0\x80\xFD[\x80c&\xFE\x99Q\x11a\0\xF4W\x80c&\xFE\x99Q\x14a\x01\x8CW\x80c1<\xE5g\x14a\x01\xB7W\x80c9P\x93Q\x14a\x01\xC6W\x80cH\xDB_\x89\x14a\x01\xD9W\x80cp\xA0\x821\x14a\x01\xECW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01&W\x80c\t^\xA7\xB3\x14a\x01DW\x80c\x18\x16\r\xDD\x14a\x01gW\x80c#\xB8r\xDD\x14a\x01yW[`\0\x80\xFD[a\x01.a\x02\xBDV[`@Qa\x01;\x91\x90a\x0B{V[`@Q\x80\x91\x03\x90\xF3[a\x01Wa\x01R6`\x04a\x0B\xDEV[a\x03OV[`@Q\x90\x15\x15\x81R` \x01a\x01;V[`\x02T[`@Q\x90\x81R` \x01a\x01;V[a\x01Wa\x01\x876`\x04a\x0C\nV[a\x03iV[`\x08Ta\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01;V[`@Q`\x12\x81R` \x01a\x01;V[a\x01Wa\x01\xD46`\x04a\x0B\xDEV[a\x03\x8DV[`\x07Ta\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ka\x01\xFA6`\x04a\x0CKV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02\x1Da\x03\xAFV[\0[`\x05T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x9FV[a\x01.a\x03\xC3V[a\x01Wa\x02F6`\x04a\x0CoV[a\x03\xD2V[a\x01Wa\x02Y6`\x04a\x0B\xDEV[a\x06`V[a\x01Wa\x02l6`\x04a\x0B\xDEV[a\x06\xDBV[`\x06Ta\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\tTa\x01\x9F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ka\x02\xA56`\x04a\x0C\xB1V[a\x06\xE9V[a\x02\x1Da\x02\xB86`\x04a\x0CKV[a\x07\x14V[```\x03\x80Ta\x02\xCC\x90a\x0C\xEAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xF8\x90a\x0C\xEAV[\x80\x15a\x03EW\x80`\x1F\x10a\x03\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x03]\x81\x85\x85a\x07\x8DV[`\x01\x91PP[\x92\x91PPV[`\x003a\x03w\x85\x82\x85a\x08\xB1V[a\x03\x82\x85\x85\x85a\t+V[P`\x01\x94\x93PPPPV[`\x003a\x03]\x81\x85\x85a\x03\xA0\x83\x83a\x06\xE9V[a\x03\xAA\x91\x90a\r$V[a\x07\x8DV[a\x03\xB7a\n\xCFV[a\x03\xC1`\0a\x0B)V[V[```\x04\x80Ta\x02\xCC\x90a\x0C\xEAV[`\x08T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x13\x9B\xDD\x08\x1B\x19Y\xD8X\xDEH\x18\xDB\xDB\x9D\x1C\x98X\xDD`j\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\tT`\x07T`@QcO\x08A\xE5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\0\x92\x91\x90\x91\x16\x90c\x9E\x10\x83\xCA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9F\x91\x90a\rEV[`\tT`@Qc\x06\xE3\x81}`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xDCp/\xA0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x13\x91\x90a\rbV[`\tT`\x07T`@Qc>\x87\xF4\xA3`\xE2\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92c\xFA\x1F\xD2\x8C\x92a\x05N\x92\x16\x90`\0\x906\x90`\x04\x01a\r{V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05hW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05|W=`\0\x80>=`\0\xFD[PPPPa\x05\x8B\x84\x87\x87a\t+V[`\tT`@Qc\x06\xE3\x81}`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`\x01\x95P\x83\x92\x16\x90c\xDCp/\xA0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFD\x91\x90a\rbV[\x11\x15a\x06WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FAlert has been triggered, revert`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x04!V[PP\x93\x92PPPV[`\x003\x81a\x06n\x82\x86a\x06\xE9V[\x90P\x83\x81\x10\x15a\x06\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x04!V[a\x03\x82\x82\x86\x86\x84\x03a\x07\x8DV[`\x003a\x03]\x81\x85\x85a\t+V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x07\x1Ca\n\xCFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x04!V[a\x07\x8A\x81a\x0B)V[PV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x07\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0a\x08\xBD\x84\x84a\x06\xE9V[\x90P`\0\x19\x81\x14a\t%W\x81\x81\x10\x15a\t\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x04!V[a\t%\x84\x84\x84\x84\x03a\x07\x8DV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\t\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\niW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x04!V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x87\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x92Q\x85\x81R\x90\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\t%V[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04!V[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x0B\xA8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0B\x8CV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x8AW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xF1W`\0\x80\xFD[\x825a\x0B\xFC\x81a\x0B\xC9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x1FW`\0\x80\xFD[\x835a\x0C*\x81a\x0B\xC9V[\x92P` \x84\x015a\x0C:\x81a\x0B\xC9V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x0C]W`\0\x80\xFD[\x815a\x0Ch\x81a\x0B\xC9V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x84W`\0\x80\xFD[\x835a\x0C\x8F\x81a\x0B\xC9V[\x92P` \x84\x015\x91P`@\x84\x015a\x0C\xA6\x81a\x0B\xC9V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xC4W`\0\x80\xFD[\x825a\x0C\xCF\x81a\x0B\xC9V[\x91P` \x83\x015a\x0C\xDF\x81a\x0B\xC9V[\x80\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xFEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\x1EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x03cWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\rWW`\0\x80\xFD[\x81Qa\x0Ch\x81a\x0B\xC9V[`\0` \x82\x84\x03\x12\x15a\rtW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x88\xBF\xBE\x8F,]\xE9*;\xE8\xC0E-\x05\xC6B\xD2\x82|\x06\xF2Gfu&\xE7MhU\x9E\xE8#dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static DOUBLEENTRYPOINT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DoubleEntryPoint<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DoubleEntryPoint<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for DoubleEntryPoint<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for DoubleEntryPoint<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for DoubleEntryPoint<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DoubleEntryPoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DoubleEntryPoint<M> {
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
                DOUBLEENTRYPOINT_ABI.clone(),
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
                DOUBLEENTRYPOINT_ABI.clone(),
                DOUBLEENTRYPOINT_BYTECODE.clone().into(),
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
        ///Calls the contract's `cryptoVault` (0xd444d8a0)
        /// function
        pub fn crypto_vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([212, 68, 216, 160], ())
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
        ///Calls the contract's `delegateTransfer`
        /// (0x9cd1a121) function
        pub fn delegate_transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            orig_sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([156, 209, 161, 33], (to, value, orig_sender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegatedFrom`
        /// (0x26fe9951) function
        pub fn delegated_from(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([38, 254, 153, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forta` (0xd8670e1f)
        /// function
        pub fn forta(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([216, 103, 14, 31], ())
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
        ///Calls the contract's `owner` (0x8da5cb5b)
        /// function
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
        ///Calls the contract's `player` (0x48db5f89)
        /// function
        pub fn player(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([72, 219, 95, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership`
        /// (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `transferOwnership`
        /// (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
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
            DoubleEntryPointEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for DoubleEntryPoint<M>
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum DoubleEntryPointEvents {
        ApprovalFilter(ApprovalFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for DoubleEntryPointEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(DoubleEntryPointEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DoubleEntryPointEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(DoubleEntryPointEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DoubleEntryPointEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for DoubleEntryPointEvents {
        fn from(value: ApprovalFilter) -> Self { Self::ApprovalFilter(value) }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
        for DoubleEntryPointEvents
    {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for DoubleEntryPointEvents {
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
    /// `cryptoVault` function with signature
    /// `cryptoVault()` and selector `0xd444d8a0`
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
    #[ethcall(name = "cryptoVault", abi = "cryptoVault()")]
    pub struct CryptoVaultCall;
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
    /// `delegateTransfer` function with signature
    /// `delegateTransfer(address,uint256,address)` and
    /// selector `0x9cd1a121`
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
        name = "delegateTransfer",
        abi = "delegateTransfer(address,uint256,address)"
    )]
    pub struct DelegateTransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub orig_sender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `delegatedFrom` function with signature
    /// `delegatedFrom()` and selector `0x26fe9951`
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
    #[ethcall(name = "delegatedFrom", abi = "delegatedFrom()")]
    pub struct DelegatedFromCall;
    ///Container type for all input parameters for the
    /// `forta` function with signature `forta()` and
    /// selector `0xd8670e1f`
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
    #[ethcall(name = "forta", abi = "forta()")]
    pub struct FortaCall;
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
    /// `owner` function with signature `owner()` and
    /// selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the
    /// `player` function with signature `player()` and
    /// selector `0x48db5f89`
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
    #[ethcall(name = "player", abi = "player()")]
    pub struct PlayerCall;
    ///Container type for all input parameters for the
    /// `renounceOwnership` function with signature
    /// `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
    ///Container type for all input parameters for the
    /// `transferOwnership` function with signature
    /// `transferOwnership(address)` and selector
    /// `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum DoubleEntryPointCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        CryptoVault(CryptoVaultCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DelegateTransfer(DelegateTransferCall),
        DelegatedFrom(DelegatedFromCall),
        Forta(FortaCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Name(NameCall),
        Owner(OwnerCall),
        Player(PlayerCall),
        RenounceOwnership(RenounceOwnershipCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for DoubleEntryPointCalls {
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
                <CryptoVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CryptoVault(decoded));
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
            if let Ok(decoded) =
                <DelegateTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DelegateTransfer(decoded));
            }
            if let Ok(decoded) =
                <DelegatedFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DelegatedFrom(decoded));
            }
            if let Ok(decoded) =
                <FortaCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Forta(decoded));
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
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PlayerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Player(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
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
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DoubleEntryPointCalls {
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
                Self::CryptoVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegatedFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Forta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Player(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DoubleEntryPointCalls {
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
                Self::CryptoVault(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Decimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegateTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegatedFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Forta(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Player(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Transfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for DoubleEntryPointCalls {
        fn from(value: AllowanceCall) -> Self { Self::Allowance(value) }
    }
    impl ::core::convert::From<ApproveCall> for DoubleEntryPointCalls {
        fn from(value: ApproveCall) -> Self { Self::Approve(value) }
    }
    impl ::core::convert::From<BalanceOfCall> for DoubleEntryPointCalls {
        fn from(value: BalanceOfCall) -> Self { Self::BalanceOf(value) }
    }
    impl ::core::convert::From<CryptoVaultCall> for DoubleEntryPointCalls {
        fn from(value: CryptoVaultCall) -> Self { Self::CryptoVault(value) }
    }
    impl ::core::convert::From<DecimalsCall> for DoubleEntryPointCalls {
        fn from(value: DecimalsCall) -> Self { Self::Decimals(value) }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for DoubleEntryPointCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DelegateTransferCall> for DoubleEntryPointCalls {
        fn from(value: DelegateTransferCall) -> Self {
            Self::DelegateTransfer(value)
        }
    }
    impl ::core::convert::From<DelegatedFromCall> for DoubleEntryPointCalls {
        fn from(value: DelegatedFromCall) -> Self { Self::DelegatedFrom(value) }
    }
    impl ::core::convert::From<FortaCall> for DoubleEntryPointCalls {
        fn from(value: FortaCall) -> Self { Self::Forta(value) }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for DoubleEntryPointCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<NameCall> for DoubleEntryPointCalls {
        fn from(value: NameCall) -> Self { Self::Name(value) }
    }
    impl ::core::convert::From<OwnerCall> for DoubleEntryPointCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<PlayerCall> for DoubleEntryPointCalls {
        fn from(value: PlayerCall) -> Self { Self::Player(value) }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DoubleEntryPointCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for DoubleEntryPointCalls {
        fn from(value: SymbolCall) -> Self { Self::Symbol(value) }
    }
    impl ::core::convert::From<TotalSupplyCall> for DoubleEntryPointCalls {
        fn from(value: TotalSupplyCall) -> Self { Self::TotalSupply(value) }
    }
    impl ::core::convert::From<TransferCall> for DoubleEntryPointCalls {
        fn from(value: TransferCall) -> Self { Self::Transfer(value) }
    }
    impl ::core::convert::From<TransferFromCall> for DoubleEntryPointCalls {
        fn from(value: TransferFromCall) -> Self { Self::TransferFrom(value) }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DoubleEntryPointCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
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
    /// `cryptoVault` function with signature
    /// `cryptoVault()` and selector `0xd444d8a0`
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
    pub struct CryptoVaultReturn(pub ::ethers::core::types::Address);
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
    /// `delegateTransfer` function with signature
    /// `delegateTransfer(address,uint256,address)` and
    /// selector `0x9cd1a121`
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
    pub struct DelegateTransferReturn(pub bool);
    ///Container type for all return fields from the
    /// `delegatedFrom` function with signature
    /// `delegatedFrom()` and selector `0x26fe9951`
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
    pub struct DelegatedFromReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `forta` function with signature `forta()` and
    /// selector `0xd8670e1f`
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
    pub struct FortaReturn(pub ::ethers::core::types::Address);
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
    /// `owner` function with signature `owner()` and
    /// selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `player` function with signature `player()` and
    /// selector `0x48db5f89`
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
    pub struct PlayerReturn(pub ::ethers::core::types::Address);
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
