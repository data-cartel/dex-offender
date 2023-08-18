pub use elevator::*;
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
pub mod elevator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("floor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("floor"),
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
                    ::std::borrow::ToOwned::to_owned("goTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("goTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_floor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("top"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("top"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ELEVATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02W\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c@iSc\x14a\0FW\x80c\xED\x9Aq4\x14a\0bW\x80c\xFEm\xCD\xBA\x14a\0wW[`\0\x80\xFD[a\0O`\x01T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0ua\0p6`\x04a\x01\xDFV[a\0\x94V[\0[`\0Ta\0\x84\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0YV[`@Q\x7F_\x9AK\xCA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R3\x90\x81\x90c_\x9AK\xCA\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x11\x91\x90a\x01\xF8V[a\x01\xDBW`\x01\x82\x90U`@Q\x7F_\x9AK\xCA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c_\x9AK\xCA\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xAB\x91\x90a\x01\xF8V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U[PPV[`\0` \x82\x84\x03\x12\x15a\x01\xF1W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\nW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\x1AW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xA4\xD4\xA7D\x03\xE52F\x19h\xCC\x94 \x1FP+\x95\xD4z\x84 \r\x90\xEE\xAC`\x05\x93\x1F\xAF\xEA\xD8dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static ELEVATOR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c@iSc\x14a\0FW\x80c\xED\x9Aq4\x14a\0bW\x80c\xFEm\xCD\xBA\x14a\0wW[`\0\x80\xFD[a\0O`\x01T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0ua\0p6`\x04a\x01\xDFV[a\0\x94V[\0[`\0Ta\0\x84\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0YV[`@Q\x7F_\x9AK\xCA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R3\x90\x81\x90c_\x9AK\xCA\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x11\x91\x90a\x01\xF8V[a\x01\xDBW`\x01\x82\x90U`@Q\x7F_\x9AK\xCA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c_\x9AK\xCA\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xAB\x91\x90a\x01\xF8V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U[PPV[`\0` \x82\x84\x03\x12\x15a\x01\xF1W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\nW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\x1AW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xA4\xD4\xA7D\x03\xE52F\x19h\xCC\x94 \x1FP+\x95\xD4z\x84 \r\x90\xEE\xAC`\x05\x93\x1F\xAF\xEA\xD8dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static ELEVATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Elevator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Elevator<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Elevator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Elevator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Elevator<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Elevator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Elevator<M> {
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
                ELEVATOR_ABI.clone(),
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
                ELEVATOR_ABI.clone(),
                ELEVATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `floor` (0x40695363)
        /// function
        pub fn floor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([64, 105, 83, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `goTo` (0xed9a7134)
        /// function
        pub fn go_to(
            &self,
            floor: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 154, 113, 52], floor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `top` (0xfe6dcdba) function
        pub fn top(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([254, 109, 205, 186], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Elevator<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `floor` function with signature `floor()` and
    /// selector `0x40695363`
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
    #[ethcall(name = "floor", abi = "floor()")]
    pub struct FloorCall;
    ///Container type for all input parameters for the
    /// `goTo` function with signature `goTo(uint256)` and
    /// selector `0xed9a7134`
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
    #[ethcall(name = "goTo", abi = "goTo(uint256)")]
    pub struct GoToCall {
        pub floor: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `top` function with signature `top()` and selector
    /// `0xfe6dcdba`
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
    #[ethcall(name = "top", abi = "top()")]
    pub struct TopCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum ElevatorCalls {
        Floor(FloorCall),
        GoTo(GoToCall),
        Top(TopCall),
    }
    impl ::ethers::core::abi::AbiDecode for ElevatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <FloorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Floor(decoded));
            }
            if let Ok(decoded) =
                <GoToCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GoTo(decoded));
            }
            if let Ok(decoded) =
                <TopCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Top(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ElevatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Floor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GoTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Top(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ElevatorCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Floor(element) => ::core::fmt::Display::fmt(element, f),
                Self::GoTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::Top(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FloorCall> for ElevatorCalls {
        fn from(value: FloorCall) -> Self { Self::Floor(value) }
    }
    impl ::core::convert::From<GoToCall> for ElevatorCalls {
        fn from(value: GoToCall) -> Self { Self::GoTo(value) }
    }
    impl ::core::convert::From<TopCall> for ElevatorCalls {
        fn from(value: TopCall) -> Self { Self::Top(value) }
    }
    ///Container type for all return fields from the
    /// `floor` function with signature `floor()` and
    /// selector `0x40695363`
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
    pub struct FloorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `top`
    /// function with signature `top()` and selector
    /// `0xfe6dcdba`
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
    pub struct TopReturn(pub bool);
}
