pub use climber_timelock::*;
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
pub mod climber_timelock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("proposer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("delay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delay"),
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
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
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
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataElements"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperationId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperationId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
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
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataElements"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperationState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperationState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ClimberTimelockBase.OperationState",
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("operations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operations"),
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
                                    name: ::std::borrow::ToOwned::to_owned("readyAtTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("known"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("executed"),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("schedule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("schedule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
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
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataElements"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
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
                    ::std::borrow::ToOwned::to_owned("updateDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateDelay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
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
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("CallerNotTimelock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallerNotTimelock"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDataElementsCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidDataElementsCount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTargetsCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidTargetsCount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidValuesCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidValuesCount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewDelayAboveMax"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NewDelayAboveMax"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotReadyForExecution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotReadyForExecution",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operationId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperationAlreadyKnown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperationAlreadyKnown",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operationId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CLIMBERTIMELOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x15_8\x03\x80b\0\x15_\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xFEV[b\0\0O`\0\x80Q` b\0\x15?\x839\x81Q\x91R\x80b\0\0\xE6V[b\0\0y`\0\x80Q` b\0\x15\x1F\x839\x81Q\x91R`\0\x80Q` b\0\x15?\x839\x81Q\x91Rb\0\0\xE6V[b\0\0\x94`\0\x80Q` b\0\x15?\x839\x81Q\x91R\x83b\0\x011V[b\0\0\xAF`\0\x80Q` b\0\x15?\x839\x81Q\x91R0b\0\x011V[b\0\0\xCA`\0\x80Q` b\0\x15\x1F\x839\x81Q\x91R\x82b\0\x011V[PP`\x02\x80T`\x01`\x01`@\x1B\x03\x19\x16a\x0E\x10\x17\x90Ub\0\x026V[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[b\0\x01=\x82\x82b\0\x01AV[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x01=W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01\x9D3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x02\x12W`\0\x80\xFD[b\0\x02\x1D\x83b\0\x01\xE1V[\x91Pb\0\x02-` \x84\x01b\0\x01\xE1V[\x90P\x92P\x92\x90PV[a\x12\xD9\x80b\0\x02F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xE1W`\x005`\xE0\x1C\x80cjB\xB8\xF8\x11a\0\x7FW\x80c\x91\xD1HT\x11a\0YW\x80c\x91\xD1HT\x14a\x02zW\x80c\xA2\x17\xFD\xDF\x14a\x02\x9AW\x80c\xC7O4\x9B\x14a\x02\xAFW\x80c\xD5Gt\x1F\x14a\x03\"W`\0\x80\xFD[\x80cjB\xB8\xF8\x14a\x01\xF5W\x80cyX\0L\x14a\x02-W\x80c\x90\xBD\x1Em\x14a\x02ZW`\0\x80\xFD[\x80c&V\"}\x11a\0\xBBW\x80c&V\"}\x14a\x01\x82W\x80c//\xF1]\x14a\x01\x95W\x80c6V\x8A\xBE\x14a\x01\xB5W\x80cW\xF5%\xED\x14a\x01\xD5W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xEDW\x80c$\x8A\x9C\xA3\x14a\x01\"W\x80c$\xAD\xBC[\x14a\x01`W`\0\x80\xFD[6a\0\xE8W\0[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[Pa\x01\ra\x01\x086`\x04a\rPV[a\x03BV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01.W`\0\x80\xFD[Pa\x01Ra\x01=6`\x04a\rzV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x19V[4\x80\x15a\x01lW`\0\x80\xFD[Pa\x01\x80a\x01{6`\x04a\r\x93V[a\x03yV[\0[a\x01\x80a\x01\x906`\x04a\x0E\x07V[a\x03\xE9V[4\x80\x15a\x01\xA1W`\0\x80\xFD[Pa\x01\x80a\x01\xB06`\x04a\x0E\xC5V[a\x05\x98V[4\x80\x15a\x01\xC1W`\0\x80\xFD[Pa\x01\x80a\x01\xD06`\x04a\x0E\xC5V[a\x05\xC2V[4\x80\x15a\x01\xE1W`\0\x80\xFD[Pa\x01Ra\x01\xF06`\x04a\x0E\x07V[a\x06@V[4\x80\x15a\x02\x01W`\0\x80\xFD[P`\x02Ta\x02\x15\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[4\x80\x15a\x029W`\0\x80\xFD[Pa\x02Ma\x02H6`\x04a\rzV[a\x06\x82V[`@Qa\x01\x19\x91\x90a\x0F\x07V[4\x80\x15a\x02fW`\0\x80\xFD[Pa\x01\x80a\x02u6`\x04a\x0E\x07V[a\x07\x17V[4\x80\x15a\x02\x86W`\0\x80\xFD[Pa\x01\ra\x02\x956`\x04a\x0E\xC5V[a\x08SV[4\x80\x15a\x02\xA6W`\0\x80\xFD[Pa\x01R`\0\x81V[4\x80\x15a\x02\xBBW`\0\x80\xFD[Pa\x02\xFBa\x02\xCA6`\x04a\rzV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x81\x16\x90`\xFF`\x01`@\x1B\x82\x04\x81\x16\x91`\x01`H\x1B\x90\x04\x16\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R\x91\x15\x15` \x84\x01R\x15\x15\x90\x82\x01R``\x01a\x01\x19V[4\x80\x15a\x03.W`\0\x80\xFD[Pa\x01\x80a\x03=6`\x04a\x0E\xC5V[a\x08|V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03sWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[30\x14a\x03\x99W`@Qc\xDF\xB4\x9E1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x12u\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x03\xC6W`@Qc\x1E=\t1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x85a\x04\x07W`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14a\x04'W`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x82\x14a\x04GW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x04X\x88\x88\x88\x88\x88\x88\x88a\x06@V[\x90P`\0[`\xFF\x81\x16\x88\x11\x15a\x05'Wa\x05\x1E\x85\x85\x83`\xFF\x16\x81\x81\x10a\x04\x80Wa\x04\x80a\x0F/V[\x90P` \x02\x81\x01\x90a\x04\x92\x91\x90a\x0FEV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92P\x8A\x91PP`\xFF\x85\x16\x81\x81\x10a\x04\xDDWa\x04\xDDa\x0F/V[\x90P` \x02\x015\x8B\x8B\x85`\xFF\x16\x81\x81\x10a\x04\xF9Wa\x04\xF9a\x0F/V[\x90P` \x02\x01` \x81\x01\x90a\x05\x0E\x91\x90a\x0F\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x08\xA1V[P`\x01\x01a\x04]V[P`\x02a\x053\x82a\x06\x82V[`\x03\x81\x11\x15a\x05DWa\x05Da\x0E\xF1V[\x14a\x05jW`@Qc\x08)_\xC9`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` R`@\x90 \x80Ti\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1B\x17\x90UPPPPPPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\xB3\x81a\x08\xCFV[a\x05\xBD\x83\x83a\x08\xDCV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x062W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05aV[a\x06<\x82\x82a\t`V[PPV[`\0\x87\x87\x87\x87\x87\x87\x87`@Q` \x01a\x06_\x97\x96\x95\x94\x93\x92\x91\x90a\x10`V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\xFF`\x01`@\x1B\x82\x04\x81\x16\x15\x80\x15\x95\x84\x01\x95\x90\x95R`\x01`H\x1B\x90\x91\x04\x16\x15\x15\x91\x81\x01\x91\x90\x91R\x90a\x07\x0CW\x80`@\x01Q\x15a\x06\xE8W`\x03\x91Pa\x07\x11V[\x80Q`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x07\x03W`\x01\x91Pa\x07\x11V[`\x02\x91Pa\x07\x11V[`\0\x91P[P\x91\x90PV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\x07A\x81a\x08\xCFV[\x86\x15\x80a\x07PWPa\x01\0\x87\x10\x15[\x15a\x07nW`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x85\x14a\x07\x8EW`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x83\x14a\x07\xAEW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xBF\x89\x89\x89\x89\x89\x89\x89a\x06@V[\x90P`\0a\x07\xCC\x82a\x06\x82V[`\x03\x81\x11\x15a\x07\xDDWa\x07\xDDa\x0E\xF1V[\x14a\x07\xFEW`@Qc \xB1\x99\xD1`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05aV[`\x02Ta\x08\x14\x90`\x01`\x01`@\x1B\x03\x16Ba\x11\x14V[`\0\x91\x82R`\x01` R`@\x90\x91 \x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17`\x01`@\x1B\x17\x90UPPPPPPPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x97\x81a\x08\xCFV[a\x05\xBD\x83\x83a\t`V[``a\x08\xC7\x84\x84\x84`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x12{`)\x919a\t\xC5V[\x94\x93PPPPV[a\x08\xD9\x813a\n\xA0V[PV[a\x08\xE6\x82\x82a\x08SV[a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\x1C3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\tj\x82\x82a\x08SV[\x15a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``\x82G\x10\x15a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05aV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\nB\x91\x90a\x11_V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\x7FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x84V[``\x91P[P\x91P\x91Pa\n\x95\x87\x83\x83\x87a\n\xF9V[\x97\x96PPPPPPPV[a\n\xAA\x82\x82a\x08SV[a\x06<Wa\n\xB7\x81a\x0BrV[a\n\xC2\x83` a\x0B\x84V[`@Q` \x01a\n\xD3\x92\x91\x90a\x11{V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05a\x91`\x04\x01a\x11\xF0V[``\x83\x15a\x0BhW\x82Q`\0\x03a\x0BaW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x0BaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05aV[P\x81a\x08\xC7V[a\x08\xC7\x83\x83a\r&V[``a\x03s`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B\x93\x83`\x02a\x12#V[a\x0B\x9E\x90`\x02a\x12:V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xB5Wa\x0B\xB5a\x12MV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xDFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\xFAWa\x0B\xFAa\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0C)Wa\x0C)a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0CM\x84`\x02a\x12#V[a\x0CX\x90`\x01a\x12:V[\x90P[`\x01\x81\x11\x15a\x0C\xD0Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0C\x8CWa\x0C\x8Ca\x0F/V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0C\xA2Wa\x0C\xA2a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0C\xC9\x81a\x12cV[\x90Pa\x0C[V[P\x83\x15a\r\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05aV[\x93\x92PPPV[\x81Q\x15a\r6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05a\x91\x90a\x11\xF0V[`\0` \x82\x84\x03\x12\x15a\rbW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\r\xCEW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xE5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0E\0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x0E\"W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E9W`\0\x80\xFD[a\x0EE\x8B\x83\x8C\x01a\r\xBCV[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a\x0E^W`\0\x80\xFD[a\x0Ej\x8B\x83\x8C\x01a\r\xBCV[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a\x0E\x83W`\0\x80\xFD[Pa\x0E\x90\x8A\x82\x8B\x01a\r\xBCV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xC0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xD8W`\0\x80\xFD[\x825\x91Pa\x0E\xE8` \x84\x01a\x0E\xA9V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10a\x0F)WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0F\\W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0FvW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0E\0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\x9DW`\0\x80\xFD[a\r\x1F\x82a\x0E\xA9V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0[\x87\x81\x10\x15a\x10SW\x82\x84\x03\x89R\x815`\x1E\x19\x886\x03\x01\x81\x12a\x10\nW`\0\x80\xFD[\x87\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10%W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x104W`\0\x80\xFD[a\x10?\x86\x82\x84a\x0F\xA6V[\x9A\x87\x01\x9A\x95PPP\x90\x84\x01\x90`\x01\x01a\x0F\xE9V[P\x91\x97\x96PPPPPPPV[`\x80\x80\x82R\x81\x01\x87\x90R`\0\x88`\xA0\x83\x01\x82[\x8A\x81\x10\x15a\x10\xA1W`\x01`\x01`\xA0\x1B\x03a\x10\x8C\x84a\x0E\xA9V[\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x10sV[P\x83\x81\x03` \x85\x01R\x87\x81R`\x01`\x01`\xFB\x1B\x03\x88\x11\x15a\x10\xC1W`\0\x80\xFD[\x87`\x05\x1B\x91P\x81\x89` \x83\x017\x01\x82\x81\x03` \x90\x81\x01`@\x85\x01Ra\x10\xE9\x90\x82\x01\x86\x88a\x0F\xCFV[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x114Wa\x114a\x10\xFEV[P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11VW\x81\x81\x01Q\x83\x82\x01R` \x01a\x11>V[PP`\0\x91\x01RV[`\0\x82Qa\x11q\x81\x84` \x87\x01a\x11;V[\x91\x90\x91\x01\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x11\xB3\x81`\x17\x85\x01` \x88\x01a\x11;V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x11\xE4\x81`(\x84\x01` \x88\x01a\x11;V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x12\x0F\x81`@\x85\x01` \x87\x01a\x11;V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03sWa\x03sa\x10\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x03sWa\x03sa\x10\xFEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x81a\x12rWa\x12ra\x10\xFEV[P`\0\x19\x01\x90V\xFEAddress: low-level call with value failed\xA2dipfsX\"\x12 \x03D\xD1F>\xAD j\xCC\xC6\xD3\x16K\xC1\x7F\xB7\x86\x03\xF1\xF5g\xE1m=\x0F\xAFV\x06\xA5\xB0T\xE6dsolcC\0\x08\x15\x003\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u";
    /// The bytecode of the contract.
    pub static CLIMBERTIMELOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xE1W`\x005`\xE0\x1C\x80cjB\xB8\xF8\x11a\0\x7FW\x80c\x91\xD1HT\x11a\0YW\x80c\x91\xD1HT\x14a\x02zW\x80c\xA2\x17\xFD\xDF\x14a\x02\x9AW\x80c\xC7O4\x9B\x14a\x02\xAFW\x80c\xD5Gt\x1F\x14a\x03\"W`\0\x80\xFD[\x80cjB\xB8\xF8\x14a\x01\xF5W\x80cyX\0L\x14a\x02-W\x80c\x90\xBD\x1Em\x14a\x02ZW`\0\x80\xFD[\x80c&V\"}\x11a\0\xBBW\x80c&V\"}\x14a\x01\x82W\x80c//\xF1]\x14a\x01\x95W\x80c6V\x8A\xBE\x14a\x01\xB5W\x80cW\xF5%\xED\x14a\x01\xD5W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xEDW\x80c$\x8A\x9C\xA3\x14a\x01\"W\x80c$\xAD\xBC[\x14a\x01`W`\0\x80\xFD[6a\0\xE8W\0[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[Pa\x01\ra\x01\x086`\x04a\rPV[a\x03BV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01.W`\0\x80\xFD[Pa\x01Ra\x01=6`\x04a\rzV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x19V[4\x80\x15a\x01lW`\0\x80\xFD[Pa\x01\x80a\x01{6`\x04a\r\x93V[a\x03yV[\0[a\x01\x80a\x01\x906`\x04a\x0E\x07V[a\x03\xE9V[4\x80\x15a\x01\xA1W`\0\x80\xFD[Pa\x01\x80a\x01\xB06`\x04a\x0E\xC5V[a\x05\x98V[4\x80\x15a\x01\xC1W`\0\x80\xFD[Pa\x01\x80a\x01\xD06`\x04a\x0E\xC5V[a\x05\xC2V[4\x80\x15a\x01\xE1W`\0\x80\xFD[Pa\x01Ra\x01\xF06`\x04a\x0E\x07V[a\x06@V[4\x80\x15a\x02\x01W`\0\x80\xFD[P`\x02Ta\x02\x15\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[4\x80\x15a\x029W`\0\x80\xFD[Pa\x02Ma\x02H6`\x04a\rzV[a\x06\x82V[`@Qa\x01\x19\x91\x90a\x0F\x07V[4\x80\x15a\x02fW`\0\x80\xFD[Pa\x01\x80a\x02u6`\x04a\x0E\x07V[a\x07\x17V[4\x80\x15a\x02\x86W`\0\x80\xFD[Pa\x01\ra\x02\x956`\x04a\x0E\xC5V[a\x08SV[4\x80\x15a\x02\xA6W`\0\x80\xFD[Pa\x01R`\0\x81V[4\x80\x15a\x02\xBBW`\0\x80\xFD[Pa\x02\xFBa\x02\xCA6`\x04a\rzV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x81\x16\x90`\xFF`\x01`@\x1B\x82\x04\x81\x16\x91`\x01`H\x1B\x90\x04\x16\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R\x91\x15\x15` \x84\x01R\x15\x15\x90\x82\x01R``\x01a\x01\x19V[4\x80\x15a\x03.W`\0\x80\xFD[Pa\x01\x80a\x03=6`\x04a\x0E\xC5V[a\x08|V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03sWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[30\x14a\x03\x99W`@Qc\xDF\xB4\x9E1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x12u\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x03\xC6W`@Qc\x1E=\t1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x85a\x04\x07W`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14a\x04'W`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x82\x14a\x04GW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x04X\x88\x88\x88\x88\x88\x88\x88a\x06@V[\x90P`\0[`\xFF\x81\x16\x88\x11\x15a\x05'Wa\x05\x1E\x85\x85\x83`\xFF\x16\x81\x81\x10a\x04\x80Wa\x04\x80a\x0F/V[\x90P` \x02\x81\x01\x90a\x04\x92\x91\x90a\x0FEV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92P\x8A\x91PP`\xFF\x85\x16\x81\x81\x10a\x04\xDDWa\x04\xDDa\x0F/V[\x90P` \x02\x015\x8B\x8B\x85`\xFF\x16\x81\x81\x10a\x04\xF9Wa\x04\xF9a\x0F/V[\x90P` \x02\x01` \x81\x01\x90a\x05\x0E\x91\x90a\x0F\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x08\xA1V[P`\x01\x01a\x04]V[P`\x02a\x053\x82a\x06\x82V[`\x03\x81\x11\x15a\x05DWa\x05Da\x0E\xF1V[\x14a\x05jW`@Qc\x08)_\xC9`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` R`@\x90 \x80Ti\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1B\x17\x90UPPPPPPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\xB3\x81a\x08\xCFV[a\x05\xBD\x83\x83a\x08\xDCV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x062W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05aV[a\x06<\x82\x82a\t`V[PPV[`\0\x87\x87\x87\x87\x87\x87\x87`@Q` \x01a\x06_\x97\x96\x95\x94\x93\x92\x91\x90a\x10`V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\xFF`\x01`@\x1B\x82\x04\x81\x16\x15\x80\x15\x95\x84\x01\x95\x90\x95R`\x01`H\x1B\x90\x91\x04\x16\x15\x15\x91\x81\x01\x91\x90\x91R\x90a\x07\x0CW\x80`@\x01Q\x15a\x06\xE8W`\x03\x91Pa\x07\x11V[\x80Q`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x07\x03W`\x01\x91Pa\x07\x11V[`\x02\x91Pa\x07\x11V[`\0\x91P[P\x91\x90PV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\x07A\x81a\x08\xCFV[\x86\x15\x80a\x07PWPa\x01\0\x87\x10\x15[\x15a\x07nW`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x85\x14a\x07\x8EW`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x83\x14a\x07\xAEW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xBF\x89\x89\x89\x89\x89\x89\x89a\x06@V[\x90P`\0a\x07\xCC\x82a\x06\x82V[`\x03\x81\x11\x15a\x07\xDDWa\x07\xDDa\x0E\xF1V[\x14a\x07\xFEW`@Qc \xB1\x99\xD1`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05aV[`\x02Ta\x08\x14\x90`\x01`\x01`@\x1B\x03\x16Ba\x11\x14V[`\0\x91\x82R`\x01` R`@\x90\x91 \x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17`\x01`@\x1B\x17\x90UPPPPPPPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x97\x81a\x08\xCFV[a\x05\xBD\x83\x83a\t`V[``a\x08\xC7\x84\x84\x84`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x12{`)\x919a\t\xC5V[\x94\x93PPPPV[a\x08\xD9\x813a\n\xA0V[PV[a\x08\xE6\x82\x82a\x08SV[a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\x1C3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\tj\x82\x82a\x08SV[\x15a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``\x82G\x10\x15a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05aV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\nB\x91\x90a\x11_V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\x7FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x84V[``\x91P[P\x91P\x91Pa\n\x95\x87\x83\x83\x87a\n\xF9V[\x97\x96PPPPPPPV[a\n\xAA\x82\x82a\x08SV[a\x06<Wa\n\xB7\x81a\x0BrV[a\n\xC2\x83` a\x0B\x84V[`@Q` \x01a\n\xD3\x92\x91\x90a\x11{V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05a\x91`\x04\x01a\x11\xF0V[``\x83\x15a\x0BhW\x82Q`\0\x03a\x0BaW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x0BaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05aV[P\x81a\x08\xC7V[a\x08\xC7\x83\x83a\r&V[``a\x03s`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B\x93\x83`\x02a\x12#V[a\x0B\x9E\x90`\x02a\x12:V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xB5Wa\x0B\xB5a\x12MV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xDFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\xFAWa\x0B\xFAa\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0C)Wa\x0C)a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0CM\x84`\x02a\x12#V[a\x0CX\x90`\x01a\x12:V[\x90P[`\x01\x81\x11\x15a\x0C\xD0Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0C\x8CWa\x0C\x8Ca\x0F/V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0C\xA2Wa\x0C\xA2a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0C\xC9\x81a\x12cV[\x90Pa\x0C[V[P\x83\x15a\r\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05aV[\x93\x92PPPV[\x81Q\x15a\r6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05a\x91\x90a\x11\xF0V[`\0` \x82\x84\x03\x12\x15a\rbW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\r\xCEW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xE5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0E\0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x0E\"W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E9W`\0\x80\xFD[a\x0EE\x8B\x83\x8C\x01a\r\xBCV[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a\x0E^W`\0\x80\xFD[a\x0Ej\x8B\x83\x8C\x01a\r\xBCV[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a\x0E\x83W`\0\x80\xFD[Pa\x0E\x90\x8A\x82\x8B\x01a\r\xBCV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xC0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xD8W`\0\x80\xFD[\x825\x91Pa\x0E\xE8` \x84\x01a\x0E\xA9V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10a\x0F)WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0F\\W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0FvW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0E\0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\x9DW`\0\x80\xFD[a\r\x1F\x82a\x0E\xA9V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0[\x87\x81\x10\x15a\x10SW\x82\x84\x03\x89R\x815`\x1E\x19\x886\x03\x01\x81\x12a\x10\nW`\0\x80\xFD[\x87\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10%W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x104W`\0\x80\xFD[a\x10?\x86\x82\x84a\x0F\xA6V[\x9A\x87\x01\x9A\x95PPP\x90\x84\x01\x90`\x01\x01a\x0F\xE9V[P\x91\x97\x96PPPPPPPV[`\x80\x80\x82R\x81\x01\x87\x90R`\0\x88`\xA0\x83\x01\x82[\x8A\x81\x10\x15a\x10\xA1W`\x01`\x01`\xA0\x1B\x03a\x10\x8C\x84a\x0E\xA9V[\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x10sV[P\x83\x81\x03` \x85\x01R\x87\x81R`\x01`\x01`\xFB\x1B\x03\x88\x11\x15a\x10\xC1W`\0\x80\xFD[\x87`\x05\x1B\x91P\x81\x89` \x83\x017\x01\x82\x81\x03` \x90\x81\x01`@\x85\x01Ra\x10\xE9\x90\x82\x01\x86\x88a\x0F\xCFV[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x114Wa\x114a\x10\xFEV[P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11VW\x81\x81\x01Q\x83\x82\x01R` \x01a\x11>V[PP`\0\x91\x01RV[`\0\x82Qa\x11q\x81\x84` \x87\x01a\x11;V[\x91\x90\x91\x01\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x11\xB3\x81`\x17\x85\x01` \x88\x01a\x11;V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x11\xE4\x81`(\x84\x01` \x88\x01a\x11;V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x12\x0F\x81`@\x85\x01` \x87\x01a\x11;V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03sWa\x03sa\x10\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x03sWa\x03sa\x10\xFEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x81a\x12rWa\x12ra\x10\xFEV[P`\0\x19\x01\x90V\xFEAddress: low-level call with value failed\xA2dipfsX\"\x12 \x03D\xD1F>\xAD j\xCC\xC6\xD3\x16K\xC1\x7F\xB7\x86\x03\xF1\xF5g\xE1m=\x0F\xAFV\x06\xA5\xB0T\xE6dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static CLIMBERTIMELOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ClimberTimelock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ClimberTimelock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ClimberTimelock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ClimberTimelock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ClimberTimelock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ClimberTimelock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ClimberTimelock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CLIMBERTIMELOCK_ABI.clone(),
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
                CLIMBERTIMELOCK_ABI.clone(),
                CLIMBERTIMELOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delay` (0x6a42b8f8) function
        pub fn delay(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([106, 66, 184, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x2656227d) function
        pub fn execute(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            data_elements: ::std::vec::Vec<::ethers::core::types::Bytes>,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 86, 34, 125], (targets, values, data_elements, salt))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperationId` (0x57f525ed) function
        pub fn get_operation_id(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            data_elements: ::std::vec::Vec<::ethers::core::types::Bytes>,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([87, 245, 37, 237], (targets, values, data_elements, salt))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperationState` (0x7958004c) function
        pub fn get_operation_state(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([121, 88, 0, 76], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operations` (0xc74f349b) function
        pub fn operations(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, bool, bool)> {
            self.0
                .method_hash([199, 79, 52, 155], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `schedule` (0x90bd1e6d) function
        pub fn schedule(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            data_elements: ::std::vec::Vec<::ethers::core::types::Bytes>,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 189, 30, 109], (targets, values, data_elements, salt))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDelay` (0x24adbc5b) function
        pub fn update_delay(
            &self,
            new_delay: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 173, 188, 91], new_delay)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClimberTimelockEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ClimberTimelock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerNotTimelock` with signature `CallerNotTimelock()` and selector `0xdfb49e31`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CallerNotTimelock", abi = "CallerNotTimelock()")]
    pub struct CallerNotTimelock;
    ///Custom Error type `InvalidDataElementsCount` with signature `InvalidDataElementsCount()` and selector `0x76cefbcb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidDataElementsCount", abi = "InvalidDataElementsCount()")]
    pub struct InvalidDataElementsCount;
    ///Custom Error type `InvalidTargetsCount` with signature `InvalidTargetsCount()` and selector `0x576405a3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidTargetsCount", abi = "InvalidTargetsCount()")]
    pub struct InvalidTargetsCount;
    ///Custom Error type `InvalidValuesCount` with signature `InvalidValuesCount()` and selector `0x2e2c60fe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidValuesCount", abi = "InvalidValuesCount()")]
    pub struct InvalidValuesCount;
    ///Custom Error type `NewDelayAboveMax` with signature `NewDelayAboveMax()` and selector `0x78f424c4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NewDelayAboveMax", abi = "NewDelayAboveMax()")]
    pub struct NewDelayAboveMax;
    ///Custom Error type `NotReadyForExecution` with signature `NotReadyForExecution(bytes32)` and selector `0x414afe48`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotReadyForExecution", abi = "NotReadyForExecution(bytes32)")]
    pub struct NotReadyForExecution {
        pub operation_id: [u8; 32],
    }
    ///Custom Error type `OperationAlreadyKnown` with signature `OperationAlreadyKnown(bytes32)` and selector `0x416333a2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OperationAlreadyKnown", abi = "OperationAlreadyKnown(bytes32)")]
    pub struct OperationAlreadyKnown {
        pub operation_id: [u8; 32],
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ClimberTimelockErrors {
        CallerNotTimelock(CallerNotTimelock),
        InvalidDataElementsCount(InvalidDataElementsCount),
        InvalidTargetsCount(InvalidTargetsCount),
        InvalidValuesCount(InvalidValuesCount),
        NewDelayAboveMax(NewDelayAboveMax),
        NotReadyForExecution(NotReadyForExecution),
        OperationAlreadyKnown(OperationAlreadyKnown),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ClimberTimelockErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <CallerNotTimelock as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerNotTimelock(decoded));
            }
            if let Ok(decoded)
                = <InvalidDataElementsCount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidDataElementsCount(decoded));
            }
            if let Ok(decoded)
                = <InvalidTargetsCount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTargetsCount(decoded));
            }
            if let Ok(decoded)
                = <InvalidValuesCount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidValuesCount(decoded));
            }
            if let Ok(decoded)
                = <NewDelayAboveMax as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NewDelayAboveMax(decoded));
            }
            if let Ok(decoded)
                = <NotReadyForExecution as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotReadyForExecution(decoded));
            }
            if let Ok(decoded)
                = <OperationAlreadyKnown as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationAlreadyKnown(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ClimberTimelockErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerNotTimelock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDataElementsCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTargetsCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidValuesCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewDelayAboveMax(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotReadyForExecution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationAlreadyKnown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ClimberTimelockErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerNotTimelock as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDataElementsCount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTargetsCount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidValuesCount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewDelayAboveMax as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotReadyForExecution as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OperationAlreadyKnown as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ClimberTimelockErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerNotTimelock(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidDataElementsCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTargetsCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidValuesCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewDelayAboveMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotReadyForExecution(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperationAlreadyKnown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ClimberTimelockErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerNotTimelock> for ClimberTimelockErrors {
        fn from(value: CallerNotTimelock) -> Self {
            Self::CallerNotTimelock(value)
        }
    }
    impl ::core::convert::From<InvalidDataElementsCount> for ClimberTimelockErrors {
        fn from(value: InvalidDataElementsCount) -> Self {
            Self::InvalidDataElementsCount(value)
        }
    }
    impl ::core::convert::From<InvalidTargetsCount> for ClimberTimelockErrors {
        fn from(value: InvalidTargetsCount) -> Self {
            Self::InvalidTargetsCount(value)
        }
    }
    impl ::core::convert::From<InvalidValuesCount> for ClimberTimelockErrors {
        fn from(value: InvalidValuesCount) -> Self {
            Self::InvalidValuesCount(value)
        }
    }
    impl ::core::convert::From<NewDelayAboveMax> for ClimberTimelockErrors {
        fn from(value: NewDelayAboveMax) -> Self {
            Self::NewDelayAboveMax(value)
        }
    }
    impl ::core::convert::From<NotReadyForExecution> for ClimberTimelockErrors {
        fn from(value: NotReadyForExecution) -> Self {
            Self::NotReadyForExecution(value)
        }
    }
    impl ::core::convert::From<OperationAlreadyKnown> for ClimberTimelockErrors {
        fn from(value: OperationAlreadyKnown) -> Self {
            Self::OperationAlreadyKnown(value)
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
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ClimberTimelockEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ClimberTimelockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ClimberTimelockEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ClimberTimelockEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ClimberTimelockEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ClimberTimelockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ClimberTimelockEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ClimberTimelockEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ClimberTimelockEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `delay` function with signature `delay()` and selector `0x6a42b8f8`
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
    #[ethcall(name = "delay", abi = "delay()")]
    pub struct DelayCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`
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
    #[ethcall(name = "execute", abi = "execute(address[],uint256[],bytes[],bytes32)")]
    pub struct ExecuteCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data_elements: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `getOperationId` function with signature `getOperationId(address[],uint256[],bytes[],bytes32)` and selector `0x57f525ed`
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
        name = "getOperationId",
        abi = "getOperationId(address[],uint256[],bytes[],bytes32)"
    )]
    pub struct GetOperationIdCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data_elements: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `getOperationState` function with signature `getOperationState(bytes32)` and selector `0x7958004c`
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
    #[ethcall(name = "getOperationState", abi = "getOperationState(bytes32)")]
    pub struct GetOperationStateCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operations` function with signature `operations(bytes32)` and selector `0xc74f349b`
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
    #[ethcall(name = "operations", abi = "operations(bytes32)")]
    pub struct OperationsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `schedule` function with signature `schedule(address[],uint256[],bytes[],bytes32)` and selector `0x90bd1e6d`
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
    #[ethcall(name = "schedule", abi = "schedule(address[],uint256[],bytes[],bytes32)")]
    pub struct ScheduleCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data_elements: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `updateDelay` function with signature `updateDelay(uint64)` and selector `0x24adbc5b`
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
    #[ethcall(name = "updateDelay", abi = "updateDelay(uint64)")]
    pub struct UpdateDelayCall {
        pub new_delay: u64,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ClimberTimelockCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        Delay(DelayCall),
        Execute(ExecuteCall),
        GetOperationId(GetOperationIdCall),
        GetOperationState(GetOperationStateCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Operations(OperationsCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        Schedule(ScheduleCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdateDelay(UpdateDelayCall),
    }
    impl ::ethers::core::abi::AbiDecode for ClimberTimelockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <DelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delay(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <GetOperationIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOperationId(decoded));
            }
            if let Ok(decoded)
                = <GetOperationStateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetOperationState(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <OperationsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Operations(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <ScheduleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Schedule(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <UpdateDelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateDelay(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ClimberTimelockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperationId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperationState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Operations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Schedule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ClimberTimelockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delay(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperationId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperationState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operations(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Schedule(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDelay(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ClimberTimelockCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<DelayCall> for ClimberTimelockCalls {
        fn from(value: DelayCall) -> Self {
            Self::Delay(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for ClimberTimelockCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetOperationIdCall> for ClimberTimelockCalls {
        fn from(value: GetOperationIdCall) -> Self {
            Self::GetOperationId(value)
        }
    }
    impl ::core::convert::From<GetOperationStateCall> for ClimberTimelockCalls {
        fn from(value: GetOperationStateCall) -> Self {
            Self::GetOperationState(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ClimberTimelockCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ClimberTimelockCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ClimberTimelockCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<OperationsCall> for ClimberTimelockCalls {
        fn from(value: OperationsCall) -> Self {
            Self::Operations(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ClimberTimelockCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ClimberTimelockCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<ScheduleCall> for ClimberTimelockCalls {
        fn from(value: ScheduleCall) -> Self {
            Self::Schedule(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ClimberTimelockCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpdateDelayCall> for ClimberTimelockCalls {
        fn from(value: UpdateDelayCall) -> Self {
            Self::UpdateDelay(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `delay` function with signature `delay()` and selector `0x6a42b8f8`
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
    pub struct DelayReturn(pub u64);
    ///Container type for all return fields from the `getOperationId` function with signature `getOperationId(address[],uint256[],bytes[],bytes32)` and selector `0x57f525ed`
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
    pub struct GetOperationIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getOperationState` function with signature `getOperationState(bytes32)` and selector `0x7958004c`
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
    pub struct GetOperationStateReturn {
        pub state: u8,
    }
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `operations` function with signature `operations(bytes32)` and selector `0xc74f349b`
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
    pub struct OperationsReturn {
        pub ready_at_timestamp: u64,
        pub known: bool,
        pub executed: bool,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
