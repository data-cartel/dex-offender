pub use upgradeable_proxy::*;
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
pub mod upgradeable_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(
                ::ethers::core::abi::ethabi::Constructor {
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_logic"),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },
                    ],
                },
            ),
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("Upgraded"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned(
                                "implementation"
                            ),
                            kind:
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UPGRADEABLEPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x03\xB48\x03\x80a\x03\xB4\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xC4V[a\0Z`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\x02\x92V[`\0\x80Q` a\x03\x94\x839\x81Q\x91R\x14a\0vWa\0va\x02\xB9V[a\0\x7F\x82a\0\xF7V[\x80Q\x15a\0\xF0W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\0\xA0\x91\x90a\x02\xCFV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\0\xDBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xE0V[``\x91P[PP\x90P\x80a\0\xEEW`\0\x80\xFD[P[PPa\x02\xEBV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x01xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FUpgradeableProxy: new implementa`D\x82\x01R\x7Ftion is not a contract\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Q` a\x03\x94\x839\x81Q\x91RUV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x01\xBBW\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xA3V[PP`\0\x91\x01RV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xD7W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xEEW`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x02\x0BW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\x1FW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x021Wa\x021a\x01\x8AV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02YWa\x02Ya\x01\x8AV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x02rW`\0\x80\xFD[a\x02\x83\x83` \x83\x01` \x88\x01a\x01\xA0V[\x80\x95PPPPPP\x92P\x92\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x02\xB3WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Qa\x02\xE1\x81\x84` \x87\x01a\x01\xA0V[\x91\x90\x91\x01\x92\x91PPV[`\x9B\x80a\x02\xF9`\09`\0\xF3\xFE`\x80`@R6`\x10W`\x0E`\x13V[\0[`\x0E[`@`<\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x90V[`BV[V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15``W=`\0\xF3[=`\0\xFD\xFE\xA2dipfsX\"\x12 \xD1U;\xF4\xC78\xBA\xA86:\xDD\xCE\x1D\xBA\x05{\x9B\xEA\x81h\xCB\x92\xF5\xF3\xC6Z\xAE_}\xB3Y\xF6dsolcC\0\x08\x14\x0036\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC";
    /// The bytecode of the contract.
    pub static UPGRADEABLEPROXY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R6`\x10W`\x0E`\x13V[\0[`\x0E[`@`<\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x90V[`BV[V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15``W=`\0\xF3[=`\0\xFD\xFE\xA2dipfsX\"\x12 \xD1U;\xF4\xC78\xBA\xA86:\xDD\xCE\x1D\xBA\x05{\x9B\xEA\x81h\xCB\x92\xF5\xF3\xC6Z\xAE_}\xB3Y\xF6dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static UPGRADEABLEPROXY_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct UpgradeableProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UpgradeableProxy<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for UpgradeableProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for UpgradeableProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for UpgradeableProxy<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UpgradeableProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UpgradeableProxy<M> {
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
                UPGRADEABLEPROXY_ABI.clone(),
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
                UPGRADEABLEPROXY_ABI.clone(),
                UPGRADEABLEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
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
            UpgradedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for UpgradeableProxy<M>
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
}
