pub use ether_payment_fallback::*;
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
pub mod ether_payment_fallback {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeReceived\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ETHERPAYMENTFALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        122,
        128,
        97,
        0,
        30,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        54,
        96,
        63,
        87,
        96,
        64,
        81,
        52,
        129,
        82,
        51,
        144,
        127,
        61,
        12,
        233,
        191,
        195,
        237,
        125,
        104,
        98,
        219,
        178,
        139,
        45,
        234,
        148,
        86,
        31,
        231,
        20,
        161,
        180,
        208,
        25,
        170,
        138,
        243,
        151,
        48,
        209,
        173,
        124,
        61,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        162,
        0,
        91,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        176,
        189,
        230,
        213,
        114,
        7,
        103,
        218,
        85,
        52,
        58,
        222,
        251,
        60,
        164,
        217,
        237,
        167,
        113,
        7,
        28,
        154,
        206,
        56,
        241,
        192,
        71,
        144,
        48,
        101,
        235,
        194,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        20,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static ETHERPAYMENTFALLBACK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        54,
        96,
        63,
        87,
        96,
        64,
        81,
        52,
        129,
        82,
        51,
        144,
        127,
        61,
        12,
        233,
        191,
        195,
        237,
        125,
        104,
        98,
        219,
        178,
        139,
        45,
        234,
        148,
        86,
        31,
        231,
        20,
        161,
        180,
        208,
        25,
        170,
        138,
        243,
        151,
        48,
        209,
        173,
        124,
        61,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        162,
        0,
        91,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        176,
        189,
        230,
        213,
        114,
        7,
        103,
        218,
        85,
        52,
        58,
        222,
        251,
        60,
        164,
        217,
        237,
        167,
        113,
        7,
        28,
        154,
        206,
        56,
        241,
        192,
        71,
        144,
        48,
        101,
        235,
        194,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        20,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static ETHERPAYMENTFALLBACK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct EtherPaymentFallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EtherPaymentFallback<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EtherPaymentFallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EtherPaymentFallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EtherPaymentFallback<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(EtherPaymentFallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EtherPaymentFallback<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ETHERPAYMENTFALLBACK_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
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
                ETHERPAYMENTFALLBACK_ABI.clone(),
                ETHERPAYMENTFALLBACK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `SafeReceived` event
        pub fn safe_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeReceivedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeReceivedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EtherPaymentFallback<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "SafeReceived", abi = "SafeReceived(address,uint256)")]
    pub struct SafeReceivedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
}
