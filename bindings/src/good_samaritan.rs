pub use good_samaritan::*;
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
pub mod good_samaritan {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("coin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("coin"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Coin"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestDonation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestDonation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enoughBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wallet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wallet"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Wallet"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    pub static GOODSAMARITAN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\0\x1D\x90a\x01\rV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\09W=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@Qa\0e\x90a\x01\x1AV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\0\x91W=`\0\x80>=`\0\xFD[P`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90\x81\x17\x90\x91U`\0T`@Qc\x82\xE4ku`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x90\x91\x16\x90c\x82\xE4ku\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x04W=`\0\x80>=`\0\xFD[PPPPa\x01'V[a\x03\xD6\x80a\x03D\x839\x01\x90V[a\x02\xF1\x80a\x07\x1A\x839\x01\x90V[a\x02\x0E\x80a\x016`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x11\xDF\x99\x95\x14a\0FW\x80c%\x1468\x14a\0vW\x80cR\x1E\xB2s\x14a\0\x8EW[`\0\x80\xFD[`\x01Ta\0Y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0~a\0\xA1V[`@Q\x90\x15\x15\x81R` \x01a\0mV[`\0Ta\0Y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T`@Qc|\x1BW\xE7`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF86\xAF\xCE\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xE7W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\0\xF8WP`\x01[a\x01\xD2W=\x80\x80\x15a\x01&W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01+V[``\x91P[P\x80Q` \x80\x83\x01\x91\x90\x91 `@\x80Q`\x04\x81R`$\x81\x01\x90\x91R\x91\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cV\x9DE\xCF`\xE1\x1B\x17\x81R\x91Q\x90\x91 \x03a\x01\xCEW`\0T`@Qc\x1C\x81p\xCB`\xE3\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\x0B\x86X\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xAEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xC2W=`\0\x80>=`\0\xFD[PPPP`\0\x91PP\x90V[P\x90V[P`\x01\x90V\xFE\xA2dipfsX\"\x12 \xA66\xC8\x14\x0E\xDB(\x9D\x8E\xD8R\x01\xC3\xBC1\xB8\x8B\xF4\r\x8B\x82\xA69Vm\x98\xF5\xBB\xCC\xAA\xB1RdsolcC\0\x08\x15\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x03\xA4\x80a\x002`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x11\xDF\x99\x95\x14a\0\\W\x80c\x82\xE4ku\x14a\0\x8BW\x80c\x8D\xA5\xCB[\x14a\0\xA0W\x80c\xE4\x0B\x86X\x14a\0\xB3W\x80c\xF86\xAF\xCE\x14a\0\xC6W[`\0\x80\xFD[`\x01Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x9Ea\0\x996`\x04a\x031V[a\0\xD9V[\0[`\0Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\x9Ea\0\xC16`\x04a\x031V[a\x01&V[a\0\x9Ea\0\xD46`\x04a\x031V[a\x02)V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x04W`@Qc_\xC4\x83\xC5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01QW`@Qc_\xC4\x83\xC5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@Qc'\xE25\xE3`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90\x83\x90\x83\x90c'\xE25\xE3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xC7\x91\x90a\x03UV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x0EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\"W=`\0\x80>=`\0\xFD[PPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02TW`@Qc_\xC4\x83\xC5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@Qc'\xE25\xE3`\xE0\x1B\x81R0`\x04\x82\x01R`\n\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c'\xE25\xE3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xC1\x91\x90a\x03UV[\x10\x15a\x02\xE0W`@QcV\x9DE\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\n`$\x83\x01R\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x01\xF4V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x19W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x03CW`\0\x80\xFD[\x815a\x03N\x81a\x03\x1CV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03gW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \xAF\x9A\xDB\xF1)>\xDE\xA4\xFA\xC2\x06\xD5\xC7\xCE\x14B=}\xFB\x13[\x0F\xC48\x84Q\xC4\x9Dy\x9D\xC8\xECdsolcC\0\x08\x15\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x02\xF18\x03\x80a\x02\xF1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0QV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 b\x0FB@\x90Ua\0\x81V[`\0` \x82\x84\x03\x12\x15a\0cW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0zW`\0\x80\xFD[\x93\x92PPPV[a\x02a\x80a\0\x90`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c'\xE25\xE3\x14a\0;W\x80c\xA9\x05\x9C\xBB\x14a\0mW[`\0\x80\xFD[a\0[a\0I6`\x04a\x01\x9DV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x80a\0{6`\x04a\x01\xBFV[a\0\x82V[\0[3`\0\x90\x81R` \x81\x90R`@\x90 T\x80\x82\x11a\x01ZW3`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x84\x92\x90a\0\xB8\x90\x84\x90a\x01\xFFV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x84\x92\x90a\0\xE5\x90\x84\x90a\x02\x18V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x01UW`@Qc&4\x1E-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x98\xD0x\xB4\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01PW=`\0\x80>=`\0\xFD[PPPP[PPPV[`@Qc\xCFG\x91\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01`@Q\x80\x91\x03\x90\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x98W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xAFW`\0\x80\xFD[a\x01\xB8\x82a\x01\x81V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xD2W`\0\x80\xFD[a\x01\xDB\x83a\x01\x81V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\x12Wa\x02\x12a\x01\xE9V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\x12Wa\x02\x12a\x01\xE9V\xFE\xA2dipfsX\"\x12 b\xDE\xD9;%\xEC\xC9\x07LE\xC9\xDB\x7F\xCBh\xC6\x047\x12\xEC[lL\x1DR\xD4hZw\xD0\xF9\x9CdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static GOODSAMARITAN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x11\xDF\x99\x95\x14a\0FW\x80c%\x1468\x14a\0vW\x80cR\x1E\xB2s\x14a\0\x8EW[`\0\x80\xFD[`\x01Ta\0Y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0~a\0\xA1V[`@Q\x90\x15\x15\x81R` \x01a\0mV[`\0Ta\0Y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x80T`@Qc|\x1BW\xE7`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF86\xAF\xCE\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xE7W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\0\xF8WP`\x01[a\x01\xD2W=\x80\x80\x15a\x01&W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01+V[``\x91P[P\x80Q` \x80\x83\x01\x91\x90\x91 `@\x80Q`\x04\x81R`$\x81\x01\x90\x91R\x91\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cV\x9DE\xCF`\xE1\x1B\x17\x81R\x91Q\x90\x91 \x03a\x01\xCEW`\0T`@Qc\x1C\x81p\xCB`\xE3\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\x0B\x86X\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xAEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xC2W=`\0\x80>=`\0\xFD[PPPP`\0\x91PP\x90V[P\x90V[P`\x01\x90V\xFE\xA2dipfsX\"\x12 \xA66\xC8\x14\x0E\xDB(\x9D\x8E\xD8R\x01\xC3\xBC1\xB8\x8B\xF4\r\x8B\x82\xA69Vm\x98\xF5\xBB\xCC\xAA\xB1RdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static GOODSAMARITAN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GoodSamaritan<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GoodSamaritan<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GoodSamaritan<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GoodSamaritan<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GoodSamaritan<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GoodSamaritan))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GoodSamaritan<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GOODSAMARITAN_ABI.clone(),
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
                GOODSAMARITAN_ABI.clone(),
                GOODSAMARITAN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `coin` (0x11df9995) function
        pub fn coin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([17, 223, 153, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestDonation` (0x25143638) function
        pub fn request_donation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 20, 54, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wallet` (0x521eb273) function
        pub fn wallet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([82, 30, 178, 115], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GoodSamaritan<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `coin` function with signature `coin()` and selector `0x11df9995`
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
    #[ethcall(name = "coin", abi = "coin()")]
    pub struct CoinCall;
    ///Container type for all input parameters for the `requestDonation` function with signature `requestDonation()` and selector `0x25143638`
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
    #[ethcall(name = "requestDonation", abi = "requestDonation()")]
    pub struct RequestDonationCall;
    ///Container type for all input parameters for the `wallet` function with signature `wallet()` and selector `0x521eb273`
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
    #[ethcall(name = "wallet", abi = "wallet()")]
    pub struct WalletCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GoodSamaritanCalls {
        Coin(CoinCall),
        RequestDonation(RequestDonationCall),
        Wallet(WalletCall),
    }
    impl ::ethers::core::abi::AbiDecode for GoodSamaritanCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CoinCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Coin(decoded));
            }
            if let Ok(decoded)
                = <RequestDonationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestDonation(decoded));
            }
            if let Ok(decoded)
                = <WalletCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Wallet(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GoodSamaritanCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Coin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestDonation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Wallet(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GoodSamaritanCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Coin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestDonation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wallet(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CoinCall> for GoodSamaritanCalls {
        fn from(value: CoinCall) -> Self {
            Self::Coin(value)
        }
    }
    impl ::core::convert::From<RequestDonationCall> for GoodSamaritanCalls {
        fn from(value: RequestDonationCall) -> Self {
            Self::RequestDonation(value)
        }
    }
    impl ::core::convert::From<WalletCall> for GoodSamaritanCalls {
        fn from(value: WalletCall) -> Self {
            Self::Wallet(value)
        }
    }
    ///Container type for all return fields from the `coin` function with signature `coin()` and selector `0x11df9995`
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
    pub struct CoinReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `requestDonation` function with signature `requestDonation()` and selector `0x25143638`
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
    pub struct RequestDonationReturn {
        pub enough_balance: bool,
    }
    ///Container type for all return fields from the `wallet` function with signature `wallet()` and selector `0x521eb273`
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
    pub struct WalletReturn(pub ::ethers::core::types::Address);
}
