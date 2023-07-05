pub use exchange::*;
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
pub mod exchange {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_oracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("buyOne"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("buyOne"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("oracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract TrustfulOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sellOne"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sellOne"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                                        ::std::borrow::ToOwned::to_owned("contract DamnValuableNFT"),
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
                    ::std::borrow::ToOwned::to_owned("TokenBought"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenBought"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("TokenSold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenSold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("seller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPayment"),
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
                    ::std::borrow::ToOwned::to_owned("SellerNotOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SellerNotOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("TransferNotApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferNotApproved",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EXCHANGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R`@Qa*/8\x03\x80a*/\x839\x81\x01`@\x81\x90Ra\0\"\x91a\0\xCBV[`\x01`\0U`@Qa\x003\x90a\0\xBEV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\0OW=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81\x90R`@\x80Qc8\xA8\x0CS`\xE1\x1B\x81R\x90QcqP\x18\xA6\x91`\x04\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\0\x95W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xA9W=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x16`\xA0Ra\0\xFBV[a\x1EQ\x80a\x0B\xDE\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\0\xDDW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xF4W`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Qa\n\x81a\x01]`\09`\0\x81\x81`|\x01R\x81\x81a\x01;\x01Ra\x04\xCF\x01R`\0\x81\x81`\xEA\x01R\x81\x81a\x01j\x01R\x81\x81a\x02\x83\x01R\x81\x81a\x03n\x01R\x81\x81a\x049\x01R\x81\x81a\x04\xFE\x01R\x81\x81a\x06$\x01Ra\x06\x9C\x01Ra\n\x81`\0\xF3\xFE`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80ck^(\x96\x14a\0OW\x80c}\xC0\xD1\xD0\x14a\0jW\x80c\xBD\xCBX|\x14a\0\xB6W\x80c\xFC\x0CTj\x14a\0\xD8W`\0\x80\xFD[6a\0JW\0[`\0\x80\xFD[a\0Wa\x01\x0CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0vW`\0\x80\xFD[Pa\0\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0aV[4\x80\x15a\0\xC2W`\0\x80\xFD[Pa\0\xD6a\0\xD16`\x04a\x08\xCFV[a\x03PV[\0[4\x80\x15a\0\xE4W`\0\x80\xFD[Pa\0\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x01\x16a\x07XV[4`\0\x03a\x017W`@Qc\x07\x8Die`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD9e\xB3\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xEE\x91\x90\x81\x01\x90a\t\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\n\x91\x90a\t\xCFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02K\x91\x90a\n\x02V[\x90P\x804\x10\x15a\x02nW`@Qc\x07\x8Die`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc@\xD0\x97\xC3`\xE0\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c@\xD0\x97\xC3\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xF8\x91\x90a\n\x02V[\x91Pa\x03\x0734\x83\x90\x03a\x07\xB1V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R3\x91\x7F(\xCA\xB0\xD6`\xED\x8A\xED\xD6\x1A\x8C\x9D\xB0\x0B\x97\xF6\xA2\xD6~\x07\xD8w\x95\x99OD\x0B\x18\xBC_\x1A\xA3\x91\x01`@Q\x80\x91\x03\x90\xA2Pa\x03M`\x01`\0UV[\x90V[a\x03Xa\x07XV[`@Qc1\xA9\x10\x8F`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90ccR!\x1E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE1\x91\x90a\n\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x19W`@Qb\x06\x18?`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@Qc\x02\x06\x04\xBF`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x08\x18\x12\xFC\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xA4\x91\x90a\n\x1BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\xCBW`@Qc|\x15\xD3\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD9e\xB3\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x82\x91\x90\x81\x01\x90a\t\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x9E\x91\x90a\t\xCFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xDF\x91\x90a\n\x02V[\x90P\x80G\x10\x15a\x06\x02W`@Qc\x106\xB5\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06pW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x84W=`\0\x80>=`\0\xFD[PP`@Qc\x08R\xCD\x8D`\xE3\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92PcB\x96lh\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xEAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xFEW=`\0\x80>=`\0\xFD[Pa\x07\x0F\x92P3\x91P\x83\x90Pa\x07\xB1V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R3\x91\x7F5\xEA\x94\xAD\xDB\xC6-\xF2\x81\xAD\xE4\x9C\xC2\xAD\x9B=\x92Y\x16ap\xCFp)(\xB4\xDD\xC0E\xF7%n\x91\x01`@Q\x80\x91\x03\x90\xA2Pa\x07U`\x01`\0UV[PV[`\x02`\0T\x03a\x07\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x04\x10V[`\x02`\0UV[\x80G\x10\x15a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x04\x10V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08NW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08SV[``\x91P[PP\x90P\x80a\x08\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xE1W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\t\x19W\x81\x81\x01Q\x83\x82\x01R` \x01a\t\x01V[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\t4W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tLW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\t`W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\trWa\tra\x08\xE8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\t\x9AWa\t\x9Aa\x08\xE8V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\t\xB3W`\0\x80\xFD[a\t\xC4\x83` \x83\x01` \x88\x01a\x08\xFEV[\x97\x96PPPPPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t\xEE\x81`@\x85\x01` \x87\x01a\x08\xFEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\x14W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n-W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nDW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ~\xB7\xAEN\x9D\xAEx\xAB\xE4X\xA3\xC2\xBE\xFE\x8FD|\x1E\rF6\xC3\xA6L*<\\\xC9C\xB9\x05\xA8dsolcC\0\x08\x14\x003`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x11\x18[[\x95\x98[\x1DXX\x9B\x19S\x91\x95`\x8A\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x11\x15\x93\x91\x95`\xDA\x1B\x81RP\x81`\0\x90\x81b\0\0i\x91\x90b\0\x01\xCBV[P`\x01b\0\0x\x82\x82b\0\x01\xCBV[PPPb\0\0\x8C3b\0\0\x9F` \x1B` \x1CV[b\0\0\x993`\x01b\0\0\xDBV[b\0\x02\x97V[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80`\0\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01QW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01rWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xC6W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\xA1WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\xC2W\x82\x81U`\x01\x01b\0\x01\xADV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01\xE7Wb\0\x01\xE7b\0\x01&V[b\0\x01\xFF\x81b\0\x01\xF8\x84Tb\0\x01<V[\x84b\0\x01xV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x027W`\0\x84\x15b\0\x02\x1EWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\xC2V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02hW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02GV[P\x85\x82\x10\x15b\0\x02\x87W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x1B\xAA\x80b\0\x02\xA7`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xEEW`\x005`\xE0\x1C\x80cT\xD1\xF1=\x11a\x01\rW\x80c\xA2,\xB4e\x11a\0\xA0W\x80c\xD7S?\x02\x11a\0oW\x80c\xD7S?\x02\x14a\x05jW\x80c\xE9\x85\xE9\xC5\x14a\x05\x88W\x80c\xF0N(>\x14a\x05\xD1W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE4W\x80c\xFE\xE8\x1C\xF4\x14a\x05\xF7W`\0\x80\xFD[\x80c\xA2,\xB4e\x14a\x04\xF5W\x80c\xB8\x8DO\xDE\x14a\x05\x15W\x80c\xC8{V\xDD\x14a\x055W\x80c\xD59\x13\x93\x14a\x05UW`\0\x80\xFD[\x80csY\xE4\x1F\x11a\0\xDCW\x80csY\xE4\x1F\x14a\x04\x84W\x80c\x8D\xA5\xCB[\x14a\x04\xB1W\x80c\x95\xD8\x9BA\x14a\x04\xCAW\x80c\x98\xBD\xF6\xF5\x14a\x04\xDFW`\0\x80\xFD[\x80cT\xD1\xF1=\x14a\x044W\x80ccR!\x1E\x14a\x04<W\x80cp\xA0\x821\x14a\x04\\W\x80cqP\x18\xA6\x14a\x04|W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\x85W\x80cB\x84.\x0E\x11a\x01TW\x80cB\x84.\x0E\x14a\x03\xAAW\x80cB\x96lh\x14a\x03\xCAW\x80cJN\xE7\xB1\x14a\x03\xEAW\x80cQNb\xFC\x14a\x03\xFDW`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x03/W\x80c%i)b\x14a\x03OW\x80c-\xE9H\x07\x14a\x03WW\x80c@\xD0\x97\xC3\x14a\x03\x8AW`\0\x80\xFD[\x80c\x13\xA6a\xED\x11a\x01\xC1W\x80c\x13\xA6a\xED\x14a\x02\xA4W\x80c\x18:On\x14a\x02\xD2W\x80c\x1C\x10\x89?\x14a\x02\xE5W\x80c\x1C\xD6M\xF4\x14a\x02\xF8W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xF3W\x80c\x06\xFD\xDE\x03\x14a\x02(W\x80c\x08\x18\x12\xFC\x14a\x02JW\x80c\t^\xA7\xB3\x14a\x02\x82W[`\0\x80\xFD[4\x80\x15a\x01\xFFW`\0\x80\xFD[Pa\x02\x13a\x02\x0E6`\x04a\x16\0V[a\x06*V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x024W`\0\x80\xFD[Pa\x02=a\x06|V[`@Qa\x02\x1F\x91\x90a\x16mV[4\x80\x15a\x02VW`\0\x80\xFD[Pa\x02ja\x02e6`\x04a\x16\x80V[a\x07\x0EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x1FV[4\x80\x15a\x02\x8EW`\0\x80\xFD[Pa\x02\xA2a\x02\x9D6`\x04a\x16\xB5V[a\x075V[\0[4\x80\x15a\x02\xB0W`\0\x80\xFD[Pa\x02\xC4a\x02\xBF6`\x04a\x17&V[a\x08OV[`@Q\x90\x81R` \x01a\x02\x1FV[a\x02\xA2a\x02\xE06`\x04a\x16\x80V[a\x08xV[a\x02\xA2a\x02\xF36`\x04a\x16\xB5V[a\x08\x85V[4\x80\x15a\x03\x04W`\0\x80\xFD[Pa\x02\x13a\x03\x136`\x04a\x16\xB5V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x81\x16\x14\x90V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x02\xA2a\x03J6`\x04a\x17\xDFV[a\x08\x9BV[a\x02\xA2a\x08\xCDV[4\x80\x15a\x03cW`\0\x80\xFD[Pa\x02\xC4a\x03r6`\x04a\x18\x1BV[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x02\xC4a\x03\xA56`\x04a\x18\x1BV[a\t\x1DV[4\x80\x15a\x03\xB6W`\0\x80\xFD[Pa\x02\xA2a\x03\xC56`\x04a\x17\xDFV[a\tSV[4\x80\x15a\x03\xD6W`\0\x80\xFD[Pa\x02\xA2a\x03\xE56`\x04a\x16\x80V[a\tnV[a\x02\xA2a\x03\xF86`\x04a\x16\xB5V[a\t\x9CV[4\x80\x15a\x04\tW`\0\x80\xFD[Pa\x02\x13a\x04\x186`\x04a\x16\xB5V[c\x8Bx\xC6\xD8`\x0C\x90\x81R`\0\x92\x90\x92R` \x90\x91 T\x16\x15\x15\x90V[a\x02\xA2a\t\xAEV[4\x80\x15a\x04HW`\0\x80\xFD[Pa\x02ja\x04W6`\x04a\x16\x80V[a\t\xEAV[4\x80\x15a\x04hW`\0\x80\xFD[Pa\x02\xC4a\x04w6`\x04a\x18\x1BV[a\nJV[a\x02\xA2a\n\xD0V[4\x80\x15a\x04\x90W`\0\x80\xFD[Pa\x04\xA4a\x04\x9F6`\x04a\x16\x80V[a\n\xE4V[`@Qa\x02\x1F\x91\x90a\x186V[4\x80\x15a\x04\xBDW`\0\x80\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02jV[4\x80\x15a\x04\xD6W`\0\x80\xFD[Pa\x02=a\x0B\x1DV[4\x80\x15a\x04\xEBW`\0\x80\xFD[Pa\x02\xC4`\x06T\x81V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x02\xA2a\x05\x106`\x04a\x18}V[a\x0B,V[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\xA2a\x0506`\x04a\x18\xB9V[a\x0B7V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02=a\x05P6`\x04a\x16\x80V[a\x0BoV[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02\xC4`\x01\x81V[4\x80\x15a\x05vW`\0\x80\xFD[P`@Qb\x02\xA3\0\x81R` \x01a\x02\x1FV[4\x80\x15a\x05\x94W`\0\x80\xFD[Pa\x02\x13a\x05\xA36`\x04a\x19yV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x02\xA2a\x05\xDF6`\x04a\x18\x1BV[a\x0B\xE3V[a\x02\xA2a\x05\xF26`\x04a\x18\x1BV[a\x0C V[4\x80\x15a\x06\x03W`\0\x80\xFD[Pa\x02\xC4a\x06\x126`\x04a\x18\x1BV[c8\x9Au\xE1`\x0C\x90\x81R`\0\x91\x90\x91R` \x90 T\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x06[WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x06vWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\0\x80Ta\x06\x8B\x90a\x19\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xB7\x90a\x19\xACV[\x80\x15a\x07\x04W\x80`\x1F\x10a\x06\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x07\x19\x82a\x0CGV[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x07@\x82a\t\xEAV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x07\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x07\xCEWPa\x07\xCE\x813a\x05\xA3V[a\x08@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\x07\xA9V[a\x08J\x83\x83a\x0C\xA6V[PPPV[`\0\x81Q`\x05\x1B[\x80\x15a\x08rW\x82\x81\x01Q`\x01\x90\x1B\x90\x91\x17\x90`\x1F\x19\x01a\x08WV[P\x91\x90PV[a\x08\x823\x82a\r\x14V[PV[a\x08\x8Da\rcV[a\x08\x97\x82\x82a\r~V[PPV[a\x08\xA63[\x82a\r\xC9V[a\x08\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x08J\x83\x83\x83a\x0EHV[`\0b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3`\0R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D`\0\x80\xA2PV[`\0`\x01a\t*\x81a\x0F\xACV[`\x06T\x91Pa\t9\x83\x83a\x0F\xD2V[`\x06`\0\x81Ta\tH\x90a\x1A-V[\x90\x91UP\x90\x92\x91PPV[a\x08J\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x0B7V[a\tw3a\x08\xA0V[a\t\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x08\x82\x81a\x0F\xECV[a\t\xA4a\rcV[a\x08\x97\x82\x82a\r\x14V[c8\x9Au\xE1`\x0CR3`\0R`\0` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92`\0\x80\xA2V[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x06vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x07\xA9V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01Rh64\xB2\x107\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x07\xA9V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\n\xD8a\rcV[a\n\xE2`\0a\x10\x81V[V[`@Q` \x81\x01`\0\x83[\x81\x83R`\x05\x1B` \x16\x90\x91\x01\x90`\x01\x01\x83\x81\x1C\x80a\n\xEFWPP`\x1F\x19\x82\x82\x03\x01`\x05\x1C\x82R`@R\x91\x90PV[```\x01\x80Ta\x06\x8B\x90a\x19\xACV[a\x08\x973\x83\x83a\x10\xBFV[a\x0BA3\x83a\r\xC9V[a\x0B]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x19\xE0V[a\x0Bi\x84\x84\x84\x84a\x11\x8DV[PPPPV[``a\x0Bz\x82a\x0CGV[`\0a\x0B\x91`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0B\xB1W`@Q\x80` \x01`@R\x80`\0\x81RPa\x0B\xDCV[\x80a\x0B\xBB\x84a\x11\xC0V[`@Q` \x01a\x0B\xCC\x92\x91\x90a\x1ATV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[a\x0B\xEBa\rcV[c8\x9Au\xE1`\x0CR\x80`\0R` `\x0C \x80TB\x11\x15a\x0C\x13Wco^\x88\x18`\0R`\x04`\x1C\xFD[`\0\x90Ua\x08\x82\x81a\x10\x81V[a\x0C(a\rcV[\x80``\x1Ba\x0C>WctH\xFB\xAE`\0R`\x04`\x1C\xFD[a\x08\x82\x81a\x10\x81V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x08\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x0C\xDB\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x80T\x82\x81\x16\x81\x18\x92PP\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[c\x8Bx\xC6\xD8\x19T3\x14a\n\xE2Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8`\x0CR\x81`\0R` `\x0C \x81\x81T\x17\x91P\x81\x81UP\x80`\x0CQ``\x1C\x7FqZ\xD5\xCEa\xFC\x95\x95\xC7\xB4\x15(\x9DY\xCF ?#\xA9O\xA0o\x04\xAF~H\x9A\nv\xE1\xFE&`\0\x80\xA3PPV[`\0\x80a\r\xD5\x83a\t\xEAV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\x1CWP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x0E@WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x0E5\x84a\x07\x0EV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E[\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\x83V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\xA9V[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xF6\x82a\t\xEAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\x83V[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4PPPV[c\x8Bx\xC6\xD8`\x0CR3`\0R\x80` `\x0C T\x16a\x08\x82Wc\x82\xB4)\0`\0R`\x04`\x1C\xFD[a\x08\x97\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x12SV[`\0a\x0F\xF7\x82a\t\xEAV[\x90Pa\x10\x02\x82a\t\xEAV[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R`\x03\x84R\x82\x85 \x80T`\0\x19\x01\x90U\x87\x85R`\x02\x90\x93R\x81\x84 \x80T\x90\x91\x16\x90UQ\x92\x93P\x84\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x83\x90\xA4PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3UV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x11 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x11\x98\x84\x84\x84a\x0EHV[a\x11\xA4\x84\x84\x84\x84a\x12\x86V[a\x0BiW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[```\0a\x11\xCD\x83a\x13\x87V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xEDWa\x11\xEDa\x16\xDFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\x17W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12!WP\x93\x92PPPV[a\x12]\x83\x83a\x14_V[a\x12j`\0\x84\x84\x84a\x12\x86V[a\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x13|W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x12\xCA\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a\x1B\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x13\x05WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x13\x02\x91\x81\x01\x90a\x1BWV[`\x01[a\x13bW=\x80\x80\x15a\x133W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x138V[``\x91P[P\x80Q`\0\x03a\x13ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xA9\x90a\x1A\xC8V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x0E@V[P`\x01\x94\x93PPPPV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x13\xC6Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x13\xF2Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x14\x10Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x14(Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x14<Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x14NW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x06vW`\x01\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x15\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x15\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x07\xA9V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4PPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08\x82W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x16\x12W`\0\x80\xFD[\x815a\x0B\xDC\x81a\x15\xEAV[`\0[\x83\x81\x10\x15a\x168W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16 V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x16Y\x81` \x86\x01` \x86\x01a\x16\x1DV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0B\xDC` \x83\x01\x84a\x16AV[`\0` \x82\x84\x03\x12\x15a\x16\x92W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xB0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xC8W`\0\x80\xFD[a\x16\xD1\x83a\x16\x99V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\x1EWa\x17\x1Ea\x16\xDFV[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x179W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17QW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x17eW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x17wWa\x17wa\x16\xDFV[\x80`\x05\x1B\x91Pa\x17\x88\x84\x83\x01a\x16\xF5V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x17\xA2W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x17\xD3W\x845\x92P`\xFF\x83\x16\x83\x14a\x17\xC3W`\0\x80\x81\xFD[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x17\xA7V[\x98\x97PPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17\xF4W`\0\x80\xFD[a\x17\xFD\x84a\x16\x99V[\x92Pa\x18\x0B` \x85\x01a\x16\x99V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x18-W`\0\x80\xFD[a\x0B\xDC\x82a\x16\x99V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x18qW\x83Q`\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x18RV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x90W`\0\x80\xFD[a\x18\x99\x83a\x16\x99V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x18\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x18\xCFW`\0\x80\xFD[a\x18\xD8\x85a\x16\x99V[\x93P` a\x18\xE7\x81\x87\x01a\x16\x99V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\x0BW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x19\x1FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x191Wa\x191a\x16\xDFV[a\x19C`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x16\xF5V[\x91P\x80\x82R\x89\x84\x82\x85\x01\x01\x11\x15a\x19YW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\x8CW`\0\x80\xFD[a\x19\x95\x83a\x16\x99V[\x91Pa\x19\xA3` \x84\x01a\x16\x99V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19\xC0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08rWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`-\x90\x82\x01R\x7FERC721: caller is not token owne`@\x82\x01Rl\x1C\x88\x1B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\x9A\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01\x82\x01a\x1AMWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0\x83Qa\x1Af\x81\x84` \x88\x01a\x16\x1DV[\x83Q\x90\x83\x01\x90a\x1Az\x81\x83` \x88\x01a\x16\x1DV[\x01\x94\x93PPPPV[` \x80\x82R`%\x90\x82\x01R\x7FERC721: transfer from incorrect `@\x82\x01Rd7\xBB\xB72\xB9`\xD9\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`2\x90\x82\x01R\x7FERC721: transfer to non ERC721Re`@\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a\x1BM\x90\x83\x01\x84a\x16AV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1BiW`\0\x80\xFD[\x81Qa\x0B\xDC\x81a\x15\xEAV\xFE\xA2dipfsX\"\x12 w\x1A\n)L\x97\xD5Q\x81\xC9\x17\x95\xF7\0\x04^\xFF\xEDwK\xDD\xAB\xD4\xE20y\xAC\x80\x11\xCAtpdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static EXCHANGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80ck^(\x96\x14a\0OW\x80c}\xC0\xD1\xD0\x14a\0jW\x80c\xBD\xCBX|\x14a\0\xB6W\x80c\xFC\x0CTj\x14a\0\xD8W`\0\x80\xFD[6a\0JW\0[`\0\x80\xFD[a\0Wa\x01\x0CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0vW`\0\x80\xFD[Pa\0\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0aV[4\x80\x15a\0\xC2W`\0\x80\xFD[Pa\0\xD6a\0\xD16`\x04a\x08\xCFV[a\x03PV[\0[4\x80\x15a\0\xE4W`\0\x80\xFD[Pa\0\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x01\x16a\x07XV[4`\0\x03a\x017W`@Qc\x07\x8Die`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD9e\xB3\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xEE\x91\x90\x81\x01\x90a\t\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\n\x91\x90a\t\xCFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02K\x91\x90a\n\x02V[\x90P\x804\x10\x15a\x02nW`@Qc\x07\x8Die`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc@\xD0\x97\xC3`\xE0\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c@\xD0\x97\xC3\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xF8\x91\x90a\n\x02V[\x91Pa\x03\x0734\x83\x90\x03a\x07\xB1V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R3\x91\x7F(\xCA\xB0\xD6`\xED\x8A\xED\xD6\x1A\x8C\x9D\xB0\x0B\x97\xF6\xA2\xD6~\x07\xD8w\x95\x99OD\x0B\x18\xBC_\x1A\xA3\x91\x01`@Q\x80\x91\x03\x90\xA2Pa\x03M`\x01`\0UV[\x90V[a\x03Xa\x07XV[`@Qc1\xA9\x10\x8F`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90ccR!\x1E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE1\x91\x90a\n\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x19W`@Qb\x06\x18?`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@Qc\x02\x06\x04\xBF`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x08\x18\x12\xFC\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xA4\x91\x90a\n\x1BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\xCBW`@Qc|\x15\xD3\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD9e\xB3\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x82\x91\x90\x81\x01\x90a\t\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x9E\x91\x90a\t\xCFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xDF\x91\x90a\n\x02V[\x90P\x80G\x10\x15a\x06\x02W`@Qc\x106\xB5\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06pW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x84W=`\0\x80>=`\0\xFD[PP`@Qc\x08R\xCD\x8D`\xE3\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92PcB\x96lh\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xEAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xFEW=`\0\x80>=`\0\xFD[Pa\x07\x0F\x92P3\x91P\x83\x90Pa\x07\xB1V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R3\x91\x7F5\xEA\x94\xAD\xDB\xC6-\xF2\x81\xAD\xE4\x9C\xC2\xAD\x9B=\x92Y\x16ap\xCFp)(\xB4\xDD\xC0E\xF7%n\x91\x01`@Q\x80\x91\x03\x90\xA2Pa\x07U`\x01`\0UV[PV[`\x02`\0T\x03a\x07\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x04\x10V[`\x02`\0UV[\x80G\x10\x15a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x04\x10V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08NW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08SV[``\x91P[PP\x90P\x80a\x08\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04\x10V[PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xE1W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\t\x19W\x81\x81\x01Q\x83\x82\x01R` \x01a\t\x01V[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\t4W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tLW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\t`W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\trWa\tra\x08\xE8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\t\x9AWa\t\x9Aa\x08\xE8V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\t\xB3W`\0\x80\xFD[a\t\xC4\x83` \x83\x01` \x88\x01a\x08\xFEV[\x97\x96PPPPPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t\xEE\x81`@\x85\x01` \x87\x01a\x08\xFEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\x14W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n-W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nDW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ~\xB7\xAEN\x9D\xAEx\xAB\xE4X\xA3\xC2\xBE\xFE\x8FD|\x1E\rF6\xC3\xA6L*<\\\xC9C\xB9\x05\xA8dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static EXCHANGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Exchange<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Exchange<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Exchange<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Exchange<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Exchange<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Exchange)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Exchange<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXCHANGE_ABI.clone(),
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
                EXCHANGE_ABI.clone(),
                EXCHANGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `buyOne` (0x6b5e2896) function
        pub fn buy_one(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 94, 40, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracle` (0x7dc0d1d0) function
        pub fn oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sellOne` (0xbdcb587c) function
        pub fn sell_one(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 203, 88, 124], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
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
        ///Gets the contract's `TokenBought` event
        pub fn token_bought_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenBoughtFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenSold` event
        pub fn token_sold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenSoldFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExchangeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Exchange<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidPayment` with signature `InvalidPayment()` and selector `0x3c6b4b28`
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
    #[etherror(name = "InvalidPayment", abi = "InvalidPayment()")]
    pub struct InvalidPayment;
    ///Custom Error type `NotEnoughFunds` with signature `NotEnoughFunds()` and selector `0x81b5ad68`
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
    #[etherror(name = "NotEnoughFunds", abi = "NotEnoughFunds()")]
    pub struct NotEnoughFunds;
    ///Custom Error type `SellerNotOwner` with signature `SellerNotOwner(uint256)` and selector `0x0006183f`
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
    #[etherror(name = "SellerNotOwner", abi = "SellerNotOwner(uint256)")]
    pub struct SellerNotOwner {
        pub id: ::ethers::core::types::U256,
    }
    ///Custom Error type `TransferNotApproved` with signature `TransferNotApproved()` and selector `0xf82ba75a`
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
    #[etherror(name = "TransferNotApproved", abi = "TransferNotApproved()")]
    pub struct TransferNotApproved;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExchangeErrors {
        InvalidPayment(InvalidPayment),
        NotEnoughFunds(NotEnoughFunds),
        SellerNotOwner(SellerNotOwner),
        TransferNotApproved(TransferNotApproved),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ExchangeErrors {
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
                = <InvalidPayment as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidPayment(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughFunds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotEnoughFunds(decoded));
            }
            if let Ok(decoded)
                = <SellerNotOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SellerNotOwner(decoded));
            }
            if let Ok(decoded)
                = <TransferNotApproved as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferNotApproved(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExchangeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SellerNotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferNotApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ExchangeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidPayment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughFunds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SellerNotOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferNotApproved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ExchangeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidPayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SellerNotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferNotApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ExchangeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidPayment> for ExchangeErrors {
        fn from(value: InvalidPayment) -> Self {
            Self::InvalidPayment(value)
        }
    }
    impl ::core::convert::From<NotEnoughFunds> for ExchangeErrors {
        fn from(value: NotEnoughFunds) -> Self {
            Self::NotEnoughFunds(value)
        }
    }
    impl ::core::convert::From<SellerNotOwner> for ExchangeErrors {
        fn from(value: SellerNotOwner) -> Self {
            Self::SellerNotOwner(value)
        }
    }
    impl ::core::convert::From<TransferNotApproved> for ExchangeErrors {
        fn from(value: TransferNotApproved) -> Self {
            Self::TransferNotApproved(value)
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
    #[ethevent(name = "TokenBought", abi = "TokenBought(address,uint256,uint256)")]
    pub struct TokenBoughtFilter {
        #[ethevent(indexed)]
        pub buyer: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
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
    #[ethevent(name = "TokenSold", abi = "TokenSold(address,uint256,uint256)")]
    pub struct TokenSoldFilter {
        #[ethevent(indexed)]
        pub seller: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExchangeEvents {
        TokenBoughtFilter(TokenBoughtFilter),
        TokenSoldFilter(TokenSoldFilter),
    }
    impl ::ethers::contract::EthLogDecode for ExchangeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = TokenBoughtFilter::decode_log(log) {
                return Ok(ExchangeEvents::TokenBoughtFilter(decoded));
            }
            if let Ok(decoded) = TokenSoldFilter::decode_log(log) {
                return Ok(ExchangeEvents::TokenSoldFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ExchangeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::TokenBoughtFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenSoldFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<TokenBoughtFilter> for ExchangeEvents {
        fn from(value: TokenBoughtFilter) -> Self {
            Self::TokenBoughtFilter(value)
        }
    }
    impl ::core::convert::From<TokenSoldFilter> for ExchangeEvents {
        fn from(value: TokenSoldFilter) -> Self {
            Self::TokenSoldFilter(value)
        }
    }
    ///Container type for all input parameters for the `buyOne` function with signature `buyOne()` and selector `0x6b5e2896`
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
    #[ethcall(name = "buyOne", abi = "buyOne()")]
    pub struct BuyOneCall;
    ///Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    ///Container type for all input parameters for the `sellOne` function with signature `sellOne(uint256)` and selector `0xbdcb587c`
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
    #[ethcall(name = "sellOne", abi = "sellOne(uint256)")]
    pub struct SellOneCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExchangeCalls {
        BuyOne(BuyOneCall),
        Oracle(OracleCall),
        SellOne(SellOneCall),
        Token(TokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExchangeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BuyOneCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BuyOne(decoded));
            }
            if let Ok(decoded)
                = <OracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Oracle(decoded));
            }
            if let Ok(decoded)
                = <SellOneCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SellOne(decoded));
            }
            if let Ok(decoded)
                = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExchangeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BuyOne(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Oracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SellOne(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ExchangeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BuyOne(element) => ::core::fmt::Display::fmt(element, f),
                Self::Oracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::SellOne(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BuyOneCall> for ExchangeCalls {
        fn from(value: BuyOneCall) -> Self {
            Self::BuyOne(value)
        }
    }
    impl ::core::convert::From<OracleCall> for ExchangeCalls {
        fn from(value: OracleCall) -> Self {
            Self::Oracle(value)
        }
    }
    impl ::core::convert::From<SellOneCall> for ExchangeCalls {
        fn from(value: SellOneCall) -> Self {
            Self::SellOne(value)
        }
    }
    impl ::core::convert::From<TokenCall> for ExchangeCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    ///Container type for all return fields from the `buyOne` function with signature `buyOne()` and selector `0x6b5e2896`
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
    pub struct BuyOneReturn {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    pub struct OracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
}
