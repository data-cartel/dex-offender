pub use motorbike::*;
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
pub mod motorbike {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(
                ::ethers::core::abi::ethabi::Constructor {
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_logic"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },],
                },
            ),
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOTORBIKE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x02\xB48\x03\x80a\x02\xB4\x839\x81\x81\x01`@R` \x81\x10\x15a\x003W`\0\x80\xFD[PQa\0I\x81a\x01\xD1` \x90\x81\x1Ba\0c\x17\x90\x1CV[a\0\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`-\x81R` \x01\x80a\x02\x87`-\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x80a\0\xAE\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x01\xD7V[\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c J\x7F\x07`\xE2\x1B\x17\x81R\x91Q\x81Q`\0\x94\x86\x16\x93\x82\x91\x80\x83\x83[` \x83\x10a\x01!W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x01\x02V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\x81W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x86V[``\x91P[PP\x90P\x80a\x01\xCAW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10\xD8[\x1B\x08\x19\x98Z[\x19Y`\xAA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPa\x01\xDAV[;\x15\x15\x90V[\x90V[`\x9F\x80a\x01\xE8`\09`\0\xF3\xFE`\x80`@R`;`-\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`=V[T`\x01`\x01`\xA0\x1B\x03\x16`@V[\0[\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15`^W=`\0\xF3[=`\0\xFD[;\x15\x15\x90V\xFE\xA2dipfsX\"\x12 \x81y*9\xFD\x9Bc]\x0F\xE6\x7F<s)\xE2J\xBF\x1E\x80\xEC{\x87\xAB\xFD\0rV\xF2\x0E\xF4\x14\x02dsolcC\0\x06\x0C\x003ERC1967: new implementation is not a contract";
    /// The bytecode of the contract.
    pub static MOTORBIKE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`;`-\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`=V[T`\x01`\x01`\xA0\x1B\x03\x16`@V[\0[\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15`^W=`\0\xF3[=`\0\xFD[;\x15\x15\x90V\xFE\xA2dipfsX\"\x12 \x81y*9\xFD\x9Bc]\x0F\xE6\x7F<s)\xE2J\xBF\x1E\x80\xEC{\x87\xAB\xFD\0rV\xF2\x0E\xF4\x14\x02dsolcC\0\x06\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static MOTORBIKE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Motorbike<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Motorbike<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Motorbike<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Motorbike<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Motorbike<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Motorbike))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Motorbike<M> {
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
                MOTORBIKE_ABI.clone(),
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
                MOTORBIKE_ABI.clone(),
                MOTORBIKE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Motorbike<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
