pub use i_am_true_library_contract_please_believe_me::*;
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
pub mod i_am_true_library_contract_please_believe_me {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addressToUint256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressToUint256"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_adr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lala"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("setTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTime"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_time"),
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
                    ::std::borrow::ToOwned::to_owned("timeZone1Library"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("timeZone1Library"),
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
                    ::std::borrow::ToOwned::to_owned("timeZone2Library"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("timeZone2Library"),
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
    pub static IAMTRUELIBRARYCONTRACTPLEASEBELIEVEME_ABI:
        ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\x0E\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80c=\xC7\x94\"\x11a\0PW\x80c=\xC7\x94\"\x14a\x01\rW\x80cu\x8F\xB9U\x14a\x01-W\x80c\x8D\xA5\xCB[\x14a\x01bW`\0\x80\xFD[\x80c'\xD6\x97O\x14a\0lW\x80c;\xEB&\xC4\x14a\0\xB6W[`\0\x80\xFD[`\x01Ta\0\x8C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Ba\0\xC46`\x04a\x01\x82V[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[`\0Ta\0\x8C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01Ta\x01;6`\x04a\x01\x9BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x90\x81R` \x01a\0\xADV[`\x02Ta\0\x8C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0` \x82\x84\x03\x12\x15a\x01\x94W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xADW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xD1W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 W.K!\x14\x06\xF6\xAEi\xF1=&\x95a\xAA\xC4m>&^\xAB\0(q!s.l\xE6\xEC[}dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static IAMTRUELIBRARYCONTRACTPLEASEBELIEVEME_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80c=\xC7\x94\"\x11a\0PW\x80c=\xC7\x94\"\x14a\x01\rW\x80cu\x8F\xB9U\x14a\x01-W\x80c\x8D\xA5\xCB[\x14a\x01bW`\0\x80\xFD[\x80c'\xD6\x97O\x14a\0lW\x80c;\xEB&\xC4\x14a\0\xB6W[`\0\x80\xFD[`\x01Ta\0\x8C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Ba\0\xC46`\x04a\x01\x82V[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[`\0Ta\0\x8C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01Ta\x01;6`\x04a\x01\x9BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x90\x81R` \x01a\0\xADV[`\x02Ta\0\x8C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0` \x82\x84\x03\x12\x15a\x01\x94W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xADW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xD1W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 W.K!\x14\x06\xF6\xAEi\xF1=&\x95a\xAA\xC4m>&^\xAB\0(q!s.l\xE6\xEC[}dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static IAMTRUELIBRARYCONTRACTPLEASEBELIEVEME_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct IAmTrueLibraryContractPleaseBelieveMe<M>(
        ::ethers::contract::Contract<M>,
    );
    impl<M> ::core::clone::Clone for IAmTrueLibraryContractPleaseBelieveMe<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for IAmTrueLibraryContractPleaseBelieveMe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for IAmTrueLibraryContractPleaseBelieveMe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for IAmTrueLibraryContractPleaseBelieveMe<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(
                IAmTrueLibraryContractPleaseBelieveMe
            ))
            .field(&self.address())
            .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware>
        IAmTrueLibraryContractPleaseBelieveMe<M>
    {
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
                IAMTRUELIBRARYCONTRACTPLEASEBELIEVEME_ABI.clone(),
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
                IAMTRUELIBRARYCONTRACTPLEASEBELIEVEME_ABI.clone(),
                IAMTRUELIBRARYCONTRACTPLEASEBELIEVEME_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addressToUint256`
        /// (0x758fb955) function
        pub fn address_to_uint_256(
            &self,
            adr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([117, 143, 185, 85], adr)
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
        ///Calls the contract's `setTime` (0x3beb26c4)
        /// function
        pub fn set_time(
            &self,
            time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 235, 38, 196], time)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timeZone1Library`
        /// (0x3dc79422) function
        pub fn time_zone_1_library(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([61, 199, 148, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timeZone2Library`
        /// (0x27d6974f) function
        pub fn time_zone_2_library(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([39, 214, 151, 79], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>>
        for IAmTrueLibraryContractPleaseBelieveMe<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `addressToUint256` function with signature
    /// `addressToUint256(address)` and selector
    /// `0x758fb955`
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
    #[ethcall(name = "addressToUint256", abi = "addressToUint256(address)")]
    pub struct AddressToUint256Call {
        pub adr: ::ethers::core::types::Address,
    }
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
    /// `setTime` function with signature `setTime(uint256)`
    /// and selector `0x3beb26c4`
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
    #[ethcall(name = "setTime", abi = "setTime(uint256)")]
    pub struct SetTimeCall {
        pub time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `timeZone1Library` function with signature
    /// `timeZone1Library()` and selector `0x3dc79422`
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
    #[ethcall(name = "timeZone1Library", abi = "timeZone1Library()")]
    pub struct TimeZone1LibraryCall;
    ///Container type for all input parameters for the
    /// `timeZone2Library` function with signature
    /// `timeZone2Library()` and selector `0x27d6974f`
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
    #[ethcall(name = "timeZone2Library", abi = "timeZone2Library()")]
    pub struct TimeZone2LibraryCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum IAmTrueLibraryContractPleaseBelieveMeCalls {
        AddressToUint256(AddressToUint256Call),
        Owner(OwnerCall),
        SetTime(SetTimeCall),
        TimeZone1Library(TimeZone1LibraryCall),
        TimeZone2Library(TimeZone2LibraryCall),
    }
    impl ::ethers::core::abi::AbiDecode
        for IAmTrueLibraryContractPleaseBelieveMeCalls
    {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AddressToUint256Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AddressToUint256(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <SetTimeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetTime(decoded));
            }
            if let Ok(decoded) =
                <TimeZone1LibraryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TimeZone1Library(decoded));
            }
            if let Ok(decoded) =
                <TimeZone2LibraryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TimeZone2Library(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode
        for IAmTrueLibraryContractPleaseBelieveMeCalls
    {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressToUint256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimeZone1Library(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimeZone2Library(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAmTrueLibraryContractPleaseBelieveMeCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::AddressToUint256(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimeZone1Library(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TimeZone2Library(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressToUint256Call>
        for IAmTrueLibraryContractPleaseBelieveMeCalls
    {
        fn from(value: AddressToUint256Call) -> Self {
            Self::AddressToUint256(value)
        }
    }
    impl ::core::convert::From<OwnerCall>
        for IAmTrueLibraryContractPleaseBelieveMeCalls
    {
        fn from(value: OwnerCall) -> Self { Self::Owner(value) }
    }
    impl ::core::convert::From<SetTimeCall>
        for IAmTrueLibraryContractPleaseBelieveMeCalls
    {
        fn from(value: SetTimeCall) -> Self { Self::SetTime(value) }
    }
    impl ::core::convert::From<TimeZone1LibraryCall>
        for IAmTrueLibraryContractPleaseBelieveMeCalls
    {
        fn from(value: TimeZone1LibraryCall) -> Self {
            Self::TimeZone1Library(value)
        }
    }
    impl ::core::convert::From<TimeZone2LibraryCall>
        for IAmTrueLibraryContractPleaseBelieveMeCalls
    {
        fn from(value: TimeZone2LibraryCall) -> Self {
            Self::TimeZone2Library(value)
        }
    }
    ///Container type for all return fields from the
    /// `addressToUint256` function with signature
    /// `addressToUint256(address)` and selector
    /// `0x758fb955`
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
    pub struct AddressToUint256Return {
        pub lala: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the
    /// `timeZone1Library` function with signature
    /// `timeZone1Library()` and selector `0x3dc79422`
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
    pub struct TimeZone1LibraryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the
    /// `timeZone2Library` function with signature
    /// `timeZone2Library()` and selector `0x27d6974f`
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
    pub struct TimeZone2LibraryReturn(pub ::ethers::core::types::Address);
}
