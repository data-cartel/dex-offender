pub use reward_token::*;
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
pub mod reward_token {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
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
    pub static REWARDTOKEN_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k)2\xBB\xB0\xB92\x10*7\xB5\xB2\xB7`\xA1\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x14\x95\xD5`\xEA\x1B\x81RP\x81`\x03\x90\x81a\0a\x91\x90a\x01\xB8V[P`\x04a\0n\x82\x82a\x01\xB8V[PPPa\0\x803a\0\x90` \x1B` \x1CV[a\0\x8B3`\x01a\0\xCCV[a\x02wV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80`\0\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01aWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x01\xB3W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x01\x90WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xAFW\x82\x81U`\x01\x01a\x01\x9CV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xD1Wa\x01\xD1a\x01\x17V[a\x01\xE5\x81a\x01\xDF\x84Ta\x01-V[\x84a\x01gV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x02\x1AW`\0\x84\x15a\x02\x02WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x01\xAFV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x02IW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x02*V[P\x85\x82\x10\x15a\x02gW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x10\xEB\x80a\x02\x86`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xC2W`\x005`\xE0\x1C\x80cQNb\xFC\x11a\0\xF7W\x80c\xA4W\xC2\xD7\x11a\0\x95W\x80c\xDDb\xED>\x11a\0dW\x80c\xDDb\xED>\x14a\x04\xECW\x80c\xF0N(>\x14a\x05\x0CW\x80c\xF2\xFD\xE3\x8B\x14a\x05\x1FW\x80c\xFE\xE8\x1C\xF4\x14a\x052W`\0\x80\xFD[\x80c\xA4W\xC2\xD7\x14a\x04yW\x80c\xA9\x05\x9C\xBB\x14a\x04\x99W\x80c\xD59\x13\x93\x14a\x04\xB9W\x80c\xD7S?\x02\x14a\x04\xCEW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\0\xD1W\x80cqP\x18\xA6\x14a\x04\x03W\x80csY\xE4\x1F\x14a\x04\x0BW\x80c\x8D\xA5\xCB[\x14a\x048W\x80c\x95\xD8\x9BA\x14a\x04dW`\0\x80\xFD[\x80cQNb\xFC\x14a\x03\x8EW\x80cT\xD1\xF1=\x14a\x03\xC5W\x80cp\xA0\x821\x14a\x03\xCDW`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01dW\x80c1<\xE5g\x11a\x01>W\x80c1<\xE5g\x14a\x03\x1FW\x80c9P\x93Q\x14a\x03;W\x80c@\xC1\x0F\x19\x14a\x03[W\x80cJN\xE7\xB1\x14a\x03{W`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x02\xC4W\x80c%i)b\x14a\x02\xE4W\x80c-\xE9H\x07\x14a\x02\xECW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x01\xA0W\x80c\x18\x16\r\xDD\x14a\x02PW\x80c\x18:On\x14a\x02eW\x80c\x1C\x10\x89?\x14a\x02zW\x80c\x1C\xD6M\xF4\x14a\x02\x8DW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\xC7W\x80c\t^\xA7\xB3\x14a\x01\xF2W\x80c\x13\xA6a\xED\x14a\x02\"W[`\0\x80\xFD[4\x80\x15a\x01\xD3W`\0\x80\xFD[Pa\x01\xDCa\x05eV[`@Qa\x01\xE9\x91\x90a\r\xEEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFEW`\0\x80\xFD[Pa\x02\x12a\x02\r6`\x04a\x0EYV[a\x05\xF7V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE9V[4\x80\x15a\x02.W`\0\x80\xFD[Pa\x02Ba\x02=6`\x04a\x0E\xAAV[a\x06\x11V[`@Q\x90\x81R` \x01a\x01\xE9V[4\x80\x15a\x02\\W`\0\x80\xFD[P`\x02Ta\x02BV[a\x02xa\x02s6`\x04a\x0FoV[a\x06:V[\0[a\x02xa\x02\x886`\x04a\x0EYV[a\x06GV[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\x12a\x02\xA86`\x04a\x0EYV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x02\xD0W`\0\x80\xFD[Pa\x02\x12a\x02\xDF6`\x04a\x0F\x88V[a\x06]V[a\x02xa\x06\x81V[4\x80\x15a\x02\xF8W`\0\x80\xFD[Pa\x02Ba\x03\x076`\x04a\x0F\xC4V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03+W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x01\xE9V[4\x80\x15a\x03GW`\0\x80\xFD[Pa\x02\x12a\x03V6`\x04a\x0EYV[a\x06\xD1V[4\x80\x15a\x03gW`\0\x80\xFD[Pa\x02xa\x03v6`\x04a\x0EYV[a\x06\xF3V[a\x02xa\x03\x896`\x04a\x0EYV[a\x07\rV[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x02\x12a\x03\xA96`\x04a\x0EYV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02xa\x07\x1FV[4\x80\x15a\x03\xD9W`\0\x80\xFD[Pa\x02Ba\x03\xE86`\x04a\x0F\xC4V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02xa\x07[V[4\x80\x15a\x04\x17W`\0\x80\xFD[Pa\x04+a\x04&6`\x04a\x0FoV[a\x07oV[`@Qa\x01\xE9\x91\x90a\x0F\xE6V[4\x80\x15a\x04DW`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE9V[4\x80\x15a\x04pW`\0\x80\xFD[Pa\x01\xDCa\x07\xA8V[4\x80\x15a\x04\x85W`\0\x80\xFD[Pa\x02\x12a\x04\x946`\x04a\x0EYV[a\x07\xB7V[4\x80\x15a\x04\xA5W`\0\x80\xFD[Pa\x02\x12a\x04\xB46`\x04a\x0EYV[a\x087V[4\x80\x15a\x04\xC5W`\0\x80\xFD[Pa\x02B`\x01\x81V[4\x80\x15a\x04\xDAW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x01\xE9V[4\x80\x15a\x04\xF8W`\0\x80\xFD[Pa\x02Ba\x05\x076`\x04a\x10-V[a\x08EV[a\x02xa\x05\x1A6`\x04a\x0F\xC4V[a\x08pV[a\x02xa\x05-6`\x04a\x0F\xC4V[a\x08\xADV[4\x80\x15a\x05>W`\0\x80\xFD[Pa\x02Ba\x05M6`\x04a\x0F\xC4V[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[```\x03\x80Ta\x05t\x90a\x10`V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xA0\x90a\x10`V[\x80\x15a\x05\xEDW\x80`\x1F\x10a\x05\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xEDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\x05\x81\x85\x85a\x08\xD4V[`\x01\x91PP[\x92\x91PPV[`\0\x81Q`\x05\x1B[\x80\x15a\x064W\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x06\x19V[P\x91\x90PV[a\x06D3\x82a\t\xF8V[PV[a\x06Oa\nGV[a\x06Y\x82\x82a\nbV[PPV[`\x003a\x06k\x85\x82\x85a\n\xADV[a\x06v\x85\x85\x85a\x0B'V[P`\x01\x94\x93PPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\x003a\x06\x05\x81\x85\x85a\x06\xE4\x83\x83a\x08EV[a\x06\xEE\x91\x90a\x10\x94V[a\x08\xD4V[`\x01a\x06\xFE\x81a\x0C\xCBV[a\x07\x08\x83\x83a\x0C\xF1V[PPPV[a\x07\x15a\nGV[a\x06Y\x82\x82a\t\xF8V[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x07ca\nGV[a\x07m`\0a\r\xB0V[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\x07zWPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x04\x80Ta\x05t\x90a\x10`V[`\x003\x81a\x07\xC5\x82\x86a\x08EV[\x90P\x83\x81\x10\x15a\x08*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06v\x82\x86\x86\x84\x03a\x08\xD4V[`\x003a\x06\x05\x81\x85\x85a\x0B'V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x08xa\nGV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x08\xA0Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x06D\x81a\r\xB0V[a\x08\xB5a\nGV[\x80``\x1Ba\x08\xCBWctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x06D\x81a\r\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\t6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\x07mWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0a\n\xB9\x84\x84a\x08EV[\x90P`\0\x19\x81\x14a\x0B!W\x81\x81\x10\x15a\x0B\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x08!V[a\x0B!\x84\x84\x84\x84\x03a\x08\xD4V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0B\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0CeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x87\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x92Q\x85\x81R\x90\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x0B!V[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x06DWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\rGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x08!V[\x80`\x02`\0\x82\x82Ta\rY\x91\x90a\x10\x94V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x0E\x1CW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0E\0V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0ETW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0ElW`\0\x80\xFD[a\x0Eu\x83a\x0E=V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\xFF\x81\x16\x81\x14a\x0ETW`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x0E\xBDW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xD5W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\xE9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0E\xFBWa\x0E\xFBa\x0E\x83V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x0F Wa\x0F a\x0E\x83V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x0F>W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x0FcWa\x0FT\x85a\x0E\x99V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x0FCV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\x81W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x9DW`\0\x80\xFD[a\x0F\xA6\x84a\x0E=V[\x92Pa\x0F\xB4` \x85\x01a\x0E=V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\xD6W`\0\x80\xFD[a\x0F\xDF\x82a\x0E=V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10!W\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10\x02V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10@W`\0\x80\xFD[a\x10I\x83a\x0E=V[\x91Pa\x10W` \x84\x01a\x0E=V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x10tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x064WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\x0BWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x9A}\x86K`#\xBF\x0Be)\xDF\xC1U~\xAC\xEAT%\x81h\x8C\xFA\x91\xD9\xD4\xFD\x1E~\xEF\xD8\x893dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static REWARDTOKEN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xC2W`\x005`\xE0\x1C\x80cQNb\xFC\x11a\0\xF7W\x80c\xA4W\xC2\xD7\x11a\0\x95W\x80c\xDDb\xED>\x11a\0dW\x80c\xDDb\xED>\x14a\x04\xECW\x80c\xF0N(>\x14a\x05\x0CW\x80c\xF2\xFD\xE3\x8B\x14a\x05\x1FW\x80c\xFE\xE8\x1C\xF4\x14a\x052W`\0\x80\xFD[\x80c\xA4W\xC2\xD7\x14a\x04yW\x80c\xA9\x05\x9C\xBB\x14a\x04\x99W\x80c\xD59\x13\x93\x14a\x04\xB9W\x80c\xD7S?\x02\x14a\x04\xCEW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\0\xD1W\x80cqP\x18\xA6\x14a\x04\x03W\x80csY\xE4\x1F\x14a\x04\x0BW\x80c\x8D\xA5\xCB[\x14a\x048W\x80c\x95\xD8\x9BA\x14a\x04dW`\0\x80\xFD[\x80cQNb\xFC\x14a\x03\x8EW\x80cT\xD1\xF1=\x14a\x03\xC5W\x80cp\xA0\x821\x14a\x03\xCDW`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01dW\x80c1<\xE5g\x11a\x01>W\x80c1<\xE5g\x14a\x03\x1FW\x80c9P\x93Q\x14a\x03;W\x80c@\xC1\x0F\x19\x14a\x03[W\x80cJN\xE7\xB1\x14a\x03{W`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x02\xC4W\x80c%i)b\x14a\x02\xE4W\x80c-\xE9H\x07\x14a\x02\xECW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x01\xA0W\x80c\x18\x16\r\xDD\x14a\x02PW\x80c\x18:On\x14a\x02eW\x80c\x1C\x10\x89?\x14a\x02zW\x80c\x1C\xD6M\xF4\x14a\x02\x8DW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\xC7W\x80c\t^\xA7\xB3\x14a\x01\xF2W\x80c\x13\xA6a\xED\x14a\x02\"W[`\0\x80\xFD[4\x80\x15a\x01\xD3W`\0\x80\xFD[Pa\x01\xDCa\x05eV[`@Qa\x01\xE9\x91\x90a\r\xEEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFEW`\0\x80\xFD[Pa\x02\x12a\x02\r6`\x04a\x0EYV[a\x05\xF7V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE9V[4\x80\x15a\x02.W`\0\x80\xFD[Pa\x02Ba\x02=6`\x04a\x0E\xAAV[a\x06\x11V[`@Q\x90\x81R` \x01a\x01\xE9V[4\x80\x15a\x02\\W`\0\x80\xFD[P`\x02Ta\x02BV[a\x02xa\x02s6`\x04a\x0FoV[a\x06:V[\0[a\x02xa\x02\x886`\x04a\x0EYV[a\x06GV[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\x12a\x02\xA86`\x04a\x0EYV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x02\xD0W`\0\x80\xFD[Pa\x02\x12a\x02\xDF6`\x04a\x0F\x88V[a\x06]V[a\x02xa\x06\x81V[4\x80\x15a\x02\xF8W`\0\x80\xFD[Pa\x02Ba\x03\x076`\x04a\x0F\xC4V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03+W`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x01\xE9V[4\x80\x15a\x03GW`\0\x80\xFD[Pa\x02\x12a\x03V6`\x04a\x0EYV[a\x06\xD1V[4\x80\x15a\x03gW`\0\x80\xFD[Pa\x02xa\x03v6`\x04a\x0EYV[a\x06\xF3V[a\x02xa\x03\x896`\x04a\x0EYV[a\x07\rV[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x02\x12a\x03\xA96`\x04a\x0EYV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02xa\x07\x1FV[4\x80\x15a\x03\xD9W`\0\x80\xFD[Pa\x02Ba\x03\xE86`\x04a\x0F\xC4V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02xa\x07[V[4\x80\x15a\x04\x17W`\0\x80\xFD[Pa\x04+a\x04&6`\x04a\x0FoV[a\x07oV[`@Qa\x01\xE9\x91\x90a\x0F\xE6V[4\x80\x15a\x04DW`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE9V[4\x80\x15a\x04pW`\0\x80\xFD[Pa\x01\xDCa\x07\xA8V[4\x80\x15a\x04\x85W`\0\x80\xFD[Pa\x02\x12a\x04\x946`\x04a\x0EYV[a\x07\xB7V[4\x80\x15a\x04\xA5W`\0\x80\xFD[Pa\x02\x12a\x04\xB46`\x04a\x0EYV[a\x087V[4\x80\x15a\x04\xC5W`\0\x80\xFD[Pa\x02B`\x01\x81V[4\x80\x15a\x04\xDAW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x01\xE9V[4\x80\x15a\x04\xF8W`\0\x80\xFD[Pa\x02Ba\x05\x076`\x04a\x10-V[a\x08EV[a\x02xa\x05\x1A6`\x04a\x0F\xC4V[a\x08pV[a\x02xa\x05-6`\x04a\x0F\xC4V[a\x08\xADV[4\x80\x15a\x05>W`\0\x80\xFD[Pa\x02Ba\x05M6`\x04a\x0F\xC4V[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[```\x03\x80Ta\x05t\x90a\x10`V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xA0\x90a\x10`V[\x80\x15a\x05\xEDW\x80`\x1F\x10a\x05\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xEDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\x05\x81\x85\x85a\x08\xD4V[`\x01\x91PP[\x92\x91PPV[`\0\x81Q`\x05\x1B[\x80\x15a\x064W\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x06\x19V[P\x91\x90PV[a\x06D3\x82a\t\xF8V[PV[a\x06Oa\nGV[a\x06Y\x82\x82a\nbV[PPV[`\x003a\x06k\x85\x82\x85a\n\xADV[a\x06v\x85\x85\x85a\x0B'V[P`\x01\x94\x93PPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\x003a\x06\x05\x81\x85\x85a\x06\xE4\x83\x83a\x08EV[a\x06\xEE\x91\x90a\x10\x94V[a\x08\xD4V[`\x01a\x06\xFE\x81a\x0C\xCBV[a\x07\x08\x83\x83a\x0C\xF1V[PPPV[a\x07\x15a\nGV[a\x06Y\x82\x82a\t\xF8V[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x07ca\nGV[a\x07m`\0a\r\xB0V[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\x07zWPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x04\x80Ta\x05t\x90a\x10`V[`\x003\x81a\x07\xC5\x82\x86a\x08EV[\x90P\x83\x81\x10\x15a\x08*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06v\x82\x86\x86\x84\x03a\x08\xD4V[`\x003a\x06\x05\x81\x85\x85a\x0B'V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x08xa\nGV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x08\xA0Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x06D\x81a\r\xB0V[a\x08\xB5a\nGV[\x80``\x1Ba\x08\xCBWctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x06D\x81a\r\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\t6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\x07mWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0a\n\xB9\x84\x84a\x08EV[\x90P`\0\x19\x81\x14a\x0B!W\x81\x81\x10\x15a\x0B\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x08!V[a\x0B!\x84\x84\x84\x84\x03a\x08\xD4V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0B\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0CeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x08!V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x87\x03\x90U\x93\x87\x16\x80\x83R\x91\x84\x90 \x80T\x87\x01\x90U\x92Q\x85\x81R\x90\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x0B!V[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x06DWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\rGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x08!V[\x80`\x02`\0\x82\x82Ta\rY\x91\x90a\x10\x94V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x0E\x1CW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0E\0V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0ETW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0ElW`\0\x80\xFD[a\x0Eu\x83a\x0E=V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\xFF\x81\x16\x81\x14a\x0ETW`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x0E\xBDW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xD5W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\xE9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0E\xFBWa\x0E\xFBa\x0E\x83V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x0F Wa\x0F a\x0E\x83V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x0F>W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x0FcWa\x0FT\x85a\x0E\x99V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x0FCV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\x81W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x9DW`\0\x80\xFD[a\x0F\xA6\x84a\x0E=V[\x92Pa\x0F\xB4` \x85\x01a\x0E=V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\xD6W`\0\x80\xFD[a\x0F\xDF\x82a\x0E=V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10!W\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10\x02V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10@W`\0\x80\xFD[a\x10I\x83a\x0E=V[\x91Pa\x10W` \x84\x01a\x0E=V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x10tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x064WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\x0BWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x9A}\x86K`#\xBF\x0Be)\xDF\xC1U~\xAC\xEAT%\x81h\x8C\xFA\x91\xD9\xD4\xFD\x1E~\xEF\xD8\x893dsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static REWARDTOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct RewardToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RewardToken<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for RewardToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for RewardToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for RewardToken<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RewardToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RewardToken<M> {
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
                REWARDTOKEN_ABI.clone(),
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
                REWARDTOKEN_ABI.clone(),
                REWARDTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
            RewardTokenEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for RewardToken<M>
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NoHandoverRequest", abi = "NoHandoverRequest()")]
    pub struct NoHandoverRequest;
    ///Custom Error type `Unauthorized` with signature
    /// `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
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
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum RewardTokenErrors {
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        NoHandoverRequest(NoHandoverRequest),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RewardTokenErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <NewOwnerIsZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
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
                <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RewardTokenErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NewOwnerIsZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoHandoverRequest(element) => {
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
    impl ::ethers::contract::ContractRevert for RewardTokenErrors {
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
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RewardTokenErrors {
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
                Self::Unauthorized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RewardTokenErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<NewOwnerIsZeroAddress> for RewardTokenErrors {
        fn from(value: NewOwnerIsZeroAddress) -> Self {
            Self::NewOwnerIsZeroAddress(value)
        }
    }
    impl ::core::convert::From<NoHandoverRequest> for RewardTokenErrors {
        fn from(value: NoHandoverRequest) -> Self {
            Self::NoHandoverRequest(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for RewardTokenErrors {
        fn from(value: Unauthorized) -> Self { Self::Unauthorized(value) }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum RewardTokenEvents {
        ApprovalFilter(ApprovalFilter),
        OwnershipHandoverCanceledFilter(OwnershipHandoverCanceledFilter),
        OwnershipHandoverRequestedFilter(OwnershipHandoverRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RolesUpdatedFilter(RolesUpdatedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for RewardTokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(RewardTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) =
                OwnershipHandoverCanceledFilter::decode_log(log)
            {
                return Ok(RewardTokenEvents::OwnershipHandoverCanceledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                OwnershipHandoverRequestedFilter::decode_log(log)
            {
                return Ok(
                    RewardTokenEvents::OwnershipHandoverRequestedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(RewardTokenEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RolesUpdatedFilter::decode_log(log) {
                return Ok(RewardTokenEvents::RolesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(RewardTokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RewardTokenEvents {
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
                Self::TransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for RewardTokenEvents {
        fn from(value: ApprovalFilter) -> Self { Self::ApprovalFilter(value) }
    }
    impl ::core::convert::From<OwnershipHandoverCanceledFilter>
        for RewardTokenEvents
    {
        fn from(value: OwnershipHandoverCanceledFilter) -> Self {
            Self::OwnershipHandoverCanceledFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverRequestedFilter>
        for RewardTokenEvents
    {
        fn from(value: OwnershipHandoverRequestedFilter) -> Self {
            Self::OwnershipHandoverRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for RewardTokenEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RolesUpdatedFilter> for RewardTokenEvents {
        fn from(value: RolesUpdatedFilter) -> Self {
            Self::RolesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for RewardTokenEvents {
        fn from(value: TransferFilter) -> Self { Self::TransferFilter(value) }
    }
    ///Container type for all input parameters for the
    /// `MINTER_ROLE` function with signature
    /// `MINTER_ROLE()` and selector `0xd5391393`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MINTER_ROLE", abi = "MINTER_ROLE()")]
    pub struct MinterRoleCall;
    ///Container type for all input parameters for the
    /// `allowance` function with signature
    /// `allowance(address,address)` and selector
    /// `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    /// `cancelOwnershipHandover` function with signature
    /// `cancelOwnershipHandover()` and selector
    /// `0x54d1f13d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    /// `symbol` function with signature `symbol()` and
    /// selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum RewardTokenCalls {
        MinterRole(MinterRoleCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
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
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for RewardTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <MinterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinterRole(decoded));
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
            if let Ok(decoded) = <CancelOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelOwnershipHandover(decoded));
            }
            if let Ok(decoded) = <CompleteOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteOwnershipHandover(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
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
            if let Ok(decoded) = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
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
            if let Ok(decoded) = <OrdinalsFromRolesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OrdinalsFromRoles(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <OwnershipHandoverExpiresAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnershipHandoverExpiresAt(decoded));
            }
            if let Ok(decoded) = <OwnershipHandoverValidForCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnershipHandoverValidFor(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
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
            if let Ok(decoded) = <RequestOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
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
            if let Ok(decoded) = <RolesFromOrdinalsCall as ::ethers::core::abi::AbiDecode>::decode(
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
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RewardTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MinterRole(element) => {
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
    impl ::core::fmt::Display for RewardTokenCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::MinterRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Allowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<MinterRoleCall> for RewardTokenCalls {
        fn from(value: MinterRoleCall) -> Self { Self::MinterRole(value) }
    }
    impl ::core::convert::From<AllowanceCall> for RewardTokenCalls {
        fn from(value: AllowanceCall) -> Self { Self::Allowance(value) }
    }
    impl ::core::convert::From<ApproveCall> for RewardTokenCalls {
        fn from(value: ApproveCall) -> Self { Self::Approve(value) }
    }
    impl ::core::convert::From<BalanceOfCall> for RewardTokenCalls {
        fn from(value: BalanceOfCall) -> Self { Self::BalanceOf(value) }
    }
    impl ::core::convert::From<CancelOwnershipHandoverCall> for RewardTokenCalls {
        fn from(value: CancelOwnershipHandoverCall) -> Self {
            Self::CancelOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<CompleteOwnershipHandoverCall> for RewardTokenCalls {
        fn from(value: CompleteOwnershipHandoverCall) -> Self {
            Self::CompleteOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for RewardTokenCalls {
        fn from(value: DecimalsCall) -> Self { Self::Decimals(value) }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for RewardTokenCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<GrantRolesCall> for RewardTokenCalls {
        fn from(value: GrantRolesCall) -> Self { Self::GrantRoles(value) }
    }
    impl ::core::convert::From<HasAllRolesCall> for RewardTokenCalls {
        fn from(value: HasAllRolesCall) -> Self { Self::HasAllRoles(value) }
    }
    impl ::core::convert::From<HasAnyRoleCall> for RewardTokenCalls {
        fn from(value: HasAnyRoleCall) -> Self { Self::HasAnyRole(value) }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for RewardTokenCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<MintCall> for RewardTokenCalls {
        fn from(value: MintCall) -> Self { Self::Mint(value) }
    }
    impl ::core::convert::From<NameCall> for RewardTokenCalls {
        fn from(value: NameCall) -> Self { Self::Name(value) }
    }
    impl ::core::convert::From<OrdinalsFromRolesCall> for RewardTokenCalls {
        fn from(value: OrdinalsFromRolesCall) -> Self {
            Self::OrdinalsFromRoles(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RewardTokenCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<OwnershipHandoverExpiresAtCall>
        for RewardTokenCalls
    {
        fn from(value: OwnershipHandoverExpiresAtCall) -> Self {
            Self::OwnershipHandoverExpiresAt(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverValidForCall> for RewardTokenCalls {
        fn from(value: OwnershipHandoverValidForCall) -> Self {
            Self::OwnershipHandoverValidFor(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for RewardTokenCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RenounceRolesCall> for RewardTokenCalls {
        fn from(value: RenounceRolesCall) -> Self { Self::RenounceRoles(value) }
    }
    impl ::core::convert::From<RequestOwnershipHandoverCall> for RewardTokenCalls {
        fn from(value: RequestOwnershipHandoverCall) -> Self {
            Self::RequestOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<RevokeRolesCall> for RewardTokenCalls {
        fn from(value: RevokeRolesCall) -> Self { Self::RevokeRoles(value) }
    }
    impl ::core::convert::From<RolesFromOrdinalsCall> for RewardTokenCalls {
        fn from(value: RolesFromOrdinalsCall) -> Self {
            Self::RolesFromOrdinals(value)
        }
    }
    impl ::core::convert::From<RolesOfCall> for RewardTokenCalls {
        fn from(value: RolesOfCall) -> Self { Self::RolesOf(value) }
    }
    impl ::core::convert::From<SymbolCall> for RewardTokenCalls {
        fn from(value: SymbolCall) -> Self { Self::Symbol(value) }
    }
    impl ::core::convert::From<TotalSupplyCall> for RewardTokenCalls {
        fn from(value: TotalSupplyCall) -> Self { Self::TotalSupply(value) }
    }
    impl ::core::convert::From<TransferCall> for RewardTokenCalls {
        fn from(value: TransferCall) -> Self { Self::Transfer(value) }
    }
    impl ::core::convert::From<TransferFromCall> for RewardTokenCalls {
        fn from(value: TransferFromCall) -> Self { Self::TransferFrom(value) }
    }
    impl ::core::convert::From<TransferOwnershipCall> for RewardTokenCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the
    /// `MINTER_ROLE` function with signature
    /// `MINTER_ROLE()` and selector `0xd5391393`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinterRoleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `allowance` function with signature
    /// `allowance(address,address)` and selector
    /// `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `decimals` function with signature `decimals()` and
    /// selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    /// `symbol` function with signature `symbol()` and
    /// selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TransferFromReturn(pub bool);
}
