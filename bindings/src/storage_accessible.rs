pub use storage_accessible::*;
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
pub mod storage_accessible {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"offset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"length\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStorageAt\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"calldataPayload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"simulateAndRevert\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static STORAGEACCESSIBLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
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
        97,
        2,
        246,
        128,
        97,
        0,
        32,
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
        86,
        36,
        178,
        91,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        180,
        250,
        186,
        9,
        20,
        97,
        0,
        100,
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
        36,
        86,
        91,
        97,
        0,
        121,
        86,
        91,
        96,
        64,
        81,
        97,
        0,
        91,
        145,
        144,
        97,
        1,
        70,
        86,
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
        119,
        97,
        0,
        114,
        54,
        96,
        4,
        97,
        1,
        170,
        86,
        91,
        97,
        1,
        1,
        86,
        91,
        0,
        91,
        96,
        96,
        96,
        0,
        97,
        0,
        136,
        131,
        96,
        32,
        97,
        2,
        144,
        86,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        21,
        97,
        0,
        160,
        87,
        97,
        0,
        160,
        97,
        1,
        148,
        86,
        91,
        96,
        64,
        81,
        144,
        128,
        130,
        82,
        128,
        96,
        31,
        1,
        96,
        31,
        25,
        22,
        96,
        32,
        1,
        130,
        1,
        96,
        64,
        82,
        128,
        21,
        97,
        0,
        202,
        87,
        96,
        32,
        130,
        1,
        129,
        128,
        54,
        131,
        55,
        1,
        144,
        80,
        91,
        80,
        144,
        80,
        96,
        0,
        91,
        131,
        129,
        16,
        21,
        97,
        0,
        247,
        87,
        132,
        129,
        1,
        84,
        96,
        32,
        128,
        131,
        2,
        132,
        1,
        1,
        82,
        128,
        97,
        0,
        239,
        129,
        97,
        2,
        167,
        86,
        91,
        145,
        80,
        80,
        97,
        0,
        208,
        86,
        91,
        80,
        144,
        80,
        91,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        130,
        81,
        96,
        32,
        132,
        1,
        133,
        90,
        244,
        128,
        96,
        0,
        82,
        80,
        61,
        96,
        32,
        82,
        61,
        96,
        0,
        96,
        64,
        62,
        96,
        64,
        61,
        1,
        96,
        0,
        253,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        1,
        55,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        80,
        128,
        53,
        146,
        96,
        32,
        144,
        145,
        1,
        53,
        145,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        128,
        131,
        82,
        131,
        81,
        128,
        130,
        133,
        1,
        82,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        1,
        115,
        87,
        133,
        129,
        1,
        131,
        1,
        81,
        133,
        130,
        1,
        96,
        64,
        1,
        82,
        130,
        1,
        97,
        1,
        87,
        86,
        91,
        80,
        96,
        0,
        96,
        64,
        130,
        134,
        1,
        1,
        82,
        96,
        64,
        96,
        31,
        25,
        96,
        31,
        131,
        1,
        22,
        133,
        1,
        1,
        146,
        80,
        80,
        80,
        146,
        145,
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
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        1,
        189,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
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
        1,
        212,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        80,
        96,
        32,
        131,
        1,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        241,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        133,
        1,
        145,
        80,
        133,
        96,
        31,
        131,
        1,
        18,
        97,
        2,
        5,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        2,
        23,
        87,
        97,
        2,
        23,
        97,
        1,
        148,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
        1,
        96,
        31,
        25,
        144,
        129,
        22,
        96,
        63,
        1,
        22,
        129,
        1,
        144,
        131,
        130,
        17,
        129,
        131,
        16,
        23,
        21,
        97,
        2,
        63,
        87,
        97,
        2,
        63,
        97,
        1,
        148,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        129,
        82,
        136,
        96,
        32,
        132,
        135,
        1,
        1,
        17,
        21,
        97,
        2,
        88,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        96,
        32,
        134,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        96,
        32,
        132,
        131,
        1,
        1,
        82,
        128,
        149,
        80,
        80,
        80,
        80,
        80,
        80,
        146,
        80,
        146,
        144,
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
        128,
        130,
        2,
        129,
        21,
        130,
        130,
        4,
        132,
        20,
        23,
        97,
        0,
        251,
        87,
        97,
        0,
        251,
        97,
        2,
        122,
        86,
        91,
        96,
        0,
        96,
        1,
        130,
        1,
        97,
        2,
        185,
        87,
        97,
        2,
        185,
        97,
        2,
        122,
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
        76,
        112,
        146,
        108,
        241,
        157,
        105,
        146,
        103,
        26,
        25,
        212,
        90,
        117,
        14,
        152,
        187,
        112,
        97,
        183,
        108,
        176,
        38,
        15,
        205,
        6,
        131,
        75,
        220,
        202,
        88,
        127,
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
    pub static STORAGEACCESSIBLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        86,
        36,
        178,
        91,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        180,
        250,
        186,
        9,
        20,
        97,
        0,
        100,
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
        36,
        86,
        91,
        97,
        0,
        121,
        86,
        91,
        96,
        64,
        81,
        97,
        0,
        91,
        145,
        144,
        97,
        1,
        70,
        86,
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
        119,
        97,
        0,
        114,
        54,
        96,
        4,
        97,
        1,
        170,
        86,
        91,
        97,
        1,
        1,
        86,
        91,
        0,
        91,
        96,
        96,
        96,
        0,
        97,
        0,
        136,
        131,
        96,
        32,
        97,
        2,
        144,
        86,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        17,
        21,
        97,
        0,
        160,
        87,
        97,
        0,
        160,
        97,
        1,
        148,
        86,
        91,
        96,
        64,
        81,
        144,
        128,
        130,
        82,
        128,
        96,
        31,
        1,
        96,
        31,
        25,
        22,
        96,
        32,
        1,
        130,
        1,
        96,
        64,
        82,
        128,
        21,
        97,
        0,
        202,
        87,
        96,
        32,
        130,
        1,
        129,
        128,
        54,
        131,
        55,
        1,
        144,
        80,
        91,
        80,
        144,
        80,
        96,
        0,
        91,
        131,
        129,
        16,
        21,
        97,
        0,
        247,
        87,
        132,
        129,
        1,
        84,
        96,
        32,
        128,
        131,
        2,
        132,
        1,
        1,
        82,
        128,
        97,
        0,
        239,
        129,
        97,
        2,
        167,
        86,
        91,
        145,
        80,
        80,
        97,
        0,
        208,
        86,
        91,
        80,
        144,
        80,
        91,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        130,
        81,
        96,
        32,
        132,
        1,
        133,
        90,
        244,
        128,
        96,
        0,
        82,
        80,
        61,
        96,
        32,
        82,
        61,
        96,
        0,
        96,
        64,
        62,
        96,
        64,
        61,
        1,
        96,
        0,
        253,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        1,
        55,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        80,
        128,
        53,
        146,
        96,
        32,
        144,
        145,
        1,
        53,
        145,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        128,
        131,
        82,
        131,
        81,
        128,
        130,
        133,
        1,
        82,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        1,
        115,
        87,
        133,
        129,
        1,
        131,
        1,
        81,
        133,
        130,
        1,
        96,
        64,
        1,
        82,
        130,
        1,
        97,
        1,
        87,
        86,
        91,
        80,
        96,
        0,
        96,
        64,
        130,
        134,
        1,
        1,
        82,
        96,
        64,
        96,
        31,
        25,
        96,
        31,
        131,
        1,
        22,
        133,
        1,
        1,
        146,
        80,
        80,
        80,
        146,
        145,
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
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        1,
        189,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
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
        1,
        212,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        80,
        96,
        32,
        131,
        1,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        241,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        133,
        1,
        145,
        80,
        133,
        96,
        31,
        131,
        1,
        18,
        97,
        2,
        5,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        2,
        23,
        87,
        97,
        2,
        23,
        97,
        1,
        148,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
        1,
        96,
        31,
        25,
        144,
        129,
        22,
        96,
        63,
        1,
        22,
        129,
        1,
        144,
        131,
        130,
        17,
        129,
        131,
        16,
        23,
        21,
        97,
        2,
        63,
        87,
        97,
        2,
        63,
        97,
        1,
        148,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        129,
        82,
        136,
        96,
        32,
        132,
        135,
        1,
        1,
        17,
        21,
        97,
        2,
        88,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        96,
        32,
        134,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        96,
        32,
        132,
        131,
        1,
        1,
        82,
        128,
        149,
        80,
        80,
        80,
        80,
        80,
        80,
        146,
        80,
        146,
        144,
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
        128,
        130,
        2,
        129,
        21,
        130,
        130,
        4,
        132,
        20,
        23,
        97,
        0,
        251,
        87,
        97,
        0,
        251,
        97,
        2,
        122,
        86,
        91,
        96,
        0,
        96,
        1,
        130,
        1,
        97,
        2,
        185,
        87,
        97,
        2,
        185,
        97,
        2,
        122,
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
        76,
        112,
        146,
        108,
        241,
        157,
        105,
        146,
        103,
        26,
        25,
        212,
        90,
        117,
        14,
        152,
        187,
        112,
        97,
        183,
        108,
        176,
        38,
        15,
        205,
        6,
        131,
        75,
        220,
        202,
        88,
        127,
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
    pub static STORAGEACCESSIBLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StorageAccessible<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StorageAccessible<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StorageAccessible<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StorageAccessible<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StorageAccessible<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(StorageAccessible)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StorageAccessible<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STORAGEACCESSIBLE_ABI.clone(),
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
                STORAGEACCESSIBLE_ABI.clone(),
                STORAGEACCESSIBLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getStorageAt` (0x5624b25b) function
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
        ///Calls the contract's `simulateAndRevert` (0xb4faba09) function
        pub fn simulate_and_revert(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 250, 186, 9], (target_contract, calldata_payload))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StorageAccessible<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
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
    #[ethcall(name = "getStorageAt", abi = "getStorageAt(uint256,uint256)")]
    pub struct GetStorageAtCall {
        pub offset: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateAndRevert` function with signature `simulateAndRevert(address,bytes)` and selector `0xb4faba09`
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
    #[ethcall(name = "simulateAndRevert", abi = "simulateAndRevert(address,bytes)")]
    pub struct SimulateAndRevertCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StorageAccessibleCalls {
        GetStorageAt(GetStorageAtCall),
        SimulateAndRevert(SimulateAndRevertCall),
    }
    impl ::ethers::core::abi::AbiDecode for StorageAccessibleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetStorageAtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetStorageAt(decoded));
            }
            if let Ok(decoded)
                = <SimulateAndRevertCall as ::ethers::core::abi::AbiDecode>::decode(
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
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetStorageAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateAndRevert(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetStorageAtCall> for StorageAccessibleCalls {
        fn from(value: GetStorageAtCall) -> Self {
            Self::GetStorageAt(value)
        }
    }
    impl ::core::convert::From<SimulateAndRevertCall> for StorageAccessibleCalls {
        fn from(value: SimulateAndRevertCall) -> Self {
            Self::SimulateAndRevert(value)
        }
    }
    ///Container type for all return fields from the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
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
    pub struct GetStorageAtReturn(pub ::ethers::core::types::Bytes);
}
