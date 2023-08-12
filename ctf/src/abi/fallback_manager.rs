pub use fallback_manager::*;
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
pub mod fallback_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("setFallbackHandler"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned(
                        "setFallbackHandler"
                    ),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("handler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },],
                    outputs: ::std::vec![],
                    constant: ::core::option::Option::None,
                    state_mutability:
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("ChangedFallbackHandler"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned(
                        "ChangedFallbackHandler",
                    ),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("handler"),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FALLBACKMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01\xAB\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xF0\x8A\x03#\x14a\0\x84W[\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5\x80T\x80a\0UW\0[6`\0\x8073``\x1B6R`\0\x80`\x146\x01`\0\x80\x85Z\xF1\x90P=`\0\x80>\x80a\0~W=`\0\xFD[P=`\0\xF3[a\0\x97a\0\x926`\x04a\x01EV[a\0\x99V[\0[a\0\xA1a\x01\x08V[a\0\xC9\x81\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[30\x14a\x01CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[V[`\0` \x82\x84\x03\x12\x15a\x01WW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01nW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x11y\xEE\xB7\xAAH\x06\xC0\x7F?\xB8\xB8\xE1M\xB4h0\xD8\xE2\xEA\x0B\x15\xA4\r\xD7\xC3\xFE~\x13\xBD\x16\xC8dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static FALLBACKMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xF0\x8A\x03#\x14a\0\x84W[\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5\x80T\x80a\0UW\0[6`\0\x8073``\x1B6R`\0\x80`\x146\x01`\0\x80\x85Z\xF1\x90P=`\0\x80>\x80a\0~W=`\0\xFD[P=`\0\xF3[a\0\x97a\0\x926`\x04a\x01EV[a\0\x99V[\0[a\0\xA1a\x01\x08V[a\0\xC9\x81\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[30\x14a\x01CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[V[`\0` \x82\x84\x03\x12\x15a\x01WW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01nW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x11y\xEE\xB7\xAAH\x06\xC0\x7F?\xB8\xB8\xE1M\xB4h0\xD8\xE2\xEA\x0B\x15\xA4\r\xD7\xC3\xFE~\x13\xBD\x16\xC8dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static FALLBACKMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FallbackManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FallbackManager<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for FallbackManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for FallbackManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for FallbackManager<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FallbackManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FallbackManager<M> {
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
                FALLBACKMANAGER_ABI.clone(),
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
                FALLBACKMANAGER_ABI.clone(),
                FALLBACKMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `setFallbackHandler`
        /// (0xf08a0323) function
        pub fn set_fallback_handler(
            &self,
            handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 138, 3, 35], handler)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ChangedFallbackHandler`
        /// event
        pub fn changed_fallback_handler_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedFallbackHandlerFilter,
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
            ChangedFallbackHandlerFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for FallbackManager<M>
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
    #[ethevent(
        name = "ChangedFallbackHandler",
        abi = "ChangedFallbackHandler(address)"
    )]
    pub struct ChangedFallbackHandlerFilter {
        pub handler: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `setFallbackHandler` function with signature
    /// `setFallbackHandler(address)` and selector
    /// `0xf08a0323`
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
    #[ethcall(name = "setFallbackHandler", abi = "setFallbackHandler(address)")]
    pub struct SetFallbackHandlerCall {
        pub handler: ::ethers::core::types::Address,
    }
}
