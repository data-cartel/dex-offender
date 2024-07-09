pub use flash_loan_receiver::*;
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
pub mod flash_loan_receiver {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(
                ::ethers::core::abi::ethabi::Constructor {
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },],
                },
            ),
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("onFlashLoan"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("onFlashLoan"),
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
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
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
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("fee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                256usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                                32usize,
                            ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes32"),
                        ),
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability:
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("UnsupportedCurrency"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned(
                        "UnsupportedCurrency",
                    ),
                    inputs: ::std::vec![],
                },],
            )]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FLASHLOANRECEIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa\x02\xA18\x03\x80a\x02\xA1\x839\x81\x01`@\x81\x90R`,\x91`PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`~V[`\0` \x82\x84\x03\x12\x15`aW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`wW`\0\x80\xFD[\x93\x92PPPV[a\x02\x14\x80a\0\x8D`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\"W`\x005`\xE0\x1C\x80c#\xE3\x0C\x8B\x14a\0.W`\0\x80\xFD[6a\0)W\0[`\0\x80\xFD[4\x80\x15a\0:W`\0\x80\xFD[Pa\0Na\0I6`\x04a\x018V[a\0`V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x003`\0T\x14a\0yWcH\xF5\xC3\xED`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\0\xB6W`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x01`\0Ta\0\xCF\x90`\x01`\x01`\xA0\x1B\x03\x16\x82a\0\xFCV[P\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x97\x96PPPPPPPV[`\0\x80`\0\x80\x84\x86Z\xF1a\x01\x18Wc\xB1-\x13\xEB`\0R`\x04`\x1C\xFD[PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x013W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x01QW`\0\x80\xFD[a\x01Z\x87a\x01\x1CV[\x95Pa\x01h` \x88\x01a\x01\x1CV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\x93W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x01\xA7W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xB6W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x01\xC8W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V\xFE\xA2dipfsX\"\x12 \x80\xD8\x18\xE3\xD76\xDFR3E\xE6x-\x81\x1A\\h}xQ\xF1\xAB\xC3\xF0\xCET\xCEl\xBC\xCAYrdsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static FLASHLOANRECEIVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\"W`\x005`\xE0\x1C\x80c#\xE3\x0C\x8B\x14a\0.W`\0\x80\xFD[6a\0)W\0[`\0\x80\xFD[4\x80\x15a\0:W`\0\x80\xFD[Pa\0Na\0I6`\x04a\x018V[a\0`V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x003`\0T\x14a\0yWcH\xF5\xC3\xED`\0R`\x04`\x1C\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\0\xB6W`@Qc\x111\xFAq`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x01`\0Ta\0\xCF\x90`\x01`\x01`\xA0\x1B\x03\x16\x82a\0\xFCV[P\x7FC\x91H\xF0\xBB\xC6\x82\xCA\x07\x9EF\xD6\xE2\xC2\xF0\xC1\xE3\xB8 \xF1\xA2\x91\xB0i\xD8\x88*\xBF\x8C\xF1\x8D\xD9\x97\x96PPPPPPPV[`\0\x80`\0\x80\x84\x86Z\xF1a\x01\x18Wc\xB1-\x13\xEB`\0R`\x04`\x1C\xFD[PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x013W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x01QW`\0\x80\xFD[a\x01Z\x87a\x01\x1CV[\x95Pa\x01h` \x88\x01a\x01\x1CV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\x93W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x01\xA7W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xB6W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x01\xC8W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V\xFE\xA2dipfsX\"\x12 \x80\xD8\x18\xE3\xD76\xDFR3E\xE6x-\x81\x1A\\h}xQ\xF1\xAB\xC3\xF0\xCET\xCEl\xBC\xCAYrdsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static FLASHLOANRECEIVER_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FlashLoanReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FlashLoanReceiver<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for FlashLoanReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for FlashLoanReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for FlashLoanReceiver<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FlashLoanReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FlashLoanReceiver<M> {
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
                FLASHLOANRECEIVER_ABI.clone(),
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
                FLASHLOANRECEIVER_ABI.clone(),
                FLASHLOANRECEIVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `onFlashLoan` (0x23e30c8b)
        /// function
        pub fn on_flash_loan(
            &self,
            p0: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            fee: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([35, 227, 12, 139], (p0, token, amount, fee, p4))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for FlashLoanReceiver<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `UnsupportedCurrency` with
    /// signature `UnsupportedCurrency()` and selector
    /// `0x2263f4e2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "UnsupportedCurrency", abi = "UnsupportedCurrency()")]
    pub struct UnsupportedCurrency;
    ///Container type for all input parameters for the
    /// `onFlashLoan` function with signature
    /// `onFlashLoan(address,address,uint256,uint256,bytes)`
    /// and selector `0x23e30c8b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onFlashLoan",
        abi = "onFlashLoan(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnFlashLoanCall {
        pub p0: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
        pub p4: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the
    /// `onFlashLoan` function with signature
    /// `onFlashLoan(address,address,uint256,uint256,bytes)`
    /// and selector `0x23e30c8b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OnFlashLoanReturn(pub [u8; 32]);
}
