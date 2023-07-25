pub use coin_suka::*;
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
pub mod coin_suka {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_coin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("coinContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("coinContract"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CoinFlip"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("guess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("guess"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COINSUKA_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01`\xFF\x1B`\x01U4\x80\x15a\0\x18W`\0\x80\xFD[P`@Qa\x02\xA18\x03\x80a\x02\xA1\x839\x81\x01`@\x81\x90Ra\x007\x91a\0\\V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x8CV[`\0` \x82\x84\x03\x12\x15a\0nW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x85W`\0\x80\xFD[\x93\x92PPPV[a\x02\x06\x80a\0\x9B`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cr\xDDR\xE3\x14a\0;W\x80c\xA2\x8Bt\x9A\x14a\0jW[`\0\x80\xFD[`\0Ta\0N\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ra\0tV[\0[`\0a\0\x81`\x01Ca\x01^V[`\x01T\x90@\x91P`\0\x90a\0\x95\x90\x83a\x01\x85V[\x90P`\0\x81`\x01\x14a\0\xA8W`\0a\0\xABV[`\x01[\x90P\x80\x15\x15`\x01\x03a\x01-W`\0T`@Qc\x1D&?g`\xE0\x1B\x81R`\x01`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x1D&?g\x90`$\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01'\x91\x90a\x01\xA7V[PPPPV[`\0\x80T`@Qc\x1D&?g`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x1D&?g\x90`$\x01a\0\xE4V[\x81\x81\x03\x81\x81\x11\x15a\x01\x7FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x82a\x01\xA2WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x01\xB9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x01\xC9W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x96\x0E\xE3\xDD\xCB\xE1\xD2p\xDEH\xC6g\xF0P\xC5@\xF8<\xCBPUt\x9Ck\xBBUJ9\xCC\x18\xC0\x01dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static COINSUKA_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cr\xDDR\xE3\x14a\0;W\x80c\xA2\x8Bt\x9A\x14a\0jW[`\0\x80\xFD[`\0Ta\0N\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ra\0tV[\0[`\0a\0\x81`\x01Ca\x01^V[`\x01T\x90@\x91P`\0\x90a\0\x95\x90\x83a\x01\x85V[\x90P`\0\x81`\x01\x14a\0\xA8W`\0a\0\xABV[`\x01[\x90P\x80\x15\x15`\x01\x03a\x01-W`\0T`@Qc\x1D&?g`\xE0\x1B\x81R`\x01`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x1D&?g\x90`$\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01'\x91\x90a\x01\xA7V[PPPPV[`\0\x80T`@Qc\x1D&?g`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x1D&?g\x90`$\x01a\0\xE4V[\x81\x81\x03\x81\x81\x11\x15a\x01\x7FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x82a\x01\xA2WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x01\xB9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x01\xC9W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x96\x0E\xE3\xDD\xCB\xE1\xD2p\xDEH\xC6g\xF0P\xC5@\xF8<\xCBPUt\x9Ck\xBBUJ9\xCC\x18\xC0\x01dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static COINSUKA_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CoinSUKA<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CoinSUKA<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CoinSUKA<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CoinSUKA<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CoinSUKA<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CoinSUKA)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CoinSUKA<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COINSUKA_ABI.clone(),
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
                COINSUKA_ABI.clone(),
                COINSUKA_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `coinContract` (0x72dd52e3) function
        pub fn coin_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([114, 221, 82, 227], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `guess` (0xa28b749a) function
        pub fn guess(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 139, 116, 154], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CoinSUKA<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `coinContract` function with signature `coinContract()` and selector `0x72dd52e3`
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
    #[ethcall(name = "coinContract", abi = "coinContract()")]
    pub struct CoinContractCall;
    ///Container type for all input parameters for the `guess` function with signature `guess()` and selector `0xa28b749a`
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
    #[ethcall(name = "guess", abi = "guess()")]
    pub struct GuessCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CoinSUKACalls {
        CoinContract(CoinContractCall),
        Guess(GuessCall),
    }
    impl ::ethers::core::abi::AbiDecode for CoinSUKACalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CoinContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CoinContract(decoded));
            }
            if let Ok(decoded)
                = <GuessCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Guess(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CoinSUKACalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CoinContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Guess(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CoinSUKACalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CoinContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::Guess(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CoinContractCall> for CoinSUKACalls {
        fn from(value: CoinContractCall) -> Self {
            Self::CoinContract(value)
        }
    }
    impl ::core::convert::From<GuessCall> for CoinSUKACalls {
        fn from(value: GuessCall) -> Self {
            Self::Guess(value)
        }
    }
    ///Container type for all return fields from the `coinContract` function with signature `coinContract()` and selector `0x72dd52e3`
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
    pub struct CoinContractReturn(pub ::ethers::core::types::Address);
}
