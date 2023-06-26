pub use i_signature_validator::*;
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
pub mod i_signature_validator {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_signature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidSignature\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ISIGNATUREVALIDATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ISignatureValidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISignatureValidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISignatureValidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISignatureValidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISignatureValidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ISignatureValidator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISignatureValidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISIGNATUREVALIDATOR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `isValidSignature` (0x20c13b0b) function
        pub fn is_valid_signature(
            &self,
            data: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([32, 193, 59, 11], (data, signature))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISignatureValidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
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
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes,bytes)")]
    pub struct IsValidSignatureCall {
        pub data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
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
    pub struct IsValidSignatureReturn(pub [u8; 4]);
}
