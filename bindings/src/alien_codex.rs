pub use alien_codex::*;
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
pub mod alien_codex {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("codex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("codex"),
                            inputs: ::std::vec![
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("contact"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contact"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOwner"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("makeContact"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makeContact"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("record"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("record"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_content"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("retract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("retract"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revise"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("i"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_content"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ALIENCODEX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x04\xE1\x80a\0%`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0fW\x80c\x8D\xA5\xCB[\x14a\0\xFCW\x80c\x8F2\xD5\x9B\x14a\x01 W\x80c\x94\xBDui\x14a\x01(W\x80c\xB5\xC6E\xBD\x14a\x01WW\x80c\xF2\xFD\xE3\x8B\x14a\x01tWa\0\x9EV[\x80c\x039\xF3\0\x14a\0\xA3W\x80c2\x8BR\xCB\x14a\0\xC8W\x80c3\xA8\xC4Z\x14a\0\xD0W\x80cG\xF5{2\x14a\0\xECW\x80cqP\x18\xA6\x14a\0\xF4W[`\0\x80\xFD[a\0\xC6`\x04\x806\x03`@\x81\x10\x15a\0\xB9W`\0\x80\xFD[P\x805\x90` \x015a\x01\x9AV[\0[a\0\xC6a\x01\xCBV[a\0\xD8a\x01\xE0V[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xC6a\x01\xF0V[a\0\xC6a\x02\x19V[a\x01\x04a\x02\xBCV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xD8a\x02\xCCV[a\x01E`\x04\x806\x03` \x81\x10\x15a\x01>W`\0\x80\xFD[P5a\x02\xDDV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xC6`\x04\x806\x03` \x81\x10\x15a\x01mW`\0\x80\xFD[P5a\x02\xFBV[a\0\xC6`\x04\x806\x03` \x81\x10\x15a\x01\x8AW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x03BV[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x01\xADW\xFE[\x80`\x01\x83\x81T\x81\x10a\x01\xBBW\xFE[`\0\x91\x82R` \x90\x91 \x01UPPV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x02\x03W\xFE[`\x01\x80T\x90a\x02\x16\x90`\0\x19\x83\x01a\x04?V[PV[a\x02!a\x02\xCCV[a\x02rW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x16[\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x90V[`\x01\x81\x81T\x81\x10a\x02\xEAW\xFE[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x03\x0EW\xFE[`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01UV[a\x03Ja\x02\xCCV[a\x03\x9BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x02\x16\x81`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80a\x04\x87`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x81T\x81\x83U\x81\x81\x11\x15a\x04cW`\0\x83\x81R` \x90 a\x04c\x91\x81\x01\x90\x83\x01a\x04hV[PPPV[a\x02\xC9\x91\x90[\x80\x82\x11\x15a\x04\x82W`\0\x81U`\x01\x01a\x04nV[P\x90V\xFEOwnable: new owner is the zero address\xA2ebzzr1X 4\xB6/\xC7\x90Aq\x01\xD9[\xB53\xD9G\x80\xA99jv.\xC7\x01\x07\x0F\"q\xFFY\x93Y\xC2xdsolcC\0\x05\x11\x002";
    /// The bytecode of the contract.
    pub static ALIENCODEX_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0fW\x80c\x8D\xA5\xCB[\x14a\0\xFCW\x80c\x8F2\xD5\x9B\x14a\x01 W\x80c\x94\xBDui\x14a\x01(W\x80c\xB5\xC6E\xBD\x14a\x01WW\x80c\xF2\xFD\xE3\x8B\x14a\x01tWa\0\x9EV[\x80c\x039\xF3\0\x14a\0\xA3W\x80c2\x8BR\xCB\x14a\0\xC8W\x80c3\xA8\xC4Z\x14a\0\xD0W\x80cG\xF5{2\x14a\0\xECW\x80cqP\x18\xA6\x14a\0\xF4W[`\0\x80\xFD[a\0\xC6`\x04\x806\x03`@\x81\x10\x15a\0\xB9W`\0\x80\xFD[P\x805\x90` \x015a\x01\x9AV[\0[a\0\xC6a\x01\xCBV[a\0\xD8a\x01\xE0V[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xC6a\x01\xF0V[a\0\xC6a\x02\x19V[a\x01\x04a\x02\xBCV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xD8a\x02\xCCV[a\x01E`\x04\x806\x03` \x81\x10\x15a\x01>W`\0\x80\xFD[P5a\x02\xDDV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\xC6`\x04\x806\x03` \x81\x10\x15a\x01mW`\0\x80\xFD[P5a\x02\xFBV[a\0\xC6`\x04\x806\x03` \x81\x10\x15a\x01\x8AW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x03BV[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x01\xADW\xFE[\x80`\x01\x83\x81T\x81\x10a\x01\xBBW\xFE[`\0\x91\x82R` \x90\x91 \x01UPPV[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x02\x03W\xFE[`\x01\x80T\x90a\x02\x16\x90`\0\x19\x83\x01a\x04?V[PV[a\x02!a\x02\xCCV[a\x02rW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x16[\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14\x90V[`\x01\x81\x81T\x81\x10a\x02\xEAW\xFE[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\0T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x03\x0EW\xFE[`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01UV[a\x03Ja\x02\xCCV[a\x03\x9BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x02\x16\x81`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80a\x04\x87`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x81T\x81\x83U\x81\x81\x11\x15a\x04cW`\0\x83\x81R` \x90 a\x04c\x91\x81\x01\x90\x83\x01a\x04hV[PPPV[a\x02\xC9\x91\x90[\x80\x82\x11\x15a\x04\x82W`\0\x81U`\x01\x01a\x04nV[P\x90V\xFEOwnable: new owner is the zero address\xA2ebzzr1X 4\xB6/\xC7\x90Aq\x01\xD9[\xB53\xD9G\x80\xA99jv.\xC7\x01\x07\x0F\"q\xFFY\x93Y\xC2xdsolcC\0\x05\x11\x002";
    /// The deployed bytecode of the contract.
    pub static ALIENCODEX_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AlienCodex<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AlienCodex<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AlienCodex<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AlienCodex<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AlienCodex<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AlienCodex)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AlienCodex<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ALIENCODEX_ABI.clone(),
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
                ALIENCODEX_ABI.clone(),
                ALIENCODEX_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `codex` (0x94bd7569) function
        pub fn codex(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([148, 189, 117, 105], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contact` (0x33a8c45a) function
        pub fn contact(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([51, 168, 196, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOwner` (0x8f32d59b) function
        pub fn is_owner(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([143, 50, 213, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makeContact` (0x328b52cb) function
        pub fn make_contact(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 139, 82, 203], ())
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
        ///Calls the contract's `record` (0xb5c645bd) function
        pub fn record(
            &self,
            content: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 198, 69, 189], content)
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
        ///Calls the contract's `retract` (0x47f57b32) function
        pub fn retract(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 245, 123, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revise` (0x0339f300) function
        pub fn revise(
            &self,
            i: ::ethers::core::types::U256,
            content: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 57, 243, 0], (i, content))
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AlienCodex<M> {
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
    ///Container type for all input parameters for the `codex` function with signature `codex(uint256)` and selector `0x94bd7569`
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
    #[ethcall(name = "codex", abi = "codex(uint256)")]
    pub struct CodexCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `contact` function with signature `contact()` and selector `0x33a8c45a`
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
    #[ethcall(name = "contact", abi = "contact()")]
    pub struct ContactCall;
    ///Container type for all input parameters for the `isOwner` function with signature `isOwner()` and selector `0x8f32d59b`
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
    #[ethcall(name = "isOwner", abi = "isOwner()")]
    pub struct IsOwnerCall;
    ///Container type for all input parameters for the `makeContact` function with signature `makeContact()` and selector `0x328b52cb`
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
    #[ethcall(name = "makeContact", abi = "makeContact()")]
    pub struct MakeContactCall;
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
    ///Container type for all input parameters for the `record` function with signature `record(bytes32)` and selector `0xb5c645bd`
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
    #[ethcall(name = "record", abi = "record(bytes32)")]
    pub struct RecordCall {
        pub content: [u8; 32],
    }
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
    ///Container type for all input parameters for the `retract` function with signature `retract()` and selector `0x47f57b32`
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
    #[ethcall(name = "retract", abi = "retract()")]
    pub struct RetractCall;
    ///Container type for all input parameters for the `revise` function with signature `revise(uint256,bytes32)` and selector `0x0339f300`
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
    #[ethcall(name = "revise", abi = "revise(uint256,bytes32)")]
    pub struct ReviseCall {
        pub i: ::ethers::core::types::U256,
        pub content: [u8; 32],
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AlienCodexCalls {
        Codex(CodexCall),
        Contact(ContactCall),
        IsOwner(IsOwnerCall),
        MakeContact(MakeContactCall),
        Owner(OwnerCall),
        Record(RecordCall),
        RenounceOwnership(RenounceOwnershipCall),
        Retract(RetractCall),
        Revise(ReviseCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for AlienCodexCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CodexCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Codex(decoded));
            }
            if let Ok(decoded)
                = <ContactCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Contact(decoded));
            }
            if let Ok(decoded)
                = <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded)
                = <MakeContactCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MakeContact(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RecordCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Record(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <RetractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Retract(decoded));
            }
            if let Ok(decoded)
                = <ReviseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Revise(decoded));
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
    impl ::ethers::core::abi::AbiEncode for AlienCodexCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Codex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Contact(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MakeContact(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Record(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Retract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Revise(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AlienCodexCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Codex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Contact(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakeContact(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Record(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Retract(element) => ::core::fmt::Display::fmt(element, f),
                Self::Revise(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CodexCall> for AlienCodexCalls {
        fn from(value: CodexCall) -> Self {
            Self::Codex(value)
        }
    }
    impl ::core::convert::From<ContactCall> for AlienCodexCalls {
        fn from(value: ContactCall) -> Self {
            Self::Contact(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for AlienCodexCalls {
        fn from(value: IsOwnerCall) -> Self {
            Self::IsOwner(value)
        }
    }
    impl ::core::convert::From<MakeContactCall> for AlienCodexCalls {
        fn from(value: MakeContactCall) -> Self {
            Self::MakeContact(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AlienCodexCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RecordCall> for AlienCodexCalls {
        fn from(value: RecordCall) -> Self {
            Self::Record(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AlienCodexCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RetractCall> for AlienCodexCalls {
        fn from(value: RetractCall) -> Self {
            Self::Retract(value)
        }
    }
    impl ::core::convert::From<ReviseCall> for AlienCodexCalls {
        fn from(value: ReviseCall) -> Self {
            Self::Revise(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AlienCodexCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `codex` function with signature `codex(uint256)` and selector `0x94bd7569`
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
    pub struct CodexReturn(pub [u8; 32]);
    ///Container type for all return fields from the `contact` function with signature `contact()` and selector `0x33a8c45a`
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
    pub struct ContactReturn(pub bool);
    ///Container type for all return fields from the `isOwner` function with signature `isOwner()` and selector `0x8f32d59b`
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
    pub struct IsOwnerReturn(pub bool);
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
}
