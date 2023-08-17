pub use gnosis_safe::*;
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
pub mod gnosis_safe {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VERSION"),
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
                    ::std::borrow::ToOwned::to_owned("addOwnerWithThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addOwnerWithThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("approveHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approveHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hashToApprove"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("approvedHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvedHashes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("changeThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeThreshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("checkNSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkNSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requiredSignatures",
                                    ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("disableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("disableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevModule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                    ::std::borrow::ToOwned::to_owned("encodeTransactionData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeTransactionData",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execTransaction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execTransaction"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execTransactionFromModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModule",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "execTransactionFromModuleReturnData",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModuleReturnData",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainId"),
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
                    ::std::borrow::ToOwned::to_owned("getModulesPaginated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getModulesPaginated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pageSize"),
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
                                    name: ::std::borrow::ToOwned::to_owned("array"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("next"),
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
                    ::std::borrow::ToOwned::to_owned("getOwners"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOwners"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStorageAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStorageAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("offset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getThreshold"),
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
                    ::std::borrow::ToOwned::to_owned("getTransactionHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTransactionHash"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                (
                    ::std::borrow::ToOwned::to_owned("isOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                (
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                    ::std::borrow::ToOwned::to_owned("removeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("requiredTxGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requiredTxGas"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFallbackHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFallbackHandler"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("handler"),
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
                    ::std::borrow::ToOwned::to_owned("setGuard"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGuard"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("guard"),
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
                    ::std::borrow::ToOwned::to_owned("setup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owners"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackHandler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
                    ::std::borrow::ToOwned::to_owned("signedMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signedMessages"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldataPayload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("swapOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("AddedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddedOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApproveHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApproveHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approvedHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedFallbackHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangedFallbackHandler",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("handler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedGuard"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangedGuard"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("guard"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangedThreshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
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
                    ::std::borrow::ToOwned::to_owned("DisabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DisabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionFailure"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleFailure",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionSuccess"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("RemovedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemovedOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SafeReceived"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
                    ::std::borrow::ToOwned::to_owned("SafeSetup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SafeSetup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owners"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackHandler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SignMsg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GNOSISSAFE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01`\x04Ua0\x0B\x80a\0%`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xDCW`\x005`\xE0\x1C\x80c\xAF\xFE\xD0\xE0\x11a\x01\x02W\x80c\xE1\x9A\x9D\xD9\x11a\0\x95W\x80c\xF0\x8A\x03#\x11a\0dW\x80c\xF0\x8A\x03#\x14a\x06 W\x80c\xF6\x98\xDA%\x14a\x06@W\x80c\xF8\xDC]\xD9\x14a\x06UW\x80c\xFF\xA1\xADt\x14a\x06uWa\x02\x18V[\x80c\xE1\x9A\x9D\xD9\x14a\x05\xABW\x80c\xE3\x18\xB5+\x14a\x05\xCBW\x80c\xE7R5\xB8\x14a\x05\xEBW\x80c\xE8f7\xDB\x14a\x06\0Wa\x02\x18V[\x80c\xCC/\x84R\x11a\0\xD1W\x80c\xCC/\x84R\x14a\x05\x1DW\x80c\xD4\xD9\xBD\xCD\x14a\x05KW\x80c\xD8\xD1\x1Fx\x14a\x05kW\x80c\xE0\t\xCF\xDE\x14a\x05\x8BWa\x02\x18V[\x80c\xAF\xFE\xD0\xE0\x14a\x04\xA7W\x80c\xB4\xFA\xBA\t\x14a\x04\xBDW\x80c\xB6>\x80\r\x14a\x04\xDDW\x80c\xC4\xCA:\x9C\x14a\x04\xFDWa\x02\x18V[\x80cV$\xB2[\x11a\x01zW\x80cjv\x12\x02\x11a\x01IW\x80cjv\x12\x02\x14a\x04\x1AW\x80c}\x83)t\x14a\x04-W\x80c\x93O:\x11\x14a\x04eW\x80c\xA0\xE6~+\x14a\x04\x85Wa\x02\x18V[\x80cV$\xB2[\x14a\x03\x80W\x80cZ\xE6\xBD7\x14a\x03\xADW\x80ca\x0BY%\x14a\x03\xDAW\x80ciN\x80\xC3\x14a\x03\xFAWa\x02\x18V[\x80c/T\xBFn\x11a\x01\xB6W\x80c/T\xBFn\x14a\x02\xF5W\x80c4\x08\xE4p\x14a\x03\x15W\x80cF\x87!\xA7\x14a\x032W\x80cR)\x07?\x14a\x03RWa\x02\x18V[\x80c\rX/\x13\x14a\x02~W\x80c\x12\xFBh\xE0\x14a\x02\xA0W\x80c-\x9A\xD5=\x14a\x02\xC0Wa\x02\x18V[6a\x02\x18W`@Q4\x81R3\x90\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[4\x80\x15a\x02$W`\0\x80\xFD[P\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5\x80T\x80a\x02OW\0[6`\0\x8073``\x1B6R`\0\x80`\x146\x01`\0\x80\x85Z\xF1\x90P=`\0\x80>\x80a\x02xW=`\0\xFD[P=`\0\xF3[4\x80\x15a\x02\x8AW`\0\x80\xFD[Pa\x02\x9Ea\x02\x996`\x04a$\xA2V[a\x06\xA6V[\0[4\x80\x15a\x02\xACW`\0\x80\xFD[Pa\x02\x9Ea\x02\xBB6`\x04a%pV[a\x08\x06V[4\x80\x15a\x02\xCCW`\0\x80\xFD[Pa\x02\xE0a\x02\xDB6`\x04a%\xE4V[a\x0CqV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x01W`\0\x80\xFD[Pa\x02\xE0a\x03\x106`\x04a%\xE4V[a\x0C\xACV[4\x80\x15a\x03!W`\0\x80\xFD[PF[`@Q\x90\x81R` \x01a\x02\xECV[4\x80\x15a\x03>W`\0\x80\xFD[Pa\x02\xE0a\x03M6`\x04a&\x10V[a\x0C\xE4V[4\x80\x15a\x03^W`\0\x80\xFD[Pa\x03ra\x03m6`\x04a&\x10V[a\r\xBBV[`@Qa\x02\xEC\x92\x91\x90a&\xBFV[4\x80\x15a\x03\x8CW`\0\x80\xFD[Pa\x03\xA0a\x03\x9B6`\x04a&\xDAV[a\r\xF1V[`@Qa\x02\xEC\x91\x90a&\xFCV[4\x80\x15a\x03\xB9W`\0\x80\xFD[Pa\x03$a\x03\xC86`\x04a'\x0FV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xE6W`\0\x80\xFD[Pa\x02\x9Ea\x03\xF56`\x04a%\xE4V[a\x0EvV[4\x80\x15a\x04\x06W`\0\x80\xFD[Pa\x02\x9Ea\x04\x156`\x04a'\x0FV[a\x0F\xB8V[a\x02\xE0a\x04(6`\x04a'pV[a\x10PV[4\x80\x15a\x049W`\0\x80\xFD[Pa\x03$a\x04H6`\x04a$\xA2V[`\x08` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04qW`\0\x80\xFD[Pa\x02\x9Ea\x04\x806`\x04a(HV[a\x13\x99V[4\x80\x15a\x04\x91W`\0\x80\xFD[Pa\x04\x9Aa\x13\xE3V[`@Qa\x02\xEC\x91\x90a(\xF8V[4\x80\x15a\x04\xB3W`\0\x80\xFD[Pa\x03$`\x05T\x81V[4\x80\x15a\x04\xC9W`\0\x80\xFD[Pa\x02\x9Ea\x04\xD86`\x04a)\x0BV[a\x14\xD3V[4\x80\x15a\x04\xE9W`\0\x80\xFD[Pa\x02\x9Ea\x04\xF86`\x04a)ZV[a\x14\xF6V[4\x80\x15a\x05\tW`\0\x80\xFD[Pa\x03$a\x05\x186`\x04a*NV[a\x16\x17V[4\x80\x15a\x05)W`\0\x80\xFD[Pa\x05=a\x0586`\x04a$\xA2V[a\x16\xB1V[`@Qa\x02\xEC\x92\x91\x90a*\xBEV[4\x80\x15a\x05WW`\0\x80\xFD[Pa\x02\x9Ea\x05f6`\x04a'\x0FV[a\x17\xAAV[4\x80\x15a\x05wW`\0\x80\xFD[Pa\x03$a\x05\x866`\x04a*\xE8V[a\x18?V[4\x80\x15a\x05\x97W`\0\x80\xFD[Pa\x02\x9Ea\x05\xA66`\x04a+\xA8V[a\x18lV[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x02\x9Ea\x05\xC66`\x04a%\xE4V[a\x19\x9BV[4\x80\x15a\x05\xD7W`\0\x80\xFD[Pa\x02\x9Ea\x05\xE66`\x04a+\xE1V[a\x1A\0V[4\x80\x15a\x05\xF7W`\0\x80\xFD[P`\x04Ta\x03$V[4\x80\x15a\x06\x0CW`\0\x80\xFD[Pa\x03\xA0a\x06\x1B6`\x04a*\xE8V[a\x1B\xEFV[4\x80\x15a\x06,W`\0\x80\xFD[Pa\x02\x9Ea\x06;6`\x04a%\xE4V[a\x1C\xC8V[4\x80\x15a\x06LW`\0\x80\xFD[Pa\x03$a\x1D1V[4\x80\x15a\x06aW`\0\x80\xFD[Pa\x02\x9Ea\x06p6`\x04a,,V[a\x1D\x88V[4\x80\x15a\x06\x81W`\0\x80\xFD[Pa\x03\xA0`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\x12\xE32\xE3`\xDC\x1B\x81RP\x81V[a\x06\xAEa\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x06\xD0WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x06\xE5WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x15[a\x07\nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15a\x07BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,\x8CV[`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R`@\x81 \x80T\x93\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90\x93U`\x01\x83R\x83T\x90\x91\x16\x17\x90\x91U`\x03\x80T\x91a\x07\xAF\x83a,\xC1V[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x04T\x14a\x08\x02Wa\x08\x02\x81a\x0F\xB8V[PPV[a\x08\x11\x81`Aa\x1F4V[\x82Q\x10\x15a\x08IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03#`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`\0\x80\x80`\0\x80`\0[\x86\x81\x10\x15a\x0CeW`A\x81\x81\x02\x89\x01` \x81\x01Q`@\x82\x01Q\x91\x90\x92\x01Q`\xFF\x16\x95P\x90\x93P\x91P`\0\x84\x90\x03a\n$W\x91\x93P\x83\x91a\x08\x94\x87`Aa\x1F4V[\x82\x10\x15a\x08\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS021`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[\x87Qa\x08\xD8\x83` a\x1FpV[\x11\x15a\t\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[` \x82\x89\x01\x81\x01Q\x89Q\x90\x91a\t1\x90\x83\x90a\t+\x90\x87\x90a\x1FpV[\x90a\x1FpV[\x11\x15a\tgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS023`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`@Qc \xC1;\x0B`\xE0\x1B\x80\x82R\x8A\x85\x01` \x01\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c \xC1;\x0B\x90a\t\x9D\x90\x8F\x90\x86\x90`\x04\x01a,\xDAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDE\x91\x90a,\xFFV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\n\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x0C\x8D`\xDA\x1B`D\x82\x01R`d\x01a\x07\x01V[PPa\x0B\xCBV[\x83`\xFF\x16`\x01\x03a\n\xA6W\x91\x93P\x83\x913`\x01`\x01`\xA0\x1B\x03\x84\x16\x14\x80a\nmWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x08` \x90\x81R`@\x80\x83 \x8D\x84R\x90\x91R\x90 T\x15\x15[a\n\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS025`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[a\x0B\xCBV[`\x1E\x84`\xFF\x16\x11\x15a\x0BkW`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x8B\x90R`\x01\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04\x86a\x0B\x0B\x91\x90a-)V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x93\x90\x93R`\xFF\x90\x91\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0BZW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94Pa\x0B\xCBV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8C\x90R`\xFF\x86\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\xBEW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94P[\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x80\x15a\x0C\x05WP`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15\x15[\x80\x15a\x0C\x1BWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\x01\x14\x15[a\x0COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x1B`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[\x84\x95P\x80\x80a\x0C]\x90a,\xC1V[\x91PPa\x08SV[PPPPPPPPPPV[`\0`\x01`\x01`\x01`\xA0\x1B\x03\x83\x16\x14\x80\x15\x90a\x0C\xA6WP`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x16\x15\x15[\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x80\x15\x90a\x0C\xA6WPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15\x15\x90V[`\x003`\x01\x14\x80\x15\x90a\r\x0EWP3`\0\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[a\rBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x01a\x07\x01V[a\rO\x85\x85\x85\x85Za\x1F\x8CV[\x90P\x80\x15a\r\x87W`@Q3\x90\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8\x90`\0\x90\xA2a\r\xB3V[`@Q3\x90\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u\x90`\0\x90\xA2[\x94\x93PPPPV[`\0``a\r\xCB\x86\x86\x86\x86a\x0C\xE4V[\x91P`@Q` =\x01\x81\x01`@R=\x81R=`\0` \x83\x01>\x80\x91PP\x94P\x94\x92PPPV[```\0a\x0E\0\x83` a-BV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x17Wa\x0E\x17a$\xCEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0EAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0EnW\x84\x81\x01T` \x80\x83\x02\x84\x01\x01R\x80a\x0Ef\x81a,\xC1V[\x91PPa\x0EGV[P\x93\x92PPPV[a\x0E~a\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x0E\xA0WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x0E\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x16\x15a\x0F$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01` \x81\x81R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x81\x81R`@\x80\x82 \x80T\x94\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x94\x85\x16\x17\x90\x94U\x95\x90\x95R\x82T\x16\x84\x17\x90\x91UQ\x91\x82R\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x91\x01[`@Q\x80\x91\x03\x90\xA1PV[a\x0F\xC0a\x1E\xFBV[`\x03T\x81\x11\x15a\x0F\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a-YV[`\x01\x81\x10\x15a\x10\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x04\x81\x90U`@Q\x81\x81R\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x90` \x01a\x0F\xADV[`\0\x80`\0a\x10j\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E`\x05Ta\x1B\xEFV[`\x05\x80T\x91\x92P`\0a\x10|\x83a,\xC1V[\x90\x91UPP\x80Q` \x82\x01 \x91Pa\x10\x95\x82\x82\x86a\x13\x99V[P`\0a\x10\xC0\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x11FW\x80`\x01`\x01`\xA0\x1B\x03\x16cu\xF0\xBBR\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F3`@Q\x8Dc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x13\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a-\xB0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11AW=`\0\x80>=`\0\xFD[PPPP[a\x11ra\x11U\x8Aa\t\xC4a.uV[`?a\x11b\x8C`@a-BV[a\x11l\x91\x90a.\x88V[\x90a\x1F\xD3V[a\x11~\x90a\x01\xF4a.uV[Z\x10\x15a\x11\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x13`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`\0Z\x90Pa\x12&\x8F\x8F\x8F\x8F\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8E\x8C`\0\x14a\x12\x13W\x8Ea\x1F\x8CV[a\t\xC4Za\x12!\x91\x90a.\xAAV[a\x1F\x8CV[\x93Pa\x123Z\x82\x90a\x1F\xEAV[\x90P\x83\x80a\x12@WP\x89\x15\x15[\x80a\x12JWP\x87\x15\x15[a\x12~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS013`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\0\x88\x15a\x12\x96Wa\x12\x93\x82\x8B\x8B\x8B\x8Ba \x05V[\x90P[\x84\x15a\x12\xDAW`@\x80Q\x85\x81R` \x81\x01\x83\x90R\x7FD.q_bcF\xE8\xC5C\x81\0-\xA6\x14\xF6+\xEE\x8D'8e5\xB2R\x1E\xC8T\x08\x98Un\x91\x01`@Q\x80\x91\x03\x90\xA1a\x13\x14V[`@\x80Q\x85\x81R` \x81\x01\x83\x90R\x7F#B\x8B\x18\xAC\xFB>\xA6K\x08\xDC\x0C\x1D)n\xA9\xC0\x97\x02\xC0\x90\x83\xCARr\xE6M\x11[h}#\x91\x01`@Q\x80\x91\x03\x90\xA1[PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13\x88W`@Qc\x12d\xE2m`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R\x83\x15\x15`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x93'\x13h\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\x83W=`\0\x80>=`\0\xFD[PPPP[PP\x9B\x9APPPPPPPPPPPV[`\x04T\x80a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS001`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[a\x13\xDD\x84\x84\x84\x84a\x08\x06V[PPPPV[```\0`\x03T`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x01Wa\x14\x01a$\xCEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14*W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\0\x90\x81R`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0T\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14a\x14\xCBW\x80\x83\x83\x81Q\x81\x10a\x14\x8BWa\x14\x8Ba.\xBDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16`\0\x90\x81R`\x02\x90\x92R`@\x90\x91 T\x16\x81a\x14\xC3\x81a,\xC1V[\x92PPa\x14gV[P\x90\x92\x91PPV[`\0\x80\x82Q` \x84\x01\x85Z\xF4\x80`\0RP=` R=`\0`@>`@=\x01`\0\xFD[a\x154\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa!\x0B\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x15kWa\x15k\x84\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[a\x15\xAB\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\"\xF1\x92PPPV[\x81\x15a\x15\xC2Wa\x15\xC0\x82`\0`\x01\x86\x85a \x05V[P[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x14\x1D\xF8h\xA63\x1A\xF5(\xE3\x8C\x83\xB7\xAA\x03\xED\xC1\x9B\xE6n7\xAEg\xF9([\xF4\xF8\xE3\xC6\xA1\xA8\x8B\x8B\x8B\x8B\x89`@Qa\x16\x03\x95\x94\x93\x92\x91\x90a.\xD3V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[`\0\x80Z\x90Pa\x16`\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPPZa\x1F\x8CV[a\x16iW`\0\x80\xFD[`\0Za\x16v\x90\x83a.\xAAV[\x90P\x80`@Q` \x01a\x16\x8B\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x07\x01\x91`\x04\x01a&\xFCV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xCDWa\x16\xCDa$\xCEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xF6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x01` R`@\x81 T\x92\x94P\x91\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x179WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x17DWP\x84\x82\x10[\x15a\x17\x9CW\x80\x84\x83\x81Q\x81\x10a\x17\\Wa\x17\\a.\xBDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16`\0\x90\x81R`\x01\x90\x92R`@\x90\x91 T\x16\x81a\x17\x94\x81a,\xC1V[\x92PPa\x17\x17V[\x90\x83R\x91\x94\x91\x93P\x90\x91PPV[3`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x17\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x033`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[3`\0\x81\x81R`\x08` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 `\x01\x90UQ\x83\x91\x7F\xF2\xA0\xEB\x15dr\xD1D\x02U\xB0\xD7\xC1\xE1\x9C\xC0q\x15\xD1\x05\x1F\xE6\x05\xB0\xDC\xE6\x9A\xCF\xEC\x88M\x9C\x91\xA3PV[`\0a\x18T\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x1B\xEFV[\x80Q\x90` \x01 \x90P\x9B\x9APPPPPPPPPPPV[a\x18ta\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x18\x96WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x18\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x81\x16\x90\x82\x16\x14a\x19\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x80T\x88\x87\x16\x85R\x82\x85 \x80T\x91\x90\x97\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x96U\x92\x84\x90R\x82T\x90\x94\x16\x90\x91U\x91Q\x90\x81R\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[a\x19\xA3a\x1E\xFBV[\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8\x81\x81U`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\x11Q\x11i\x14Q[\xC0\x89\x1F\xF9\x04zl\xB3,\xF9\x02To\x83\x06d\x99\xBC\xF8\xBA3\xD25?\xA2\x90` \x01a\x19\x8FV[a\x1A\x08a\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1A*WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x1A?WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[a\x1A[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15a\x1A\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,\x8CV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1A\xB5WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1A\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1B%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x87\x87\x16\x80\x86R\x83\x86 \x80T\x92\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90U\x96\x8A\x16\x85R\x82\x85 \x80T\x82\x16\x90\x97\x17\x90\x96U\x92\x84\x90R\x82T\x90\x94\x16\x90\x91U\x91Q\x90\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[```\0\x7F\xBB\x83\x10\xD4\x866\x8D\xB6\xBDo\x84\x94\x02\xFD\xD7:\xD5=1kZK&D\xADn\xFE\x0F\x94\x12\x86\xD8`\0\x1B\x8D\x8D\x8D\x8D`@Qa\x1C)\x92\x91\x90a/?V[`@Q\x90\x81\x90\x03\x81 a\x1CO\x94\x93\x92\x91\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90` \x01a/OV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x19`\xF8\x1B`\x01`\xF8\x1Ba\x1C{a\x1D1V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x93\x84\x16` \x82\x01R\x92\x90\x91\x16`!\x83\x01R`\"\x82\x01R`B\x81\x01\x82\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x9B\x9APPPPPPPPPPPV[a\x1C\xD0a\x1E\xFBV[a\x1C\xF8\x81\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0\x90` \x01a\x0F\xADV[`\0\x7FG\xE7\x954\xA2E\x95.\x8B\x16\x89:3k\x85\xA3\xD9\xEA\x9F\xA8\xC5s\xF3\xD8\x03\xAF\xB9*yF\x92\x18F`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R0``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x1D\x90a\x1E\xFBV[\x80`\x01`\x03Ta\x1D\xA0\x91\x90a.\xAAV[\x10\x15a\x1D\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a-YV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1D\xE0WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1D\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1EPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T\x88\x86\x16\x84R\x91\x83 \x80T\x92\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x94U\x91\x81R\x82T\x90\x91\x16\x90\x91U`\x03\x80T\x91a\x1E\xA3\x83a/\xBEV[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x04T\x14a\x1E\xF6Wa\x1E\xF6\x81a\x0F\xB8V[PPPV[30\x14a\x1F2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[V[`\0\x82`\0\x03a\x1FFWP`\0a\x0C\xA6V[`\0a\x1FR\x83\x85a-BV[\x90P\x82a\x1F_\x85\x83a.\x88V[\x14a\x1FiW`\0\x80\xFD[\x93\x92PPPV[`\0\x80a\x1F}\x83\x85a.uV[\x90P\x83\x81\x10\x15a\x1FiW`\0\x80\xFD[`\0`\x01\x83`\x01\x81\x11\x15a\x1F\xA2Wa\x1F\xA2a-xV[\x03a\x1F\xBAW`\0\x80\x85Q` \x87\x01\x89\x86\xF4\x90Pa\x1F\xCAV[`\0\x80\x85Q` \x87\x01\x88\x8A\x87\xF1\x90P[\x95\x94PPPPPV[`\0\x81\x83\x10\x15a\x1F\xE3W\x81a\x1FiV[P\x90\x91\x90PV[`\0\x82\x82\x11\x15a\x1F\xF9W`\0\x80\xFD[`\0a\r\xB3\x83\x85a.\xAAV[`\0\x80`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a \x1DW\x82a \x1FV[2[\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16a \xB2Wa Q:\x86\x10a ?W:a AV[\x85[a K\x89\x89a\x1FpV[\x90a\x1F4V[`@Q\x90\x92P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa \xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS011`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[a!\x01V[a \xC0\x85a K\x89\x89a\x1FpV[\x91Pa \xCD\x84\x82\x84a#\xEBV[a!\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x99`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[P\x95\x94PPPPPV[`\x04T\x15a!CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3#\x03`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[\x81Q\x81\x11\x15a!dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a-YV[`\x01\x81\x10\x15a!\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\0[\x83Q\x81\x10\x15a\"\xBEW`\0\x84\x82\x81Q\x81\x10a!\xBFWa!\xBFa.\xBDV[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!\xF6WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\"\x0BWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[\x80\x15a\")WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[a\"EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15a\"}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,\x8CV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U\x80a\"\xB6\x81a,\xC1V[\x91PPa!\xA2V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01\x17\x90U\x90Q`\x03U`\x04UV[`\x01`\0\x81\x90R` R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/T`\x01`\x01`\xA0\x1B\x03\x16\x15a#[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x13\x03`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\0\x81\x90R` \x81\x90R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x08\x02Wa#\xB7\x82`\0\x83`\x01Za\x1F\x8CV[a\x08\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x03`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Q`\0\x93\x92\x91\x84\x91\x90\x82\x89a'\x10Z\x03\xF1=\x80\x15a$]W` \x81\x14a$eW`\0\x93Pa$pV[\x81\x93Pa$pV[`\0Q\x15\x82\x15\x17\x15\x93P[PPP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\x8FW`\0\x80\xFD[PV[\x805a$\x9D\x81a$zV[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xB5W`\0\x80\xFD[\x825a$\xC0\x81a$zV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a$\xF5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x0FWa%\x0Fa$\xCEV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a%7Wa%7a$\xCEV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a%PW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a%\x86W`\0\x80\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\xA4W`\0\x80\xFD[a%\xB0\x88\x83\x89\x01a$\xE4V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a%\xC6W`\0\x80\xFD[Pa%\xD3\x87\x82\x88\x01a$\xE4V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a%\xF6W`\0\x80\xFD[\x815a\x1Fi\x81a$zV[\x805`\x02\x81\x10a$\x9DW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&&W`\0\x80\xFD[\x845a&1\x81a$zV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&SW`\0\x80\xFD[a&_\x87\x82\x88\x01a$\xE4V[\x92PPa&n``\x86\x01a&\x01V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a&\x9FW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a&\x83V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\r\xB3`@\x83\x01\x84a&yV[`\0\x80`@\x83\x85\x03\x12\x15a&\xEDW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x81R`\0a\x1Fi` \x83\x01\x84a&yV[`\0` \x82\x84\x03\x12\x15a'!W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a':W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'QW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a'iW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01@\x8C\x8E\x03\x12\x15a'\x92W`\0\x80\xFD[a'\x9B\x8Ca$\x92V[\x9AP` \x8C\x015\x99P`\x01`\x01`@\x1B\x03\x80`@\x8E\x015\x11\x15a'\xBDW`\0\x80\xFD[a'\xCD\x8E`@\x8F\x015\x8F\x01a'(V[\x90\x9AP\x98Pa'\xDE``\x8E\x01a&\x01V[\x97P`\x80\x8D\x015\x96P`\xA0\x8D\x015\x95P`\xC0\x8D\x015\x94Pa(\x01`\xE0\x8E\x01a$\x92V[\x93Pa(\x10a\x01\0\x8E\x01a$\x92V[\x92P\x80a\x01 \x8E\x015\x11\x15a($W`\0\x80\xFD[Pa(6\x8Da\x01 \x8E\x015\x8E\x01a$\xE4V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`\0``\x84\x86\x03\x12\x15a(]W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a({W`\0\x80\xFD[a(\x87\x87\x83\x88\x01a$\xE4V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a(\x9DW`\0\x80\xFD[Pa(\xAA\x86\x82\x87\x01a$\xE4V[\x91PP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a(\xEDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a(\xC8V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x1Fi` \x83\x01\x84a(\xB4V[`\0\x80`@\x83\x85\x03\x12\x15a)\x1EW`\0\x80\xFD[\x825a))\x81a$zV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)DW`\0\x80\xFD[a)P\x85\x82\x86\x01a$\xE4V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x8B\x8D\x03\x12\x15a)zW`\0\x80\xFD[\x8A5`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a)\x91W`\0\x80\xFD[\x81\x8D\x01\x91P\x8D`\x1F\x83\x01\x12a)\xA5W`\0\x80\xFD[\x815\x81\x81\x11\x15a)\xB4W`\0\x80\xFD[\x8E` \x82`\x05\x1B\x85\x01\x01\x11\x15a)\xC9W`\0\x80\xFD[` \x83\x81\x01\x9DP\x90\x9BP\x8D\x015\x99Pa)\xE4`@\x8E\x01a$\x92V[\x98P``\x8D\x015\x91P\x80\x82\x11\x15a)\xFAW`\0\x80\xFD[Pa*\x07\x8D\x82\x8E\x01a'(V[\x90\x97P\x95Pa*\x1A\x90P`\x80\x8C\x01a$\x92V[\x93Pa*(`\xA0\x8C\x01a$\x92V[\x92P`\xC0\x8B\x015\x91Pa*=`\xE0\x8C\x01a$\x92V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a*fW`\0\x80\xFD[\x855a*q\x81a$zV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x93W`\0\x80\xFD[a*\x9F\x88\x82\x89\x01a'(V[\x90\x94P\x92Pa*\xB2\x90P``\x87\x01a&\x01V[\x90P\x92\x95P\x92\x95\x90\x93PV[`@\x81R`\0a*\xD1`@\x83\x01\x85a(\xB4V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01@\x8C\x8E\x03\x12\x15a+\nW`\0\x80\xFD[\x8B5a+\x15\x81a$zV[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+7W`\0\x80\xFD[a+C\x8E\x82\x8F\x01a'(V[\x90\x9AP\x98Pa+V\x90P``\x8D\x01a&\x01V[\x96P`\x80\x8C\x015\x95P`\xA0\x8C\x015\x94P`\xC0\x8C\x015\x93P`\xE0\x8C\x015a+{\x81a$zV[\x92Pa\x01\0\x8C\x015a+\x8C\x81a$zV[\x80\x92PPa\x01 \x8C\x015\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`@\x83\x85\x03\x12\x15a+\xBBW`\0\x80\xFD[\x825a+\xC6\x81a$zV[\x91P` \x83\x015a+\xD6\x81a$zV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a+\xF6W`\0\x80\xFD[\x835a,\x01\x81a$zV[\x92P` \x84\x015a,\x11\x81a$zV[\x91P`@\x84\x015a,!\x81a$zV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a,AW`\0\x80\xFD[\x835a,L\x81a$zV[\x92P` \x84\x015a,\\\x81a$zV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x80\x82R`\x05\x90\x82\x01RdGS203`\xD8\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x05\x90\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a,\xD3Wa,\xD3a,\xABV[P`\x01\x01\x90V[`@\x81R`\0a,\xED`@\x83\x01\x85a&yV[\x82\x81\x03` \x84\x01Ra\x1F\xCA\x81\x85a&yV[`\0` \x82\x84\x03\x12\x15a-\x11W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1FiW`\0\x80\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xA6Wa\x0C\xA6a,\xABV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\xA6Wa\x0C\xA6a,\xABV[` \x80\x82R`\x05\x90\x82\x01RdGS201`\xD8\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a-\xACWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x81R` \x81\x01\x8C\x90Ra\x01``@\x82\x01\x81\x90R\x81\x01\x8A\x90R`\0a\x01\x80\x8B\x8D\x82\x85\x017`\0\x83\x8D\x01\x82\x01R`\x1F\x8C\x01`\x1F\x19\x16\x83\x01a-\xFC``\x85\x01\x8Da-\x8EV[\x8A`\x80\x85\x01R\x89`\xA0\x85\x01R\x88`\xC0\x85\x01Ra.#`\xE0\x85\x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x87\x16a\x01\0\x85\x01R\x81\x84\x82\x03\x01a\x01 \x85\x01Ra.J\x82\x82\x01\x87a&yV[\x92PPPa.da\x01@\x83\x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x9D\x9CPPPPPPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0C\xA6Wa\x0C\xA6a,\xABV[`\0\x82a.\xA5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0C\xA6Wa\x0C\xA6a,\xABV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x80\x80\x82R\x81\x01\x85\x90R`\0\x86`\xA0\x83\x01\x82[\x88\x81\x10\x15a/\x16W\x825a.\xF9\x81a$zV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xE6V[P` \x84\x01\x96\x90\x96RPP`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`@\x82\x01R\x91\x16``\x90\x91\x01R\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x8B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16` \x83\x01R`@\x82\x01\x8B\x90R``\x82\x01\x8A\x90Ra\x01`\x82\x01\x90a/\x83`\x80\x84\x01\x8Ba-\x8EV[`\xA0\x83\x01\x98\x90\x98R`\xC0\x82\x01\x96\x90\x96R`\xE0\x81\x01\x94\x90\x94R\x91\x85\x16a\x01\0\x84\x01R\x90\x93\x16a\x01 \x82\x01Ra\x01@\x01\x91\x90\x91R\x95\x94PPPPPV[`\0\x81a/\xCDWa/\xCDa,\xABV[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 ~X\xEBL8\xD8MB\xCB2x>\xB5c\xC2y%\rv\xA3^\xAF\xF1\x02\xA3\x8C`\x04 R59dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static GNOSISSAFE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xDCW`\x005`\xE0\x1C\x80c\xAF\xFE\xD0\xE0\x11a\x01\x02W\x80c\xE1\x9A\x9D\xD9\x11a\0\x95W\x80c\xF0\x8A\x03#\x11a\0dW\x80c\xF0\x8A\x03#\x14a\x06 W\x80c\xF6\x98\xDA%\x14a\x06@W\x80c\xF8\xDC]\xD9\x14a\x06UW\x80c\xFF\xA1\xADt\x14a\x06uWa\x02\x18V[\x80c\xE1\x9A\x9D\xD9\x14a\x05\xABW\x80c\xE3\x18\xB5+\x14a\x05\xCBW\x80c\xE7R5\xB8\x14a\x05\xEBW\x80c\xE8f7\xDB\x14a\x06\0Wa\x02\x18V[\x80c\xCC/\x84R\x11a\0\xD1W\x80c\xCC/\x84R\x14a\x05\x1DW\x80c\xD4\xD9\xBD\xCD\x14a\x05KW\x80c\xD8\xD1\x1Fx\x14a\x05kW\x80c\xE0\t\xCF\xDE\x14a\x05\x8BWa\x02\x18V[\x80c\xAF\xFE\xD0\xE0\x14a\x04\xA7W\x80c\xB4\xFA\xBA\t\x14a\x04\xBDW\x80c\xB6>\x80\r\x14a\x04\xDDW\x80c\xC4\xCA:\x9C\x14a\x04\xFDWa\x02\x18V[\x80cV$\xB2[\x11a\x01zW\x80cjv\x12\x02\x11a\x01IW\x80cjv\x12\x02\x14a\x04\x1AW\x80c}\x83)t\x14a\x04-W\x80c\x93O:\x11\x14a\x04eW\x80c\xA0\xE6~+\x14a\x04\x85Wa\x02\x18V[\x80cV$\xB2[\x14a\x03\x80W\x80cZ\xE6\xBD7\x14a\x03\xADW\x80ca\x0BY%\x14a\x03\xDAW\x80ciN\x80\xC3\x14a\x03\xFAWa\x02\x18V[\x80c/T\xBFn\x11a\x01\xB6W\x80c/T\xBFn\x14a\x02\xF5W\x80c4\x08\xE4p\x14a\x03\x15W\x80cF\x87!\xA7\x14a\x032W\x80cR)\x07?\x14a\x03RWa\x02\x18V[\x80c\rX/\x13\x14a\x02~W\x80c\x12\xFBh\xE0\x14a\x02\xA0W\x80c-\x9A\xD5=\x14a\x02\xC0Wa\x02\x18V[6a\x02\x18W`@Q4\x81R3\x90\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[4\x80\x15a\x02$W`\0\x80\xFD[P\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5\x80T\x80a\x02OW\0[6`\0\x8073``\x1B6R`\0\x80`\x146\x01`\0\x80\x85Z\xF1\x90P=`\0\x80>\x80a\x02xW=`\0\xFD[P=`\0\xF3[4\x80\x15a\x02\x8AW`\0\x80\xFD[Pa\x02\x9Ea\x02\x996`\x04a$\xA2V[a\x06\xA6V[\0[4\x80\x15a\x02\xACW`\0\x80\xFD[Pa\x02\x9Ea\x02\xBB6`\x04a%pV[a\x08\x06V[4\x80\x15a\x02\xCCW`\0\x80\xFD[Pa\x02\xE0a\x02\xDB6`\x04a%\xE4V[a\x0CqV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x01W`\0\x80\xFD[Pa\x02\xE0a\x03\x106`\x04a%\xE4V[a\x0C\xACV[4\x80\x15a\x03!W`\0\x80\xFD[PF[`@Q\x90\x81R` \x01a\x02\xECV[4\x80\x15a\x03>W`\0\x80\xFD[Pa\x02\xE0a\x03M6`\x04a&\x10V[a\x0C\xE4V[4\x80\x15a\x03^W`\0\x80\xFD[Pa\x03ra\x03m6`\x04a&\x10V[a\r\xBBV[`@Qa\x02\xEC\x92\x91\x90a&\xBFV[4\x80\x15a\x03\x8CW`\0\x80\xFD[Pa\x03\xA0a\x03\x9B6`\x04a&\xDAV[a\r\xF1V[`@Qa\x02\xEC\x91\x90a&\xFCV[4\x80\x15a\x03\xB9W`\0\x80\xFD[Pa\x03$a\x03\xC86`\x04a'\x0FV[`\x07` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xE6W`\0\x80\xFD[Pa\x02\x9Ea\x03\xF56`\x04a%\xE4V[a\x0EvV[4\x80\x15a\x04\x06W`\0\x80\xFD[Pa\x02\x9Ea\x04\x156`\x04a'\x0FV[a\x0F\xB8V[a\x02\xE0a\x04(6`\x04a'pV[a\x10PV[4\x80\x15a\x049W`\0\x80\xFD[Pa\x03$a\x04H6`\x04a$\xA2V[`\x08` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04qW`\0\x80\xFD[Pa\x02\x9Ea\x04\x806`\x04a(HV[a\x13\x99V[4\x80\x15a\x04\x91W`\0\x80\xFD[Pa\x04\x9Aa\x13\xE3V[`@Qa\x02\xEC\x91\x90a(\xF8V[4\x80\x15a\x04\xB3W`\0\x80\xFD[Pa\x03$`\x05T\x81V[4\x80\x15a\x04\xC9W`\0\x80\xFD[Pa\x02\x9Ea\x04\xD86`\x04a)\x0BV[a\x14\xD3V[4\x80\x15a\x04\xE9W`\0\x80\xFD[Pa\x02\x9Ea\x04\xF86`\x04a)ZV[a\x14\xF6V[4\x80\x15a\x05\tW`\0\x80\xFD[Pa\x03$a\x05\x186`\x04a*NV[a\x16\x17V[4\x80\x15a\x05)W`\0\x80\xFD[Pa\x05=a\x0586`\x04a$\xA2V[a\x16\xB1V[`@Qa\x02\xEC\x92\x91\x90a*\xBEV[4\x80\x15a\x05WW`\0\x80\xFD[Pa\x02\x9Ea\x05f6`\x04a'\x0FV[a\x17\xAAV[4\x80\x15a\x05wW`\0\x80\xFD[Pa\x03$a\x05\x866`\x04a*\xE8V[a\x18?V[4\x80\x15a\x05\x97W`\0\x80\xFD[Pa\x02\x9Ea\x05\xA66`\x04a+\xA8V[a\x18lV[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x02\x9Ea\x05\xC66`\x04a%\xE4V[a\x19\x9BV[4\x80\x15a\x05\xD7W`\0\x80\xFD[Pa\x02\x9Ea\x05\xE66`\x04a+\xE1V[a\x1A\0V[4\x80\x15a\x05\xF7W`\0\x80\xFD[P`\x04Ta\x03$V[4\x80\x15a\x06\x0CW`\0\x80\xFD[Pa\x03\xA0a\x06\x1B6`\x04a*\xE8V[a\x1B\xEFV[4\x80\x15a\x06,W`\0\x80\xFD[Pa\x02\x9Ea\x06;6`\x04a%\xE4V[a\x1C\xC8V[4\x80\x15a\x06LW`\0\x80\xFD[Pa\x03$a\x1D1V[4\x80\x15a\x06aW`\0\x80\xFD[Pa\x02\x9Ea\x06p6`\x04a,,V[a\x1D\x88V[4\x80\x15a\x06\x81W`\0\x80\xFD[Pa\x03\xA0`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\x12\xE32\xE3`\xDC\x1B\x81RP\x81V[a\x06\xAEa\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x06\xD0WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x06\xE5WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x15[a\x07\nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15a\x07BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,\x8CV[`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R`@\x81 \x80T\x93\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90\x93U`\x01\x83R\x83T\x90\x91\x16\x17\x90\x91U`\x03\x80T\x91a\x07\xAF\x83a,\xC1V[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x04T\x14a\x08\x02Wa\x08\x02\x81a\x0F\xB8V[PPV[a\x08\x11\x81`Aa\x1F4V[\x82Q\x10\x15a\x08IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03#`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`\0\x80\x80`\0\x80`\0[\x86\x81\x10\x15a\x0CeW`A\x81\x81\x02\x89\x01` \x81\x01Q`@\x82\x01Q\x91\x90\x92\x01Q`\xFF\x16\x95P\x90\x93P\x91P`\0\x84\x90\x03a\n$W\x91\x93P\x83\x91a\x08\x94\x87`Aa\x1F4V[\x82\x10\x15a\x08\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS021`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[\x87Qa\x08\xD8\x83` a\x1FpV[\x11\x15a\t\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[` \x82\x89\x01\x81\x01Q\x89Q\x90\x91a\t1\x90\x83\x90a\t+\x90\x87\x90a\x1FpV[\x90a\x1FpV[\x11\x15a\tgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS023`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`@Qc \xC1;\x0B`\xE0\x1B\x80\x82R\x8A\x85\x01` \x01\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c \xC1;\x0B\x90a\t\x9D\x90\x8F\x90\x86\x90`\x04\x01a,\xDAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDE\x91\x90a,\xFFV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\n\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x0C\x8D`\xDA\x1B`D\x82\x01R`d\x01a\x07\x01V[PPa\x0B\xCBV[\x83`\xFF\x16`\x01\x03a\n\xA6W\x91\x93P\x83\x913`\x01`\x01`\xA0\x1B\x03\x84\x16\x14\x80a\nmWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x08` \x90\x81R`@\x80\x83 \x8D\x84R\x90\x91R\x90 T\x15\x15[a\n\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS025`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[a\x0B\xCBV[`\x1E\x84`\xFF\x16\x11\x15a\x0BkW`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x8B\x90R`\x01\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04\x86a\x0B\x0B\x91\x90a-)V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x93\x90\x93R`\xFF\x90\x91\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0BZW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94Pa\x0B\xCBV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8C\x90R`\xFF\x86\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\xBEW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94P[\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x80\x15a\x0C\x05WP`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15\x15[\x80\x15a\x0C\x1BWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\x01\x14\x15[a\x0COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x1B`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[\x84\x95P\x80\x80a\x0C]\x90a,\xC1V[\x91PPa\x08SV[PPPPPPPPPPV[`\0`\x01`\x01`\x01`\xA0\x1B\x03\x83\x16\x14\x80\x15\x90a\x0C\xA6WP`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x16\x15\x15[\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x80\x15\x90a\x0C\xA6WPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15\x15\x90V[`\x003`\x01\x14\x80\x15\x90a\r\x0EWP3`\0\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[a\rBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x01a\x07\x01V[a\rO\x85\x85\x85\x85Za\x1F\x8CV[\x90P\x80\x15a\r\x87W`@Q3\x90\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8\x90`\0\x90\xA2a\r\xB3V[`@Q3\x90\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u\x90`\0\x90\xA2[\x94\x93PPPPV[`\0``a\r\xCB\x86\x86\x86\x86a\x0C\xE4V[\x91P`@Q` =\x01\x81\x01`@R=\x81R=`\0` \x83\x01>\x80\x91PP\x94P\x94\x92PPPV[```\0a\x0E\0\x83` a-BV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x17Wa\x0E\x17a$\xCEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0EAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0EnW\x84\x81\x01T` \x80\x83\x02\x84\x01\x01R\x80a\x0Ef\x81a,\xC1V[\x91PPa\x0EGV[P\x93\x92PPPV[a\x0E~a\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x0E\xA0WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x0E\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x16\x15a\x0F$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01` \x81\x81R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x81\x81R`@\x80\x82 \x80T\x94\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x94\x85\x16\x17\x90\x94U\x95\x90\x95R\x82T\x16\x84\x17\x90\x91UQ\x91\x82R\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x91\x01[`@Q\x80\x91\x03\x90\xA1PV[a\x0F\xC0a\x1E\xFBV[`\x03T\x81\x11\x15a\x0F\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a-YV[`\x01\x81\x10\x15a\x10\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x04\x81\x90U`@Q\x81\x81R\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x90` \x01a\x0F\xADV[`\0\x80`\0a\x10j\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E`\x05Ta\x1B\xEFV[`\x05\x80T\x91\x92P`\0a\x10|\x83a,\xC1V[\x90\x91UPP\x80Q` \x82\x01 \x91Pa\x10\x95\x82\x82\x86a\x13\x99V[P`\0a\x10\xC0\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x11FW\x80`\x01`\x01`\xA0\x1B\x03\x16cu\xF0\xBBR\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F3`@Q\x8Dc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x13\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a-\xB0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11AW=`\0\x80>=`\0\xFD[PPPP[a\x11ra\x11U\x8Aa\t\xC4a.uV[`?a\x11b\x8C`@a-BV[a\x11l\x91\x90a.\x88V[\x90a\x1F\xD3V[a\x11~\x90a\x01\xF4a.uV[Z\x10\x15a\x11\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x13`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`\0Z\x90Pa\x12&\x8F\x8F\x8F\x8F\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8E\x8C`\0\x14a\x12\x13W\x8Ea\x1F\x8CV[a\t\xC4Za\x12!\x91\x90a.\xAAV[a\x1F\x8CV[\x93Pa\x123Z\x82\x90a\x1F\xEAV[\x90P\x83\x80a\x12@WP\x89\x15\x15[\x80a\x12JWP\x87\x15\x15[a\x12~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS013`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\0\x88\x15a\x12\x96Wa\x12\x93\x82\x8B\x8B\x8B\x8Ba \x05V[\x90P[\x84\x15a\x12\xDAW`@\x80Q\x85\x81R` \x81\x01\x83\x90R\x7FD.q_bcF\xE8\xC5C\x81\0-\xA6\x14\xF6+\xEE\x8D'8e5\xB2R\x1E\xC8T\x08\x98Un\x91\x01`@Q\x80\x91\x03\x90\xA1a\x13\x14V[`@\x80Q\x85\x81R` \x81\x01\x83\x90R\x7F#B\x8B\x18\xAC\xFB>\xA6K\x08\xDC\x0C\x1D)n\xA9\xC0\x97\x02\xC0\x90\x83\xCARr\xE6M\x11[h}#\x91\x01`@Q\x80\x91\x03\x90\xA1[PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13\x88W`@Qc\x12d\xE2m`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R\x83\x15\x15`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x93'\x13h\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\x83W=`\0\x80>=`\0\xFD[PPPP[PP\x9B\x9APPPPPPPPPPPV[`\x04T\x80a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS001`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[a\x13\xDD\x84\x84\x84\x84a\x08\x06V[PPPPV[```\0`\x03T`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x01Wa\x14\x01a$\xCEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14*W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\0\x90\x81R`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0T\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14a\x14\xCBW\x80\x83\x83\x81Q\x81\x10a\x14\x8BWa\x14\x8Ba.\xBDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16`\0\x90\x81R`\x02\x90\x92R`@\x90\x91 T\x16\x81a\x14\xC3\x81a,\xC1V[\x92PPa\x14gV[P\x90\x92\x91PPV[`\0\x80\x82Q` \x84\x01\x85Z\xF4\x80`\0RP=` R=`\0`@>`@=\x01`\0\xFD[a\x154\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa!\x0B\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x15kWa\x15k\x84\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[a\x15\xAB\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\"\xF1\x92PPPV[\x81\x15a\x15\xC2Wa\x15\xC0\x82`\0`\x01\x86\x85a \x05V[P[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x14\x1D\xF8h\xA63\x1A\xF5(\xE3\x8C\x83\xB7\xAA\x03\xED\xC1\x9B\xE6n7\xAEg\xF9([\xF4\xF8\xE3\xC6\xA1\xA8\x8B\x8B\x8B\x8B\x89`@Qa\x16\x03\x95\x94\x93\x92\x91\x90a.\xD3V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[`\0\x80Z\x90Pa\x16`\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPPZa\x1F\x8CV[a\x16iW`\0\x80\xFD[`\0Za\x16v\x90\x83a.\xAAV[\x90P\x80`@Q` \x01a\x16\x8B\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x07\x01\x91`\x04\x01a&\xFCV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xCDWa\x16\xCDa$\xCEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xF6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x01` R`@\x81 T\x92\x94P\x91\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x179WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x17DWP\x84\x82\x10[\x15a\x17\x9CW\x80\x84\x83\x81Q\x81\x10a\x17\\Wa\x17\\a.\xBDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16`\0\x90\x81R`\x01\x90\x92R`@\x90\x91 T\x16\x81a\x17\x94\x81a,\xC1V[\x92PPa\x17\x17V[\x90\x83R\x91\x94\x91\x93P\x90\x91PPV[3`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x17\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x033`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[3`\0\x81\x81R`\x08` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 `\x01\x90UQ\x83\x91\x7F\xF2\xA0\xEB\x15dr\xD1D\x02U\xB0\xD7\xC1\xE1\x9C\xC0q\x15\xD1\x05\x1F\xE6\x05\xB0\xDC\xE6\x9A\xCF\xEC\x88M\x9C\x91\xA3PV[`\0a\x18T\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x1B\xEFV[\x80Q\x90` \x01 \x90P\x9B\x9APPPPPPPPPPPV[a\x18ta\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x18\x96WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x18\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x81\x16\x90\x82\x16\x14a\x19\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x80T\x88\x87\x16\x85R\x82\x85 \x80T\x91\x90\x97\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x96U\x92\x84\x90R\x82T\x90\x94\x16\x90\x91U\x91Q\x90\x81R\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[a\x19\xA3a\x1E\xFBV[\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8\x81\x81U`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\x11Q\x11i\x14Q[\xC0\x89\x1F\xF9\x04zl\xB3,\xF9\x02To\x83\x06d\x99\xBC\xF8\xBA3\xD25?\xA2\x90` \x01a\x19\x8FV[a\x1A\x08a\x1E\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1A*WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x1A?WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[a\x1A[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15a\x1A\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,\x8CV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1A\xB5WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1A\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1B%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x87\x87\x16\x80\x86R\x83\x86 \x80T\x92\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90U\x96\x8A\x16\x85R\x82\x85 \x80T\x82\x16\x90\x97\x17\x90\x96U\x92\x84\x90R\x82T\x90\x94\x16\x90\x91U\x91Q\x90\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[```\0\x7F\xBB\x83\x10\xD4\x866\x8D\xB6\xBDo\x84\x94\x02\xFD\xD7:\xD5=1kZK&D\xADn\xFE\x0F\x94\x12\x86\xD8`\0\x1B\x8D\x8D\x8D\x8D`@Qa\x1C)\x92\x91\x90a/?V[`@Q\x90\x81\x90\x03\x81 a\x1CO\x94\x93\x92\x91\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90` \x01a/OV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x19`\xF8\x1B`\x01`\xF8\x1Ba\x1C{a\x1D1V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x93\x84\x16` \x82\x01R\x92\x90\x91\x16`!\x83\x01R`\"\x82\x01R`B\x81\x01\x82\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x9B\x9APPPPPPPPPPPV[a\x1C\xD0a\x1E\xFBV[a\x1C\xF8\x81\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0\x90` \x01a\x0F\xADV[`\0\x7FG\xE7\x954\xA2E\x95.\x8B\x16\x89:3k\x85\xA3\xD9\xEA\x9F\xA8\xC5s\xF3\xD8\x03\xAF\xB9*yF\x92\x18F`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R0``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x1D\x90a\x1E\xFBV[\x80`\x01`\x03Ta\x1D\xA0\x91\x90a.\xAAV[\x10\x15a\x1D\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a-YV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1D\xE0WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1D\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1EPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T\x88\x86\x16\x84R\x91\x83 \x80T\x92\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x94U\x91\x81R\x82T\x90\x91\x16\x90\x91U`\x03\x80T\x91a\x1E\xA3\x83a/\xBEV[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x04T\x14a\x1E\xF6Wa\x1E\xF6\x81a\x0F\xB8V[PPPV[30\x14a\x1F2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[V[`\0\x82`\0\x03a\x1FFWP`\0a\x0C\xA6V[`\0a\x1FR\x83\x85a-BV[\x90P\x82a\x1F_\x85\x83a.\x88V[\x14a\x1FiW`\0\x80\xFD[\x93\x92PPPV[`\0\x80a\x1F}\x83\x85a.uV[\x90P\x83\x81\x10\x15a\x1FiW`\0\x80\xFD[`\0`\x01\x83`\x01\x81\x11\x15a\x1F\xA2Wa\x1F\xA2a-xV[\x03a\x1F\xBAW`\0\x80\x85Q` \x87\x01\x89\x86\xF4\x90Pa\x1F\xCAV[`\0\x80\x85Q` \x87\x01\x88\x8A\x87\xF1\x90P[\x95\x94PPPPPV[`\0\x81\x83\x10\x15a\x1F\xE3W\x81a\x1FiV[P\x90\x91\x90PV[`\0\x82\x82\x11\x15a\x1F\xF9W`\0\x80\xFD[`\0a\r\xB3\x83\x85a.\xAAV[`\0\x80`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a \x1DW\x82a \x1FV[2[\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16a \xB2Wa Q:\x86\x10a ?W:a AV[\x85[a K\x89\x89a\x1FpV[\x90a\x1F4V[`@Q\x90\x92P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa \xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS011`\xD8\x1B`D\x82\x01R`d\x01a\x07\x01V[a!\x01V[a \xC0\x85a K\x89\x89a\x1FpV[\x91Pa \xCD\x84\x82\x84a#\xEBV[a!\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x99`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[P\x95\x94PPPPPV[`\x04T\x15a!CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3#\x03`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[\x81Q\x81\x11\x15a!dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a-YV[`\x01\x81\x10\x15a!\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\0[\x83Q\x81\x10\x15a\"\xBEW`\0\x84\x82\x81Q\x81\x10a!\xBFWa!\xBFa.\xBDV[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!\xF6WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\"\x0BWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[\x80\x15a\")WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[a\"EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,mV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16\x15a\"}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x01\x90a,\x8CV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U\x80a\"\xB6\x81a,\xC1V[\x91PPa!\xA2V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01\x17\x90U\x90Q`\x03U`\x04UV[`\x01`\0\x81\x90R` R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/T`\x01`\x01`\xA0\x1B\x03\x16\x15a#[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x13\x03`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`\x01`\0\x81\x90R` \x81\x90R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x08\x02Wa#\xB7\x82`\0\x83`\x01Za\x1F\x8CV[a\x08\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x03`\xDC\x1B`D\x82\x01R`d\x01a\x07\x01V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Q`\0\x93\x92\x91\x84\x91\x90\x82\x89a'\x10Z\x03\xF1=\x80\x15a$]W` \x81\x14a$eW`\0\x93Pa$pV[\x81\x93Pa$pV[`\0Q\x15\x82\x15\x17\x15\x93P[PPP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\x8FW`\0\x80\xFD[PV[\x805a$\x9D\x81a$zV[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xB5W`\0\x80\xFD[\x825a$\xC0\x81a$zV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a$\xF5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x0FWa%\x0Fa$\xCEV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a%7Wa%7a$\xCEV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a%PW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a%\x86W`\0\x80\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\xA4W`\0\x80\xFD[a%\xB0\x88\x83\x89\x01a$\xE4V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a%\xC6W`\0\x80\xFD[Pa%\xD3\x87\x82\x88\x01a$\xE4V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a%\xF6W`\0\x80\xFD[\x815a\x1Fi\x81a$zV[\x805`\x02\x81\x10a$\x9DW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&&W`\0\x80\xFD[\x845a&1\x81a$zV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&SW`\0\x80\xFD[a&_\x87\x82\x88\x01a$\xE4V[\x92PPa&n``\x86\x01a&\x01V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a&\x9FW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a&\x83V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\r\xB3`@\x83\x01\x84a&yV[`\0\x80`@\x83\x85\x03\x12\x15a&\xEDW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x81R`\0a\x1Fi` \x83\x01\x84a&yV[`\0` \x82\x84\x03\x12\x15a'!W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a':W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'QW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a'iW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01@\x8C\x8E\x03\x12\x15a'\x92W`\0\x80\xFD[a'\x9B\x8Ca$\x92V[\x9AP` \x8C\x015\x99P`\x01`\x01`@\x1B\x03\x80`@\x8E\x015\x11\x15a'\xBDW`\0\x80\xFD[a'\xCD\x8E`@\x8F\x015\x8F\x01a'(V[\x90\x9AP\x98Pa'\xDE``\x8E\x01a&\x01V[\x97P`\x80\x8D\x015\x96P`\xA0\x8D\x015\x95P`\xC0\x8D\x015\x94Pa(\x01`\xE0\x8E\x01a$\x92V[\x93Pa(\x10a\x01\0\x8E\x01a$\x92V[\x92P\x80a\x01 \x8E\x015\x11\x15a($W`\0\x80\xFD[Pa(6\x8Da\x01 \x8E\x015\x8E\x01a$\xE4V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`\0``\x84\x86\x03\x12\x15a(]W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a({W`\0\x80\xFD[a(\x87\x87\x83\x88\x01a$\xE4V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a(\x9DW`\0\x80\xFD[Pa(\xAA\x86\x82\x87\x01a$\xE4V[\x91PP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a(\xEDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a(\xC8V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x1Fi` \x83\x01\x84a(\xB4V[`\0\x80`@\x83\x85\x03\x12\x15a)\x1EW`\0\x80\xFD[\x825a))\x81a$zV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)DW`\0\x80\xFD[a)P\x85\x82\x86\x01a$\xE4V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x8B\x8D\x03\x12\x15a)zW`\0\x80\xFD[\x8A5`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a)\x91W`\0\x80\xFD[\x81\x8D\x01\x91P\x8D`\x1F\x83\x01\x12a)\xA5W`\0\x80\xFD[\x815\x81\x81\x11\x15a)\xB4W`\0\x80\xFD[\x8E` \x82`\x05\x1B\x85\x01\x01\x11\x15a)\xC9W`\0\x80\xFD[` \x83\x81\x01\x9DP\x90\x9BP\x8D\x015\x99Pa)\xE4`@\x8E\x01a$\x92V[\x98P``\x8D\x015\x91P\x80\x82\x11\x15a)\xFAW`\0\x80\xFD[Pa*\x07\x8D\x82\x8E\x01a'(V[\x90\x97P\x95Pa*\x1A\x90P`\x80\x8C\x01a$\x92V[\x93Pa*(`\xA0\x8C\x01a$\x92V[\x92P`\xC0\x8B\x015\x91Pa*=`\xE0\x8C\x01a$\x92V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a*fW`\0\x80\xFD[\x855a*q\x81a$zV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x93W`\0\x80\xFD[a*\x9F\x88\x82\x89\x01a'(V[\x90\x94P\x92Pa*\xB2\x90P``\x87\x01a&\x01V[\x90P\x92\x95P\x92\x95\x90\x93PV[`@\x81R`\0a*\xD1`@\x83\x01\x85a(\xB4V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01@\x8C\x8E\x03\x12\x15a+\nW`\0\x80\xFD[\x8B5a+\x15\x81a$zV[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+7W`\0\x80\xFD[a+C\x8E\x82\x8F\x01a'(V[\x90\x9AP\x98Pa+V\x90P``\x8D\x01a&\x01V[\x96P`\x80\x8C\x015\x95P`\xA0\x8C\x015\x94P`\xC0\x8C\x015\x93P`\xE0\x8C\x015a+{\x81a$zV[\x92Pa\x01\0\x8C\x015a+\x8C\x81a$zV[\x80\x92PPa\x01 \x8C\x015\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`@\x83\x85\x03\x12\x15a+\xBBW`\0\x80\xFD[\x825a+\xC6\x81a$zV[\x91P` \x83\x015a+\xD6\x81a$zV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a+\xF6W`\0\x80\xFD[\x835a,\x01\x81a$zV[\x92P` \x84\x015a,\x11\x81a$zV[\x91P`@\x84\x015a,!\x81a$zV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a,AW`\0\x80\xFD[\x835a,L\x81a$zV[\x92P` \x84\x015a,\\\x81a$zV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x80\x82R`\x05\x90\x82\x01RdGS203`\xD8\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x05\x90\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a,\xD3Wa,\xD3a,\xABV[P`\x01\x01\x90V[`@\x81R`\0a,\xED`@\x83\x01\x85a&yV[\x82\x81\x03` \x84\x01Ra\x1F\xCA\x81\x85a&yV[`\0` \x82\x84\x03\x12\x15a-\x11W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1FiW`\0\x80\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xA6Wa\x0C\xA6a,\xABV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\xA6Wa\x0C\xA6a,\xABV[` \x80\x82R`\x05\x90\x82\x01RdGS201`\xD8\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a-\xACWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x81R` \x81\x01\x8C\x90Ra\x01``@\x82\x01\x81\x90R\x81\x01\x8A\x90R`\0a\x01\x80\x8B\x8D\x82\x85\x017`\0\x83\x8D\x01\x82\x01R`\x1F\x8C\x01`\x1F\x19\x16\x83\x01a-\xFC``\x85\x01\x8Da-\x8EV[\x8A`\x80\x85\x01R\x89`\xA0\x85\x01R\x88`\xC0\x85\x01Ra.#`\xE0\x85\x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x87\x16a\x01\0\x85\x01R\x81\x84\x82\x03\x01a\x01 \x85\x01Ra.J\x82\x82\x01\x87a&yV[\x92PPPa.da\x01@\x83\x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x9D\x9CPPPPPPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0C\xA6Wa\x0C\xA6a,\xABV[`\0\x82a.\xA5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0C\xA6Wa\x0C\xA6a,\xABV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x80\x80\x82R\x81\x01\x85\x90R`\0\x86`\xA0\x83\x01\x82[\x88\x81\x10\x15a/\x16W\x825a.\xF9\x81a$zV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xE6V[P` \x84\x01\x96\x90\x96RPP`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`@\x82\x01R\x91\x16``\x90\x91\x01R\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x8B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16` \x83\x01R`@\x82\x01\x8B\x90R``\x82\x01\x8A\x90Ra\x01`\x82\x01\x90a/\x83`\x80\x84\x01\x8Ba-\x8EV[`\xA0\x83\x01\x98\x90\x98R`\xC0\x82\x01\x96\x90\x96R`\xE0\x81\x01\x94\x90\x94R\x91\x85\x16a\x01\0\x84\x01R\x90\x93\x16a\x01 \x82\x01Ra\x01@\x01\x91\x90\x91R\x95\x94PPPPPV[`\0\x81a/\xCDWa/\xCDa,\xABV[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 ~X\xEBL8\xD8MB\xCB2x>\xB5c\xC2y%\rv\xA3^\xAF\xF1\x02\xA3\x8C`\x04 R59dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static GNOSISSAFE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct GnosisSafe<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GnosisSafe<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for GnosisSafe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for GnosisSafe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for GnosisSafe<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GnosisSafe))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GnosisSafe<M> {
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
                GNOSISSAFE_ABI.clone(),
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
                GNOSISSAFE_ABI.clone(),
                GNOSISSAFE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `VERSION` (0xffa1ad74)
        /// function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOwnerWithThreshold`
        /// (0x0d582f13) function
        pub fn add_owner_with_threshold(
            &self,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 88, 47, 19], (owner, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveHash` (0xd4d9bdcd)
        /// function
        pub fn approve_hash(
            &self,
            hash_to_approve: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 217, 189, 205], hash_to_approve)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvedHashes`
        /// (0x7d832974) function
        pub fn approved_hashes(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([125, 131, 41, 116], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeThreshold`
        /// (0x694e80c3) function
        pub fn change_threshold(
            &self,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 78, 128, 195], threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkNSignatures`
        /// (0x12fb68e0) function
        pub fn check_n_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
            required_signatures: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 251, 104, 224],
                    (data_hash, data, signatures, required_signatures),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures`
        /// (0x934f3a11) function
        pub fn check_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 79, 58, 17], (data_hash, data, signatures))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableModule`
        /// (0xe009cfde) function
        pub fn disable_module(
            &self,
            prev_module: ::ethers::core::types::Address,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 9, 207, 222], (prev_module, module))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator`
        /// (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableModule` (0x610b5925)
        /// function
        pub fn enable_module(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 11, 89, 37], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeTransactionData`
        /// (0xe86637db) function
        pub fn encode_transaction_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [232, 102, 55, 219],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransaction`
        /// (0x6a761202) function
        pub fn exec_transaction(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [106, 118, 18, 2],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        signatures,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModule`
        /// (0x468721a7) function
        pub fn exec_transaction_from_module(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 135, 33, 167], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's
        /// `execTransactionFromModuleReturnData`
        /// (0x5229073f) function
        pub fn exec_transaction_from_module_return_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([82, 41, 7, 63], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470)
        /// function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModulesPaginated`
        /// (0xcc2f8452) function
        pub fn get_modules_paginated(
            &self,
            start: ::ethers::core::types::Address,
            page_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([204, 47, 132, 82], (start, page_size))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOwners` (0xa0e67e2b)
        /// function
        pub fn get_owners(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([160, 230, 126, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStorageAt` (0x5624b25b)
        /// function
        pub fn get_storage_at(
            &self,
            offset: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([86, 36, 178, 91], (offset, length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getThreshold` (0xe75235b8)
        /// function
        pub fn get_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([231, 82, 53, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTransactionHash`
        /// (0xd8d11f78) function
        pub fn get_transaction_hash(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [216, 209, 31, 120],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isModuleEnabled`
        /// (0x2d9ad53d) function
        pub fn is_module_enabled(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 154, 213, 61], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOwner` (0x2f54bf6e)
        /// function
        pub fn is_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 84, 191, 110], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0)
        /// function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeOwner` (0xf8dc5dd9)
        /// function
        pub fn remove_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 220, 93, 217],
                    (prev_owner, owner, threshold),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requiredTxGas`
        /// (0xc4ca3a9c) function
        pub fn required_tx_gas(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([196, 202, 58, 156], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackHandler`
        /// (0xf08a0323) function
        pub fn set_fallback_handler(
            &self,
            handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 138, 3, 35], handler)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGuard` (0xe19a9dd9)
        /// function
        pub fn set_guard(
            &self,
            guard: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 157, 217], guard)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setup` (0xb63e800d)
        /// function
        pub fn setup(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
            threshold: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            fallback_handler: ::ethers::core::types::Address,
            payment_token: ::ethers::core::types::Address,
            payment: ::ethers::core::types::U256,
            payment_receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [182, 62, 128, 13],
                    (
                        owners,
                        threshold,
                        to,
                        data,
                        fallback_handler,
                        payment_token,
                        payment,
                        payment_receiver,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedMessages`
        /// (0x5ae6bd37) function
        pub fn signed_messages(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([90, 230, 189, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateAndRevert`
        /// (0xb4faba09) function
        pub fn simulate_and_revert(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [180, 250, 186, 9],
                    (target_contract, calldata_payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapOwner` (0xe318b52b)
        /// function
        pub fn swap_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            old_owner: ::ethers::core::types::Address,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 24, 181, 43],
                    (prev_owner, old_owner, new_owner),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedOwner` event
        pub fn added_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApproveHash` event
        pub fn approve_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApproveHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedFallbackHandler`
        /// event
        pub fn changed_fallback_handler_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedFallbackHandlerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedGuard` event
        pub fn changed_guard_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedGuardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedThreshold` event
        pub fn changed_threshold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedThresholdFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DisabledModule` event
        pub fn disabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnabledModule` event
        pub fn enabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFailure` event
        pub fn execution_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleFailure`
        /// event
        pub fn execution_from_module_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleSuccess`
        /// event
        pub fn execution_from_module_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionSuccess` event
        pub fn execution_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemovedOwner` event
        pub fn removed_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemovedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeReceived` event
        pub fn safe_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeSetup` event
        pub fn safe_setup_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeSetupFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SignMsg` event
        pub fn sign_msg_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SignMsgFilter,
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
            GnosisSafeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for GnosisSafe<M>
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
    #[ethevent(name = "AddedOwner", abi = "AddedOwner(address)")]
    pub struct AddedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "ApproveHash", abi = "ApproveHash(bytes32,address)")]
    pub struct ApproveHashFilter {
        #[ethevent(indexed)]
        pub approved_hash: [u8; 32],
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
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
        name = "ChangedFallbackHandler",
        abi = "ChangedFallbackHandler(address)"
    )]
    pub struct ChangedFallbackHandlerFilter {
        pub handler: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedGuard", abi = "ChangedGuard(address)")]
    pub struct ChangedGuardFilter {
        pub guard: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedThreshold", abi = "ChangedThreshold(uint256)")]
    pub struct ChangedThresholdFilter {
        pub threshold: ::ethers::core::types::U256,
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
    #[ethevent(name = "DisabledModule", abi = "DisabledModule(address)")]
    pub struct DisabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "EnabledModule", abi = "EnabledModule(address)")]
    pub struct EnabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
        name = "ExecutionFailure",
        abi = "ExecutionFailure(bytes32,uint256)"
    )]
    pub struct ExecutionFailureFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
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
        name = "ExecutionFromModuleFailure",
        abi = "ExecutionFromModuleFailure(address)"
    )]
    pub struct ExecutionFromModuleFailureFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
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
        name = "ExecutionFromModuleSuccess",
        abi = "ExecutionFromModuleSuccess(address)"
    )]
    pub struct ExecutionFromModuleSuccessFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
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
        name = "ExecutionSuccess",
        abi = "ExecutionSuccess(bytes32,uint256)"
    )]
    pub struct ExecutionSuccessFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
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
    #[ethevent(name = "RemovedOwner", abi = "RemovedOwner(address)")]
    pub struct RemovedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "SafeReceived", abi = "SafeReceived(address,uint256)")]
    pub struct SafeReceivedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
        name = "SafeSetup",
        abi = "SafeSetup(address,address[],uint256,address,address)"
    )]
    pub struct SafeSetupFilter {
        #[ethevent(indexed)]
        pub initiator: ::ethers::core::types::Address,
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub initializer: ::ethers::core::types::Address,
        pub fallback_handler: ::ethers::core::types::Address,
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
    #[ethevent(name = "SignMsg", abi = "SignMsg(bytes32)")]
    pub struct SignMsgFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum GnosisSafeEvents {
        AddedOwnerFilter(AddedOwnerFilter),
        ApproveHashFilter(ApproveHashFilter),
        ChangedFallbackHandlerFilter(ChangedFallbackHandlerFilter),
        ChangedGuardFilter(ChangedGuardFilter),
        ChangedThresholdFilter(ChangedThresholdFilter),
        DisabledModuleFilter(DisabledModuleFilter),
        EnabledModuleFilter(EnabledModuleFilter),
        ExecutionFailureFilter(ExecutionFailureFilter),
        ExecutionFromModuleFailureFilter(ExecutionFromModuleFailureFilter),
        ExecutionFromModuleSuccessFilter(ExecutionFromModuleSuccessFilter),
        ExecutionSuccessFilter(ExecutionSuccessFilter),
        RemovedOwnerFilter(RemovedOwnerFilter),
        SafeReceivedFilter(SafeReceivedFilter),
        SafeSetupFilter(SafeSetupFilter),
        SignMsgFilter(SignMsgFilter),
    }
    impl ::ethers::contract::EthLogDecode for GnosisSafeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedOwnerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::AddedOwnerFilter(decoded));
            }
            if let Ok(decoded) = ApproveHashFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ApproveHashFilter(decoded));
            }
            if let Ok(decoded) = ChangedFallbackHandlerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedFallbackHandlerFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ChangedGuardFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedGuardFilter(decoded));
            }
            if let Ok(decoded) = ChangedThresholdFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedThresholdFilter(decoded));
            }
            if let Ok(decoded) = DisabledModuleFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::DisabledModuleFilter(decoded));
            }
            if let Ok(decoded) = EnabledModuleFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::EnabledModuleFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFailureFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionFailureFilter(decoded));
            }
            if let Ok(decoded) =
                ExecutionFromModuleFailureFilter::decode_log(log)
            {
                return Ok(GnosisSafeEvents::ExecutionFromModuleFailureFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                ExecutionFromModuleSuccessFilter::decode_log(log)
            {
                return Ok(GnosisSafeEvents::ExecutionFromModuleSuccessFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ExecutionSuccessFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionSuccessFilter(decoded));
            }
            if let Ok(decoded) = RemovedOwnerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::RemovedOwnerFilter(decoded));
            }
            if let Ok(decoded) = SafeReceivedFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SafeReceivedFilter(decoded));
            }
            if let Ok(decoded) = SafeSetupFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SafeSetupFilter(decoded));
            }
            if let Ok(decoded) = SignMsgFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SignMsgFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GnosisSafeEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::AddedOwnerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApproveHashFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedFallbackHandlerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedGuardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedThresholdFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DisabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovedOwnerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeSetupFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignMsgFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddedOwnerFilter> for GnosisSafeEvents {
        fn from(value: AddedOwnerFilter) -> Self {
            Self::AddedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<ApproveHashFilter> for GnosisSafeEvents {
        fn from(value: ApproveHashFilter) -> Self {
            Self::ApproveHashFilter(value)
        }
    }
    impl ::core::convert::From<ChangedFallbackHandlerFilter> for GnosisSafeEvents {
        fn from(value: ChangedFallbackHandlerFilter) -> Self {
            Self::ChangedFallbackHandlerFilter(value)
        }
    }
    impl ::core::convert::From<ChangedGuardFilter> for GnosisSafeEvents {
        fn from(value: ChangedGuardFilter) -> Self {
            Self::ChangedGuardFilter(value)
        }
    }
    impl ::core::convert::From<ChangedThresholdFilter> for GnosisSafeEvents {
        fn from(value: ChangedThresholdFilter) -> Self {
            Self::ChangedThresholdFilter(value)
        }
    }
    impl ::core::convert::From<DisabledModuleFilter> for GnosisSafeEvents {
        fn from(value: DisabledModuleFilter) -> Self {
            Self::DisabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<EnabledModuleFilter> for GnosisSafeEvents {
        fn from(value: EnabledModuleFilter) -> Self {
            Self::EnabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFailureFilter> for GnosisSafeEvents {
        fn from(value: ExecutionFailureFilter) -> Self {
            Self::ExecutionFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleFailureFilter>
        for GnosisSafeEvents
    {
        fn from(value: ExecutionFromModuleFailureFilter) -> Self {
            Self::ExecutionFromModuleFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleSuccessFilter>
        for GnosisSafeEvents
    {
        fn from(value: ExecutionFromModuleSuccessFilter) -> Self {
            Self::ExecutionFromModuleSuccessFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionSuccessFilter> for GnosisSafeEvents {
        fn from(value: ExecutionSuccessFilter) -> Self {
            Self::ExecutionSuccessFilter(value)
        }
    }
    impl ::core::convert::From<RemovedOwnerFilter> for GnosisSafeEvents {
        fn from(value: RemovedOwnerFilter) -> Self {
            Self::RemovedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SafeReceivedFilter> for GnosisSafeEvents {
        fn from(value: SafeReceivedFilter) -> Self {
            Self::SafeReceivedFilter(value)
        }
    }
    impl ::core::convert::From<SafeSetupFilter> for GnosisSafeEvents {
        fn from(value: SafeSetupFilter) -> Self { Self::SafeSetupFilter(value) }
    }
    impl ::core::convert::From<SignMsgFilter> for GnosisSafeEvents {
        fn from(value: SignMsgFilter) -> Self { Self::SignMsgFilter(value) }
    }
    ///Container type for all input parameters for the
    /// `VERSION` function with signature `VERSION()` and
    /// selector `0xffa1ad74`
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
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the
    /// `addOwnerWithThreshold` function with signature
    /// `addOwnerWithThreshold(address,uint256)` and
    /// selector `0x0d582f13`
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
        name = "addOwnerWithThreshold",
        abi = "addOwnerWithThreshold(address,uint256)"
    )]
    pub struct AddOwnerWithThresholdCall {
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `approveHash` function with signature
    /// `approveHash(bytes32)` and selector `0xd4d9bdcd`
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
    #[ethcall(name = "approveHash", abi = "approveHash(bytes32)")]
    pub struct ApproveHashCall {
        pub hash_to_approve: [u8; 32],
    }
    ///Container type for all input parameters for the
    /// `approvedHashes` function with signature
    /// `approvedHashes(address,bytes32)` and selector
    /// `0x7d832974`
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
    #[ethcall(name = "approvedHashes", abi = "approvedHashes(address,bytes32)")]
    pub struct ApprovedHashesCall(
        pub ::ethers::core::types::Address,
        pub [u8; 32],
    );
    ///Container type for all input parameters for the
    /// `changeThreshold` function with signature
    /// `changeThreshold(uint256)` and selector `0x694e80c3`
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
    #[ethcall(name = "changeThreshold", abi = "changeThreshold(uint256)")]
    pub struct ChangeThresholdCall {
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `checkNSignatures` function with signature
    /// `checkNSignatures(bytes32,bytes,bytes,uint256)` and
    /// selector `0x12fb68e0`
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
        name = "checkNSignatures",
        abi = "checkNSignatures(bytes32,bytes,bytes,uint256)"
    )]
    pub struct CheckNSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
        pub required_signatures: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `checkSignatures` function with signature
    /// `checkSignatures(bytes32,bytes,bytes)` and selector
    /// `0x934f3a11`
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
        name = "checkSignatures",
        abi = "checkSignatures(bytes32,bytes,bytes)"
    )]
    pub struct CheckSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `disableModule` function with signature
    /// `disableModule(address,address)` and selector
    /// `0xe009cfde`
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
    #[ethcall(name = "disableModule", abi = "disableModule(address,address)")]
    pub struct DisableModuleCall {
        pub prev_module: ::ethers::core::types::Address,
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `domainSeparator` function with signature
    /// `domainSeparator()` and selector `0xf698da25`
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
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the
    /// `enableModule` function with signature
    /// `enableModule(address)` and selector `0x610b5925`
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
    #[ethcall(name = "enableModule", abi = "enableModule(address)")]
    pub struct EnableModuleCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `encodeTransactionData` function with signature
    /// `encodeTransactionData(address,uint256,bytes,uint8,
    /// uint256,uint256,uint256,address,address,uint256)`
    /// and selector `0xe86637db`
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
        name = "encodeTransactionData",
        abi = "encodeTransactionData(address,uint256,bytes,uint8,uint256,\
               uint256,uint256,address,address,uint256)"
    )]
    pub struct EncodeTransactionDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `execTransaction` function with signature
    /// `execTransaction(address,uint256,bytes,uint8,
    /// uint256,uint256,uint256,address,address,bytes)` and
    /// selector `0x6a761202`
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
        name = "execTransaction",
        abi = "execTransaction(address,uint256,bytes,uint8,uint256,uint256,\
               uint256,address,address,bytes)"
    )]
    pub struct ExecTransactionCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `execTransactionFromModule` function with signature
    /// `execTransactionFromModule(address,uint256,bytes,
    /// uint8)` and selector `0x468721a7`
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
        name = "execTransactionFromModule",
        abi = "execTransactionFromModule(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the
    /// `execTransactionFromModuleReturnData` function with
    /// signature `execTransactionFromModuleReturnData(address,
    /// uint256,bytes,uint8)` and selector `0x5229073f`
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
        name = "execTransactionFromModuleReturnData",
        abi = "execTransactionFromModuleReturnData(address,uint256,bytes,\
               uint8)"
    )]
    pub struct ExecTransactionFromModuleReturnDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the
    /// `getChainId` function with signature `getChainId()`
    /// and selector `0x3408e470`
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
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the
    /// `getModulesPaginated` function with signature
    /// `getModulesPaginated(address,uint256)` and selector
    /// `0xcc2f8452`
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
        name = "getModulesPaginated",
        abi = "getModulesPaginated(address,uint256)"
    )]
    pub struct GetModulesPaginatedCall {
        pub start: ::ethers::core::types::Address,
        pub page_size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `getOwners` function with signature `getOwners()`
    /// and selector `0xa0e67e2b`
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
    #[ethcall(name = "getOwners", abi = "getOwners()")]
    pub struct GetOwnersCall;
    ///Container type for all input parameters for the
    /// `getStorageAt` function with signature
    /// `getStorageAt(uint256,uint256)` and selector
    /// `0x5624b25b`
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
    #[ethcall(name = "getStorageAt", abi = "getStorageAt(uint256,uint256)")]
    pub struct GetStorageAtCall {
        pub offset: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `getThreshold` function with signature
    /// `getThreshold()` and selector `0xe75235b8`
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
    #[ethcall(name = "getThreshold", abi = "getThreshold()")]
    pub struct GetThresholdCall;
    ///Container type for all input parameters for the
    /// `getTransactionHash` function with signature
    /// `getTransactionHash(address,uint256,bytes,uint8,
    /// uint256,uint256,uint256,address,address,uint256)`
    /// and selector `0xd8d11f78`
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
        name = "getTransactionHash",
        abi = "getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,\
               uint256,address,address,uint256)"
    )]
    pub struct GetTransactionHashCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `isModuleEnabled` function with signature
    /// `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    #[ethcall(name = "isModuleEnabled", abi = "isModuleEnabled(address)")]
    pub struct IsModuleEnabledCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `isOwner` function with signature `isOwner(address)`
    /// and selector `0x2f54bf6e`
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
    #[ethcall(name = "isOwner", abi = "isOwner(address)")]
    pub struct IsOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `nonce` function with signature `nonce()` and
    /// selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the
    /// `removeOwner` function with signature
    /// `removeOwner(address,address,uint256)` and selector
    /// `0xf8dc5dd9`
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
        name = "removeOwner",
        abi = "removeOwner(address,address,uint256)"
    )]
    pub struct RemoveOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `requiredTxGas` function with signature
    /// `requiredTxGas(address,uint256,bytes,uint8)` and
    /// selector `0xc4ca3a9c`
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
        name = "requiredTxGas",
        abi = "requiredTxGas(address,uint256,bytes,uint8)"
    )]
    pub struct RequiredTxGasCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the
    /// `setFallbackHandler` function with signature
    /// `setFallbackHandler(address)` and selector
    /// `0xf08a0323`
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
    #[ethcall(name = "setFallbackHandler", abi = "setFallbackHandler(address)")]
    pub struct SetFallbackHandlerCall {
        pub handler: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `setGuard` function with signature
    /// `setGuard(address)` and selector `0xe19a9dd9`
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
    #[ethcall(name = "setGuard", abi = "setGuard(address)")]
    pub struct SetGuardCall {
        pub guard: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `setup` function with signature
    /// `setup(address[],uint256,address,bytes,address,
    /// address,uint256,address)` and selector `0xb63e800d`
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
        name = "setup",
        abi = "setup(address[],uint256,address,bytes,address,address,uint256,\
               address)"
    )]
    pub struct SetupCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub fallback_handler: ::ethers::core::types::Address,
        pub payment_token: ::ethers::core::types::Address,
        pub payment: ::ethers::core::types::U256,
        pub payment_receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `signedMessages` function with signature
    /// `signedMessages(bytes32)` and selector `0x5ae6bd37`
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
    #[ethcall(name = "signedMessages", abi = "signedMessages(bytes32)")]
    pub struct SignedMessagesCall(pub [u8; 32]);
    ///Container type for all input parameters for the
    /// `simulateAndRevert` function with signature
    /// `simulateAndRevert(address,bytes)` and selector
    /// `0xb4faba09`
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
        name = "simulateAndRevert",
        abi = "simulateAndRevert(address,bytes)"
    )]
    pub struct SimulateAndRevertCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `swapOwner` function with signature
    /// `swapOwner(address,address,address)` and selector
    /// `0xe318b52b`
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
    #[ethcall(name = "swapOwner", abi = "swapOwner(address,address,address)")]
    pub struct SwapOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub old_owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum GnosisSafeCalls {
        Version(VersionCall),
        AddOwnerWithThreshold(AddOwnerWithThresholdCall),
        ApproveHash(ApproveHashCall),
        ApprovedHashes(ApprovedHashesCall),
        ChangeThreshold(ChangeThresholdCall),
        CheckNSignatures(CheckNSignaturesCall),
        CheckSignatures(CheckSignaturesCall),
        DisableModule(DisableModuleCall),
        DomainSeparator(DomainSeparatorCall),
        EnableModule(EnableModuleCall),
        EncodeTransactionData(EncodeTransactionDataCall),
        ExecTransaction(ExecTransactionCall),
        ExecTransactionFromModule(ExecTransactionFromModuleCall),
        ExecTransactionFromModuleReturnData(
            ExecTransactionFromModuleReturnDataCall,
        ),
        GetChainId(GetChainIdCall),
        GetModulesPaginated(GetModulesPaginatedCall),
        GetOwners(GetOwnersCall),
        GetStorageAt(GetStorageAtCall),
        GetThreshold(GetThresholdCall),
        GetTransactionHash(GetTransactionHashCall),
        IsModuleEnabled(IsModuleEnabledCall),
        IsOwner(IsOwnerCall),
        Nonce(NonceCall),
        RemoveOwner(RemoveOwnerCall),
        RequiredTxGas(RequiredTxGasCall),
        SetFallbackHandler(SetFallbackHandlerCall),
        SetGuard(SetGuardCall),
        Setup(SetupCall),
        SignedMessages(SignedMessagesCall),
        SimulateAndRevert(SimulateAndRevertCall),
        SwapOwner(SwapOwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for GnosisSafeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <AddOwnerWithThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddOwnerWithThreshold(decoded));
            }
            if let Ok(decoded) =
                <ApproveHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ApproveHash(decoded));
            }
            if let Ok(decoded) =
                <ApprovedHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ApprovedHashes(decoded));
            }
            if let Ok(decoded) =
                <ChangeThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ChangeThreshold(decoded));
            }
            if let Ok(decoded) =
                <CheckNSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CheckNSignatures(decoded));
            }
            if let Ok(decoded) =
                <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) =
                <DisableModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DisableModule(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <EnableModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::EnableModule(decoded));
            }
            if let Ok(decoded)
                = <EncodeTransactionDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeTransactionData(decoded));
            }
            if let Ok(decoded) =
                <ExecTransactionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ExecTransaction(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecTransactionFromModule(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleReturnDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecTransactionFromModuleReturnData(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded)
                = <GetModulesPaginatedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetModulesPaginated(decoded));
            }
            if let Ok(decoded) =
                <GetOwnersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOwners(decoded));
            }
            if let Ok(decoded) =
                <GetStorageAtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetStorageAt(decoded));
            }
            if let Ok(decoded) =
                <GetThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetThreshold(decoded));
            }
            if let Ok(decoded)
                = <GetTransactionHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTransactionHash(decoded));
            }
            if let Ok(decoded) =
                <IsModuleEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsModuleEnabled(decoded));
            }
            if let Ok(decoded) =
                <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded) =
                <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) =
                <RemoveOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RemoveOwner(decoded));
            }
            if let Ok(decoded) =
                <RequiredTxGasCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RequiredTxGas(decoded));
            }
            if let Ok(decoded)
                = <SetFallbackHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFallbackHandler(decoded));
            }
            if let Ok(decoded) =
                <SetGuardCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetGuard(decoded));
            }
            if let Ok(decoded) =
                <SetupCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Setup(decoded));
            }
            if let Ok(decoded) =
                <SignedMessagesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SignedMessages(decoded));
            }
            if let Ok(decoded)
                = <SimulateAndRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SimulateAndRevert(decoded));
            }
            if let Ok(decoded) =
                <SwapOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GnosisSafeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Version(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOwnerWithThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApprovedHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckNSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DisableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeTransactionData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetModulesPaginated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOwners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStorageAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTransactionHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsModuleEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequiredTxGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGuard(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Setup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAndRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GnosisSafeCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOwnerWithThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApproveHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovedHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckNSignatures(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckSignatures(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DisableModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnableModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeTransactionData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransactionFromModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetModulesPaginated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOwners(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStorageAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTransactionHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsModuleEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequiredTxGas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFallbackHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGuard(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMessages(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateAndRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<VersionCall> for GnosisSafeCalls {
        fn from(value: VersionCall) -> Self { Self::Version(value) }
    }
    impl ::core::convert::From<AddOwnerWithThresholdCall> for GnosisSafeCalls {
        fn from(value: AddOwnerWithThresholdCall) -> Self {
            Self::AddOwnerWithThreshold(value)
        }
    }
    impl ::core::convert::From<ApproveHashCall> for GnosisSafeCalls {
        fn from(value: ApproveHashCall) -> Self { Self::ApproveHash(value) }
    }
    impl ::core::convert::From<ApprovedHashesCall> for GnosisSafeCalls {
        fn from(value: ApprovedHashesCall) -> Self {
            Self::ApprovedHashes(value)
        }
    }
    impl ::core::convert::From<ChangeThresholdCall> for GnosisSafeCalls {
        fn from(value: ChangeThresholdCall) -> Self {
            Self::ChangeThreshold(value)
        }
    }
    impl ::core::convert::From<CheckNSignaturesCall> for GnosisSafeCalls {
        fn from(value: CheckNSignaturesCall) -> Self {
            Self::CheckNSignatures(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for GnosisSafeCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<DisableModuleCall> for GnosisSafeCalls {
        fn from(value: DisableModuleCall) -> Self { Self::DisableModule(value) }
    }
    impl ::core::convert::From<DomainSeparatorCall> for GnosisSafeCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EnableModuleCall> for GnosisSafeCalls {
        fn from(value: EnableModuleCall) -> Self { Self::EnableModule(value) }
    }
    impl ::core::convert::From<EncodeTransactionDataCall> for GnosisSafeCalls {
        fn from(value: EncodeTransactionDataCall) -> Self {
            Self::EncodeTransactionData(value)
        }
    }
    impl ::core::convert::From<ExecTransactionCall> for GnosisSafeCalls {
        fn from(value: ExecTransactionCall) -> Self {
            Self::ExecTransaction(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleCall> for GnosisSafeCalls {
        fn from(value: ExecTransactionFromModuleCall) -> Self {
            Self::ExecTransactionFromModule(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleReturnDataCall>
        for GnosisSafeCalls
    {
        fn from(value: ExecTransactionFromModuleReturnDataCall) -> Self {
            Self::ExecTransactionFromModuleReturnData(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for GnosisSafeCalls {
        fn from(value: GetChainIdCall) -> Self { Self::GetChainId(value) }
    }
    impl ::core::convert::From<GetModulesPaginatedCall> for GnosisSafeCalls {
        fn from(value: GetModulesPaginatedCall) -> Self {
            Self::GetModulesPaginated(value)
        }
    }
    impl ::core::convert::From<GetOwnersCall> for GnosisSafeCalls {
        fn from(value: GetOwnersCall) -> Self { Self::GetOwners(value) }
    }
    impl ::core::convert::From<GetStorageAtCall> for GnosisSafeCalls {
        fn from(value: GetStorageAtCall) -> Self { Self::GetStorageAt(value) }
    }
    impl ::core::convert::From<GetThresholdCall> for GnosisSafeCalls {
        fn from(value: GetThresholdCall) -> Self { Self::GetThreshold(value) }
    }
    impl ::core::convert::From<GetTransactionHashCall> for GnosisSafeCalls {
        fn from(value: GetTransactionHashCall) -> Self {
            Self::GetTransactionHash(value)
        }
    }
    impl ::core::convert::From<IsModuleEnabledCall> for GnosisSafeCalls {
        fn from(value: IsModuleEnabledCall) -> Self {
            Self::IsModuleEnabled(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for GnosisSafeCalls {
        fn from(value: IsOwnerCall) -> Self { Self::IsOwner(value) }
    }
    impl ::core::convert::From<NonceCall> for GnosisSafeCalls {
        fn from(value: NonceCall) -> Self { Self::Nonce(value) }
    }
    impl ::core::convert::From<RemoveOwnerCall> for GnosisSafeCalls {
        fn from(value: RemoveOwnerCall) -> Self { Self::RemoveOwner(value) }
    }
    impl ::core::convert::From<RequiredTxGasCall> for GnosisSafeCalls {
        fn from(value: RequiredTxGasCall) -> Self { Self::RequiredTxGas(value) }
    }
    impl ::core::convert::From<SetFallbackHandlerCall> for GnosisSafeCalls {
        fn from(value: SetFallbackHandlerCall) -> Self {
            Self::SetFallbackHandler(value)
        }
    }
    impl ::core::convert::From<SetGuardCall> for GnosisSafeCalls {
        fn from(value: SetGuardCall) -> Self { Self::SetGuard(value) }
    }
    impl ::core::convert::From<SetupCall> for GnosisSafeCalls {
        fn from(value: SetupCall) -> Self { Self::Setup(value) }
    }
    impl ::core::convert::From<SignedMessagesCall> for GnosisSafeCalls {
        fn from(value: SignedMessagesCall) -> Self {
            Self::SignedMessages(value)
        }
    }
    impl ::core::convert::From<SimulateAndRevertCall> for GnosisSafeCalls {
        fn from(value: SimulateAndRevertCall) -> Self {
            Self::SimulateAndRevert(value)
        }
    }
    impl ::core::convert::From<SwapOwnerCall> for GnosisSafeCalls {
        fn from(value: SwapOwnerCall) -> Self { Self::SwapOwner(value) }
    }
    ///Container type for all return fields from the
    /// `VERSION` function with signature `VERSION()` and
    /// selector `0xffa1ad74`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the
    /// `approvedHashes` function with signature
    /// `approvedHashes(address,bytes32)` and selector
    /// `0x7d832974`
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
    pub struct ApprovedHashesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `domainSeparator` function with signature
    /// `domainSeparator()` and selector `0xf698da25`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the
    /// `encodeTransactionData` function with signature
    /// `encodeTransactionData(address,uint256,bytes,uint8,
    /// uint256,uint256,uint256,address,address,uint256)`
    /// and selector `0xe86637db`
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
    pub struct EncodeTransactionDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the
    /// `execTransaction` function with signature
    /// `execTransaction(address,uint256,bytes,uint8,
    /// uint256,uint256,uint256,address,address,bytes)` and
    /// selector `0x6a761202`
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
    pub struct ExecTransactionReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the
    /// `execTransactionFromModule` function with signature
    /// `execTransactionFromModule(address,uint256,bytes,
    /// uint8)` and selector `0x468721a7`
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
    pub struct ExecTransactionFromModuleReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the
    /// `execTransactionFromModuleReturnData` function with
    /// signature `execTransactionFromModuleReturnData(address,
    /// uint256,bytes,uint8)` and selector `0x5229073f`
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
    pub struct ExecTransactionFromModuleReturnDataReturn {
        pub success: bool,
        pub return_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the
    /// `getChainId` function with signature `getChainId()`
    /// and selector `0x3408e470`
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
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getModulesPaginated` function with signature
    /// `getModulesPaginated(address,uint256)` and selector
    /// `0xcc2f8452`
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
    pub struct GetModulesPaginatedReturn {
        pub array: ::std::vec::Vec<::ethers::core::types::Address>,
        pub next: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the
    /// `getOwners` function with signature `getOwners()`
    /// and selector `0xa0e67e2b`
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
    pub struct GetOwnersReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the
    /// `getStorageAt` function with signature
    /// `getStorageAt(uint256,uint256)` and selector
    /// `0x5624b25b`
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
    pub struct GetStorageAtReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the
    /// `getThreshold` function with signature
    /// `getThreshold()` and selector `0xe75235b8`
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
    pub struct GetThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getTransactionHash` function with signature
    /// `getTransactionHash(address,uint256,bytes,uint8,
    /// uint256,uint256,uint256,address,address,uint256)`
    /// and selector `0xd8d11f78`
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
    pub struct GetTransactionHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the
    /// `isModuleEnabled` function with signature
    /// `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    pub struct IsModuleEnabledReturn(pub bool);
    ///Container type for all return fields from the
    /// `isOwner` function with signature `isOwner(address)`
    /// and selector `0x2f54bf6e`
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
    pub struct IsOwnerReturn(pub bool);
    ///Container type for all return fields from the
    /// `nonce` function with signature `nonce()` and
    /// selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `requiredTxGas` function with signature
    /// `requiredTxGas(address,uint256,bytes,uint8)` and
    /// selector `0xc4ca3a9c`
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
    pub struct RequiredTxGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `signedMessages` function with signature
    /// `signedMessages(bytes32)` and selector `0x5ae6bd37`
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
    pub struct SignedMessagesReturn(pub ::ethers::core::types::U256);
}
