pub use coin::*;
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
pub mod coin {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("wallet_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dest_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount_"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("current"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("required"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x02\xF18\x03\x80a\x02\xF1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0QV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 b\x0FB@\x90Ua\0\x81V[`\0` \x82\x84\x03\x12\x15a\0cW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0zW`\0\x80\xFD[\x93\x92PPPV[a\x02a\x80a\0\x90`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c'\xE25\xE3\x14a\0;W\x80c\xA9\x05\x9C\xBB\x14a\0mW[`\0\x80\xFD[a\0[a\0I6`\x04a\x01\x9DV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x80a\0{6`\x04a\x01\xBFV[a\0\x82V[\0[3`\0\x90\x81R` \x81\x90R`@\x90 T\x80\x82\x11a\x01ZW3`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x84\x92\x90a\0\xB8\x90\x84\x90a\x01\xFFV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x84\x92\x90a\0\xE5\x90\x84\x90a\x02\x18V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x01UW`@Qc&4\x1E-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x98\xD0x\xB4\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01PW=`\0\x80>=`\0\xFD[PPPP[PPPV[`@Qc\xCFG\x91\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01`@Q\x80\x91\x03\x90\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x98W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xAFW`\0\x80\xFD[a\x01\xB8\x82a\x01\x81V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xD2W`\0\x80\xFD[a\x01\xDB\x83a\x01\x81V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\x12Wa\x02\x12a\x01\xE9V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\x12Wa\x02\x12a\x01\xE9V\xFE\xA2dipfsX\"\x12 \xE9\x9BH\xFA\x8B\x14 \xC6\xFC\xD3Y(\xDC\xFEws\x86\x11\x9C\x1B\r\x84\xE6\xB6C\x89\xEF\x08_\x92(bdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static COIN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c'\xE25\xE3\x14a\0;W\x80c\xA9\x05\x9C\xBB\x14a\0mW[`\0\x80\xFD[a\0[a\0I6`\x04a\x01\x9DV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x80a\0{6`\x04a\x01\xBFV[a\0\x82V[\0[3`\0\x90\x81R` \x81\x90R`@\x90 T\x80\x82\x11a\x01ZW3`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x84\x92\x90a\0\xB8\x90\x84\x90a\x01\xFFV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x84\x92\x90a\0\xE5\x90\x84\x90a\x02\x18V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x01UW`@Qc&4\x1E-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x98\xD0x\xB4\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01PW=`\0\x80>=`\0\xFD[PPPP[PPPV[`@Qc\xCFG\x91\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01`@Q\x80\x91\x03\x90\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x98W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xAFW`\0\x80\xFD[a\x01\xB8\x82a\x01\x81V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xD2W`\0\x80\xFD[a\x01\xDB\x83a\x01\x81V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\x12Wa\x02\x12a\x01\xE9V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\x12Wa\x02\x12a\x01\xE9V\xFE\xA2dipfsX\"\x12 \xE9\x9BH\xFA\x8B\x14 \xC6\xFC\xD3Y(\xDC\xFEws\x86\x11\x9C\x1B\r\x84\xE6\xB6C\x89\xEF\x08_\x92(bdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static COIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Coin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Coin<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for Coin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for Coin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for Coin<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Coin))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Coin<M> {
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
                COIN_ABI.clone(),
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
                COIN_ABI.clone(),
                COIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `balances` (0x27e235e3)
        /// function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([39, 226, 53, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb)
        /// function
        pub fn transfer(
            &self,
            dest: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 5, 156, 187], (dest, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for Coin<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientBalance` with
    /// signature `InsufficientBalance(uint256,uint256)` and
    /// selector `0xcf479181`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "InsufficientBalance",
        abi = "InsufficientBalance(uint256,uint256)"
    )]
    pub struct InsufficientBalance {
        pub current: ::ethers::core::types::U256,
        pub required: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `balances` function with signature
    /// `balances(address)` and selector `0x27e235e3`
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
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the
    /// `transfer` function with signature
    /// `transfer(address,uint256)` and selector
    /// `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub dest: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum CoinCalls {
        Balances(BalancesCall),
        Transfer(TransferCall),
    }
    impl ::ethers::core::abi::AbiDecode for CoinCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Transfer(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CoinCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Balances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CoinCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Balances(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Transfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BalancesCall> for CoinCalls {
        fn from(value: BalancesCall) -> Self { Self::Balances(value) }
    }
    impl ::core::convert::From<TransferCall> for CoinCalls {
        fn from(value: TransferCall) -> Self { Self::Transfer(value) }
    }
    ///Container type for all return fields from the
    /// `balances` function with signature
    /// `balances(address)` and selector `0x27e235e3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalancesReturn(pub ::ethers::core::types::U256);
}
