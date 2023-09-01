pub use truster_lender_pool::*;
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
pub mod truster_lender_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract DamnValuableToken",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("flashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract DamnValuableToken",
                                        ),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RepayFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RepayFailed"),
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
    pub static TRUSTERLENDERPOOL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07>8\x03\x80a\x07>\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0uV[`\0` \x82\x84\x03\x12\x15a\0WW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0nW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x06\x9Ba\0\xA3`\09`\0\x81\x81`h\x01R\x81\x81`\xC4\x01R\x81\x81a\x01`\x01Ra\x022\x01Ra\x06\x9B`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\xAB\x19\xE0\xC0\x14a\0;W\x80c\xFC\x0CTj\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x05\x1CV[a\0\xA2V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0ZV[`\0a\0\xACa\x02\xDCV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x017\x91\x90a\x05\xB7V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`$\x82\x01\x8A\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xCF\x91\x90a\x05\xD0V[Pa\x02\x1A\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x01`\x01`\xA0\x1B\x03\x89\x16\x92\x91PPa\x03:V[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA5\x91\x90a\x05\xB7V[\x10\x15a\x02\xC4W`@Qc\x9Ep:\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x91PPa\x02\xD3`\x01`\0UV[\x95\x94PPPPPV[`\x02`\0T\x03a\x033W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[``a\x03~\x83\x83`\0`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FAddress: low-level call failed\0\0\x81RPa\x03\x85V[\x93\x92PPPV[``\x82G\x10\x15a\x03\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03*V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x04\x02\x91\x90a\x06\x16V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04?W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04DV[``\x91P[P\x91P\x91Pa\x04U\x87\x83\x83\x87a\x04bV[\x92PPP[\x94\x93PPPPV[``\x83\x15a\x04\xD1W\x82Q`\0\x03a\x04\xCAW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03*V[P\x81a\x04ZV[a\x04Z\x83\x83\x81Q\x15a\x04\xE6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03*\x91\x90a\x062V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x17W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x054W`\0\x80\xFD[\x855\x94Pa\x05D` \x87\x01a\x05\0V[\x93Pa\x05R`@\x87\x01a\x05\0V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05oW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x05\x83W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\x92W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x05\xA4W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xE2W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03~W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x06\rW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xF5V[PP`\0\x91\x01RV[`\0\x82Qa\x06(\x81\x84` \x87\x01a\x05\xF2V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x06Q\x81`@\x85\x01` \x87\x01a\x05\xF2V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x92H^\xD24n|\xBBW?G\x1A\xA5\x1D\xD0V@z\x97v\x10Q\xE9#\xB4\r\x0B\xAA\xFE\xAD\xF2\x88dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static TRUSTERLENDERPOOL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\xAB\x19\xE0\xC0\x14a\0;W\x80c\xFC\x0CTj\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x05\x1CV[a\0\xA2V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0ZV[`\0a\0\xACa\x02\xDCV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x017\x91\x90a\x05\xB7V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`$\x82\x01\x8A\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xCF\x91\x90a\x05\xD0V[Pa\x02\x1A\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x01`\x01`\xA0\x1B\x03\x89\x16\x92\x91PPa\x03:V[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA5\x91\x90a\x05\xB7V[\x10\x15a\x02\xC4W`@Qc\x9Ep:\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x91PPa\x02\xD3`\x01`\0UV[\x95\x94PPPPPV[`\x02`\0T\x03a\x033W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[``a\x03~\x83\x83`\0`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FAddress: low-level call failed\0\0\x81RPa\x03\x85V[\x93\x92PPPV[``\x82G\x10\x15a\x03\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03*V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x04\x02\x91\x90a\x06\x16V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04?W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04DV[``\x91P[P\x91P\x91Pa\x04U\x87\x83\x83\x87a\x04bV[\x92PPP[\x94\x93PPPPV[``\x83\x15a\x04\xD1W\x82Q`\0\x03a\x04\xCAW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03*V[P\x81a\x04ZV[a\x04Z\x83\x83\x81Q\x15a\x04\xE6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03*\x91\x90a\x062V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x17W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x054W`\0\x80\xFD[\x855\x94Pa\x05D` \x87\x01a\x05\0V[\x93Pa\x05R`@\x87\x01a\x05\0V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05oW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x05\x83W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\x92W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x05\xA4W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xE2W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03~W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x06\rW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xF5V[PP`\0\x91\x01RV[`\0\x82Qa\x06(\x81\x84` \x87\x01a\x05\xF2V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x06Q\x81`@\x85\x01` \x87\x01a\x05\xF2V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x92H^\xD24n|\xBBW?G\x1A\xA5\x1D\xD0V@z\x97v\x10Q\xE9#\xB4\r\x0B\xAA\xFE\xAD\xF2\x88dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static TRUSTERLENDERPOOL_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct TrusterLenderPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TrusterLenderPool<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for TrusterLenderPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for TrusterLenderPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for TrusterLenderPool<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TrusterLenderPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TrusterLenderPool<M> {
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
                TRUSTERLENDERPOOL_ABI.clone(),
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
                TRUSTERLENDERPOOL_ABI.clone(),
                TRUSTERLENDERPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `flashLoan` (0xab19e0c0)
        /// function
        pub fn flash_loan(
            &self,
            amount: ::ethers::core::types::U256,
            borrower: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [171, 25, 224, 192],
                    (amount, borrower, target, data),
                )
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
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for TrusterLenderPool<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `RepayFailed` with signature
    /// `RepayFailed()` and selector `0x9e703a05`
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
    #[etherror(name = "RepayFailed", abi = "RepayFailed()")]
    pub struct RepayFailed;
    ///Container type for all input parameters for the
    /// `flashLoan` function with signature
    /// `flashLoan(uint256,address,address,bytes)` and
    /// selector `0xab19e0c0`
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
        name = "flashLoan",
        abi = "flashLoan(uint256,address,address,bytes)"
    )]
    pub struct FlashLoanCall {
        pub amount: ::ethers::core::types::U256,
        pub borrower: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
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
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum TrusterLenderPoolCalls {
        FlashLoan(FlashLoanCall),
        Token(TokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for TrusterLenderPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Token(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TrusterLenderPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TrusterLenderPoolCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::FlashLoan(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FlashLoanCall> for TrusterLenderPoolCalls {
        fn from(value: FlashLoanCall) -> Self { Self::FlashLoan(value) }
    }
    impl ::core::convert::From<TokenCall> for TrusterLenderPoolCalls {
        fn from(value: TokenCall) -> Self { Self::Token(value) }
    }
    ///Container type for all return fields from the
    /// `flashLoan` function with signature
    /// `flashLoan(uint256,address,address,bytes)` and
    /// selector `0xab19e0c0`
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
    pub struct FlashLoanReturn(pub bool);
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
}
