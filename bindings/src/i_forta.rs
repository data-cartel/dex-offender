pub use i_forta::*;
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
pub mod i_forta {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IFORTA_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IForta<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IForta<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IForta<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IForta<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IForta<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IForta)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IForta<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IFORTA_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `notify` (0xfa1fd28c) function
        pub fn notify(
            &self,
            user: ::ethers::core::types::Address,
            msg_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 31, 210, 140], (user, msg_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `raiseAlert` (0x087a43c1) function
        pub fn raise_alert(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 122, 67, 193], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDetectionBot` (0x9e927c68) function
        pub fn set_detection_bot(
            &self,
            detection_bot_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 146, 124, 104], detection_bot_address)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IForta<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `notify` function with signature `notify(address,bytes)` and selector `0xfa1fd28c`
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
    #[ethcall(name = "notify", abi = "notify(address,bytes)")]
    pub struct NotifyCall {
        pub user: ::ethers::core::types::Address,
        pub msg_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `raiseAlert` function with signature `raiseAlert(address)` and selector `0x087a43c1`
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
    #[ethcall(name = "raiseAlert", abi = "raiseAlert(address)")]
    pub struct RaiseAlertCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setDetectionBot` function with signature `setDetectionBot(address)` and selector `0x9e927c68`
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
    #[ethcall(name = "setDetectionBot", abi = "setDetectionBot(address)")]
    pub struct SetDetectionBotCall {
        pub detection_bot_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IFortaCalls {
        Notify(NotifyCall),
        RaiseAlert(RaiseAlertCall),
        SetDetectionBot(SetDetectionBotCall),
    }
    impl ::ethers::core::abi::AbiDecode for IFortaCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <NotifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Notify(decoded));
            }
            if let Ok(decoded)
                = <RaiseAlertCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RaiseAlert(decoded));
            }
            if let Ok(decoded)
                = <SetDetectionBotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDetectionBot(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IFortaCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Notify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseAlert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDetectionBot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IFortaCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Notify(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseAlert(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDetectionBot(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<NotifyCall> for IFortaCalls {
        fn from(value: NotifyCall) -> Self {
            Self::Notify(value)
        }
    }
    impl ::core::convert::From<RaiseAlertCall> for IFortaCalls {
        fn from(value: RaiseAlertCall) -> Self {
            Self::RaiseAlert(value)
        }
    }
    impl ::core::convert::From<SetDetectionBotCall> for IFortaCalls {
        fn from(value: SetDetectionBotCall) -> Self {
            Self::SetDetectionBot(value)
        }
    }
}
