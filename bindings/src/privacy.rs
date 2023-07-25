pub use privacy::*;
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
pub mod privacy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_data"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ),
                            3usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes32[3]"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ID"),
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
                    ::std::borrow::ToOwned::to_owned("locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("locked"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unlock"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        16usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes16"),
                                    ),
                                },
                            ],
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
    pub static PRIVACY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91UB\x90\x81\x90U`\x02\x80Ta\xFF\xFF\x90\x92\x16b\x01\0\0\x02c\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17a\xFF\n\x17\x90U4\x80\x15a\0GW`\0\x80\xFD[P`@Qa\x02z8\x03\x80a\x02z\x839\x81\x01`@\x81\x90Ra\0f\x91a\0\xCCV[a\0r`\x03\x82\x81a\0yV[PPa\x01WV[\x82`\x03\x81\x01\x92\x82\x15a\0\xA7W\x91` \x02\x82\x01[\x82\x81\x11\x15a\0\xA7W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\0\x8CV[Pa\0\xB3\x92\x91Pa\0\xB7V[P\x90V[[\x80\x82\x11\x15a\0\xB3W`\0\x81U`\x01\x01a\0\xB8V[`\0``\x82\x84\x03\x12\x15a\0\xDEW`\0\x80\xFD[\x82`\x1F\x83\x01\x12a\0\xEDW`\0\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\x1DWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x80``\x84\x01\x85\x81\x11\x15a\x012W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x01LW\x80Q\x83R` \x92\x83\x01\x92\x01a\x014V[P\x91\x95\x94PPPPPV[a\x01\x14\x80a\x01f`\09`\0\xF3\xFE`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`<W`\x005`\xE0\x1C\x80c\xB3\xCE\xA2\x17\x14`AW\x80c\xCF0\x90\x12\x14`\\W\x80c\xE1\xAF\xB0\x8C\x14`wW[`\0\x80\xFD[`I`\x01T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`h\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`SV[`\x86`\x826`\x04`\xAFV[`\x88V[\0[`\x05T`\x01`\x01`\x80\x1B\x03\x19\x82\x81\x16\x91\x16\x14`\xA2W`\0\x80\xFD[P`\0\x80T`\xFF\x19\x16\x90UV[`\0` \x82\x84\x03\x12\x15`\xC0W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x19\x81\x16\x81\x14`\xD7W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 Va3\x829\x95\x95\xF0\xDB\x92\x0E\xEE\xDE\x9E\xA1?1\xA3\x8E)\xAA\x0B\xE6\x94fWA\xEF\xF1\xD4\xCC\x91dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static PRIVACY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`<W`\x005`\xE0\x1C\x80c\xB3\xCE\xA2\x17\x14`AW\x80c\xCF0\x90\x12\x14`\\W\x80c\xE1\xAF\xB0\x8C\x14`wW[`\0\x80\xFD[`I`\x01T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`h\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`SV[`\x86`\x826`\x04`\xAFV[`\x88V[\0[`\x05T`\x01`\x01`\x80\x1B\x03\x19\x82\x81\x16\x91\x16\x14`\xA2W`\0\x80\xFD[P`\0\x80T`\xFF\x19\x16\x90UV[`\0` \x82\x84\x03\x12\x15`\xC0W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x19\x81\x16\x81\x14`\xD7W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 Va3\x829\x95\x95\xF0\xDB\x92\x0E\xEE\xDE\x9E\xA1?1\xA3\x8E)\xAA\x0B\xE6\x94fWA\xEF\xF1\xD4\xCC\x91dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static PRIVACY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Privacy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Privacy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Privacy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Privacy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Privacy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Privacy)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Privacy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PRIVACY_ABI.clone(),
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
                PRIVACY_ABI.clone(),
                PRIVACY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ID` (0xb3cea217) function
        pub fn id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 206, 162, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked` (0xcf309012) function
        pub fn locked(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([207, 48, 144, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unlock` (0xe1afb08c) function
        pub fn unlock(
            &self,
            key: [u8; 16],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 175, 176, 140], key)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Privacy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `ID` function with signature `ID()` and selector `0xb3cea217`
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
    #[ethcall(name = "ID", abi = "ID()")]
    pub struct IdCall;
    ///Container type for all input parameters for the `locked` function with signature `locked()` and selector `0xcf309012`
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
    #[ethcall(name = "locked", abi = "locked()")]
    pub struct LockedCall;
    ///Container type for all input parameters for the `unlock` function with signature `unlock(bytes16)` and selector `0xe1afb08c`
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
    #[ethcall(name = "unlock", abi = "unlock(bytes16)")]
    pub struct UnlockCall {
        pub key: [u8; 16],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PrivacyCalls {
        Id(IdCall),
        Locked(LockedCall),
        Unlock(UnlockCall),
    }
    impl ::ethers::core::abi::AbiDecode for PrivacyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <IdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Id(decoded));
            }
            if let Ok(decoded)
                = <LockedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded)
                = <UnlockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unlock(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PrivacyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Id(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unlock(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PrivacyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Id(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unlock(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IdCall> for PrivacyCalls {
        fn from(value: IdCall) -> Self {
            Self::Id(value)
        }
    }
    impl ::core::convert::From<LockedCall> for PrivacyCalls {
        fn from(value: LockedCall) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<UnlockCall> for PrivacyCalls {
        fn from(value: UnlockCall) -> Self {
            Self::Unlock(value)
        }
    }
    ///Container type for all return fields from the `ID` function with signature `ID()` and selector `0xb3cea217`
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
    pub struct IdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `locked` function with signature `locked()` and selector `0xcf309012`
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
    pub struct LockedReturn(pub bool);
}
