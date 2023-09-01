pub use switch::*;
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
pub mod switch {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("flipSwitch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flipSwitch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
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
                    ::std::borrow::ToOwned::to_owned("offSelector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("offSelector"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("switchOn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("switchOn"),
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
                    ::std::borrow::ToOwned::to_owned("turnSwitchOff"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("turnSwitchOff"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("turnSwitchOn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("turnSwitchOn"),
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
    pub static SWITCH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80Td\xFF\xFF\xFF\xFF\0\x19\x16d `n\x15\0\x17\x90U4\x80\x15a\0%W`\0\x80\xFD[Pa\x03\xFD\x80a\x005`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c `n\x15\x14a\0\\W\x80c0\xC1:\xDE\x14a\0fW\x80cZ,\xFAf\x14a\0yW\x80cv\"~\x12\x14a\0\xA9W\x80c\xF9\xF8\xF8\x95\x14a\0\xB1W[`\0\x80\xFD[a\0da\0\xCEV[\0[a\0da\0t6`\x04a\x02\xE7V[a\x01.V[`\0Ta\0\x8B\x90a\x01\0\x90\x04`\xE0\x1B\x81V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0da\x02UV[`\0Ta\0\xBE\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xA0V[30\x14a\x01\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FOnly the contract can call this\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16\x90UV[a\x016a\x02\xB3V[`\x04`D\x827`\0T\x81Qa\x01\0\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x01\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FCan only call the turnOffSwitch `D\x82\x01Rg3:\xB71\xBA4\xB7\xB7`\xC1\x1B`d\x82\x01R`\x84\x01a\x01\x19V[`\x000`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x01\xCC\x91\x90a\x03\x98V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x02\tW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\x0EV[``\x91P[PP\x90P\x80a\x02PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x0Cl-\x8D\x84\x0C\xCC--\x8C\xAC\x84\x07E`\x93\x1B`D\x82\x01R`d\x01a\x01\x19V[PPPV[30\x14a\x02\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FOnly the contract can call this\0`D\x82\x01R`d\x01a\x01\x19V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90UV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x02\xF9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\x11W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x03%W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x037Wa\x037a\x02\xD1V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x03_Wa\x03_a\x02\xD1V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x03xW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0\x82Q`\0[\x81\x81\x10\x15a\x03\xB9W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x03\x9FV[P`\0\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 [Z7\xFC;&\x9A\xE1\xD9\xA3\xF1=\xBD\xFC\x7F\\R\xE8\0\xB1e\x0C>\x87\x0C\x8Ck^I$a_dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static SWITCH_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c `n\x15\x14a\0\\W\x80c0\xC1:\xDE\x14a\0fW\x80cZ,\xFAf\x14a\0yW\x80cv\"~\x12\x14a\0\xA9W\x80c\xF9\xF8\xF8\x95\x14a\0\xB1W[`\0\x80\xFD[a\0da\0\xCEV[\0[a\0da\0t6`\x04a\x02\xE7V[a\x01.V[`\0Ta\0\x8B\x90a\x01\0\x90\x04`\xE0\x1B\x81V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0da\x02UV[`\0Ta\0\xBE\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xA0V[30\x14a\x01\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FOnly the contract can call this\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16\x90UV[a\x016a\x02\xB3V[`\x04`D\x827`\0T\x81Qa\x01\0\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x01\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FCan only call the turnOffSwitch `D\x82\x01Rg3:\xB71\xBA4\xB7\xB7`\xC1\x1B`d\x82\x01R`\x84\x01a\x01\x19V[`\x000`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x01\xCC\x91\x90a\x03\x98V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x02\tW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\x0EV[``\x91P[PP\x90P\x80a\x02PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x0Cl-\x8D\x84\x0C\xCC--\x8C\xAC\x84\x07E`\x93\x1B`D\x82\x01R`d\x01a\x01\x19V[PPPV[30\x14a\x02\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FOnly the contract can call this\0`D\x82\x01R`d\x01a\x01\x19V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90UV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x02\xF9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\x11W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x03%W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x037Wa\x037a\x02\xD1V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x03_Wa\x03_a\x02\xD1V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x03xW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0\x82Q`\0[\x81\x81\x10\x15a\x03\xB9W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x03\x9FV[P`\0\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 [Z7\xFC;&\x9A\xE1\xD9\xA3\xF1=\xBD\xFC\x7F\\R\xE8\0\xB1e\x0C>\x87\x0C\x8Ck^I$a_dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static SWITCH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Switch<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Switch<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Switch<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Switch<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Switch<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Switch))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Switch<M> {
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
                SWITCH_ABI.clone(),
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
                SWITCH_ABI.clone(),
                SWITCH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `flipSwitch` (0x30c13ade)
        /// function
        pub fn flip_switch(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 193, 58, 222], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `offSelector` (0x5a2cfa66)
        /// function
        pub fn off_selector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([90, 44, 250, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `switchOn` (0xf9f8f895)
        /// function
        pub fn switch_on(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([249, 248, 248, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `turnSwitchOff`
        /// (0x20606e15) function
        pub fn turn_switch_off(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 96, 110, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `turnSwitchOn` (0x76227e12)
        /// function
        pub fn turn_switch_on(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 34, 126, 18], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Switch<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `flipSwitch` function with signature
    /// `flipSwitch(bytes)` and selector `0x30c13ade`
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
    #[ethcall(name = "flipSwitch", abi = "flipSwitch(bytes)")]
    pub struct FlipSwitchCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the
    /// `offSelector` function with signature
    /// `offSelector()` and selector `0x5a2cfa66`
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
    #[ethcall(name = "offSelector", abi = "offSelector()")]
    pub struct OffSelectorCall;
    ///Container type for all input parameters for the
    /// `switchOn` function with signature `switchOn()` and
    /// selector `0xf9f8f895`
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
    #[ethcall(name = "switchOn", abi = "switchOn()")]
    pub struct SwitchOnCall;
    ///Container type for all input parameters for the
    /// `turnSwitchOff` function with signature
    /// `turnSwitchOff()` and selector `0x20606e15`
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
    #[ethcall(name = "turnSwitchOff", abi = "turnSwitchOff()")]
    pub struct TurnSwitchOffCall;
    ///Container type for all input parameters for the
    /// `turnSwitchOn` function with signature
    /// `turnSwitchOn()` and selector `0x76227e12`
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
    #[ethcall(name = "turnSwitchOn", abi = "turnSwitchOn()")]
    pub struct TurnSwitchOnCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum SwitchCalls {
        FlipSwitch(FlipSwitchCall),
        OffSelector(OffSelectorCall),
        SwitchOn(SwitchOnCall),
        TurnSwitchOff(TurnSwitchOffCall),
        TurnSwitchOn(TurnSwitchOnCall),
    }
    impl ::ethers::core::abi::AbiDecode for SwitchCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <FlipSwitchCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FlipSwitch(decoded));
            }
            if let Ok(decoded) =
                <OffSelectorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OffSelector(decoded));
            }
            if let Ok(decoded) =
                <SwitchOnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwitchOn(decoded));
            }
            if let Ok(decoded) =
                <TurnSwitchOffCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TurnSwitchOff(decoded));
            }
            if let Ok(decoded) =
                <TurnSwitchOnCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TurnSwitchOn(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SwitchCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FlipSwitch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OffSelector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwitchOn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TurnSwitchOff(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TurnSwitchOn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SwitchCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::FlipSwitch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OffSelector(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwitchOn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TurnSwitchOff(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TurnSwitchOn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FlipSwitchCall> for SwitchCalls {
        fn from(value: FlipSwitchCall) -> Self { Self::FlipSwitch(value) }
    }
    impl ::core::convert::From<OffSelectorCall> for SwitchCalls {
        fn from(value: OffSelectorCall) -> Self { Self::OffSelector(value) }
    }
    impl ::core::convert::From<SwitchOnCall> for SwitchCalls {
        fn from(value: SwitchOnCall) -> Self { Self::SwitchOn(value) }
    }
    impl ::core::convert::From<TurnSwitchOffCall> for SwitchCalls {
        fn from(value: TurnSwitchOffCall) -> Self { Self::TurnSwitchOff(value) }
    }
    impl ::core::convert::From<TurnSwitchOnCall> for SwitchCalls {
        fn from(value: TurnSwitchOnCall) -> Self { Self::TurnSwitchOn(value) }
    }
    ///Container type for all return fields from the
    /// `offSelector` function with signature
    /// `offSelector()` and selector `0x5a2cfa66`
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
    pub struct OffSelectorReturn(pub [u8; 4]);
    ///Container type for all return fields from the
    /// `switchOn` function with signature `switchOn()` and
    /// selector `0xf9f8f895`
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
    pub struct SwitchOnReturn(pub bool);
}
