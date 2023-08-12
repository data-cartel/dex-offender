pub use wallet_deployer::*;
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
pub mod wallet_deployer {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gem"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("can"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("can"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("u"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
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
                    ::std::borrow::ToOwned::to_owned("chief"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chief"),
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
                    ::std::borrow::ToOwned::to_owned("copy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("copy"),
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
                    ::std::borrow::ToOwned::to_owned("drop"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("drop"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wat"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fact"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fact"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IGnosisSafeProxyFactory",
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
                    ::std::borrow::ToOwned::to_owned("gem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gem"),
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
                    ::std::borrow::ToOwned::to_owned("mom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mom"),
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
                    ::std::borrow::ToOwned::to_owned("pay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pay"),
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
                    ::std::borrow::ToOwned::to_owned("rule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_mom"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Boom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Boom"),
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
    pub static WALLETDEPLOYER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R3`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[P`@Qa\x06\xC48\x03\x80a\x06\xC4\x839\x81\x01`@\x81\x90Ra\x003\x91a\0DV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0Ra\0tV[`\0` \x82\x84\x03\x12\x15a\0VW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0mW`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Qa\x06\x1Da\0\xA7`\09`\0\x81\x81a\x01j\x01Ra\x02\xAB\x01R`\0\x81\x81a\x01\x91\x01Ra\x03\x85\x01Ra\x06\x1D`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x92W`\x005`\xE0\x1C\x80c`rj\xBB\x11a\0fW\x80c`rj\xBB\x14a\x01\x1AW\x80cs\xE3\xCE\x80\x14a\x015W\x80c{\x04Xe\x14a\x01PW\x80c{\xD2\xBE\xA7\x14a\x01eW\x80c\xFF\xD8d\xD3\x14a\x01\x8CW`\0\x80\xFD[\x80b\xB5#\x95\x14a\0\x97W\x80c\x1B\x92e\xB8\x14a\0\xC7W\x80c=\xF7\xC8m\x14a\0\xE4W\x80cE8\xC4\xEB\x14a\0\xF7W[`\0\x80\xFD[a\0\xAAa\0\xA56`\x04a\x04$V[a\x01\xB3V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x81V[`@Q\x90\x81R` \x01a\0\xBEV[`\0Ta\0\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\na\x01\x056`\x04a\x04\xEDV[a\x03&V[`@Q\x90\x15\x15\x81R` \x01a\0\xBEV[a\0\xAAs4\xCF\xACdo0\x13V\xFA\xA8\xB2\x1E\x94\"~5\x83\xFE?_\x81V[a\0\xAAsv\xE2\xCF\xC1\xF5\xFA\x8Fj[?\xC4\xC8\xF4x\x8F\x01\x16\x86\x1F\x9B\x81V[a\x01ca\x01^6`\x04a\x05&V[a\x03zV[\0[a\0\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qca\xB6\x9A\xBD`\xE0\x1B\x81R`\0\x90sv\xE2\xCF\xC1\xF5\xFA\x8Fj[?\xC4\xC8\xF4x\x8F\x01\x16\x86\x1F\x9B\x90ca\xB6\x9A\xBD\x90a\x02\x03\x90s4\xCF\xACdo0\x13V\xFA\xA8\xB2\x1E\x94\"~5\x83\xFE?_\x90\x86\x90`\x04\x01a\x05JV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02F\x91\x90a\x05\xA8V[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15\x90a\x02jWPa\x02h3\x82a\x03&V[\x15[\x15a\x02\x88W`@Qc\x1F\t\xFE\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03 \x91\x90a\x05\xC5V[P\x91\x90PV[`\0\x80T\x80;a\x032W\0[`@Q`D\x81\x01`@RcE8\xC4\xEB`\xE0\x1B\x81R\x84`\x04\x82\x01R\x83`$\x82\x01R` \x81`D\x83\x85Z\xFAa\x03aW\0[\x80Q\x15=\x15\x19\x16\x15a\x03oW\0[P`\x01\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x03\xB9WP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15[\x80a\x03\xCEWP`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[\x15a\x03\xECW`@Qc\x1F\t\xFE\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x046W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04NW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x04bW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04tWa\x04ta\x04\x0EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\x9CWa\x04\x9Ca\x04\x0EV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x04\xB5W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xEAW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\0W`\0\x80\xFD[\x825a\x05\x0B\x81a\x04\xD5V[\x91P` \x83\x015a\x05\x1B\x81a\x04\xD5V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x058W`\0\x80\xFD[\x815a\x05C\x81a\x04\xD5V[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x05\x86W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x05jV[P`\0``\x82\x86\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\xBAW`\0\x80\xFD[\x81Qa\x05C\x81a\x04\xD5V[`\0` \x82\x84\x03\x12\x15a\x05\xD7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05CW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x94I\xFBe7\x08\xAF\x93\x1Dq\x19s\x04\x99\xED\xB4\x98XC\xEB\xA4#\xE2\xB9\x962\xCF\xAD\xD9\xC7B?dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static WALLETDEPLOYER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x92W`\x005`\xE0\x1C\x80c`rj\xBB\x11a\0fW\x80c`rj\xBB\x14a\x01\x1AW\x80cs\xE3\xCE\x80\x14a\x015W\x80c{\x04Xe\x14a\x01PW\x80c{\xD2\xBE\xA7\x14a\x01eW\x80c\xFF\xD8d\xD3\x14a\x01\x8CW`\0\x80\xFD[\x80b\xB5#\x95\x14a\0\x97W\x80c\x1B\x92e\xB8\x14a\0\xC7W\x80c=\xF7\xC8m\x14a\0\xE4W\x80cE8\xC4\xEB\x14a\0\xF7W[`\0\x80\xFD[a\0\xAAa\0\xA56`\x04a\x04$V[a\x01\xB3V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x81V[`@Q\x90\x81R` \x01a\0\xBEV[`\0Ta\0\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\na\x01\x056`\x04a\x04\xEDV[a\x03&V[`@Q\x90\x15\x15\x81R` \x01a\0\xBEV[a\0\xAAs4\xCF\xACdo0\x13V\xFA\xA8\xB2\x1E\x94\"~5\x83\xFE?_\x81V[a\0\xAAsv\xE2\xCF\xC1\xF5\xFA\x8Fj[?\xC4\xC8\xF4x\x8F\x01\x16\x86\x1F\x9B\x81V[a\x01ca\x01^6`\x04a\x05&V[a\x03zV[\0[a\0\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qca\xB6\x9A\xBD`\xE0\x1B\x81R`\0\x90sv\xE2\xCF\xC1\xF5\xFA\x8Fj[?\xC4\xC8\xF4x\x8F\x01\x16\x86\x1F\x9B\x90ca\xB6\x9A\xBD\x90a\x02\x03\x90s4\xCF\xACdo0\x13V\xFA\xA8\xB2\x1E\x94\"~5\x83\xFE?_\x90\x86\x90`\x04\x01a\x05JV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02F\x91\x90a\x05\xA8V[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15\x90a\x02jWPa\x02h3\x82a\x03&V[\x15[\x15a\x02\x88W`@Qc\x1F\t\xFE\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03 \x91\x90a\x05\xC5V[P\x91\x90PV[`\0\x80T\x80;a\x032W\0[`@Q`D\x81\x01`@RcE8\xC4\xEB`\xE0\x1B\x81R\x84`\x04\x82\x01R\x83`$\x82\x01R` \x81`D\x83\x85Z\xFAa\x03aW\0[\x80Q\x15=\x15\x19\x16\x15a\x03oW\0[P`\x01\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x03\xB9WP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15[\x80a\x03\xCEWP`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[\x15a\x03\xECW`@Qc\x1F\t\xFE\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x046W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04NW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x04bW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04tWa\x04ta\x04\x0EV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\x9CWa\x04\x9Ca\x04\x0EV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x04\xB5W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xEAW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05\0W`\0\x80\xFD[\x825a\x05\x0B\x81a\x04\xD5V[\x91P` \x83\x015a\x05\x1B\x81a\x04\xD5V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x058W`\0\x80\xFD[\x815a\x05C\x81a\x04\xD5V[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x05\x86W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x05jV[P`\0``\x82\x86\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\xBAW`\0\x80\xFD[\x81Qa\x05C\x81a\x04\xD5V[`\0` \x82\x84\x03\x12\x15a\x05\xD7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05CW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x94I\xFBe7\x08\xAF\x93\x1Dq\x19s\x04\x99\xED\xB4\x98XC\xEB\xA4#\xE2\xB9\x962\xCF\xAD\xD9\xC7B?dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static WALLETDEPLOYER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct WalletDeployer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WalletDeployer<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for WalletDeployer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for WalletDeployer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for WalletDeployer<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WalletDeployer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WalletDeployer<M> {
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
                WALLETDEPLOYER_ABI.clone(),
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
                WALLETDEPLOYER_ABI.clone(),
                WALLETDEPLOYER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `can` (0x4538c4eb) function
        pub fn can(
            &self,
            u: ::ethers::core::types::Address,
            a: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([69, 56, 196, 235], (u, a))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chief` (0xffd864d3)
        /// function
        pub fn chief(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([255, 216, 100, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `copy` (0x60726abb)
        /// function
        pub fn copy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([96, 114, 106, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `drop` (0x00b52395)
        /// function
        pub fn drop(
            &self,
            wat: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([0, 181, 35, 149], wat)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fact` (0x73e3ce80)
        /// function
        pub fn fact(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([115, 227, 206, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gem` (0x7bd2bea7) function
        pub fn gem(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([123, 210, 190, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mom` (0x3df7c86d) function
        pub fn mom(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([61, 247, 200, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pay` (0x1b9265b8) function
        pub fn pay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([27, 146, 101, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rule` (0x7b045865)
        /// function
        pub fn rule(
            &self,
            mom: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 4, 88, 101], mom)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for WalletDeployer<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Boom` with signature `Boom()` and
    /// selector `0x7c27fae4`
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
    #[etherror(name = "Boom", abi = "Boom()")]
    pub struct Boom;
    ///Container type for all input parameters for the
    /// `can` function with signature `can(address,address)`
    /// and selector `0x4538c4eb`
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
    #[ethcall(name = "can", abi = "can(address,address)")]
    pub struct CanCall {
        pub u: ::ethers::core::types::Address,
        pub a: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `chief` function with signature `chief()` and
    /// selector `0xffd864d3`
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
    #[ethcall(name = "chief", abi = "chief()")]
    pub struct ChiefCall;
    ///Container type for all input parameters for the
    /// `copy` function with signature `copy()` and selector
    /// `0x60726abb`
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
    #[ethcall(name = "copy", abi = "copy()")]
    pub struct CopyCall;
    ///Container type for all input parameters for the
    /// `drop` function with signature `drop(bytes)` and
    /// selector `0x00b52395`
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
    #[ethcall(name = "drop", abi = "drop(bytes)")]
    pub struct DropCall {
        pub wat: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `fact` function with signature `fact()` and selector
    /// `0x73e3ce80`
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
    #[ethcall(name = "fact", abi = "fact()")]
    pub struct FactCall;
    ///Container type for all input parameters for the
    /// `gem` function with signature `gem()` and selector
    /// `0x7bd2bea7`
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
    #[ethcall(name = "gem", abi = "gem()")]
    pub struct GemCall;
    ///Container type for all input parameters for the
    /// `mom` function with signature `mom()` and selector
    /// `0x3df7c86d`
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
    #[ethcall(name = "mom", abi = "mom()")]
    pub struct MomCall;
    ///Container type for all input parameters for the
    /// `pay` function with signature `pay()` and selector
    /// `0x1b9265b8`
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
    #[ethcall(name = "pay", abi = "pay()")]
    pub struct PayCall;
    ///Container type for all input parameters for the
    /// `rule` function with signature `rule(address)` and
    /// selector `0x7b045865`
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
    #[ethcall(name = "rule", abi = "rule(address)")]
    pub struct RuleCall {
        pub mom: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum WalletDeployerCalls {
        Can(CanCall),
        Chief(ChiefCall),
        Copy(CopyCall),
        Drop(DropCall),
        Fact(FactCall),
        Gem(GemCall),
        Mom(MomCall),
        Pay(PayCall),
        Rule(RuleCall),
    }
    impl ::ethers::core::abi::AbiDecode for WalletDeployerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Can(decoded));
            }
            if let Ok(decoded) =
                <ChiefCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Chief(decoded));
            }
            if let Ok(decoded) =
                <CopyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Copy(decoded));
            }
            if let Ok(decoded) =
                <DropCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Drop(decoded));
            }
            if let Ok(decoded) =
                <FactCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Fact(decoded));
            }
            if let Ok(decoded) =
                <GemCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Gem(decoded));
            }
            if let Ok(decoded) =
                <MomCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Mom(decoded));
            }
            if let Ok(decoded) =
                <PayCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Pay(decoded));
            }
            if let Ok(decoded) =
                <RuleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Rule(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WalletDeployerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Can(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Chief(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Copy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Drop(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Fact(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for WalletDeployerCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Can(element) => ::core::fmt::Display::fmt(element, f),
                Self::Chief(element) => ::core::fmt::Display::fmt(element, f),
                Self::Copy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Drop(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fact(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pay(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rule(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CanCall> for WalletDeployerCalls {
        fn from(value: CanCall) -> Self { Self::Can(value) }
    }
    impl ::core::convert::From<ChiefCall> for WalletDeployerCalls {
        fn from(value: ChiefCall) -> Self { Self::Chief(value) }
    }
    impl ::core::convert::From<CopyCall> for WalletDeployerCalls {
        fn from(value: CopyCall) -> Self { Self::Copy(value) }
    }
    impl ::core::convert::From<DropCall> for WalletDeployerCalls {
        fn from(value: DropCall) -> Self { Self::Drop(value) }
    }
    impl ::core::convert::From<FactCall> for WalletDeployerCalls {
        fn from(value: FactCall) -> Self { Self::Fact(value) }
    }
    impl ::core::convert::From<GemCall> for WalletDeployerCalls {
        fn from(value: GemCall) -> Self { Self::Gem(value) }
    }
    impl ::core::convert::From<MomCall> for WalletDeployerCalls {
        fn from(value: MomCall) -> Self { Self::Mom(value) }
    }
    impl ::core::convert::From<PayCall> for WalletDeployerCalls {
        fn from(value: PayCall) -> Self { Self::Pay(value) }
    }
    impl ::core::convert::From<RuleCall> for WalletDeployerCalls {
        fn from(value: RuleCall) -> Self { Self::Rule(value) }
    }
    ///Container type for all return fields from the `can`
    /// function with signature `can(address,address)` and
    /// selector `0x4538c4eb`
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
    pub struct CanReturn(pub bool);
    ///Container type for all return fields from the
    /// `chief` function with signature `chief()` and
    /// selector `0xffd864d3`
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
    pub struct ChiefReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `copy`
    /// function with signature `copy()` and selector
    /// `0x60726abb`
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
    pub struct CopyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `drop`
    /// function with signature `drop(bytes)` and selector
    /// `0x00b52395`
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
    pub struct DropReturn {
        pub aim: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `fact`
    /// function with signature `fact()` and selector
    /// `0x73e3ce80`
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
    pub struct FactReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `gem`
    /// function with signature `gem()` and selector
    /// `0x7bd2bea7`
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
    pub struct GemReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mom`
    /// function with signature `mom()` and selector
    /// `0x3df7c86d`
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
    pub struct MomReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pay`
    /// function with signature `pay()` and selector
    /// `0x1b9265b8`
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
    pub struct PayReturn(pub ::ethers::core::types::U256);
}
