pub use ether_payment_fallback::*;
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
pub mod ether_payment_fallback {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("SafeReceived"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("SafeReceived"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("value"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                256usize,
                            ),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ETHERPAYMENTFALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`z\x80a\0\x1E`\09`\0\xF3\xFE`\x80`@R6`?W`@Q4\x81R3\x90\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x01\xF9(6x~\xA7\r\xC8\xE9\xE7\xED\xC0\x92\x19!\xEC-\xC8\xAD\xB5/u\xAD\x85\xAE\xF7\xCFR\x8C\xA4RdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ETHERPAYMENTFALLBACK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R6`?W`@Q4\x81R3\x90\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x01\xF9(6x~\xA7\r\xC8\xE9\xE7\xED\xC0\x92\x19!\xEC-\xC8\xAD\xB5/u\xAD\x85\xAE\xF7\xCFR\x8C\xA4RdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ETHERPAYMENTFALLBACK_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EtherPaymentFallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EtherPaymentFallback<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for EtherPaymentFallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for EtherPaymentFallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for EtherPaymentFallback<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EtherPaymentFallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EtherPaymentFallback<M> {
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
                ETHERPAYMENTFALLBACK_ABI.clone(),
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
        /// Returns an `Event` builder for all the events of
        /// this contract.
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
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for EtherPaymentFallback<M>
    {
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
        Hash,
    )]
    #[ethevent(name = "SafeReceived", abi = "SafeReceived(address,uint256)")]
    pub struct SafeReceivedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
}
