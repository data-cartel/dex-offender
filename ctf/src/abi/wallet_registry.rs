pub use wallet_registry::*;
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
pub mod wallet_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("masterCopyAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("walletFactoryAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialBeneficiaries"),
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
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addBeneficiary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addBeneficiary"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("beneficiary"),
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
                    ::std::borrow::ToOwned::to_owned("beneficiaries"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beneficiaries"),
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
                    ::std::borrow::ToOwned::to_owned("masterCopy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("masterCopy"),
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
                    ::std::borrow::ToOwned::to_owned("proxyCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxyCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GnosisSafeProxy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                (
                    ::std::borrow::ToOwned::to_owned("walletFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("walletFactory"),
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
                    ::std::borrow::ToOwned::to_owned("wallets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wallets"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallerNotFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallerNotFactory"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FakeMasterCopy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FakeMasterCopy"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFallbackManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidFallbackManager",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidInitialization",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOwnersCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidOwnersCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidThreshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("NotEnoughFunds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughFunds"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnerIsNotABeneficiary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnerIsNotABeneficiary",
                            ),
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
    pub static WALLETREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x0FX8\x03\x80b\0\x0FX\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x014V[b\0\0?3b\0\0\xC5V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x80R\x83\x81\x16`\xA0R\x82\x16`\xC0R`\0[\x81Q\x81\x10\x15b\0\0\xBAW`\x01`\0\x80\x84\x84\x81Q\x81\x10b\0\0\x7FWb\0\0\x7Fb\0\x02?V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\x01\x01b\0\0[V[PPPPPb\0\x02UV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80`\0\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x19W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01KW`\0\x80\xFD[b\0\x01V\x85b\0\x01\x01V[\x93P` b\0\x01g\x81\x87\x01b\0\x01\x01V[\x93Pb\0\x01w`@\x87\x01b\0\x01\x01V[``\x87\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01\x95W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12b\0\x01\xAAW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x01\xBFWb\0\x01\xBFb\0\x01\x1EV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15b\0\x01\xE7Wb\0\x01\xE7b\0\x01\x1EV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x8B\x83\x11\x15b\0\x02\x06W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x02/Wb\0\x02\x1F\x85b\0\x01\x01V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92b\0\x02\x0BV[\x98\x9B\x97\x9AP\x95\x98PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x80Q`\xA0Q`\xC0Qa\x0C\xB7b\0\x02\xA1`\09`\0\x81\x81a\x02\xB1\x01R\x81\x81a\x033\x01Ra\x06\xA8\x01R`\0\x81\x81a\x029\x01Ra\x03\xD1\x01R`\0\x81\x81a\x02\x05\x01Ra\x04\x11\x01Ra\x0C\xB7`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xE8W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x8AW\x80c\xF0N(>\x11a\0YW\x80c\xF0N(>\x14a\x02yW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x8CW\x80c\xFC\x0CTj\x14a\x02\x9FW\x80c\xFE\xE8\x1C\xF4\x14a\x02\xD3W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\xDAW\x80c\xA6\x19Hn\x14a\x01\xF3W\x80c\xC5\xC06\x99\x14a\x02'W\x80c\xD7S?\x02\x14a\x02[W`\0\x80\xFD[\x80cT\xD1\xF1=\x11a\0\xC6W\x80cT\xD1\xF1=\x14a\x01\\W\x80cY&e\x1D\x14a\x01dW\x80cqP\x18\xA6\x14a\x01\x84W\x80c\x89\xB0\x8F\x11\x14a\x01\x8CW`\0\x80\xFD[\x80c\x01Vw9\x14a\0\xEDW\x80c\x1ER\xB5\x18\x14a\x012W\x80c%i)b\x14a\x01TW[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[Pa\x01\x1Da\x01\x086`\x04a\txV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01>W`\0\x80\xFD[Pa\x01Ra\x01M6`\x04a\t\x9CV[a\x03\x14V[\0[a\x01Ra\x06\xE2V[a\x01Ra\x072V[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01Ra\x01\x7F6`\x04a\txV[a\x07nV[a\x01Ra\x07\x9AV[4\x80\x15a\x01\x98W`\0\x80\xFD[Pa\x01\xC2a\x01\xA76`\x04a\txV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01)V[4\x80\x15a\x01\xE6W`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x01\xC2V[4\x80\x15a\x01\xFFW`\0\x80\xFD[Pa\x01\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x023W`\0\x80\xFD[Pa\x01\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02gW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x01)V[a\x01Ra\x02\x876`\x04a\txV[a\x07\xAEV[a\x01Ra\x02\x9A6`\x04a\txV[a\x07\xEEV[4\x80\x15a\x02\xABW`\0\x80\xFD[Pa\x01\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xDFW`\0\x80\xFD[Pa\x03\x06a\x02\xEE6`\x04a\txV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[`@Q\x90\x81R` \x01a\x01)V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Rg\x8A\xC7#\x04\x89\xE8\0\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA6\x91\x90a\n6V[\x10\x15a\x03\xC5W`@Qc\x106\xB5\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x843`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x0FW`@Qc\xA8Ax\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04aW`@Qc\xF7>Yg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xB6>\x80\r`\xE0\x1Ba\x04w`\x04`\0\x86\x88a\nOV[a\x04\x80\x91a\nyV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x04\xA8W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0C\x91\x90a\n6V[\x90P`\x01\x81\x14a\x057W`@Qce\x1At\x9B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x9F\x91\x90\x81\x01\x90a\n\xF0V[\x90P`\x01\x81Q\x14a\x05\xC8W\x80Q`@Qc\xBB\xF0M\xC9`\xE0\x1B\x81R`\x04\x01a\x05.\x91\x81R` \x01\x90V[`\0\x81`\0\x81Q\x81\x10a\x05\xDDWa\x05\xDDa\x0B\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R\x91\x82\x90R`@\x90\x91 T\x90\x91P`\xFF\x16a\x06%W`@Qc\x1D\xDDo\x9D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x060\x85a\x08\x15V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x06eW`@Qcr?\xE0\xC1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05.V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U`\x01\x90\x91R\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x87\x16\x91\x90\x91\x17\x90Ua\x06\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86g\x8A\xC7#\x04\x89\xE8\0\0a\x08\xC4V[PPPPPPPPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x07va\t\nV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x07\xA2a\t\nV[a\x07\xAC`\0a\t%V[V[a\x07\xB6a\t\nV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x07\xDEWco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x07\xEB\x81a\t%V[PV[a\x07\xF6a\t\nV[\x80``\x1Ba\x08\x0CWctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x07\xEB\x81a\t%V[`@QcV$\xB2[`\xE0\x1B\x81R\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5`\x04\x82\x01R` `$\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cV$\xB2[\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xAB\x91\x90\x81\x01\x90a\x0B\xB8V[\x80` \x01\x90Q\x81\x01\x90a\x08\xBE\x91\x90a\x0CdV[\x92\x91PPV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\t\0Wc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[c\x8Bx\xC6\xD8\x19T3\x14a\x07\xACWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xEBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\x8AW`\0\x80\xFD[\x815a\t\x95\x81a\tcV[\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\t\xB4W`\0\x80\xFD[\x855a\t\xBF\x81a\tcV[\x94P` \x86\x015a\t\xCF\x81a\tcV[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xECW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n\0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\x0FW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\n!W`\0\x80\xFD[\x96\x99\x95\x98PP` \x01\x95``\x015\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\nHW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x85\x85\x11\x15a\n_W`\0\x80\xFD[\x83\x86\x11\x15a\nlW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\x01`\x01`\xE0\x1B\x03\x19\x815\x81\x81\x16\x91`\x04\x85\x10\x15a\n\xA1W\x80\x81\x86`\x04\x03`\x03\x1B\x1B\x83\x16\x16\x92P[PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xE8Wa\n\xE8a\n\xA9V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0B\x03W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x1BW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B/W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0BAWa\x0BAa\n\xA9V[\x80`\x05\x1B\x91Pa\x0BR\x84\x83\x01a\n\xBFV[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x0BlW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x0B\x96W\x84Q\x92Pa\x0B\x86\x83a\tcV[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x0BqV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x0B\xCBW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xE3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\xF7W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0C\tWa\x0C\ta\n\xA9V[a\x0C\x1B`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\n\xBFV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x0C1W`\0\x80\xFD[`\0[\x81\x81\x10\x15a\x0COW\x83\x81\x01\x85\x01Q\x83\x82\x01\x86\x01R\x84\x01a\x0C4V[P`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0CvW`\0\x80\xFD[\x81Qa\t\x95\x81a\tcV\xFE\xA2dipfsX\"\x12 iV\xFB\xC4\xD1\xA7\x88\xBDt!\xFB\xB7\xF7p\xEE\\\xF4\x02\xB1\xE2A\xC7\x90;>\xA9\x81\x17\xB7\x19\xA2OdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static WALLETREGISTRY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xE8W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x8AW\x80c\xF0N(>\x11a\0YW\x80c\xF0N(>\x14a\x02yW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x8CW\x80c\xFC\x0CTj\x14a\x02\x9FW\x80c\xFE\xE8\x1C\xF4\x14a\x02\xD3W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\xDAW\x80c\xA6\x19Hn\x14a\x01\xF3W\x80c\xC5\xC06\x99\x14a\x02'W\x80c\xD7S?\x02\x14a\x02[W`\0\x80\xFD[\x80cT\xD1\xF1=\x11a\0\xC6W\x80cT\xD1\xF1=\x14a\x01\\W\x80cY&e\x1D\x14a\x01dW\x80cqP\x18\xA6\x14a\x01\x84W\x80c\x89\xB0\x8F\x11\x14a\x01\x8CW`\0\x80\xFD[\x80c\x01Vw9\x14a\0\xEDW\x80c\x1ER\xB5\x18\x14a\x012W\x80c%i)b\x14a\x01TW[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[Pa\x01\x1Da\x01\x086`\x04a\txV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01>W`\0\x80\xFD[Pa\x01Ra\x01M6`\x04a\t\x9CV[a\x03\x14V[\0[a\x01Ra\x06\xE2V[a\x01Ra\x072V[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01Ra\x01\x7F6`\x04a\txV[a\x07nV[a\x01Ra\x07\x9AV[4\x80\x15a\x01\x98W`\0\x80\xFD[Pa\x01\xC2a\x01\xA76`\x04a\txV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01)V[4\x80\x15a\x01\xE6W`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x01\xC2V[4\x80\x15a\x01\xFFW`\0\x80\xFD[Pa\x01\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x023W`\0\x80\xFD[Pa\x01\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02gW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x01)V[a\x01Ra\x02\x876`\x04a\txV[a\x07\xAEV[a\x01Ra\x02\x9A6`\x04a\txV[a\x07\xEEV[4\x80\x15a\x02\xABW`\0\x80\xFD[Pa\x01\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xDFW`\0\x80\xFD[Pa\x03\x06a\x02\xEE6`\x04a\txV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[`@Q\x90\x81R` \x01a\x01)V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Rg\x8A\xC7#\x04\x89\xE8\0\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA6\x91\x90a\n6V[\x10\x15a\x03\xC5W`@Qc\x106\xB5\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x843`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x0FW`@Qc\xA8Ax\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04aW`@Qc\xF7>Yg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xB6>\x80\r`\xE0\x1Ba\x04w`\x04`\0\x86\x88a\nOV[a\x04\x80\x91a\nyV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x04\xA8W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0C\x91\x90a\n6V[\x90P`\x01\x81\x14a\x057W`@Qce\x1At\x9B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x9F\x91\x90\x81\x01\x90a\n\xF0V[\x90P`\x01\x81Q\x14a\x05\xC8W\x80Q`@Qc\xBB\xF0M\xC9`\xE0\x1B\x81R`\x04\x01a\x05.\x91\x81R` \x01\x90V[`\0\x81`\0\x81Q\x81\x10a\x05\xDDWa\x05\xDDa\x0B\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R\x91\x82\x90R`@\x90\x91 T\x90\x91P`\xFF\x16a\x06%W`@Qc\x1D\xDDo\x9D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x060\x85a\x08\x15V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x06eW`@Qcr?\xE0\xC1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05.V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U`\x01\x90\x91R\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x87\x16\x91\x90\x91\x17\x90Ua\x06\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86g\x8A\xC7#\x04\x89\xE8\0\0a\x08\xC4V[PPPPPPPPPPV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[a\x07va\t\nV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x07\xA2a\t\nV[a\x07\xAC`\0a\t%V[V[a\x07\xB6a\t\nV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x07\xDEWco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x07\xEB\x81a\t%V[PV[a\x07\xF6a\t\nV[\x80``\x1Ba\x08\x0CWctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x07\xEB\x81a\t%V[`@QcV$\xB2[`\xE0\x1B\x81R\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5`\x04\x82\x01R` `$\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cV$\xB2[\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xAB\x91\x90\x81\x01\x90a\x0B\xB8V[\x80` \x01\x90Q\x81\x01\x90a\x08\xBE\x91\x90a\x0CdV[\x92\x91PPV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\t\0Wc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[c\x8Bx\xC6\xD8\x19T3\x14a\x07\xACWc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xEBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\x8AW`\0\x80\xFD[\x815a\t\x95\x81a\tcV[\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\t\xB4W`\0\x80\xFD[\x855a\t\xBF\x81a\tcV[\x94P` \x86\x015a\t\xCF\x81a\tcV[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xECW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n\0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\x0FW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\n!W`\0\x80\xFD[\x96\x99\x95\x98PP` \x01\x95``\x015\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\nHW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x85\x85\x11\x15a\n_W`\0\x80\xFD[\x83\x86\x11\x15a\nlW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\x01`\x01`\xE0\x1B\x03\x19\x815\x81\x81\x16\x91`\x04\x85\x10\x15a\n\xA1W\x80\x81\x86`\x04\x03`\x03\x1B\x1B\x83\x16\x16\x92P[PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xE8Wa\n\xE8a\n\xA9V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0B\x03W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x1BW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B/W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0BAWa\x0BAa\n\xA9V[\x80`\x05\x1B\x91Pa\x0BR\x84\x83\x01a\n\xBFV[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x0BlW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x0B\x96W\x84Q\x92Pa\x0B\x86\x83a\tcV[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x0BqV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x0B\xCBW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xE3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\xF7W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0C\tWa\x0C\ta\n\xA9V[a\x0C\x1B`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\n\xBFV[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x0C1W`\0\x80\xFD[`\0[\x81\x81\x10\x15a\x0COW\x83\x81\x01\x85\x01Q\x83\x82\x01\x86\x01R\x84\x01a\x0C4V[P`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0CvW`\0\x80\xFD[\x81Qa\t\x95\x81a\tcV\xFE\xA2dipfsX\"\x12 iV\xFB\xC4\xD1\xA7\x88\xBDt!\xFB\xB7\xF7p\xEE\\\xF4\x02\xB1\xE2A\xC7\x90;>\xA9\x81\x17\xB7\x19\xA2OdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static WALLETREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct WalletRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WalletRegistry<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for WalletRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for WalletRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for WalletRegistry<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WalletRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WalletRegistry<M> {
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
                WALLETREGISTRY_ABI.clone(),
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
                WALLETREGISTRY_ABI.clone(),
                WALLETREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addBeneficiary`
        /// (0x5926651d) function
        pub fn add_beneficiary(
            &self,
            beneficiary: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 38, 101, 29], beneficiary)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beneficiaries`
        /// (0x01567739) function
        pub fn beneficiaries(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 86, 119, 57], p0)
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
        ///Calls the contract's `masterCopy` (0xa619486e)
        /// function
        pub fn master_copy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([166, 25, 72, 110], ())
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
        ///Calls the contract's `proxyCreated` (0x1e52b518)
        /// function
        pub fn proxy_created(
            &self,
            proxy: ::ethers::core::types::Address,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 82, 181, 24],
                    (proxy, singleton, initializer, p3),
                )
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
        ///Calls the contract's `requestOwnershipHandover`
        /// (0x25692962) function
        pub fn request_ownership_handover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 105, 41, 98], ())
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
        ///Calls the contract's `walletFactory`
        /// (0xc5c03699) function
        pub fn wallet_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 192, 54, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wallets` (0x89b08f11)
        /// function
        pub fn wallets(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([137, 176, 143, 17], p0)
                .expect("method not found (this should never happen)")
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
        /// Returns an `Event` builder for all the events of
        /// this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WalletRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for WalletRegistry<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerNotFactory` with signature
    /// `CallerNotFactory()` and selector `0xa84178ab`
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
    #[etherror(name = "CallerNotFactory", abi = "CallerNotFactory()")]
    pub struct CallerNotFactory;
    ///Custom Error type `FakeMasterCopy` with signature
    /// `FakeMasterCopy()` and selector `0xf73e5967`
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
    #[etherror(name = "FakeMasterCopy", abi = "FakeMasterCopy()")]
    pub struct FakeMasterCopy;
    ///Custom Error type `InvalidFallbackManager` with
    /// signature `InvalidFallbackManager(address)` and
    /// selector `0xe47fc182`
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
    #[etherror(
        name = "InvalidFallbackManager",
        abi = "InvalidFallbackManager(address)"
    )]
    pub struct InvalidFallbackManager {
        pub fallback_manager: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidInitialization` with
    /// signature `InvalidInitialization()` and selector
    /// `0xf92ee8a9`
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
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `InvalidOwnersCount` with
    /// signature `InvalidOwnersCount(uint256)` and selector
    /// `0xbbf04dc9`
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
    #[etherror(
        name = "InvalidOwnersCount",
        abi = "InvalidOwnersCount(uint256)"
    )]
    pub struct InvalidOwnersCount {
        pub count: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidThreshold` with signature
    /// `InvalidThreshold(uint256)` and selector
    /// `0x651a749b`
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
    #[etherror(name = "InvalidThreshold", abi = "InvalidThreshold(uint256)")]
    pub struct InvalidThreshold {
        pub threshold: ::ethers::core::types::U256,
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
    ///Custom Error type `NotEnoughFunds` with signature
    /// `NotEnoughFunds()` and selector `0x81b5ad68`
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
    #[etherror(name = "NotEnoughFunds", abi = "NotEnoughFunds()")]
    pub struct NotEnoughFunds;
    ///Custom Error type `OwnerIsNotABeneficiary` with
    /// signature `OwnerIsNotABeneficiary()` and selector
    /// `0xeeeb7ce8`
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
    #[etherror(
        name = "OwnerIsNotABeneficiary",
        abi = "OwnerIsNotABeneficiary()"
    )]
    pub struct OwnerIsNotABeneficiary;
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
    pub enum WalletRegistryErrors {
        CallerNotFactory(CallerNotFactory),
        FakeMasterCopy(FakeMasterCopy),
        InvalidFallbackManager(InvalidFallbackManager),
        InvalidInitialization(InvalidInitialization),
        InvalidOwnersCount(InvalidOwnersCount),
        InvalidThreshold(InvalidThreshold),
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        NoHandoverRequest(NoHandoverRequest),
        NotEnoughFunds(NotEnoughFunds),
        OwnerIsNotABeneficiary(OwnerIsNotABeneficiary),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for WalletRegistryErrors {
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
                <CallerNotFactory as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CallerNotFactory(decoded));
            }
            if let Ok(decoded) =
                <FakeMasterCopy as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FakeMasterCopy(decoded));
            }
            if let Ok(decoded)
                = <InvalidFallbackManager as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidFallbackManager(decoded));
            }
            if let Ok(decoded)
                = <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) =
                <InvalidOwnersCount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InvalidOwnersCount(decoded));
            }
            if let Ok(decoded) =
                <InvalidThreshold as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InvalidThreshold(decoded));
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
                <NotEnoughFunds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotEnoughFunds(decoded));
            }
            if let Ok(decoded)
                = <OwnerIsNotABeneficiary as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OwnerIsNotABeneficiary(decoded));
            }
            if let Ok(decoded) =
                <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WalletRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerNotFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FakeMasterCopy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFallbackManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOwnersCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewOwnerIsZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoHandoverRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnerIsNotABeneficiary(element) => {
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
    impl ::ethers::contract::ContractRevert for WalletRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerNotFactory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FakeMasterCopy as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFallbackManager as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOwnersCount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewOwnerIsZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoHandoverRequest as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughFunds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnerIsNotABeneficiary as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for WalletRegistryErrors {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::CallerNotFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FakeMasterCopy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidFallbackManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInitialization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOwnersCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewOwnerIsZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoHandoverRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughFunds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerIsNotABeneficiary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for WalletRegistryErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<CallerNotFactory> for WalletRegistryErrors {
        fn from(value: CallerNotFactory) -> Self {
            Self::CallerNotFactory(value)
        }
    }
    impl ::core::convert::From<FakeMasterCopy> for WalletRegistryErrors {
        fn from(value: FakeMasterCopy) -> Self { Self::FakeMasterCopy(value) }
    }
    impl ::core::convert::From<InvalidFallbackManager> for WalletRegistryErrors {
        fn from(value: InvalidFallbackManager) -> Self {
            Self::InvalidFallbackManager(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for WalletRegistryErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<InvalidOwnersCount> for WalletRegistryErrors {
        fn from(value: InvalidOwnersCount) -> Self {
            Self::InvalidOwnersCount(value)
        }
    }
    impl ::core::convert::From<InvalidThreshold> for WalletRegistryErrors {
        fn from(value: InvalidThreshold) -> Self {
            Self::InvalidThreshold(value)
        }
    }
    impl ::core::convert::From<NewOwnerIsZeroAddress> for WalletRegistryErrors {
        fn from(value: NewOwnerIsZeroAddress) -> Self {
            Self::NewOwnerIsZeroAddress(value)
        }
    }
    impl ::core::convert::From<NoHandoverRequest> for WalletRegistryErrors {
        fn from(value: NoHandoverRequest) -> Self {
            Self::NoHandoverRequest(value)
        }
    }
    impl ::core::convert::From<NotEnoughFunds> for WalletRegistryErrors {
        fn from(value: NotEnoughFunds) -> Self { Self::NotEnoughFunds(value) }
    }
    impl ::core::convert::From<OwnerIsNotABeneficiary> for WalletRegistryErrors {
        fn from(value: OwnerIsNotABeneficiary) -> Self {
            Self::OwnerIsNotABeneficiary(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for WalletRegistryErrors {
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
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum WalletRegistryEvents {
        OwnershipHandoverCanceledFilter(OwnershipHandoverCanceledFilter),
        OwnershipHandoverRequestedFilter(OwnershipHandoverRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for WalletRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) =
                OwnershipHandoverCanceledFilter::decode_log(log)
            {
                return Ok(
                    WalletRegistryEvents::OwnershipHandoverCanceledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                OwnershipHandoverRequestedFilter::decode_log(log)
            {
                return Ok(
                    WalletRegistryEvents::OwnershipHandoverRequestedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(WalletRegistryEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WalletRegistryEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::OwnershipHandoverCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipHandoverRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipHandoverCanceledFilter>
        for WalletRegistryEvents
    {
        fn from(value: OwnershipHandoverCanceledFilter) -> Self {
            Self::OwnershipHandoverCanceledFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverRequestedFilter>
        for WalletRegistryEvents
    {
        fn from(value: OwnershipHandoverRequestedFilter) -> Self {
            Self::OwnershipHandoverRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
        for WalletRegistryEvents
    {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the
    /// `addBeneficiary` function with signature
    /// `addBeneficiary(address)` and selector `0x5926651d`
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
    #[ethcall(name = "addBeneficiary", abi = "addBeneficiary(address)")]
    pub struct AddBeneficiaryCall {
        pub beneficiary: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `beneficiaries` function with signature
    /// `beneficiaries(address)` and selector `0x01567739`
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
    #[ethcall(name = "beneficiaries", abi = "beneficiaries(address)")]
    pub struct BeneficiariesCall(pub ::ethers::core::types::Address);
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
    /// `masterCopy` function with signature `masterCopy()`
    /// and selector `0xa619486e`
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
    #[ethcall(name = "masterCopy", abi = "masterCopy()")]
    pub struct MasterCopyCall;
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
    /// `proxyCreated` function with signature
    /// `proxyCreated(address,address,bytes,uint256)` and
    /// selector `0x1e52b518`
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
        name = "proxyCreated",
        abi = "proxyCreated(address,address,bytes,uint256)"
    )]
    pub struct ProxyCreatedCall {
        pub proxy: ::ethers::core::types::Address,
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub p3: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the
    /// `walletFactory` function with signature
    /// `walletFactory()` and selector `0xc5c03699`
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
    #[ethcall(name = "walletFactory", abi = "walletFactory()")]
    pub struct WalletFactoryCall;
    ///Container type for all input parameters for the
    /// `wallets` function with signature `wallets(address)`
    /// and selector `0x89b08f11`
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
    #[ethcall(name = "wallets", abi = "wallets(address)")]
    pub struct WalletsCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum WalletRegistryCalls {
        AddBeneficiary(AddBeneficiaryCall),
        Beneficiaries(BeneficiariesCall),
        CancelOwnershipHandover(CancelOwnershipHandoverCall),
        CompleteOwnershipHandover(CompleteOwnershipHandoverCall),
        MasterCopy(MasterCopyCall),
        Owner(OwnerCall),
        OwnershipHandoverExpiresAt(OwnershipHandoverExpiresAtCall),
        OwnershipHandoverValidFor(OwnershipHandoverValidForCall),
        ProxyCreated(ProxyCreatedCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequestOwnershipHandover(RequestOwnershipHandoverCall),
        Token(TokenCall),
        TransferOwnership(TransferOwnershipCall),
        WalletFactory(WalletFactoryCall),
        Wallets(WalletsCall),
    }
    impl ::ethers::core::abi::AbiDecode for WalletRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AddBeneficiaryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AddBeneficiary(decoded));
            }
            if let Ok(decoded) =
                <BeneficiariesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::Beneficiaries(decoded));
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
                <MasterCopyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MasterCopy(decoded));
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
            if let Ok(decoded) =
                <ProxyCreatedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ProxyCreated(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <RequestOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RequestOwnershipHandover(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WalletFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::WalletFactory(decoded));
            }
            if let Ok(decoded) =
                <WalletsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Wallets(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WalletRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddBeneficiary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Beneficiaries(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MasterCopy(element) => {
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
                Self::ProxyCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WalletFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Wallets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for WalletRegistryCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::AddBeneficiary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Beneficiaries(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MasterCopy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipHandoverExpiresAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipHandoverValidFor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProxyCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WalletFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Wallets(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddBeneficiaryCall> for WalletRegistryCalls {
        fn from(value: AddBeneficiaryCall) -> Self {
            Self::AddBeneficiary(value)
        }
    }
    impl ::core::convert::From<BeneficiariesCall> for WalletRegistryCalls {
        fn from(value: BeneficiariesCall) -> Self { Self::Beneficiaries(value) }
    }
    impl ::core::convert::From<CancelOwnershipHandoverCall>
        for WalletRegistryCalls
    {
        fn from(value: CancelOwnershipHandoverCall) -> Self {
            Self::CancelOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<CompleteOwnershipHandoverCall>
        for WalletRegistryCalls
    {
        fn from(value: CompleteOwnershipHandoverCall) -> Self {
            Self::CompleteOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<MasterCopyCall> for WalletRegistryCalls {
        fn from(value: MasterCopyCall) -> Self { Self::MasterCopy(value) }
    }
    impl ::core::convert::From<OwnerCall> for WalletRegistryCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<OwnershipHandoverExpiresAtCall>
        for WalletRegistryCalls
    {
        fn from(value: OwnershipHandoverExpiresAtCall) -> Self {
            Self::OwnershipHandoverExpiresAt(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverValidForCall>
        for WalletRegistryCalls
    {
        fn from(value: OwnershipHandoverValidForCall) -> Self {
            Self::OwnershipHandoverValidFor(value)
        }
    }
    impl ::core::convert::From<ProxyCreatedCall> for WalletRegistryCalls {
        fn from(value: ProxyCreatedCall) -> Self { Self::ProxyCreated(value) }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for WalletRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RequestOwnershipHandoverCall>
        for WalletRegistryCalls
    {
        fn from(value: RequestOwnershipHandoverCall) -> Self {
            Self::RequestOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<TokenCall> for WalletRegistryCalls {
        fn from(value: TokenCall) -> Self { Self::Token(value) }
    }
    impl ::core::convert::From<TransferOwnershipCall> for WalletRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WalletFactoryCall> for WalletRegistryCalls {
        fn from(value: WalletFactoryCall) -> Self { Self::WalletFactory(value) }
    }
    impl ::core::convert::From<WalletsCall> for WalletRegistryCalls {
        fn from(value: WalletsCall) -> Self { Self::Wallets(value) }
    }
    ///Container type for all return fields from the
    /// `beneficiaries` function with signature
    /// `beneficiaries(address)` and selector `0x01567739`
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
    pub struct BeneficiariesReturn(pub bool);
    ///Container type for all return fields from the
    /// `masterCopy` function with signature `masterCopy()`
    /// and selector `0xa619486e`
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
    pub struct MasterCopyReturn(pub ::ethers::core::types::Address);
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
    /// `walletFactory` function with signature
    /// `walletFactory()` and selector `0xc5c03699`
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
    pub struct WalletFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `wallets` function with signature `wallets(address)`
    /// and selector `0x89b08f11`
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
    pub struct WalletsReturn(pub ::ethers::core::types::Address);
}
