pub use trustful_oracle::*;
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
pub mod trustful_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("sources"),
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
                        name: ::std::borrow::ToOwned::to_owned("enableInitialization"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("INITIALIZER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INITIALIZER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("MIN_SOURCES"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MIN_SOURCES"),
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
                    ::std::borrow::ToOwned::to_owned("TRUSTED_SOURCE_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TRUSTED_SOURCE_ROLE",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getAllPricesForSymbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAllPricesForSymbol",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prices"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMedianPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMedianPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("getPriceBySource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPriceBySource"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMember"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMember"),
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
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
                    ::std::borrow::ToOwned::to_owned("postPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("postPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPrice"),
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
                    ::std::borrow::ToOwned::to_owned("setupInitialPrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setupInitialPrices"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sources"),
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
                                    name: ::std::borrow::ToOwned::to_owned("symbols"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prices"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                (
                    ::std::borrow::ToOwned::to_owned("UpdatedPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UpdatedPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPrice"),
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
                    ::std::borrow::ToOwned::to_owned("NotEnoughSources"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughSources"),
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
    pub static TRUSTFULORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x17\xA58\x03\x80a\x17\xA5\x839\x81\x01`@\x81\x90Ra\0/\x91a\x02iV[`\x01\x82Q\x10\x15a\0RW`@Qc\x1A\xBC\x04\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x82Q\x81\x10\x15a\0\xAFWa\0\xA7\x7F\xC6y\x03\x86&I\x9F\xF8j}\xEB\xD1\xAB\xE4|\xFE\xE1\xE3*\x1A\xFC7H\xD33Al.\x93F\x0E\xE5\x84\x83\x81Q\x81\x10a\0\x94Wa\0\x94a\x03?V[` \x02` \x01\x01Qa\0\xE7` \x1B` \x1CV[`\x01\x01a\0UV[P\x80\x15a\0\xE0Wa\0\xE0\x7F0\xD4\x1AY|\xAC\x12}\x82I\xD3\x12\x98\xB5\x0EH\x1E\xE8,?JI\xFF\x93\xC7j\"sZ\xA9\xF3\xAD3a\0\xE7V[PPa\x03UV[a\0\xF1\x82\x82a\0\xF5V[PPV[a\0\xFF\x82\x82a\x01\x1CV[`\0\x82\x81R`\x01` R`@\x90 a\x01\x17\x90\x82a\x01\xBAV[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\0\xF1W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x01v3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x01\xCF\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x01\xD8V[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x02\x1FWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x01\xD2V[P`\0a\x01\xD2V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02TW`\0\x80\xFD[\x91\x90PV[\x80Q\x80\x15\x15\x81\x14a\x02TW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x02|W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x02\x93W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\xA7W`\0\x80\xFD[\x81Q` \x82\x82\x11\x15a\x02\xBBWa\x02\xBBa\x02'V[\x81`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x86\x82\x11\x17\x15a\x02\xE0Wa\x02\xE0a\x02'V[`@R\x92\x83R\x81\x83\x01\x93P\x84\x81\x01\x82\x01\x92\x89\x84\x11\x15a\x02\xFEW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x03#Wa\x03\x14\x86a\x02=V[\x85R\x94\x82\x01\x94\x93\x82\x01\x93a\x03\x03V[\x96Pa\x032\x90P\x87\x82\x01a\x02YV[\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[a\x14A\x80a\x03d`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80c\x90\x10\xD0|\x11a\0\xA2W\x80c\xA2\x17\xFD\xDF\x11a\0qW\x80c\xA2\x17\xFD\xDF\x14a\x02YW\x80c\xBC\xC8-\xC4\x14a\x02aW\x80c\xCA\x15\xC8s\x14a\x02tW\x80c\xD5Gt\x1F\x14a\x02\x87W\x80c\xD9e\xB3\x08\x14a\x02\x9AW`\0\x80\xFD[\x80c\x90\x10\xD0|\x14a\x01\xE6W\x80c\x91i\x1F\xD9\x14a\x02\x11W\x80c\x91\xD1HT\x14a\x02&W\x80c\x9A\xBDJ%\x14a\x029W`\0\x80\xFD[\x80c//\xF1]\x11a\0\xDEW\x80c//\xF1]\x14a\x01\xA3W\x80c6V\x8A\xBE\x14a\x01\xB8W\x80cTbz\xF0\x14a\x01\xCBW\x80cf\x93-\x86\x14a\x01\xD3W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x10W\x80c$\x8A\x9C\xA3\x14a\x018W\x80c'\xDF=>\x14a\x01iW\x80c)\xD1G\x10\x14a\x01\x90W[`\0\x80\xFD[a\x01#a\x01\x1E6`\x04a\x0E\x01V[a\x02\xADV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01[a\x01F6`\x04a\x0E+V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01/V[a\x01[\x7F0\xD4\x1AY|\xAC\x12}\x82I\xD3\x12\x98\xB5\x0EH\x1E\xE8,?JI\xFF\x93\xC7j\"sZ\xA9\xF3\xAD\x81V[a\x01[a\x01\x9E6`\x04a\x0F\x03V[a\x02\xD8V[a\x01\xB6a\x01\xB16`\x04a\x0FQV[a\x03\x15V[\0[a\x01\xB6a\x01\xC66`\x04a\x0FQV[a\x03?V[a\x01[`\x01\x81V[a\x01\xB6a\x01\xE16`\x04a\x0F\xC0V[a\x03\xC2V[a\x01\xF9a\x01\xF46`\x04a\x10ZV[a\x04\xEDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01/V[a\x01[`\0\x80Q` a\x13\xEC\x839\x81Q\x91R\x81V[a\x01#a\x0246`\x04a\x0FQV[a\x05\x0CV[a\x02La\x02G6`\x04a\x10|V[a\x055V[`@Qa\x01/\x91\x90a\x10\xB9V[a\x01[`\0\x81V[a\x01\xB6a\x02o6`\x04a\x11?V[a\x05\xF5V[a\x01[a\x02\x826`\x04a\x0E+V[a\x06UV[a\x01\xB6a\x02\x956`\x04a\x0FQV[a\x06lV[a\x01[a\x02\xA86`\x04a\x11\x8BV[a\x06\x91V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x02\xD2WPa\x02\xD2\x82a\x06\xD2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x80\x82 \x90Qa\x02\xFE\x90\x85\x90a\x11\xF1V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x030\x81a\x07\x07V[a\x03:\x83\x83a\x07\x14V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xBE\x82\x82a\x076V[PPV[\x7F0\xD4\x1AY|\xAC\x12}\x82I\xD3\x12\x98\xB5\x0EH\x1E\xE8,?JI\xFF\x93\xC7j\"sZ\xA9\xF3\xADa\x03\xEC\x81a\x07\x07V[\x85\x84\x14\x80\x15a\x03\xFAWP\x83\x82\x14[a\x04\x03W`\0\x80\xFD[`\0[\x86\x81\x10\x15a\x04\xB9Wa\x04\xB1\x88\x88\x83\x81\x81\x10a\x04#Wa\x04#a\x12\rV[\x90P` \x02\x01` \x81\x01\x90a\x048\x91\x90a\x12#V[\x87\x87\x84\x81\x81\x10a\x04JWa\x04Ja\x12\rV[\x90P` \x02\x81\x01\x90a\x04\\\x91\x90a\x12>V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92P\x88\x91P\x86\x90P\x81\x81\x10a\x04\xA5Wa\x04\xA5a\x12\rV[\x90P` \x02\x015a\x07XV[`\x01\x01a\x04\x06V[Pa\x04\xE4\x7F0\xD4\x1AY|\xAC\x12}\x82I\xD3\x12\x98\xB5\x0EH\x1E\xE8,?JI\xFF\x93\xC7j\"sZ\xA9\xF3\xAD3a\x03?V[PPPPPPPV[`\0\x82\x81R`\x01` R`@\x81 a\x05\x05\x90\x83a\x082V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\0a\x05P`\0\x80Q` a\x13\xEC\x839\x81Q\x91Ra\x06UV[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05kWa\x05ka\x0EDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x94W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81\x81\x10\x15a\x05\xEEW`\0a\x05\xBC`\0\x80Q` a\x13\xEC\x839\x81Q\x91R\x83a\x04\xEDV[\x90Pa\x05\xC8\x85\x82a\x02\xD8V[\x84\x83\x81Q\x81\x10a\x05\xDAWa\x05\xDAa\x12\rV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05\x9AV[PP\x91\x90PV[`\0\x80Q` a\x13\xEC\x839\x81Q\x91Ra\x06\r\x81a\x07\x07V[a\x06O3\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92Pa\x07X\x91PPV[PPPPV[`\0\x81\x81R`\x01` R`@\x81 a\x02\xD2\x90a\x08>V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x06\x87\x81a\x07\x07V[a\x03:\x83\x83a\x076V[`\0a\x05\x05\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08H\x92PPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\xD2WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\xD2V[a\x07\x11\x813a\t&V[PV[a\x07\x1E\x82\x82a\t\x7FV[`\0\x82\x81R`\x01` R`@\x90 a\x03:\x90\x82a\n\x03V[a\x07@\x82\x82a\n\x18V[`\0\x82\x81R`\x01` R`@\x90 a\x03:\x90\x82a\n}V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x80\x82 \x90Qa\x07~\x90\x85\x90a\x11\xF1V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x81`\x02`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x84`@Qa\x07\xC3\x91\x90a\x11\xF1V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91Ua\x07\xE1\x90\x84\x90a\x11\xF1V[`@\x80Q\x91\x82\x90\x03\x82 \x83\x83R` \x83\x01\x85\x90R\x91`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F~\xBF\xC8\xF1\xD3\xDF\xF8\xB5l\xE7\x0E\x11]\x8B\xA5\xB1\x8D\x9A\xF7b\x18?'\xEE4\xE3\xE7\xEC^\xF9_\x9F\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[`\0a\x05\x05\x83\x83a\n\x92V[`\0a\x02\xD2\x82T\x90V[`\0\x80a\x08T\x83a\x055V[\x90Pa\x08_\x81a\n\xBCV[`\x02\x81Qa\x08m\x91\x90a\x12\x9BV[`\0\x03a\x08\xF7W`\0\x81`\x01`\x02\x84Qa\x08\x87\x91\x90a\x12\xC5V[a\x08\x91\x91\x90a\x12\xD9V[\x81Q\x81\x10a\x08\xA1Wa\x08\xA1a\x12\rV[` \x02` \x01\x01Q\x90P`\0\x82`\x02\x84Qa\x08\xBC\x91\x90a\x12\xC5V[\x81Q\x81\x10a\x08\xCCWa\x08\xCCa\x12\rV[` \x02` \x01\x01Q\x90P`\x02\x81\x83a\x08\xE4\x91\x90a\x12\xECV[a\x08\xEE\x91\x90a\x12\xC5V[\x95\x94PPPPPV[\x80`\x02\x82Qa\t\x06\x91\x90a\x12\xC5V[\x81Q\x81\x10a\t\x16Wa\t\x16a\x12\rV[` \x02` \x01\x01Q\x91PP\x91\x90PV[a\t0\x82\x82a\x05\x0CV[a\x03\xBEWa\t=\x81a\x0B\x11V[a\tH\x83` a\x0B#V[`@Q` \x01a\tY\x92\x91\x90a\x12\xFFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\xAB\x91`\x04\x01a\x13tV[a\t\x89\x82\x82a\x05\x0CV[a\x03\xBEW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\xBF3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x05\x05\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0C\xBFV[a\n\"\x82\x82a\x05\x0CV[\x15a\x03\xBEW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x05\x05\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\r\x0EV[`\0\x82`\0\x01\x82\x81T\x81\x10a\n\xA9Wa\n\xA9a\x12\rV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[\x80Q`\0\x82R\x80`\x05\x1B\x82\x01` `\x1F\x19\x81\x85\x01[\x82\x01\x83\x81\x11a\x0B\tW\x80Q\x82\x82\x01\x80Q\x82\x81\x11a\n\xF0WPPPa\n\xD1V[[\x81\x86\x01R\x83\x01\x80Q\x82\x81\x11a\n\xF1WP\x84\x01Ra\n\xD1V[PPP\x91RPV[``a\x02\xD2`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B2\x83`\x02a\x13\xA7V[a\x0B=\x90`\x02a\x12\xECV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BUWa\x0BUa\x0EDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\x7FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\x9AWa\x0B\x9Aa\x12\rV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0B\xC9Wa\x0B\xC9a\x12\rV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0B\xED\x84`\x02a\x13\xA7V[a\x0B\xF8\x90`\x01a\x12\xECV[\x90P[`\x01\x81\x11\x15a\x0CpWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0C,Wa\x0C,a\x12\rV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0CBWa\x0CBa\x12\rV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0Ci\x81a\x13\xBEV[\x90Pa\x0B\xFBV[P\x83\x15a\x05\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03\xABV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\r\x06WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\xD2V[P`\0a\x02\xD2V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\r\xF7W`\0a\r2`\x01\x83a\x12\xD9V[\x85T\x90\x91P`\0\x90a\rF\x90`\x01\x90a\x12\xD9V[\x90P\x81\x81\x14a\r\xABW`\0\x86`\0\x01\x82\x81T\x81\x10a\rfWa\rfa\x12\rV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\r\x89Wa\r\x89a\x12\rV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\r\xBCWa\r\xBCa\x13\xD5V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02\xD2V[`\0\x91PPa\x02\xD2V[`\0` \x82\x84\x03\x12\x15a\x0E\x13W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\x05W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E=W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0EkW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x86Wa\x0E\x86a\x0EDV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0E\xAEWa\x0E\xAEa\x0EDV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0E\xC7W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xFEW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x16W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F-W`\0\x80\xFD[a\x0F9\x85\x82\x86\x01a\x0EZV[\x92PPa\x0FH` \x84\x01a\x0E\xE7V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FdW`\0\x80\xFD[\x825\x91Pa\x0FH` \x84\x01a\x0E\xE7V[`\0\x80\x83`\x1F\x84\x01\x12a\x0F\x86W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9EW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0F\xB9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x0F\xD9W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xF1W`\0\x80\xFD[a\x0F\xFD\x8A\x83\x8B\x01a\x0FtV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a\x10\x16W`\0\x80\xFD[a\x10\"\x8A\x83\x8B\x01a\x0FtV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a\x10;W`\0\x80\xFD[Pa\x10H\x89\x82\x8A\x01a\x0FtV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10mW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x10\x8EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xA5W`\0\x80\xFD[a\x10\xB1\x84\x82\x85\x01a\x0EZV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10\xF1W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10\xD5V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x11\x0FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11'W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0F\xB9W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x11TW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11kW`\0\x80\xFD[a\x11w\x86\x82\x87\x01a\x10\xFDV[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x11\x9EW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xB5W`\0\x80\xFD[a\x11\xC1\x85\x82\x86\x01a\x10\xFDV[\x90\x96\x90\x95P\x93PPPPV[`\0[\x83\x81\x10\x15a\x11\xE8W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xD0V[PP`\0\x91\x01RV[`\0\x82Qa\x12\x03\x81\x84` \x87\x01a\x11\xCDV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x125W`\0\x80\xFD[a\x05\x05\x82a\x0E\xE7V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x12UW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12pW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0F\xB9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x12\xAAWa\x12\xAAa\x12\x85V[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x12\xD4Wa\x12\xD4a\x12\x85V[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02\xD2Wa\x02\xD2a\x12\xAFV[\x80\x82\x01\x80\x82\x11\x15a\x02\xD2Wa\x02\xD2a\x12\xAFV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x137\x81`\x17\x85\x01` \x88\x01a\x11\xCDV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x13h\x81`(\x84\x01` \x88\x01a\x11\xCDV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x13\x93\x81`@\x85\x01` \x87\x01a\x11\xCDV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xD2Wa\x02\xD2a\x12\xAFV[`\0\x81a\x13\xCDWa\x13\xCDa\x12\xAFV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xC6y\x03\x86&I\x9F\xF8j}\xEB\xD1\xAB\xE4|\xFE\xE1\xE3*\x1A\xFC7H\xD33Al.\x93F\x0E\xE5\xA2dipfsX\"\x12 \xF3\xB9\x9E\xED\xE1\xA6`Y,k\xC9\x8E\x9D\xDA\x910\xB9_\xCF\x9C\x08\x0E\n\"\xCE\x05\xEA\xF3\x82\xCB\xA3\xDEdsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static TRUSTFULORACLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80c\x90\x10\xD0|\x11a\0\xA2W\x80c\xA2\x17\xFD\xDF\x11a\0qW\x80c\xA2\x17\xFD\xDF\x14a\x02YW\x80c\xBC\xC8-\xC4\x14a\x02aW\x80c\xCA\x15\xC8s\x14a\x02tW\x80c\xD5Gt\x1F\x14a\x02\x87W\x80c\xD9e\xB3\x08\x14a\x02\x9AW`\0\x80\xFD[\x80c\x90\x10\xD0|\x14a\x01\xE6W\x80c\x91i\x1F\xD9\x14a\x02\x11W\x80c\x91\xD1HT\x14a\x02&W\x80c\x9A\xBDJ%\x14a\x029W`\0\x80\xFD[\x80c//\xF1]\x11a\0\xDEW\x80c//\xF1]\x14a\x01\xA3W\x80c6V\x8A\xBE\x14a\x01\xB8W\x80cTbz\xF0\x14a\x01\xCBW\x80cf\x93-\x86\x14a\x01\xD3W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x10W\x80c$\x8A\x9C\xA3\x14a\x018W\x80c'\xDF=>\x14a\x01iW\x80c)\xD1G\x10\x14a\x01\x90W[`\0\x80\xFD[a\x01#a\x01\x1E6`\x04a\x0E\x01V[a\x02\xADV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01[a\x01F6`\x04a\x0E+V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01/V[a\x01[\x7F0\xD4\x1AY|\xAC\x12}\x82I\xD3\x12\x98\xB5\x0EH\x1E\xE8,?JI\xFF\x93\xC7j\"sZ\xA9\xF3\xAD\x81V[a\x01[a\x01\x9E6`\x04a\x0F\x03V[a\x02\xD8V[a\x01\xB6a\x01\xB16`\x04a\x0FQV[a\x03\x15V[\0[a\x01\xB6a\x01\xC66`\x04a\x0FQV[a\x03?V[a\x01[`\x01\x81V[a\x01\xB6a\x01\xE16`\x04a\x0F\xC0V[a\x03\xC2V[a\x01\xF9a\x01\xF46`\x04a\x10ZV[a\x04\xEDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01/V[a\x01[`\0\x80Q` a\x13\xEC\x839\x81Q\x91R\x81V[a\x01#a\x0246`\x04a\x0FQV[a\x05\x0CV[a\x02La\x02G6`\x04a\x10|V[a\x055V[`@Qa\x01/\x91\x90a\x10\xB9V[a\x01[`\0\x81V[a\x01\xB6a\x02o6`\x04a\x11?V[a\x05\xF5V[a\x01[a\x02\x826`\x04a\x0E+V[a\x06UV[a\x01\xB6a\x02\x956`\x04a\x0FQV[a\x06lV[a\x01[a\x02\xA86`\x04a\x11\x8BV[a\x06\x91V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x02\xD2WPa\x02\xD2\x82a\x06\xD2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x80\x82 \x90Qa\x02\xFE\x90\x85\x90a\x11\xF1V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x030\x81a\x07\x07V[a\x03:\x83\x83a\x07\x14V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xBE\x82\x82a\x076V[PPV[\x7F0\xD4\x1AY|\xAC\x12}\x82I\xD3\x12\x98\xB5\x0EH\x1E\xE8,?JI\xFF\x93\xC7j\"sZ\xA9\xF3\xADa\x03\xEC\x81a\x07\x07V[\x85\x84\x14\x80\x15a\x03\xFAWP\x83\x82\x14[a\x04\x03W`\0\x80\xFD[`\0[\x86\x81\x10\x15a\x04\xB9Wa\x04\xB1\x88\x88\x83\x81\x81\x10a\x04#Wa\x04#a\x12\rV[\x90P` \x02\x01` \x81\x01\x90a\x048\x91\x90a\x12#V[\x87\x87\x84\x81\x81\x10a\x04JWa\x04Ja\x12\rV[\x90P` \x02\x81\x01\x90a\x04\\\x91\x90a\x12>V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92P\x88\x91P\x86\x90P\x81\x81\x10a\x04\xA5Wa\x04\xA5a\x12\rV[\x90P` \x02\x015a\x07XV[`\x01\x01a\x04\x06V[Pa\x04\xE4\x7F0\xD4\x1AY|\xAC\x12}\x82I\xD3\x12\x98\xB5\x0EH\x1E\xE8,?JI\xFF\x93\xC7j\"sZ\xA9\xF3\xAD3a\x03?V[PPPPPPPV[`\0\x82\x81R`\x01` R`@\x81 a\x05\x05\x90\x83a\x082V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\0a\x05P`\0\x80Q` a\x13\xEC\x839\x81Q\x91Ra\x06UV[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05kWa\x05ka\x0EDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x94W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81\x81\x10\x15a\x05\xEEW`\0a\x05\xBC`\0\x80Q` a\x13\xEC\x839\x81Q\x91R\x83a\x04\xEDV[\x90Pa\x05\xC8\x85\x82a\x02\xD8V[\x84\x83\x81Q\x81\x10a\x05\xDAWa\x05\xDAa\x12\rV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05\x9AV[PP\x91\x90PV[`\0\x80Q` a\x13\xEC\x839\x81Q\x91Ra\x06\r\x81a\x07\x07V[a\x06O3\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92Pa\x07X\x91PPV[PPPPV[`\0\x81\x81R`\x01` R`@\x81 a\x02\xD2\x90a\x08>V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x06\x87\x81a\x07\x07V[a\x03:\x83\x83a\x076V[`\0a\x05\x05\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08H\x92PPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\xD2WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\xD2V[a\x07\x11\x813a\t&V[PV[a\x07\x1E\x82\x82a\t\x7FV[`\0\x82\x81R`\x01` R`@\x90 a\x03:\x90\x82a\n\x03V[a\x07@\x82\x82a\n\x18V[`\0\x82\x81R`\x01` R`@\x90 a\x03:\x90\x82a\n}V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x80\x82 \x90Qa\x07~\x90\x85\x90a\x11\xF1V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x81`\x02`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x84`@Qa\x07\xC3\x91\x90a\x11\xF1V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x91\x90\x91Ua\x07\xE1\x90\x84\x90a\x11\xF1V[`@\x80Q\x91\x82\x90\x03\x82 \x83\x83R` \x83\x01\x85\x90R\x91`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F~\xBF\xC8\xF1\xD3\xDF\xF8\xB5l\xE7\x0E\x11]\x8B\xA5\xB1\x8D\x9A\xF7b\x18?'\xEE4\xE3\xE7\xEC^\xF9_\x9F\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[`\0a\x05\x05\x83\x83a\n\x92V[`\0a\x02\xD2\x82T\x90V[`\0\x80a\x08T\x83a\x055V[\x90Pa\x08_\x81a\n\xBCV[`\x02\x81Qa\x08m\x91\x90a\x12\x9BV[`\0\x03a\x08\xF7W`\0\x81`\x01`\x02\x84Qa\x08\x87\x91\x90a\x12\xC5V[a\x08\x91\x91\x90a\x12\xD9V[\x81Q\x81\x10a\x08\xA1Wa\x08\xA1a\x12\rV[` \x02` \x01\x01Q\x90P`\0\x82`\x02\x84Qa\x08\xBC\x91\x90a\x12\xC5V[\x81Q\x81\x10a\x08\xCCWa\x08\xCCa\x12\rV[` \x02` \x01\x01Q\x90P`\x02\x81\x83a\x08\xE4\x91\x90a\x12\xECV[a\x08\xEE\x91\x90a\x12\xC5V[\x95\x94PPPPPV[\x80`\x02\x82Qa\t\x06\x91\x90a\x12\xC5V[\x81Q\x81\x10a\t\x16Wa\t\x16a\x12\rV[` \x02` \x01\x01Q\x91PP\x91\x90PV[a\t0\x82\x82a\x05\x0CV[a\x03\xBEWa\t=\x81a\x0B\x11V[a\tH\x83` a\x0B#V[`@Q` \x01a\tY\x92\x91\x90a\x12\xFFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\xAB\x91`\x04\x01a\x13tV[a\t\x89\x82\x82a\x05\x0CV[a\x03\xBEW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\xBF3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x05\x05\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0C\xBFV[a\n\"\x82\x82a\x05\x0CV[\x15a\x03\xBEW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x05\x05\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\r\x0EV[`\0\x82`\0\x01\x82\x81T\x81\x10a\n\xA9Wa\n\xA9a\x12\rV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[\x80Q`\0\x82R\x80`\x05\x1B\x82\x01` `\x1F\x19\x81\x85\x01[\x82\x01\x83\x81\x11a\x0B\tW\x80Q\x82\x82\x01\x80Q\x82\x81\x11a\n\xF0WPPPa\n\xD1V[[\x81\x86\x01R\x83\x01\x80Q\x82\x81\x11a\n\xF1WP\x84\x01Ra\n\xD1V[PPP\x91RPV[``a\x02\xD2`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B2\x83`\x02a\x13\xA7V[a\x0B=\x90`\x02a\x12\xECV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BUWa\x0BUa\x0EDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\x7FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\x9AWa\x0B\x9Aa\x12\rV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0B\xC9Wa\x0B\xC9a\x12\rV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0B\xED\x84`\x02a\x13\xA7V[a\x0B\xF8\x90`\x01a\x12\xECV[\x90P[`\x01\x81\x11\x15a\x0CpWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0C,Wa\x0C,a\x12\rV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0CBWa\x0CBa\x12\rV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0Ci\x81a\x13\xBEV[\x90Pa\x0B\xFBV[P\x83\x15a\x05\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03\xABV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\r\x06WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\xD2V[P`\0a\x02\xD2V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\r\xF7W`\0a\r2`\x01\x83a\x12\xD9V[\x85T\x90\x91P`\0\x90a\rF\x90`\x01\x90a\x12\xD9V[\x90P\x81\x81\x14a\r\xABW`\0\x86`\0\x01\x82\x81T\x81\x10a\rfWa\rfa\x12\rV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\r\x89Wa\r\x89a\x12\rV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\r\xBCWa\r\xBCa\x13\xD5V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02\xD2V[`\0\x91PPa\x02\xD2V[`\0` \x82\x84\x03\x12\x15a\x0E\x13W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\x05W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E=W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0EkW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x86Wa\x0E\x86a\x0EDV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0E\xAEWa\x0E\xAEa\x0EDV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0E\xC7W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xFEW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x16W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F-W`\0\x80\xFD[a\x0F9\x85\x82\x86\x01a\x0EZV[\x92PPa\x0FH` \x84\x01a\x0E\xE7V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FdW`\0\x80\xFD[\x825\x91Pa\x0FH` \x84\x01a\x0E\xE7V[`\0\x80\x83`\x1F\x84\x01\x12a\x0F\x86W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9EW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0F\xB9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x0F\xD9W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xF1W`\0\x80\xFD[a\x0F\xFD\x8A\x83\x8B\x01a\x0FtV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a\x10\x16W`\0\x80\xFD[a\x10\"\x8A\x83\x8B\x01a\x0FtV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a\x10;W`\0\x80\xFD[Pa\x10H\x89\x82\x8A\x01a\x0FtV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10mW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x10\x8EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xA5W`\0\x80\xFD[a\x10\xB1\x84\x82\x85\x01a\x0EZV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10\xF1W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10\xD5V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x11\x0FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11'W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0F\xB9W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x11TW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11kW`\0\x80\xFD[a\x11w\x86\x82\x87\x01a\x10\xFDV[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x11\x9EW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xB5W`\0\x80\xFD[a\x11\xC1\x85\x82\x86\x01a\x10\xFDV[\x90\x96\x90\x95P\x93PPPPV[`\0[\x83\x81\x10\x15a\x11\xE8W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xD0V[PP`\0\x91\x01RV[`\0\x82Qa\x12\x03\x81\x84` \x87\x01a\x11\xCDV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x125W`\0\x80\xFD[a\x05\x05\x82a\x0E\xE7V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x12UW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12pW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0F\xB9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x12\xAAWa\x12\xAAa\x12\x85V[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x12\xD4Wa\x12\xD4a\x12\x85V[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02\xD2Wa\x02\xD2a\x12\xAFV[\x80\x82\x01\x80\x82\x11\x15a\x02\xD2Wa\x02\xD2a\x12\xAFV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x137\x81`\x17\x85\x01` \x88\x01a\x11\xCDV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x13h\x81`(\x84\x01` \x88\x01a\x11\xCDV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x13\x93\x81`@\x85\x01` \x87\x01a\x11\xCDV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xD2Wa\x02\xD2a\x12\xAFV[`\0\x81a\x13\xCDWa\x13\xCDa\x12\xAFV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xC6y\x03\x86&I\x9F\xF8j}\xEB\xD1\xAB\xE4|\xFE\xE1\xE3*\x1A\xFC7H\xD33Al.\x93F\x0E\xE5\xA2dipfsX\"\x12 \xF3\xB9\x9E\xED\xE1\xA6`Y,k\xC9\x8E\x9D\xDA\x910\xB9_\xCF\x9C\x08\x0E\n\"\xCE\x05\xEA\xF3\x82\xCB\xA3\xDEdsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static TRUSTFULORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct TrustfulOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TrustfulOracle<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for TrustfulOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for TrustfulOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for TrustfulOracle<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TrustfulOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TrustfulOracle<M> {
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
                TRUSTFULORACLE_ABI.clone(),
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
                TRUSTFULORACLE_ABI.clone(),
                TRUSTFULORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE`
        /// (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INITIALIZER_ROLE`
        /// (0x27df3d3e) function
        pub fn initializer_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([39, 223, 61, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MIN_SOURCES` (0x54627af0)
        /// function
        pub fn min_sources(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([84, 98, 122, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TRUSTED_SOURCE_ROLE`
        /// (0x91691fd9) function
        pub fn trusted_source_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([145, 105, 31, 217], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllPricesForSymbol`
        /// (0x9abd4a25) function
        pub fn get_all_prices_for_symbol(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([154, 189, 74, 37], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMedianPrice`
        /// (0xd965b308) function
        pub fn get_median_price(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([217, 101, 179, 8], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceBySource`
        /// (0x29d14710) function
        pub fn get_price_by_source(
            &self,
            symbol: ::std::string::String,
            source: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([41, 209, 71, 16], (symbol, source))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3)
        /// function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMember`
        /// (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount`
        /// (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d)
        /// function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854)
        /// function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `postPrice` (0xbcc82dc4)
        /// function
        pub fn post_price(
            &self,
            symbol: ::std::string::String,
            new_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 200, 45, 196], (symbol, new_price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe)
        /// function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f)
        /// function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setupInitialPrices`
        /// (0x66932d86) function
        pub fn setup_initial_prices(
            &self,
            sources: ::std::vec::Vec<::ethers::core::types::Address>,
            symbols: ::std::vec::Vec<::std::string::String>,
            prices: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 147, 45, 134], (sources, symbols, prices))
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
        ///Gets the contract's `UpdatedPrice` event
        pub fn updated_price_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdatedPriceFilter,
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
            TrustfulOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for TrustfulOracle<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotEnoughSources` with signature
    /// `NotEnoughSources()` and selector `0x357809a6`
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
    #[etherror(name = "NotEnoughSources", abi = "NotEnoughSources()")]
    pub struct NotEnoughSources;
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
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RoleGranted",
        abi = "RoleGranted(bytes32,address,address)"
    )]
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
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RoleRevoked",
        abi = "RoleRevoked(bytes32,address,address)"
    )]
    pub struct RoleRevokedFilter {
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
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "UpdatedPrice",
        abi = "UpdatedPrice(address,string,uint256,uint256)"
    )]
    pub struct UpdatedPriceFilter {
        #[ethevent(indexed)]
        pub source: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub symbol: ::ethers::core::types::H256,
        pub old_price: ::ethers::core::types::U256,
        pub new_price: ::ethers::core::types::U256,
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
    pub enum TrustfulOracleEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpdatedPriceFilter(UpdatedPriceFilter),
    }
    impl ::ethers::contract::EthLogDecode for TrustfulOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(TrustfulOracleEvents::RoleAdminChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(TrustfulOracleEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(TrustfulOracleEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpdatedPriceFilter::decode_log(log) {
                return Ok(TrustfulOracleEvents::UpdatedPriceFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TrustfulOracleEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdatedPriceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for TrustfulOracleEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for TrustfulOracleEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for TrustfulOracleEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpdatedPriceFilter> for TrustfulOracleEvents {
        fn from(value: UpdatedPriceFilter) -> Self {
            Self::UpdatedPriceFilter(value)
        }
    }
    ///Container type for all input parameters for the
    /// `DEFAULT_ADMIN_ROLE` function with signature
    /// `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the
    /// `INITIALIZER_ROLE` function with signature
    /// `INITIALIZER_ROLE()` and selector `0x27df3d3e`
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
    #[ethcall(name = "INITIALIZER_ROLE", abi = "INITIALIZER_ROLE()")]
    pub struct InitializerRoleCall;
    ///Container type for all input parameters for the
    /// `MIN_SOURCES` function with signature
    /// `MIN_SOURCES()` and selector `0x54627af0`
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
    #[ethcall(name = "MIN_SOURCES", abi = "MIN_SOURCES()")]
    pub struct MinSourcesCall;
    ///Container type for all input parameters for the
    /// `TRUSTED_SOURCE_ROLE` function with signature
    /// `TRUSTED_SOURCE_ROLE()` and selector `0x91691fd9`
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
    #[ethcall(name = "TRUSTED_SOURCE_ROLE", abi = "TRUSTED_SOURCE_ROLE()")]
    pub struct TrustedSourceRoleCall;
    ///Container type for all input parameters for the
    /// `getAllPricesForSymbol` function with signature
    /// `getAllPricesForSymbol(string)` and selector
    /// `0x9abd4a25`
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
        name = "getAllPricesForSymbol",
        abi = "getAllPricesForSymbol(string)"
    )]
    pub struct GetAllPricesForSymbolCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the
    /// `getMedianPrice` function with signature
    /// `getMedianPrice(string)` and selector `0xd965b308`
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
    #[ethcall(name = "getMedianPrice", abi = "getMedianPrice(string)")]
    pub struct GetMedianPriceCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the
    /// `getPriceBySource` function with signature
    /// `getPriceBySource(string,address)` and selector
    /// `0x29d14710`
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
        name = "getPriceBySource",
        abi = "getPriceBySource(string,address)"
    )]
    pub struct GetPriceBySourceCall {
        pub symbol: ::std::string::String,
        pub source: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `getRoleAdmin` function with signature
    /// `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the
    /// `getRoleMember` function with signature
    /// `getRoleMember(bytes32,uint256)` and selector
    /// `0x9010d07c`
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
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `getRoleMemberCount` function with signature
    /// `getRoleMemberCount(bytes32)` and selector
    /// `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the
    /// `grantRole` function with signature
    /// `grantRole(bytes32,address)` and selector
    /// `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `hasRole` function with signature
    /// `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `postPrice` function with signature
    /// `postPrice(string,uint256)` and selector
    /// `0xbcc82dc4`
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
    #[ethcall(name = "postPrice", abi = "postPrice(string,uint256)")]
    pub struct PostPriceCall {
        pub symbol: ::std::string::String,
        pub new_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `renounceRole` function with signature
    /// `renounceRole(bytes32,address)` and selector
    /// `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `revokeRole` function with signature
    /// `revokeRole(bytes32,address)` and selector
    /// `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `setupInitialPrices` function with signature
    /// `setupInitialPrices(address[],string[],uint256[])`
    /// and selector `0x66932d86`
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
        name = "setupInitialPrices",
        abi = "setupInitialPrices(address[],string[],uint256[])"
    )]
    pub struct SetupInitialPricesCall {
        pub sources: ::std::vec::Vec<::ethers::core::types::Address>,
        pub symbols: ::std::vec::Vec<::std::string::String>,
        pub prices: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the
    /// `supportsInterface` function with signature
    /// `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
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
    pub enum TrustfulOracleCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        InitializerRole(InitializerRoleCall),
        MinSources(MinSourcesCall),
        TrustedSourceRole(TrustedSourceRoleCall),
        GetAllPricesForSymbol(GetAllPricesForSymbolCall),
        GetMedianPrice(GetMedianPriceCall),
        GetPriceBySource(GetPriceBySourceCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        PostPrice(PostPriceCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetupInitialPrices(SetupInitialPricesCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for TrustfulOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <InitializerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InitializerRole(decoded));
            }
            if let Ok(decoded) =
                <MinSourcesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinSources(decoded));
            }
            if let Ok(decoded) = <TrustedSourceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TrustedSourceRole(decoded));
            }
            if let Ok(decoded) = <GetAllPricesForSymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAllPricesForSymbol(decoded));
            }
            if let Ok(decoded) =
                <GetMedianPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetMedianPrice(decoded));
            }
            if let Ok(decoded) =
                <GetPriceBySourceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetPriceBySource(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) =
                <PostPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PostPrice(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SetupInitialPricesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetupInitialPrices(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TrustfulOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinSources(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrustedSourceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllPricesForSymbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMedianPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriceBySource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PostPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetupInitialPrices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TrustfulOracleCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinSources(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TrustedSourceRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAllPricesForSymbol(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMedianPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPriceBySource(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleMember(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PostPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetupInitialPrices(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for TrustfulOracleCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<InitializerRoleCall> for TrustfulOracleCalls {
        fn from(value: InitializerRoleCall) -> Self {
            Self::InitializerRole(value)
        }
    }
    impl ::core::convert::From<MinSourcesCall> for TrustfulOracleCalls {
        fn from(value: MinSourcesCall) -> Self { Self::MinSources(value) }
    }
    impl ::core::convert::From<TrustedSourceRoleCall> for TrustfulOracleCalls {
        fn from(value: TrustedSourceRoleCall) -> Self {
            Self::TrustedSourceRole(value)
        }
    }
    impl ::core::convert::From<GetAllPricesForSymbolCall> for TrustfulOracleCalls {
        fn from(value: GetAllPricesForSymbolCall) -> Self {
            Self::GetAllPricesForSymbol(value)
        }
    }
    impl ::core::convert::From<GetMedianPriceCall> for TrustfulOracleCalls {
        fn from(value: GetMedianPriceCall) -> Self {
            Self::GetMedianPrice(value)
        }
    }
    impl ::core::convert::From<GetPriceBySourceCall> for TrustfulOracleCalls {
        fn from(value: GetPriceBySourceCall) -> Self {
            Self::GetPriceBySource(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for TrustfulOracleCalls {
        fn from(value: GetRoleAdminCall) -> Self { Self::GetRoleAdmin(value) }
    }
    impl ::core::convert::From<GetRoleMemberCall> for TrustfulOracleCalls {
        fn from(value: GetRoleMemberCall) -> Self { Self::GetRoleMember(value) }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for TrustfulOracleCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for TrustfulOracleCalls {
        fn from(value: GrantRoleCall) -> Self { Self::GrantRole(value) }
    }
    impl ::core::convert::From<HasRoleCall> for TrustfulOracleCalls {
        fn from(value: HasRoleCall) -> Self { Self::HasRole(value) }
    }
    impl ::core::convert::From<PostPriceCall> for TrustfulOracleCalls {
        fn from(value: PostPriceCall) -> Self { Self::PostPrice(value) }
    }
    impl ::core::convert::From<RenounceRoleCall> for TrustfulOracleCalls {
        fn from(value: RenounceRoleCall) -> Self { Self::RenounceRole(value) }
    }
    impl ::core::convert::From<RevokeRoleCall> for TrustfulOracleCalls {
        fn from(value: RevokeRoleCall) -> Self { Self::RevokeRole(value) }
    }
    impl ::core::convert::From<SetupInitialPricesCall> for TrustfulOracleCalls {
        fn from(value: SetupInitialPricesCall) -> Self {
            Self::SetupInitialPrices(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for TrustfulOracleCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the
    /// `DEFAULT_ADMIN_ROLE` function with signature
    /// `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the
    /// `INITIALIZER_ROLE` function with signature
    /// `INITIALIZER_ROLE()` and selector `0x27df3d3e`
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
    pub struct InitializerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the
    /// `MIN_SOURCES` function with signature
    /// `MIN_SOURCES()` and selector `0x54627af0`
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
    pub struct MinSourcesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `TRUSTED_SOURCE_ROLE` function with signature
    /// `TRUSTED_SOURCE_ROLE()` and selector `0x91691fd9`
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
    pub struct TrustedSourceRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the
    /// `getAllPricesForSymbol` function with signature
    /// `getAllPricesForSymbol(string)` and selector
    /// `0x9abd4a25`
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
    pub struct GetAllPricesForSymbolReturn {
        pub prices: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the
    /// `getMedianPrice` function with signature
    /// `getMedianPrice(string)` and selector `0xd965b308`
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
    pub struct GetMedianPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getPriceBySource` function with signature
    /// `getPriceBySource(string,address)` and selector
    /// `0x29d14710`
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
    pub struct GetPriceBySourceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getRoleAdmin` function with signature
    /// `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the
    /// `getRoleMember` function with signature
    /// `getRoleMember(bytes32,uint256)` and selector
    /// `0x9010d07c`
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
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `getRoleMemberCount` function with signature
    /// `getRoleMemberCount(bytes32)` and selector
    /// `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `hasRole` function with signature
    /// `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the
    /// `supportsInterface` function with signature
    /// `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
