pub use free_rider_recovery::*;
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
pub mod free_rider_recovery {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(
                ::ethers::core::abi::ethabi::Constructor {
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_beneficiary"
                            ),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_nft"),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                    ],
                },
            ),
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("onERC721Received"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                256usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },
                    ],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::string::String::new(),
                        kind:
                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                4usize,
                            ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes4"),
                        ),
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability:
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallerNotNFT"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CallerNotNFT"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokenID"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "InvalidTokenID"
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "tokenId"
                                ),
                                kind:
                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughFunding"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "NotEnoughFunding"
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OriginNotBeneficiary"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "OriginNotBeneficiary",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StillNotOwningToken"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "StillNotOwningToken",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "tokenId"
                                ),
                                kind:
                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FREERIDERRECOVERY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R`@Qa\x06\x878\x03\x80a\x06\x87\x839\x81\x01`@\x81\x90Ra\0\"\x91a\0\xDDV[`\x01`\0U4h\x02p\x80\x1D\x94l\x94\0\0\x14a\0PW`@Qc;\xD4\xD2\xE9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x80R\x81\x16`\xA0\x81\x90R`@Qc\xA2,\xB4e`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`$\x82\x01Rc\xA2,\xB4e\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xA2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xB6W=`\0\x80>=`\0\xFD[PPPPPPa\x01\x10V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xD8W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0\xF0W`\0\x80\xFD[a\0\xF9\x83a\0\xC1V[\x91Pa\x01\x07` \x84\x01a\0\xC1V[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x05Ma\x01:`\09`\0\x81\x81`u\x01Ra\x01F\x01R`\0`\xBE\x01Ra\x05M`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x15\x0Bz\x02\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x03\xECV[a\0`V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0a\0ja\x02GV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xB3W`@Qcq(?k`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[2`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xFCW`@Qc^\x1BgG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x83\x11\x15a\x01&W`@Qc\xB7\xC8\x82\xEB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@Qc1\xA9\x10\x8F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ccR!\x1E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB1\x91\x90a\x04\xCCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01\xDBW`@QcJ\x12\xC5E`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x01\x1DV[`\x01`\0\x81Ta\x01\xEA\x90a\x04\xF0V[\x91\x82\x90UP`\x06\x03a\x02,W`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x02\x0C\x91\x90a\x04\xCCV[\x90Pa\x02*`\x01`\x01`\xA0\x1B\x03\x82\x16h\x02p\x80\x1D\x94l\x94\0\0a\x02\xA0V[P[Pc\n\x85\xBD\x01`\xE1\x1Ba\x02?`\x01`\0UV[\x94\x93PPPPV[`\x02`\0T\x03a\x02\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x01\x1DV[`\x02`\0UV[\x80G\x10\x15a\x02\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x01\x1DV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03BV[``\x91P[PP\x90P\x80a\x03\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\x1DV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xD3W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x04\x02W`\0\x80\xFD[\x845a\x04\r\x81a\x03\xBEV[\x93P` \x85\x015a\x04\x1D\x81a\x03\xBEV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04AW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x04UW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04gWa\x04ga\x03\xD6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\x8FWa\x04\x8Fa\x03\xD6V[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x04\xA8W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a\x04\xDEW`\0\x80\xFD[\x81Qa\x04\xE9\x81a\x03\xBEV[\x93\x92PPPV[`\0`\x01\x82\x01a\x05\x10WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x1CJ\tB\x88\x83X\xFD\xA8\xFC'\x80|_J\x9B\xDB\xE9\x1BB\xFA\xE7\x97\xE3\xD1\x92\x8B\x83=\x89\x9C\xF6dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static FREERIDERRECOVERY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x15\x0Bz\x02\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x03\xECV[a\0`V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0a\0ja\x02GV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xB3W`@Qcq(?k`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[2`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xFCW`@Qc^\x1BgG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x83\x11\x15a\x01&W`@Qc\xB7\xC8\x82\xEB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@Qc1\xA9\x10\x8F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ccR!\x1E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB1\x91\x90a\x04\xCCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01\xDBW`@QcJ\x12\xC5E`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x01\x1DV[`\x01`\0\x81Ta\x01\xEA\x90a\x04\xF0V[\x91\x82\x90UP`\x06\x03a\x02,W`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x02\x0C\x91\x90a\x04\xCCV[\x90Pa\x02*`\x01`\x01`\xA0\x1B\x03\x82\x16h\x02p\x80\x1D\x94l\x94\0\0a\x02\xA0V[P[Pc\n\x85\xBD\x01`\xE1\x1Ba\x02?`\x01`\0UV[\x94\x93PPPPV[`\x02`\0T\x03a\x02\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x01\x1DV[`\x02`\0UV[\x80G\x10\x15a\x02\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x01\x1DV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03BV[``\x91P[PP\x90P\x80a\x03\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\x1DV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xD3W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x04\x02W`\0\x80\xFD[\x845a\x04\r\x81a\x03\xBEV[\x93P` \x85\x015a\x04\x1D\x81a\x03\xBEV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04AW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x04UW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04gWa\x04ga\x03\xD6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\x8FWa\x04\x8Fa\x03\xD6V[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x04\xA8W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a\x04\xDEW`\0\x80\xFD[\x81Qa\x04\xE9\x81a\x03\xBEV[\x93\x92PPPV[`\0`\x01\x82\x01a\x05\x10WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x1CJ\tB\x88\x83X\xFD\xA8\xFC'\x80|_J\x9B\xDB\xE9\x1BB\xFA\xE7\x97\xE3\xD1\x92\x8B\x83=\x89\x9C\xF6dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static FREERIDERRECOVERY_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FreeRiderRecovery<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FreeRiderRecovery<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for FreeRiderRecovery<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for FreeRiderRecovery<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for FreeRiderRecovery<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FreeRiderRecovery))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FreeRiderRecovery<M> {
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
                FREERIDERRECOVERY_ABI.clone(),
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
                FREERIDERRECOVERY_ABI.clone(),
                FREERIDERRECOVERY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `onERC721Received`
        /// (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, token_id, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for FreeRiderRecovery<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerNotNFT` with signature
    /// `CallerNotNFT()` and selector `0xe2507ed6`
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
    #[etherror(name = "CallerNotNFT", abi = "CallerNotNFT()")]
    pub struct CallerNotNFT;
    ///Custom Error type `InvalidTokenID` with signature
    /// `InvalidTokenID(uint256)` and selector `0xb7c882eb`
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
    #[etherror(name = "InvalidTokenID", abi = "InvalidTokenID(uint256)")]
    pub struct InvalidTokenID {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `NotEnoughFunding` with signature
    /// `NotEnoughFunding()` and selector `0xef534ba4`
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
    #[etherror(name = "NotEnoughFunding", abi = "NotEnoughFunding()")]
    pub struct NotEnoughFunding;
    ///Custom Error type `OriginNotBeneficiary` with
    /// signature `OriginNotBeneficiary()` and selector
    /// `0xbc36ce8e`
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
    #[etherror(name = "OriginNotBeneficiary", abi = "OriginNotBeneficiary()")]
    pub struct OriginNotBeneficiary;
    ///Custom Error type `StillNotOwningToken` with
    /// signature `StillNotOwningToken(uint256)` and
    /// selector `0x94258a8a`
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
        name = "StillNotOwningToken",
        abi = "StillNotOwningToken(uint256)"
    )]
    pub struct StillNotOwningToken {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's custom
    /// errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum FreeRiderRecoveryErrors {
        CallerNotNFT(CallerNotNFT),
        InvalidTokenID(InvalidTokenID),
        NotEnoughFunding(NotEnoughFunding),
        OriginNotBeneficiary(OriginNotBeneficiary),
        StillNotOwningToken(StillNotOwningToken),
        /// The standard solidity revert string, with
        /// selector Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FreeRiderRecoveryErrors {
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
                <CallerNotNFT as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CallerNotNFT(decoded));
            }
            if let Ok(decoded) =
                <InvalidTokenID as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidTokenID(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughFunding as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NotEnoughFunding(decoded));
            }
            if let Ok(decoded) =
                <OriginNotBeneficiary as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OriginNotBeneficiary(decoded));
            }
            if let Ok(decoded) =
                <StillNotOwningToken as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::StillNotOwningToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FreeRiderRecoveryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerNotNFT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokenID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughFunding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OriginNotBeneficiary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StillNotOwningToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => {
                    ::ethers::core::abi::AbiEncode::encode(s)
                }
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FreeRiderRecoveryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerNotNFT as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidTokenID as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughFunding as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OriginNotBeneficiary as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StillNotOwningToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FreeRiderRecoveryErrors {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::CallerNotNFT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTokenID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughFunding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OriginNotBeneficiary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StillNotOwningToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FreeRiderRecoveryErrors {
        fn from(value: String) -> Self { Self::RevertString(value) }
    }
    impl ::core::convert::From<CallerNotNFT> for FreeRiderRecoveryErrors {
        fn from(value: CallerNotNFT) -> Self { Self::CallerNotNFT(value) }
    }
    impl ::core::convert::From<InvalidTokenID> for FreeRiderRecoveryErrors {
        fn from(value: InvalidTokenID) -> Self { Self::InvalidTokenID(value) }
    }
    impl ::core::convert::From<NotEnoughFunding> for FreeRiderRecoveryErrors {
        fn from(value: NotEnoughFunding) -> Self {
            Self::NotEnoughFunding(value)
        }
    }
    impl ::core::convert::From<OriginNotBeneficiary> for FreeRiderRecoveryErrors {
        fn from(value: OriginNotBeneficiary) -> Self {
            Self::OriginNotBeneficiary(value)
        }
    }
    impl ::core::convert::From<StillNotOwningToken> for FreeRiderRecoveryErrors {
        fn from(value: StillNotOwningToken) -> Self {
            Self::StillNotOwningToken(value)
        }
    }
    ///Container type for all input parameters for the
    /// `onERC721Received` function with signature
    /// `onERC721Received(address,address,uint256,bytes)`
    /// and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall {
        pub p0: ::ethers::core::types::Address,
        pub p1: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the
    /// `onERC721Received` function with signature
    /// `onERC721Received(address,address,uint256,bytes)`
    /// and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
}
