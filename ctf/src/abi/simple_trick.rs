pub use simple_trick::*;
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
pub mod simple_trick {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_target"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address payable"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkPassword"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkPassword"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_password"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("target"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GatekeeperThree"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trick"),
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
                    ::std::borrow::ToOwned::to_owned("trickInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trickInit"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trickyTrick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trickyTrick"),
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
    pub static SIMPLETRICK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@RB`\x02U4\x80\x15a\0\x14W`\0\x80\xFD[P`@Qa\x02p8\x03\x80a\x02p\x839\x81\x01`@\x81\x90Ra\x003\x91a\0XV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x88V[`\0` \x82\x84\x03\x12\x15a\0jW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x81W`\0\x80\xFD[\x93\x92PPPV[a\x01\xD9\x80a\0\x97`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80cL\xBB\x81\x7F\x14a\0\\W\x80ci\r\xA2\xB2\x14a\0uW\x80c\x9EK.G\x14a\0\xA5W\x80c\xB7\xE0\x02\x91\x14a\0\xC8W\x80c\xD4\xB89\x92\x14a\0\xD0W[`\0\x80\xFD[a\0s`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x160\x17\x90UV[\0[`\x01Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB8a\0\xB36`\x04a\x01\x8AV[a\0\xE3V[`@Q\x90\x15\x15\x81R` \x01a\0\x9CV[a\0sa\x01\x01V[`\0Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0`\x02T\x82\x03a\0\xF6WP`\x01\x91\x90PV[PPB`\x02U`\0\x90V[03\x14\x80\x15a\x01\x1BWP`\x01T`\x01`\x01`\xA0\x1B\x03\x160\x14\x15[\x15a\x01\x88W`\0T`\x02T`@Qcd\xB0\x0B\xA7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xC9`\x17N\x91a\x01U\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x83W=`\0\x80>=`\0\xFD[PPPP[V[`\0` \x82\x84\x03\x12\x15a\x01\x9CW`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 \x10_o\x94zG\t\xC6+\xC1d\x89\x13c;\xA5:,\xBBP\xB8<\x9B\xDEn\xB6pG\xF8c\xA7\x18dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static SIMPLETRICK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80cL\xBB\x81\x7F\x14a\0\\W\x80ci\r\xA2\xB2\x14a\0uW\x80c\x9EK.G\x14a\0\xA5W\x80c\xB7\xE0\x02\x91\x14a\0\xC8W\x80c\xD4\xB89\x92\x14a\0\xD0W[`\0\x80\xFD[a\0s`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x160\x17\x90UV[\0[`\x01Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB8a\0\xB36`\x04a\x01\x8AV[a\0\xE3V[`@Q\x90\x15\x15\x81R` \x01a\0\x9CV[a\0sa\x01\x01V[`\0Ta\0\x88\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0`\x02T\x82\x03a\0\xF6WP`\x01\x91\x90PV[PPB`\x02U`\0\x90V[03\x14\x80\x15a\x01\x1BWP`\x01T`\x01`\x01`\xA0\x1B\x03\x160\x14\x15[\x15a\x01\x88W`\0T`\x02T`@Qcd\xB0\x0B\xA7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xC9`\x17N\x91a\x01U\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01oW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x83W=`\0\x80>=`\0\xFD[PPPP[V[`\0` \x82\x84\x03\x12\x15a\x01\x9CW`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 \x10_o\x94zG\t\xC6+\xC1d\x89\x13c;\xA5:,\xBBP\xB8<\x9B\xDEn\xB6pG\xF8c\xA7\x18dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static SIMPLETRICK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SimpleTrick<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleTrick<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for SimpleTrick<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for SimpleTrick<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for SimpleTrick<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SimpleTrick))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleTrick<M> {
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
                SIMPLETRICK_ABI.clone(),
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
                SIMPLETRICK_ABI.clone(),
                SIMPLETRICK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkPassword`
        /// (0x9e4b2e47) function
        pub fn check_password(
            &self,
            password: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([158, 75, 46, 71], password)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `target` (0xd4b83992)
        /// function
        pub fn target(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([212, 184, 57, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trick` (0x690da2b2)
        /// function
        pub fn trick(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([105, 13, 162, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trickInit` (0x4cbb817f)
        /// function
        pub fn trick_init(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 187, 129, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trickyTrick` (0xb7e00291)
        /// function
        pub fn tricky_trick(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 224, 2, 145], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for SimpleTrick<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `checkPassword` function with signature
    /// `checkPassword(uint256)` and selector `0x9e4b2e47`
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
    #[ethcall(name = "checkPassword", abi = "checkPassword(uint256)")]
    pub struct CheckPasswordCall {
        pub password: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `target` function with signature `target()` and
    /// selector `0xd4b83992`
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
    #[ethcall(name = "target", abi = "target()")]
    pub struct TargetCall;
    ///Container type for all input parameters for the
    /// `trick` function with signature `trick()` and
    /// selector `0x690da2b2`
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
    #[ethcall(name = "trick", abi = "trick()")]
    pub struct TrickCall;
    ///Container type for all input parameters for the
    /// `trickInit` function with signature `trickInit()`
    /// and selector `0x4cbb817f`
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
    #[ethcall(name = "trickInit", abi = "trickInit()")]
    pub struct TrickInitCall;
    ///Container type for all input parameters for the
    /// `trickyTrick` function with signature
    /// `trickyTrick()` and selector `0xb7e00291`
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
    #[ethcall(name = "trickyTrick", abi = "trickyTrick()")]
    pub struct TrickyTrickCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum SimpleTrickCalls {
        CheckPassword(CheckPasswordCall),
        Target(TargetCall),
        Trick(TrickCall),
        TrickInit(TrickInitCall),
        TrickyTrick(TrickyTrickCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleTrickCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckPasswordCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CheckPassword(decoded));
            }
            if let Ok(decoded) =
                <TargetCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Target(decoded));
            }
            if let Ok(decoded) =
                <TrickCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Trick(decoded));
            }
            if let Ok(decoded) =
                <TrickInitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TrickInit(decoded));
            }
            if let Ok(decoded) =
                <TrickyTrickCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TrickyTrick(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleTrickCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckPassword(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Target(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Trick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrickInit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrickyTrick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SimpleTrickCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::CheckPassword(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Target(element) => ::core::fmt::Display::fmt(element, f),
                Self::Trick(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrickInit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TrickyTrick(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CheckPasswordCall> for SimpleTrickCalls {
        fn from(value: CheckPasswordCall) -> Self { Self::CheckPassword(value) }
    }
    impl ::core::convert::From<TargetCall> for SimpleTrickCalls {
        fn from(value: TargetCall) -> Self { Self::Target(value) }
    }
    impl ::core::convert::From<TrickCall> for SimpleTrickCalls {
        fn from(value: TrickCall) -> Self { Self::Trick(value) }
    }
    impl ::core::convert::From<TrickInitCall> for SimpleTrickCalls {
        fn from(value: TrickInitCall) -> Self { Self::TrickInit(value) }
    }
    impl ::core::convert::From<TrickyTrickCall> for SimpleTrickCalls {
        fn from(value: TrickyTrickCall) -> Self { Self::TrickyTrick(value) }
    }
    ///Container type for all return fields from the
    /// `checkPassword` function with signature
    /// `checkPassword(uint256)` and selector `0x9e4b2e47`
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
    pub struct CheckPasswordReturn(pub bool);
    ///Container type for all return fields from the
    /// `target` function with signature `target()` and
    /// selector `0xd4b83992`
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
    pub struct TargetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `trick` function with signature `trick()` and
    /// selector `0x690da2b2`
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
    pub struct TrickReturn(pub ::ethers::core::types::Address);
}
