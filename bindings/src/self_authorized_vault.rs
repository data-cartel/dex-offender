pub use self_authorized_vault::*;
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
pub mod self_authorized_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WAITING_PERIOD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WAITING_PERIOD"),
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
                    ::std::borrow::ToOwned::to_owned("WITHDRAWAL_LIMIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WITHDRAWAL_LIMIT"),
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
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getActionId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActionId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("executor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialized"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("permissions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permissions"),
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
                    ::std::borrow::ToOwned::to_owned("setPermissions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPermissions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ids"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                    ::std::borrow::ToOwned::to_owned("sweepFunds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepFunds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("who"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ids"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("AlreadyInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadyInitialized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallerNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallerNotAllowed"),
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
                    ::std::borrow::ToOwned::to_owned("NotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotAllowed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TargetNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TargetNotAllowed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalWaitingPeriodNotEnded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawalWaitingPeriodNotEnded",
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
    pub static SELFAUTHORIZEDVAULT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@RB`\x03U4\x80\x15a\0\x14W`\0\x80\xFD[P`\x01`\0Ua\n\xFE\x80a\0)`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x82\xEE\r\x1D\x11a\0fW\x80c\x82\xEE\r\x1D\x14a\x01\x14W\x80c\x85\xFBp\x9D\x14a\x01#W\x80c\xAE\xAB\xAEk\x14a\x018W\x80c\xB4\xD28\x8F\x14a\x01KW\x80c\xD9\xCA\xED\x12\x14a\x01nW`\0\x80\xFD[\x80c\x15\x8E\xF9>\x14a\0\xA3W\x80c\x1C\xFFy\xCD\x14a\0\xC5W\x80c&m\xF7\x82\x14a\0\xE5W\x80c>\x15$\x99\x14a\0\xF7W\x80co\x85\xC7\xE4\x14a\x01\nW[`\0\x80\xFD[`\x01Ta\0\xB0\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD8a\0\xD36`\x04a\x07_V[a\x01\x81V[`@Qa\0\xBC\x91\x90a\x084V[`\x03T[`@Q\x90\x81R` \x01a\0\xBCV[a\0\xE9a\x01\x056`\x04a\x08GV[a\x02oV[a\0\xE9b\x13\xC6\x80\x81V[a\0\xE9g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x016a\x0116`\x04a\x08\x9FV[a\x02\xCAV[\0[a\x016a\x01F6`\x04a\x08\xEEV[a\x03cV[a\0\xB0a\x01Y6`\x04a\t\xACV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x016a\x01|6`\x04a\t\xC5V[a\x04%V[``a\x01\x8Ba\x04\xB2V[`d\x805\x90`\x02`\0a\x01\x9F\x843\x8Aa\x02oV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16a\x01\xD0W`@Qc\x1E\xB4\x9Dm`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\x10\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x05\x10\x92PPPV[a\x02Z\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x01`\x01`\xA0\x1B\x03\x8A\x16\x92\x91PPa\x059V[\x92PPPa\x02h`\x01`\0UV[\x93\x92PPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x84\x16` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16`$\x84\x01R\x83\x90\x1B\x16`8\x82\x01R`\0\x90`L\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[30\x14a\x02\xEAW`@Qc\x01W\x83\xE9`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x03_\x90\x82\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x036W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03Z\x91\x90a\n\x06V[a\x05\x86V[PPV[`\x01T`\xFF\x16\x15a\x03\x86W`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x03\xDBW`\x01`\x02`\0\x84\x84\x81Q\x81\x10a\x03\xAAWa\x03\xAAa\n\x1FV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\x01\x01a\x03\x89V[P`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`@Q\x7F\x0EM\x88EB\xD5OM*\x1D\xE5\x0Bf\x11\xA3\x8A\x11T\x84\xDDC\x9BZ\xDEO\xD5\xA0g\xC4(f'\x90a\x04\x1A\x903\x90\x84\x90a\n5V[`@Q\x80\x91\x03\x90\xA1PV[30\x14a\x04EW`@Qc\x01W\x83\xE9`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x04nW`@Qc\x9A\xBCt\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x13\xC6\x80`\x03Ta\x04\x7F\x91\x90a\n\x8BV[B\x11a\x04\x9EW`@Qc\x0FT\r-`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B`\x03Ua\x04\xAD\x83\x83\x83a\x05\x86V[PPPV[`\x02`\0T\x03a\x05\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\x01`\x01`\xA0\x1B\x03\x82\x160\x14a\x03_W`@QcH\xCB\xF2m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``a\x05}\x83\x83`\0`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FAddress: low-level call failed\0\0\x81RPa\x05\xCCV[\x90P[\x92\x91PPV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\x05\xC2Wc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[``\x82G\x10\x15a\x06-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05\0V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x06I\x91\x90a\n\xACV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\x8BV[``\x91P[P\x91P\x91Pa\x06\x9C\x87\x83\x83\x87a\x06\xA9V[\x92PPP[\x94\x93PPPPV[``\x83\x15a\x07\x18W\x82Q`\0\x03a\x07\x11W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x07\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\0V[P\x81a\x06\xA1V[a\x06\xA1\x83\x83\x81Q\x15a\x07-W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\0\x91\x90a\x084V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\\W`\0\x80\xFD[PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07tW`\0\x80\xFD[\x835a\x07\x7F\x81a\x07GV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\x9CW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x07\xB0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\xBFW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\xD1W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x07\xFFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xE7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x08 \x81` \x86\x01` \x86\x01a\x07\xE4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x05}` \x83\x01\x84a\x08\x08V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x08\\W`\0\x80\xFD[\x835`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08tW`\0\x80\xFD[\x92P` \x84\x015a\x08\x84\x81a\x07GV[\x91P`@\x84\x015a\x08\x94\x81a\x07GV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xB2W`\0\x80\xFD[\x825a\x08\xBD\x81a\x07GV[\x91P` \x83\x015a\x08\xCD\x81a\x07GV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\t\x01W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x19W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t-W`\0\x80\xFD[\x815\x81\x81\x11\x15a\t?Wa\t?a\x08\xD8V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\tdWa\tda\x08\xD8V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\t\x82W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\t\xA0W\x845\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\t\x87V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\xBEW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\t\xDAW`\0\x80\xFD[\x835a\t\xE5\x81a\x07GV[\x92P` \x84\x015a\t\xF5\x81a\x07GV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\x18W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R`\0\x91\x84\x82\x01\x91\x90``\x85\x01\x90\x84[\x81\x81\x10\x15a\n~W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\nbV[P\x90\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\x80WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82Qa\n\xBE\x81\x84` \x87\x01a\x07\xE4V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 ;\xED=\xC4\xCB\xA1\x16\xCB6\x11\xC9\x9E\xF6\xD1N\xBE\xEE\x8DT\x8Aa\x03z\xE0\x16\xB3\xF5\x07\xC5\xD3\r\xBFdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static SELFAUTHORIZEDVAULT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x82\xEE\r\x1D\x11a\0fW\x80c\x82\xEE\r\x1D\x14a\x01\x14W\x80c\x85\xFBp\x9D\x14a\x01#W\x80c\xAE\xAB\xAEk\x14a\x018W\x80c\xB4\xD28\x8F\x14a\x01KW\x80c\xD9\xCA\xED\x12\x14a\x01nW`\0\x80\xFD[\x80c\x15\x8E\xF9>\x14a\0\xA3W\x80c\x1C\xFFy\xCD\x14a\0\xC5W\x80c&m\xF7\x82\x14a\0\xE5W\x80c>\x15$\x99\x14a\0\xF7W\x80co\x85\xC7\xE4\x14a\x01\nW[`\0\x80\xFD[`\x01Ta\0\xB0\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD8a\0\xD36`\x04a\x07_V[a\x01\x81V[`@Qa\0\xBC\x91\x90a\x084V[`\x03T[`@Q\x90\x81R` \x01a\0\xBCV[a\0\xE9a\x01\x056`\x04a\x08GV[a\x02oV[a\0\xE9b\x13\xC6\x80\x81V[a\0\xE9g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x016a\x0116`\x04a\x08\x9FV[a\x02\xCAV[\0[a\x016a\x01F6`\x04a\x08\xEEV[a\x03cV[a\0\xB0a\x01Y6`\x04a\t\xACV[`\x02` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x016a\x01|6`\x04a\t\xC5V[a\x04%V[``a\x01\x8Ba\x04\xB2V[`d\x805\x90`\x02`\0a\x01\x9F\x843\x8Aa\x02oV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16a\x01\xD0W`@Qc\x1E\xB4\x9Dm`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\x10\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x05\x10\x92PPPV[a\x02Z\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x01`\x01`\xA0\x1B\x03\x8A\x16\x92\x91PPa\x059V[\x92PPPa\x02h`\x01`\0UV[\x93\x92PPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x84\x16` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16`$\x84\x01R\x83\x90\x1B\x16`8\x82\x01R`\0\x90`L\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[30\x14a\x02\xEAW`@Qc\x01W\x83\xE9`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x03_\x90\x82\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x036W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03Z\x91\x90a\n\x06V[a\x05\x86V[PPV[`\x01T`\xFF\x16\x15a\x03\x86W`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x03\xDBW`\x01`\x02`\0\x84\x84\x81Q\x81\x10a\x03\xAAWa\x03\xAAa\n\x1FV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\x01\x01a\x03\x89V[P`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`@Q\x7F\x0EM\x88EB\xD5OM*\x1D\xE5\x0Bf\x11\xA3\x8A\x11T\x84\xDDC\x9BZ\xDEO\xD5\xA0g\xC4(f'\x90a\x04\x1A\x903\x90\x84\x90a\n5V[`@Q\x80\x91\x03\x90\xA1PV[30\x14a\x04EW`@Qc\x01W\x83\xE9`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x04nW`@Qc\x9A\xBCt\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x13\xC6\x80`\x03Ta\x04\x7F\x91\x90a\n\x8BV[B\x11a\x04\x9EW`@Qc\x0FT\r-`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B`\x03Ua\x04\xAD\x83\x83\x83a\x05\x86V[PPPV[`\x02`\0T\x03a\x05\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\x01`\x01`\xA0\x1B\x03\x82\x160\x14a\x03_W`@QcH\xCB\xF2m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``a\x05}\x83\x83`\0`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FAddress: low-level call failed\0\0\x81RPa\x05\xCCV[\x90P[\x92\x91PPV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B`\0R` `\0`D`\x10`\0\x87Z\xF1=\x15`\x01`\0Q\x14\x17\x16a\x05\xC2Wc\x90\xB8\xEC\x18`\0R`\x04`\x1C\xFD[`\0`4RPPPV[``\x82G\x10\x15a\x06-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05\0V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x06I\x91\x90a\n\xACV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\x8BV[``\x91P[P\x91P\x91Pa\x06\x9C\x87\x83\x83\x87a\x06\xA9V[\x92PPP[\x94\x93PPPPV[``\x83\x15a\x07\x18W\x82Q`\0\x03a\x07\x11W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x07\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\0V[P\x81a\x06\xA1V[a\x06\xA1\x83\x83\x81Q\x15a\x07-W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\0\x91\x90a\x084V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\\W`\0\x80\xFD[PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07tW`\0\x80\xFD[\x835a\x07\x7F\x81a\x07GV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\x9CW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x07\xB0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\xBFW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\xD1W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x07\xFFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xE7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x08 \x81` \x86\x01` \x86\x01a\x07\xE4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x05}` \x83\x01\x84a\x08\x08V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x08\\W`\0\x80\xFD[\x835`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08tW`\0\x80\xFD[\x92P` \x84\x015a\x08\x84\x81a\x07GV[\x91P`@\x84\x015a\x08\x94\x81a\x07GV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xB2W`\0\x80\xFD[\x825a\x08\xBD\x81a\x07GV[\x91P` \x83\x015a\x08\xCD\x81a\x07GV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\t\x01W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x19W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t-W`\0\x80\xFD[\x815\x81\x81\x11\x15a\t?Wa\t?a\x08\xD8V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\tdWa\tda\x08\xD8V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\t\x82W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\t\xA0W\x845\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\t\x87V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\xBEW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\t\xDAW`\0\x80\xFD[\x835a\t\xE5\x81a\x07GV[\x92P` \x84\x015a\t\xF5\x81a\x07GV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\x18W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R`\0\x91\x84\x82\x01\x91\x90``\x85\x01\x90\x84[\x81\x81\x10\x15a\n~W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\nbV[P\x90\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\x80WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82Qa\n\xBE\x81\x84` \x87\x01a\x07\xE4V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 ;\xED=\xC4\xCB\xA1\x16\xCB6\x11\xC9\x9E\xF6\xD1N\xBE\xEE\x8DT\x8Aa\x03z\xE0\x16\xB3\xF5\x07\xC5\xD3\r\xBFdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static SELFAUTHORIZEDVAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SelfAuthorizedVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SelfAuthorizedVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SelfAuthorizedVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SelfAuthorizedVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SelfAuthorizedVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SelfAuthorizedVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SelfAuthorizedVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SELFAUTHORIZEDVAULT_ABI.clone(),
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
                SELFAUTHORIZEDVAULT_ABI.clone(),
                SELFAUTHORIZEDVAULT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WAITING_PERIOD` (0x6f85c7e4) function
        pub fn waiting_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([111, 133, 199, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WITHDRAWAL_LIMIT` (0x82ee0d1d) function
        pub fn withdrawal_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([130, 238, 13, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x1cff79cd) function
        pub fn execute(
            &self,
            target: ::ethers::core::types::Address,
            action_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([28, 255, 121, 205], (target, action_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionId` (0x3e152499) function
        pub fn get_action_id(
            &self,
            selector: [u8; 4],
            executor: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([62, 21, 36, 153], (selector, executor, target))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLastWithdrawalTimestamp` (0x266df782) function
        pub fn get_last_withdrawal_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 109, 247, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialized` (0x158ef93e) function
        pub fn initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permissions` (0xb4d2388f) function
        pub fn permissions(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([180, 210, 56, 143], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPermissions` (0xaeabae6b) function
        pub fn set_permissions(
            &self,
            ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 171, 174, 107], ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepFunds` (0x85fb709d) function
        pub fn sweep_funds(
            &self,
            receiver: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 251, 112, 157], (receiver, token))
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SelfAuthorizedVault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyInitialized` with signature `AlreadyInitialized()` and selector `0x0dc149f0`
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
    #[etherror(name = "AlreadyInitialized", abi = "AlreadyInitialized()")]
    pub struct AlreadyInitialized;
    ///Custom Error type `CallerNotAllowed` with signature `CallerNotAllowed()` and selector `0x2af07d20`
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
    #[etherror(name = "CallerNotAllowed", abi = "CallerNotAllowed()")]
    pub struct CallerNotAllowed;
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
    ///Custom Error type `NotAllowed` with signature `NotAllowed()` and selector `0x3d693ada`
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
    #[etherror(name = "NotAllowed", abi = "NotAllowed()")]
    pub struct NotAllowed;
    ///Custom Error type `TargetNotAllowed` with signature `TargetNotAllowed()` and selector `0x48cbf26d`
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
    #[etherror(name = "TargetNotAllowed", abi = "TargetNotAllowed()")]
    pub struct TargetNotAllowed;
    ///Custom Error type `WithdrawalWaitingPeriodNotEnded` with signature `WithdrawalWaitingPeriodNotEnded()` and selector `0xf540d2d0`
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
    #[etherror(
        name = "WithdrawalWaitingPeriodNotEnded",
        abi = "WithdrawalWaitingPeriodNotEnded()"
    )]
    pub struct WithdrawalWaitingPeriodNotEnded;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SelfAuthorizedVaultErrors {
        AlreadyInitialized(AlreadyInitialized),
        CallerNotAllowed(CallerNotAllowed),
        InvalidWithdrawalAmount(InvalidWithdrawalAmount),
        NotAllowed(NotAllowed),
        TargetNotAllowed(TargetNotAllowed),
        WithdrawalWaitingPeriodNotEnded(WithdrawalWaitingPeriodNotEnded),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SelfAuthorizedVaultErrors {
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
                = <AlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyInitialized(decoded));
            }
            if let Ok(decoded)
                = <CallerNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerNotAllowed(decoded));
            }
            if let Ok(decoded)
                = <InvalidWithdrawalAmount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidWithdrawalAmount(decoded));
            }
            if let Ok(decoded)
                = <NotAllowed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotAllowed(decoded));
            }
            if let Ok(decoded)
                = <TargetNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TargetNotAllowed(decoded));
            }
            if let Ok(decoded)
                = <WithdrawalWaitingPeriodNotEnded as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawalWaitingPeriodNotEnded(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SelfAuthorizedVaultErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWithdrawalAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalWaitingPeriodNotEnded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SelfAuthorizedVaultErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadyInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CallerNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidWithdrawalAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotAllowed as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <TargetNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WithdrawalWaitingPeriodNotEnded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SelfAuthorizedVaultErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWithdrawalAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalWaitingPeriodNotEnded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SelfAuthorizedVaultErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyInitialized> for SelfAuthorizedVaultErrors {
        fn from(value: AlreadyInitialized) -> Self {
            Self::AlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<CallerNotAllowed> for SelfAuthorizedVaultErrors {
        fn from(value: CallerNotAllowed) -> Self {
            Self::CallerNotAllowed(value)
        }
    }
    impl ::core::convert::From<InvalidWithdrawalAmount> for SelfAuthorizedVaultErrors {
        fn from(value: InvalidWithdrawalAmount) -> Self {
            Self::InvalidWithdrawalAmount(value)
        }
    }
    impl ::core::convert::From<NotAllowed> for SelfAuthorizedVaultErrors {
        fn from(value: NotAllowed) -> Self {
            Self::NotAllowed(value)
        }
    }
    impl ::core::convert::From<TargetNotAllowed> for SelfAuthorizedVaultErrors {
        fn from(value: TargetNotAllowed) -> Self {
            Self::TargetNotAllowed(value)
        }
    }
    impl ::core::convert::From<WithdrawalWaitingPeriodNotEnded>
    for SelfAuthorizedVaultErrors {
        fn from(value: WithdrawalWaitingPeriodNotEnded) -> Self {
            Self::WithdrawalWaitingPeriodNotEnded(value)
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
    #[ethevent(name = "Initialized", abi = "Initialized(address,bytes32[])")]
    pub struct InitializedFilter {
        pub who: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `WAITING_PERIOD` function with signature `WAITING_PERIOD()` and selector `0x6f85c7e4`
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
    #[ethcall(name = "WAITING_PERIOD", abi = "WAITING_PERIOD()")]
    pub struct WaitingPeriodCall;
    ///Container type for all input parameters for the `WITHDRAWAL_LIMIT` function with signature `WITHDRAWAL_LIMIT()` and selector `0x82ee0d1d`
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
    #[ethcall(name = "WITHDRAWAL_LIMIT", abi = "WITHDRAWAL_LIMIT()")]
    pub struct WithdrawalLimitCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(address,bytes)` and selector `0x1cff79cd`
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
    #[ethcall(name = "execute", abi = "execute(address,bytes)")]
    pub struct ExecuteCall {
        pub target: ::ethers::core::types::Address,
        pub action_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getActionId` function with signature `getActionId(bytes4,address,address)` and selector `0x3e152499`
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
    #[ethcall(name = "getActionId", abi = "getActionId(bytes4,address,address)")]
    pub struct GetActionIdCall {
        pub selector: [u8; 4],
        pub executor: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `0x158ef93e`
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
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
    ///Container type for all input parameters for the `permissions` function with signature `permissions(bytes32)` and selector `0xb4d2388f`
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
    #[ethcall(name = "permissions", abi = "permissions(bytes32)")]
    pub struct PermissionsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `setPermissions` function with signature `setPermissions(bytes32[])` and selector `0xaeabae6b`
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
    #[ethcall(name = "setPermissions", abi = "setPermissions(bytes32[])")]
    pub struct SetPermissionsCall {
        pub ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `sweepFunds` function with signature `sweepFunds(address,address)` and selector `0x85fb709d`
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
    #[ethcall(name = "sweepFunds", abi = "sweepFunds(address,address)")]
    pub struct SweepFundsCall {
        pub receiver: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
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
    pub enum SelfAuthorizedVaultCalls {
        WaitingPeriod(WaitingPeriodCall),
        WithdrawalLimit(WithdrawalLimitCall),
        Execute(ExecuteCall),
        GetActionId(GetActionIdCall),
        GetLastWithdrawalTimestamp(GetLastWithdrawalTimestampCall),
        Initialized(InitializedCall),
        Permissions(PermissionsCall),
        SetPermissions(SetPermissionsCall),
        SweepFunds(SweepFundsCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for SelfAuthorizedVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <WaitingPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WaitingPeriod(decoded));
            }
            if let Ok(decoded)
                = <WithdrawalLimitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawalLimit(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <GetActionIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetActionId(decoded));
            }
            if let Ok(decoded)
                = <GetLastWithdrawalTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLastWithdrawalTimestamp(decoded));
            }
            if let Ok(decoded)
                = <InitializedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialized(decoded));
            }
            if let Ok(decoded)
                = <PermissionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permissions(decoded));
            }
            if let Ok(decoded)
                = <SetPermissionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPermissions(decoded));
            }
            if let Ok(decoded)
                = <SweepFundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SweepFunds(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SelfAuthorizedVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::WaitingPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetActionId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLastWithdrawalTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Permissions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPermissions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SelfAuthorizedVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::WaitingPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActionId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLastWithdrawalTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permissions(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPermissions(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<WaitingPeriodCall> for SelfAuthorizedVaultCalls {
        fn from(value: WaitingPeriodCall) -> Self {
            Self::WaitingPeriod(value)
        }
    }
    impl ::core::convert::From<WithdrawalLimitCall> for SelfAuthorizedVaultCalls {
        fn from(value: WithdrawalLimitCall) -> Self {
            Self::WithdrawalLimit(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for SelfAuthorizedVaultCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetActionIdCall> for SelfAuthorizedVaultCalls {
        fn from(value: GetActionIdCall) -> Self {
            Self::GetActionId(value)
        }
    }
    impl ::core::convert::From<GetLastWithdrawalTimestampCall>
    for SelfAuthorizedVaultCalls {
        fn from(value: GetLastWithdrawalTimestampCall) -> Self {
            Self::GetLastWithdrawalTimestamp(value)
        }
    }
    impl ::core::convert::From<InitializedCall> for SelfAuthorizedVaultCalls {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<PermissionsCall> for SelfAuthorizedVaultCalls {
        fn from(value: PermissionsCall) -> Self {
            Self::Permissions(value)
        }
    }
    impl ::core::convert::From<SetPermissionsCall> for SelfAuthorizedVaultCalls {
        fn from(value: SetPermissionsCall) -> Self {
            Self::SetPermissions(value)
        }
    }
    impl ::core::convert::From<SweepFundsCall> for SelfAuthorizedVaultCalls {
        fn from(value: SweepFundsCall) -> Self {
            Self::SweepFunds(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for SelfAuthorizedVaultCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `WAITING_PERIOD` function with signature `WAITING_PERIOD()` and selector `0x6f85c7e4`
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
    pub struct WaitingPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `WITHDRAWAL_LIMIT` function with signature `WITHDRAWAL_LIMIT()` and selector `0x82ee0d1d`
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
    pub struct WithdrawalLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `execute` function with signature `execute(address,bytes)` and selector `0x1cff79cd`
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
    pub struct ExecuteReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getActionId` function with signature `getActionId(bytes4,address,address)` and selector `0x3e152499`
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
    pub struct GetActionIdReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `initialized` function with signature `initialized()` and selector `0x158ef93e`
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
    pub struct InitializedReturn(pub bool);
    ///Container type for all return fields from the `permissions` function with signature `permissions(bytes32)` and selector `0xb4d2388f`
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
    pub struct PermissionsReturn(pub bool);
}
