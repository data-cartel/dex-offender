pub use storage_accessible::*;
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
pub mod storage_accessible {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getStorageAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStorageAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("offset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldataPayload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STORAGEACCESSIBLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x02\xCC\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cV$\xB2[\x14a\0;W\x80c\xB4\xFA\xBA\t\x14a\0dW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\x1AV[a\0yV[`@Qa\0[\x91\x90a\x01<V[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x01\xA1V[a\0\xF7V[\0[```\0a\0\x88\x83` a\x02qV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xA0Wa\0\xA0a\x01\x8BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\0\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\0\xEDW\x84\x81\x01T` \x80\x83\x02\x84\x01\x01R`\x01\x01a\0\xD0V[P\x90P[\x92\x91PPV[`\0\x80\x82Q` \x84\x01\x85Z\xF4\x80`\0RP=` R=`\0`@>`@=\x01`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01-W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01jW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01NV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xB4W`\0\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xCBW`\0\x80\xFD[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xE8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x01\xFCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\x0EWa\x02\x0Ea\x01\x8BV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x026Wa\x026a\x01\x8BV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x02OW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\0\xF1WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD0\xC7ZJu\xF3\r\xB7\xBC\x0Ek\x8F\xA45 \xE5r\xDE&\xE4\x14,3\x039\r\x11%\\\x96iQdsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static STORAGEACCESSIBLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cV$\xB2[\x14a\0;W\x80c\xB4\xFA\xBA\t\x14a\0dW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\x1AV[a\0yV[`@Qa\0[\x91\x90a\x01<V[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x01\xA1V[a\0\xF7V[\0[```\0a\0\x88\x83` a\x02qV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xA0Wa\0\xA0a\x01\x8BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\0\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\0\xEDW\x84\x81\x01T` \x80\x83\x02\x84\x01\x01R`\x01\x01a\0\xD0V[P\x90P[\x92\x91PPV[`\0\x80\x82Q` \x84\x01\x85Z\xF4\x80`\0RP=` R=`\0`@>`@=\x01`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01-W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01jW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01NV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xB4W`\0\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xCBW`\0\x80\xFD[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xE8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x01\xFCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\x0EWa\x02\x0Ea\x01\x8BV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x026Wa\x026a\x01\x8BV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x02OW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\0\xF1WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD0\xC7ZJu\xF3\r\xB7\xBC\x0Ek\x8F\xA45 \xE5r\xDE&\xE4\x14,3\x039\r\x11%\\\x96iQdsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static STORAGEACCESSIBLE_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StorageAccessible<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StorageAccessible<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for StorageAccessible<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for StorageAccessible<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for StorageAccessible<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StorageAccessible))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StorageAccessible<M> {
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
                STORAGEACCESSIBLE_ABI.clone(),
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
                STORAGEACCESSIBLE_ABI.clone(),
                STORAGEACCESSIBLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getStorageAt` (0x5624b25b)
        /// function
        pub fn get_storage_at(
            &self,
            offset: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([86, 36, 178, 91], (offset, length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateAndRevert`
        /// (0xb4faba09) function
        pub fn simulate_and_revert(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [180, 250, 186, 9],
                    (target_contract, calldata_payload),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for StorageAccessible<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `getStorageAt` function with signature
    /// `getStorageAt(uint256,uint256)` and selector
    /// `0x5624b25b`
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
    #[ethcall(name = "getStorageAt", abi = "getStorageAt(uint256,uint256)")]
    pub struct GetStorageAtCall {
        pub offset: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the
    /// `simulateAndRevert` function with signature
    /// `simulateAndRevert(address,bytes)` and selector
    /// `0xb4faba09`
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
    #[ethcall(
        name = "simulateAndRevert",
        abi = "simulateAndRevert(address,bytes)"
    )]
    pub struct SimulateAndRevertCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum StorageAccessibleCalls {
        GetStorageAt(GetStorageAtCall),
        SimulateAndRevert(SimulateAndRevertCall),
    }
    impl ::ethers::core::abi::AbiDecode for StorageAccessibleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetStorageAtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetStorageAt(decoded));
            }
            if let Ok(decoded) = <SimulateAndRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateAndRevert(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StorageAccessibleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetStorageAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAndRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StorageAccessibleCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::GetStorageAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateAndRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetStorageAtCall> for StorageAccessibleCalls {
        fn from(value: GetStorageAtCall) -> Self { Self::GetStorageAt(value) }
    }
    impl ::core::convert::From<SimulateAndRevertCall> for StorageAccessibleCalls {
        fn from(value: SimulateAndRevertCall) -> Self {
            Self::SimulateAndRevert(value)
        }
    }
    ///Container type for all return fields from the
    /// `getStorageAt` function with signature
    /// `getStorageAt(uint256,uint256)` and selector
    /// `0x5624b25b`
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
    pub struct GetStorageAtReturn(pub ::ethers::core::types::Bytes);
}
