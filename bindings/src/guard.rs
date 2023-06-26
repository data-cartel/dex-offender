pub use guard::*;
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
pub mod guard {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"checkAfterExecution\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"enum Enum.Operation\",\"name\":\"operation\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"safeTxGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasPrice\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"refundReceiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signatures\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"msgSender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"checkTransaction\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static GUARD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct Guard<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Guard<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Guard<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Guard<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Guard<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Guard)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Guard<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GUARD_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `checkAfterExecution` (0x93271368) function
        pub fn check_after_execution(
            &self,
            tx_hash: [u8; 32],
            success: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 39, 19, 104], (tx_hash, success))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkTransaction` (0x75f0bb52) function
        pub fn check_transaction(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            signatures: ::ethers::core::types::Bytes,
            msg_sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [117, 240, 187, 82],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        signatures,
                        msg_sender,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Guard<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `checkAfterExecution` function with signature `checkAfterExecution(bytes32,bool)` and selector `0x93271368`
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
    #[ethcall(name = "checkAfterExecution", abi = "checkAfterExecution(bytes32,bool)")]
    pub struct CheckAfterExecutionCall {
        pub tx_hash: [u8; 32],
        pub success: bool,
    }
    ///Container type for all input parameters for the `checkTransaction` function with signature `checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)` and selector `0x75f0bb52`
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
    #[ethcall(
        name = "checkTransaction",
        abi = "checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)"
    )]
    pub struct CheckTransactionCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub signatures: ::ethers::core::types::Bytes,
        pub msg_sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GuardCalls {
        CheckAfterExecution(CheckAfterExecutionCall),
        CheckTransaction(CheckTransactionCall),
    }
    impl ::ethers::core::abi::AbiDecode for GuardCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CheckAfterExecutionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckAfterExecution(decoded));
            }
            if let Ok(decoded)
                = <CheckTransactionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckTransaction(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GuardCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckAfterExecution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GuardCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckAfterExecution(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckTransaction(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckAfterExecutionCall> for GuardCalls {
        fn from(value: CheckAfterExecutionCall) -> Self {
            Self::CheckAfterExecution(value)
        }
    }
    impl ::core::convert::From<CheckTransactionCall> for GuardCalls {
        fn from(value: CheckTransactionCall) -> Self {
            Self::CheckTransaction(value)
        }
    }
}
