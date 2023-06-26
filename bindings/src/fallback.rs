pub use fallback::*;
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
pub mod fallback {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"contribute\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"contributions\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getContribution\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static FALLBACK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
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
        1,
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
        144,
        129,
        23,
        144,
        145,
        85,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        144,
        32,
        104,
        54,
        53,
        201,
        173,
        197,
        222,
        160,
        0,
        0,
        144,
        85,
        97,
        2,
        234,
        128,
        97,
        0,
        79,
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
        96,
        4,
        54,
        16,
        97,
        0,
        78,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        60,
        207,
        214,
        11,
        20,
        97,
        0,
        150,
        87,
        128,
        99,
        66,
        233,
        76,
        144,
        20,
        97,
        0,
        173,
        87,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        97,
        0,
        237,
        87,
        128,
        99,
        215,
        187,
        153,
        186,
        20,
        97,
        1,
        37,
        87,
        128,
        99,
        241,
        15,
        223,
        92,
        20,
        97,
        1,
        45,
        87,
        96,
        0,
        128,
        253,
        91,
        54,
        97,
        0,
        145,
        87,
        96,
        0,
        52,
        17,
        128,
        21,
        97,
        0,
        113,
        87,
        80,
        51,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        144,
        32,
        84,
        21,
        21,
        91,
        97,
        0,
        122,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
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
        144,
        129,
        23,
        144,
        145,
        85,
        0,
        91,
        96,
        0,
        128,
        253,
        91,
        52,
        128,
        21,
        97,
        0,
        162,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        171,
        97,
        1,
        79,
        86,
        91,
        0,
        91,
        52,
        128,
        21,
        97,
        0,
        185,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        218,
        97,
        0,
        200,
        54,
        96,
        4,
        97,
        2,
        93,
        86,
        91,
        96,
        0,
        96,
        32,
        129,
        144,
        82,
        144,
        129,
        82,
        96,
        64,
        144,
        32,
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
        52,
        128,
        21,
        97,
        0,
        249,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        1,
        84,
        97,
        1,
        13,
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
        97,
        0,
        228,
        86,
        91,
        97,
        0,
        171,
        97,
        1,
        233,
        86,
        91,
        52,
        128,
        21,
        97,
        1,
        57,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        51,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        144,
        32,
        84,
        97,
        0,
        218,
        86,
        91,
        96,
        1,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        51,
        20,
        97,
        1,
        173,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        23,
        96,
        36,
        130,
        1,
        82,
        127,
        99,
        97,
        108,
        108,
        101,
        114,
        32,
        105,
        115,
        32,
        110,
        111,
        116,
        32,
        116,
        104,
        101,
        32,
        111,
        119,
        110,
        101,
        114,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        68,
        130,
        1,
        82,
        96,
        100,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        253,
        91,
        96,
        1,
        84,
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
        144,
        71,
        128,
        21,
        97,
        8,
        252,
        2,
        145,
        96,
        0,
        129,
        129,
        129,
        133,
        136,
        136,
        241,
        147,
        80,
        80,
        80,
        80,
        21,
        128,
        21,
        97,
        1,
        230,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        86,
        91,
        102,
        3,
        141,
        126,
        164,
        198,
        128,
        0,
        52,
        16,
        97,
        1,
        252,
        87,
        96,
        0,
        128,
        253,
        91,
        51,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        129,
        32,
        128,
        84,
        52,
        146,
        144,
        97,
        2,
        27,
        144,
        132,
        144,
        97,
        2,
        141,
        86,
        91,
        144,
        145,
        85,
        80,
        80,
        96,
        1,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        128,
        130,
        32,
        84,
        51,
        131,
        82,
        145,
        32,
        84,
        17,
        21,
        97,
        2,
        91,
        87,
        96,
        1,
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
        91,
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
        2,
        111,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
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
        2,
        134,
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
        128,
        130,
        1,
        128,
        130,
        17,
        21,
        97,
        2,
        174,
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
        146,
        145,
        80,
        80,
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
        227,
        243,
        98,
        19,
        122,
        5,
        74,
        126,
        189,
        172,
        143,
        241,
        112,
        188,
        24,
        5,
        127,
        135,
        201,
        233,
        224,
        82,
        195,
        254,
        81,
        139,
        254,
        184,
        21,
        212,
        171,
        86,
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
    pub static FALLBACK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        97,
        0,
        78,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        60,
        207,
        214,
        11,
        20,
        97,
        0,
        150,
        87,
        128,
        99,
        66,
        233,
        76,
        144,
        20,
        97,
        0,
        173,
        87,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        97,
        0,
        237,
        87,
        128,
        99,
        215,
        187,
        153,
        186,
        20,
        97,
        1,
        37,
        87,
        128,
        99,
        241,
        15,
        223,
        92,
        20,
        97,
        1,
        45,
        87,
        96,
        0,
        128,
        253,
        91,
        54,
        97,
        0,
        145,
        87,
        96,
        0,
        52,
        17,
        128,
        21,
        97,
        0,
        113,
        87,
        80,
        51,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        144,
        32,
        84,
        21,
        21,
        91,
        97,
        0,
        122,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
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
        144,
        129,
        23,
        144,
        145,
        85,
        0,
        91,
        96,
        0,
        128,
        253,
        91,
        52,
        128,
        21,
        97,
        0,
        162,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        171,
        97,
        1,
        79,
        86,
        91,
        0,
        91,
        52,
        128,
        21,
        97,
        0,
        185,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        218,
        97,
        0,
        200,
        54,
        96,
        4,
        97,
        2,
        93,
        86,
        91,
        96,
        0,
        96,
        32,
        129,
        144,
        82,
        144,
        129,
        82,
        96,
        64,
        144,
        32,
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
        52,
        128,
        21,
        97,
        0,
        249,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        1,
        84,
        97,
        1,
        13,
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
        97,
        0,
        228,
        86,
        91,
        97,
        0,
        171,
        97,
        1,
        233,
        86,
        91,
        52,
        128,
        21,
        97,
        1,
        57,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        51,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        144,
        32,
        84,
        97,
        0,
        218,
        86,
        91,
        96,
        1,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        51,
        20,
        97,
        1,
        173,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        23,
        96,
        36,
        130,
        1,
        82,
        127,
        99,
        97,
        108,
        108,
        101,
        114,
        32,
        105,
        115,
        32,
        110,
        111,
        116,
        32,
        116,
        104,
        101,
        32,
        111,
        119,
        110,
        101,
        114,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        68,
        130,
        1,
        82,
        96,
        100,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        253,
        91,
        96,
        1,
        84,
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
        144,
        71,
        128,
        21,
        97,
        8,
        252,
        2,
        145,
        96,
        0,
        129,
        129,
        129,
        133,
        136,
        136,
        241,
        147,
        80,
        80,
        80,
        80,
        21,
        128,
        21,
        97,
        1,
        230,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        86,
        91,
        102,
        3,
        141,
        126,
        164,
        198,
        128,
        0,
        52,
        16,
        97,
        1,
        252,
        87,
        96,
        0,
        128,
        253,
        91,
        51,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        129,
        32,
        128,
        84,
        52,
        146,
        144,
        97,
        2,
        27,
        144,
        132,
        144,
        97,
        2,
        141,
        86,
        91,
        144,
        145,
        85,
        80,
        80,
        96,
        1,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        129,
        144,
        82,
        96,
        64,
        128,
        130,
        32,
        84,
        51,
        131,
        82,
        145,
        32,
        84,
        17,
        21,
        97,
        2,
        91,
        87,
        96,
        1,
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
        91,
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
        2,
        111,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
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
        2,
        134,
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
        128,
        130,
        1,
        128,
        130,
        17,
        21,
        97,
        2,
        174,
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
        146,
        145,
        80,
        80,
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
        227,
        243,
        98,
        19,
        122,
        5,
        74,
        126,
        189,
        172,
        143,
        241,
        112,
        188,
        24,
        5,
        127,
        135,
        201,
        233,
        224,
        82,
        195,
        254,
        81,
        139,
        254,
        184,
        21,
        212,
        171,
        86,
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
    pub static FALLBACK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Fallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Fallback<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Fallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Fallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Fallback<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Fallback)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Fallback<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FALLBACK_ABI.clone(),
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
                FALLBACK_ABI.clone(),
                FALLBACK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `contribute` (0xd7bb99ba) function
        pub fn contribute(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 187, 153, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contributions` (0x42e94c90) function
        pub fn contributions(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 233, 76, 144], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getContribution` (0xf10fdf5c) function
        pub fn get_contribution(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 15, 223, 92], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `withdraw` (0x3ccfd60b) function
        pub fn withdraw(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Fallback<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `contribute` function with signature `contribute()` and selector `0xd7bb99ba`
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
    #[ethcall(name = "contribute", abi = "contribute()")]
    pub struct ContributeCall;
    ///Container type for all input parameters for the `contributions` function with signature `contributions(address)` and selector `0x42e94c90`
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
    #[ethcall(name = "contributions", abi = "contributions(address)")]
    pub struct ContributionsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getContribution` function with signature `getContribution()` and selector `0xf10fdf5c`
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
    #[ethcall(name = "getContribution", abi = "getContribution()")]
    pub struct GetContributionCall;
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
    pub enum FallbackCalls {
        Contribute(ContributeCall),
        Contributions(ContributionsCall),
        GetContribution(GetContributionCall),
        Owner(OwnerCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for FallbackCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ContributeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Contribute(decoded));
            }
            if let Ok(decoded)
                = <ContributionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Contributions(decoded));
            }
            if let Ok(decoded)
                = <GetContributionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetContribution(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FallbackCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Contribute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Contributions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetContribution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FallbackCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Contribute(element) => ::core::fmt::Display::fmt(element, f),
                Self::Contributions(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetContribution(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ContributeCall> for FallbackCalls {
        fn from(value: ContributeCall) -> Self {
            Self::Contribute(value)
        }
    }
    impl ::core::convert::From<ContributionsCall> for FallbackCalls {
        fn from(value: ContributionsCall) -> Self {
            Self::Contributions(value)
        }
    }
    impl ::core::convert::From<GetContributionCall> for FallbackCalls {
        fn from(value: GetContributionCall) -> Self {
            Self::GetContribution(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for FallbackCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for FallbackCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `contributions` function with signature `contributions(address)` and selector `0x42e94c90`
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
    pub struct ContributionsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getContribution` function with signature `getContribution()` and selector `0xf10fdf5c`
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
    pub struct GetContributionReturn(pub ::ethers::core::types::U256);
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
