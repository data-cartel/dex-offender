pub use coin_flip::*;
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
pub mod coin_flip {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_offender"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_wins"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("consecutiveWins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("consecutiveWins"),
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
                    ::std::borrow::ToOwned::to_owned("flip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flip"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_guess"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COINFLIP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x01`\xFF\x1B`\x02U4\x80\x15a\0\x18W`\0\x80\xFD[P`@Qa\x02U8\x03\x80a\x02U\x839\x81\x01`@\x81\x90Ra\x007\x91a\0BV[PP`\0\x80Ua\0|V[`\0\x80`@\x83\x85\x03\x12\x15a\0UW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0lW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[a\x01\xCA\x80a\0\x8B`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x1D&?g\x14a\0;W\x80c\xE6\xF34\xD7\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\x01V[a\0zV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0l`\0T\x81V[`@Q\x90\x81R` \x01a\0ZV[`\0\x80a\0\x88`\x01Ca\x01@V[@`\0\x1C\x90P\x80`\x01T\x03a\0\x9CW`\0\x80\xFD[`\x01\x81\x90U`\x02T`\0\x90a\0\xB1\x90\x83a\x01YV[\x90P`\0\x81`\x01\x14a\0\xC4W`\0a\0\xC7V[`\x01[\x90P\x84\x15\x15\x81\x15\x15\x03a\0\xF3W`\0\x80T\x90\x80a\0\xE3\x83a\x01{V[\x90\x91UP`\x01\x96\x95PPPPPPV[PP`\0\x80\x80U\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x01\x13W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x01#W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x01SWa\x01Sa\x01*V[\x92\x91PPV[`\0\x82a\x01vWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01\x82\x01a\x01\x8DWa\x01\x8Da\x01*V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x10\xF4\x9B\xF5\xFE\xE1\x007\xCCA\x0B\x18\xDA\xAC&\xE6e\xB1\x9B\xB9\xBC\xE4\x89\xAD$\xD5SS\xA3Y\x11\x80dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static COINFLIP_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x1D&?g\x14a\0;W\x80c\xE6\xF34\xD7\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\x01V[a\0zV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0l`\0T\x81V[`@Q\x90\x81R` \x01a\0ZV[`\0\x80a\0\x88`\x01Ca\x01@V[@`\0\x1C\x90P\x80`\x01T\x03a\0\x9CW`\0\x80\xFD[`\x01\x81\x90U`\x02T`\0\x90a\0\xB1\x90\x83a\x01YV[\x90P`\0\x81`\x01\x14a\0\xC4W`\0a\0\xC7V[`\x01[\x90P\x84\x15\x15\x81\x15\x15\x03a\0\xF3W`\0\x80T\x90\x80a\0\xE3\x83a\x01{V[\x90\x91UP`\x01\x96\x95PPPPPPV[PP`\0\x80\x80U\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x01\x13W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x01#W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x01SWa\x01Sa\x01*V[\x92\x91PPV[`\0\x82a\x01vWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01\x82\x01a\x01\x8DWa\x01\x8Da\x01*V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x10\xF4\x9B\xF5\xFE\xE1\x007\xCCA\x0B\x18\xDA\xAC&\xE6e\xB1\x9B\xB9\xBC\xE4\x89\xAD$\xD5SS\xA3Y\x11\x80dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static COINFLIP_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CoinFlip<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CoinFlip<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CoinFlip<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CoinFlip<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CoinFlip<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CoinFlip)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CoinFlip<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COINFLIP_ABI.clone(),
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
                COINFLIP_ABI.clone(),
                COINFLIP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `consecutiveWins` (0xe6f334d7) function
        pub fn consecutive_wins(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([230, 243, 52, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flip` (0x1d263f67) function
        pub fn flip(
            &self,
            guess: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 38, 63, 103], guess)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CoinFlip<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `consecutiveWins` function with signature `consecutiveWins()` and selector `0xe6f334d7`
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
    #[ethcall(name = "consecutiveWins", abi = "consecutiveWins()")]
    pub struct ConsecutiveWinsCall;
    ///Container type for all input parameters for the `flip` function with signature `flip(bool)` and selector `0x1d263f67`
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
    #[ethcall(name = "flip", abi = "flip(bool)")]
    pub struct FlipCall {
        pub guess: bool,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CoinFlipCalls {
        ConsecutiveWins(ConsecutiveWinsCall),
        Flip(FlipCall),
    }
    impl ::ethers::core::abi::AbiDecode for CoinFlipCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ConsecutiveWinsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ConsecutiveWins(decoded));
            }
            if let Ok(decoded)
                = <FlipCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Flip(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CoinFlipCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConsecutiveWins(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Flip(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CoinFlipCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConsecutiveWins(element) => ::core::fmt::Display::fmt(element, f),
                Self::Flip(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConsecutiveWinsCall> for CoinFlipCalls {
        fn from(value: ConsecutiveWinsCall) -> Self {
            Self::ConsecutiveWins(value)
        }
    }
    impl ::core::convert::From<FlipCall> for CoinFlipCalls {
        fn from(value: FlipCall) -> Self {
            Self::Flip(value)
        }
    }
    ///Container type for all return fields from the `consecutiveWins` function with signature `consecutiveWins()` and selector `0xe6f334d7`
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
    pub struct ConsecutiveWinsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `flip` function with signature `flip(bool)` and selector `0x1d263f67`
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
    pub struct FlipReturn(pub bool);
}
