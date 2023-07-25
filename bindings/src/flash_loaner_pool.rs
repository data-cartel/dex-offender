pub use flash_loaner_pool::*;
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
pub mod flash_loaner_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("liquidityTokenAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidityToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidityToken"),
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
                    ::std::borrow::ToOwned::to_owned("CallerIsNotContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CallerIsNotContract",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FlashLoanNotPaidBack"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FlashLoanNotPaidBack",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughTokenBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotEnoughTokenBalance",
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
    pub static FLASHLOANERPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x06\xBD8\x03\x80a\x06\xBD\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0uV[`\0` \x82\x84\x03\x12\x15a\0WW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0nW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x06\x1Aa\0\xA3`\09`\0\x81\x81`@\x01R\x81\x81`\xB3\x01R\x81\x81a\x01\x84\x01Ra\x02Y\x01Ra\x06\x1A`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cC\xCD\x8F~\x14a\0;W\x80c\x9A\xB6\x03\xB9\x14a\0~W[`\0\x80\xFD[a\0b\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x91a\0\x8C6`\x04a\x05\x1DV[a\0\x93V[\0[a\0\x9Ba\x02\xF9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01&\x91\x90a\x056V[\x90P\x80\x82\x11\x15a\x01IW`@Qc]\xD3\xB4\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3;a\x01hW`@Qc{Ri\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF9\x91\x90a\x05OV[Pa\x02A\x82`@Q`$\x01a\x02\x10\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cO\xD9\xCB\x81`\xE0\x1B\x17\x90R3\x90a\x03WV[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCC\x91\x90a\x056V[\x10\x15a\x02\xEBW`@Qc\x04\xC9\x92\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x02\xF6`\x01`\0UV[PV[`\x02`\0T\x03a\x03PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[``a\x03\x9B\x83\x83`\0`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FAddress: low-level call failed\0\0\x81RPa\x03\xA2V[\x93\x92PPPV[``\x82G\x10\x15a\x04\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03GV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x04\x1F\x91\x90a\x05\x95V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\\W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04aV[``\x91P[P\x91P\x91Pa\x04r\x87\x83\x83\x87a\x04\x7FV[\x92PPP[\x94\x93PPPPV[``\x83\x15a\x04\xEEW\x82Q`\0\x03a\x04\xE7W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03GV[P\x81a\x04wV[a\x04w\x83\x83\x81Q\x15a\x05\x03W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03G\x91\x90a\x05\xB1V[`\0` \x82\x84\x03\x12\x15a\x05/W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05HW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05aW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x9BW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x05\x8CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05tV[PP`\0\x91\x01RV[`\0\x82Qa\x05\xA7\x81\x84` \x87\x01a\x05qV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x05\xD0\x81`@\x85\x01` \x87\x01a\x05qV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xABcE\x86\xAE\xEAA\xB2\xBD\x1Cg\x88O\x88\xB1\xB7&\xF5\x12<\xC6=\xDC*m\xFB[\x92\xBC\xEC\xE2\xEEdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static FLASHLOANERPOOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cC\xCD\x8F~\x14a\0;W\x80c\x9A\xB6\x03\xB9\x14a\0~W[`\0\x80\xFD[a\0b\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x91a\0\x8C6`\x04a\x05\x1DV[a\0\x93V[\0[a\0\x9Ba\x02\xF9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01&\x91\x90a\x056V[\x90P\x80\x82\x11\x15a\x01IW`@Qc]\xD3\xB4\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3;a\x01hW`@Qc{Ri\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF9\x91\x90a\x05OV[Pa\x02A\x82`@Q`$\x01a\x02\x10\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cO\xD9\xCB\x81`\xE0\x1B\x17\x90R3\x90a\x03WV[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCC\x91\x90a\x056V[\x10\x15a\x02\xEBW`@Qc\x04\xC9\x92\xBD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x02\xF6`\x01`\0UV[PV[`\x02`\0T\x03a\x03PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[``a\x03\x9B\x83\x83`\0`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FAddress: low-level call failed\0\0\x81RPa\x03\xA2V[\x93\x92PPPV[``\x82G\x10\x15a\x04\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03GV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x04\x1F\x91\x90a\x05\x95V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\\W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04aV[``\x91P[P\x91P\x91Pa\x04r\x87\x83\x83\x87a\x04\x7FV[\x92PPP[\x94\x93PPPPV[``\x83\x15a\x04\xEEW\x82Q`\0\x03a\x04\xE7W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03GV[P\x81a\x04wV[a\x04w\x83\x83\x81Q\x15a\x05\x03W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03G\x91\x90a\x05\xB1V[`\0` \x82\x84\x03\x12\x15a\x05/W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05HW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05aW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x9BW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x05\x8CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05tV[PP`\0\x91\x01RV[`\0\x82Qa\x05\xA7\x81\x84` \x87\x01a\x05qV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x05\xD0\x81`@\x85\x01` \x87\x01a\x05qV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xABcE\x86\xAE\xEAA\xB2\xBD\x1Cg\x88O\x88\xB1\xB7&\xF5\x12<\xC6=\xDC*m\xFB[\x92\xBC\xEC\xE2\xEEdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static FLASHLOANERPOOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FlashLoanerPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FlashLoanerPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FlashLoanerPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FlashLoanerPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FlashLoanerPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FlashLoanerPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FlashLoanerPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FLASHLOANERPOOL_ABI.clone(),
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
                FLASHLOANERPOOL_ABI.clone(),
                FLASHLOANERPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `flashLoan` (0x9ab603b9) function
        pub fn flash_loan(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 182, 3, 185], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidityToken` (0x43cd8f7e) function
        pub fn liquidity_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([67, 205, 143, 126], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FlashLoanerPool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerIsNotContract` with signature `CallerIsNotContract()` and selector `0x7b5269d3`
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
    #[etherror(name = "CallerIsNotContract", abi = "CallerIsNotContract()")]
    pub struct CallerIsNotContract;
    ///Custom Error type `FlashLoanNotPaidBack` with signature `FlashLoanNotPaidBack()` and selector `0x0993257a`
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
    #[etherror(name = "FlashLoanNotPaidBack", abi = "FlashLoanNotPaidBack()")]
    pub struct FlashLoanNotPaidBack;
    ///Custom Error type `NotEnoughTokenBalance` with signature `NotEnoughTokenBalance()` and selector `0x5dd3b4d1`
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
    #[etherror(name = "NotEnoughTokenBalance", abi = "NotEnoughTokenBalance()")]
    pub struct NotEnoughTokenBalance;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FlashLoanerPoolErrors {
        CallerIsNotContract(CallerIsNotContract),
        FlashLoanNotPaidBack(FlashLoanNotPaidBack),
        NotEnoughTokenBalance(NotEnoughTokenBalance),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FlashLoanerPoolErrors {
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
                = <CallerIsNotContract as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerIsNotContract(decoded));
            }
            if let Ok(decoded)
                = <FlashLoanNotPaidBack as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FlashLoanNotPaidBack(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughTokenBalance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotEnoughTokenBalance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FlashLoanerPoolErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerIsNotContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashLoanNotPaidBack(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughTokenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FlashLoanerPoolErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerIsNotContract as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FlashLoanNotPaidBack as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughTokenBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FlashLoanerPoolErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerIsNotContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashLoanNotPaidBack(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughTokenBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FlashLoanerPoolErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerIsNotContract> for FlashLoanerPoolErrors {
        fn from(value: CallerIsNotContract) -> Self {
            Self::CallerIsNotContract(value)
        }
    }
    impl ::core::convert::From<FlashLoanNotPaidBack> for FlashLoanerPoolErrors {
        fn from(value: FlashLoanNotPaidBack) -> Self {
            Self::FlashLoanNotPaidBack(value)
        }
    }
    impl ::core::convert::From<NotEnoughTokenBalance> for FlashLoanerPoolErrors {
        fn from(value: NotEnoughTokenBalance) -> Self {
            Self::NotEnoughTokenBalance(value)
        }
    }
    ///Container type for all input parameters for the `flashLoan` function with signature `flashLoan(uint256)` and selector `0x9ab603b9`
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
    #[ethcall(name = "flashLoan", abi = "flashLoan(uint256)")]
    pub struct FlashLoanCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidityToken` function with signature `liquidityToken()` and selector `0x43cd8f7e`
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
    #[ethcall(name = "liquidityToken", abi = "liquidityToken()")]
    pub struct LiquidityTokenCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FlashLoanerPoolCalls {
        FlashLoan(FlashLoanCall),
        LiquidityToken(LiquidityTokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for FlashLoanerPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FlashLoan(decoded));
            }
            if let Ok(decoded)
                = <LiquidityTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LiquidityToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FlashLoanerPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidityToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FlashLoanerPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityToken(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FlashLoanCall> for FlashLoanerPoolCalls {
        fn from(value: FlashLoanCall) -> Self {
            Self::FlashLoan(value)
        }
    }
    impl ::core::convert::From<LiquidityTokenCall> for FlashLoanerPoolCalls {
        fn from(value: LiquidityTokenCall) -> Self {
            Self::LiquidityToken(value)
        }
    }
    ///Container type for all return fields from the `liquidityToken` function with signature `liquidityToken()` and selector `0x43cd8f7e`
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
    pub struct LiquidityTokenReturn(pub ::ethers::core::types::Address);
}
