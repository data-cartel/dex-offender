pub use recovery_solution::*;
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
pub mod recovery_solution {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("solution"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("solution"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("creator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::string::String::new(),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability:
                        ::ethers::core::abi::ethabi::StateMutability::Pure,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RECOVERYSOLUTION_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x01\x12\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`(W`\x005`\xE0\x1C\x80c\xD29\x06\xFC\x14`-W[`\0\x80\xFD[`\x92`86`\x04`\xAEV[`@\x80Q`k`\xF9\x1B` \x80\x83\x01\x91\x90\x91R`%`\xFA\x1B`!\x83\x01R``\x93\x90\x93\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\"\x82\x01R`\x01`\xF8\x1B`6\x82\x01R\x81Q`\x17\x81\x83\x03\x01\x81R`7\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0` \x82\x84\x03\x12\x15`\xBFW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\xD5W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 _#\xF6n\xAA\x83\x99\x17\xC8\x88]:\x8A\xCB`\xC2\x17\n\x045\0\x0C\xFE\x07\xF2\xAA\xEB\x84h\xD4\xFBkdsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static RECOVERYSOLUTION_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`(W`\x005`\xE0\x1C\x80c\xD29\x06\xFC\x14`-W[`\0\x80\xFD[`\x92`86`\x04`\xAEV[`@\x80Q`k`\xF9\x1B` \x80\x83\x01\x91\x90\x91R`%`\xFA\x1B`!\x83\x01R``\x93\x90\x93\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\"\x82\x01R`\x01`\xF8\x1B`6\x82\x01R\x81Q`\x17\x81\x83\x03\x01\x81R`7\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0` \x82\x84\x03\x12\x15`\xBFW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\xD5W`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 _#\xF6n\xAA\x83\x99\x17\xC8\x88]:\x8A\xCB`\xC2\x17\n\x045\0\x0C\xFE\x07\xF2\xAA\xEB\x84h\xD4\xFBkdsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static RECOVERYSOLUTION_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct RecoverySolution<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RecoverySolution<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for RecoverySolution<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for RecoverySolution<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for RecoverySolution<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RecoverySolution))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RecoverySolution<M> {
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
                RECOVERYSOLUTION_ABI.clone(),
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
                RECOVERYSOLUTION_ABI.clone(),
                RECOVERYSOLUTION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `solution` (0xd23906fc)
        /// function
        pub fn solution(
            &self,
            creator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 57, 6, 252], creator)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for RecoverySolution<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `solution` function with signature
    /// `solution(address)` and selector `0xd23906fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "solution", abi = "solution(address)")]
    pub struct SolutionCall {
        pub creator: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the
    /// `solution` function with signature
    /// `solution(address)` and selector `0xd23906fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SolutionReturn(pub ::ethers::core::types::Address);
}
