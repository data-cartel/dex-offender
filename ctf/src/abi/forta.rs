pub use forta::*;
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
pub mod forta {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("botRaisedAlerts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("botRaisedAlerts"),
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
                    ::std::borrow::ToOwned::to_owned("notify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("notify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("raiseAlert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("raiseAlert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
                    ::std::borrow::ToOwned::to_owned("setDetectionBot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDetectionBot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "detectionBotAddress",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("usersDetectionBots"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("usersDetectionBots"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDetectionBot"),
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
    pub static FORTA_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x03y\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x08zC\xC1\x14a\0\\W\x80c\x9E\x10\x83\xCA\x14a\0qW\x80c\x9E\x92|h\x14a\0\xB7W\x80c\xDCp/\xA0\x14a\0\xF4W\x80c\xFA\x1F\xD2\x8C\x14a\x01\"W[`\0\x80\xFD[a\0oa\0j6`\x04a\x027V[a\x015V[\0[a\0\x9Aa\0\x7F6`\x04a\x027V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0oa\0\xC56`\x04a\x027V[3`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x01\x14a\x01\x026`\x04a\x027V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xAEV[a\0oa\x0106`\x04a\x02YV[a\x01\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x163\x14a\x01YWPV[3`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x92\x90\x91a\x01{\x90\x84\x90a\x02\xDCV[\x90\x91UPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16a\x01\xA7WPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R` \x81\x90R`@\x90\x81\x90 T\x90Qc\x11\x05[U`\xE1\x1B\x81R\x91\x16\x90c\"\n\xB6\xAA\x90a\x01\xEA\x90\x86\x90\x86\x90\x86\x90`\x04\x01a\x03\x03V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x04W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\x15WP`\x01[PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x022W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02IW`\0\x80\xFD[a\x02R\x82a\x02\x1BV[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x02nW`\0\x80\xFD[a\x02w\x84a\x02\x1BV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x94W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02\xA8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xB7W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x02\xC9W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x02\xFDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x18\x17G\xBC\x8D\xCES\xBA\xD13\xD9\xA7\xAD\x97\xE8\x84qSW\xB8(xv\x8B\x0FE\xB5A\xAAV\xACjdsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static FORTA_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x08zC\xC1\x14a\0\\W\x80c\x9E\x10\x83\xCA\x14a\0qW\x80c\x9E\x92|h\x14a\0\xB7W\x80c\xDCp/\xA0\x14a\0\xF4W\x80c\xFA\x1F\xD2\x8C\x14a\x01\"W[`\0\x80\xFD[a\0oa\0j6`\x04a\x027V[a\x015V[\0[a\0\x9Aa\0\x7F6`\x04a\x027V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0oa\0\xC56`\x04a\x027V[3`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x01\x14a\x01\x026`\x04a\x027V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xAEV[a\0oa\x0106`\x04a\x02YV[a\x01\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x163\x14a\x01YWPV[3`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x92\x90\x91a\x01{\x90\x84\x90a\x02\xDCV[\x90\x91UPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16a\x01\xA7WPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R` \x81\x90R`@\x90\x81\x90 T\x90Qc\x11\x05[U`\xE1\x1B\x81R\x91\x16\x90c\"\n\xB6\xAA\x90a\x01\xEA\x90\x86\x90\x86\x90\x86\x90`\x04\x01a\x03\x03V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x04W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\x15WP`\x01[PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x022W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02IW`\0\x80\xFD[a\x02R\x82a\x02\x1BV[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x02nW`\0\x80\xFD[a\x02w\x84a\x02\x1BV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x94W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02\xA8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xB7W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x02\xC9W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x02\xFDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x18\x17G\xBC\x8D\xCES\xBA\xD13\xD9\xA7\xAD\x97\xE8\x84qSW\xB8(xv\x8B\x0FE\xB5A\xAAV\xACjdsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static FORTA_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Forta<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Forta<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Forta<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Forta<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Forta<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Forta))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Forta<M> {
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
                FORTA_ABI.clone(),
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
                FORTA_ABI.clone(),
                FORTA_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `botRaisedAlerts`
        /// (0xdc702fa0) function
        pub fn bot_raised_alerts(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([220, 112, 47, 160], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notify` (0xfa1fd28c)
        /// function
        pub fn notify(
            &self,
            user: ::ethers::core::types::Address,
            msg_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 31, 210, 140], (user, msg_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `raiseAlert` (0x087a43c1)
        /// function
        pub fn raise_alert(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 122, 67, 193], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDetectionBot`
        /// (0x9e927c68) function
        pub fn set_detection_bot(
            &self,
            detection_bot_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 146, 124, 104], detection_bot_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usersDetectionBots`
        /// (0x9e1083ca) function
        pub fn users_detection_bots(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([158, 16, 131, 202], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Forta<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `botRaisedAlerts` function with signature
    /// `botRaisedAlerts(address)` and selector `0xdc702fa0`
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
    #[ethcall(name = "botRaisedAlerts", abi = "botRaisedAlerts(address)")]
    pub struct BotRaisedAlertsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the
    /// `notify` function with signature
    /// `notify(address,bytes)` and selector `0xfa1fd28c`
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
    #[ethcall(name = "notify", abi = "notify(address,bytes)")]
    pub struct NotifyCall {
        pub user: ::ethers::core::types::Address,
        pub msg_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `raiseAlert` function with signature
    /// `raiseAlert(address)` and selector `0x087a43c1`
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
    #[ethcall(name = "raiseAlert", abi = "raiseAlert(address)")]
    pub struct RaiseAlertCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `setDetectionBot` function with signature
    /// `setDetectionBot(address)` and selector `0x9e927c68`
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
    #[ethcall(name = "setDetectionBot", abi = "setDetectionBot(address)")]
    pub struct SetDetectionBotCall {
        pub detection_bot_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `usersDetectionBots` function with signature
    /// `usersDetectionBots(address)` and selector
    /// `0x9e1083ca`
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
    #[ethcall(name = "usersDetectionBots", abi = "usersDetectionBots(address)")]
    pub struct UsersDetectionBotsCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum FortaCalls {
        BotRaisedAlerts(BotRaisedAlertsCall),
        Notify(NotifyCall),
        RaiseAlert(RaiseAlertCall),
        SetDetectionBot(SetDetectionBotCall),
        UsersDetectionBots(UsersDetectionBotsCall),
    }
    impl ::ethers::core::abi::AbiDecode for FortaCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BotRaisedAlertsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::BotRaisedAlerts(decoded));
            }
            if let Ok(decoded) =
                <NotifyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Notify(decoded));
            }
            if let Ok(decoded) =
                <RaiseAlertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RaiseAlert(decoded));
            }
            if let Ok(decoded) =
                <SetDetectionBotCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetDetectionBot(decoded));
            }
            if let Ok(decoded) = <UsersDetectionBotsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsersDetectionBots(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FortaCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BotRaisedAlerts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Notify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RaiseAlert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDetectionBot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsersDetectionBots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FortaCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::BotRaisedAlerts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Notify(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseAlert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDetectionBot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UsersDetectionBots(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BotRaisedAlertsCall> for FortaCalls {
        fn from(value: BotRaisedAlertsCall) -> Self {
            Self::BotRaisedAlerts(value)
        }
    }
    impl ::core::convert::From<NotifyCall> for FortaCalls {
        fn from(value: NotifyCall) -> Self { Self::Notify(value) }
    }
    impl ::core::convert::From<RaiseAlertCall> for FortaCalls {
        fn from(value: RaiseAlertCall) -> Self { Self::RaiseAlert(value) }
    }
    impl ::core::convert::From<SetDetectionBotCall> for FortaCalls {
        fn from(value: SetDetectionBotCall) -> Self {
            Self::SetDetectionBot(value)
        }
    }
    impl ::core::convert::From<UsersDetectionBotsCall> for FortaCalls {
        fn from(value: UsersDetectionBotsCall) -> Self {
            Self::UsersDetectionBots(value)
        }
    }
    ///Container type for all return fields from the
    /// `botRaisedAlerts` function with signature
    /// `botRaisedAlerts(address)` and selector `0xdc702fa0`
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
    pub struct BotRaisedAlertsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `usersDetectionBots` function with signature
    /// `usersDetectionBots(address)` and selector
    /// `0x9e1083ca`
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
    pub struct UsersDetectionBotsReturn(pub ::ethers::core::types::Address);
}
