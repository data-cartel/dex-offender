pub use engine::*;
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
pub mod engine {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("horsePower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("horsePower"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgrader"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgrader"),
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
    pub static ENGINE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x04\xD9\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80cO\x1E\xF2\x86\x14a\0DW\x80cVOmq\x14a\0\xFCW\x80c\x81)\xFC\x1C\x14a\x01#W\x80c\xAF&\x97E\x14a\x018W[`\0\x80\xFD[a\0\xFA`\x04\x806\x03`@\x81\x10\x15a\0ZW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015d\x01\0\0\0\0\x81\x11\x15a\0\x85W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\0\x97W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\0\xB9W`\0\x80\xFD[\x91\x90\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\x01i\x94PPPPPV[\0[4\x80\x15a\x01\x08W`\0\x80\xFD[Pa\x01\x11a\x01\x7FV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x01/W`\0\x80\xFD[Pa\0\xFAa\x01\x85V[4\x80\x15a\x01DW`\0\x80\xFD[Pa\x01Ma\x02FV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01qa\x02[V[a\x01{\x82\x82a\x02\xB2V[PPV[`\x01T\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x80a\x01\x9EWPa\x01\x9Ea\x03\xACV[\x80a\x01\xACWP`\0T`\xFF\x16\x15[a\x01\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`.\x81R` \x01\x80a\x04I`.\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x02\x12W`\0\x80T`\xFF\x19a\xFF\0\x19\x90\x91\x16a\x01\0\x17\x16`\x01\x17\x90U[a\x03\xE8`\x01U`\0\x80Tb\x01\0\0`\x01`\xB0\x1B\x03\x19\x163b\x01\0\0\x02\x17\x90U\x80\x15a\x02CW`\0\x80Ta\xFF\0\x19\x16\x90U[PV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xB0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlCan't upgrade`\x98\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[V[a\x02\xBB\x82a\x03\xBDV[\x80Q\x15a\x01{W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x02\xFEW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x02\xDFV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03^W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03cV[``\x91P[PP\x90P\x80a\x03\xA7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10\xD8[\x1B\x08\x19\x98Z[\x19Y`\xAA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPV[`\0a\x03\xB70a\x04BV[\x15\x90P\x90V[a\x03\xC6\x81a\x04BV[a\x04\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`-\x81R` \x01\x80a\x04w`-\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[;\x15\x15\x90V\xFEInitializable: contract is already initializedERC1967: new implementation is not a contract\xA2dipfsX\"\x12 \x1A\xCD\xFA\r\x9E\x89\x94\"\xD9\x9C \x81\x8A\xEA\xEAQ\xC8\xE9X4\x8F\xC0fqT\x81~\xDE\xAF&\xA1\x94dsolcC\0\x06\x0C\x003";
    /// The bytecode of the contract.
    pub static ENGINE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80cO\x1E\xF2\x86\x14a\0DW\x80cVOmq\x14a\0\xFCW\x80c\x81)\xFC\x1C\x14a\x01#W\x80c\xAF&\x97E\x14a\x018W[`\0\x80\xFD[a\0\xFA`\x04\x806\x03`@\x81\x10\x15a\0ZW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015d\x01\0\0\0\0\x81\x11\x15a\0\x85W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\0\x97W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\0\xB9W`\0\x80\xFD[\x91\x90\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\x01i\x94PPPPPV[\0[4\x80\x15a\x01\x08W`\0\x80\xFD[Pa\x01\x11a\x01\x7FV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x01/W`\0\x80\xFD[Pa\0\xFAa\x01\x85V[4\x80\x15a\x01DW`\0\x80\xFD[Pa\x01Ma\x02FV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01qa\x02[V[a\x01{\x82\x82a\x02\xB2V[PPV[`\x01T\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x80a\x01\x9EWPa\x01\x9Ea\x03\xACV[\x80a\x01\xACWP`\0T`\xFF\x16\x15[a\x01\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`.\x81R` \x01\x80a\x04I`.\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x02\x12W`\0\x80T`\xFF\x19a\xFF\0\x19\x90\x91\x16a\x01\0\x17\x16`\x01\x17\x90U[a\x03\xE8`\x01U`\0\x80Tb\x01\0\0`\x01`\xB0\x1B\x03\x19\x163b\x01\0\0\x02\x17\x90U\x80\x15a\x02CW`\0\x80Ta\xFF\0\x19\x16\x90U[PV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xB0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlCan't upgrade`\x98\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[V[a\x02\xBB\x82a\x03\xBDV[\x80Q\x15a\x01{W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x02\xFEW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x02\xDFV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03^W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03cV[``\x91P[PP\x90P\x80a\x03\xA7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10\xD8[\x1B\x08\x19\x98Z[\x19Y`\xAA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPV[`\0a\x03\xB70a\x04BV[\x15\x90P\x90V[a\x03\xC6\x81a\x04BV[a\x04\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`-\x81R` \x01\x80a\x04w`-\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[;\x15\x15\x90V\xFEInitializable: contract is already initializedERC1967: new implementation is not a contract\xA2dipfsX\"\x12 \x1A\xCD\xFA\r\x9E\x89\x94\"\xD9\x9C \x81\x8A\xEA\xEAQ\xC8\xE9X4\x8F\xC0fqT\x81~\xDE\xAF&\xA1\x94dsolcC\0\x06\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static ENGINE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Engine<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Engine<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Engine<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Engine<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Engine<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Engine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Engine<M> {
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
                ENGINE_ABI.clone(),
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
                ENGINE_ABI.clone(),
                ENGINE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `horsePower` (0x564f6d71)
        /// function
        pub fn horse_power(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([86, 79, 109, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c)
        /// function
        pub fn initialize(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall`
        /// (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrader` (0xaf269745)
        /// function
        pub fn upgrader(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([175, 38, 151, 69], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Engine<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `horsePower` function with signature `horsePower()`
    /// and selector `0x564f6d71`
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
    #[ethcall(name = "horsePower", abi = "horsePower()")]
    pub struct HorsePowerCall;
    ///Container type for all input parameters for the
    /// `initialize` function with signature `initialize()`
    /// and selector `0x8129fc1c`
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
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the
    /// `upgradeToAndCall` function with signature
    /// `upgradeToAndCall(address,bytes)` and selector
    /// `0x4f1ef286`
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
    #[ethcall(
        name = "upgradeToAndCall",
        abi = "upgradeToAndCall(address,bytes)"
    )]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `upgrader` function with signature `upgrader()` and
    /// selector `0xaf269745`
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
    #[ethcall(name = "upgrader", abi = "upgrader()")]
    pub struct UpgraderCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum EngineCalls {
        HorsePower(HorsePowerCall),
        Initialize(InitializeCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Upgrader(UpgraderCall),
    }
    impl ::ethers::core::abi::AbiDecode for EngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <HorsePowerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HorsePower(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) =
                <UpgraderCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Upgrader(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HorsePower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Upgrader(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EngineCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::HorsePower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradeToAndCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Upgrader(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<HorsePowerCall> for EngineCalls {
        fn from(value: HorsePowerCall) -> Self { Self::HorsePower(value) }
    }
    impl ::core::convert::From<InitializeCall> for EngineCalls {
        fn from(value: InitializeCall) -> Self { Self::Initialize(value) }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for EngineCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<UpgraderCall> for EngineCalls {
        fn from(value: UpgraderCall) -> Self { Self::Upgrader(value) }
    }
    ///Container type for all return fields from the
    /// `horsePower` function with signature `horsePower()`
    /// and selector `0x564f6d71`
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
    pub struct HorsePowerReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `upgrader` function with signature `upgrader()` and
    /// selector `0xaf269745`
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
    pub struct UpgraderReturn(pub ::ethers::core::types::Address);
}
