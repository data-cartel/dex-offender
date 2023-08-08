pub use owner_manager::*;
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
pub mod owner_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OWNERMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\ng\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xA0\xE6~+\x11a\0[W\x80c\xA0\xE6~+\x14a\0\xD2W\x80c\xE3\x18\xB5+\x14a\0\xE7W\x80c\xE7R5\xB8\x14a\0\xFAW\x80c\xF8\xDC]\xD9\x14a\x01\x0BW`\0\x80\xFD[\x80c\rX/\x13\x14a\0\x82W\x80c/T\xBFn\x14a\0\x97W\x80ciN\x80\xC3\x14a\0\xBFW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x08\\V[a\x01\x1EV[\0[a\0\xAAa\0\xA56`\x04a\x08\x86V[a\x02\x97V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\xCD6`\x04a\x08\xA8V[a\x02\xD2V[a\0\xDAa\x03\x88V[`@Qa\0\xB6\x91\x90a\x08\xC1V[a\0\x95a\0\xF56`\x04a\t\x0EV[a\x04xV[`\x02T`@Q\x90\x81R` \x01a\0\xB6V[a\0\x95a\x01\x196`\x04a\tQV[a\x06}V[a\x01&a\x08\x07V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x01HWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x01]WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x15[a\x01\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x15a\x01\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`D\x82\x01R`d\x01a\x01yV[`\0` \x81\x90R\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}\x80T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x80\x85R`@\x85 \x80T\x92\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x01\x80\x85R\x83T\x90\x91\x16\x90\x91\x17\x90\x91U\x80T\x91a\x02@\x83a\t\xC2V[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x02T\x14a\x02\x93Wa\x02\x93\x81a\x02\xD2V[PPV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x80\x15\x90a\x02\xCCWP`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x15\x15[\x92\x91PPV[a\x02\xDAa\x08\x07V[`\x01T\x81\x11\x15a\x03\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS201`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01\x81\x10\x15a\x03MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x01yV[`\x02\x81\x90U`@Q\x81\x81R\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[```\0`\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xA7Wa\x03\xA7a\t\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xD0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\0\x90\x81R` \x81\x90R\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}T\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14a\x04pW\x80\x83\x83\x81Q\x81\x10a\x041Wa\x041a\t\xF1V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16`\0\x90\x81R\x91\x82\x90R`@\x90\x91 T\x16\x81a\x04h\x81a\t\xC2V[\x92PPa\x04\rV[P\x90\x92\x91PPV[a\x04\x80a\x08\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x04\xA2WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x04\xB7WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[a\x04\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x15a\x05#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x05EWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x05aW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x16\x90\x83\x16\x14a\x05\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x87\x87\x16\x80\x86R\x83\x86 \x80T\x92\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90U\x96\x8A\x16\x85R\x82\x85 \x80T\x82\x16\x90\x97\x17\x90\x96U\x92\x84\x90R\x82T\x90\x94\x16\x90\x91U\x91Q\x90\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[a\x06\x85a\x08\x07V[\x80`\x01\x80Ta\x06\x94\x91\x90a\n\x07V[\x10\x15a\x06\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS201`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x06\xECWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x07\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x16\x90\x83\x16\x14a\x07\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 \x80T\x88\x86\x16\x84R\x91\x83 \x80T\x92\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x94U\x91\x81R\x82T\x90\x91\x16\x90\x91U`\x01\x80T\x91a\x07\xAF\x83a\n\x1AV[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x02T\x14a\x08\x02Wa\x08\x02\x81a\x02\xD2V[PPPV[30\x14a\x08>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08WW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x08oW`\0\x80\xFD[a\x08x\x83a\x08@V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x98W`\0\x80\xFD[a\x08\xA1\x82a\x08@V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xBAW`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\t\x02W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x08\xDDV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\t#W`\0\x80\xFD[a\t,\x84a\x08@V[\x92Pa\t:` \x85\x01a\x08@V[\x91Pa\tH`@\x85\x01a\x08@V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\tfW`\0\x80\xFD[a\to\x84a\x08@V[\x92Pa\t}` \x85\x01a\x08@V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[` \x80\x82R`\x05\x90\x82\x01RdGS203`\xD8\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t\xD4Wa\t\xD4a\t\xACV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xCCWa\x02\xCCa\t\xACV[`\0\x81a\n)Wa\n)a\t\xACV[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 3\x05\x13K_\x7F3\x1B\x94MgK\xB0\x9D+IF\xE8\x9A7\xA2\xDB22~\x01.:)\xBF\xE7\xA4dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static OWNERMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xA0\xE6~+\x11a\0[W\x80c\xA0\xE6~+\x14a\0\xD2W\x80c\xE3\x18\xB5+\x14a\0\xE7W\x80c\xE7R5\xB8\x14a\0\xFAW\x80c\xF8\xDC]\xD9\x14a\x01\x0BW`\0\x80\xFD[\x80c\rX/\x13\x14a\0\x82W\x80c/T\xBFn\x14a\0\x97W\x80ciN\x80\xC3\x14a\0\xBFW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x08\\V[a\x01\x1EV[\0[a\0\xAAa\0\xA56`\x04a\x08\x86V[a\x02\x97V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\xCD6`\x04a\x08\xA8V[a\x02\xD2V[a\0\xDAa\x03\x88V[`@Qa\0\xB6\x91\x90a\x08\xC1V[a\0\x95a\0\xF56`\x04a\t\x0EV[a\x04xV[`\x02T`@Q\x90\x81R` \x01a\0\xB6V[a\0\x95a\x01\x196`\x04a\tQV[a\x06}V[a\x01&a\x08\x07V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x01HWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x01]WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x15[a\x01\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x15a\x01\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`D\x82\x01R`d\x01a\x01yV[`\0` \x81\x90R\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}\x80T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x80\x85R`@\x85 \x80T\x92\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x01\x80\x85R\x83T\x90\x91\x16\x90\x91\x17\x90\x91U\x80T\x91a\x02@\x83a\t\xC2V[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x02T\x14a\x02\x93Wa\x02\x93\x81a\x02\xD2V[PPV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x80\x15\x90a\x02\xCCWP`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x15\x15[\x92\x91PPV[a\x02\xDAa\x08\x07V[`\x01T\x81\x11\x15a\x03\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS201`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01\x81\x10\x15a\x03MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x01yV[`\x02\x81\x90U`@Q\x81\x81R\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[```\0`\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xA7Wa\x03\xA7a\t\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xD0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\0\x90\x81R` \x81\x90R\x7F\xAD\xA5\x011\"\xD3\x95\xBA<Tw\"\x83\xFB\x06\x9B\x10B`V\xEF\x8C\xA5GP\xCB\x9B\xB5R\xA5\x9E}T\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14a\x04pW\x80\x83\x83\x81Q\x81\x10a\x041Wa\x041a\t\xF1V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16`\0\x90\x81R\x91\x82\x90R`@\x90\x91 T\x16\x81a\x04h\x81a\t\xC2V[\x92PPa\x04\rV[P\x90\x92\x91PPV[a\x04\x80a\x08\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x04\xA2WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x04\xB7WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[a\x04\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x15a\x05#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x05EWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x05aW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x16\x90\x83\x16\x14a\x05\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T\x87\x87\x16\x80\x86R\x83\x86 \x80T\x92\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90U\x96\x8A\x16\x85R\x82\x85 \x80T\x82\x16\x90\x97\x17\x90\x96U\x92\x84\x90R\x82T\x90\x94\x16\x90\x91U\x91Q\x90\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[a\x06\x85a\x08\x07V[\x80`\x01\x80Ta\x06\x94\x91\x90a\n\x07V[\x10\x15a\x06\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS201`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x06\xECWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x07\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01y\x90a\t\x8DV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x16\x90\x83\x16\x14a\x07\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 \x80T\x88\x86\x16\x84R\x91\x83 \x80T\x92\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x94U\x91\x81R\x82T\x90\x91\x16\x90\x91U`\x01\x80T\x91a\x07\xAF\x83a\n\x1AV[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x90` \x01`@Q\x80\x91\x03\x90\xA1\x80`\x02T\x14a\x08\x02Wa\x08\x02\x81a\x02\xD2V[PPPV[30\x14a\x08>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01a\x01yV[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08WW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x08oW`\0\x80\xFD[a\x08x\x83a\x08@V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x98W`\0\x80\xFD[a\x08\xA1\x82a\x08@V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xBAW`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\t\x02W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x08\xDDV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\t#W`\0\x80\xFD[a\t,\x84a\x08@V[\x92Pa\t:` \x85\x01a\x08@V[\x91Pa\tH`@\x85\x01a\x08@V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\tfW`\0\x80\xFD[a\to\x84a\x08@V[\x92Pa\t}` \x85\x01a\x08@V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[` \x80\x82R`\x05\x90\x82\x01RdGS203`\xD8\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t\xD4Wa\t\xD4a\t\xACV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xCCWa\x02\xCCa\t\xACV[`\0\x81a\n)Wa\n)a\t\xACV[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 3\x05\x13K_\x7F3\x1B\x94MgK\xB0\x9D+IF\xE8\x9A7\xA2\xDB22~\x01.:)\xBF\xE7\xA4dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static OWNERMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct OwnerManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OwnerManager<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for OwnerManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for OwnerManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for OwnerManager<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OwnerManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OwnerManager<M> {
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
                OWNERMANAGER_ABI.clone(),
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
                OWNERMANAGER_ABI.clone(),
                OWNERMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        /// Returns an `Event` builder for all the events of
        /// this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for OwnerManager<M>
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
    #[ethevent(name = "RemovedOwner", abi = "RemovedOwner(address)")]
    pub struct RemovedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum OwnerManagerEvents {
        AddedOwnerFilter(AddedOwnerFilter),
        ChangedThresholdFilter(ChangedThresholdFilter),
        RemovedOwnerFilter(RemovedOwnerFilter),
    }
    impl ::ethers::contract::EthLogDecode for OwnerManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedOwnerFilter::decode_log(log) {
                return Ok(OwnerManagerEvents::AddedOwnerFilter(decoded));
            }
            if let Ok(decoded) = ChangedThresholdFilter::decode_log(log) {
                return Ok(OwnerManagerEvents::ChangedThresholdFilter(decoded));
            }
            if let Ok(decoded) = RemovedOwnerFilter::decode_log(log) {
                return Ok(OwnerManagerEvents::RemovedOwnerFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OwnerManagerEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::AddedOwnerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedThresholdFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovedOwnerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddedOwnerFilter> for OwnerManagerEvents {
        fn from(value: AddedOwnerFilter) -> Self {
            Self::AddedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<ChangedThresholdFilter> for OwnerManagerEvents {
        fn from(value: ChangedThresholdFilter) -> Self {
            Self::ChangedThresholdFilter(value)
        }
    }
    impl ::core::convert::From<RemovedOwnerFilter> for OwnerManagerEvents {
        fn from(value: RemovedOwnerFilter) -> Self {
            Self::RemovedOwnerFilter(value)
        }
    }
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
    pub enum OwnerManagerCalls {
        AddOwnerWithThreshold(AddOwnerWithThresholdCall),
        ChangeThreshold(ChangeThresholdCall),
        GetOwners(GetOwnersCall),
        GetThreshold(GetThresholdCall),
        IsOwner(IsOwnerCall),
        RemoveOwner(RemoveOwnerCall),
        SwapOwner(SwapOwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for OwnerManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddOwnerWithThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddOwnerWithThreshold(decoded));
            }
            if let Ok(decoded) =
                <ChangeThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ChangeThreshold(decoded));
            }
            if let Ok(decoded) =
                <GetOwnersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOwners(decoded));
            }
            if let Ok(decoded) =
                <GetThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetThreshold(decoded));
            }
            if let Ok(decoded) =
                <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded) =
                <RemoveOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RemoveOwner(decoded));
            }
            if let Ok(decoded) =
                <SwapOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OwnerManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddOwnerWithThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOwners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OwnerManagerCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::AddOwnerWithThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOwners(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddOwnerWithThresholdCall> for OwnerManagerCalls {
        fn from(value: AddOwnerWithThresholdCall) -> Self {
            Self::AddOwnerWithThreshold(value)
        }
    }
    impl ::core::convert::From<ChangeThresholdCall> for OwnerManagerCalls {
        fn from(value: ChangeThresholdCall) -> Self {
            Self::ChangeThreshold(value)
        }
    }
    impl ::core::convert::From<GetOwnersCall> for OwnerManagerCalls {
        fn from(value: GetOwnersCall) -> Self { Self::GetOwners(value) }
    }
    impl ::core::convert::From<GetThresholdCall> for OwnerManagerCalls {
        fn from(value: GetThresholdCall) -> Self { Self::GetThreshold(value) }
    }
    impl ::core::convert::From<IsOwnerCall> for OwnerManagerCalls {
        fn from(value: IsOwnerCall) -> Self { Self::IsOwner(value) }
    }
    impl ::core::convert::From<RemoveOwnerCall> for OwnerManagerCalls {
        fn from(value: RemoveOwnerCall) -> Self { Self::RemoveOwner(value) }
    }
    impl ::core::convert::From<SwapOwnerCall> for OwnerManagerCalls {
        fn from(value: SwapOwnerCall) -> Self { Self::SwapOwner(value) }
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
}
