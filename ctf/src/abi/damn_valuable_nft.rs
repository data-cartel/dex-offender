pub use damn_valuable_nft::*;
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
pub mod damn_valuable_nft {
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("safeMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("tokenIdCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenIdCounter"),
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
    pub static DAMNVALUABLENFT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x11\x18[[\x95\x98[\x1DXX\x9B\x19S\x91\x95`\x8A\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x11\x15\x93\x91\x95`\xDA\x1B\x81RP\x81`\0\x90\x81b\0\0i\x91\x90b\0\x01\xCBV[P`\x01b\0\0x\x82\x82b\0\x01\xCBV[PPPb\0\0\x8C3b\0\0\x9F` \x1B` \x1CV[b\0\0\x993`\x01b\0\0\xDBV[b\0\x02\x97V[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80`\0\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01QW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01rWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xC6W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\xA1WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\xC2W\x82\x81U`\x01\x01b\0\x01\xADV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01\xE7Wb\0\x01\xE7b\0\x01&V[b\0\x01\xFF\x81b\0\x01\xF8\x84Tb\0\x01<V[\x84b\0\x01xV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x027W`\0\x84\x15b\0\x02\x1EWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\xC2V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02hW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02GV[P\x85\x82\x10\x15b\0\x02\x87W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x1B\xAA\x80b\0\x02\xA7`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xEEW`\x005`\xE0\x1C\x80cT\xD1\xF1=\x11a\x01\rW\x80c\xA2,\xB4e\x11a\0\xA0W\x80c\xD7S?\x02\x11a\0oW\x80c\xD7S?\x02\x14a\x05jW\x80c\xE9\x85\xE9\xC5\x14a\x05\x88W\x80c\xF0N(>\x14a\x05\xD1W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE4W\x80c\xFE\xE8\x1C\xF4\x14a\x05\xF7W`\0\x80\xFD[\x80c\xA2,\xB4e\x14a\x04\xF5W\x80c\xB8\x8DO\xDE\x14a\x05\x15W\x80c\xC8{V\xDD\x14a\x055W\x80c\xD59\x13\x93\x14a\x05UW`\0\x80\xFD[\x80csY\xE4\x1F\x11a\0\xDCW\x80csY\xE4\x1F\x14a\x04\x84W\x80c\x8D\xA5\xCB[\x14a\x04\xB1W\x80c\x95\xD8\x9BA\x14a\x04\xCAW\x80c\x98\xBD\xF6\xF5\x14a\x04\xDFW`\0\x80\xFD[\x80cT\xD1\xF1=\x14a\x044W\x80ccR!\x1E\x14a\x04<W\x80cp\xA0\x821\x14a\x04\\W\x80cqP\x18\xA6\x14a\x04|W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\x85W\x80cB\x84.\x0E\x11a\x01TW\x80cB\x84.\x0E\x14a\x03\xAAW\x80cB\x96lh\x14a\x03\xCAW\x80cJN\xE7\xB1\x14a\x03\xEAW\x80cQNb\xFC\x14a\x03\xFDW`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x03/W\x80c%i)b\x14a\x03OW\x80c-\xE9H\x07\x14a\x03WW\x80c@\xD0\x97\xC3\x14a\x03\x8AW`\0\x80\xFD[\x80c\x13\xA6a\xED\x11a\x01\xC1W\x80c\x13\xA6a\xED\x14a\x02\xA4W\x80c\x18:On\x14a\x02\xD2W\x80c\x1C\x10\x89?\x14a\x02\xE5W\x80c\x1C\xD6M\xF4\x14a\x02\xF8W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xF3W\x80c\x06\xFD\xDE\x03\x14a\x02(W\x80c\x08\x18\x12\xFC\x14a\x02JW\x80c\t^\xA7\xB3\x14a\x02\x82W[`\0\x80\xFD[4\x80\x15a\x01\xFFW`\0\x80\xFD[Pa\x02\x13a\x02\x0E6`\x04a\x16\0V[a\x06*V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x024W`\0\x80\xFD[Pa\x02=a\x06|V[`@Qa\x02\x1F\x91\x90a\x16mV[4\x80\x15a\x02VW`\0\x80\xFD[Pa\x02ja\x02e6`\x04a\x16\x80V[a\x07\x0EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x1FV[4\x80\x15a\x02\x8EW`\0\x80\xFD[Pa\x02\xA2a\x02\x9D6`\x04a\x16\xB5V[a\x075V[\0[4\x80\x15a\x02\xB0W`\0\x80\xFD[Pa\x02\xC4a\x02\xBF6`\x04a\x17&V[a\x08OV[`@Q\x90\x81R` \x01a\x02\x1FV[a\x02\xA2a\x02\xE06`\x04a\x16\x80V[a\x08xV[a\x02\xA2a\x02\xF36`\x04a\x16\xB5V[a\x08\x85V[4\x80\x15a\x03\x04W`\0\x80\xFD[Pa\x02\x13a\x03\x136`\x04a\x16\xB5V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x02\xA2a\x03J6`\x04a\x17\xDFV[a\x08\x9BV[a\x02\xA2a\x08\xCDV[4\x80\x15a\x03cW`\0\x80\xFD[Pa\x02\xC4a\x03r6`\x04a\x18\x1BV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x02\xC4a\x03\xA56`\x04a\x18\x1BV[a\t\x1DV[4\x80\x15a\x03\xB6W`\0\x80\xFD[Pa\x02\xA2a\x03\xC56`\x04a\x17\xDFV[a\tSV[4\x80\x15a\x03\xD6W`\0\x80\xFD[Pa\x02\xA2a\x03\xE56`\x04a\x16\x80V[a\tnV[a\x02\xA2a\x03\xF86`\x04a\x16\xB5V[a\t\x9CV[4\x80\x15a\x04\tW`\0\x80\xFD[Pa\x02\x13a\x04\x186`\x04a\x16\xB5V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02\xA2a\t\xAEV[4\x80\x15a\x04HW`\0\x80\xFD[Pa\x02ja\x04W6`\x04a\x16\x80V[a\t\xEAV[4\x80\x15a\x04hW`\0\x80\xFD[Pa\x02\xC4a\x04w6`\x04a\x18\x1BV[a\nJV[a\x02\xA2a\n\xD0V[4\x80\x15a\x04\x90W`\0\x80\xFD[Pa\x04\xA4a\x04\x9F6`\x04a\x16\x80V[a\n\xE4V[`@Qa\x02\x1F\x91\x90a\x186V[4\x80\x15a\x04\xBDW`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02jV[4\x80\x15a\x04\xD6W`\0\x80\xFD[Pa\x02=a\x0B\x1DV[4\x80\x15a\x04\xEBW`\0\x80\xFD[Pa\x02\xC4`\x06T\x81V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x02\xA2a\x05\x106`\x04a\x18}V[a\x0B,V[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\xA2a\x0506`\x04a\x18\xB9V[a\x0B7V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02=a\x05P6`\x04a\x16\x80V[a\x0BoV[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02\xC4`\x01\x81V[4\x80\x15a\x05vW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x02\x1FV[4\x80\x15a\x05\x94W`\0\x80\xFD[Pa\x02\x13a\x05\xA36`\x04a\x19yV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x02\xA2a\x05\xDF6`\x04a\x18\x1BV[a\x0B\xE3V[a\x02\xA2a\x05\xF26`\x04a\x18\x1BV[a\x0C V[4\x80\x15a\x06\x03W`\0\x80\xFD[Pa\x02\xC4a\x06\x126`\x04a\x18\x1BV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x06[WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x06vWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\0\x80Ta\x06\x8B\x90a\x19\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xB7\x90a\x19\xACV[\x80\x15a\x07\x04W\x80`\x1F\x10a\x06\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x07\x19\x82a\x0CGV[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x07@\x82a\t\xEAV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x07\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x07\xCEWPa\x07\xCE\x813a\x05\xA3V[a\x08@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\x07\xA9V[a\x08J\x83\x83a\x0C\xA6V[PPPV[`\0\x81Q`\x05\x1B[\x80\x15a\x08rW\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x08WV[P\x91\x90PV[a\x08\x823\x82a\r\x14V[PV[a\x08\x8Da\rcV[a\x08\x97\x82\x82a\r~V[PPV[a\x08\xA63[\x82a\r\xC9V[a\x08\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x08J\x83\x83\x83a\x0EHV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\0`\x01a\t*\x81a\x0F\xACV[`\x06T\x91Pa\t9\x83\x83a\x0F\xD2V[`\x06`\0\x81Ta\tH\x90a\x1A-V[\x90\x91UP\x90\x92\x91PPV[a\x08J\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x0B7V[a\tw3a\x08\xA0V[a\t\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x08\x82\x81a\x0F\xECV[a\t\xA4a\rcV[a\x08\x97\x82\x82a\r\x14V[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x06vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x07\xA9V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01Rh64\xB2\x107\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x07\xA9V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\n\xD8a\rcV[a\n\xE2`\0a\x10\x81V[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\n\xEFWPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x01\x80Ta\x06\x8B\x90a\x19\xACV[a\x08\x973\x83\x83a\x10\xBFV[a\x0BA3\x83a\r\xC9V[a\x0B]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x0Bi\x84\x84\x84\x84a\x11\x8DV[PPPPV[``a\x0Bz\x82a\x0CGV[`\0a\x0B\x91`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0B\xB1W`@Q\x80` \x01`@R\x80`\0\x81RPa\x0B\xDCV[\x80a\x0B\xBB\x84a\x11\xC0V[`@Q` \x01a\x0B\xCC\x92\x91\x90a\x1ATV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[a\x0B\xEBa\rcV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x0C\x13Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x08\x82\x81a\x10\x81V[a\x0C(a\rcV[\x80``\x1Ba\x0C>WctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x08\x82\x81a\x10\x81V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x08\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x0C\xDB\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\n\xE2Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0\x80a\r\xD5\x83a\t\xEAV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\x1CWP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x0E@WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x0E5\x84a\x07\x0EV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E[\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\x83V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\xA9V[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xF6\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\x83V[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4PPPV[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x08\x82Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[a\x08\x97\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x12SV[`\0a\x0F\xF7\x82a\t\xEAV[\x90Pa\x10\x02\x82a\t\xEAV[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R`\x03\x84R\x82\x85 \x80T`\0\x19\x01\x90U\x87\x85R`\x02\x90\x93R\x81\x84 \x80T\x90\x91\x16\x90UQ\x92\x93P\x84\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x83\x90\xA4PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x11 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x11\x98\x84\x84\x84a\x0EHV[a\x11\xA4\x84\x84\x84\x84a\x12\x86V[a\x0BiW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[```\0a\x11\xCD\x83a\x13\x87V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xEDWa\x11\xEDa\x16\xDFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\x17W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12!WP\x93\x92PPPV[a\x12]\x83\x83a\x14_V[a\x12j`\0\x84\x84\x84a\x12\x86V[a\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x13|W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x12\xCA\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a\x1B\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x13\x05WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x13\x02\x91\x81\x01\x90a\x1BWV[`\x01[a\x13bW=\x80\x80\x15a\x133W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x138V[``\x91P[P\x80Q`\0\x03a\x13ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x0E@V[P`\x01\x94\x93PPPPV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x13\xC6Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x13\xF2Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x14\x10Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x14(Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x14<Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x14NW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x06vW`\x01\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x15\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x15\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4PPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08\x82W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x16\x12W`\0\x80\xFD[\x815a\x0B\xDC\x81a\x15\xEAV[`\0[\x83\x81\x10\x15a\x168W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16 V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x16Y\x81` \x86\x01` \x86\x01a\x16\x1DV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0B\xDC` \x83\x01\x84a\x16AV[`\0` \x82\x84\x03\x12\x15a\x16\x92W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xB0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xC8W`\0\x80\xFD[a\x16\xD1\x83a\x16\x99V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\x1EWa\x17\x1Ea\x16\xDFV[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x179W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17QW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x17eW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x17wWa\x17wa\x16\xDFV[\x80`\x05\x1B\x91Pa\x17\x88\x84\x83\x01a\x16\xF5V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x17\xA2W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x17\xD3W\x845\x92P`\xFF\x83\x16\x83\x14a\x17\xC3W`\0\x80\x81\xFD[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x17\xA7V[\x98\x97PPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17\xF4W`\0\x80\xFD[a\x17\xFD\x84a\x16\x99V[\x92Pa\x18\x0B` \x85\x01a\x16\x99V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x18-W`\0\x80\xFD[a\x0B\xDC\x82a\x16\x99V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x18qW\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x18RV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x90W`\0\x80\xFD[a\x18\x99\x83a\x16\x99V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x18\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x18\xCFW`\0\x80\xFD[a\x18\xD8\x85a\x16\x99V[\x93P` a\x18\xE7\x81\x87\x01a\x16\x99V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\x0BW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x19\x1FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x191Wa\x191a\x16\xDFV[a\x19C`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x16\xF5V[\x91P\x80\x82R\x89\x84\x82\x85\x01\x01\x11\x15a\x19YW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\x8CW`\0\x80\xFD[a\x19\x95\x83a\x16\x99V[\x91Pa\x19\xA3` \x84\x01a\x16\x99V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19\xC0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08rWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`-\x90\x82\x01R\x7FERC721: caller is not token owne`@\x82\x01Rl\x1C\x88\x1B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\x9A\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01\x82\x01a\x1AMWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0\x83Qa\x1Af\x81\x84` \x88\x01a\x16\x1DV[\x83Q\x90\x83\x01\x90a\x1Az\x81\x83` \x88\x01a\x16\x1DV[\x01\x94\x93PPPPV[` \x80\x82R`%\x90\x82\x01R\x7FERC721: transfer from incorrect `@\x82\x01Rd7\xBB\xB72\xB9`\xD9\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`2\x90\x82\x01R\x7FERC721: transfer to non ERC721Re`@\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a\x1BM\x90\x83\x01\x84a\x16AV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1BiW`\0\x80\xFD[\x81Qa\x0B\xDC\x81a\x15\xEAV\xFE\xA2dipfsX\"\x12 \xD0\x85\x06\xC0\xD3.\xE8\x92^\x08\x0C\x05\xE60\xC6\x16\x9D\xF5u\xD7\")\xA3\xB07\x0B\x0F\xD5\x0B\xF0\x1D\x96dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static DAMNVALUABLENFT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xEEW`\x005`\xE0\x1C\x80cT\xD1\xF1=\x11a\x01\rW\x80c\xA2,\xB4e\x11a\0\xA0W\x80c\xD7S?\x02\x11a\0oW\x80c\xD7S?\x02\x14a\x05jW\x80c\xE9\x85\xE9\xC5\x14a\x05\x88W\x80c\xF0N(>\x14a\x05\xD1W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE4W\x80c\xFE\xE8\x1C\xF4\x14a\x05\xF7W`\0\x80\xFD[\x80c\xA2,\xB4e\x14a\x04\xF5W\x80c\xB8\x8DO\xDE\x14a\x05\x15W\x80c\xC8{V\xDD\x14a\x055W\x80c\xD59\x13\x93\x14a\x05UW`\0\x80\xFD[\x80csY\xE4\x1F\x11a\0\xDCW\x80csY\xE4\x1F\x14a\x04\x84W\x80c\x8D\xA5\xCB[\x14a\x04\xB1W\x80c\x95\xD8\x9BA\x14a\x04\xCAW\x80c\x98\xBD\xF6\xF5\x14a\x04\xDFW`\0\x80\xFD[\x80cT\xD1\xF1=\x14a\x044W\x80ccR!\x1E\x14a\x04<W\x80cp\xA0\x821\x14a\x04\\W\x80cqP\x18\xA6\x14a\x04|W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\x85W\x80cB\x84.\x0E\x11a\x01TW\x80cB\x84.\x0E\x14a\x03\xAAW\x80cB\x96lh\x14a\x03\xCAW\x80cJN\xE7\xB1\x14a\x03\xEAW\x80cQNb\xFC\x14a\x03\xFDW`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x03/W\x80c%i)b\x14a\x03OW\x80c-\xE9H\x07\x14a\x03WW\x80c@\xD0\x97\xC3\x14a\x03\x8AW`\0\x80\xFD[\x80c\x13\xA6a\xED\x11a\x01\xC1W\x80c\x13\xA6a\xED\x14a\x02\xA4W\x80c\x18:On\x14a\x02\xD2W\x80c\x1C\x10\x89?\x14a\x02\xE5W\x80c\x1C\xD6M\xF4\x14a\x02\xF8W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xF3W\x80c\x06\xFD\xDE\x03\x14a\x02(W\x80c\x08\x18\x12\xFC\x14a\x02JW\x80c\t^\xA7\xB3\x14a\x02\x82W[`\0\x80\xFD[4\x80\x15a\x01\xFFW`\0\x80\xFD[Pa\x02\x13a\x02\x0E6`\x04a\x16\0V[a\x06*V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x024W`\0\x80\xFD[Pa\x02=a\x06|V[`@Qa\x02\x1F\x91\x90a\x16mV[4\x80\x15a\x02VW`\0\x80\xFD[Pa\x02ja\x02e6`\x04a\x16\x80V[a\x07\x0EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x1FV[4\x80\x15a\x02\x8EW`\0\x80\xFD[Pa\x02\xA2a\x02\x9D6`\x04a\x16\xB5V[a\x075V[\0[4\x80\x15a\x02\xB0W`\0\x80\xFD[Pa\x02\xC4a\x02\xBF6`\x04a\x17&V[a\x08OV[`@Q\x90\x81R` \x01a\x02\x1FV[a\x02\xA2a\x02\xE06`\x04a\x16\x80V[a\x08xV[a\x02\xA2a\x02\xF36`\x04a\x16\xB5V[a\x08\x85V[4\x80\x15a\x03\x04W`\0\x80\xFD[Pa\x02\x13a\x03\x136`\x04a\x16\xB5V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x02\xA2a\x03J6`\x04a\x17\xDFV[a\x08\x9BV[a\x02\xA2a\x08\xCDV[4\x80\x15a\x03cW`\0\x80\xFD[Pa\x02\xC4a\x03r6`\x04a\x18\x1BV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x02\xC4a\x03\xA56`\x04a\x18\x1BV[a\t\x1DV[4\x80\x15a\x03\xB6W`\0\x80\xFD[Pa\x02\xA2a\x03\xC56`\x04a\x17\xDFV[a\tSV[4\x80\x15a\x03\xD6W`\0\x80\xFD[Pa\x02\xA2a\x03\xE56`\x04a\x16\x80V[a\tnV[a\x02\xA2a\x03\xF86`\x04a\x16\xB5V[a\t\x9CV[4\x80\x15a\x04\tW`\0\x80\xFD[Pa\x02\x13a\x04\x186`\x04a\x16\xB5V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02\xA2a\t\xAEV[4\x80\x15a\x04HW`\0\x80\xFD[Pa\x02ja\x04W6`\x04a\x16\x80V[a\t\xEAV[4\x80\x15a\x04hW`\0\x80\xFD[Pa\x02\xC4a\x04w6`\x04a\x18\x1BV[a\nJV[a\x02\xA2a\n\xD0V[4\x80\x15a\x04\x90W`\0\x80\xFD[Pa\x04\xA4a\x04\x9F6`\x04a\x16\x80V[a\n\xE4V[`@Qa\x02\x1F\x91\x90a\x186V[4\x80\x15a\x04\xBDW`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02jV[4\x80\x15a\x04\xD6W`\0\x80\xFD[Pa\x02=a\x0B\x1DV[4\x80\x15a\x04\xEBW`\0\x80\xFD[Pa\x02\xC4`\x06T\x81V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x02\xA2a\x05\x106`\x04a\x18}V[a\x0B,V[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\xA2a\x0506`\x04a\x18\xB9V[a\x0B7V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02=a\x05P6`\x04a\x16\x80V[a\x0BoV[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02\xC4`\x01\x81V[4\x80\x15a\x05vW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x02\x1FV[4\x80\x15a\x05\x94W`\0\x80\xFD[Pa\x02\x13a\x05\xA36`\x04a\x19yV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x02\xA2a\x05\xDF6`\x04a\x18\x1BV[a\x0B\xE3V[a\x02\xA2a\x05\xF26`\x04a\x18\x1BV[a\x0C V[4\x80\x15a\x06\x03W`\0\x80\xFD[Pa\x02\xC4a\x06\x126`\x04a\x18\x1BV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x06[WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x06vWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\0\x80Ta\x06\x8B\x90a\x19\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xB7\x90a\x19\xACV[\x80\x15a\x07\x04W\x80`\x1F\x10a\x06\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x07\x19\x82a\x0CGV[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x07@\x82a\t\xEAV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x07\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x07\xCEWPa\x07\xCE\x813a\x05\xA3V[a\x08@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\x07\xA9V[a\x08J\x83\x83a\x0C\xA6V[PPPV[`\0\x81Q`\x05\x1B[\x80\x15a\x08rW\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x08WV[P\x91\x90PV[a\x08\x823\x82a\r\x14V[PV[a\x08\x8Da\rcV[a\x08\x97\x82\x82a\r~V[PPV[a\x08\xA63[\x82a\r\xC9V[a\x08\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x08J\x83\x83\x83a\x0EHV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\0`\x01a\t*\x81a\x0F\xACV[`\x06T\x91Pa\t9\x83\x83a\x0F\xD2V[`\x06`\0\x81Ta\tH\x90a\x1A-V[\x90\x91UP\x90\x92\x91PPV[a\x08J\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x0B7V[a\tw3a\x08\xA0V[a\t\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x08\x82\x81a\x0F\xECV[a\t\xA4a\rcV[a\x08\x97\x82\x82a\r\x14V[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x06vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x07\xA9V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01Rh64\xB2\x107\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x07\xA9V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\n\xD8a\rcV[a\n\xE2`\0a\x10\x81V[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\n\xEFWPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x01\x80Ta\x06\x8B\x90a\x19\xACV[a\x08\x973\x83\x83a\x10\xBFV[a\x0BA3\x83a\r\xC9V[a\x0B]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x0Bi\x84\x84\x84\x84a\x11\x8DV[PPPPV[``a\x0Bz\x82a\x0CGV[`\0a\x0B\x91`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0B\xB1W`@Q\x80` \x01`@R\x80`\0\x81RPa\x0B\xDCV[\x80a\x0B\xBB\x84a\x11\xC0V[`@Q` \x01a\x0B\xCC\x92\x91\x90a\x1ATV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[a\x0B\xEBa\rcV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x0C\x13Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x08\x82\x81a\x10\x81V[a\x0C(a\rcV[\x80``\x1Ba\x0C>WctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x08\x82\x81a\x10\x81V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x08\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x0C\xDB\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\n\xE2Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0\x80a\r\xD5\x83a\t\xEAV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\x1CWP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x0E@WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x0E5\x84a\x07\x0EV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E[\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\x83V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\xA9V[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xF6\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\x83V[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4PPPV[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x08\x82Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[a\x08\x97\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x12SV[`\0a\x0F\xF7\x82a\t\xEAV[\x90Pa\x10\x02\x82a\t\xEAV[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R`\x03\x84R\x82\x85 \x80T`\0\x19\x01\x90U\x87\x85R`\x02\x90\x93R\x81\x84 \x80T\x90\x91\x16\x90UQ\x92\x93P\x84\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x83\x90\xA4PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x11 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x11\x98\x84\x84\x84a\x0EHV[a\x11\xA4\x84\x84\x84\x84a\x12\x86V[a\x0BiW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[```\0a\x11\xCD\x83a\x13\x87V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xEDWa\x11\xEDa\x16\xDFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\x17W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12!WP\x93\x92PPPV[a\x12]\x83\x83a\x14_V[a\x12j`\0\x84\x84\x84a\x12\x86V[a\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x13|W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x12\xCA\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a\x1B\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x13\x05WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x13\x02\x91\x81\x01\x90a\x1BWV[`\x01[a\x13bW=\x80\x80\x15a\x133W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x138V[``\x91P[P\x80Q`\0\x03a\x13ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x0E@V[P`\x01\x94\x93PPPPV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x13\xC6Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x13\xF2Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x14\x10Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x14(Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x14<Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x14NW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x06vW`\x01\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x15\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x15\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4PPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08\x82W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x16\x12W`\0\x80\xFD[\x815a\x0B\xDC\x81a\x15\xEAV[`\0[\x83\x81\x10\x15a\x168W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16 V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x16Y\x81` \x86\x01` \x86\x01a\x16\x1DV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0B\xDC` \x83\x01\x84a\x16AV[`\0` \x82\x84\x03\x12\x15a\x16\x92W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xB0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xC8W`\0\x80\xFD[a\x16\xD1\x83a\x16\x99V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\x1EWa\x17\x1Ea\x16\xDFV[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x179W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17QW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x17eW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x17wWa\x17wa\x16\xDFV[\x80`\x05\x1B\x91Pa\x17\x88\x84\x83\x01a\x16\xF5V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x17\xA2W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x17\xD3W\x845\x92P`\xFF\x83\x16\x83\x14a\x17\xC3W`\0\x80\x81\xFD[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x17\xA7V[\x98\x97PPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17\xF4W`\0\x80\xFD[a\x17\xFD\x84a\x16\x99V[\x92Pa\x18\x0B` \x85\x01a\x16\x99V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x18-W`\0\x80\xFD[a\x0B\xDC\x82a\x16\x99V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x18qW\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x18RV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x90W`\0\x80\xFD[a\x18\x99\x83a\x16\x99V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x18\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x18\xCFW`\0\x80\xFD[a\x18\xD8\x85a\x16\x99V[\x93P` a\x18\xE7\x81\x87\x01a\x16\x99V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\x0BW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x19\x1FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x191Wa\x191a\x16\xDFV[a\x19C`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x16\xF5V[\x91P\x80\x82R\x89\x84\x82\x85\x01\x01\x11\x15a\x19YW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\x8CW`\0\x80\xFD[a\x19\x95\x83a\x16\x99V[\x91Pa\x19\xA3` \x84\x01a\x16\x99V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19\xC0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08rWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`-\x90\x82\x01R\x7FERC721: caller is not token owne`@\x82\x01Rl\x1C\x88\x1B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\x9A\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01\x82\x01a\x1AMWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0\x83Qa\x1Af\x81\x84` \x88\x01a\x16\x1DV[\x83Q\x90\x83\x01\x90a\x1Az\x81\x83` \x88\x01a\x16\x1DV[\x01\x94\x93PPPPV[` \x80\x82R`%\x90\x82\x01R\x7FERC721: transfer from incorrect `@\x82\x01Rd7\xBB\xB72\xB9`\xD9\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`2\x90\x82\x01R\x7FERC721: transfer to non ERC721Re`@\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a\x1BM\x90\x83\x01\x84a\x16AV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1BiW`\0\x80\xFD[\x81Qa\x0B\xDC\x81a\x15\xEAV\xFE\xA2dipfsX\"\x12 \xD0\x85\x06\xC0\xD3.\xE8\x92^\x08\x0C\x05\xE60\xC6\x16\x9D\xF5u\xD7\")\xA3\xB07\x0B\x0F\xD5\x0B\xF0\x1D\x96dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static DAMNVALUABLENFT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DamnValuableNFT<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DamnValuableNFT<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for DamnValuableNFT<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for DamnValuableNFT<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for DamnValuableNFT<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DamnValuableNFT))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DamnValuableNFT<M> {
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
                DAMNVALUABLENFT_ABI.clone(),
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
                DAMNVALUABLENFT_ABI.clone(),
                DAMNVALUABLENFT_BYTECODE.clone().into(),
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
        ///Calls the contract's `approve` (0x095ea7b3)
        /// function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231)
        /// function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68)
        /// function
        pub fn burn(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], token_id)
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
        ///Calls the contract's `getApproved` (0x081812fc)
        /// function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
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
        ///Calls the contract's `isApprovedForAll`
        /// (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
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
        ///Calls the contract's `ownerOf` (0x6352211e)
        /// function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
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
        ///Calls the contract's `safeMint` (0x40d097c3)
        /// function
        pub fn safe_mint(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([64, 208, 151, 195], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom`
        /// (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom`
        /// (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll`
        /// (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface`
        /// (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        ///Calls the contract's `tokenIdCounter`
        /// (0x98bdf6f5) function
        pub fn token_id_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([152, 189, 246, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd)
        /// function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd)
        /// function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
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
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
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
            DamnValuableNFTEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for DamnValuableNFT<M>
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
    pub enum DamnValuableNFTErrors {
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        NoHandoverRequest(NoHandoverRequest),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DamnValuableNFTErrors {
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
                <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DamnValuableNFTErrors {
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
    impl ::ethers::contract::ContractRevert for DamnValuableNFTErrors {
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
    impl ::core::fmt::Display for DamnValuableNFTErrors {
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
    impl ::core::convert::From<::std::string::String> for DamnValuableNFTErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<NewOwnerIsZeroAddress> for DamnValuableNFTErrors {
        fn from(value: NewOwnerIsZeroAddress) -> Self {
            Self::NewOwnerIsZeroAddress(value)
        }
    }
    impl ::core::convert::From<NoHandoverRequest> for DamnValuableNFTErrors {
        fn from(value: NoHandoverRequest) -> Self {
            Self::NoHandoverRequest(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for DamnValuableNFTErrors {
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
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
        name = "ApprovalForAll",
        abi = "ApprovalForAll(address,address,bool)"
    )]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum DamnValuableNFTEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        OwnershipHandoverCanceledFilter(OwnershipHandoverCanceledFilter),
        OwnershipHandoverRequestedFilter(OwnershipHandoverRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RolesUpdatedFilter(RolesUpdatedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for DamnValuableNFTEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(DamnValuableNFTEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(DamnValuableNFTEvents::ApprovalForAllFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                OwnershipHandoverCanceledFilter::decode_log(log)
            {
                return Ok(
                    DamnValuableNFTEvents::OwnershipHandoverCanceledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                OwnershipHandoverRequestedFilter::decode_log(log)
            {
                return Ok(
                    DamnValuableNFTEvents::OwnershipHandoverRequestedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DamnValuableNFTEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RolesUpdatedFilter::decode_log(log) {
                return Ok(DamnValuableNFTEvents::RolesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(DamnValuableNFTEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DamnValuableNFTEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalForAllFilter(element) => {
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
    impl ::core::convert::From<ApprovalFilter> for DamnValuableNFTEvents {
        fn from(value: ApprovalFilter) -> Self { Self::ApprovalFilter(value) }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for DamnValuableNFTEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverCanceledFilter>
        for DamnValuableNFTEvents
    {
        fn from(value: OwnershipHandoverCanceledFilter) -> Self {
            Self::OwnershipHandoverCanceledFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverRequestedFilter>
        for DamnValuableNFTEvents
    {
        fn from(value: OwnershipHandoverRequestedFilter) -> Self {
            Self::OwnershipHandoverRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
        for DamnValuableNFTEvents
    {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RolesUpdatedFilter> for DamnValuableNFTEvents {
        fn from(value: RolesUpdatedFilter) -> Self {
            Self::RolesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for DamnValuableNFTEvents {
        fn from(value: TransferFilter) -> Self { Self::TransferFilter(value) }
    }
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
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
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
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `burn` function with signature `burn(uint256)` and
    /// selector `0x42966c68`
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
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub token_id: ::ethers::core::types::U256,
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
    /// `getApproved` function with signature
    /// `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
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
    /// `isApprovedForAll` function with signature
    /// `isApprovedForAll(address,address)` and selector
    /// `0xe985e9c5`
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
        name = "isApprovedForAll",
        abi = "isApprovedForAll(address,address)"
    )]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
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
    /// `ownerOf` function with signature `ownerOf(uint256)`
    /// and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
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
    /// `safeMint` function with signature
    /// `safeMint(address)` and selector `0x40d097c3`
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
    #[ethcall(name = "safeMint", abi = "safeMint(address)")]
    pub struct SafeMintCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `safeTransferFrom` function with signature
    /// `safeTransferFrom(address,address,uint256)` and
    /// selector `0x42842e0e`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `safeTransferFrom` function with signature
    /// `safeTransferFrom(address,address,uint256,bytes)`
    /// and selector `0xb88d4fde`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `setApprovalForAll` function with signature
    /// `setApprovalForAll(address,bool)` and selector
    /// `0xa22cb465`
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
        name = "setApprovalForAll",
        abi = "setApprovalForAll(address,bool)"
    )]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the
    /// `supportsInterface` function with signature
    /// `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
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
    /// `tokenIdCounter` function with signature
    /// `tokenIdCounter()` and selector `0x98bdf6f5`
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
    #[ethcall(name = "tokenIdCounter", abi = "tokenIdCounter()")]
    pub struct TokenIdCounterCall;
    ///Container type for all input parameters for the
    /// `tokenURI` function with signature
    /// `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
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
        pub token_id: ::ethers::core::types::U256,
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
    pub enum DamnValuableNFTCalls {
        MinterRole(MinterRoleCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        CancelOwnershipHandover(CancelOwnershipHandoverCall),
        CompleteOwnershipHandover(CompleteOwnershipHandoverCall),
        GetApproved(GetApprovedCall),
        GrantRoles(GrantRolesCall),
        HasAllRoles(HasAllRolesCall),
        HasAnyRole(HasAnyRoleCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OrdinalsFromRoles(OrdinalsFromRolesCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        OwnershipHandoverExpiresAt(OwnershipHandoverExpiresAtCall),
        OwnershipHandoverValidFor(OwnershipHandoverValidForCall),
        RenounceOwnership(RenounceOwnershipCall),
        RenounceRoles(RenounceRolesCall),
        RequestOwnershipHandover(RequestOwnershipHandoverCall),
        RevokeRoles(RevokeRolesCall),
        RolesFromOrdinals(RolesFromOrdinalsCall),
        RolesOf(RolesOfCall),
        SafeMint(SafeMintCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(
            SafeTransferFromWithFromAndToAndDataCall,
        ),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenIdCounter(TokenIdCounterCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for DamnValuableNFTCalls {
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
                <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetApproved(decoded));
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
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsApprovedForAll(decoded));
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
            if let Ok(decoded) =
                <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OwnerOf(decoded));
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
                <SafeMintCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeMint(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded)
                = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenIdCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TokenIdCounter(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenURI(decoded));
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
    impl ::ethers::core::abi::AbiEncode for DamnValuableNFTCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MinterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
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
                Self::GetApproved(element) => {
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
                Self::IsApprovedForAll(element) => {
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
                Self::OwnerOf(element) => {
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
                Self::SafeMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenIdCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
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
    impl ::core::fmt::Display for DamnValuableNFTCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::MinterRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetApproved(element) => {
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
                Self::IsApprovedForAll(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrdinalsFromRoles(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SafeMint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeTransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenIdCounter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenURI(element) => {
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
    impl ::core::convert::From<MinterRoleCall> for DamnValuableNFTCalls {
        fn from(value: MinterRoleCall) -> Self { Self::MinterRole(value) }
    }
    impl ::core::convert::From<ApproveCall> for DamnValuableNFTCalls {
        fn from(value: ApproveCall) -> Self { Self::Approve(value) }
    }
    impl ::core::convert::From<BalanceOfCall> for DamnValuableNFTCalls {
        fn from(value: BalanceOfCall) -> Self { Self::BalanceOf(value) }
    }
    impl ::core::convert::From<BurnCall> for DamnValuableNFTCalls {
        fn from(value: BurnCall) -> Self { Self::Burn(value) }
    }
    impl ::core::convert::From<CancelOwnershipHandoverCall>
        for DamnValuableNFTCalls
    {
        fn from(value: CancelOwnershipHandoverCall) -> Self {
            Self::CancelOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<CompleteOwnershipHandoverCall>
        for DamnValuableNFTCalls
    {
        fn from(value: CompleteOwnershipHandoverCall) -> Self {
            Self::CompleteOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for DamnValuableNFTCalls {
        fn from(value: GetApprovedCall) -> Self { Self::GetApproved(value) }
    }
    impl ::core::convert::From<GrantRolesCall> for DamnValuableNFTCalls {
        fn from(value: GrantRolesCall) -> Self { Self::GrantRoles(value) }
    }
    impl ::core::convert::From<HasAllRolesCall> for DamnValuableNFTCalls {
        fn from(value: HasAllRolesCall) -> Self { Self::HasAllRoles(value) }
    }
    impl ::core::convert::From<HasAnyRoleCall> for DamnValuableNFTCalls {
        fn from(value: HasAnyRoleCall) -> Self { Self::HasAnyRole(value) }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for DamnValuableNFTCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<NameCall> for DamnValuableNFTCalls {
        fn from(value: NameCall) -> Self { Self::Name(value) }
    }
    impl ::core::convert::From<OrdinalsFromRolesCall> for DamnValuableNFTCalls {
        fn from(value: OrdinalsFromRolesCall) -> Self {
            Self::OrdinalsFromRoles(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DamnValuableNFTCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<OwnerOfCall> for DamnValuableNFTCalls {
        fn from(value: OwnerOfCall) -> Self { Self::OwnerOf(value) }
    }
    impl ::core::convert::From<OwnershipHandoverExpiresAtCall>
        for DamnValuableNFTCalls
    {
        fn from(value: OwnershipHandoverExpiresAtCall) -> Self {
            Self::OwnershipHandoverExpiresAt(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverValidForCall>
        for DamnValuableNFTCalls
    {
        fn from(value: OwnershipHandoverValidForCall) -> Self {
            Self::OwnershipHandoverValidFor(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DamnValuableNFTCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RenounceRolesCall> for DamnValuableNFTCalls {
        fn from(value: RenounceRolesCall) -> Self { Self::RenounceRoles(value) }
    }
    impl ::core::convert::From<RequestOwnershipHandoverCall>
        for DamnValuableNFTCalls
    {
        fn from(value: RequestOwnershipHandoverCall) -> Self {
            Self::RequestOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<RevokeRolesCall> for DamnValuableNFTCalls {
        fn from(value: RevokeRolesCall) -> Self { Self::RevokeRoles(value) }
    }
    impl ::core::convert::From<RolesFromOrdinalsCall> for DamnValuableNFTCalls {
        fn from(value: RolesFromOrdinalsCall) -> Self {
            Self::RolesFromOrdinals(value)
        }
    }
    impl ::core::convert::From<RolesOfCall> for DamnValuableNFTCalls {
        fn from(value: RolesOfCall) -> Self { Self::RolesOf(value) }
    }
    impl ::core::convert::From<SafeMintCall> for DamnValuableNFTCalls {
        fn from(value: SafeMintCall) -> Self { Self::SafeMint(value) }
    }
    impl ::core::convert::From<SafeTransferFromCall> for DamnValuableNFTCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
        for DamnValuableNFTCalls
    {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for DamnValuableNFTCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for DamnValuableNFTCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for DamnValuableNFTCalls {
        fn from(value: SymbolCall) -> Self { Self::Symbol(value) }
    }
    impl ::core::convert::From<TokenIdCounterCall> for DamnValuableNFTCalls {
        fn from(value: TokenIdCounterCall) -> Self {
            Self::TokenIdCounter(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for DamnValuableNFTCalls {
        fn from(value: TokenURICall) -> Self { Self::TokenURI(value) }
    }
    impl ::core::convert::From<TransferFromCall> for DamnValuableNFTCalls {
        fn from(value: TransferFromCall) -> Self { Self::TransferFrom(value) }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DamnValuableNFTCalls {
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinterRoleReturn(pub ::ethers::core::types::U256);
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
    /// `getApproved` function with signature
    /// `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
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
    /// `isApprovedForAll` function with signature
    /// `isApprovedForAll(address,address)` and selector
    /// `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
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
    /// `ownerOf` function with signature `ownerOf(uint256)`
    /// and selector `0x6352211e`
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
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
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
    /// `safeMint` function with signature
    /// `safeMint(address)` and selector `0x40d097c3`
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
    pub struct SafeMintReturn {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the
    /// `supportsInterface` function with signature
    /// `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
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
    /// `tokenIdCounter` function with signature
    /// `tokenIdCounter()` and selector `0x98bdf6f5`
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
    pub struct TokenIdCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `tokenURI` function with signature
    /// `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}
