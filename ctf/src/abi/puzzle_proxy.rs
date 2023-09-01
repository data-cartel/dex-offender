pub use puzzle_proxy::*;
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
pub mod puzzle_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_implementation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_initData"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("approveNewAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approveNewAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_expectedAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("pendingAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("proposeNewAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposeNewAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newAdmin"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newImplementation",
                                    ),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PUZZLEPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\x958\x03\x80a\x07\x95\x839\x81\x01`@\x81\x90Ra\0/\x91a\x02\x06V[\x81\x81a\0\\`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\x02\xD6V[`\0\x80Q` a\x07u\x839\x81Q\x91R\x14a\0xWa\0xa\x02\xFDV[a\0\x81\x82a\x01\x1DV[\x80Q\x15a\0\xF2W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\0\xA2\x91\x90a\x03\x13V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\0\xDDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xE2V[``\x91P[PP\x90P\x80a\0\xF0W`\0\x80\xFD[P[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x93\x90\x93\x17\x90\x92UPa\x03/\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x01\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FUpgradeableProxy: new implementa`D\x82\x01R\x7Ftion is not a contract\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Q` a\x07u\x839\x81Q\x91RUV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xC7W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x01\xFDW\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xE5V[PP`\0\x91\x01RV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02\x1BW`\0\x80\xFD[a\x02$\x84a\x01\xB0V[\x92Pa\x022` \x85\x01a\x01\xB0V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x02OW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02cW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x02uWa\x02ua\x01\xCCV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\x9DWa\x02\x9Da\x01\xCCV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x02\xB6W`\0\x80\xFD[a\x02\xC7\x83` \x83\x01` \x88\x01a\x01\xE2V[\x80\x95PPPPPP\x92P\x92P\x92V[\x81\x81\x03\x81\x81\x11\x15a\x02\xF7WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Qa\x03%\x81\x84` \x87\x01a\x01\xE2V[\x91\x90\x91\x01\x92\x91PPV[a\x047\x80a\x03>`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0NW`\x005`\xE0\x1C\x80c&x\"G\x14a\0eW\x80c6Y\xCF\xE6\x14a\0\xA1W\x80c\xA0/\xCC\n\x14a\0\xC1W\x80c\xA67gF\x14a\0\xE1W\x80c\xF8Q\xA4@\x14a\x01\x1EWa\0]V[6a\0]Wa\0[a\x01>V[\0[a\0[a\x01>V[4\x80\x15a\0qW`\0\x80\xFD[P`\0Ta\0\x85\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xADW`\0\x80\xFD[Pa\0[a\0\xBC6`\x04a\x03\xD1V[a\x01pV[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0[a\0\xDC6`\x04a\x03\xD1V[a\x01\xD5V[4\x80\x15a\0\xEDW`\0\x80\xFD[Pa\0[a\0\xFC6`\x04a\x03\xD1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x01*W`\0\x80\xFD[P`\x01Ta\0\x85\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01na\x01i\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x90V[a\x02\xD3V[V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv!\xB0\xB662\xB9\x104\xB9\x9077\xBA\x10:42\x900\xB26\xB4\xB7`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xD2\x81a\x02\xF7V[PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv!\xB0\xB662\xB9\x104\xB9\x9077\xBA\x10:42\x900\xB26\xB4\xB7`I\x1B`D\x82\x01R`d\x01a\x01\xC0V[`\0T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a\x02\xAEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FExpected new admin by the curren`D\x82\x01R\x7Ft admin is not the pending admin`d\x82\x01R`\x84\x01a\x01\xC0V[P`\0T`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x02\xF2W=`\0\xF3[=`\0\xFD[a\x03\0\x81a\x037V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x03\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FUpgradeableProxy: new implementa`D\x82\x01Ru\x1D\x1A[\xDB\x88\x1A\\\xC8\x1B\x9B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`R\x1B`d\x82\x01R`\x84\x01a\x01\xC0V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCUV[`\0` \x82\x84\x03\x12\x15a\x03\xE3W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xFAW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 Z\xF0#\n\xFD\x81N-#\xF2d\x819\xB9D\x18R\xEE\xA6\xDE\x03\r^\xC1\x06\xA1uN\x16\xB39\x1AdsolcC\0\x08\x15\x0036\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC";
    /// The bytecode of the contract.
    pub static PUZZLEPROXY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0NW`\x005`\xE0\x1C\x80c&x\"G\x14a\0eW\x80c6Y\xCF\xE6\x14a\0\xA1W\x80c\xA0/\xCC\n\x14a\0\xC1W\x80c\xA67gF\x14a\0\xE1W\x80c\xF8Q\xA4@\x14a\x01\x1EWa\0]V[6a\0]Wa\0[a\x01>V[\0[a\0[a\x01>V[4\x80\x15a\0qW`\0\x80\xFD[P`\0Ta\0\x85\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xADW`\0\x80\xFD[Pa\0[a\0\xBC6`\x04a\x03\xD1V[a\x01pV[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0[a\0\xDC6`\x04a\x03\xD1V[a\x01\xD5V[4\x80\x15a\0\xEDW`\0\x80\xFD[Pa\0[a\0\xFC6`\x04a\x03\xD1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x01*W`\0\x80\xFD[P`\x01Ta\0\x85\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01na\x01i\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x90V[a\x02\xD3V[V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv!\xB0\xB662\xB9\x104\xB9\x9077\xBA\x10:42\x900\xB26\xB4\xB7`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xD2\x81a\x02\xF7V[PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv!\xB0\xB662\xB9\x104\xB9\x9077\xBA\x10:42\x900\xB26\xB4\xB7`I\x1B`D\x82\x01R`d\x01a\x01\xC0V[`\0T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a\x02\xAEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FExpected new admin by the curren`D\x82\x01R\x7Ft admin is not the pending admin`d\x82\x01R`\x84\x01a\x01\xC0V[P`\0T`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x02\xF2W=`\0\xF3[=`\0\xFD[a\x03\0\x81a\x037V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x03\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FUpgradeableProxy: new implementa`D\x82\x01Ru\x1D\x1A[\xDB\x88\x1A\\\xC8\x1B\x9B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`R\x1B`d\x82\x01R`\x84\x01a\x01\xC0V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCUV[`\0` \x82\x84\x03\x12\x15a\x03\xE3W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xFAW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 Z\xF0#\n\xFD\x81N-#\xF2d\x819\xB9D\x18R\xEE\xA6\xDE\x03\r^\xC1\x06\xA1uN\x16\xB39\x1AdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static PUZZLEPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PuzzleProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PuzzleProxy<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for PuzzleProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for PuzzleProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for PuzzleProxy<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PuzzleProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PuzzleProxy<M> {
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
                PUZZLEPROXY_ABI.clone(),
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
                PUZZLEPROXY_ABI.clone(),
                PUZZLEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `admin` (0xf851a440)
        /// function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveNewAdmin`
        /// (0xa02fcc0a) function
        pub fn approve_new_admin(
            &self,
            expected_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 47, 204, 10], expected_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingAdmin` (0x26782247)
        /// function
        pub fn pending_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposeNewAdmin`
        /// (0xa6376746) function
        pub fn propose_new_admin(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 55, 103, 70], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x3659cfe6)
        /// function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
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
        /// Returns an `Event` builder for all the events of
        /// this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for PuzzleProxy<M>
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `admin` function with signature `admin()` and
    /// selector `0xf851a440`
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
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the
    /// `approveNewAdmin` function with signature
    /// `approveNewAdmin(address)` and selector `0xa02fcc0a`
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
    #[ethcall(name = "approveNewAdmin", abi = "approveNewAdmin(address)")]
    pub struct ApproveNewAdminCall {
        pub expected_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `pendingAdmin` function with signature
    /// `pendingAdmin()` and selector `0x26782247`
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
    #[ethcall(name = "pendingAdmin", abi = "pendingAdmin()")]
    pub struct PendingAdminCall;
    ///Container type for all input parameters for the
    /// `proposeNewAdmin` function with signature
    /// `proposeNewAdmin(address)` and selector `0xa6376746`
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
    #[ethcall(name = "proposeNewAdmin", abi = "proposeNewAdmin(address)")]
    pub struct ProposeNewAdminCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `upgradeTo` function with signature
    /// `upgradeTo(address)` and selector `0x3659cfe6`
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
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum PuzzleProxyCalls {
        Admin(AdminCall),
        ApproveNewAdmin(ApproveNewAdminCall),
        PendingAdmin(PendingAdminCall),
        ProposeNewAdmin(ProposeNewAdminCall),
        UpgradeTo(UpgradeToCall),
    }
    impl ::ethers::core::abi::AbiDecode for PuzzleProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) =
                <ApproveNewAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ApproveNewAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <ProposeNewAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ProposeNewAdmin(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PuzzleProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Admin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveNewAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposeNewAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PuzzleProxyCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveNewAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposeNewAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradeTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminCall> for PuzzleProxyCalls {
        fn from(value: AdminCall) -> Self { Self::Admin(value) }
    }
    impl ::core::convert::From<ApproveNewAdminCall> for PuzzleProxyCalls {
        fn from(value: ApproveNewAdminCall) -> Self {
            Self::ApproveNewAdmin(value)
        }
    }
    impl ::core::convert::From<PendingAdminCall> for PuzzleProxyCalls {
        fn from(value: PendingAdminCall) -> Self { Self::PendingAdmin(value) }
    }
    impl ::core::convert::From<ProposeNewAdminCall> for PuzzleProxyCalls {
        fn from(value: ProposeNewAdminCall) -> Self {
            Self::ProposeNewAdmin(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for PuzzleProxyCalls {
        fn from(value: UpgradeToCall) -> Self { Self::UpgradeTo(value) }
    }
    ///Container type for all return fields from the
    /// `admin` function with signature `admin()` and
    /// selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `pendingAdmin` function with signature
    /// `pendingAdmin()` and selector `0x26782247`
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
    pub struct PendingAdminReturn(pub ::ethers::core::types::Address);
}
