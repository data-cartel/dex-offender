pub use i_proxy::*;
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
pub mod i_proxy {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"masterCopy\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IPROXY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IProxy)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPROXY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `masterCopy` (0xa619486e) function
        pub fn master_copy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([166, 25, 72, 110], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IProxy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `masterCopy` function with signature `masterCopy()` and selector `0xa619486e`
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
    #[ethcall(name = "masterCopy", abi = "masterCopy()")]
    pub struct MasterCopyCall;
    ///Container type for all return fields from the `masterCopy` function with signature `masterCopy()` and selector `0xa619486e`
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
    pub struct MasterCopyReturn(pub ::ethers::core::types::Address);
}
