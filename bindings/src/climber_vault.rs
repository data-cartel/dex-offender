pub use climber_vault::*;
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
pub mod climber_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getLastWithdrawalTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLastWithdrawalTimestamp",
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
                    ::std::borrow::ToOwned::to_owned("getSweeper"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSweeper"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sweeper"),
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
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
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
                    ::std::borrow::ToOwned::to_owned("sweepFunds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepFunds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                (
                    ::std::borrow::ToOwned::to_owned("upgradeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
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
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("beacon"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
                    ::std::borrow::ToOwned::to_owned("CallerNotSweeper"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallerNotSweeper"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWithdrawalAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidWithdrawalAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWithdrawalTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidWithdrawalTime",
                            ),
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
    pub static CLIMBERVAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[Pa\0\x1Da\0\"V[a\0\xE1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\0\xDFW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x80Qa&\xB2a\x01\x18`\09`\0\x81\x81a\x02\xE1\x01R\x81\x81a\x03-\x01R\x81\x81a\x03\xD2\x01R\x81\x81a\x04\x15\x01Ra\x04\xB1\x01Ra&\xB2`\0\xF3\xFE`\x80`@R`\x046\x10b\0\0\xA9W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11b\0\0lW\x80cqP\x18\xA6\x14b\0\x01NW\x80c\x8D\xA5\xCB[\x14b\0\x01fW\x80c\xC0\xC5;\x8B\x14b\0\x01\x9AW\x80c\xD9\xCA\xED\x12\x14b\0\x01\xBFW\x80c\xDE\xB0\xF0p\x14b\0\x01\xE4W\x80c\xF2\xFD\xE3\x8B\x14b\0\x02\x04W`\0\x80\xFD[\x80c\x0F\xE2\x89\x08\x14b\0\0\xAEW\x80c&m\xF7\x82\x14b\0\0\xD5W\x80c6Y\xCF\xE6\x14b\0\0\xFAW\x80cO\x1E\xF2\x86\x14b\0\x01\x1FW\x80cR\xD1\x90-\x14b\0\x016W[`\0\x80\xFD[4\x80\x15b\0\0\xBBW`\0\x80\xFD[Pb\0\0\xD3b\0\0\xCD6`\x04b\0\r\xB1V[b\0\x02)V[\0[4\x80\x15b\0\0\xE2W`\0\x80\xFD[P`\xC9T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15b\0\x01\x07W`\0\x80\xFD[Pb\0\0\xD3b\0\x01\x196`\x04b\0\r\xB1V[b\0\x02\xD7V[b\0\0\xD3b\0\x0106`\x04b\0\r\xE5V[b\0\x03\xC8V[4\x80\x15b\0\x01CW`\0\x80\xFD[Pb\0\0\xE7b\0\x04\xA4V[4\x80\x15b\0\x01[W`\0\x80\xFD[Pb\0\0\xD3b\0\x05ZV[4\x80\x15b\0\x01sW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01b\0\0\xF1V[4\x80\x15b\0\x01\xA7W`\0\x80\xFD[Pb\0\0\xD3b\0\x01\xB96`\x04b\0\x0E\xB3V[b\0\x05rV[4\x80\x15b\0\x01\xCCW`\0\x80\xFD[Pb\0\0\xD3b\0\x01\xDE6`\x04b\0\x0E\xFDV[b\0\x07\rV[4\x80\x15b\0\x01\xF1W`\0\x80\xFD[P`\xCAT`\x01`\x01`\xA0\x1B\x03\x16b\0\x01\x81V[4\x80\x15b\0\x02\x11W`\0\x80\xFD[Pb\0\0\xD3b\0\x02#6`\x04b\0\r\xB1V[b\0\x07\x90V[`\xCAT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x02UW`@Qc\xF9\x08a\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xCAT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Rb\0\x02\xD4\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x02\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02\xCE\x91\x90b\0\x0F>V[b\0\x08\x0CV[PV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03b\0\x03+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0FXV[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16b\0\x03v`\0\x80Q` b\0&6\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\x03\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0F\xA4V[b\0\x03\xAA\x81b\0\x08SV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Rb\0\x02\xD4\x91\x83\x91\x90b\0\x08]V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03b\0\x04\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0FXV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16b\0\x04^`\0\x80Q` b\0&6\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\x04\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0F\xA4V[b\0\x04\x92\x82b\0\x08SV[b\0\x04\xA0\x82\x82`\x01b\0\x08]V[PPV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14b\0\x05FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x03\"V[P`\0\x80Q` b\0&6\x839\x81Q\x91R\x90V[b\0\x05db\0\t\xD5V[b\0\x05p`\0b\0\n1V[V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x05\x93WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x05\xAFWP0;\x15\x80\x15b\0\x05\xAFWP`\0T`\xFF\x16`\x01\x14[b\0\x06\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x068W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x06Bb\0\n\x83V[b\0\x06Lb\0\n\xB7V[b\0\x06\x9B\x84\x84`@Qb\0\x06`\x90b\0\r\x86V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x06\x94W=`\0\x80>=`\0\xFD[Pb\0\x07\x90V[`\xCA\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ub\0\x06\xC0B`\xC9UV[\x80\x15b\0\x07\x07W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[b\0\x07\x17b\0\t\xD5V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15b\0\x07AW`@Qc\x9A\xBCt\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x13\xC6\x80`\xC9Tb\0\x07T\x91\x90b\0\x0F\xF0V[B\x11b\0\x07tW`@Qc \xDB\xD7\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\0\x07~B`\xC9UV[b\0\x07\x8B\x83\x83\x83b\0\x08\x0CV[PPPV[b\0\x07\x9Ab\0\t\xD5V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[b\0\x02\xD4\x81b\0\n1V[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16b\0\x08IWc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[b\0\x02\xD4b\0\t\xD5V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15b\0\x08\x93Wb\0\x07\x8B\x83b\0\n\xE1V[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15b\0\x08\xF0WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Rb\0\x08\xED\x91\x81\x01\x90b\0\x0F>V[`\x01[b\0\tUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[`\0\x80Q` b\0&6\x839\x81Q\x91R\x81\x14b\0\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[Pb\0\x07\x8B\x83\x83\x83b\0\x0B\x80V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x03\"V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\n\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x10\x12V[b\0\x05pb\0\x0B\xABV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x05pW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x10\x12V[`\x01`\x01`\xA0\x1B\x03\x81\x16;b\0\x0BPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[`\0\x80Q` b\0&6\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[b\0\x0B\x8B\x83b\0\x0B\xE0V[`\0\x82Q\x11\x80b\0\x0B\x99WP\x80[\x15b\0\x07\x8BWb\0\x07\x07\x83\x83b\0\x0C\"V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x0B\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x10\x12V[b\0\x05p3b\0\n1V[b\0\x0B\xEB\x81b\0\n\xE1V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``b\0\x0CJ\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01b\0&V`'\x919b\0\x0CSV[\x90P[\x92\x91PPV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qb\0\x0Cr\x91\x90b\0\x10\x83V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14b\0\x0C\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x0C\xB4V[``\x91P[P\x91P\x91Pb\0\x0C\xC7\x86\x83\x83\x87b\0\x0C\xD1V[\x96\x95PPPPPPV[``\x83\x15b\0\rEW\x82Q`\0\x03b\0\r=W`\x01`\x01`\xA0\x1B\x03\x85\x16;b\0\r=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01b\0\x03\"V[P\x81b\0\rQV[b\0\rQ\x83\x83b\0\rYV[\x94\x93PPPPV[\x81Q\x15b\0\rjW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x91\x90b\0\x10\xA1V[a\x15_\x80b\0\x10\xD7\x839\x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\r\xACW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\r\xC4W`\0\x80\xFD[b\0\x0CJ\x82b\0\r\x94V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0\r\xF9W`\0\x80\xFD[b\0\x0E\x04\x83b\0\r\x94V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x0E\"W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x0E7W`\0\x80\xFD[\x815\x81\x81\x11\x15b\0\x0ELWb\0\x0ELb\0\r\xCFV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x0EwWb\0\x0Ewb\0\r\xCFV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15b\0\x0E\x91W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x0E\xC9W`\0\x80\xFD[b\0\x0E\xD4\x84b\0\r\x94V[\x92Pb\0\x0E\xE4` \x85\x01b\0\r\x94V[\x91Pb\0\x0E\xF4`@\x85\x01b\0\r\x94V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x0F\x13W`\0\x80\xFD[b\0\x0F\x1E\x84b\0\r\x94V[\x92Pb\0\x0F.` \x85\x01b\0\r\x94V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x0FQW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15b\0\x0CMWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`\0[\x83\x81\x10\x15b\0\x10zW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x10`V[PP`\0\x91\x01RV[`\0\x82Qb\0\x10\x97\x81\x84` \x87\x01b\0\x10]V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Rb\0\x10\xC2\x81`@\x85\x01` \x87\x01b\0\x10]V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x15_8\x03\x80b\0\x15_\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xFEV[b\0\0O`\0\x80Q` b\0\x15?\x839\x81Q\x91R\x80b\0\0\xE6V[b\0\0y`\0\x80Q` b\0\x15\x1F\x839\x81Q\x91R`\0\x80Q` b\0\x15?\x839\x81Q\x91Rb\0\0\xE6V[b\0\0\x94`\0\x80Q` b\0\x15?\x839\x81Q\x91R\x83b\0\x011V[b\0\0\xAF`\0\x80Q` b\0\x15?\x839\x81Q\x91R0b\0\x011V[b\0\0\xCA`\0\x80Q` b\0\x15\x1F\x839\x81Q\x91R\x82b\0\x011V[PP`\x02\x80T`\x01`\x01`@\x1B\x03\x19\x16a\x0E\x10\x17\x90Ub\0\x026V[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[b\0\x01=\x82\x82b\0\x01AV[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x01=W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01\x9D3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x02\x12W`\0\x80\xFD[b\0\x02\x1D\x83b\0\x01\xE1V[\x91Pb\0\x02-` \x84\x01b\0\x01\xE1V[\x90P\x92P\x92\x90PV[a\x12\xD9\x80b\0\x02F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xE1W`\x005`\xE0\x1C\x80cjB\xB8\xF8\x11a\0\x7FW\x80c\x91\xD1HT\x11a\0YW\x80c\x91\xD1HT\x14a\x02zW\x80c\xA2\x17\xFD\xDF\x14a\x02\x9AW\x80c\xC7O4\x9B\x14a\x02\xAFW\x80c\xD5Gt\x1F\x14a\x03\"W`\0\x80\xFD[\x80cjB\xB8\xF8\x14a\x01\xF5W\x80cyX\0L\x14a\x02-W\x80c\x90\xBD\x1Em\x14a\x02ZW`\0\x80\xFD[\x80c&V\"}\x11a\0\xBBW\x80c&V\"}\x14a\x01\x82W\x80c//\xF1]\x14a\x01\x95W\x80c6V\x8A\xBE\x14a\x01\xB5W\x80cW\xF5%\xED\x14a\x01\xD5W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xEDW\x80c$\x8A\x9C\xA3\x14a\x01\"W\x80c$\xAD\xBC[\x14a\x01`W`\0\x80\xFD[6a\0\xE8W\0[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[Pa\x01\ra\x01\x086`\x04a\rPV[a\x03BV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01.W`\0\x80\xFD[Pa\x01Ra\x01=6`\x04a\rzV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x19V[4\x80\x15a\x01lW`\0\x80\xFD[Pa\x01\x80a\x01{6`\x04a\r\x93V[a\x03yV[\0[a\x01\x80a\x01\x906`\x04a\x0E\x07V[a\x03\xE9V[4\x80\x15a\x01\xA1W`\0\x80\xFD[Pa\x01\x80a\x01\xB06`\x04a\x0E\xC5V[a\x05\x98V[4\x80\x15a\x01\xC1W`\0\x80\xFD[Pa\x01\x80a\x01\xD06`\x04a\x0E\xC5V[a\x05\xC2V[4\x80\x15a\x01\xE1W`\0\x80\xFD[Pa\x01Ra\x01\xF06`\x04a\x0E\x07V[a\x06@V[4\x80\x15a\x02\x01W`\0\x80\xFD[P`\x02Ta\x02\x15\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[4\x80\x15a\x029W`\0\x80\xFD[Pa\x02Ma\x02H6`\x04a\rzV[a\x06\x82V[`@Qa\x01\x19\x91\x90a\x0F\x07V[4\x80\x15a\x02fW`\0\x80\xFD[Pa\x01\x80a\x02u6`\x04a\x0E\x07V[a\x07\x17V[4\x80\x15a\x02\x86W`\0\x80\xFD[Pa\x01\ra\x02\x956`\x04a\x0E\xC5V[a\x08SV[4\x80\x15a\x02\xA6W`\0\x80\xFD[Pa\x01R`\0\x81V[4\x80\x15a\x02\xBBW`\0\x80\xFD[Pa\x02\xFBa\x02\xCA6`\x04a\rzV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x81\x16\x90`\xFF`\x01`@\x1B\x82\x04\x81\x16\x91`\x01`H\x1B\x90\x04\x16\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R\x91\x15\x15` \x84\x01R\x15\x15\x90\x82\x01R``\x01a\x01\x19V[4\x80\x15a\x03.W`\0\x80\xFD[Pa\x01\x80a\x03=6`\x04a\x0E\xC5V[a\x08|V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03sWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[30\x14a\x03\x99W`@Qc\xDF\xB4\x9E1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x12u\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x03\xC6W`@Qc\x1E=\t1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x85a\x04\x07W`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14a\x04'W`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x82\x14a\x04GW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x04X\x88\x88\x88\x88\x88\x88\x88a\x06@V[\x90P`\0[`\xFF\x81\x16\x88\x11\x15a\x05'Wa\x05\x1E\x85\x85\x83`\xFF\x16\x81\x81\x10a\x04\x80Wa\x04\x80a\x0F/V[\x90P` \x02\x81\x01\x90a\x04\x92\x91\x90a\x0FEV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92P\x8A\x91PP`\xFF\x85\x16\x81\x81\x10a\x04\xDDWa\x04\xDDa\x0F/V[\x90P` \x02\x015\x8B\x8B\x85`\xFF\x16\x81\x81\x10a\x04\xF9Wa\x04\xF9a\x0F/V[\x90P` \x02\x01` \x81\x01\x90a\x05\x0E\x91\x90a\x0F\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x08\xA1V[P`\x01\x01a\x04]V[P`\x02a\x053\x82a\x06\x82V[`\x03\x81\x11\x15a\x05DWa\x05Da\x0E\xF1V[\x14a\x05jW`@Qc\x08)_\xC9`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` R`@\x90 \x80Ti\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1B\x17\x90UPPPPPPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\xB3\x81a\x08\xCFV[a\x05\xBD\x83\x83a\x08\xDCV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x062W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05aV[a\x06<\x82\x82a\t`V[PPV[`\0\x87\x87\x87\x87\x87\x87\x87`@Q` \x01a\x06_\x97\x96\x95\x94\x93\x92\x91\x90a\x10`V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\xFF`\x01`@\x1B\x82\x04\x81\x16\x15\x80\x15\x95\x84\x01\x95\x90\x95R`\x01`H\x1B\x90\x91\x04\x16\x15\x15\x91\x81\x01\x91\x90\x91R\x90a\x07\x0CW\x80`@\x01Q\x15a\x06\xE8W`\x03\x91Pa\x07\x11V[\x80Q`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x07\x03W`\x01\x91Pa\x07\x11V[`\x02\x91Pa\x07\x11V[`\0\x91P[P\x91\x90PV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\x07A\x81a\x08\xCFV[\x86\x15\x80a\x07PWPa\x01\0\x87\x10\x15[\x15a\x07nW`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x85\x14a\x07\x8EW`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x83\x14a\x07\xAEW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xBF\x89\x89\x89\x89\x89\x89\x89a\x06@V[\x90P`\0a\x07\xCC\x82a\x06\x82V[`\x03\x81\x11\x15a\x07\xDDWa\x07\xDDa\x0E\xF1V[\x14a\x07\xFEW`@Qc \xB1\x99\xD1`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05aV[`\x02Ta\x08\x14\x90`\x01`\x01`@\x1B\x03\x16Ba\x11\x14V[`\0\x91\x82R`\x01` R`@\x90\x91 \x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17`\x01`@\x1B\x17\x90UPPPPPPPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x97\x81a\x08\xCFV[a\x05\xBD\x83\x83a\t`V[``a\x08\xC7\x84\x84\x84`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x12{`)\x919a\t\xC5V[\x94\x93PPPPV[a\x08\xD9\x813a\n\xA0V[PV[a\x08\xE6\x82\x82a\x08SV[a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\x1C3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\tj\x82\x82a\x08SV[\x15a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``\x82G\x10\x15a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05aV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\nB\x91\x90a\x11_V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\x7FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x84V[``\x91P[P\x91P\x91Pa\n\x95\x87\x83\x83\x87a\n\xF9V[\x97\x96PPPPPPPV[a\n\xAA\x82\x82a\x08SV[a\x06<Wa\n\xB7\x81a\x0BrV[a\n\xC2\x83` a\x0B\x84V[`@Q` \x01a\n\xD3\x92\x91\x90a\x11{V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05a\x91`\x04\x01a\x11\xF0V[``\x83\x15a\x0BhW\x82Q`\0\x03a\x0BaW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x0BaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05aV[P\x81a\x08\xC7V[a\x08\xC7\x83\x83a\r&V[``a\x03s`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B\x93\x83`\x02a\x12#V[a\x0B\x9E\x90`\x02a\x12:V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xB5Wa\x0B\xB5a\x12MV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xDFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\xFAWa\x0B\xFAa\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0C)Wa\x0C)a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0CM\x84`\x02a\x12#V[a\x0CX\x90`\x01a\x12:V[\x90P[`\x01\x81\x11\x15a\x0C\xD0Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0C\x8CWa\x0C\x8Ca\x0F/V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0C\xA2Wa\x0C\xA2a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0C\xC9\x81a\x12cV[\x90Pa\x0C[V[P\x83\x15a\r\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05aV[\x93\x92PPPV[\x81Q\x15a\r6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05a\x91\x90a\x11\xF0V[`\0` \x82\x84\x03\x12\x15a\rbW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\r\xCEW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xE5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0E\0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x0E\"W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E9W`\0\x80\xFD[a\x0EE\x8B\x83\x8C\x01a\r\xBCV[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a\x0E^W`\0\x80\xFD[a\x0Ej\x8B\x83\x8C\x01a\r\xBCV[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a\x0E\x83W`\0\x80\xFD[Pa\x0E\x90\x8A\x82\x8B\x01a\r\xBCV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xC0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xD8W`\0\x80\xFD[\x825\x91Pa\x0E\xE8` \x84\x01a\x0E\xA9V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10a\x0F)WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0F\\W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0FvW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0E\0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\x9DW`\0\x80\xFD[a\r\x1F\x82a\x0E\xA9V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0[\x87\x81\x10\x15a\x10SW\x82\x84\x03\x89R\x815`\x1E\x19\x886\x03\x01\x81\x12a\x10\nW`\0\x80\xFD[\x87\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10%W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x104W`\0\x80\xFD[a\x10?\x86\x82\x84a\x0F\xA6V[\x9A\x87\x01\x9A\x95PPP\x90\x84\x01\x90`\x01\x01a\x0F\xE9V[P\x91\x97\x96PPPPPPPV[`\x80\x80\x82R\x81\x01\x87\x90R`\0\x88`\xA0\x83\x01\x82[\x8A\x81\x10\x15a\x10\xA1W`\x01`\x01`\xA0\x1B\x03a\x10\x8C\x84a\x0E\xA9V[\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x10sV[P\x83\x81\x03` \x85\x01R\x87\x81R`\x01`\x01`\xFB\x1B\x03\x88\x11\x15a\x10\xC1W`\0\x80\xFD[\x87`\x05\x1B\x91P\x81\x89` \x83\x017\x01\x82\x81\x03` \x90\x81\x01`@\x85\x01Ra\x10\xE9\x90\x82\x01\x86\x88a\x0F\xCFV[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x114Wa\x114a\x10\xFEV[P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11VW\x81\x81\x01Q\x83\x82\x01R` \x01a\x11>V[PP`\0\x91\x01RV[`\0\x82Qa\x11q\x81\x84` \x87\x01a\x11;V[\x91\x90\x91\x01\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x11\xB3\x81`\x17\x85\x01` \x88\x01a\x11;V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x11\xE4\x81`(\x84\x01` \x88\x01a\x11;V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x12\x0F\x81`@\x85\x01` \x87\x01a\x11;V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03sWa\x03sa\x10\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x03sWa\x03sa\x10\xFEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x81a\x12rWa\x12ra\x10\xFEV[P`\0\x19\x01\x90V\xFEAddress: low-level call with value failed\xA2dipfsX\"\x12 8\x83\xA5B\xBF\xF1L\xD6\xE4x\xE5\x99\xFB\xB1'(\xFDj\x1C\xB5\xC1\xE6\x7F\xEA\xC1\xE8\xB6\xA0\x14\xA1X\xDCdsolcC\0\x08\x14\x003\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xA2dipfsX\"\x12 p8\x89\x9E\x12H)\xF06\xAF\x8D\x93=l\x84p#\x94\xDA\xEF\x9FQ|\xE4\xF5q:\x19\xCE\xBB\xE7\xBFdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static CLIMBERVAULT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10b\0\0\xA9W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11b\0\0lW\x80cqP\x18\xA6\x14b\0\x01NW\x80c\x8D\xA5\xCB[\x14b\0\x01fW\x80c\xC0\xC5;\x8B\x14b\0\x01\x9AW\x80c\xD9\xCA\xED\x12\x14b\0\x01\xBFW\x80c\xDE\xB0\xF0p\x14b\0\x01\xE4W\x80c\xF2\xFD\xE3\x8B\x14b\0\x02\x04W`\0\x80\xFD[\x80c\x0F\xE2\x89\x08\x14b\0\0\xAEW\x80c&m\xF7\x82\x14b\0\0\xD5W\x80c6Y\xCF\xE6\x14b\0\0\xFAW\x80cO\x1E\xF2\x86\x14b\0\x01\x1FW\x80cR\xD1\x90-\x14b\0\x016W[`\0\x80\xFD[4\x80\x15b\0\0\xBBW`\0\x80\xFD[Pb\0\0\xD3b\0\0\xCD6`\x04b\0\r\xB1V[b\0\x02)V[\0[4\x80\x15b\0\0\xE2W`\0\x80\xFD[P`\xC9T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15b\0\x01\x07W`\0\x80\xFD[Pb\0\0\xD3b\0\x01\x196`\x04b\0\r\xB1V[b\0\x02\xD7V[b\0\0\xD3b\0\x0106`\x04b\0\r\xE5V[b\0\x03\xC8V[4\x80\x15b\0\x01CW`\0\x80\xFD[Pb\0\0\xE7b\0\x04\xA4V[4\x80\x15b\0\x01[W`\0\x80\xFD[Pb\0\0\xD3b\0\x05ZV[4\x80\x15b\0\x01sW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01b\0\0\xF1V[4\x80\x15b\0\x01\xA7W`\0\x80\xFD[Pb\0\0\xD3b\0\x01\xB96`\x04b\0\x0E\xB3V[b\0\x05rV[4\x80\x15b\0\x01\xCCW`\0\x80\xFD[Pb\0\0\xD3b\0\x01\xDE6`\x04b\0\x0E\xFDV[b\0\x07\rV[4\x80\x15b\0\x01\xF1W`\0\x80\xFD[P`\xCAT`\x01`\x01`\xA0\x1B\x03\x16b\0\x01\x81V[4\x80\x15b\0\x02\x11W`\0\x80\xFD[Pb\0\0\xD3b\0\x02#6`\x04b\0\r\xB1V[b\0\x07\x90V[`\xCAT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x02UW`@Qc\xF9\x08a\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xCAT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Rb\0\x02\xD4\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x02\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02\xCE\x91\x90b\0\x0F>V[b\0\x08\x0CV[PV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03b\0\x03+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0FXV[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16b\0\x03v`\0\x80Q` b\0&6\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\x03\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0F\xA4V[b\0\x03\xAA\x81b\0\x08SV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Rb\0\x02\xD4\x91\x83\x91\x90b\0\x08]V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03b\0\x04\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0FXV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16b\0\x04^`\0\x80Q` b\0&6\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\x04\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x0F\xA4V[b\0\x04\x92\x82b\0\x08SV[b\0\x04\xA0\x82\x82`\x01b\0\x08]V[PPV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14b\0\x05FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x03\"V[P`\0\x80Q` b\0&6\x839\x81Q\x91R\x90V[b\0\x05db\0\t\xD5V[b\0\x05p`\0b\0\n1V[V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x05\x93WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x05\xAFWP0;\x15\x80\x15b\0\x05\xAFWP`\0T`\xFF\x16`\x01\x14[b\0\x06\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x068W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x06Bb\0\n\x83V[b\0\x06Lb\0\n\xB7V[b\0\x06\x9B\x84\x84`@Qb\0\x06`\x90b\0\r\x86V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x06\x94W=`\0\x80>=`\0\xFD[Pb\0\x07\x90V[`\xCA\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ub\0\x06\xC0B`\xC9UV[\x80\x15b\0\x07\x07W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[b\0\x07\x17b\0\t\xD5V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15b\0\x07AW`@Qc\x9A\xBCt\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x13\xC6\x80`\xC9Tb\0\x07T\x91\x90b\0\x0F\xF0V[B\x11b\0\x07tW`@Qc \xDB\xD7\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\0\x07~B`\xC9UV[b\0\x07\x8B\x83\x83\x83b\0\x08\x0CV[PPPV[b\0\x07\x9Ab\0\t\xD5V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[b\0\x02\xD4\x81b\0\n1V[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16b\0\x08IWc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[b\0\x02\xD4b\0\t\xD5V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15b\0\x08\x93Wb\0\x07\x8B\x83b\0\n\xE1V[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15b\0\x08\xF0WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Rb\0\x08\xED\x91\x81\x01\x90b\0\x0F>V[`\x01[b\0\tUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[`\0\x80Q` b\0&6\x839\x81Q\x91R\x81\x14b\0\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[Pb\0\x07\x8B\x83\x83\x83b\0\x0B\x80V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x03\"V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\n\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x10\x12V[b\0\x05pb\0\x0B\xABV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x05pW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x10\x12V[`\x01`\x01`\xA0\x1B\x03\x81\x16;b\0\x0BPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01b\0\x03\"V[`\0\x80Q` b\0&6\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[b\0\x0B\x8B\x83b\0\x0B\xE0V[`\0\x82Q\x11\x80b\0\x0B\x99WP\x80[\x15b\0\x07\x8BWb\0\x07\x07\x83\x83b\0\x0C\"V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x0B\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x90b\0\x10\x12V[b\0\x05p3b\0\n1V[b\0\x0B\xEB\x81b\0\n\xE1V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``b\0\x0CJ\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01b\0&V`'\x919b\0\x0CSV[\x90P[\x92\x91PPV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qb\0\x0Cr\x91\x90b\0\x10\x83V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14b\0\x0C\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x0C\xB4V[``\x91P[P\x91P\x91Pb\0\x0C\xC7\x86\x83\x83\x87b\0\x0C\xD1V[\x96\x95PPPPPPV[``\x83\x15b\0\rEW\x82Q`\0\x03b\0\r=W`\x01`\x01`\xA0\x1B\x03\x85\x16;b\0\r=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01b\0\x03\"V[P\x81b\0\rQV[b\0\rQ\x83\x83b\0\rYV[\x94\x93PPPPV[\x81Q\x15b\0\rjW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\"\x91\x90b\0\x10\xA1V[a\x15_\x80b\0\x10\xD7\x839\x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\r\xACW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\r\xC4W`\0\x80\xFD[b\0\x0CJ\x82b\0\r\x94V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0\r\xF9W`\0\x80\xFD[b\0\x0E\x04\x83b\0\r\x94V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x0E\"W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x0E7W`\0\x80\xFD[\x815\x81\x81\x11\x15b\0\x0ELWb\0\x0ELb\0\r\xCFV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x0EwWb\0\x0Ewb\0\r\xCFV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15b\0\x0E\x91W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x0E\xC9W`\0\x80\xFD[b\0\x0E\xD4\x84b\0\r\x94V[\x92Pb\0\x0E\xE4` \x85\x01b\0\r\x94V[\x91Pb\0\x0E\xF4`@\x85\x01b\0\r\x94V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x0F\x13W`\0\x80\xFD[b\0\x0F\x1E\x84b\0\r\x94V[\x92Pb\0\x0F.` \x85\x01b\0\r\x94V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x0FQW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15b\0\x0CMWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`\0[\x83\x81\x10\x15b\0\x10zW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x10`V[PP`\0\x91\x01RV[`\0\x82Qb\0\x10\x97\x81\x84` \x87\x01b\0\x10]V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Rb\0\x10\xC2\x81`@\x85\x01` \x87\x01b\0\x10]V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x15_8\x03\x80b\0\x15_\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xFEV[b\0\0O`\0\x80Q` b\0\x15?\x839\x81Q\x91R\x80b\0\0\xE6V[b\0\0y`\0\x80Q` b\0\x15\x1F\x839\x81Q\x91R`\0\x80Q` b\0\x15?\x839\x81Q\x91Rb\0\0\xE6V[b\0\0\x94`\0\x80Q` b\0\x15?\x839\x81Q\x91R\x83b\0\x011V[b\0\0\xAF`\0\x80Q` b\0\x15?\x839\x81Q\x91R0b\0\x011V[b\0\0\xCA`\0\x80Q` b\0\x15\x1F\x839\x81Q\x91R\x82b\0\x011V[PP`\x02\x80T`\x01`\x01`@\x1B\x03\x19\x16a\x0E\x10\x17\x90Ub\0\x026V[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[b\0\x01=\x82\x82b\0\x01AV[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x01=W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01\x9D3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x02\x12W`\0\x80\xFD[b\0\x02\x1D\x83b\0\x01\xE1V[\x91Pb\0\x02-` \x84\x01b\0\x01\xE1V[\x90P\x92P\x92\x90PV[a\x12\xD9\x80b\0\x02F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xE1W`\x005`\xE0\x1C\x80cjB\xB8\xF8\x11a\0\x7FW\x80c\x91\xD1HT\x11a\0YW\x80c\x91\xD1HT\x14a\x02zW\x80c\xA2\x17\xFD\xDF\x14a\x02\x9AW\x80c\xC7O4\x9B\x14a\x02\xAFW\x80c\xD5Gt\x1F\x14a\x03\"W`\0\x80\xFD[\x80cjB\xB8\xF8\x14a\x01\xF5W\x80cyX\0L\x14a\x02-W\x80c\x90\xBD\x1Em\x14a\x02ZW`\0\x80\xFD[\x80c&V\"}\x11a\0\xBBW\x80c&V\"}\x14a\x01\x82W\x80c//\xF1]\x14a\x01\x95W\x80c6V\x8A\xBE\x14a\x01\xB5W\x80cW\xF5%\xED\x14a\x01\xD5W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xEDW\x80c$\x8A\x9C\xA3\x14a\x01\"W\x80c$\xAD\xBC[\x14a\x01`W`\0\x80\xFD[6a\0\xE8W\0[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[Pa\x01\ra\x01\x086`\x04a\rPV[a\x03BV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01.W`\0\x80\xFD[Pa\x01Ra\x01=6`\x04a\rzV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x19V[4\x80\x15a\x01lW`\0\x80\xFD[Pa\x01\x80a\x01{6`\x04a\r\x93V[a\x03yV[\0[a\x01\x80a\x01\x906`\x04a\x0E\x07V[a\x03\xE9V[4\x80\x15a\x01\xA1W`\0\x80\xFD[Pa\x01\x80a\x01\xB06`\x04a\x0E\xC5V[a\x05\x98V[4\x80\x15a\x01\xC1W`\0\x80\xFD[Pa\x01\x80a\x01\xD06`\x04a\x0E\xC5V[a\x05\xC2V[4\x80\x15a\x01\xE1W`\0\x80\xFD[Pa\x01Ra\x01\xF06`\x04a\x0E\x07V[a\x06@V[4\x80\x15a\x02\x01W`\0\x80\xFD[P`\x02Ta\x02\x15\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[4\x80\x15a\x029W`\0\x80\xFD[Pa\x02Ma\x02H6`\x04a\rzV[a\x06\x82V[`@Qa\x01\x19\x91\x90a\x0F\x07V[4\x80\x15a\x02fW`\0\x80\xFD[Pa\x01\x80a\x02u6`\x04a\x0E\x07V[a\x07\x17V[4\x80\x15a\x02\x86W`\0\x80\xFD[Pa\x01\ra\x02\x956`\x04a\x0E\xC5V[a\x08SV[4\x80\x15a\x02\xA6W`\0\x80\xFD[Pa\x01R`\0\x81V[4\x80\x15a\x02\xBBW`\0\x80\xFD[Pa\x02\xFBa\x02\xCA6`\x04a\rzV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x81\x16\x90`\xFF`\x01`@\x1B\x82\x04\x81\x16\x91`\x01`H\x1B\x90\x04\x16\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x94\x16\x84R\x91\x15\x15` \x84\x01R\x15\x15\x90\x82\x01R``\x01a\x01\x19V[4\x80\x15a\x03.W`\0\x80\xFD[Pa\x01\x80a\x03=6`\x04a\x0E\xC5V[a\x08|V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03sWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[30\x14a\x03\x99W`@Qc\xDF\xB4\x9E1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x12u\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x03\xC6W`@Qc\x1E=\t1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x85a\x04\x07W`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14a\x04'W`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x82\x14a\x04GW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x04X\x88\x88\x88\x88\x88\x88\x88a\x06@V[\x90P`\0[`\xFF\x81\x16\x88\x11\x15a\x05'Wa\x05\x1E\x85\x85\x83`\xFF\x16\x81\x81\x10a\x04\x80Wa\x04\x80a\x0F/V[\x90P` \x02\x81\x01\x90a\x04\x92\x91\x90a\x0FEV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92P\x8A\x91PP`\xFF\x85\x16\x81\x81\x10a\x04\xDDWa\x04\xDDa\x0F/V[\x90P` \x02\x015\x8B\x8B\x85`\xFF\x16\x81\x81\x10a\x04\xF9Wa\x04\xF9a\x0F/V[\x90P` \x02\x01` \x81\x01\x90a\x05\x0E\x91\x90a\x0F\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x08\xA1V[P`\x01\x01a\x04]V[P`\x02a\x053\x82a\x06\x82V[`\x03\x81\x11\x15a\x05DWa\x05Da\x0E\xF1V[\x14a\x05jW`@Qc\x08)_\xC9`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` R`@\x90 \x80Ti\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1B\x17\x90UPPPPPPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\xB3\x81a\x08\xCFV[a\x05\xBD\x83\x83a\x08\xDCV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x062W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05aV[a\x06<\x82\x82a\t`V[PPV[`\0\x87\x87\x87\x87\x87\x87\x87`@Q` \x01a\x06_\x97\x96\x95\x94\x93\x92\x91\x90a\x10`V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\xFF`\x01`@\x1B\x82\x04\x81\x16\x15\x80\x15\x95\x84\x01\x95\x90\x95R`\x01`H\x1B\x90\x91\x04\x16\x15\x15\x91\x81\x01\x91\x90\x91R\x90a\x07\x0CW\x80`@\x01Q\x15a\x06\xE8W`\x03\x91Pa\x07\x11V[\x80Q`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x07\x03W`\x01\x91Pa\x07\x11V[`\x02\x91Pa\x07\x11V[`\0\x91P[P\x91\x90PV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\x07A\x81a\x08\xCFV[\x86\x15\x80a\x07PWPa\x01\0\x87\x10\x15[\x15a\x07nW`@QcWd\x05\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x85\x14a\x07\x8EW`@Qc\x17\x160\x7F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x83\x14a\x07\xAEW`@Qcv\xCE\xFB\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xBF\x89\x89\x89\x89\x89\x89\x89a\x06@V[\x90P`\0a\x07\xCC\x82a\x06\x82V[`\x03\x81\x11\x15a\x07\xDDWa\x07\xDDa\x0E\xF1V[\x14a\x07\xFEW`@Qc \xB1\x99\xD1`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x05aV[`\x02Ta\x08\x14\x90`\x01`\x01`@\x1B\x03\x16Ba\x11\x14V[`\0\x91\x82R`\x01` R`@\x90\x91 \x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17`\x01`@\x1B\x17\x90UPPPPPPPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x97\x81a\x08\xCFV[a\x05\xBD\x83\x83a\t`V[``a\x08\xC7\x84\x84\x84`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x12{`)\x919a\t\xC5V[\x94\x93PPPPV[a\x08\xD9\x813a\n\xA0V[PV[a\x08\xE6\x82\x82a\x08SV[a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\x1C3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\tj\x82\x82a\x08SV[\x15a\x06<W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``\x82G\x10\x15a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05aV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\nB\x91\x90a\x11_V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\x7FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x84V[``\x91P[P\x91P\x91Pa\n\x95\x87\x83\x83\x87a\n\xF9V[\x97\x96PPPPPPPV[a\n\xAA\x82\x82a\x08SV[a\x06<Wa\n\xB7\x81a\x0BrV[a\n\xC2\x83` a\x0B\x84V[`@Q` \x01a\n\xD3\x92\x91\x90a\x11{V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05a\x91`\x04\x01a\x11\xF0V[``\x83\x15a\x0BhW\x82Q`\0\x03a\x0BaW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x0BaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05aV[P\x81a\x08\xC7V[a\x08\xC7\x83\x83a\r&V[``a\x03s`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B\x93\x83`\x02a\x12#V[a\x0B\x9E\x90`\x02a\x12:V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xB5Wa\x0B\xB5a\x12MV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xDFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\xFAWa\x0B\xFAa\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0C)Wa\x0C)a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0CM\x84`\x02a\x12#V[a\x0CX\x90`\x01a\x12:V[\x90P[`\x01\x81\x11\x15a\x0C\xD0Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0C\x8CWa\x0C\x8Ca\x0F/V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0C\xA2Wa\x0C\xA2a\x0F/V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0C\xC9\x81a\x12cV[\x90Pa\x0C[V[P\x83\x15a\r\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05aV[\x93\x92PPPV[\x81Q\x15a\r6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05a\x91\x90a\x11\xF0V[`\0` \x82\x84\x03\x12\x15a\rbW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x1FW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\r\xCEW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xE5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0E\0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x0E\"W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E9W`\0\x80\xFD[a\x0EE\x8B\x83\x8C\x01a\r\xBCV[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a\x0E^W`\0\x80\xFD[a\x0Ej\x8B\x83\x8C\x01a\r\xBCV[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a\x0E\x83W`\0\x80\xFD[Pa\x0E\x90\x8A\x82\x8B\x01a\r\xBCV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xC0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xD8W`\0\x80\xFD[\x825\x91Pa\x0E\xE8` \x84\x01a\x0E\xA9V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10a\x0F)WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0F\\W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0FvW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0E\0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\x9DW`\0\x80\xFD[a\r\x1F\x82a\x0E\xA9V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0[\x87\x81\x10\x15a\x10SW\x82\x84\x03\x89R\x815`\x1E\x19\x886\x03\x01\x81\x12a\x10\nW`\0\x80\xFD[\x87\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10%W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x104W`\0\x80\xFD[a\x10?\x86\x82\x84a\x0F\xA6V[\x9A\x87\x01\x9A\x95PPP\x90\x84\x01\x90`\x01\x01a\x0F\xE9V[P\x91\x97\x96PPPPPPPV[`\x80\x80\x82R\x81\x01\x87\x90R`\0\x88`\xA0\x83\x01\x82[\x8A\x81\x10\x15a\x10\xA1W`\x01`\x01`\xA0\x1B\x03a\x10\x8C\x84a\x0E\xA9V[\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x10sV[P\x83\x81\x03` \x85\x01R\x87\x81R`\x01`\x01`\xFB\x1B\x03\x88\x11\x15a\x10\xC1W`\0\x80\xFD[\x87`\x05\x1B\x91P\x81\x89` \x83\x017\x01\x82\x81\x03` \x90\x81\x01`@\x85\x01Ra\x10\xE9\x90\x82\x01\x86\x88a\x0F\xCFV[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x114Wa\x114a\x10\xFEV[P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11VW\x81\x81\x01Q\x83\x82\x01R` \x01a\x11>V[PP`\0\x91\x01RV[`\0\x82Qa\x11q\x81\x84` \x87\x01a\x11;V[\x91\x90\x91\x01\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x11\xB3\x81`\x17\x85\x01` \x88\x01a\x11;V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x11\xE4\x81`(\x84\x01` \x88\x01a\x11;V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x12\x0F\x81`@\x85\x01` \x87\x01a\x11;V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03sWa\x03sa\x10\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x03sWa\x03sa\x10\xFEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x81a\x12rWa\x12ra\x10\xFEV[P`\0\x19\x01\x90V\xFEAddress: low-level call with value failed\xA2dipfsX\"\x12 8\x83\xA5B\xBF\xF1L\xD6\xE4x\xE5\x99\xFB\xB1'(\xFDj\x1C\xB5\xC1\xE6\x7F\xEA\xC1\xE8\xB6\xA0\x14\xA1X\xDCdsolcC\0\x08\x14\x003\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xA2dipfsX\"\x12 p8\x89\x9E\x12H)\xF06\xAF\x8D\x93=l\x84p#\x94\xDA\xEF\x9FQ|\xE4\xF5q:\x19\xCE\xBB\xE7\xBFdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static CLIMBERVAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ClimberVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ClimberVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ClimberVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ClimberVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ClimberVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ClimberVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ClimberVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CLIMBERVAULT_ABI.clone(),
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
                CLIMBERVAULT_ABI.clone(),
                CLIMBERVAULT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getLastWithdrawalTimestamp` (0x266df782) function
        pub fn get_last_withdrawal_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 109, 247, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSweeper` (0xdeb0f070) function
        pub fn get_sweeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([222, 176, 240, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc0c53b8b) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            proposer: ::ethers::core::types::Address,
            sweeper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 197, 59, 139], (admin, proposer, sweeper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
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
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepFunds` (0x0fe28908) function
        pub fn sweep_funds(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 226, 137, 8], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x3659cfe6) function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xd9caed12) function
        pub fn withdraw(
            &self,
            token: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 202, 237, 18], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BeaconUpgraded` event
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconUpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClimberVaultEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ClimberVault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerNotSweeper` with signature `CallerNotSweeper()` and selector `0xf90861ff`
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
    #[etherror(name = "CallerNotSweeper", abi = "CallerNotSweeper()")]
    pub struct CallerNotSweeper;
    ///Custom Error type `InvalidWithdrawalAmount` with signature `InvalidWithdrawalAmount()` and selector `0x9abc7491`
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
    #[etherror(name = "InvalidWithdrawalAmount", abi = "InvalidWithdrawalAmount()")]
    pub struct InvalidWithdrawalAmount;
    ///Custom Error type `InvalidWithdrawalTime` with signature `InvalidWithdrawalTime()` and selector `0x41b7af92`
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
    #[etherror(name = "InvalidWithdrawalTime", abi = "InvalidWithdrawalTime()")]
    pub struct InvalidWithdrawalTime;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ClimberVaultErrors {
        CallerNotSweeper(CallerNotSweeper),
        InvalidWithdrawalAmount(InvalidWithdrawalAmount),
        InvalidWithdrawalTime(InvalidWithdrawalTime),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ClimberVaultErrors {
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
                = <CallerNotSweeper as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerNotSweeper(decoded));
            }
            if let Ok(decoded)
                = <InvalidWithdrawalAmount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidWithdrawalAmount(decoded));
            }
            if let Ok(decoded)
                = <InvalidWithdrawalTime as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidWithdrawalTime(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ClimberVaultErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerNotSweeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWithdrawalAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWithdrawalTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ClimberVaultErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerNotSweeper as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidWithdrawalAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidWithdrawalTime as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ClimberVaultErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerNotSweeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWithdrawalAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidWithdrawalTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ClimberVaultErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerNotSweeper> for ClimberVaultErrors {
        fn from(value: CallerNotSweeper) -> Self {
            Self::CallerNotSweeper(value)
        }
    }
    impl ::core::convert::From<InvalidWithdrawalAmount> for ClimberVaultErrors {
        fn from(value: InvalidWithdrawalAmount) -> Self {
            Self::InvalidWithdrawalAmount(value)
        }
    }
    impl ::core::convert::From<InvalidWithdrawalTime> for ClimberVaultErrors {
        fn from(value: InvalidWithdrawalTime) -> Self {
            Self::InvalidWithdrawalTime(value)
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ::ethers::core::types::Address,
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        Hash
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ClimberVaultEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ClimberVaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(ClimberVaultEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(ClimberVaultEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ClimberVaultEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ClimberVaultEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(ClimberVaultEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ClimberVaultEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for ClimberVaultEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for ClimberVaultEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for ClimberVaultEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ClimberVaultEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for ClimberVaultEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `getLastWithdrawalTimestamp` function with signature `getLastWithdrawalTimestamp()` and selector `0x266df782`
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
    #[ethcall(name = "getLastWithdrawalTimestamp", abi = "getLastWithdrawalTimestamp()")]
    pub struct GetLastWithdrawalTimestampCall;
    ///Container type for all input parameters for the `getSweeper` function with signature `getSweeper()` and selector `0xdeb0f070`
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
    #[ethcall(name = "getSweeper", abi = "getSweeper()")]
    pub struct GetSweeperCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address)` and selector `0xc0c53b8b`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address)")]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub proposer: ::ethers::core::types::Address,
        pub sweeper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `sweepFunds` function with signature `sweepFunds(address)` and selector `0x0fe28908`
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
    #[ethcall(name = "sweepFunds", abi = "sweepFunds(address)")]
    pub struct SweepFundsCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `0x3659cfe6`
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
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address,address,uint256)` and selector `0xd9caed12`
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
    #[ethcall(name = "withdraw", abi = "withdraw(address,address,uint256)")]
    pub struct WithdrawCall {
        pub token: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ClimberVaultCalls {
        GetLastWithdrawalTimestamp(GetLastWithdrawalTimestampCall),
        GetSweeper(GetSweeperCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        RenounceOwnership(RenounceOwnershipCall),
        SweepFunds(SweepFundsCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for ClimberVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetLastWithdrawalTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLastWithdrawalTimestamp(decoded));
            }
            if let Ok(decoded)
                = <GetSweeperCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSweeper(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SweepFundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SweepFunds(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded)
                = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ClimberVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetLastWithdrawalTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSweeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ClimberVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetLastWithdrawalTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSweeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetLastWithdrawalTimestampCall> for ClimberVaultCalls {
        fn from(value: GetLastWithdrawalTimestampCall) -> Self {
            Self::GetLastWithdrawalTimestamp(value)
        }
    }
    impl ::core::convert::From<GetSweeperCall> for ClimberVaultCalls {
        fn from(value: GetSweeperCall) -> Self {
            Self::GetSweeper(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ClimberVaultCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ClimberVaultCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for ClimberVaultCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ClimberVaultCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SweepFundsCall> for ClimberVaultCalls {
        fn from(value: SweepFundsCall) -> Self {
            Self::SweepFunds(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ClimberVaultCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for ClimberVaultCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for ClimberVaultCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for ClimberVaultCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `getLastWithdrawalTimestamp` function with signature `getLastWithdrawalTimestamp()` and selector `0x266df782`
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
    pub struct GetLastWithdrawalTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSweeper` function with signature `getSweeper()` and selector `0xdeb0f070`
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
    pub struct GetSweeperReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
}
