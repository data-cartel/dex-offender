pub use i_simple_governance::*;
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
pub mod i_simple_governance {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("executeAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeAction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returndata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("action"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISimpleGovernance.GovernanceAction",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getActionCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActionCounter"),
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
                    ::std::borrow::ToOwned::to_owned("getActionDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActionDelay"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delay"),
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
                    ::std::borrow::ToOwned::to_owned("getGovernanceToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGovernanceToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("queueAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queueAction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActionExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ActionExecuted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ActionQueued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ActionQueued"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActionFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ActionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotExecute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CannotExecute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTarget"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTarget"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughVotes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("who"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TargetMustHaveCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TargetMustHaveCode"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISIMPLEGOVERNANCE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct ISimpleGovernance<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISimpleGovernance<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISimpleGovernance<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISimpleGovernance<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISimpleGovernance<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ISimpleGovernance))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISimpleGovernance<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISIMPLEGOVERNANCE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `executeAction` (0xc0c1cf55) function
        pub fn execute_action(
            &self,
            action_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([192, 193, 207, 85], action_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAction` (0xb6e76873) function
        pub fn get_action(
            &self,
            action_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, GovernanceAction> {
            self.0
                .method_hash([182, 231, 104, 115], action_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionCounter` (0x9aca08d4) function
        pub fn get_action_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 202, 8, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionDelay` (0x12057a14) function
        pub fn get_action_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 5, 122, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGovernanceToken` (0x3f8a037d) function
        pub fn get_governance_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 138, 3, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queueAction` (0x52ecb90a) function
        pub fn queue_action(
            &self,
            target: ::ethers::core::types::Address,
            value: u128,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([82, 236, 185, 10], (target, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ActionExecuted` event
        pub fn action_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ActionExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ActionQueued` event
        pub fn action_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ActionQueuedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ISimpleGovernanceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISimpleGovernance<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActionFailed` with signature `ActionFailed(uint256)` and selector `0xa6a7dbbd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ActionFailed", abi = "ActionFailed(uint256)")]
    pub struct ActionFailed {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `CannotExecute` with signature `CannotExecute(uint256)` and selector `0xb452faaf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CannotExecute", abi = "CannotExecute(uint256)")]
    pub struct CannotExecute {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidTarget` with signature `InvalidTarget()` and selector `0x82d5d76a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidTarget", abi = "InvalidTarget()")]
    pub struct InvalidTarget;
    ///Custom Error type `NotEnoughVotes` with signature `NotEnoughVotes(address)` and selector `0xfb124aea`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotEnoughVotes", abi = "NotEnoughVotes(address)")]
    pub struct NotEnoughVotes {
        pub who: ::ethers::core::types::Address,
    }
    ///Custom Error type `TargetMustHaveCode` with signature `TargetMustHaveCode()` and selector `0x6dd4aa65`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TargetMustHaveCode", abi = "TargetMustHaveCode()")]
    pub struct TargetMustHaveCode;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISimpleGovernanceErrors {
        ActionFailed(ActionFailed),
        CannotExecute(CannotExecute),
        InvalidTarget(InvalidTarget),
        NotEnoughVotes(NotEnoughVotes),
        TargetMustHaveCode(TargetMustHaveCode),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ISimpleGovernanceErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <ActionFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActionFailed(decoded));
            }
            if let Ok(decoded)
                = <CannotExecute as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotExecute(decoded));
            }
            if let Ok(decoded)
                = <InvalidTarget as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTarget(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughVotes as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotEnoughVotes(decoded));
            }
            if let Ok(decoded)
                = <TargetMustHaveCode as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TargetMustHaveCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISimpleGovernanceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActionFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotExecute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTarget(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetMustHaveCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ISimpleGovernanceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ActionFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <CannotExecute as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTarget as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughVotes as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TargetMustHaveCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ISimpleGovernanceErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActionFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotExecute(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTarget(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetMustHaveCode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ISimpleGovernanceErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ActionFailed> for ISimpleGovernanceErrors {
        fn from(value: ActionFailed) -> Self {
            Self::ActionFailed(value)
        }
    }
    impl ::core::convert::From<CannotExecute> for ISimpleGovernanceErrors {
        fn from(value: CannotExecute) -> Self {
            Self::CannotExecute(value)
        }
    }
    impl ::core::convert::From<InvalidTarget> for ISimpleGovernanceErrors {
        fn from(value: InvalidTarget) -> Self {
            Self::InvalidTarget(value)
        }
    }
    impl ::core::convert::From<NotEnoughVotes> for ISimpleGovernanceErrors {
        fn from(value: NotEnoughVotes) -> Self {
            Self::NotEnoughVotes(value)
        }
    }
    impl ::core::convert::From<TargetMustHaveCode> for ISimpleGovernanceErrors {
        fn from(value: TargetMustHaveCode) -> Self {
            Self::TargetMustHaveCode(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ActionExecuted", abi = "ActionExecuted(uint256,address)")]
    pub struct ActionExecutedFilter {
        pub action_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ActionQueued", abi = "ActionQueued(uint256,address)")]
    pub struct ActionQueuedFilter {
        pub action_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISimpleGovernanceEvents {
        ActionExecutedFilter(ActionExecutedFilter),
        ActionQueuedFilter(ActionQueuedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ISimpleGovernanceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ActionExecutedFilter::decode_log(log) {
                return Ok(ISimpleGovernanceEvents::ActionExecutedFilter(decoded));
            }
            if let Ok(decoded) = ActionQueuedFilter::decode_log(log) {
                return Ok(ISimpleGovernanceEvents::ActionQueuedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ISimpleGovernanceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActionExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ActionQueuedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ActionExecutedFilter> for ISimpleGovernanceEvents {
        fn from(value: ActionExecutedFilter) -> Self {
            Self::ActionExecutedFilter(value)
        }
    }
    impl ::core::convert::From<ActionQueuedFilter> for ISimpleGovernanceEvents {
        fn from(value: ActionQueuedFilter) -> Self {
            Self::ActionQueuedFilter(value)
        }
    }
    ///Container type for all input parameters for the `executeAction` function with signature `executeAction(uint256)` and selector `0xc0c1cf55`
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
    #[ethcall(name = "executeAction", abi = "executeAction(uint256)")]
    pub struct ExecuteActionCall {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAction` function with signature `getAction(uint256)` and selector `0xb6e76873`
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
    #[ethcall(name = "getAction", abi = "getAction(uint256)")]
    pub struct GetActionCall {
        pub action_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getActionCounter` function with signature `getActionCounter()` and selector `0x9aca08d4`
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
    #[ethcall(name = "getActionCounter", abi = "getActionCounter()")]
    pub struct GetActionCounterCall;
    ///Container type for all input parameters for the `getActionDelay` function with signature `getActionDelay()` and selector `0x12057a14`
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
    #[ethcall(name = "getActionDelay", abi = "getActionDelay()")]
    pub struct GetActionDelayCall;
    ///Container type for all input parameters for the `getGovernanceToken` function with signature `getGovernanceToken()` and selector `0x3f8a037d`
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
    #[ethcall(name = "getGovernanceToken", abi = "getGovernanceToken()")]
    pub struct GetGovernanceTokenCall;
    ///Container type for all input parameters for the `queueAction` function with signature `queueAction(address,uint128,bytes)` and selector `0x52ecb90a`
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
    #[ethcall(name = "queueAction", abi = "queueAction(address,uint128,bytes)")]
    pub struct QueueActionCall {
        pub target: ::ethers::core::types::Address,
        pub value: u128,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISimpleGovernanceCalls {
        ExecuteAction(ExecuteActionCall),
        GetAction(GetActionCall),
        GetActionCounter(GetActionCounterCall),
        GetActionDelay(GetActionDelayCall),
        GetGovernanceToken(GetGovernanceTokenCall),
        QueueAction(QueueActionCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISimpleGovernanceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ExecuteActionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteAction(decoded));
            }
            if let Ok(decoded)
                = <GetActionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAction(decoded));
            }
            if let Ok(decoded)
                = <GetActionCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetActionCounter(decoded));
            }
            if let Ok(decoded)
                = <GetActionDelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetActionDelay(decoded));
            }
            if let Ok(decoded)
                = <GetGovernanceTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetGovernanceToken(decoded));
            }
            if let Ok(decoded)
                = <QueueActionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueueAction(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISimpleGovernanceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExecuteAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActionCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActionDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGovernanceToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QueueAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ISimpleGovernanceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecuteAction(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAction(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActionCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActionDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGovernanceToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QueueAction(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteActionCall> for ISimpleGovernanceCalls {
        fn from(value: ExecuteActionCall) -> Self {
            Self::ExecuteAction(value)
        }
    }
    impl ::core::convert::From<GetActionCall> for ISimpleGovernanceCalls {
        fn from(value: GetActionCall) -> Self {
            Self::GetAction(value)
        }
    }
    impl ::core::convert::From<GetActionCounterCall> for ISimpleGovernanceCalls {
        fn from(value: GetActionCounterCall) -> Self {
            Self::GetActionCounter(value)
        }
    }
    impl ::core::convert::From<GetActionDelayCall> for ISimpleGovernanceCalls {
        fn from(value: GetActionDelayCall) -> Self {
            Self::GetActionDelay(value)
        }
    }
    impl ::core::convert::From<GetGovernanceTokenCall> for ISimpleGovernanceCalls {
        fn from(value: GetGovernanceTokenCall) -> Self {
            Self::GetGovernanceToken(value)
        }
    }
    impl ::core::convert::From<QueueActionCall> for ISimpleGovernanceCalls {
        fn from(value: QueueActionCall) -> Self {
            Self::QueueAction(value)
        }
    }
    ///Container type for all return fields from the `executeAction` function with signature `executeAction(uint256)` and selector `0xc0c1cf55`
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
    pub struct ExecuteActionReturn {
        pub returndata: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getAction` function with signature `getAction(uint256)` and selector `0xb6e76873`
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
    pub struct GetActionReturn {
        pub action: GovernanceAction,
    }
    ///Container type for all return fields from the `getActionCounter` function with signature `getActionCounter()` and selector `0x9aca08d4`
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
    pub struct GetActionCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getActionDelay` function with signature `getActionDelay()` and selector `0x12057a14`
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
    pub struct GetActionDelayReturn {
        pub delay: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getGovernanceToken` function with signature `getGovernanceToken()` and selector `0x3f8a037d`
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
    pub struct GetGovernanceTokenReturn {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `queueAction` function with signature `queueAction(address,uint128,bytes)` and selector `0x52ecb90a`
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
    pub struct QueueActionReturn {
        pub action_id: ::ethers::core::types::U256,
    }
}
