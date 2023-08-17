pub use fallout::*;
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
pub mod fallout {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Fal1out"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("Fal1out"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocate"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allocatorBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocatorBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allocator"),
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
                    ::std::borrow::ToOwned::to_owned("collectAllocations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("collectAllocations"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendAllocation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendAllocation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allocator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
    pub static FALLOUT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03A\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80co\xAB]\xDF\x14a\0ZW\x80c\x8A\xA9o8\x14a\0dW\x80c\x8D\xA5\xCB[\x14a\0yW\x80c\xA2\xDE\xA2o\x14a\0\xAAW\x80c\xAB\xAA\x99\x16\x14a\0\xDDW\x80c\xFF\xD4\x0BV\x14a\0\xE5W[`\0\x80\xFD[a\0ba\x01*V[\0[4\x80\x15a\0pW`\0\x80\xFD[Pa\0ba\x01ZV[4\x80\x15a\0\x85W`\0\x80\xFD[Pa\0\x8Ea\x01\xE8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\0\xB6W`\0\x80\xFD[Pa\0b`\x04\x806\x03` \x81\x10\x15a\0\xCDW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x01\xF7V[a\0ba\x02]V[4\x80\x15a\0\xF1W`\0\x80\xFD[Pa\x01\x18`\x04\x806\x03` \x81\x10\x15a\x01\x08W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x02\x8FV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90\x81\x90U`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 4\x90UV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xB9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@Q3\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xE5W=`\0\x80>=`\0\xFD[PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 Ta\x02\x19W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 T\x90Q\x81\x15a\x08\xFC\x02\x92\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02YW=`\0\x80>=`\0\xFD[PPV[3`\0\x90\x81R` \x81\x90R`@\x90 Ta\x02}\x904c\xFF\xFF\xFF\xFFa\x02\xAA\x16V[3`\0\x90\x81R` \x81\x90R`@\x90 UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`\0\x82\x82\x01\x83\x81\x10\x15a\x03\x04W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSafeMath: addition overflow\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x02\x1A\xAE\x9F\x1F\x04\x9E! )k\xCA\x97\xD6o\x87[\r \x8C\x1C\xB0B\xFB\xFB\xA5qpi\xF8\x03\x9CdsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static FALLOUT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0UW`\x005`\xE0\x1C\x80co\xAB]\xDF\x14a\0ZW\x80c\x8A\xA9o8\x14a\0dW\x80c\x8D\xA5\xCB[\x14a\0yW\x80c\xA2\xDE\xA2o\x14a\0\xAAW\x80c\xAB\xAA\x99\x16\x14a\0\xDDW\x80c\xFF\xD4\x0BV\x14a\0\xE5W[`\0\x80\xFD[a\0ba\x01*V[\0[4\x80\x15a\0pW`\0\x80\xFD[Pa\0ba\x01ZV[4\x80\x15a\0\x85W`\0\x80\xFD[Pa\0\x8Ea\x01\xE8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\0\xB6W`\0\x80\xFD[Pa\0b`\x04\x806\x03` \x81\x10\x15a\0\xCDW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x01\xF7V[a\0ba\x02]V[4\x80\x15a\0\xF1W`\0\x80\xFD[Pa\x01\x18`\x04\x806\x03` \x81\x10\x15a\x01\x08W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x02\x8FV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90\x81\x90U`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 4\x90UV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xB9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@Q3\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xE5W=`\0\x80>=`\0\xFD[PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 Ta\x02\x19W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 T\x90Q\x81\x15a\x08\xFC\x02\x92\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02YW=`\0\x80>=`\0\xFD[PPV[3`\0\x90\x81R` \x81\x90R`@\x90 Ta\x02}\x904c\xFF\xFF\xFF\xFFa\x02\xAA\x16V[3`\0\x90\x81R` \x81\x90R`@\x90 UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`\0\x82\x82\x01\x83\x81\x10\x15a\x03\x04W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSafeMath: addition overflow\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x02\x1A\xAE\x9F\x1F\x04\x9E! )k\xCA\x97\xD6o\x87[\r \x8C\x1C\xB0B\xFB\xFB\xA5qpi\xF8\x03\x9CdsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static FALLOUT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Fallout<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Fallout<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Fallout<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Fallout<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Fallout<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Fallout))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Fallout<M> {
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
                FALLOUT_ABI.clone(),
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
                FALLOUT_ABI.clone(),
                FALLOUT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `Fal1out` (0x6fab5ddf)
        /// function
        pub fn fal_1out(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 171, 93, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocate` (0xabaa9916)
        /// function
        pub fn allocate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 170, 153, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocatorBalance`
        /// (0xffd40b56) function
        pub fn allocator_balance(
            &self,
            allocator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([255, 212, 11, 86], allocator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collectAllocations`
        /// (0x8aa96f38) function
        pub fn collect_allocations(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 169, 111, 56], ())
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
        ///Calls the contract's `sendAllocation`
        /// (0xa2dea26f) function
        pub fn send_allocation(
            &self,
            allocator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 222, 162, 111], allocator)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Fallout<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `Fal1out` function with signature `Fal1out()` and
    /// selector `0x6fab5ddf`
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
    #[ethcall(name = "Fal1out", abi = "Fal1out()")]
    pub struct Fal1OutCall;
    ///Container type for all input parameters for the
    /// `allocate` function with signature `allocate()` and
    /// selector `0xabaa9916`
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
    #[ethcall(name = "allocate", abi = "allocate()")]
    pub struct AllocateCall;
    ///Container type for all input parameters for the
    /// `allocatorBalance` function with signature
    /// `allocatorBalance(address)` and selector
    /// `0xffd40b56`
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
    #[ethcall(name = "allocatorBalance", abi = "allocatorBalance(address)")]
    pub struct AllocatorBalanceCall {
        pub allocator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `collectAllocations` function with signature
    /// `collectAllocations()` and selector `0x8aa96f38`
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
    #[ethcall(name = "collectAllocations", abi = "collectAllocations()")]
    pub struct CollectAllocationsCall;
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
    /// `sendAllocation` function with signature
    /// `sendAllocation(address)` and selector `0xa2dea26f`
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
    #[ethcall(name = "sendAllocation", abi = "sendAllocation(address)")]
    pub struct SendAllocationCall {
        pub allocator: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum FalloutCalls {
        Fal1Out(Fal1OutCall),
        Allocate(AllocateCall),
        AllocatorBalance(AllocatorBalanceCall),
        CollectAllocations(CollectAllocationsCall),
        Owner(OwnerCall),
        SendAllocation(SendAllocationCall),
    }
    impl ::ethers::core::abi::AbiDecode for FalloutCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <Fal1OutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Fal1Out(decoded));
            }
            if let Ok(decoded) =
                <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) =
                <AllocatorBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AllocatorBalance(decoded));
            }
            if let Ok(decoded)
                = <CollectAllocationsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CollectAllocations(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <SendAllocationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SendAllocation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FalloutCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Fal1Out(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllocatorBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollectAllocations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendAllocation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FalloutCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Fal1Out(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllocatorBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollectAllocations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendAllocation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Fal1OutCall> for FalloutCalls {
        fn from(value: Fal1OutCall) -> Self { Self::Fal1Out(value) }
    }
    impl ::core::convert::From<AllocateCall> for FalloutCalls {
        fn from(value: AllocateCall) -> Self { Self::Allocate(value) }
    }
    impl ::core::convert::From<AllocatorBalanceCall> for FalloutCalls {
        fn from(value: AllocatorBalanceCall) -> Self {
            Self::AllocatorBalance(value)
        }
    }
    impl ::core::convert::From<CollectAllocationsCall> for FalloutCalls {
        fn from(value: CollectAllocationsCall) -> Self {
            Self::CollectAllocations(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for FalloutCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<SendAllocationCall> for FalloutCalls {
        fn from(value: SendAllocationCall) -> Self {
            Self::SendAllocation(value)
        }
    }
    ///Container type for all return fields from the
    /// `allocatorBalance` function with signature
    /// `allocatorBalance(address)` and selector
    /// `0xffd40b56`
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
    pub struct AllocatorBalanceReturn(pub ::ethers::core::types::U256);
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
