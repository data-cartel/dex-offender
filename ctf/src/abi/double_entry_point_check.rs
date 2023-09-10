pub use double_entry_point_check::*;
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
pub mod double_entry_point_check {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_instance"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checker"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checker"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("result"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("result"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweep"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweep"),
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DOUBLEENTRYPOINTCHECK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x02\x80T`\xFF`\xA0\x1B\x19\x16\x90U`\x04\x80Ta\xFF\0\x19\x16\x90U4\x80\x15a\0(W`\0\x80\xFD[P`@Qa\x06\xB68\x03\x80a\x06\xB6\x839\x81\x01`@\x81\x90Ra\0G\x91a\0\xF5V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\x06\xA2&\xC5`\xE5\x1B\x81R\x90Qc\xD4D\xD8\xA0\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\0\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xC4\x91\x90a\0\xF5V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x91U`\x02\x80T\x90\x92\x16\x17\x90UPa\x01%V[`\0` \x82\x84\x03\x12\x15a\x01\x07W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x1EW`\0\x80\xFD[\x93\x92PPPV[a\x05\x82\x80a\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c5\xFA\xA4\x16\x14a\0FW\x80ce7!G\x14a\0PW\x80c\xCFS\x03\xCF\x14a\0qW[`\0\x80\xFD[a\0Na\0yV[\0[`\x04Ta\0]\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Na\x02\x86V[`\x02T`\0T`@\x80Qc&\xFE\x99Q`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\x1B\xE1\x95`\x93\x16\x91c&\xFE\x99Q\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\0\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xF1\x91\x90a\x03\x82V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x012W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01CWP`\x01[a\x02bW`\x02\x80T`\xFF`\xA0\x1B\x19\x16\x90U`\0\x80T`@\x80Qc\x06\xA2&\xC5`\xE5\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cp\xA0\x821\x91\x83\x91c\xD4D\xD8\xA0\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xCB\x91\x90a\x03\x82V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x023\x91\x90a\x03\xB2V[`@\x80Q\x92\x90\x91\x11` \x83\x01R\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x03\x90\x81a\x02_\x91\x90a\x04jV[PV[`\x02\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`@\x80Q`\0` \x82\x01R\x01a\x02BV[`\x02T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x03NW`\x03\x80Ta\x02\xA4\x90a\x03\xE1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xD0\x90a\x03\xE1V[\x80\x15a\x03\x1DW\x80`\x1F\x10a\x02\xF2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x1DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\x035\x91\x90a\x05*V[`\x04\x80T\x91\x15\x15a\x01\0\x02a\xFF\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U[`\x04Ta\x01\0\x90\x04`\xFF\x16\x80\x15a\x03oWP`\x02T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15[`\x04\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0` \x82\x84\x03\x12\x15a\x03\x94W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xABW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\xC4W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\xF5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\x15WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x04eW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x04BWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04aW\x82\x81U`\x01\x01a\x04NV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x84Wa\x04\x84a\x03\xCBV[a\x04\x98\x81a\x04\x92\x84Ta\x03\xE1V[\x84a\x04\x1BV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x04\xCDW`\0\x84\x15a\x04\xB5WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x04aV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x04\xFCW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x04\xDDV[P\x85\x82\x10\x15a\x05\x1AW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a\x05<W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xABW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xEFO\xB5\xC4I\xD6tq\xBF\x86\xCEkB\xB5\xDF*h\xB1\r\x8Cf;[\xC4]\x99\xE15\xE9W\xF7\xC1dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static DOUBLEENTRYPOINTCHECK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c5\xFA\xA4\x16\x14a\0FW\x80ce7!G\x14a\0PW\x80c\xCFS\x03\xCF\x14a\0qW[`\0\x80\xFD[a\0Na\0yV[\0[`\x04Ta\0]\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Na\x02\x86V[`\x02T`\0T`@\x80Qc&\xFE\x99Q`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93c\x1B\xE1\x95`\x93\x16\x91c&\xFE\x99Q\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\0\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xF1\x91\x90a\x03\x82V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x012W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01CWP`\x01[a\x02bW`\x02\x80T`\xFF`\xA0\x1B\x19\x16\x90U`\0\x80T`@\x80Qc\x06\xA2&\xC5`\xE5\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cp\xA0\x821\x91\x83\x91c\xD4D\xD8\xA0\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xCB\x91\x90a\x03\x82V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x023\x91\x90a\x03\xB2V[`@\x80Q\x92\x90\x91\x11` \x83\x01R\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x03\x90\x81a\x02_\x91\x90a\x04jV[PV[`\x02\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`@\x80Q`\0` \x82\x01R\x01a\x02BV[`\x02T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x03NW`\x03\x80Ta\x02\xA4\x90a\x03\xE1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xD0\x90a\x03\xE1V[\x80\x15a\x03\x1DW\x80`\x1F\x10a\x02\xF2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x1DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\x035\x91\x90a\x05*V[`\x04\x80T\x91\x15\x15a\x01\0\x02a\xFF\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U[`\x04Ta\x01\0\x90\x04`\xFF\x16\x80\x15a\x03oWP`\x02T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15[`\x04\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0` \x82\x84\x03\x12\x15a\x03\x94W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xABW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\xC4W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\xF5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\x15WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x04eW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x04BWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04aW\x82\x81U`\x01\x01a\x04NV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x84Wa\x04\x84a\x03\xCBV[a\x04\x98\x81a\x04\x92\x84Ta\x03\xE1V[\x84a\x04\x1BV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x04\xCDW`\0\x84\x15a\x04\xB5WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x04aV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x04\xFCW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x04\xDDV[P\x85\x82\x10\x15a\x05\x1AW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15a\x05<W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xABW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xEFO\xB5\xC4I\xD6tq\xBF\x86\xCEkB\xB5\xDF*h\xB1\r\x8Cf;[\xC4]\x99\xE15\xE9W\xF7\xC1dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static DOUBLEENTRYPOINTCHECK_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DoubleEntryPointCheck<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DoubleEntryPointCheck<M> {
        fn clone(&self) -> Self { Self(::core::clone::Clone::clone(&self.0)) }
    }
    impl<M> ::core::ops::Deref for DoubleEntryPointCheck<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> ::core::ops::DerefMut for DoubleEntryPointCheck<M> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl<M> ::core::fmt::Debug for DoubleEntryPointCheck<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DoubleEntryPointCheck))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DoubleEntryPointCheck<M> {
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
                DOUBLEENTRYPOINTCHECK_ABI.clone(),
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
                DOUBLEENTRYPOINTCHECK_ABI.clone(),
                DOUBLEENTRYPOINTCHECK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checker` (0xcf5303cf)
        /// function
        pub fn checker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 83, 3, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `result` (0x65372147)
        /// function
        pub fn result(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([101, 55, 33, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweep` (0x35faa416)
        /// function
        pub fn sweep(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 250, 164, 22], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for DoubleEntryPointCheck<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the
    /// `checker` function with signature `checker()` and
    /// selector `0xcf5303cf`
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
    #[ethcall(name = "checker", abi = "checker()")]
    pub struct CheckerCall;
    ///Container type for all input parameters for the
    /// `result` function with signature `result()` and
    /// selector `0x65372147`
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
    #[ethcall(name = "result", abi = "result()")]
    pub struct ResultCall;
    ///Container type for all input parameters for the
    /// `sweep` function with signature `sweep()` and
    /// selector `0x35faa416`
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
    #[ethcall(name = "sweep", abi = "sweep()")]
    pub struct SweepCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash,
    )]
    pub enum DoubleEntryPointCheckCalls {
        Checker(CheckerCall),
        Result(ResultCall),
        Sweep(SweepCall),
    }
    impl ::ethers::core::abi::AbiDecode for DoubleEntryPointCheckCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Checker(decoded));
            }
            if let Ok(decoded) =
                <ResultCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Result(decoded));
            }
            if let Ok(decoded) =
                <SweepCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Sweep(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DoubleEntryPointCheckCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Checker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Result(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sweep(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DoubleEntryPointCheckCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Checker(element) => ::core::fmt::Display::fmt(element, f),
                Self::Result(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sweep(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckerCall> for DoubleEntryPointCheckCalls {
        fn from(value: CheckerCall) -> Self { Self::Checker(value) }
    }
    impl ::core::convert::From<ResultCall> for DoubleEntryPointCheckCalls {
        fn from(value: ResultCall) -> Self { Self::Result(value) }
    }
    impl ::core::convert::From<SweepCall> for DoubleEntryPointCheckCalls {
        fn from(value: SweepCall) -> Self { Self::Sweep(value) }
    }
    ///Container type for all return fields from the
    /// `result` function with signature `result()` and
    /// selector `0x65372147`
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
    pub struct ResultReturn(pub bool);
}
