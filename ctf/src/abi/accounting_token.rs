pub use accounting_token::*;
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
pub mod accounting_token {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BURNER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BURNER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("SNAPSHOT_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SNAPSHOT_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cancelOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cancelOwnershipHandover",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("completeOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeOwnershipHandover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
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
                    ::std::borrow::ToOwned::to_owned("grantRoles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRoles"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasAllRoles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasAllRoles"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("hasAnyRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasAnyRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
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
                            outputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("ordinalsFromRoles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ordinalsFromRoles"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                                    name: ::std::borrow::ToOwned::to_owned("ordinals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("ownershipHandoverExpiresAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ownershipHandoverExpiresAt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("ownershipHandoverValidFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ownershipHandoverValidFor",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRoles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRoles"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestOwnershipHandover",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeRoles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRoles"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rolesFromOrdinals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rolesFromOrdinals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ordinals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                    ::std::borrow::ToOwned::to_owned("rolesOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rolesOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
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
                    ::std::borrow::ToOwned::to_owned("snapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("snapshot"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("OwnershipHandoverCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipHandoverCanceled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipHandoverRequested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipHandoverRequested",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
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
                    ::std::borrow::ToOwned::to_owned("RolesUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RolesUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("roles"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NewOwnerIsZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewOwnerIsZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoHandoverRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoHandoverRequest"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotImplemented"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotImplemented"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
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
    pub static ACCOUNTINGTOKEN_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e9*7\xB5\xB2\xB7`\xD1\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c9*%\xA7`\xE1\x1B\x81RP\x81`\x03\x90\x81b\0\0_\x91\x90b\0\x01\xC1V[P`\x04b\0\0n\x82\x82b\0\x01\xC1V[PPPb\0\0\x823b\0\0\x95` \x1B` \x1CV[b\0\0\x8F3`\x07b\0\0\xD1V[b\0\x02\x8DV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80`\0\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01hWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xBCW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\x97WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\xB8W\x82\x81U`\x01\x01b\0\x01\xA3V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01\xDDWb\0\x01\xDDb\0\x01\x1CV[b\0\x01\xF5\x81b\0\x01\xEE\x84Tb\0\x012V[\x84b\0\x01nV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02-W`\0\x84\x15b\0\x02\x14WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\xB8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02^W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02=V[P\x85\x82\x10\x15b\0\x02}W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x14~\x80b\0\x02\x9D`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x04W`\x005`\xE0\x1C\x80cT\xD1\xF1=\x11a\x01\x18W\x80c\x9D\xC2\x9F\xAC\x11a\0\xA0W\x80c\xD7S?\x02\x11a\0oW\x80c\xD7S?\x02\x14a\x05\x8FW\x80c\xDDb\xED>\x14a\x05\xADW\x80c\xF0N(>\x14a\x05\xCDW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE0W\x80c\xFE\xE8\x1C\xF4\x14a\x05\xF3W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x14a\x05:W\x80c\xA4W\xC2\xD7\x14a\x05ZW\x80c\xA9\x05\x9C\xBB\x14a\x024W\x80c\xD59\x13\x93\x14a\x05zW`\0\x80\xFD[\x80csY\xE4\x1F\x11a\0\xE7W\x80csY\xE4\x1F\x14a\x04\x97W\x80c\x8D\xA5\xCB[\x14a\x04\xC4W\x80c\x95\xD8\x9BA\x14a\x04\xF0W\x80c\x97\x11qZ\x14a\x05\x05W\x80c\x98\x1B$\xD0\x14a\x05\x1AW`\0\x80\xFD[\x80cT\xD1\xF1=\x14a\x04<W\x80cp(\xE2\xCD\x14a\x04DW\x80cp\xA0\x821\x14a\x04YW\x80cqP\x18\xA6\x14a\x04\x8FW`\0\x80\xFD[\x80c%i)b\x11a\x01\x9BW\x80c9P\x93Q\x11a\x01jW\x80c9P\x93Q\x14a\x03\x92W\x80c@\xC1\x0F\x19\x14a\x03\xB2W\x80cJN\xE7\xB1\x14a\x03\xD2W\x80cN\xE2\xCD~\x14a\x03\xE5W\x80cQNb\xFC\x14a\x04\x05W`\0\x80\xFD[\x80c%i)b\x14a\x03&W\x80c(,Q\xF3\x14a\x03.W\x80c-\xE9H\x07\x14a\x03CW\x80c1<\xE5g\x14a\x03vW`\0\x80\xFD[\x80c\x18:On\x11a\x01\xD7W\x80c\x18:On\x14a\x02\xA7W\x80c\x1C\x10\x89?\x14a\x02\xBCW\x80c\x1C\xD6M\xF4\x14a\x02\xCFW\x80c#\xB8r\xDD\x14a\x03\x06W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x02\tW\x80c\t^\xA7\xB3\x14a\x024W\x80c\x13\xA6a\xED\x14a\x02dW\x80c\x18\x16\r\xDD\x14a\x02\x92W[`\0\x80\xFD[4\x80\x15a\x02\x15W`\0\x80\xFD[Pa\x02\x1Ea\x06&V[`@Qa\x02+\x91\x90a\x11;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02@W`\0\x80\xFD[Pa\x02Ta\x02O6`\x04a\x11\xA0V[a\x06\xB8V[`@Q\x90\x15\x15\x81R` \x01a\x02+V[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x02\x84a\x02\x7F6`\x04a\x11\xF1V[a\x06\xD2V[`@Q\x90\x81R` \x01a\x02+V[4\x80\x15a\x02\x9EW`\0\x80\xFD[P`\x02Ta\x02\x84V[a\x02\xBAa\x02\xB56`\x04a\x12\xB6V[a\x06\xFBV[\0[a\x02\xBAa\x02\xCA6`\x04a\x11\xA0V[a\x07\x08V[4\x80\x15a\x02\xDBW`\0\x80\xFD[Pa\x02Ta\x02\xEA6`\x04a\x11\xA0V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x03\x12W`\0\x80\xFD[Pa\x02Ta\x03!6`\x04a\x12\xCFV[a\x07\x1EV[a\x02\xBAa\x07BV[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x02\x84`\x04\x81V[4\x80\x15a\x03OW`\0\x80\xFD[Pa\x02\x84a\x03^6`\x04a\x13\x0BV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03\x82W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x02+V[4\x80\x15a\x03\x9EW`\0\x80\xFD[Pa\x02Ta\x03\xAD6`\x04a\x11\xA0V[a\x07\x92V[4\x80\x15a\x03\xBEW`\0\x80\xFD[Pa\x02\xBAa\x03\xCD6`\x04a\x11\xA0V[a\x07\xB4V[a\x02\xBAa\x03\xE06`\x04a\x11\xA0V[a\x07\xCEV[4\x80\x15a\x03\xF1W`\0\x80\xFD[Pa\x02\x84a\x04\x006`\x04a\x11\xA0V[a\x07\xE0V[4\x80\x15a\x04\x11W`\0\x80\xFD[Pa\x02Ta\x04 6`\x04a\x11\xA0V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02\xBAa\x089V[4\x80\x15a\x04PW`\0\x80\xFD[Pa\x02\x84`\x02\x81V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x02\x84a\x04t6`\x04a\x13\x0BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02\xBAa\x08uV[4\x80\x15a\x04\xA3W`\0\x80\xFD[Pa\x04\xB7a\x04\xB26`\x04a\x12\xB6V[a\x08\x89V[`@Qa\x02+\x91\x90a\x13&V[4\x80\x15a\x04\xD0W`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[4\x80\x15a\x04\xFCW`\0\x80\xFD[Pa\x02\x1Ea\x08\xC2V[4\x80\x15a\x05\x11W`\0\x80\xFD[Pa\x02\x84a\x08\xD1V[4\x80\x15a\x05&W`\0\x80\xFD[Pa\x02\x84a\x0556`\x04a\x12\xB6V[a\x08\xECV[4\x80\x15a\x05FW`\0\x80\xFD[Pa\x02\xBAa\x05U6`\x04a\x11\xA0V[a\t\x17V[4\x80\x15a\x05fW`\0\x80\xFD[Pa\x02Ta\x05u6`\x04a\x11\xA0V[a\t,V[4\x80\x15a\x05\x86W`\0\x80\xFD[Pa\x02\x84`\x01\x81V[4\x80\x15a\x05\x9BW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x02+V[4\x80\x15a\x05\xB9W`\0\x80\xFD[Pa\x02\x84a\x05\xC86`\x04a\x13mV[a\t\xACV[a\x02\xBAa\x05\xDB6`\x04a\x13\x0BV[a\t\xD7V[a\x02\xBAa\x05\xEE6`\x04a\x13\x0BV[a\n\x14V[4\x80\x15a\x05\xFFW`\0\x80\xFD[Pa\x02\x84a\x06\x0E6`\x04a\x13\x0BV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[```\x03\x80Ta\x065\x90a\x13\xA0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06a\x90a\x13\xA0V[\x80\x15a\x06\xAEW\x80`\x1F\x10a\x06\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\xC6\x81\x85\x85a\n;V[`\x01\x91PP[\x92\x91PPV[`\0\x81Q`\x05\x1B[\x80\x15a\x06\xF5W\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x06\xDAV[P\x91\x90PV[a\x07\x053\x82a\nTV[PV[a\x07\x10a\n\xA3V[a\x07\x1A\x82\x82a\n\xBEV[PPV[`\x003a\x07,\x85\x82\x85a\x0B\tV[a\x077\x85\x85\x85a\n;V[P`\x01\x94\x93PPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\x003a\x06\xC6\x81\x85\x85a\x07\xA5\x83\x83a\t\xACV[a\x07\xAF\x91\x90a\x13\xEAV[a\n;V[`\x01a\x07\xBF\x81a\x0B\x83V[a\x07\xC9\x83\x83a\x0B\xA9V[PPPV[a\x07\xD6a\n\xA3V[a\x07\x1A\x82\x82a\nTV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x05` R`@\x81 \x81\x90\x81\x90a\x08\x07\x90\x85\x90a\x0CtV[\x91P\x91P\x81a\x08.W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 Ta\x080V[\x80[\x95\x94PPPPPV[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x08}a\n\xA3V[a\x08\x87`\0a\rjV[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\x08\x94WPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x04\x80Ta\x065\x90a\x13\xA0V[`\0`\x02a\x08\xDE\x81a\x0B\x83V[a\x08\xE6a\r\xA8V[\x91PP\x90V[`\0\x80`\0a\x08\xFC\x84`\x06a\x0CtV[\x91P\x91P\x81a\t\rW`\x02Ta\t\x0FV[\x80[\x94\x93PPPPV[`\x04a\t\"\x81a\x0B\x83V[a\x07\xC9\x83\x83a\x0E\x02V[`\x003\x81a\t:\x82\x86a\t\xACV[\x90P\x83\x81\x10\x15a\t\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x077\x82\x86\x86\x84\x03a\n;V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\t\xDFa\n\xA3V[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\n\x07Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x07\x05\x81a\rjV[a\n\x1Ca\n\xA3V[\x80``\x1Ba\n2WctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x07\x05\x81a\rjV[`@Qc\xD6#G%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\x08\x87Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0a\x0B\x15\x84\x84a\t\xACV[\x90P`\0\x19\x81\x14a\x0B}W\x81\x81\x10\x15a\x0BpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\t\x96V[a\x0B}\x84\x84\x84\x84\x03a\n;V[PPPPV[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x07\x05Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\t\x96V[a\x0C\x0B`\0\x83\x83a\x0F@V[\x80`\x02`\0\x82\x82Ta\x0C\x1D\x91\x90a\x13\xEAV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80`\0\x84\x11a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x04U$3#\x056\xE6\x17\x076\x86\xF7C\xA2\x06\x96B\x06\x972\x03`T\x1B`D\x82\x01R`d\x01a\t\x96V[a\x0C\xC8a\x0F\x88V[\x84\x11\x15a\r\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Snapshot: nonexistent id\0\0\0`D\x82\x01R`d\x01a\t\x96V[`\0a\r#\x84\x86a\x0F\x98V[\x84T\x90\x91P\x81\x03a\r;W`\0\x80\x92P\x92PPa\rcV[`\x01\x84`\x01\x01\x82\x81T\x81\x10a\rRWa\rRa\x13\xFDV[\x90`\0R` `\0 \x01T\x92P\x92PP[\x92P\x92\x90PV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\0a\r\xB8`\x08\x80T`\x01\x01\x90UV[`\0a\r\xC2a\x0F\x88V[\x90P\x7F\x800\xE8;\x04\xD8{\xEFSH\x0E&&2f\xD6\xCAf\x86:\xA8Pj\xCAo%Y\xD1\x8A\xA1\xCBg\x81`@Qa\r\xF5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0EbW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\t\x96V[a\x0En\x82`\0\x83a\x0F@V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0E\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\t\x96V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x86\x86\x03\x90U`\x02\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0F_Wa\x0FW\x82a\x10EV[a\x07\xC9a\x10wV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0FvWa\x0FW\x83a\x10EV[a\x0F\x7F\x83a\x10EV[a\x07\xC9\x82a\x10EV[`\0a\x0F\x93`\x08T\x90V[\x90P\x90V[\x81T`\0\x90\x81\x03a\x0F\xABWP`\0a\x06\xCCV[\x82T`\0\x90[\x80\x82\x10\x15a\x0F\xF8W`\0a\x0F\xC5\x83\x83a\x10\x85V[`\0\x87\x81R` \x90 \x90\x91P\x85\x90\x82\x01T\x11\x15a\x0F\xE4W\x80\x91Pa\x0F\xF2V[a\x0F\xEF\x81`\x01a\x13\xEAV[\x92P[Pa\x0F\xB1V[`\0\x82\x11\x80\x15a\x10$WP\x83a\x10!\x86a\x10\x13`\x01\x86a\x14\x13V[`\0\x91\x82R` \x90\x91 \x01\x90V[T\x14[\x15a\x10=Wa\x104`\x01\x83a\x14\x13V[\x92PPPa\x06\xCCV[P\x90Pa\x06\xCCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x91\x83\x90R\x90\x91 Ta\x07\x05\x91\x90a\x10\xA7V[a\x10\xA7V[a\x08\x87`\x06a\x10r`\x02T\x90V[`\0a\x10\x94`\x02\x84\x84\x18a\x14&V[a\x10\xA0\x90\x84\x84\x16a\x13\xEAV[\x93\x92PPPV[`\0a\x10\xB1a\x0F\x88V[\x90P\x80a\x10\xBD\x84a\x10\xF1V[\x10\x15a\x07\xC9W\x82T`\x01\x80\x82\x01\x85U`\0\x85\x81R` \x80\x82 \x90\x93\x01\x93\x90\x93U\x93\x84\x01\x80T\x94\x85\x01\x81U\x82R\x90 \x90\x91\x01UV[\x80T`\0\x90\x81\x03a\x11\x04WP`\0\x91\x90PV[\x81T\x82\x90a\x11\x14\x90`\x01\x90a\x14\x13V[\x81T\x81\x10a\x11$Wa\x11$a\x13\xFDV[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x11hW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x11LV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x116W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x11\xB3W`\0\x80\xFD[a\x11\xBC\x83a\x11\x89V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\xFF\x81\x16\x81\x14a\x116W`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x12\x04W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\x1CW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x120W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12BWa\x12Ba\x11\xCAV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x12gWa\x12ga\x11\xCAV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x12\x85W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x12\xAAWa\x12\x9B\x85a\x11\xE0V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x12\x8AV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x12\xC8W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xE4W`\0\x80\xFD[a\x12\xED\x84a\x11\x89V[\x92Pa\x12\xFB` \x85\x01a\x11\x89V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x13\x1DW`\0\x80\xFD[a\x10\xA0\x82a\x11\x89V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x13aW\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x13BV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x80W`\0\x80\xFD[a\x13\x89\x83a\x11\x89V[\x91Pa\x13\x97` \x84\x01a\x11\x89V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\xB4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xF5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xCCWa\x06\xCCa\x13\xD4V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xCCWa\x06\xCCa\x13\xD4V[`\0\x82a\x14CWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xE9\xFE{\xDD\xE8ms\xBB\x99\0w#F\x04/\x8D\x9D\xEDv\xB8\xDF\x94\xD2\xA7\xBC\x1A\xD2\xFC\xD8\xFF\xF5\x82dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static ACCOUNTINGTOKEN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02\x04W`\x005`\xE0\x1C\x80cT\xD1\xF1=\x11a\x01\x18W\x80c\x9D\xC2\x9F\xAC\x11a\0\xA0W\x80c\xD7S?\x02\x11a\0oW\x80c\xD7S?\x02\x14a\x05\x8FW\x80c\xDDb\xED>\x14a\x05\xADW\x80c\xF0N(>\x14a\x05\xCDW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE0W\x80c\xFE\xE8\x1C\xF4\x14a\x05\xF3W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x14a\x05:W\x80c\xA4W\xC2\xD7\x14a\x05ZW\x80c\xA9\x05\x9C\xBB\x14a\x024W\x80c\xD59\x13\x93\x14a\x05zW`\0\x80\xFD[\x80csY\xE4\x1F\x11a\0\xE7W\x80csY\xE4\x1F\x14a\x04\x97W\x80c\x8D\xA5\xCB[\x14a\x04\xC4W\x80c\x95\xD8\x9BA\x14a\x04\xF0W\x80c\x97\x11qZ\x14a\x05\x05W\x80c\x98\x1B$\xD0\x14a\x05\x1AW`\0\x80\xFD[\x80cT\xD1\xF1=\x14a\x04<W\x80cp(\xE2\xCD\x14a\x04DW\x80cp\xA0\x821\x14a\x04YW\x80cqP\x18\xA6\x14a\x04\x8FW`\0\x80\xFD[\x80c%i)b\x11a\x01\x9BW\x80c9P\x93Q\x11a\x01jW\x80c9P\x93Q\x14a\x03\x92W\x80c@\xC1\x0F\x19\x14a\x03\xB2W\x80cJN\xE7\xB1\x14a\x03\xD2W\x80cN\xE2\xCD~\x14a\x03\xE5W\x80cQNb\xFC\x14a\x04\x05W`\0\x80\xFD[\x80c%i)b\x14a\x03&W\x80c(,Q\xF3\x14a\x03.W\x80c-\xE9H\x07\x14a\x03CW\x80c1<\xE5g\x14a\x03vW`\0\x80\xFD[\x80c\x18:On\x11a\x01\xD7W\x80c\x18:On\x14a\x02\xA7W\x80c\x1C\x10\x89?\x14a\x02\xBCW\x80c\x1C\xD6M\xF4\x14a\x02\xCFW\x80c#\xB8r\xDD\x14a\x03\x06W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x02\tW\x80c\t^\xA7\xB3\x14a\x024W\x80c\x13\xA6a\xED\x14a\x02dW\x80c\x18\x16\r\xDD\x14a\x02\x92W[`\0\x80\xFD[4\x80\x15a\x02\x15W`\0\x80\xFD[Pa\x02\x1Ea\x06&V[`@Qa\x02+\x91\x90a\x11;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02@W`\0\x80\xFD[Pa\x02Ta\x02O6`\x04a\x11\xA0V[a\x06\xB8V[`@Q\x90\x15\x15\x81R` \x01a\x02+V[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x02\x84a\x02\x7F6`\x04a\x11\xF1V[a\x06\xD2V[`@Q\x90\x81R` \x01a\x02+V[4\x80\x15a\x02\x9EW`\0\x80\xFD[P`\x02Ta\x02\x84V[a\x02\xBAa\x02\xB56`\x04a\x12\xB6V[a\x06\xFBV[\0[a\x02\xBAa\x02\xCA6`\x04a\x11\xA0V[a\x07\x08V[4\x80\x15a\x02\xDBW`\0\x80\xFD[Pa\x02Ta\x02\xEA6`\x04a\x11\xA0V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x03\x12W`\0\x80\xFD[Pa\x02Ta\x03!6`\x04a\x12\xCFV[a\x07\x1EV[a\x02\xBAa\x07BV[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x02\x84`\x04\x81V[4\x80\x15a\x03OW`\0\x80\xFD[Pa\x02\x84a\x03^6`\x04a\x13\x0BV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03\x82W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x02+V[4\x80\x15a\x03\x9EW`\0\x80\xFD[Pa\x02Ta\x03\xAD6`\x04a\x11\xA0V[a\x07\x92V[4\x80\x15a\x03\xBEW`\0\x80\xFD[Pa\x02\xBAa\x03\xCD6`\x04a\x11\xA0V[a\x07\xB4V[a\x02\xBAa\x03\xE06`\x04a\x11\xA0V[a\x07\xCEV[4\x80\x15a\x03\xF1W`\0\x80\xFD[Pa\x02\x84a\x04\x006`\x04a\x11\xA0V[a\x07\xE0V[4\x80\x15a\x04\x11W`\0\x80\xFD[Pa\x02Ta\x04 6`\x04a\x11\xA0V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02\xBAa\x089V[4\x80\x15a\x04PW`\0\x80\xFD[Pa\x02\x84`\x02\x81V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x02\x84a\x04t6`\x04a\x13\x0BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02\xBAa\x08uV[4\x80\x15a\x04\xA3W`\0\x80\xFD[Pa\x04\xB7a\x04\xB26`\x04a\x12\xB6V[a\x08\x89V[`@Qa\x02+\x91\x90a\x13&V[4\x80\x15a\x04\xD0W`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[4\x80\x15a\x04\xFCW`\0\x80\xFD[Pa\x02\x1Ea\x08\xC2V[4\x80\x15a\x05\x11W`\0\x80\xFD[Pa\x02\x84a\x08\xD1V[4\x80\x15a\x05&W`\0\x80\xFD[Pa\x02\x84a\x0556`\x04a\x12\xB6V[a\x08\xECV[4\x80\x15a\x05FW`\0\x80\xFD[Pa\x02\xBAa\x05U6`\x04a\x11\xA0V[a\t\x17V[4\x80\x15a\x05fW`\0\x80\xFD[Pa\x02Ta\x05u6`\x04a\x11\xA0V[a\t,V[4\x80\x15a\x05\x86W`\0\x80\xFD[Pa\x02\x84`\x01\x81V[4\x80\x15a\x05\x9BW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x02+V[4\x80\x15a\x05\xB9W`\0\x80\xFD[Pa\x02\x84a\x05\xC86`\x04a\x13mV[a\t\xACV[a\x02\xBAa\x05\xDB6`\x04a\x13\x0BV[a\t\xD7V[a\x02\xBAa\x05\xEE6`\x04a\x13\x0BV[a\n\x14V[4\x80\x15a\x05\xFFW`\0\x80\xFD[Pa\x02\x84a\x06\x0E6`\x04a\x13\x0BV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[```\x03\x80Ta\x065\x90a\x13\xA0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06a\x90a\x13\xA0V[\x80\x15a\x06\xAEW\x80`\x1F\x10a\x06\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\xC6\x81\x85\x85a\n;V[`\x01\x91PP[\x92\x91PPV[`\0\x81Q`\x05\x1B[\x80\x15a\x06\xF5W\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x06\xDAV[P\x91\x90PV[a\x07\x053\x82a\nTV[PV[a\x07\x10a\n\xA3V[a\x07\x1A\x82\x82a\n\xBEV[PPV[`\x003a\x07,\x85\x82\x85a\x0B\tV[a\x077\x85\x85\x85a\n;V[P`\x01\x94\x93PPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\x003a\x06\xC6\x81\x85\x85a\x07\xA5\x83\x83a\t\xACV[a\x07\xAF\x91\x90a\x13\xEAV[a\n;V[`\x01a\x07\xBF\x81a\x0B\x83V[a\x07\xC9\x83\x83a\x0B\xA9V[PPPV[a\x07\xD6a\n\xA3V[a\x07\x1A\x82\x82a\nTV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x05` R`@\x81 \x81\x90\x81\x90a\x08\x07\x90\x85\x90a\x0CtV[\x91P\x91P\x81a\x08.W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 Ta\x080V[\x80[\x95\x94PPPPPV[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x08}a\n\xA3V[a\x08\x87`\0a\rjV[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\x08\x94WPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x04\x80Ta\x065\x90a\x13\xA0V[`\0`\x02a\x08\xDE\x81a\x0B\x83V[a\x08\xE6a\r\xA8V[\x91PP\x90V[`\0\x80`\0a\x08\xFC\x84`\x06a\x0CtV[\x91P\x91P\x81a\t\rW`\x02Ta\t\x0FV[\x80[\x94\x93PPPPV[`\x04a\t\"\x81a\x0B\x83V[a\x07\xC9\x83\x83a\x0E\x02V[`\x003\x81a\t:\x82\x86a\t\xACV[\x90P\x83\x81\x10\x15a\t\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x077\x82\x86\x86\x84\x03a\n;V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\t\xDFa\n\xA3V[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\n\x07Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x07\x05\x81a\rjV[a\n\x1Ca\n\xA3V[\x80``\x1Ba\n2WctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x07\x05\x81a\rjV[`@Qc\xD6#G%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\x08\x87Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0a\x0B\x15\x84\x84a\t\xACV[\x90P`\0\x19\x81\x14a\x0B}W\x81\x81\x10\x15a\x0BpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\t\x96V[a\x0B}\x84\x84\x84\x84\x03a\n;V[PPPPV[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x07\x05Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\t\x96V[a\x0C\x0B`\0\x83\x83a\x0F@V[\x80`\x02`\0\x82\x82Ta\x0C\x1D\x91\x90a\x13\xEAV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80`\0\x84\x11a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x04U$3#\x056\xE6\x17\x076\x86\xF7C\xA2\x06\x96B\x06\x972\x03`T\x1B`D\x82\x01R`d\x01a\t\x96V[a\x0C\xC8a\x0F\x88V[\x84\x11\x15a\r\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Snapshot: nonexistent id\0\0\0`D\x82\x01R`d\x01a\t\x96V[`\0a\r#\x84\x86a\x0F\x98V[\x84T\x90\x91P\x81\x03a\r;W`\0\x80\x92P\x92PPa\rcV[`\x01\x84`\x01\x01\x82\x81T\x81\x10a\rRWa\rRa\x13\xFDV[\x90`\0R` `\0 \x01T\x92P\x92PP[\x92P\x92\x90PV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\0a\r\xB8`\x08\x80T`\x01\x01\x90UV[`\0a\r\xC2a\x0F\x88V[\x90P\x7F\x800\xE8;\x04\xD8{\xEFSH\x0E&&2f\xD6\xCAf\x86:\xA8Pj\xCAo%Y\xD1\x8A\xA1\xCBg\x81`@Qa\r\xF5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0EbW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\t\x96V[a\x0En\x82`\0\x83a\x0F@V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0E\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\t\x96V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x86\x86\x03\x90U`\x02\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0F_Wa\x0FW\x82a\x10EV[a\x07\xC9a\x10wV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0FvWa\x0FW\x83a\x10EV[a\x0F\x7F\x83a\x10EV[a\x07\xC9\x82a\x10EV[`\0a\x0F\x93`\x08T\x90V[\x90P\x90V[\x81T`\0\x90\x81\x03a\x0F\xABWP`\0a\x06\xCCV[\x82T`\0\x90[\x80\x82\x10\x15a\x0F\xF8W`\0a\x0F\xC5\x83\x83a\x10\x85V[`\0\x87\x81R` \x90 \x90\x91P\x85\x90\x82\x01T\x11\x15a\x0F\xE4W\x80\x91Pa\x0F\xF2V[a\x0F\xEF\x81`\x01a\x13\xEAV[\x92P[Pa\x0F\xB1V[`\0\x82\x11\x80\x15a\x10$WP\x83a\x10!\x86a\x10\x13`\x01\x86a\x14\x13V[`\0\x91\x82R` \x90\x91 \x01\x90V[T\x14[\x15a\x10=Wa\x104`\x01\x83a\x14\x13V[\x92PPPa\x06\xCCV[P\x90Pa\x06\xCCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x91\x83\x90R\x90\x91 Ta\x07\x05\x91\x90a\x10\xA7V[a\x10\xA7V[a\x08\x87`\x06a\x10r`\x02T\x90V[`\0a\x10\x94`\x02\x84\x84\x18a\x14&V[a\x10\xA0\x90\x84\x84\x16a\x13\xEAV[\x93\x92PPPV[`\0a\x10\xB1a\x0F\x88V[\x90P\x80a\x10\xBD\x84a\x10\xF1V[\x10\x15a\x07\xC9W\x82T`\x01\x80\x82\x01\x85U`\0\x85\x81R` \x80\x82 \x90\x93\x01\x93\x90\x93U\x93\x84\x01\x80T\x94\x85\x01\x81U\x82R\x90 \x90\x91\x01UV[\x80T`\0\x90\x81\x03a\x11\x04WP`\0\x91\x90PV[\x81T\x82\x90a\x11\x14\x90`\x01\x90a\x14\x13V[\x81T\x81\x10a\x11$Wa\x11$a\x13\xFDV[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[\x91\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x11hW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x11LV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x116W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x11\xB3W`\0\x80\xFD[a\x11\xBC\x83a\x11\x89V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\xFF\x81\x16\x81\x14a\x116W`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x12\x04W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\x1CW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x120W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12BWa\x12Ba\x11\xCAV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x12gWa\x12ga\x11\xCAV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x12\x85W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x12\xAAWa\x12\x9B\x85a\x11\xE0V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x12\x8AV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x12\xC8W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xE4W`\0\x80\xFD[a\x12\xED\x84a\x11\x89V[\x92Pa\x12\xFB` \x85\x01a\x11\x89V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x13\x1DW`\0\x80\xFD[a\x10\xA0\x82a\x11\x89V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x13aW\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x13BV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x80W`\0\x80\xFD[a\x13\x89\x83a\x11\x89V[\x91Pa\x13\x97` \x84\x01a\x11\x89V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\xB4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xF5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xCCWa\x06\xCCa\x13\xD4V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xCCWa\x06\xCCa\x13\xD4V[`\0\x82a\x14CWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xE9\xFE{\xDD\xE8ms\xBB\x99\0w#F\x04/\x8D\x9D\xEDv\xB8\xDF\x94\xD2\xA7\xBC\x1A\xD2\xFC\xD8\xFF\xF5\x82dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static ACCOUNTINGTOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct AccountingToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AccountingToken<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for AccountingToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for AccountingToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for AccountingToken<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AccountingToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AccountingToken<M> {
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
                ACCOUNTINGTOKEN_ABI.clone(),
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
                ACCOUNTINGTOKEN_ABI.clone(),
                ACCOUNTINGTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BURNER_ROLE` (0x282c51f3)
        /// function
        pub fn burner_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([40, 44, 81, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINTER_ROLE` (0xd5391393)
        /// function
        pub fn minter_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([213, 57, 19, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SNAPSHOT_ROLE`
        /// (0x7028e2cd) function
        pub fn snapshot_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([112, 40, 226, 205], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `burn` (0x9dc29fac)
        /// function
        pub fn burn(
            &self,
            from: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (from, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelOwnershipHandover`
        /// (0x54d1f13d) function
        pub fn cancel_ownership_handover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 209, 241, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeOwnershipHandover`
        /// (0xf04e283e) function
        pub fn complete_ownership_handover(
            &self,
            pending_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 78, 40, 62], pending_owner)
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
        ///Calls the contract's `grantRoles` (0x1c10893f)
        /// function
        pub fn grant_roles(
            &self,
            user: ::ethers::core::types::Address,
            roles: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 16, 137, 63], (user, roles))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAllRoles` (0x1cd64df4)
        /// function
        pub fn has_all_roles(
            &self,
            user: ::ethers::core::types::Address,
            roles: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([28, 214, 77, 244], (user, roles))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAnyRole` (0x514e62fc)
        /// function
        pub fn has_any_role(
            &self,
            user: ::ethers::core::types::Address,
            roles: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([81, 78, 98, 252], (user, roles))
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
        ///Calls the contract's `mint` (0x40c10f19)
        /// function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount))
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
        ///Calls the contract's `ordinalsFromRoles`
        /// (0x7359e41f) function
        pub fn ordinals_from_roles(
            &self,
            roles: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u8>>
        {
            self.0
                .method_hash([115, 89, 228, 31], roles)
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
        ///Calls the contract's
        /// `ownershipHandoverExpiresAt` (0xfee81cf4)
        /// function
        pub fn ownership_handover_expires_at(
            &self,
            pending_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([254, 232, 28, 244], pending_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownershipHandoverValidFor`
        /// (0xd7533f02) function
        pub fn ownership_handover_valid_for(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([215, 83, 63, 2], ())
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
        ///Calls the contract's `renounceRoles`
        /// (0x183a4f6e) function
        pub fn renounce_roles(
            &self,
            roles: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 58, 79, 110], roles)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestOwnershipHandover`
        /// (0x25692962) function
        pub fn request_ownership_handover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 105, 41, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRoles` (0x4a4ee7b1)
        /// function
        pub fn revoke_roles(
            &self,
            user: ::ethers::core::types::Address,
            roles: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 78, 231, 177], (user, roles))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rolesFromOrdinals`
        /// (0x13a661ed) function
        pub fn roles_from_ordinals(
            &self,
            ordinals: ::std::vec::Vec<u8>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([19, 166, 97, 237], ordinals)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rolesOf` (0x2de94807)
        /// function
        pub fn roles_of(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([45, 233, 72, 7], user)
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
        ///Gets the contract's `OwnershipHandoverCanceled`
        /// event
        pub fn ownership_handover_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipHandoverCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipHandoverRequested`
        /// event
        pub fn ownership_handover_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipHandoverRequestedFilter,
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
        ///Gets the contract's `RolesUpdated` event
        pub fn roles_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RolesUpdatedFilter,
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
            AccountingTokenEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for AccountingToken<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NewOwnerIsZeroAddress` with
    /// signature `NewOwnerIsZeroAddress()` and selector
    /// `0x7448fbae`
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
    #[etherror(name = "NewOwnerIsZeroAddress", abi = "NewOwnerIsZeroAddress()")]
    pub struct NewOwnerIsZeroAddress;
    ///Custom Error type `NoHandoverRequest` with signature
    /// `NoHandoverRequest()` and selector `0x6f5e8818`
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
    #[etherror(name = "NoHandoverRequest", abi = "NoHandoverRequest()")]
    pub struct NoHandoverRequest;
    ///Custom Error type `NotImplemented` with signature
    /// `NotImplemented()` and selector `0xd6234725`
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
    #[etherror(name = "NotImplemented", abi = "NotImplemented()")]
    pub struct NotImplemented;
    ///Custom Error type `Unauthorized` with signature
    /// `Unauthorized()` and selector `0x82b42900`
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
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Container type for all of the contract's custom
    /// errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum AccountingTokenErrors {
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        NoHandoverRequest(NoHandoverRequest),
        NotImplemented(NotImplemented),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AccountingTokenErrors {
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
            if let Ok(decoded)
                = <NewOwnerIsZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NewOwnerIsZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <NoHandoverRequest as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NoHandoverRequest(decoded));
            }
            if let Ok(decoded) =
                <NotImplemented as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotImplemented(decoded));
            }
            if let Ok(decoded) =
                <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AccountingTokenErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NewOwnerIsZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoHandoverRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotImplemented(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => {
                    ::ethers::core::abi::AbiEncode::encode(s)
                }
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AccountingTokenErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <NewOwnerIsZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoHandoverRequest as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotImplemented as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AccountingTokenErrors {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::NewOwnerIsZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoHandoverRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotImplemented(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AccountingTokenErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<NewOwnerIsZeroAddress> for AccountingTokenErrors {
        fn from(value: NewOwnerIsZeroAddress) -> Self {
            Self::NewOwnerIsZeroAddress(value)
        }
    }
    impl ::core::convert::From<NoHandoverRequest> for AccountingTokenErrors {
        fn from(value: NoHandoverRequest) -> Self {
            Self::NoHandoverRequest(value)
        }
    }
    impl ::core::convert::From<NotImplemented> for AccountingTokenErrors {
        fn from(value: NotImplemented) -> Self { Self::NotImplemented(value) }
    }
    impl ::core::convert::From<Unauthorized> for AccountingTokenErrors {
        fn from(value: Unauthorized) -> Self { Self::Unauthorized(value) }
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
        name = "OwnershipHandoverCanceled",
        abi = "OwnershipHandoverCanceled(address)"
    )]
    pub struct OwnershipHandoverCanceledFilter {
        #[ethevent(indexed)]
        pub pending_owner: ::ethers::core::types::Address,
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
        name = "OwnershipHandoverRequested",
        abi = "OwnershipHandoverRequested(address)"
    )]
    pub struct OwnershipHandoverRequestedFilter {
        #[ethevent(indexed)]
        pub pending_owner: ::ethers::core::types::Address,
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
        pub old_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "RolesUpdated", abi = "RolesUpdated(address,uint256)")]
    pub struct RolesUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub roles: ::ethers::core::types::U256,
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
    pub enum AccountingTokenEvents {
        ApprovalFilter(ApprovalFilter),
        OwnershipHandoverCanceledFilter(OwnershipHandoverCanceledFilter),
        OwnershipHandoverRequestedFilter(OwnershipHandoverRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RolesUpdatedFilter(RolesUpdatedFilter),
        SnapshotFilter(SnapshotFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for AccountingTokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(AccountingTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) =
                OwnershipHandoverCanceledFilter::decode_log(log)
            {
                return Ok(
                    AccountingTokenEvents::OwnershipHandoverCanceledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                OwnershipHandoverRequestedFilter::decode_log(log)
            {
                return Ok(
                    AccountingTokenEvents::OwnershipHandoverRequestedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AccountingTokenEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RolesUpdatedFilter::decode_log(log) {
                return Ok(AccountingTokenEvents::RolesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SnapshotFilter::decode_log(log) {
                return Ok(AccountingTokenEvents::SnapshotFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(AccountingTokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AccountingTokenEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipHandoverCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipHandoverRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RolesUpdatedFilter(element) => {
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
    impl ::core::convert::From<ApprovalFilter> for AccountingTokenEvents {
        fn from(value: ApprovalFilter) -> Self { Self::ApprovalFilter(value) }
    }
    impl ::core::convert::From<OwnershipHandoverCanceledFilter>
        for AccountingTokenEvents
    {
        fn from(value: OwnershipHandoverCanceledFilter) -> Self {
            Self::OwnershipHandoverCanceledFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverRequestedFilter>
        for AccountingTokenEvents
    {
        fn from(value: OwnershipHandoverRequestedFilter) -> Self {
            Self::OwnershipHandoverRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
        for AccountingTokenEvents
    {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RolesUpdatedFilter> for AccountingTokenEvents {
        fn from(value: RolesUpdatedFilter) -> Self {
            Self::RolesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SnapshotFilter> for AccountingTokenEvents {
        fn from(value: SnapshotFilter) -> Self { Self::SnapshotFilter(value) }
    }
    impl ::core::convert::From<TransferFilter> for AccountingTokenEvents {
        fn from(value: TransferFilter) -> Self { Self::TransferFilter(value) }
    }
    ///Container type for all input parameters for the
    /// `BURNER_ROLE` function with signature
    /// `BURNER_ROLE()` and selector `0x282c51f3`
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
    #[ethcall(name = "BURNER_ROLE", abi = "BURNER_ROLE()")]
    pub struct BurnerRoleCall;
    ///Container type for all input parameters for the
    /// `MINTER_ROLE` function with signature
    /// `MINTER_ROLE()` and selector `0xd5391393`
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
    #[ethcall(name = "MINTER_ROLE", abi = "MINTER_ROLE()")]
    pub struct MinterRoleCall;
    ///Container type for all input parameters for the
    /// `SNAPSHOT_ROLE` function with signature
    /// `SNAPSHOT_ROLE()` and selector `0x7028e2cd`
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
    #[ethcall(name = "SNAPSHOT_ROLE", abi = "SNAPSHOT_ROLE()")]
    pub struct SnapshotRoleCall;
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
    /// `burn` function with signature
    /// `burn(address,uint256)` and selector `0x9dc29fac`
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
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `cancelOwnershipHandover` function with signature
    /// `cancelOwnershipHandover()` and selector
    /// `0x54d1f13d`
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
        name = "cancelOwnershipHandover",
        abi = "cancelOwnershipHandover()"
    )]
    pub struct CancelOwnershipHandoverCall;
    ///Container type for all input parameters for the
    /// `completeOwnershipHandover` function with signature
    /// `completeOwnershipHandover(address)` and selector
    /// `0xf04e283e`
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
        name = "completeOwnershipHandover",
        abi = "completeOwnershipHandover(address)"
    )]
    pub struct CompleteOwnershipHandoverCall {
        pub pending_owner: ::ethers::core::types::Address,
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
    /// `grantRoles` function with signature
    /// `grantRoles(address,uint256)` and selector
    /// `0x1c10893f`
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
    #[ethcall(name = "grantRoles", abi = "grantRoles(address,uint256)")]
    pub struct GrantRolesCall {
        pub user: ::ethers::core::types::Address,
        pub roles: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `hasAllRoles` function with signature
    /// `hasAllRoles(address,uint256)` and selector
    /// `0x1cd64df4`
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
    #[ethcall(name = "hasAllRoles", abi = "hasAllRoles(address,uint256)")]
    pub struct HasAllRolesCall {
        pub user: ::ethers::core::types::Address,
        pub roles: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `hasAnyRole` function with signature
    /// `hasAnyRole(address,uint256)` and selector
    /// `0x514e62fc`
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
    #[ethcall(name = "hasAnyRole", abi = "hasAnyRole(address,uint256)")]
    pub struct HasAnyRoleCall {
        pub user: ::ethers::core::types::Address,
        pub roles: ::ethers::core::types::U256,
    }
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
    /// `mint` function with signature
    /// `mint(address,uint256)` and selector `0x40c10f19`
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    /// `ordinalsFromRoles` function with signature
    /// `ordinalsFromRoles(uint256)` and selector
    /// `0x7359e41f`
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
    #[ethcall(name = "ordinalsFromRoles", abi = "ordinalsFromRoles(uint256)")]
    pub struct OrdinalsFromRolesCall {
        pub roles: ::ethers::core::types::U256,
    }
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
    /// `ownershipHandoverExpiresAt` function with signature
    /// `ownershipHandoverExpiresAt(address)` and selector
    /// `0xfee81cf4`
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
        name = "ownershipHandoverExpiresAt",
        abi = "ownershipHandoverExpiresAt(address)"
    )]
    pub struct OwnershipHandoverExpiresAtCall {
        pub pending_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `ownershipHandoverValidFor` function with signature
    /// `ownershipHandoverValidFor()` and selector
    /// `0xd7533f02`
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
        name = "ownershipHandoverValidFor",
        abi = "ownershipHandoverValidFor()"
    )]
    pub struct OwnershipHandoverValidForCall;
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
    /// `renounceRoles` function with signature
    /// `renounceRoles(uint256)` and selector `0x183a4f6e`
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
    #[ethcall(name = "renounceRoles", abi = "renounceRoles(uint256)")]
    pub struct RenounceRolesCall {
        pub roles: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `requestOwnershipHandover` function with signature
    /// `requestOwnershipHandover()` and selector
    /// `0x25692962`
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
        name = "requestOwnershipHandover",
        abi = "requestOwnershipHandover()"
    )]
    pub struct RequestOwnershipHandoverCall;
    ///Container type for all input parameters for the
    /// `revokeRoles` function with signature
    /// `revokeRoles(address,uint256)` and selector
    /// `0x4a4ee7b1`
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
    #[ethcall(name = "revokeRoles", abi = "revokeRoles(address,uint256)")]
    pub struct RevokeRolesCall {
        pub user: ::ethers::core::types::Address,
        pub roles: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `rolesFromOrdinals` function with signature
    /// `rolesFromOrdinals(uint8[])` and selector
    /// `0x13a661ed`
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
    #[ethcall(name = "rolesFromOrdinals", abi = "rolesFromOrdinals(uint8[])")]
    pub struct RolesFromOrdinalsCall {
        pub ordinals: ::std::vec::Vec<u8>,
    }
    ///Container type for all input parameters for the
    /// `rolesOf` function with signature `rolesOf(address)`
    /// and selector `0x2de94807`
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
    #[ethcall(name = "rolesOf", abi = "rolesOf(address)")]
    pub struct RolesOfCall {
        pub user: ::ethers::core::types::Address,
    }
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
    pub enum AccountingTokenCalls {
        BurnerRole(BurnerRoleCall),
        MinterRole(MinterRoleCall),
        SnapshotRole(SnapshotRoleCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BalanceOfAt(BalanceOfAtCall),
        Burn(BurnCall),
        CancelOwnershipHandover(CancelOwnershipHandoverCall),
        CompleteOwnershipHandover(CompleteOwnershipHandoverCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        GrantRoles(GrantRolesCall),
        HasAllRoles(HasAllRolesCall),
        HasAnyRole(HasAnyRoleCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Mint(MintCall),
        Name(NameCall),
        OrdinalsFromRoles(OrdinalsFromRolesCall),
        Owner(OwnerCall),
        OwnershipHandoverExpiresAt(OwnershipHandoverExpiresAtCall),
        OwnershipHandoverValidFor(OwnershipHandoverValidForCall),
        RenounceOwnership(RenounceOwnershipCall),
        RenounceRoles(RenounceRolesCall),
        RequestOwnershipHandover(RequestOwnershipHandoverCall),
        RevokeRoles(RevokeRolesCall),
        RolesFromOrdinals(RolesFromOrdinalsCall),
        RolesOf(RolesOfCall),
        Snapshot(SnapshotCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        TotalSupplyAt(TotalSupplyAtCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for AccountingTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BurnerRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnerRole(decoded));
            }
            if let Ok(decoded) =
                <MinterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinterRole(decoded));
            }
            if let Ok(decoded) =
                <SnapshotRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SnapshotRole(decoded));
            }
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
                <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <CancelOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelOwnershipHandover(decoded));
            }
            if let Ok(decoded)
                = <CompleteOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompleteOwnershipHandover(decoded));
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
                <GrantRolesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GrantRoles(decoded));
            }
            if let Ok(decoded) =
                <HasAllRolesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HasAllRoles(decoded));
            }
            if let Ok(decoded) =
                <HasAnyRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasAnyRole(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <MintCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) =
                <NameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OrdinalsFromRolesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OrdinalsFromRoles(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <OwnershipHandoverExpiresAtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OwnershipHandoverExpiresAt(decoded));
            }
            if let Ok(decoded)
                = <OwnershipHandoverValidForCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OwnershipHandoverValidFor(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RenounceRolesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RenounceRoles(decoded));
            }
            if let Ok(decoded)
                = <RequestOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RequestOwnershipHandover(decoded));
            }
            if let Ok(decoded) =
                <RevokeRolesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RevokeRoles(decoded));
            }
            if let Ok(decoded)
                = <RolesFromOrdinalsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RolesFromOrdinals(decoded));
            }
            if let Ok(decoded) =
                <RolesOfCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RolesOf(decoded));
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
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AccountingTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BurnerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SnapshotRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Burn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRoles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllRoles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAnyRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrdinalsFromRoles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnershipHandoverExpiresAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnershipHandoverValidFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRoles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRoles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RolesFromOrdinals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RolesOf(element) => {
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
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AccountingTokenCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::BurnerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinterRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SnapshotRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Decimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRoles(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasAllRoles(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasAnyRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrdinalsFromRoles(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipHandoverExpiresAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipHandoverValidFor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRoles(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeRoles(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RolesFromOrdinals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RolesOf(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::TransferOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BurnerRoleCall> for AccountingTokenCalls {
        fn from(value: BurnerRoleCall) -> Self { Self::BurnerRole(value) }
    }
    impl ::core::convert::From<MinterRoleCall> for AccountingTokenCalls {
        fn from(value: MinterRoleCall) -> Self { Self::MinterRole(value) }
    }
    impl ::core::convert::From<SnapshotRoleCall> for AccountingTokenCalls {
        fn from(value: SnapshotRoleCall) -> Self { Self::SnapshotRole(value) }
    }
    impl ::core::convert::From<AllowanceCall> for AccountingTokenCalls {
        fn from(value: AllowanceCall) -> Self { Self::Allowance(value) }
    }
    impl ::core::convert::From<ApproveCall> for AccountingTokenCalls {
        fn from(value: ApproveCall) -> Self { Self::Approve(value) }
    }
    impl ::core::convert::From<BalanceOfCall> for AccountingTokenCalls {
        fn from(value: BalanceOfCall) -> Self { Self::BalanceOf(value) }
    }
    impl ::core::convert::From<BalanceOfAtCall> for AccountingTokenCalls {
        fn from(value: BalanceOfAtCall) -> Self { Self::BalanceOfAt(value) }
    }
    impl ::core::convert::From<BurnCall> for AccountingTokenCalls {
        fn from(value: BurnCall) -> Self { Self::Burn(value) }
    }
    impl ::core::convert::From<CancelOwnershipHandoverCall>
        for AccountingTokenCalls
    {
        fn from(value: CancelOwnershipHandoverCall) -> Self {
            Self::CancelOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<CompleteOwnershipHandoverCall>
        for AccountingTokenCalls
    {
        fn from(value: CompleteOwnershipHandoverCall) -> Self {
            Self::CompleteOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for AccountingTokenCalls {
        fn from(value: DecimalsCall) -> Self { Self::Decimals(value) }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for AccountingTokenCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<GrantRolesCall> for AccountingTokenCalls {
        fn from(value: GrantRolesCall) -> Self { Self::GrantRoles(value) }
    }
    impl ::core::convert::From<HasAllRolesCall> for AccountingTokenCalls {
        fn from(value: HasAllRolesCall) -> Self { Self::HasAllRoles(value) }
    }
    impl ::core::convert::From<HasAnyRoleCall> for AccountingTokenCalls {
        fn from(value: HasAnyRoleCall) -> Self { Self::HasAnyRole(value) }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for AccountingTokenCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<MintCall> for AccountingTokenCalls {
        fn from(value: MintCall) -> Self { Self::Mint(value) }
    }
    impl ::core::convert::From<NameCall> for AccountingTokenCalls {
        fn from(value: NameCall) -> Self { Self::Name(value) }
    }
    impl ::core::convert::From<OrdinalsFromRolesCall> for AccountingTokenCalls {
        fn from(value: OrdinalsFromRolesCall) -> Self {
            Self::OrdinalsFromRoles(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AccountingTokenCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<OwnershipHandoverExpiresAtCall>
        for AccountingTokenCalls
    {
        fn from(value: OwnershipHandoverExpiresAtCall) -> Self {
            Self::OwnershipHandoverExpiresAt(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverValidForCall>
        for AccountingTokenCalls
    {
        fn from(value: OwnershipHandoverValidForCall) -> Self {
            Self::OwnershipHandoverValidFor(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AccountingTokenCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RenounceRolesCall> for AccountingTokenCalls {
        fn from(value: RenounceRolesCall) -> Self { Self::RenounceRoles(value) }
    }
    impl ::core::convert::From<RequestOwnershipHandoverCall>
        for AccountingTokenCalls
    {
        fn from(value: RequestOwnershipHandoverCall) -> Self {
            Self::RequestOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<RevokeRolesCall> for AccountingTokenCalls {
        fn from(value: RevokeRolesCall) -> Self { Self::RevokeRoles(value) }
    }
    impl ::core::convert::From<RolesFromOrdinalsCall> for AccountingTokenCalls {
        fn from(value: RolesFromOrdinalsCall) -> Self {
            Self::RolesFromOrdinals(value)
        }
    }
    impl ::core::convert::From<RolesOfCall> for AccountingTokenCalls {
        fn from(value: RolesOfCall) -> Self { Self::RolesOf(value) }
    }
    impl ::core::convert::From<SnapshotCall> for AccountingTokenCalls {
        fn from(value: SnapshotCall) -> Self { Self::Snapshot(value) }
    }
    impl ::core::convert::From<SymbolCall> for AccountingTokenCalls {
        fn from(value: SymbolCall) -> Self { Self::Symbol(value) }
    }
    impl ::core::convert::From<TotalSupplyCall> for AccountingTokenCalls {
        fn from(value: TotalSupplyCall) -> Self { Self::TotalSupply(value) }
    }
    impl ::core::convert::From<TotalSupplyAtCall> for AccountingTokenCalls {
        fn from(value: TotalSupplyAtCall) -> Self { Self::TotalSupplyAt(value) }
    }
    impl ::core::convert::From<TransferCall> for AccountingTokenCalls {
        fn from(value: TransferCall) -> Self { Self::Transfer(value) }
    }
    impl ::core::convert::From<TransferFromCall> for AccountingTokenCalls {
        fn from(value: TransferFromCall) -> Self { Self::TransferFrom(value) }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AccountingTokenCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the
    /// `BURNER_ROLE` function with signature
    /// `BURNER_ROLE()` and selector `0x282c51f3`
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
    pub struct BurnerRoleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `MINTER_ROLE` function with signature
    /// `MINTER_ROLE()` and selector `0xd5391393`
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
    pub struct MinterRoleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `SNAPSHOT_ROLE` function with signature
    /// `SNAPSHOT_ROLE()` and selector `0x7028e2cd`
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
    pub struct SnapshotRoleReturn(pub ::ethers::core::types::U256);
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
    /// `hasAllRoles` function with signature
    /// `hasAllRoles(address,uint256)` and selector
    /// `0x1cd64df4`
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
    pub struct HasAllRolesReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the
    /// `hasAnyRole` function with signature
    /// `hasAnyRole(address,uint256)` and selector
    /// `0x514e62fc`
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
    pub struct HasAnyRoleReturn {
        pub result: bool,
    }
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
    /// `ordinalsFromRoles` function with signature
    /// `ordinalsFromRoles(uint256)` and selector
    /// `0x7359e41f`
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
    pub struct OrdinalsFromRolesReturn {
        pub ordinals: ::std::vec::Vec<u8>,
    }
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
    pub struct OwnerReturn {
        pub result: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the
    /// `ownershipHandoverExpiresAt` function with signature
    /// `ownershipHandoverExpiresAt(address)` and selector
    /// `0xfee81cf4`
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
    pub struct OwnershipHandoverExpiresAtReturn {
        pub result: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the
    /// `ownershipHandoverValidFor` function with signature
    /// `ownershipHandoverValidFor()` and selector
    /// `0xd7533f02`
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
    pub struct OwnershipHandoverValidForReturn(pub u64);
    ///Container type for all return fields from the
    /// `rolesFromOrdinals` function with signature
    /// `rolesFromOrdinals(uint8[])` and selector
    /// `0x13a661ed`
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
    pub struct RolesFromOrdinalsReturn {
        pub roles: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the
    /// `rolesOf` function with signature `rolesOf(address)`
    /// and selector `0x2de94807`
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
    pub struct RolesOfReturn {
        pub roles: ::ethers::core::types::U256,
    }
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
    pub struct SnapshotReturn(pub ::ethers::core::types::U256);
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
