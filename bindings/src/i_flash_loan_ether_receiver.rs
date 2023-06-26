pub use i_flash_loan_ether_receiver::*;
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
pub mod i_flash_loan_ether_receiver {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IFLASHLOANETHERRECEIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IFlashLoanEtherReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IFlashLoanEtherReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IFlashLoanEtherReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IFlashLoanEtherReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IFlashLoanEtherReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IFlashLoanEtherReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IFlashLoanEtherReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IFLASHLOANETHERRECEIVER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `execute` (0x61461954) function
        pub fn execute(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 70, 25, 84], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IFlashLoanEtherReceiver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `execute` function with signature `execute()` and selector `0x61461954`
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
    #[ethcall(name = "execute", abi = "execute()")]
    pub struct ExecuteCall;
}
