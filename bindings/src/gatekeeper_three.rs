pub use gatekeeper_three::*;
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
pub mod gatekeeper_three {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowEntrance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowEntrance"),
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
                    ::std::borrow::ToOwned::to_owned("construct0r"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("construct0r"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createTrick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createTrick"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("entrant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("entrant"),
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
                    ::std::borrow::ToOwned::to_owned("getAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_password"),
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
                    ::std::borrow::ToOwned::to_owned("trick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trick"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract SimpleTrick"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GATEKEEPERTHREE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x06r\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x7FW`\x005`\xE0\x1C\x80c\xB9\x96nV\x11a\0NW\x80c\xB9\x96nV\x14a\x019W\x80c\xC9`\x17N\x14a\x01_W\x80c\xE9}\xCBb\x14a\x01\x7FW\x80c\xF7\xED\xF0\x99\x14a\x01\x94W`\0\x80\xFD[\x80c\x0C=\x9F\xED\x14a\0\x8BW\x80ci\r\xA2\xB2\x14a\0\xC1W\x80c\x8D\xA5\xCB[\x14a\0\xF9W\x80c\x9D\xB3\x1Dw\x14a\x01\x19W`\0\x80\xFD[6a\0\x86W\0[`\0\x80\xFD[4\x80\x15a\0\x97W`\0\x80\xFD[P`\x01Ta\0\xAC\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xCDW`\0\x80\xFD[P`\x02Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB8V[4\x80\x15a\x01\x05W`\0\x80\xFD[P`\0Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01%W`\0\x80\xFD[P`\x01Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01EW`\0\x80\xFD[Pa\x01]`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90UV[\0[4\x80\x15a\x01kW`\0\x80\xFD[Pa\x01]a\x01z6`\x04a\x03\x8AV[a\x01\xA9V[4\x80\x15a\x01\x8BW`\0\x80\xFD[Pa\x01]a\x024V[4\x80\x15a\x01\xA0W`\0\x80\xFD[Pa\x01]a\x02\xD3V[`\x02T`@Qc\x9EK.G`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9EK.G\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x18\x91\x90a\x03\xA3V[\x15a\x021W`\x01\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02KW`\0\x80\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x162\x03a\x02bW`\0\x80\xFD[`\x01\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15\x14a\x02|W`\0\x80\xFD[f\x03\x8D~\xA4\xC6\x80\0G\x11\x80\x15a\x02\xB9WP`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x90f\x03\x8D~\xA4\xC6\x80\0\x90\x82\x81\x81\x81\x85\x88\x83\xF1\x15\x93PPPP[\x15a\x02\xD1W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x162\x17\x90U[V[0`@Qa\x02\xE0\x90a\x03}V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x03\x0CW=`\0\x80>=`\0\xFD[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80QcL\xBB\x81\x7F`\xE0\x1B\x81R\x90QcL\xBB\x81\x7F\x91`\x04\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x03cW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03wW=`\0\x80>=`\0\xFD[PPPPV[a\x02p\x80a\x03\xCD\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x03\x9CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\xB5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xC5W`\0\x80\xFD[\x93\x92PPPV\xFE`\x80`@RB`\x02U4\x80\x15a\0\x14W`\0\x80\xFD[P`@Qa\x02p8\x03\x80a\x02p\x839\x81\x01`@\x81\x90Ra\x003\x91a\0XV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x88V[`\0` \x82\x84\x03\x12\x15a\0jW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x81W`\0\x80\xFD[\x93\x92PPPV[a\x01\xD9\x80a\0\x97`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80cL\xBB\x81\x7F\x14a\0\\W\x80ci\r\xA2\xB2\x14a\0uW\x80c\x9EK.G\x14a\0\xA5W\x80c\xB7\xE0\x02\x91\x14a\0\xC8W\x80c\xD4\xB89\x92\x14a\0\xD0W[`\0\x80\xFD[a\0s`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x160\x17\x90UV[\0[`\x01Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB8a\0\xB36`\x04a\x01\x8AV[a\0\xE3V[`@Q\x90\x15\x15\x81R` \x01a\0\x9CV[a\0sa\x01\x01V[`\0Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0`\x02T\x82\x03a\0\xF6WP`\x01\x91\x90PV[PPB`\x02U`\0\x90V[03\x14\x80\x15a\x01\x1BWP`\x01T`\x01`\x01`\xA0\x1B\x03\x160\x14\x15[\x15a\x01\x88W`\0T`\x02T`@Qcd\xB0\x0B\xA7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xC9`\x17N\x91a\x01U\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x83W=`\0\x80>=`\0\xFD[PPPP[V[`\0` \x82\x84\x03\x12\x15a\x01\x9CW`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 |\xE1\xDD\xE9~\xE2\xAEK6\xB4\xC7[\xD4f\x03\xAC\x85\x17\xFC\x80\x80\xA5\xC6\xAEg\xB7\xC3}\x88(S\xA8dsolcC\0\x08\x14\x003\xA2dipfsX\"\x12 \xB1\xBB\x15\xA3\xC8Oe\xB0\x050<L\xA0k0\xC4\xB7\xFF\xE8\xB2\xCF\x08\x8F\xE2_U\x11\n\xDF\x85\x87\xDBdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static GATEKEEPERTHREE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x7FW`\x005`\xE0\x1C\x80c\xB9\x96nV\x11a\0NW\x80c\xB9\x96nV\x14a\x019W\x80c\xC9`\x17N\x14a\x01_W\x80c\xE9}\xCBb\x14a\x01\x7FW\x80c\xF7\xED\xF0\x99\x14a\x01\x94W`\0\x80\xFD[\x80c\x0C=\x9F\xED\x14a\0\x8BW\x80ci\r\xA2\xB2\x14a\0\xC1W\x80c\x8D\xA5\xCB[\x14a\0\xF9W\x80c\x9D\xB3\x1Dw\x14a\x01\x19W`\0\x80\xFD[6a\0\x86W\0[`\0\x80\xFD[4\x80\x15a\0\x97W`\0\x80\xFD[P`\x01Ta\0\xAC\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xCDW`\0\x80\xFD[P`\x02Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB8V[4\x80\x15a\x01\x05W`\0\x80\xFD[P`\0Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01%W`\0\x80\xFD[P`\x01Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01EW`\0\x80\xFD[Pa\x01]`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90UV[\0[4\x80\x15a\x01kW`\0\x80\xFD[Pa\x01]a\x01z6`\x04a\x03\x8AV[a\x01\xA9V[4\x80\x15a\x01\x8BW`\0\x80\xFD[Pa\x01]a\x024V[4\x80\x15a\x01\xA0W`\0\x80\xFD[Pa\x01]a\x02\xD3V[`\x02T`@Qc\x9EK.G`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9EK.G\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x18\x91\x90a\x03\xA3V[\x15a\x021W`\x01\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02KW`\0\x80\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x162\x03a\x02bW`\0\x80\xFD[`\x01\x80T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15\x14a\x02|W`\0\x80\xFD[f\x03\x8D~\xA4\xC6\x80\0G\x11\x80\x15a\x02\xB9WP`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x90f\x03\x8D~\xA4\xC6\x80\0\x90\x82\x81\x81\x81\x85\x88\x83\xF1\x15\x93PPPP[\x15a\x02\xD1W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x162\x17\x90U[V[0`@Qa\x02\xE0\x90a\x03}V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x03\x0CW=`\0\x80>=`\0\xFD[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80QcL\xBB\x81\x7F`\xE0\x1B\x81R\x90QcL\xBB\x81\x7F\x91`\x04\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x03cW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03wW=`\0\x80>=`\0\xFD[PPPPV[a\x02p\x80a\x03\xCD\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x03\x9CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\xB5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xC5W`\0\x80\xFD[\x93\x92PPPV\xFE`\x80`@RB`\x02U4\x80\x15a\0\x14W`\0\x80\xFD[P`@Qa\x02p8\x03\x80a\x02p\x839\x81\x01`@\x81\x90Ra\x003\x91a\0XV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x88V[`\0` \x82\x84\x03\x12\x15a\0jW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x81W`\0\x80\xFD[\x93\x92PPPV[a\x01\xD9\x80a\0\x97`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80cL\xBB\x81\x7F\x14a\0\\W\x80ci\r\xA2\xB2\x14a\0uW\x80c\x9EK.G\x14a\0\xA5W\x80c\xB7\xE0\x02\x91\x14a\0\xC8W\x80c\xD4\xB89\x92\x14a\0\xD0W[`\0\x80\xFD[a\0s`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x160\x17\x90UV[\0[`\x01Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB8a\0\xB36`\x04a\x01\x8AV[a\0\xE3V[`@Q\x90\x15\x15\x81R` \x01a\0\x9CV[a\0sa\x01\x01V[`\0Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0`\x02T\x82\x03a\0\xF6WP`\x01\x91\x90PV[PPB`\x02U`\0\x90V[03\x14\x80\x15a\x01\x1BWP`\x01T`\x01`\x01`\xA0\x1B\x03\x160\x14\x15[\x15a\x01\x88W`\0T`\x02T`@Qcd\xB0\x0B\xA7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xC9`\x17N\x91a\x01U\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x83W=`\0\x80>=`\0\xFD[PPPP[V[`\0` \x82\x84\x03\x12\x15a\x01\x9CW`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 |\xE1\xDD\xE9~\xE2\xAEK6\xB4\xC7[\xD4f\x03\xAC\x85\x17\xFC\x80\x80\xA5\xC6\xAEg\xB7\xC3}\x88(S\xA8dsolcC\0\x08\x14\x003\xA2dipfsX\"\x12 \xB1\xBB\x15\xA3\xC8Oe\xB0\x050<L\xA0k0\xC4\xB7\xFF\xE8\xB2\xCF\x08\x8F\xE2_U\x11\n\xDF\x85\x87\xDBdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static GATEKEEPERTHREE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GatekeeperThree<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GatekeeperThree<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GatekeeperThree<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GatekeeperThree<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GatekeeperThree<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GatekeeperThree))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GatekeeperThree<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GATEKEEPERTHREE_ABI.clone(),
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
                GATEKEEPERTHREE_ABI.clone(),
                GATEKEEPERTHREE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowEntrance` (0x0c3d9fed) function
        pub fn allow_entrance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([12, 61, 159, 237], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `construct0r` (0xb9966e56) function
        pub fn construct_0r(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 150, 110, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createTrick` (0xf7edf099) function
        pub fn create_trick(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 237, 240, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enter` (0xe97dcb62) function
        pub fn enter(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 125, 203, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `entrant` (0x9db31d77) function
        pub fn entrant(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([157, 179, 29, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllowance` (0xc960174e) function
        pub fn get_allowance(
            &self,
            password: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 96, 23, 78], password)
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
        ///Calls the contract's `trick` (0x690da2b2) function
        pub fn trick(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([105, 13, 162, 178], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GatekeeperThree<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `allowEntrance` function with signature `allowEntrance()` and selector `0x0c3d9fed`
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
    #[ethcall(name = "allowEntrance", abi = "allowEntrance()")]
    pub struct AllowEntranceCall;
    ///Container type for all input parameters for the `construct0r` function with signature `construct0r()` and selector `0xb9966e56`
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
    #[ethcall(name = "construct0r", abi = "construct0r()")]
    pub struct Construct0RCall;
    ///Container type for all input parameters for the `createTrick` function with signature `createTrick()` and selector `0xf7edf099`
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
    #[ethcall(name = "createTrick", abi = "createTrick()")]
    pub struct CreateTrickCall;
    ///Container type for all input parameters for the `enter` function with signature `enter()` and selector `0xe97dcb62`
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
    #[ethcall(name = "enter", abi = "enter()")]
    pub struct EnterCall;
    ///Container type for all input parameters for the `entrant` function with signature `entrant()` and selector `0x9db31d77`
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
    #[ethcall(name = "entrant", abi = "entrant()")]
    pub struct EntrantCall;
    ///Container type for all input parameters for the `getAllowance` function with signature `getAllowance(uint256)` and selector `0xc960174e`
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
    #[ethcall(name = "getAllowance", abi = "getAllowance(uint256)")]
    pub struct GetAllowanceCall {
        pub password: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `trick` function with signature `trick()` and selector `0x690da2b2`
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
    #[ethcall(name = "trick", abi = "trick()")]
    pub struct TrickCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GatekeeperThreeCalls {
        AllowEntrance(AllowEntranceCall),
        Construct0R(Construct0RCall),
        CreateTrick(CreateTrickCall),
        Enter(EnterCall),
        Entrant(EntrantCall),
        GetAllowance(GetAllowanceCall),
        Owner(OwnerCall),
        Trick(TrickCall),
    }
    impl ::ethers::core::abi::AbiDecode for GatekeeperThreeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AllowEntranceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllowEntrance(decoded));
            }
            if let Ok(decoded)
                = <Construct0RCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Construct0R(decoded));
            }
            if let Ok(decoded)
                = <CreateTrickCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateTrick(decoded));
            }
            if let Ok(decoded)
                = <EnterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Enter(decoded));
            }
            if let Ok(decoded)
                = <EntrantCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Entrant(decoded));
            }
            if let Ok(decoded)
                = <GetAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllowance(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <TrickCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Trick(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GatekeeperThreeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllowEntrance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Construct0R(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateTrick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Enter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Entrant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Trick(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GatekeeperThreeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowEntrance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Construct0R(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateTrick(element) => ::core::fmt::Display::fmt(element, f),
                Self::Enter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Entrant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Trick(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowEntranceCall> for GatekeeperThreeCalls {
        fn from(value: AllowEntranceCall) -> Self {
            Self::AllowEntrance(value)
        }
    }
    impl ::core::convert::From<Construct0RCall> for GatekeeperThreeCalls {
        fn from(value: Construct0RCall) -> Self {
            Self::Construct0R(value)
        }
    }
    impl ::core::convert::From<CreateTrickCall> for GatekeeperThreeCalls {
        fn from(value: CreateTrickCall) -> Self {
            Self::CreateTrick(value)
        }
    }
    impl ::core::convert::From<EnterCall> for GatekeeperThreeCalls {
        fn from(value: EnterCall) -> Self {
            Self::Enter(value)
        }
    }
    impl ::core::convert::From<EntrantCall> for GatekeeperThreeCalls {
        fn from(value: EntrantCall) -> Self {
            Self::Entrant(value)
        }
    }
    impl ::core::convert::From<GetAllowanceCall> for GatekeeperThreeCalls {
        fn from(value: GetAllowanceCall) -> Self {
            Self::GetAllowance(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for GatekeeperThreeCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<TrickCall> for GatekeeperThreeCalls {
        fn from(value: TrickCall) -> Self {
            Self::Trick(value)
        }
    }
    ///Container type for all return fields from the `allowEntrance` function with signature `allowEntrance()` and selector `0x0c3d9fed`
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
    pub struct AllowEntranceReturn(pub bool);
    ///Container type for all return fields from the `entrant` function with signature `entrant()` and selector `0x9db31d77`
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
    pub struct EntrantReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `trick` function with signature `trick()` and selector `0x690da2b2`
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
    pub struct TrickReturn(pub ::ethers::core::types::Address);
}
