pub use king::*;
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
pub mod king {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_king"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_king"),
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
                    ::std::borrow::ToOwned::to_owned("prize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prize"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static KING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x163\x90\x81\x17\x90\x92U`\0\x80T\x90\x91\x16\x90\x91\x17\x90U4`\x01Ua\x01l\x80a\0:`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\08W`\x005`\xE0\x1C\x80c)\xCCmo\x14a\0\xBBW\x80c\x8D\xA5\xCB[\x14a\0\xF2W\x80c\xE3\xAC]&\x14a\x01\x12W`\0\x80\xFD[6a\0\xB6W`\x01T4\x10\x15\x80a\0XWP`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\0aW`\0\x80\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x914\x80\x15a\x08\xFC\x02\x92\x90\x91\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\x9BW=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U4`\x01\x81\x90U\0[`\0\x80\xFD[4\x80\x15a\0\xC7W`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xFEW`\0\x80\xFD[P`\x02Ta\0\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\x1EW`\0\x80\xFD[Pa\x01(`\x01T\x81V[`@Q\x90\x81R` \x01a\0\xE9V\xFE\xA2dipfsX\"\x12 \xD3\xF3\xCBX\x14\xC0\x01\xFA\xBF\x0E\xFA_\x8EA\xD6\t\x0F\x1CzA\x11\x01\xC1\x19Z\x89\xAE\xE1\xD5\x0E\n\xD7dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static KING_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\08W`\x005`\xE0\x1C\x80c)\xCCmo\x14a\0\xBBW\x80c\x8D\xA5\xCB[\x14a\0\xF2W\x80c\xE3\xAC]&\x14a\x01\x12W`\0\x80\xFD[6a\0\xB6W`\x01T4\x10\x15\x80a\0XWP`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\0aW`\0\x80\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x914\x80\x15a\x08\xFC\x02\x92\x90\x91\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\x9BW=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U4`\x01\x81\x90U\0[`\0\x80\xFD[4\x80\x15a\0\xC7W`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xFEW`\0\x80\xFD[P`\x02Ta\0\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\x1EW`\0\x80\xFD[Pa\x01(`\x01T\x81V[`@Q\x90\x81R` \x01a\0\xE9V\xFE\xA2dipfsX\"\x12 \xD3\xF3\xCBX\x14\xC0\x01\xFA\xBF\x0E\xFA_\x8EA\xD6\t\x0F\x1CzA\x11\x01\xC1\x19Z\x89\xAE\xE1\xD5\x0E\n\xD7dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static KING_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct King<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for King<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for King<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for King<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for King<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(King)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> King<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KING_ABI.clone(),
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
                KING_ABI.clone(),
                KING_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `_king` (0x29cc6d6f) function
        pub fn king(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([41, 204, 109, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
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
        ///Calls the contract's `prize` (0xe3ac5d26) function
        pub fn prize(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 172, 93, 38], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for King<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `_king` function with signature `_king()` and selector `0x29cc6d6f`
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
    #[ethcall(name = "_king", abi = "_king()")]
    pub struct KingCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `prize` function with signature `prize()` and selector `0xe3ac5d26`
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
    #[ethcall(name = "prize", abi = "prize()")]
    pub struct PrizeCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum KingCalls {
        King(KingCall),
        Owner(OwnerCall),
        Prize(PrizeCall),
    }
    impl ::ethers::core::abi::AbiDecode for KingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <KingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::King(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PrizeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Prize(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::King(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Prize(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for KingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::King(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prize(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<KingCall> for KingCalls {
        fn from(value: KingCall) -> Self {
            Self::King(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for KingCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PrizeCall> for KingCalls {
        fn from(value: PrizeCall) -> Self {
            Self::Prize(value)
        }
    }
    ///Container type for all return fields from the `_king` function with signature `_king()` and selector `0x29cc6d6f`
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
    pub struct KingReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `prize` function with signature `prize()` and selector `0xe3ac5d26`
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
    pub struct PrizeReturn(pub ::ethers::core::types::U256);
}
