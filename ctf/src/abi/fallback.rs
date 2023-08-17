pub use fallback::*;
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
pub mod fallback {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("contribute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contribute"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("contributions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contributions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getContribution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getContribution"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U`\0\x90\x81R` \x81\x90R`@\x90 h65\xC9\xAD\xC5\xDE\xA0\0\0\x90Ua\x02\xEA\x80a\0O`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0NW`\x005`\xE0\x1C\x80c<\xCF\xD6\x0B\x14a\0\x96W\x80cB\xE9L\x90\x14a\0\xADW\x80c\x8D\xA5\xCB[\x14a\0\xEDW\x80c\xD7\xBB\x99\xBA\x14a\x01%W\x80c\xF1\x0F\xDF\\\x14a\x01-W`\0\x80\xFD[6a\0\x91W`\x004\x11\x80\x15a\0qWP3`\0\x90\x81R` \x81\x90R`@\x90 T\x15\x15[a\0zW`\0\x80\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\0[`\0\x80\xFD[4\x80\x15a\0\xA2W`\0\x80\xFD[Pa\0\xABa\x01OV[\0[4\x80\x15a\0\xB9W`\0\x80\xFD[Pa\0\xDAa\0\xC86`\x04a\x02]V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xF9W`\0\x80\xFD[P`\x01Ta\x01\r\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xE4V[a\0\xABa\x01\xE9V[4\x80\x15a\x019W`\0\x80\xFD[P3`\0\x90\x81R` \x81\x90R`@\x90 Ta\0\xDAV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xE6W=`\0\x80>=`\0\xFD[PV[f\x03\x8D~\xA4\xC6\x80\x004\x10a\x01\xFCW`\0\x80\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x02\x1B\x90\x84\x90a\x02\x8DV[\x90\x91UPP`\x01T`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 T3\x83R\x91 T\x11\x15a\x02[W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U[V[`\0` \x82\x84\x03\x12\x15a\x02oW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x86W`\0\x80\xFD[\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xAEWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD2%!b\xE3\"\x98v\x8A\x01\xBB\xBD\xABx8\x07Nr\xF7R\xF4\x1E\x07\xBA\x16X\xCAU\xD6\xCF\xF1pdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static FALLBACK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0NW`\x005`\xE0\x1C\x80c<\xCF\xD6\x0B\x14a\0\x96W\x80cB\xE9L\x90\x14a\0\xADW\x80c\x8D\xA5\xCB[\x14a\0\xEDW\x80c\xD7\xBB\x99\xBA\x14a\x01%W\x80c\xF1\x0F\xDF\\\x14a\x01-W`\0\x80\xFD[6a\0\x91W`\x004\x11\x80\x15a\0qWP3`\0\x90\x81R` \x81\x90R`@\x90 T\x15\x15[a\0zW`\0\x80\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\0[`\0\x80\xFD[4\x80\x15a\0\xA2W`\0\x80\xFD[Pa\0\xABa\x01OV[\0[4\x80\x15a\0\xB9W`\0\x80\xFD[Pa\0\xDAa\0\xC86`\x04a\x02]V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xF9W`\0\x80\xFD[P`\x01Ta\x01\r\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xE4V[a\0\xABa\x01\xE9V[4\x80\x15a\x019W`\0\x80\xFD[P3`\0\x90\x81R` \x81\x90R`@\x90 Ta\0\xDAV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xE6W=`\0\x80>=`\0\xFD[PV[f\x03\x8D~\xA4\xC6\x80\x004\x10a\x01\xFCW`\0\x80\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x02\x1B\x90\x84\x90a\x02\x8DV[\x90\x91UPP`\x01T`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 T3\x83R\x91 T\x11\x15a\x02[W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U[V[`\0` \x82\x84\x03\x12\x15a\x02oW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x86W`\0\x80\xFD[\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xAEWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD2%!b\xE3\"\x98v\x8A\x01\xBB\xBD\xABx8\x07Nr\xF7R\xF4\x1E\x07\xBA\x16X\xCAU\xD6\xCF\xF1pdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static FALLBACK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Fallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Fallback<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Fallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Fallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Fallback<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Fallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Fallback<M> {
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
                FALLBACK_ABI.clone(),
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
                FALLBACK_ABI.clone(),
                FALLBACK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `contribute` (0xd7bb99ba)
        /// function
        pub fn contribute(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 187, 153, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contributions`
        /// (0x42e94c90) function
        pub fn contributions(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([66, 233, 76, 144], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getContribution`
        /// (0xf10fdf5c) function
        pub fn get_contribution(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([241, 15, 223, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b)
        /// function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x3ccfd60b)
        /// function
        pub fn withdraw(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Fallback<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `contribute` function with signature `contribute()`
    /// and selector `0xd7bb99ba`
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
    #[ethcall(name = "contribute", abi = "contribute()")]
    pub struct ContributeCall;
    ///Container type for all input parameters for the
    /// `contributions` function with signature
    /// `contributions(address)` and selector `0x42e94c90`
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
    #[ethcall(name = "contributions", abi = "contributions(address)")]
    pub struct ContributionsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the
    /// `getContribution` function with signature
    /// `getContribution()` and selector `0xf10fdf5c`
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
    #[ethcall(name = "getContribution", abi = "getContribution()")]
    pub struct GetContributionCall;
    ///Container type for all input parameters for the
    /// `owner` function with signature `owner()` and
    /// selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the
    /// `withdraw` function with signature `withdraw()` and
    /// selector `0x3ccfd60b`
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
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum FallbackCalls {
        Contribute(ContributeCall),
        Contributions(ContributionsCall),
        GetContribution(GetContributionCall),
        Owner(OwnerCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for FallbackCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ContributeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Contribute(decoded));
            }
            if let Ok(decoded) =
                <ContributionsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::Contributions(decoded));
            }
            if let Ok(decoded) =
                <GetContributionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetContribution(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FallbackCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Contribute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Contributions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetContribution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FallbackCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Contribute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Contributions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetContribution(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ContributeCall> for FallbackCalls {
        fn from(value: ContributeCall) -> Self { Self::Contribute(value) }
    }
    impl ::core::convert::From<ContributionsCall> for FallbackCalls {
        fn from(value: ContributionsCall) -> Self { Self::Contributions(value) }
    }
    impl ::core::convert::From<GetContributionCall> for FallbackCalls {
        fn from(value: GetContributionCall) -> Self {
            Self::GetContribution(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for FallbackCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<WithdrawCall> for FallbackCalls {
        fn from(value: WithdrawCall) -> Self { Self::Withdraw(value) }
    }
    ///Container type for all return fields from the
    /// `contributions` function with signature
    /// `contributions(address)` and selector `0x42e94c90`
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
    pub struct ContributionsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `getContribution` function with signature
    /// `getContribution()` and selector `0xf10fdf5c`
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
    pub struct GetContributionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `owner` function with signature `owner()` and
    /// selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
