pub use repeat_please::*;
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
pub mod repeat_please {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_to"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address payable"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("amount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("balance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balance"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balanceee"),
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
                    ::std::borrow::ToOwned::to_owned("donate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("donate"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("give_money"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("give_money"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("original"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("original"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Reentrance"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static REPEATPLEASE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@Rg|\xF1\x14\xCC\xD0\xD7\x80\x01`\x02U4\x80\x15a\0\x1CW`\0\x80\xFD[P`@Qa\x04\x018\x03\x80a\x04\x01\x839\x81\x81\x01`@R` \x81\x10\x15a\0?W`\0\x80\xFD[PQ`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x163\x17\x90\x91U`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90Ua\x03\x83\x80a\0~`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\xAA\x8C!|\x11a\0CW\x80c\xAA\x8C!|\x14a\x01~W\x80c\xB6\x9E\xF8\xA8\x14a\x01\xA5W\x80c\xED\x88\xC6\x8E\x14a\x01\xBAWa\x01\x1EV[\x80cF\xC7\x15\xFA\x14a\x01#W\x80cp\xD5\nk\x14a\x01aW\x80c\x8D\xA5\xCB[\x14a\x01iWa\x01\x1EV[6a\x01\x1EW`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x161\x15a\x01\x1CW`\x01T`\x02T`@\x80Q\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92RQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91c.\x1A}M\x91`$\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x01\x03W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x17W=`\0\x80>=`\0\xFD[PPPP[\0[`\0\x80\xFD[4\x80\x15a\x01/W`\0\x80\xFD[Pa\x018a\x01\xCFV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\x1Ca\x01\xEBV[4\x80\x15a\x01uW`\0\x80\xFD[Pa\x018a\x01\xEDV[4\x80\x15a\x01\x8AW`\0\x80\xFD[Pa\x01\x93a\x02\tV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x01\xB1W`\0\x80\xFD[Pa\x01\x93a\x02\x0FV[4\x80\x15a\x01\xC6W`\0\x80\xFD[Pa\x01\x1Ca\x02,V[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x02T\x81V[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x161\x90V[`\x01T`\x02T`@\x80Q~6*\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92b6*\x95\x92b\x06\x1A\x80\x92\x90\x91`$\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x85\x89\x80;\x15\x80\x15a\x02\xA5W`\0\x80\xFD[P\x88\xF1\x15\x80\x15a\x02\xB9W=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@\x80Q\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92RQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x95Pc.\x1A}M\x94P`$\x80\x82\x01\x94P`\0\x93P\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPV\xFE\xA2dipfsX\"\x12 C\xA1F\x11mq\x8BV\x04\0\xCE\xC6\x1D\nN\x9A\x8D\xF7\xA5v;!\x8D\x1CJ\t\x7F()\xD2\xC2\x99dsolcC\0\x06\x0C\x003";
    /// The bytecode of the contract.
    pub static REPEATPLEASE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\xAA\x8C!|\x11a\0CW\x80c\xAA\x8C!|\x14a\x01~W\x80c\xB6\x9E\xF8\xA8\x14a\x01\xA5W\x80c\xED\x88\xC6\x8E\x14a\x01\xBAWa\x01\x1EV[\x80cF\xC7\x15\xFA\x14a\x01#W\x80cp\xD5\nk\x14a\x01aW\x80c\x8D\xA5\xCB[\x14a\x01iWa\x01\x1EV[6a\x01\x1EW`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x161\x15a\x01\x1CW`\x01T`\x02T`@\x80Q\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92RQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91c.\x1A}M\x91`$\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x01\x03W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x17W=`\0\x80>=`\0\xFD[PPPP[\0[`\0\x80\xFD[4\x80\x15a\x01/W`\0\x80\xFD[Pa\x018a\x01\xCFV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\x1Ca\x01\xEBV[4\x80\x15a\x01uW`\0\x80\xFD[Pa\x018a\x01\xEDV[4\x80\x15a\x01\x8AW`\0\x80\xFD[Pa\x01\x93a\x02\tV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x01\xB1W`\0\x80\xFD[Pa\x01\x93a\x02\x0FV[4\x80\x15a\x01\xC6W`\0\x80\xFD[Pa\x01\x1Ca\x02,V[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x02T\x81V[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x161\x90V[`\x01T`\x02T`@\x80Q~6*\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92b6*\x95\x92b\x06\x1A\x80\x92\x90\x91`$\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x85\x89\x80;\x15\x80\x15a\x02\xA5W`\0\x80\xFD[P\x88\xF1\x15\x80\x15a\x02\xB9W=`\0\x80>=`\0\xFD[PP`\x01T`\x02T`@\x80Q\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92RQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x95Pc.\x1A}M\x94P`$\x80\x82\x01\x94P`\0\x93P\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPV\xFE\xA2dipfsX\"\x12 C\xA1F\x11mq\x8BV\x04\0\xCE\xC6\x1D\nN\x9A\x8D\xF7\xA5v;!\x8D\x1CJ\t\x7F()\xD2\xC2\x99dsolcC\0\x06\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static REPEATPLEASE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct RepeatPlease<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RepeatPlease<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for RepeatPlease<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for RepeatPlease<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for RepeatPlease<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RepeatPlease))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RepeatPlease<M> {
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
                REPEATPLEASE_ABI.clone(),
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
                REPEATPLEASE_ABI.clone(),
                REPEATPLEASE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `amount` (0xaa8c217c)
        /// function
        pub fn amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([170, 140, 33, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balance` (0xb69ef8a8)
        /// function
        pub fn balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([182, 158, 248, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `donate` (0xed88c68e)
        /// function
        pub fn donate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 136, 198, 142], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `give_money` (0x70d50a6b)
        /// function
        pub fn give_money(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 213, 10, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `original` (0x46c715fa)
        /// function
        pub fn original(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 199, 21, 250], ())
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
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for RepeatPlease<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `amount` function with signature `amount()` and
    /// selector `0xaa8c217c`
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
    #[ethcall(name = "amount", abi = "amount()")]
    pub struct AmountCall;
    ///Container type for all input parameters for the
    /// `balance` function with signature `balance()` and
    /// selector `0xb69ef8a8`
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
    #[ethcall(name = "balance", abi = "balance()")]
    pub struct BalanceCall;
    ///Container type for all input parameters for the
    /// `donate` function with signature `donate()` and
    /// selector `0xed88c68e`
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
    #[ethcall(name = "donate", abi = "donate()")]
    pub struct DonateCall;
    ///Container type for all input parameters for the
    /// `give_money` function with signature `give_money()`
    /// and selector `0x70d50a6b`
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
    #[ethcall(name = "give_money", abi = "give_money()")]
    pub struct GiveMoneyCall;
    ///Container type for all input parameters for the
    /// `original` function with signature `original()` and
    /// selector `0x46c715fa`
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
    #[ethcall(name = "original", abi = "original()")]
    pub struct OriginalCall;
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
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum RepeatPleaseCalls {
        Amount(AmountCall),
        Balance(BalanceCall),
        Donate(DonateCall),
        GiveMoney(GiveMoneyCall),
        Original(OriginalCall),
        Owner(OwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for RepeatPleaseCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Amount(decoded));
            }
            if let Ok(decoded) =
                <BalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Balance(decoded));
            }
            if let Ok(decoded) =
                <DonateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Donate(decoded));
            }
            if let Ok(decoded) =
                <GiveMoneyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GiveMoney(decoded));
            }
            if let Ok(decoded) =
                <OriginalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Original(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RepeatPleaseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Amount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Balance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Donate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GiveMoney(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Original(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RepeatPleaseCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Amount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Donate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GiveMoney(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Original(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AmountCall> for RepeatPleaseCalls {
        fn from(value: AmountCall) -> Self { Self::Amount(value) }
    }
    impl ::core::convert::From<BalanceCall> for RepeatPleaseCalls {
        fn from(value: BalanceCall) -> Self { Self::Balance(value) }
    }
    impl ::core::convert::From<DonateCall> for RepeatPleaseCalls {
        fn from(value: DonateCall) -> Self { Self::Donate(value) }
    }
    impl ::core::convert::From<GiveMoneyCall> for RepeatPleaseCalls {
        fn from(value: GiveMoneyCall) -> Self { Self::GiveMoney(value) }
    }
    impl ::core::convert::From<OriginalCall> for RepeatPleaseCalls {
        fn from(value: OriginalCall) -> Self { Self::Original(value) }
    }
    impl ::core::convert::From<OwnerCall> for RepeatPleaseCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    ///Container type for all return fields from the
    /// `amount` function with signature `amount()` and
    /// selector `0xaa8c217c`
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
    pub struct AmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `balance` function with signature `balance()` and
    /// selector `0xb69ef8a8`
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
    pub struct BalanceReturn {
        pub balanceee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the
    /// `original` function with signature `original()` and
    /// selector `0x46c715fa`
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
    pub struct OriginalReturn(pub ::ethers::core::types::Address);
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
