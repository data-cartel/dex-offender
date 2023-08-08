pub use crypto_vault::*;
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
pub mod crypto_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("recipient"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("setUnderlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUnderlying"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("latestToken"),
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
                    ::std::borrow::ToOwned::to_owned("sweepToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
                    ::std::borrow::ToOwned::to_owned("sweptTokensRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sweptTokensRecipient",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("underlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("underlying"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CRYPTOVAULT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x03\xB18\x03\x80a\x03\xB1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x03\x1E\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x1B\xE1\x95`\x14a\0QW\x80c24\xA1\x97\x14a\0fW\x80co0}\xC3\x14a\0\x95W\x80c\xBD\xB22\x1F\x14a\0\xA8W[`\0\x80\xFD[a\0da\0_6`\x04a\x02\x89V[a\0\xBBV[\0[`\0Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0da\0\xB66`\x04a\x02\x89V[a\x02\x08V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x01\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FCan't transfer underlying token\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x92c\xA9\x05\x9C\xBB\x92\x91\x16\x90\x83\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x95\x91\x90a\x02\xADV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x04\x91\x90a\x02\xC6V[PPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x1C\xD9]`\xAA\x1B`D\x82\x01R`d\x01a\x01\x15V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x86W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x02\x9BW`\0\x80\xFD[\x815a\x02\xA6\x81a\x02qV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x02\xBFW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xD8W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\xA6W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x8B\xB4\x9A\xE1:\xFC\x80/\xF9C\x06\x9F\xE1p\x03\xBF1\xD75\x0E\x16\xF0\xBC_\x16\xE0fzs\x89!rdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static CRYPTOVAULT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x1B\xE1\x95`\x14a\0QW\x80c24\xA1\x97\x14a\0fW\x80co0}\xC3\x14a\0\x95W\x80c\xBD\xB22\x1F\x14a\0\xA8W[`\0\x80\xFD[a\0da\0_6`\x04a\x02\x89V[a\0\xBBV[\0[`\0Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0da\0\xB66`\x04a\x02\x89V[a\x02\x08V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x01\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FCan't transfer underlying token\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x92c\xA9\x05\x9C\xBB\x92\x91\x16\x90\x83\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x95\x91\x90a\x02\xADV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x04\x91\x90a\x02\xC6V[PPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x1C\xD9]`\xAA\x1B`D\x82\x01R`d\x01a\x01\x15V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x86W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x02\x9BW`\0\x80\xFD[\x815a\x02\xA6\x81a\x02qV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x02\xBFW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xD8W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\xA6W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x8B\xB4\x9A\xE1:\xFC\x80/\xF9C\x06\x9F\xE1p\x03\xBF1\xD75\x0E\x16\xF0\xBC_\x16\xE0fzs\x89!rdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static CRYPTOVAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct CryptoVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryptoVault<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for CryptoVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for CryptoVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for CryptoVault<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryptoVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryptoVault<M> {
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
                CRYPTOVAULT_ABI.clone(),
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
                CRYPTOVAULT_ABI.clone(),
                CRYPTOVAULT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `setUnderlying`
        /// (0xbdb2321f) function
        pub fn set_underlying(
            &self,
            latest_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 178, 50, 31], latest_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepToken` (0x1be19560)
        /// function
        pub fn sweep_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 225, 149, 96], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweptTokensRecipient`
        /// (0x3234a197) function
        pub fn swept_tokens_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([50, 52, 161, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlying` (0x6f307dc3)
        /// function
        pub fn underlying(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for CryptoVault<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `setUnderlying` function with signature
    /// `setUnderlying(address)` and selector `0xbdb2321f`
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
    #[ethcall(name = "setUnderlying", abi = "setUnderlying(address)")]
    pub struct SetUnderlyingCall {
        pub latest_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `sweepToken` function with signature
    /// `sweepToken(address)` and selector `0x1be19560`
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
    #[ethcall(name = "sweepToken", abi = "sweepToken(address)")]
    pub struct SweepTokenCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `sweptTokensRecipient` function with signature
    /// `sweptTokensRecipient()` and selector `0x3234a197`
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
    #[ethcall(name = "sweptTokensRecipient", abi = "sweptTokensRecipient()")]
    pub struct SweptTokensRecipientCall;
    ///Container type for all input parameters for the
    /// `underlying` function with signature `underlying()`
    /// and selector `0x6f307dc3`
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
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum CryptoVaultCalls {
        SetUnderlying(SetUnderlyingCall),
        SweepToken(SweepTokenCall),
        SweptTokensRecipient(SweptTokensRecipientCall),
        Underlying(UnderlyingCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryptoVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <SetUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetUnderlying(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SweepToken(decoded));
            }
            if let Ok(decoded)
                = <SweptTokensRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SweptTokensRecipient(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Underlying(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryptoVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SetUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweptTokensRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Underlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryptoVaultCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::SetUnderlying(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweepToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweptTokensRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Underlying(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<SetUnderlyingCall> for CryptoVaultCalls {
        fn from(value: SetUnderlyingCall) -> Self { Self::SetUnderlying(value) }
    }
    impl ::core::convert::From<SweepTokenCall> for CryptoVaultCalls {
        fn from(value: SweepTokenCall) -> Self { Self::SweepToken(value) }
    }
    impl ::core::convert::From<SweptTokensRecipientCall> for CryptoVaultCalls {
        fn from(value: SweptTokensRecipientCall) -> Self {
            Self::SweptTokensRecipient(value)
        }
    }
    impl ::core::convert::From<UnderlyingCall> for CryptoVaultCalls {
        fn from(value: UnderlyingCall) -> Self { Self::Underlying(value) }
    }
    ///Container type for all return fields from the
    /// `sweptTokensRecipient` function with signature
    /// `sweptTokensRecipient()` and selector `0x3234a197`
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
    pub struct SweptTokensRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `underlying` function with signature `underlying()`
    /// and selector `0x6f307dc3`
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
    pub struct UnderlyingReturn(pub ::ethers::core::types::Address);
}
