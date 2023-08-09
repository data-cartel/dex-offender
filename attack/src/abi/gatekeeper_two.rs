pub use gatekeeper_two::*;
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
pub mod gatekeeper_two {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("enter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        8usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes8"),
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
                    ::std::borrow::ToOwned::to_owned("entrant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("entrant"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GATEKEEPERTWO_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01\xE8\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c3p N\x14a\0;W\x80c\x9D\xB3\x1Dw\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01iV[a\0\xA8V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\x83\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0ZV[`\x0023\x03a\0\xB6W`\0\x80\xFD[3;\x80\x15a\0\xC3W`\0\x80\xFD[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\x003``\x1B\x16` \x82\x01R\x83\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`\xC0\x83\x90\x1C\x90`4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\xC0\x1C\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x014W`\0\x80\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x162\x17\x90U`\x01\x92PPP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01{W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x01\xABW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xE1\xCC\x17\xBF#$XX\x04`]\x9C\xBE\xBC\xA5\xB4d\xDD\xA9\xB1M,\x92*\xB2\x9C#2\xE65\xFB\xF6dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static GATEKEEPERTWO_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c3p N\x14a\0;W\x80c\x9D\xB3\x1Dw\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01iV[a\0\xA8V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\x83\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0ZV[`\x0023\x03a\0\xB6W`\0\x80\xFD[3;\x80\x15a\0\xC3W`\0\x80\xFD[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\x003``\x1B\x16` \x82\x01R\x83\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`\xC0\x83\x90\x1C\x90`4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\xC0\x1C\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x014W`\0\x80\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x162\x17\x90U`\x01\x92PPP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01{W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x01\xABW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xE1\xCC\x17\xBF#$XX\x04`]\x9C\xBE\xBC\xA5\xB4d\xDD\xA9\xB1M,\x92*\xB2\x9C#2\xE65\xFB\xF6dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static GATEKEEPERTWO_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct GatekeeperTwo<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GatekeeperTwo<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for GatekeeperTwo<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for GatekeeperTwo<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for GatekeeperTwo<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GatekeeperTwo))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GatekeeperTwo<M> {
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
                GATEKEEPERTWO_ABI.clone(),
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
                GATEKEEPERTWO_ABI.clone(),
                GATEKEEPERTWO_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `enter` (0x3370204e)
        /// function
        pub fn enter(
            &self,
            gate_key: [u8; 8],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([51, 112, 32, 78], gate_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `entrant` (0x9db31d77)
        /// function
        pub fn entrant(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([157, 179, 29, 119], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for GatekeeperTwo<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `enter` function with signature `enter(bytes8)` and
    /// selector `0x3370204e`
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
    #[ethcall(name = "enter", abi = "enter(bytes8)")]
    pub struct EnterCall {
        pub gate_key: [u8; 8],
    }
    ///Container type for all input parameters for the
    /// `entrant` function with signature `entrant()` and
    /// selector `0x9db31d77`
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
    #[ethcall(name = "entrant", abi = "entrant()")]
    pub struct EntrantCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum GatekeeperTwoCalls {
        Enter(EnterCall),
        Entrant(EntrantCall),
    }
    impl ::ethers::core::abi::AbiDecode for GatekeeperTwoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <EnterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Enter(decoded));
            }
            if let Ok(decoded) =
                <EntrantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Entrant(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GatekeeperTwoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Enter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Entrant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GatekeeperTwoCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Enter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Entrant(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EnterCall> for GatekeeperTwoCalls {
        fn from(value: EnterCall) -> Self { Self::Enter(value) }
    }
    impl ::core::convert::From<EntrantCall> for GatekeeperTwoCalls {
        fn from(value: EntrantCall) -> Self { Self::Entrant(value) }
    }
    ///Container type for all return fields from the
    /// `enter` function with signature `enter(bytes8)` and
    /// selector `0x3370204e`
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
    pub struct EnterReturn(pub bool);
    ///Container type for all return fields from the
    /// `entrant` function with signature `entrant()` and
    /// selector `0x9db31d77`
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
    pub struct EntrantReturn(pub ::ethers::core::types::Address);
}