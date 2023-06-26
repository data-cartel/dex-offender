pub use i_proxy_creation_callback::*;
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
pub mod i_proxy_creation_callback {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract GnosisSafeProxy\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_singleton\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"initializer\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"saltNonce\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proxyCreated\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IPROXYCREATIONCALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IProxyCreationCallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IProxyCreationCallback<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IProxyCreationCallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IProxyCreationCallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IProxyCreationCallback<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IProxyCreationCallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IProxyCreationCallback<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPROXYCREATIONCALLBACK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `proxyCreated` (0x1e52b518) function
        pub fn proxy_created(
            &self,
            proxy: ::ethers::core::types::Address,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 82, 181, 24],
                    (proxy, singleton, initializer, salt_nonce),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IProxyCreationCallback<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `proxyCreated` function with signature `proxyCreated(address,address,bytes,uint256)` and selector `0x1e52b518`
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
        name = "proxyCreated",
        abi = "proxyCreated(address,address,bytes,uint256)"
    )]
    pub struct ProxyCreatedCall {
        pub proxy: ::ethers::core::types::Address,
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
}
