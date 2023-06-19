pub use coin_flip::*;
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
pub mod coin_flip {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"consecutiveWins\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_guess\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"flip\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static COINFLIP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        1,
        96,
        255,
        27,
        96,
        2,
        85,
        52,
        128,
        21,
        97,
        0,
        24,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        0,
        128,
        85,
        97,
        1,
        202,
        128,
        97,
        0,
        44,
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
        4,
        54,
        16,
        97,
        0,
        54,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        29,
        38,
        63,
        103,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        230,
        243,
        52,
        215,
        20,
        97,
        0,
        99,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        78,
        97,
        0,
        73,
        54,
        96,
        4,
        97,
        1,
        1,
        86,
        91,
        97,
        0,
        122,
        86,
        91,
        96,
        64,
        81,
        144,
        21,
        21,
        129,
        82,
        96,
        32,
        1,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        97,
        0,
        108,
        96,
        0,
        84,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        97,
        0,
        90,
        86,
        91,
        96,
        0,
        128,
        97,
        0,
        136,
        96,
        1,
        67,
        97,
        1,
        64,
        86,
        91,
        64,
        96,
        0,
        28,
        144,
        80,
        128,
        96,
        1,
        84,
        3,
        97,
        0,
        156,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        129,
        144,
        85,
        96,
        2,
        84,
        96,
        0,
        144,
        97,
        0,
        177,
        144,
        131,
        97,
        1,
        89,
        86,
        91,
        144,
        80,
        96,
        0,
        129,
        96,
        1,
        20,
        97,
        0,
        196,
        87,
        96,
        0,
        97,
        0,
        199,
        86,
        91,
        96,
        1,
        91,
        144,
        80,
        132,
        21,
        21,
        129,
        21,
        21,
        3,
        97,
        0,
        243,
        87,
        96,
        0,
        128,
        84,
        144,
        128,
        97,
        0,
        227,
        131,
        97,
        1,
        123,
        86,
        91,
        144,
        145,
        85,
        80,
        96,
        1,
        150,
        149,
        80,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        80,
        80,
        96,
        0,
        128,
        128,
        85,
        147,
        146,
        80,
        80,
        80,
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
        1,
        19,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        128,
        21,
        21,
        129,
        20,
        97,
        1,
        35,
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
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        129,
        129,
        3,
        129,
        129,
        17,
        21,
        97,
        1,
        83,
        87,
        97,
        1,
        83,
        97,
        1,
        42,
        86,
        91,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        130,
        97,
        1,
        118,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        18,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        80,
        4,
        144,
        86,
        91,
        96,
        0,
        96,
        1,
        130,
        1,
        97,
        1,
        141,
        87,
        97,
        1,
        141,
        97,
        1,
        42,
        86,
        91,
        80,
        96,
        1,
        1,
        144,
        86,
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
        101,
        64,
        220,
        76,
        234,
        146,
        81,
        114,
        251,
        101,
        252,
        151,
        216,
        252,
        185,
        252,
        101,
        0,
        36,
        60,
        198,
        219,
        192,
        224,
        123,
        150,
        71,
        185,
        91,
        73,
        49,
        156,
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
    pub static COINFLIP_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        4,
        54,
        16,
        97,
        0,
        54,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        29,
        38,
        63,
        103,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        230,
        243,
        52,
        215,
        20,
        97,
        0,
        99,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        78,
        97,
        0,
        73,
        54,
        96,
        4,
        97,
        1,
        1,
        86,
        91,
        97,
        0,
        122,
        86,
        91,
        96,
        64,
        81,
        144,
        21,
        21,
        129,
        82,
        96,
        32,
        1,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        97,
        0,
        108,
        96,
        0,
        84,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        97,
        0,
        90,
        86,
        91,
        96,
        0,
        128,
        97,
        0,
        136,
        96,
        1,
        67,
        97,
        1,
        64,
        86,
        91,
        64,
        96,
        0,
        28,
        144,
        80,
        128,
        96,
        1,
        84,
        3,
        97,
        0,
        156,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        129,
        144,
        85,
        96,
        2,
        84,
        96,
        0,
        144,
        97,
        0,
        177,
        144,
        131,
        97,
        1,
        89,
        86,
        91,
        144,
        80,
        96,
        0,
        129,
        96,
        1,
        20,
        97,
        0,
        196,
        87,
        96,
        0,
        97,
        0,
        199,
        86,
        91,
        96,
        1,
        91,
        144,
        80,
        132,
        21,
        21,
        129,
        21,
        21,
        3,
        97,
        0,
        243,
        87,
        96,
        0,
        128,
        84,
        144,
        128,
        97,
        0,
        227,
        131,
        97,
        1,
        123,
        86,
        91,
        144,
        145,
        85,
        80,
        96,
        1,
        150,
        149,
        80,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        80,
        80,
        96,
        0,
        128,
        128,
        85,
        147,
        146,
        80,
        80,
        80,
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
        1,
        19,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        128,
        21,
        21,
        129,
        20,
        97,
        1,
        35,
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
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        129,
        129,
        3,
        129,
        129,
        17,
        21,
        97,
        1,
        83,
        87,
        97,
        1,
        83,
        97,
        1,
        42,
        86,
        91,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        130,
        97,
        1,
        118,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        18,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        80,
        4,
        144,
        86,
        91,
        96,
        0,
        96,
        1,
        130,
        1,
        97,
        1,
        141,
        87,
        97,
        1,
        141,
        97,
        1,
        42,
        86,
        91,
        80,
        96,
        1,
        1,
        144,
        86,
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
        101,
        64,
        220,
        76,
        234,
        146,
        81,
        114,
        251,
        101,
        252,
        151,
        216,
        252,
        185,
        252,
        101,
        0,
        36,
        60,
        198,
        219,
        192,
        224,
        123,
        150,
        71,
        185,
        91,
        73,
        49,
        156,
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
    pub static COINFLIP_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CoinFlip<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CoinFlip<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CoinFlip<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CoinFlip<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CoinFlip<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(CoinFlip)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CoinFlip<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COINFLIP_ABI.clone(),
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
                COINFLIP_ABI.clone(),
                COINFLIP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `consecutiveWins` (0xe6f334d7) function
        pub fn consecutive_wins(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([230, 243, 52, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flip` (0x1d263f67) function
        pub fn flip(
            &self,
            guess: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 38, 63, 103], guess)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CoinFlip<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `consecutiveWins` function with signature `consecutiveWins()` and selector `0xe6f334d7`
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
    #[ethcall(name = "consecutiveWins", abi = "consecutiveWins()")]
    pub struct ConsecutiveWinsCall;
    ///Container type for all input parameters for the `flip` function with signature `flip(bool)` and selector `0x1d263f67`
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
    #[ethcall(name = "flip", abi = "flip(bool)")]
    pub struct FlipCall {
        pub guess: bool,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CoinFlipCalls {
        ConsecutiveWins(ConsecutiveWinsCall),
        Flip(FlipCall),
    }
    impl ::ethers::core::abi::AbiDecode for CoinFlipCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ConsecutiveWinsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ConsecutiveWins(decoded));
            }
            if let Ok(decoded)
                = <FlipCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Flip(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CoinFlipCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConsecutiveWins(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Flip(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CoinFlipCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConsecutiveWins(element) => ::core::fmt::Display::fmt(element, f),
                Self::Flip(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConsecutiveWinsCall> for CoinFlipCalls {
        fn from(value: ConsecutiveWinsCall) -> Self {
            Self::ConsecutiveWins(value)
        }
    }
    impl ::core::convert::From<FlipCall> for CoinFlipCalls {
        fn from(value: FlipCall) -> Self {
            Self::Flip(value)
        }
    }
    ///Container type for all return fields from the `consecutiveWins` function with signature `consecutiveWins()` and selector `0xe6f334d7`
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
    pub struct ConsecutiveWinsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `flip` function with signature `flip(bool)` and selector `0x1d263f67`
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
    pub struct FlipReturn(pub bool);
}
