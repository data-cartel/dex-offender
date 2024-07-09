pub use denial::*;
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
pub mod denial {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("contractBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contractBalance"),
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
                    ::std::borrow::ToOwned::to_owned("partner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("partner"),
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
                    ::std::borrow::ToOwned::to_owned("setWithdrawPartner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setWithdrawPartner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_partner"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DENIAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x02\x91\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0NW`\x005`\xE0\x1C\x80c<\xCF\xD6\x0B\x14a\0ZW\x80cN\x1CY\x14\x14a\0qW\x80c\x8Bz\xFE.\x14a\0\xAEW\x80c\x8D\xA5\xCB[\x14a\0\xCEW\x80c\xBE\x10\x86+\x14a\0\xFCW`\0\x80\xFD[6a\0UW\0[`\0\x80\xFD[4\x80\x15a\0fW`\0\x80\xFD[Pa\0oa\x01\x1CV[\0[4\x80\x15a\0}W`\0\x80\xFD[Pa\0oa\0\x8C6`\x04a\x01\xE2V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\0\xBAW`\0\x80\xFD[P`@QG\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\0\xE4a\n\x9E\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC5V[4\x80\x15a\x01\x08W`\0\x80\xFD[P`\0Ta\0\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0a\x01)`dGa\x02\x12V[`\0\x80T`@Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91\x83\x91\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01vW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01{V[``\x91P[PP`@Qa\n\x9E\x91P\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xADW=`\0\x80>=`\0\xFD[PB`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x02` R`@\x81 \x80T\x83\x92\x90a\x01\xDA\x90\x84\x90a\x024V[\x90\x91UPPPV[`\0` \x82\x84\x03\x12\x15a\x01\xF4W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x0BW`\0\x80\xFD[\x93\x92PPPV[`\0\x82a\x02/WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x02UWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xF4:\tn\xCF+\x9A\xE9\xA0b\xDA%N< \xD0s\xDD(\x0E\xE9Nv\xD3\xD9\x05\x06\x1A#\x97fTdsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static DENIAL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0NW`\x005`\xE0\x1C\x80c<\xCF\xD6\x0B\x14a\0ZW\x80cN\x1CY\x14\x14a\0qW\x80c\x8Bz\xFE.\x14a\0\xAEW\x80c\x8D\xA5\xCB[\x14a\0\xCEW\x80c\xBE\x10\x86+\x14a\0\xFCW`\0\x80\xFD[6a\0UW\0[`\0\x80\xFD[4\x80\x15a\0fW`\0\x80\xFD[Pa\0oa\x01\x1CV[\0[4\x80\x15a\0}W`\0\x80\xFD[Pa\0oa\0\x8C6`\x04a\x01\xE2V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\0\xBAW`\0\x80\xFD[P`@QG\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\0\xE4a\n\x9E\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC5V[4\x80\x15a\x01\x08W`\0\x80\xFD[P`\0Ta\0\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0a\x01)`dGa\x02\x12V[`\0\x80T`@Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91\x83\x91\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01vW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01{V[``\x91P[PP`@Qa\n\x9E\x91P\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xADW=`\0\x80>=`\0\xFD[PB`\x01U`\0\x80T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x02` R`@\x81 \x80T\x83\x92\x90a\x01\xDA\x90\x84\x90a\x024V[\x90\x91UPPPV[`\0` \x82\x84\x03\x12\x15a\x01\xF4W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x0BW`\0\x80\xFD[\x93\x92PPPV[`\0\x82a\x02/WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x02UWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xF4:\tn\xCF+\x9A\xE9\xA0b\xDA%N< \xD0s\xDD(\x0E\xE9Nv\xD3\xD9\x05\x06\x1A#\x97fTdsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static DENIAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Denial<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Denial<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Denial<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Denial<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Denial<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Denial))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Denial<M> {
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
                DENIAL_ABI.clone(),
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
                DENIAL_ABI.clone(),
                DENIAL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `contractBalance`
        /// (0x8b7afe2e) function
        pub fn contract_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([139, 122, 254, 46], ())
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
        ///Calls the contract's `partner` (0xbe10862b)
        /// function
        pub fn partner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([190, 16, 134, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWithdrawPartner`
        /// (0x4e1c5914) function
        pub fn set_withdraw_partner(
            &self,
            partner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 28, 89, 20], partner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x3ccfd60b)
        /// function
        pub fn withdraw(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Denial<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `contractBalance` function with signature
    /// `contractBalance()` and selector `0x8b7afe2e`
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
    #[ethcall(name = "contractBalance", abi = "contractBalance()")]
    pub struct ContractBalanceCall;
    ///Container type for all input parameters for the
    /// `owner` function with signature `owner()` and
    /// selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the
    /// `partner` function with signature `partner()` and
    /// selector `0xbe10862b`
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
    #[ethcall(name = "partner", abi = "partner()")]
    pub struct PartnerCall;
    ///Container type for all input parameters for the
    /// `setWithdrawPartner` function with signature
    /// `setWithdrawPartner(address)` and selector
    /// `0x4e1c5914`
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
    #[ethcall(name = "setWithdrawPartner", abi = "setWithdrawPartner(address)")]
    pub struct SetWithdrawPartnerCall {
        pub partner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `withdraw` function with signature `withdraw()` and
    /// selector `0x3ccfd60b`
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
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
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
    pub enum DenialCalls {
        ContractBalance(ContractBalanceCall),
        Owner(OwnerCall),
        Partner(PartnerCall),
        SetWithdrawPartner(SetWithdrawPartnerCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for DenialCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ContractBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ContractBalance(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PartnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Partner(decoded));
            }
            if let Ok(decoded) = <SetWithdrawPartnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetWithdrawPartner(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DenialCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ContractBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Partner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetWithdrawPartner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DenialCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ContractBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Partner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawPartner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ContractBalanceCall> for DenialCalls {
        fn from(value: ContractBalanceCall) -> Self {
            Self::ContractBalance(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DenialCalls {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<PartnerCall> for DenialCalls {
        fn from(value: PartnerCall) -> Self { Self::Partner(value) }
    }
    impl ::core::convert::From<SetWithdrawPartnerCall> for DenialCalls {
        fn from(value: SetWithdrawPartnerCall) -> Self {
            Self::SetWithdrawPartner(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for DenialCalls {
        fn from(value: WithdrawCall) -> Self { Self::Withdraw(value) }
    }
    ///Container type for all return fields from the
    /// `contractBalance` function with signature
    /// `contractBalance()` and selector `0x8b7afe2e`
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
    pub struct ContractBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the
    /// `owner` function with signature `owner()` and
    /// selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `partner` function with signature `partner()` and
    /// selector `0xbe10862b`
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
    pub struct PartnerReturn(pub ::ethers::core::types::Address);
}
