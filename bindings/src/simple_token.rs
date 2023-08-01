pub use simple_token::*;
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
pub mod simple_token {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_creator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_initialSupply"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("balances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balances"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("destroy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("destroy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
    pub static SIMPLETOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x06@8\x03\x80a\x06@\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x8EV[`\0a\0;\x84\x82a\x01\xFAV[P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x01` R`@\x90 UPa\x02\xB9V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x89W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\xA3W`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\0\xBAW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\0\xCEW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\0\xE0Wa\0\xE0a\0\\V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01\x08Wa\x01\x08a\0\\V[\x81`@R\x82\x81R` \x93P\x89\x84\x84\x87\x01\x01\x11\x15a\x01$W`\0\x80\xFD[`\0\x91P[\x82\x82\x10\x15a\x01FW\x84\x82\x01\x84\x01Q\x81\x83\x01\x85\x01R\x90\x83\x01\x90a\x01)V[`\0\x84\x84\x83\x01\x01R\x80\x97PPPPa\x01_\x81\x87\x01a\0rV[\x93PPP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x85W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xA5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x01\xF5W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x01\xD2WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xF1W\x82\x81U`\x01\x01a\x01\xDEV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x13Wa\x02\x13a\0\\V[a\x02'\x81a\x02!\x84Ta\x01qV[\x84a\x01\xABV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x02\\W`\0\x84\x15a\x02DWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x01\xF1V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x02\x8BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x02lV[P\x85\x82\x10\x15a\x02\xA9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x03x\x80a\x02\xC8`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0BW`\x005`\xE0\x1C\x80b\xF5]\x9D\x14a\0kW\x80c\x06\xFD\xDE\x03\x14a\0\x8DW\x80c'\xE25\xE3\x14a\0\xB8W\x80c\xA9\x05\x9C\xBB\x14a\0\xF3W`\0\x80\xFD[6a\0fWa\0R4`\na\x02\"V[3`\0\x90\x81R`\x01` R`@\x90 \x81\x90U\0[`\0\x80\xFD[4\x80\x15a\0wW`\0\x80\xFD[Pa\0\x8Ba\0\x866`\x04a\x02WV[a\x01\x13V[\0[4\x80\x15a\0\x99W`\0\x80\xFD[Pa\0\xA2a\x01\x1FV[`@Qa\0\xAF\x91\x90a\x02{V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xC4W`\0\x80\xFD[Pa\0\xE5a\0\xD36`\x04a\x02WV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xAFV[4\x80\x15a\0\xFFW`\0\x80\xFD[Pa\0\x8Ba\x01\x0E6`\x04a\x02\xC9V[a\x01\xADV[\x80`\x01`\x01`\xA0\x1B\x03\x16\xFF[`\0\x80Ta\x01,\x90a\x02\xF5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01X\x90a\x02\xF5V[\x80\x15a\x01\xA5W\x80`\x1F\x10a\x01zWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xA5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\x88W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x90\x81R`\x01` R`@\x90 T\x81\x11\x15a\x01\xC9W`\0\x80\xFD[3`\0\x90\x81R`\x01` R`@\x90 Ta\x01\xE4\x90\x82\x90a\x03/V[3`\0\x90\x81R`\x01` R`@\x80\x82 \x92\x90\x92U`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R\x90\x91 UV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x029Wa\x029a\x02\x0CV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02TW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x02iW`\0\x80\xFD[\x815a\x02t\x81a\x02?V[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x02\xA8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x02\x8CV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xDCW`\0\x80\xFD[\x825a\x02\xE7\x81a\x02?V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x03)WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x029Wa\x029a\x02\x0CV\xFE\xA2dipfsX\"\x12 \x98\xA0o\x0C\x82\n\x0C\xB7'pi\x89;\xBB\xB2+\xA9\"RV7\xB7\x19i\xA73\xB8\x02{\xC7\x97$dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static SIMPLETOKEN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0BW`\x005`\xE0\x1C\x80b\xF5]\x9D\x14a\0kW\x80c\x06\xFD\xDE\x03\x14a\0\x8DW\x80c'\xE25\xE3\x14a\0\xB8W\x80c\xA9\x05\x9C\xBB\x14a\0\xF3W`\0\x80\xFD[6a\0fWa\0R4`\na\x02\"V[3`\0\x90\x81R`\x01` R`@\x90 \x81\x90U\0[`\0\x80\xFD[4\x80\x15a\0wW`\0\x80\xFD[Pa\0\x8Ba\0\x866`\x04a\x02WV[a\x01\x13V[\0[4\x80\x15a\0\x99W`\0\x80\xFD[Pa\0\xA2a\x01\x1FV[`@Qa\0\xAF\x91\x90a\x02{V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xC4W`\0\x80\xFD[Pa\0\xE5a\0\xD36`\x04a\x02WV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xAFV[4\x80\x15a\0\xFFW`\0\x80\xFD[Pa\0\x8Ba\x01\x0E6`\x04a\x02\xC9V[a\x01\xADV[\x80`\x01`\x01`\xA0\x1B\x03\x16\xFF[`\0\x80Ta\x01,\x90a\x02\xF5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01X\x90a\x02\xF5V[\x80\x15a\x01\xA5W\x80`\x1F\x10a\x01zWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xA5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\x88W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x90\x81R`\x01` R`@\x90 T\x81\x11\x15a\x01\xC9W`\0\x80\xFD[3`\0\x90\x81R`\x01` R`@\x90 Ta\x01\xE4\x90\x82\x90a\x03/V[3`\0\x90\x81R`\x01` R`@\x80\x82 \x92\x90\x92U`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R\x90\x91 UV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x029Wa\x029a\x02\x0CV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02TW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x02iW`\0\x80\xFD[\x815a\x02t\x81a\x02?V[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x02\xA8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x02\x8CV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xDCW`\0\x80\xFD[\x825a\x02\xE7\x81a\x02?V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x03)WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x029Wa\x029a\x02\x0CV\xFE\xA2dipfsX\"\x12 \x98\xA0o\x0C\x82\n\x0C\xB7'pi\x89;\xBB\xB2+\xA9\"RV7\xB7\x19i\xA73\xB8\x02{\xC7\x97$dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static SIMPLETOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SimpleToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleToken<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SimpleToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SimpleToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SimpleToken<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SimpleToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleToken<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SIMPLETOKEN_ABI.clone(),
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
                SIMPLETOKEN_ABI.clone(),
                SIMPLETOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `destroy` (0x00f55d9d) function
        pub fn destroy(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 245, 93, 157], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SimpleToken<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `destroy` function with signature `destroy(address)` and selector `0x00f55d9d`
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
    #[ethcall(name = "destroy", abi = "destroy(address)")]
    pub struct DestroyCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimpleTokenCalls {
        Balances(BalancesCall),
        Destroy(DestroyCall),
        Name(NameCall),
        Transfer(TransferCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded)
                = <DestroyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Destroy(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Balances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Destroy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SimpleTokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::Destroy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalancesCall> for SimpleTokenCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<DestroyCall> for SimpleTokenCalls {
        fn from(value: DestroyCall) -> Self {
            Self::Destroy(value)
        }
    }
    impl ::core::convert::From<NameCall> for SimpleTokenCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<TransferCall> for SimpleTokenCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    ///Container type for all return fields from the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    pub struct BalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
}
