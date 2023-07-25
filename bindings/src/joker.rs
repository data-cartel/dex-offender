pub use joker::*;
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
pub mod joker {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_king"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("becomeKing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("becomeKing"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static JOKER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`@Qa\x02\xF18\x03\x80a\x02\xF1\x839\x81\x01`@\x81\x90Ra\0\"\x91a\0UV[`\0\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x85V[`\0` \x82\x84\x03\x12\x15a\0gW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0~W`\0\x80\xFD[\x93\x92PPPV[a\x02]\x80a\0\x94`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0-W`\x005`\xE0\x1C\x80c<\xCF\xD6\x0B\x14a\0{W\x80cg\x08\xCCc\x14a\0\x90W`\0\x80\xFD[6a\0vW`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFCa\0LG`\x01a\x01\xE7V[`@Q\x81\x15\x90\x92\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0tW=`\0\x80>=`\0\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\x87W`\0\x80\xFD[Pa\0ta\0\xA5V[4\x80\x15a\0\x9CW`\0\x80\xFD[Pa\0ta\0\xF9V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xBCW`\0\x80\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91G\x80\x15a\x08\xFC\x02\x92\x90\x91\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xF6W=`\0\x80>=`\0\xFD[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x10W`\0\x80\xFD[`\x01T`@\x80Qcq\xD6.\x93`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xE3\xAC]&\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01~\x91\x90a\x02\x0EV[`\x01T`@Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xD5V[``\x91P[PP\x90P\x80a\x01\xE3W`\0\x80\xFD[PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\x08WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x02 W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 Q\xB5\x8D\x0CB6\xD4\xE5\0mX\x06jr\xD7\x17\x93z\xB7z`\x14.\x92\xDE+\xB5\xBB%\xBE\x9DGdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static JOKER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0-W`\x005`\xE0\x1C\x80c<\xCF\xD6\x0B\x14a\0{W\x80cg\x08\xCCc\x14a\0\x90W`\0\x80\xFD[6a\0vW`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFCa\0LG`\x01a\x01\xE7V[`@Q\x81\x15\x90\x92\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0tW=`\0\x80>=`\0\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\x87W`\0\x80\xFD[Pa\0ta\0\xA5V[4\x80\x15a\0\x9CW`\0\x80\xFD[Pa\0ta\0\xF9V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xBCW`\0\x80\xFD[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91G\x80\x15a\x08\xFC\x02\x92\x90\x91\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xF6W=`\0\x80>=`\0\xFD[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x10W`\0\x80\xFD[`\x01T`@\x80Qcq\xD6.\x93`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xE3\xAC]&\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01~\x91\x90a\x02\x0EV[`\x01T`@Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xD5V[``\x91P[PP\x90P\x80a\x01\xE3W`\0\x80\xFD[PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\x08WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x02 W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 Q\xB5\x8D\x0CB6\xD4\xE5\0mX\x06jr\xD7\x17\x93z\xB7z`\x14.\x92\xDE+\xB5\xBB%\xBE\x9DGdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static JOKER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Joker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Joker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Joker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Joker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Joker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Joker)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Joker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    JOKER_ABI.clone(),
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
                JOKER_ABI.clone(),
                JOKER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `becomeKing` (0x6708cc63) function
        pub fn become_king(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 8, 204, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x3ccfd60b) function
        pub fn withdraw(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Joker<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `becomeKing` function with signature `becomeKing()` and selector `0x6708cc63`
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
    #[ethcall(name = "becomeKing", abi = "becomeKing()")]
    pub struct BecomeKingCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw()` and selector `0x3ccfd60b`
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
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum JokerCalls {
        BecomeKing(BecomeKingCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for JokerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BecomeKingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BecomeKing(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for JokerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BecomeKing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for JokerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BecomeKing(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BecomeKingCall> for JokerCalls {
        fn from(value: BecomeKingCall) -> Self {
            Self::BecomeKing(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for JokerCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
}
