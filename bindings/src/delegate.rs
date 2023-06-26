pub use delegate::*;
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
pub mod delegate {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pwn\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static DELEGATE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
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
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        64,
        81,
        97,
        1,
        69,
        56,
        3,
        128,
        97,
        1,
        69,
        131,
        57,
        129,
        1,
        96,
        64,
        129,
        144,
        82,
        97,
        0,
        47,
        145,
        97,
        0,
        84,
        86,
        91,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        146,
        144,
        146,
        22,
        145,
        144,
        145,
        23,
        144,
        85,
        97,
        0,
        132,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        0,
        102,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        0,
        125,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        179,
        128,
        97,
        0,
        146,
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
        4,
        54,
        16,
        96,
        50,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        96,
        55,
        87,
        128,
        99,
        221,
        54,
        91,
        139,
        20,
        96,
        101,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        84,
        96,
        73,
        144,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        144,
        145,
        22,
        129,
        82,
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
        243,
        91,
        96,
        123,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        51,
        23,
        144,
        85,
        86,
        91,
        0,
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
        77,
        119,
        76,
        25,
        141,
        177,
        79,
        47,
        90,
        236,
        208,
        197,
        94,
        111,
        22,
        123,
        20,
        215,
        201,
        95,
        149,
        67,
        133,
        173,
        7,
        37,
        63,
        137,
        77,
        68,
        223,
        199,
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
    pub static DELEGATE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
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
        4,
        54,
        16,
        96,
        50,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        96,
        55,
        87,
        128,
        99,
        221,
        54,
        91,
        139,
        20,
        96,
        101,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        84,
        96,
        73,
        144,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        144,
        145,
        22,
        129,
        82,
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
        243,
        91,
        96,
        123,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        51,
        23,
        144,
        85,
        86,
        91,
        0,
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
        77,
        119,
        76,
        25,
        141,
        177,
        79,
        47,
        90,
        236,
        208,
        197,
        94,
        111,
        22,
        123,
        20,
        215,
        201,
        95,
        149,
        67,
        133,
        173,
        7,
        37,
        63,
        137,
        77,
        68,
        223,
        199,
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
    pub static DELEGATE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Delegate<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Delegate<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Delegate<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Delegate<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Delegate<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Delegate)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Delegate<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DELEGATE_ABI.clone(),
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
                DELEGATE_ABI.clone(),
                DELEGATE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pwn` (0xdd365b8b) function
        pub fn pwn(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 54, 91, 139], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Delegate<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pwn` function with signature `pwn()` and selector `0xdd365b8b`
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
    #[ethcall(name = "pwn", abi = "pwn()")]
    pub struct PwnCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DelegateCalls {
        Owner(OwnerCall),
        Pwn(PwnCall),
    }
    impl ::ethers::core::abi::AbiDecode for DelegateCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PwnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pwn(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DelegateCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pwn(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DelegateCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pwn(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OwnerCall> for DelegateCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PwnCall> for DelegateCalls {
        fn from(value: PwnCall) -> Self {
            Self::Pwn(value)
        }
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
