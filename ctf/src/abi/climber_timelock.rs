///Module containing a contract's types and functions.
/**

```solidity
library ClimberTimelockBase {
    type OperationState is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ClimberTimelockBase {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperationState(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<OperationState> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl OperationState {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for OperationState {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<OperationState> for u8 {
            fn from(value: OperationState) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperationState {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperationState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ClimberTimelockBase`](self) contract instance.

See the [wrapper's documentation](`ClimberTimelockBaseInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ClimberTimelockBaseInstance<P, N> {
        ClimberTimelockBaseInstance::<P, N>::new(address, __provider)
    }
    /**A [`ClimberTimelockBase`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ClimberTimelockBase`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ClimberTimelockBaseInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ClimberTimelockBaseInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ClimberTimelockBaseInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ClimberTimelockBaseInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ClimberTimelockBase`](self) contract instance.

See the [wrapper's documentation](`ClimberTimelockBaseInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> ClimberTimelockBaseInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ClimberTimelockBaseInstance<P, N> {
            ClimberTimelockBaseInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ClimberTimelockBaseInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ClimberTimelockBaseInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ClimberTimelockBase {
    type OperationState is uint8;
}

interface ClimberTimelock {
    error CallerNotTimelock();
    error InvalidDataElementsCount();
    error InvalidTargetsCount();
    error InvalidValuesCount();
    error NewDelayAboveMax();
    error NotReadyForExecution(bytes32 operationId);
    error OperationAlreadyKnown(bytes32 operationId);

    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);

    constructor(address admin, address proposer);

    receive() external payable;

    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function delay() external view returns (uint64);
    function execute(address[] memory targets, uint256[] memory values, bytes[] memory dataElements, bytes32 salt) external payable;
    function getOperationId(address[] memory targets, uint256[] memory values, bytes[] memory dataElements, bytes32 salt) external pure returns (bytes32);
    function getOperationState(bytes32 id) external view returns (ClimberTimelockBase.OperationState state);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function operations(bytes32) external view returns (uint64 readyAtTimestamp, bool known, bool executed);
    function renounceRole(bytes32 role, address account) external;
    function revokeRole(bytes32 role, address account) external;
    function schedule(address[] memory targets, uint256[] memory values, bytes[] memory dataElements, bytes32 salt) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function updateDelay(uint64 newDelay) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "proposer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delay",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "execute",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "dataElements",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getOperationId",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "dataElements",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getOperationState",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "state",
        "type": "uint8",
        "internalType": "enum ClimberTimelockBase.OperationState"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operations",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "readyAtTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "known",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "executed",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "schedule",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "dataElements",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateDelay",
    "inputs": [
      {
        "name": "newDelay",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "CallerNotTimelock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDataElementsCount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTargetsCount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidValuesCount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NewDelayAboveMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotReadyForExecution",
    "inputs": [
      {
        "name": "operationId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "OperationAlreadyKnown",
    "inputs": [
      {
        "name": "operationId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ClimberTimelock {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50604051612568380380612568833981810160405281019061003191906103dc565b6100857fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217755f1b7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217755f1b6101a060201b60201c565b6100d97fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc15f1b7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217755f1b6101a060201b60201c565b61010b7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217755f1b836101fe60201b60201c565b61013d7fa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c217755f1b306101fe60201b60201c565b61016f7fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc15f1b826101fe60201b60201c565b610e1060025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550505061041a565b5f6101b08361021260201b60201c565b9050815f5f8581526020019081526020015f20600101819055508181847fbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff60405160405180910390a4505050565b61020e828261022e60201b60201c565b5050565b5f5f5f8381526020019081526020015f20600101549050919050565b61023e828261031460201b60201c565b6103105760015f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506102b561037760201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45b5050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6103ab82610382565b9050919050565b6103bb816103a1565b81146103c5575f5ffd5b50565b5f815190506103d6816103b2565b92915050565b5f5f604083850312156103f2576103f161037e565b5b5f6103ff858286016103c8565b9250506020610410858286016103c8565b9150509250929050565b612141806104275f395ff3fe6080604052600436106100e0575f3560e01c80636a42b8f81161007e57806391d148541161005857806391d14854146102c1578063a217fddf146102fd578063c74f349b14610327578063d547741f14610365576100e7565b80636a42b8f8146102335780637958004c1461025d57806390bd1e6d14610299576100e7565b80632656227d116100ba5780632656227d1461018b5780632f2ff15d146101a757806336568abe146101cf57806357f525ed146101f7576100e7565b806301ffc9a7146100eb578063248a9ca31461012757806324adbc5b14610163576100e7565b366100e757005b5f5ffd5b3480156100f6575f5ffd5b50610111600480360381019061010c9190611372565b61038d565b60405161011e91906113b7565b60405180910390f35b348015610132575f5ffd5b5061014d60048036038101906101489190611403565b610406565b60405161015a919061143d565b60405180910390f35b34801561016e575f5ffd5b5061018960048036038101906101849190611493565b610422565b005b6101a560048036038101906101a091906115c9565b6104f9565b005b3480156101b2575f5ffd5b506101cd60048036038101906101c891906116e7565b610765565b005b3480156101da575f5ffd5b506101f560048036038101906101f091906116e7565b610786565b005b348015610202575f5ffd5b5061021d600480360381019061021891906115c9565b610809565b60405161022a919061143d565b60405180910390f35b34801561023e575f5ffd5b5061024761084a565b6040516102549190611734565b60405180910390f35b348015610268575f5ffd5b50610283600480360381019061027e9190611403565b610863565b60405161029091906117c0565b60405180910390f35b3480156102a4575f5ffd5b506102bf60048036038101906102ba91906115c9565b610937565b005b3480156102cc575f5ffd5b506102e760048036038101906102e291906116e7565b610b3f565b6040516102f491906113b7565b60405180910390f35b348015610308575f5ffd5b50610311610ba2565b60405161031e919061143d565b60405180910390f35b348015610332575f5ffd5b5061034d60048036038101906103489190611403565b610ba8565b60405161035c939291906117d9565b60405180910390f35b348015610370575f5ffd5b5061038b600480360381019061038691906116e7565b610bf9565b005b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806103ff57506103fe82610c1a565b5b9050919050565b5f5f5f8381526020019081526020015f20600101549050919050565b3073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610487576040517fdfb49e3100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b621275008167ffffffffffffffff1611156104ce576040517f78f424c400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8060025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff16021790555050565b5f8787905011610535576040517f576405a300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b848490508787905014610574576040517f2e2c60fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8282905087879050146105b3576040517f76cefbcb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6105c388888888888888610809565b90505f5f90505b888890508160ff1610156106be576106b285858360ff168181106105f1576105f061180e565b5b90506020028101906106039190611847565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505088888460ff1681811061065b5761065a61180e565b5b905060200201358b8b8560ff168181106106785761067761180e565b5b905060200201602081019061068d91906118a9565b73ffffffffffffffffffffffffffffffffffffffff16610c839092919063ffffffff16565b508060010190506105ca565b50600260038111156106d3576106d261174d565b5b6106dc82610863565b60038111156106ee576106ed61174d565b5b1461073057806040517f414afe48000000000000000000000000000000000000000000000000000000008152600401610727919061143d565b60405180910390fd5b6001805f8381526020019081526020015f205f0160096101000a81548160ff0219169083151502179055505050505050505050565b61076e82610406565b61077781610cb2565b6107818383610cc6565b505050565b61078e610da0565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146107fb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016107f290611954565b60405180910390fd5b6108058282610da7565b5050565b5f878787878787876040516020016108279796959493929190611c1c565b604051602081830303815290604052805190602001209050979650505050505050565b60025f9054906101000a900467ffffffffffffffff1681565b5f5f60015f8481526020019081526020015f206040518060600160405290815f82015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160089054906101000a900460ff161515151581526020015f820160099054906101000a900460ff161515151581525050905080602001511561092d578060400151156109045760039150610928565b805f015167ffffffffffffffff164210156109225760019150610927565b600291505b5b610931565b5f91505b50919050565b7fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc15f1b61096381610cb2565b5f88889050148061097957506101008888905010155b156109b0576040517f576405a300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8585905088889050146109ef576040517f2e2c60fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b838390508888905014610a2e576040517f76cefbcb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610a3e89898989898989610809565b90505f6003811115610a5357610a5261174d565b5b610a5c82610863565b6003811115610a6e57610a6d61174d565b5b14610ab057806040517f416333a2000000000000000000000000000000000000000000000000000000008152600401610aa7919061143d565b60405180910390fd5b60025f9054906101000a900467ffffffffffffffff1642610ad19190611ca7565b60015f8381526020019081526020015f205f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001805f8381526020019081526020015f205f0160086101000a81548160ff021916908315150217905550505050505050505050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b6001602052805f5260405f205f91509050805f015f9054906101000a900467ffffffffffffffff1690805f0160089054906101000a900460ff1690805f0160099054906101000a900460ff16905083565b610c0282610406565b610c0b81610cb2565b610c158383610da7565b505050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b6060610ca98484846040518060600160405280602981526020016120e360299139610e81565b90509392505050565b610cc381610cbe610da0565b610f4a565b50565b610cd08282610b3f565b610d9c5760015f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610d41610da0565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45b5050565b5f33905090565b610db18282610b3f565b15610e7d575f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610e22610da0565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16837ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a45b5050565b606082471015610ec6576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ebd90611d52565b60405180910390fd5b5f5f8673ffffffffffffffffffffffffffffffffffffffff168587604051610eee9190611dc2565b5f6040518083038185875af1925050503d805f8114610f28576040519150601f19603f3d011682016040523d82523d5f602084013e610f2d565b606091505b5091509150610f3e87838387610fce565b92505050949350505050565b610f548282610b3f565b610fca57610f6181611042565b610f6e835f1c602061106f565b604051602001610f7f929190611eb0565b6040516020818303038152906040526040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610fc19190611f21565b60405180910390fd5b5050565b6060831561102f575f83510361102757610fe7856112a4565b611026576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161101d90611f8b565b60405180910390fd5b5b82905061103a565b61103983836112c6565b5b949350505050565b60606110688273ffffffffffffffffffffffffffffffffffffffff16601460ff1661106f565b9050919050565b60605f60028360026110819190611fb2565b61108b9190611ff3565b67ffffffffffffffff8111156110a4576110a3612026565b5b6040519080825280601f01601f1916602001820160405280156110d65781602001600182028036833780820191505090505b5090507f3000000000000000000000000000000000000000000000000000000000000000815f8151811061110d5761110c61180e565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053507f7800000000000000000000000000000000000000000000000000000000000000816001815181106111705761116f61180e565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053505f60018460026111ae9190611fb2565b6111b89190611ff3565b90505b6001811115611257577f3031323334353637383961626364656600000000000000000000000000000000600f8616601081106111fa576111f961180e565b5b1a60f81b8282815181106112115761121061180e565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350600485901c94508061125090612053565b90506111bb565b505f841461129a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611291906120c4565b60405180910390fd5b8091505092915050565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f825111156112d85781518083602001fd5b806040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161130c9190611f21565b60405180910390fd5b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6113518161131d565b811461135b575f5ffd5b50565b5f8135905061136c81611348565b92915050565b5f6020828403121561138757611386611315565b5b5f6113948482850161135e565b91505092915050565b5f8115159050919050565b6113b18161139d565b82525050565b5f6020820190506113ca5f8301846113a8565b92915050565b5f819050919050565b6113e2816113d0565b81146113ec575f5ffd5b50565b5f813590506113fd816113d9565b92915050565b5f6020828403121561141857611417611315565b5b5f611425848285016113ef565b91505092915050565b611437816113d0565b82525050565b5f6020820190506114505f83018461142e565b92915050565b5f67ffffffffffffffff82169050919050565b61147281611456565b811461147c575f5ffd5b50565b5f8135905061148d81611469565b92915050565b5f602082840312156114a8576114a7611315565b5b5f6114b58482850161147f565b91505092915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126114df576114de6114be565b5b8235905067ffffffffffffffff8111156114fc576114fb6114c2565b5b602083019150836020820283011115611518576115176114c6565b5b9250929050565b5f5f83601f840112611534576115336114be565b5b8235905067ffffffffffffffff811115611551576115506114c2565b5b60208301915083602082028301111561156d5761156c6114c6565b5b9250929050565b5f5f83601f840112611589576115886114be565b5b8235905067ffffffffffffffff8111156115a6576115a56114c2565b5b6020830191508360208202830111156115c2576115c16114c6565b5b9250929050565b5f5f5f5f5f5f5f6080888a0312156115e4576115e3611315565b5b5f88013567ffffffffffffffff81111561160157611600611319565b5b61160d8a828b016114ca565b9750975050602088013567ffffffffffffffff8111156116305761162f611319565b5b61163c8a828b0161151f565b9550955050604088013567ffffffffffffffff81111561165f5761165e611319565b5b61166b8a828b01611574565b9350935050606061167e8a828b016113ef565b91505092959891949750929550565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6116b68261168d565b9050919050565b6116c6816116ac565b81146116d0575f5ffd5b50565b5f813590506116e1816116bd565b92915050565b5f5f604083850312156116fd576116fc611315565b5b5f61170a858286016113ef565b925050602061171b858286016116d3565b9150509250929050565b61172e81611456565b82525050565b5f6020820190506117475f830184611725565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061178b5761178a61174d565b5b50565b5f81905061179b8261177a565b919050565b5f6117aa8261178e565b9050919050565b6117ba816117a0565b82525050565b5f6020820190506117d35f8301846117b1565b92915050565b5f6060820190506117ec5f830186611725565b6117f960208301856113a8565b61180660408301846113a8565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f833560016020038436030381126118635761186261183b565b5b80840192508235915067ffffffffffffffff8211156118855761188461183f565b5b6020830192506001820236038313156118a1576118a0611843565b5b509250929050565b5f602082840312156118be576118bd611315565b5b5f6118cb848285016116d3565b91505092915050565b5f82825260208201905092915050565b7f416363657373436f6e74726f6c3a2063616e206f6e6c792072656e6f756e63655f8201527f20726f6c657320666f722073656c660000000000000000000000000000000000602082015250565b5f61193e602f836118d4565b9150611949826118e4565b604082019050919050565b5f6020820190508181035f83015261196b81611932565b9050919050565b5f82825260208201905092915050565b5f819050919050565b611994816116ac565b82525050565b5f6119a5838361198b565b60208301905092915050565b5f6119bf60208401846116d3565b905092915050565b5f602082019050919050565b5f6119de8385611972565b93506119e982611982565b805f5b85811015611a21576119fe82846119b1565b611a08888261199a565b9750611a13836119c7565b9250506001810190506119ec565b5085925050509392505050565b5f82825260208201905092915050565b5f5ffd5b82818337505050565b5f611a568385611a2e565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115611a8957611a88611a3e565b5b602083029250611a9a838584611a42565b82840190509392505050565b5f82825260208201905092915050565b5f819050919050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f611af88385611abf565b9350611b05838584611acf565b611b0e83611add565b840190509392505050565b5f611b25848484611aed565b90509392505050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83356001602003843603038112611b5657611b55611b36565b5b83810192508235915060208301925067ffffffffffffffff821115611b7e57611b7d611b2e565b5b600182023603831315611b9457611b93611b32565b5b509250929050565b5f602082019050919050565b5f611bb38385611aa6565b935083602084028501611bc584611ab6565b805f5b87811015611c0a578484038952611bdf8284611b3a565b611bea868284611b19565b9550611bf584611b9c565b935060208b019a505050600181019050611bc8565b50829750879450505050509392505050565b5f6080820190508181035f830152611c3581898b6119d3565b90508181036020830152611c4a818789611a4b565b90508181036040830152611c5f818587611ba8565b9050611c6e606083018461142e565b98975050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f611cb182611456565b9150611cbc83611456565b9250828201905067ffffffffffffffff811115611cdc57611cdb611c7a565b5b92915050565b7f416464726573733a20696e73756666696369656e742062616c616e636520666f5f8201527f722063616c6c0000000000000000000000000000000000000000000000000000602082015250565b5f611d3c6026836118d4565b9150611d4782611ce2565b604082019050919050565b5f6020820190508181035f830152611d6981611d30565b9050919050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f611d9c82611d70565b611da68185611d7a565b9350611db6818560208601611d84565b80840191505092915050565b5f611dcd8284611d92565b915081905092915050565b5f81905092915050565b7f416363657373436f6e74726f6c3a206163636f756e74200000000000000000005f82015250565b5f611e16601783611dd8565b9150611e2182611de2565b601782019050919050565b5f81519050919050565b5f611e4082611e2c565b611e4a8185611dd8565b9350611e5a818560208601611d84565b80840191505092915050565b7f206973206d697373696e6720726f6c65200000000000000000000000000000005f82015250565b5f611e9a601183611dd8565b9150611ea582611e66565b601182019050919050565b5f611eba82611e0a565b9150611ec68285611e36565b9150611ed182611e8e565b9150611edd8284611e36565b91508190509392505050565b5f611ef382611e2c565b611efd81856118d4565b9350611f0d818560208601611d84565b611f1681611add565b840191505092915050565b5f6020820190508181035f830152611f398184611ee9565b905092915050565b7f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000005f82015250565b5f611f75601d836118d4565b9150611f8082611f41565b602082019050919050565b5f6020820190508181035f830152611fa281611f69565b9050919050565b5f819050919050565b5f611fbc82611fa9565b9150611fc783611fa9565b9250828202611fd581611fa9565b91508282048414831517611fec57611feb611c7a565b5b5092915050565b5f611ffd82611fa9565b915061200883611fa9565b92508282019050808211156120205761201f611c7a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f61205d82611fa9565b91505f820361206f5761206e611c7a565b5b600182039050919050565b7f537472696e67733a20686578206c656e67746820696e73756666696369656e745f82015250565b5f6120ae6020836118d4565b91506120b98261207a565b602082019050919050565b5f6020820190508181035f8301526120db816120a2565b905091905056fe416464726573733a206c6f772d6c6576656c2063616c6c20776974682076616c7565206661696c6564a2646970667358221220a8f436f009b9991ae75b49995c387901af5e55a7d1b0c0480de58c12cb74197c64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa%h8\x03\x80a%h\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x03\xDCV[a\0\x85\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u_\x1B\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u_\x1Ba\x01\xA0` \x1B` \x1CV[a\0\xD9\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1_\x1B\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u_\x1Ba\x01\xA0` \x1B` \x1CV[a\x01\x0B\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u_\x1B\x83a\x01\xFE` \x1B` \x1CV[a\x01=\x7F\xA4\x98\x07 \\\xE4\xD3U\t.\xF5\xA8\xA1\x8FV\xE8\x91<\xF4\xA2\x01\xFB\xE2\x87\x82[\tV\x93\xC2\x17u_\x1B0a\x01\xFE` \x1B` \x1CV[a\x01o\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1_\x1B\x82a\x01\xFE` \x1B` \x1CV[a\x0E\x10`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPa\x04\x1AV[_a\x01\xB0\x83a\x02\x12` \x1B` \x1CV[\x90P\x81__\x85\x81R` \x01\x90\x81R` \x01_ `\x01\x01\x81\x90UP\x81\x81\x84\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[a\x02\x0E\x82\x82a\x02.` \x1B` \x1CV[PPV[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[a\x02>\x82\x82a\x03\x14` \x1B` \x1CV[a\x03\x10W`\x01__\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x02\xB5a\x03w` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x03\xAB\x82a\x03\x82V[\x90P\x91\x90PV[a\x03\xBB\x81a\x03\xA1V[\x81\x14a\x03\xC5W__\xFD[PV[_\x81Q\x90Pa\x03\xD6\x81a\x03\xB2V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x03\xF2Wa\x03\xF1a\x03~V[[_a\x03\xFF\x85\x82\x86\x01a\x03\xC8V[\x92PP` a\x04\x10\x85\x82\x86\x01a\x03\xC8V[\x91PP\x92P\x92\x90PV[a!A\x80a\x04'_9_\xF3\xFE`\x80`@R`\x046\x10a\0\xE0W_5`\xE0\x1C\x80cjB\xB8\xF8\x11a\0~W\x80c\x91\xD1HT\x11a\0XW\x80c\x91\xD1HT\x14a\x02\xC1W\x80c\xA2\x17\xFD\xDF\x14a\x02\xFDW\x80c\xC7O4\x9B\x14a\x03'W\x80c\xD5Gt\x1F\x14a\x03eWa\0\xE7V[\x80cjB\xB8\xF8\x14a\x023W\x80cyX\0L\x14a\x02]W\x80c\x90\xBD\x1Em\x14a\x02\x99Wa\0\xE7V[\x80c&V\"}\x11a\0\xBAW\x80c&V\"}\x14a\x01\x8BW\x80c//\xF1]\x14a\x01\xA7W\x80c6V\x8A\xBE\x14a\x01\xCFW\x80cW\xF5%\xED\x14a\x01\xF7Wa\0\xE7V[\x80c\x01\xFF\xC9\xA7\x14a\0\xEBW\x80c$\x8A\x9C\xA3\x14a\x01'W\x80c$\xAD\xBC[\x14a\x01cWa\0\xE7V[6a\0\xE7W\0[__\xFD[4\x80\x15a\0\xF6W__\xFD[Pa\x01\x11`\x04\x806\x03\x81\x01\x90a\x01\x0C\x91\x90a\x13rV[a\x03\x8DV[`@Qa\x01\x1E\x91\x90a\x13\xB7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x012W__\xFD[Pa\x01M`\x04\x806\x03\x81\x01\x90a\x01H\x91\x90a\x14\x03V[a\x04\x06V[`@Qa\x01Z\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01nW__\xFD[Pa\x01\x89`\x04\x806\x03\x81\x01\x90a\x01\x84\x91\x90a\x14\x93V[a\x04\"V[\0[a\x01\xA5`\x04\x806\x03\x81\x01\x90a\x01\xA0\x91\x90a\x15\xC9V[a\x04\xF9V[\0[4\x80\x15a\x01\xB2W__\xFD[Pa\x01\xCD`\x04\x806\x03\x81\x01\x90a\x01\xC8\x91\x90a\x16\xE7V[a\x07eV[\0[4\x80\x15a\x01\xDAW__\xFD[Pa\x01\xF5`\x04\x806\x03\x81\x01\x90a\x01\xF0\x91\x90a\x16\xE7V[a\x07\x86V[\0[4\x80\x15a\x02\x02W__\xFD[Pa\x02\x1D`\x04\x806\x03\x81\x01\x90a\x02\x18\x91\x90a\x15\xC9V[a\x08\tV[`@Qa\x02*\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02>W__\xFD[Pa\x02Ga\x08JV[`@Qa\x02T\x91\x90a\x174V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02hW__\xFD[Pa\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a\x14\x03V[a\x08cV[`@Qa\x02\x90\x91\x90a\x17\xC0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA4W__\xFD[Pa\x02\xBF`\x04\x806\x03\x81\x01\x90a\x02\xBA\x91\x90a\x15\xC9V[a\t7V[\0[4\x80\x15a\x02\xCCW__\xFD[Pa\x02\xE7`\x04\x806\x03\x81\x01\x90a\x02\xE2\x91\x90a\x16\xE7V[a\x0B?V[`@Qa\x02\xF4\x91\x90a\x13\xB7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x08W__\xFD[Pa\x03\x11a\x0B\xA2V[`@Qa\x03\x1E\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x032W__\xFD[Pa\x03M`\x04\x806\x03\x81\x01\x90a\x03H\x91\x90a\x14\x03V[a\x0B\xA8V[`@Qa\x03\\\x93\x92\x91\x90a\x17\xD9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03pW__\xFD[Pa\x03\x8B`\x04\x806\x03\x81\x01\x90a\x03\x86\x91\x90a\x16\xE7V[a\x0B\xF9V[\0[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03\xFFWPa\x03\xFE\x82a\x0C\x1AV[[\x90P\x91\x90PV[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x87W`@Q\x7F\xDF\xB4\x9E1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x12u\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x04\xCEW`@Q\x7Fx\xF4$\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[_\x87\x87\x90P\x11a\x055W`@Q\x7FWd\x05\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90P\x87\x87\x90P\x14a\x05tW`@Q\x7F.,`\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x82\x90P\x87\x87\x90P\x14a\x05\xB3W`@Q\x7Fv\xCE\xFB\xCB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x05\xC3\x88\x88\x88\x88\x88\x88\x88a\x08\tV[\x90P__\x90P[\x88\x88\x90P\x81`\xFF\x16\x10\x15a\x06\xBEWa\x06\xB2\x85\x85\x83`\xFF\x16\x81\x81\x10a\x05\xF1Wa\x05\xF0a\x18\x0EV[[\x90P` \x02\x81\x01\x90a\x06\x03\x91\x90a\x18GV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x88\x88\x84`\xFF\x16\x81\x81\x10a\x06[Wa\x06Za\x18\x0EV[[\x90P` \x02\x015\x8B\x8B\x85`\xFF\x16\x81\x81\x10a\x06xWa\x06wa\x18\x0EV[[\x90P` \x02\x01` \x81\x01\x90a\x06\x8D\x91\x90a\x18\xA9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0C\x83\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80`\x01\x01\x90Pa\x05\xCAV[P`\x02`\x03\x81\x11\x15a\x06\xD3Wa\x06\xD2a\x17MV[[a\x06\xDC\x82a\x08cV[`\x03\x81\x11\x15a\x06\xEEWa\x06\xEDa\x17MV[[\x14a\x070W\x80`@Q\x7FAJ\xFEH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07'\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xFD[`\x01\x80_\x83\x81R` \x01\x90\x81R` \x01_ _\x01`\ta\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPPPV[a\x07n\x82a\x04\x06V[a\x07w\x81a\x0C\xB2V[a\x07\x81\x83\x83a\x0C\xC6V[PPPV[a\x07\x8Ea\r\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xF2\x90a\x19TV[`@Q\x80\x91\x03\x90\xFD[a\x08\x05\x82\x82a\r\xA7V[PPV[_\x87\x87\x87\x87\x87\x87\x87`@Q` \x01a\x08'\x97\x96\x95\x94\x93\x92\x91\x90a\x1C\x1CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__`\x01_\x84\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x15\x15\x81R` \x01_\x82\x01`\t\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x15\x15\x81RPP\x90P\x80` \x01Q\x15a\t-W\x80`@\x01Q\x15a\t\x04W`\x03\x91Pa\t(V[\x80_\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x10\x15a\t\"W`\x01\x91Pa\t'V[`\x02\x91P[[a\t1V[_\x91P[P\x91\x90PV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1_\x1Ba\tc\x81a\x0C\xB2V[_\x88\x88\x90P\x14\x80a\tyWPa\x01\0\x88\x88\x90P\x10\x15[\x15a\t\xB0W`@Q\x7FWd\x05\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x85\x90P\x88\x88\x90P\x14a\t\xEFW`@Q\x7F.,`\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x83\x90P\x88\x88\x90P\x14a\n.W`@Q\x7Fv\xCE\xFB\xCB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n>\x89\x89\x89\x89\x89\x89\x89a\x08\tV[\x90P_`\x03\x81\x11\x15a\nSWa\nRa\x17MV[[a\n\\\x82a\x08cV[`\x03\x81\x11\x15a\nnWa\nma\x17MV[[\x14a\n\xB0W\x80`@Q\x7FAc3\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xA7\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xFD[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\n\xD1\x91\x90a\x1C\xA7V[`\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x80_\x83\x81R` \x01\x90\x81R` \x01_ _\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPPPPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[`\x01` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x80_\x01`\t\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x83V[a\x0C\x02\x82a\x04\x06V[a\x0C\x0B\x81a\x0C\xB2V[a\x0C\x15\x83\x83a\r\xA7V[PPPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[``a\x0C\xA9\x84\x84\x84`@Q\x80``\x01`@R\x80`)\x81R` \x01a \xE3`)\x919a\x0E\x81V[\x90P\x93\x92PPPV[a\x0C\xC3\x81a\x0C\xBEa\r\xA0V[a\x0FJV[PV[a\x0C\xD0\x82\x82a\x0B?V[a\r\x9CW`\x01__\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\rAa\r\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[_3\x90P\x90V[a\r\xB1\x82\x82a\x0B?V[\x15a\x0E}W___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0E\"a\r\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[``\x82G\x10\x15a\x0E\xC6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xBD\x90a\x1DRV[`@Q\x80\x91\x03\x90\xFD[__\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x87`@Qa\x0E\xEE\x91\x90a\x1D\xC2V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0F(W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0F-V[``\x91P[P\x91P\x91Pa\x0F>\x87\x83\x83\x87a\x0F\xCEV[\x92PPP\x94\x93PPPPV[a\x0FT\x82\x82a\x0B?V[a\x0F\xCAWa\x0Fa\x81a\x10BV[a\x0Fn\x83_\x1C` a\x10oV[`@Q` \x01a\x0F\x7F\x92\x91\x90a\x1E\xB0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xC1\x91\x90a\x1F!V[`@Q\x80\x91\x03\x90\xFD[PPV[``\x83\x15a\x10/W_\x83Q\x03a\x10'Wa\x0F\xE7\x85a\x12\xA4V[a\x10&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x1D\x90a\x1F\x8BV[`@Q\x80\x91\x03\x90\xFD[[\x82\x90Pa\x10:V[a\x109\x83\x83a\x12\xC6V[[\x94\x93PPPPV[``a\x10h\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x10oV[\x90P\x91\x90PV[``_`\x02\x83`\x02a\x10\x81\x91\x90a\x1F\xB2V[a\x10\x8B\x91\x90a\x1F\xF3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xA4Wa\x10\xA3a &V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10\xD6W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\x11\rWa\x11\x0Ca\x18\x0EV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x11pWa\x11oa\x18\x0EV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP_`\x01\x84`\x02a\x11\xAE\x91\x90a\x1F\xB2V[a\x11\xB8\x91\x90a\x1F\xF3V[\x90P[`\x01\x81\x11\x15a\x12WW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\x11\xFAWa\x11\xF9a\x18\x0EV[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x12\x11Wa\x12\x10a\x18\x0EV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x12P\x90a SV[\x90Pa\x11\xBBV[P_\x84\x14a\x12\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\x91\x90a \xC4V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_\x82Q\x11\x15a\x12\xD8W\x81Q\x80\x83` \x01\xFD[\x80`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x0C\x91\x90a\x1F!V[`@Q\x80\x91\x03\x90\xFD[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x13Q\x81a\x13\x1DV[\x81\x14a\x13[W__\xFD[PV[_\x815\x90Pa\x13l\x81a\x13HV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\x87Wa\x13\x86a\x13\x15V[[_a\x13\x94\x84\x82\x85\x01a\x13^V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x13\xB1\x81a\x13\x9DV[\x82RPPV[_` \x82\x01\x90Pa\x13\xCA_\x83\x01\x84a\x13\xA8V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x13\xE2\x81a\x13\xD0V[\x81\x14a\x13\xECW__\xFD[PV[_\x815\x90Pa\x13\xFD\x81a\x13\xD9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\x18Wa\x14\x17a\x13\x15V[[_a\x14%\x84\x82\x85\x01a\x13\xEFV[\x91PP\x92\x91PPV[a\x147\x81a\x13\xD0V[\x82RPPV[_` \x82\x01\x90Pa\x14P_\x83\x01\x84a\x14.V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x14r\x81a\x14VV[\x81\x14a\x14|W__\xFD[PV[_\x815\x90Pa\x14\x8D\x81a\x14iV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xA8Wa\x14\xA7a\x13\x15V[[_a\x14\xB5\x84\x82\x85\x01a\x14\x7FV[\x91PP\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a\x14\xDFWa\x14\xDEa\x14\xBEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xFCWa\x14\xFBa\x14\xC2V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x15\x18Wa\x15\x17a\x14\xC6V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x154Wa\x153a\x14\xBEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15QWa\x15Pa\x14\xC2V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x15mWa\x15la\x14\xC6V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x15\x89Wa\x15\x88a\x14\xBEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xA6Wa\x15\xA5a\x14\xC2V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x15\xC2Wa\x15\xC1a\x14\xC6V[[\x92P\x92\x90PV[_______`\x80\x88\x8A\x03\x12\x15a\x15\xE4Wa\x15\xE3a\x13\x15V[[_\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x01Wa\x16\0a\x13\x19V[[a\x16\r\x8A\x82\x8B\x01a\x14\xCAV[\x97P\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x160Wa\x16/a\x13\x19V[[a\x16<\x8A\x82\x8B\x01a\x15\x1FV[\x95P\x95PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16_Wa\x16^a\x13\x19V[[a\x16k\x8A\x82\x8B\x01a\x15tV[\x93P\x93PP``a\x16~\x8A\x82\x8B\x01a\x13\xEFV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x16\xB6\x82a\x16\x8DV[\x90P\x91\x90PV[a\x16\xC6\x81a\x16\xACV[\x81\x14a\x16\xD0W__\xFD[PV[_\x815\x90Pa\x16\xE1\x81a\x16\xBDV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x16\xFDWa\x16\xFCa\x13\x15V[[_a\x17\n\x85\x82\x86\x01a\x13\xEFV[\x92PP` a\x17\x1B\x85\x82\x86\x01a\x16\xD3V[\x91PP\x92P\x92\x90PV[a\x17.\x81a\x14VV[\x82RPPV[_` \x82\x01\x90Pa\x17G_\x83\x01\x84a\x17%V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x17\x8BWa\x17\x8Aa\x17MV[[PV[_\x81\x90Pa\x17\x9B\x82a\x17zV[\x91\x90PV[_a\x17\xAA\x82a\x17\x8EV[\x90P\x91\x90PV[a\x17\xBA\x81a\x17\xA0V[\x82RPPV[_` \x82\x01\x90Pa\x17\xD3_\x83\x01\x84a\x17\xB1V[\x92\x91PPV[_``\x82\x01\x90Pa\x17\xEC_\x83\x01\x86a\x17%V[a\x17\xF9` \x83\x01\x85a\x13\xA8V[a\x18\x06`@\x83\x01\x84a\x13\xA8V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[__\xFD[__\xFD[__\xFD[__\x835`\x01` \x03\x846\x03\x03\x81\x12a\x18cWa\x18ba\x18;V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\x85Wa\x18\x84a\x18?V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15a\x18\xA1Wa\x18\xA0a\x18CV[[P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x18\xBEWa\x18\xBDa\x13\x15V[[_a\x18\xCB\x84\x82\x85\x01a\x16\xD3V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FAccessControl: can only renounce_\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x19>`/\x83a\x18\xD4V[\x91Pa\x19I\x82a\x18\xE4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19k\x81a\x192V[\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x19\x94\x81a\x16\xACV[\x82RPPV[_a\x19\xA5\x83\x83a\x19\x8BV[` \x83\x01\x90P\x92\x91PPV[_a\x19\xBF` \x84\x01\x84a\x16\xD3V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x19\xDE\x83\x85a\x19rV[\x93Pa\x19\xE9\x82a\x19\x82V[\x80_[\x85\x81\x10\x15a\x1A!Wa\x19\xFE\x82\x84a\x19\xB1V[a\x1A\x08\x88\x82a\x19\x9AV[\x97Pa\x1A\x13\x83a\x19\xC7V[\x92PP`\x01\x81\x01\x90Pa\x19\xECV[P\x85\x92PPP\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[__\xFD[\x82\x81\x837PPPV[_a\x1AV\x83\x85a\x1A.V[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x1A\x89Wa\x1A\x88a\x1A>V[[` \x83\x02\x92Pa\x1A\x9A\x83\x85\x84a\x1ABV[\x82\x84\x01\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x837_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a\x1A\xF8\x83\x85a\x1A\xBFV[\x93Pa\x1B\x05\x83\x85\x84a\x1A\xCFV[a\x1B\x0E\x83a\x1A\xDDV[\x84\x01\x90P\x93\x92PPPV[_a\x1B%\x84\x84\x84a\x1A\xEDV[\x90P\x93\x92PPPV[__\xFD[__\xFD[__\xFD[__\x835`\x01` \x03\x846\x03\x03\x81\x12a\x1BVWa\x1BUa\x1B6V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1B~Wa\x1B}a\x1B.V[[`\x01\x82\x026\x03\x83\x13\x15a\x1B\x94Wa\x1B\x93a\x1B2V[[P\x92P\x92\x90PV[_` \x82\x01\x90P\x91\x90PV[_a\x1B\xB3\x83\x85a\x1A\xA6V[\x93P\x83` \x84\x02\x85\x01a\x1B\xC5\x84a\x1A\xB6V[\x80_[\x87\x81\x10\x15a\x1C\nW\x84\x84\x03\x89Ra\x1B\xDF\x82\x84a\x1B:V[a\x1B\xEA\x86\x82\x84a\x1B\x19V[\x95Pa\x1B\xF5\x84a\x1B\x9CV[\x93P` \x8B\x01\x9APPP`\x01\x81\x01\x90Pa\x1B\xC8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C5\x81\x89\x8Ba\x19\xD3V[\x90P\x81\x81\x03` \x83\x01Ra\x1CJ\x81\x87\x89a\x1AKV[\x90P\x81\x81\x03`@\x83\x01Ra\x1C_\x81\x85\x87a\x1B\xA8V[\x90Pa\x1Cn``\x83\x01\x84a\x14.V[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x1C\xB1\x82a\x14VV[\x91Pa\x1C\xBC\x83a\x14VV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xDCWa\x1C\xDBa\x1CzV[[\x92\x91PPV[\x7FAddress: insufficient balance fo_\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1D<`&\x83a\x18\xD4V[\x91Pa\x1DG\x82a\x1C\xE2V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1Di\x81a\x1D0V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x1D\x9C\x82a\x1DpV[a\x1D\xA6\x81\x85a\x1DzV[\x93Pa\x1D\xB6\x81\x85` \x86\x01a\x1D\x84V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x1D\xCD\x82\x84a\x1D\x92V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x1E\x16`\x17\x83a\x1D\xD8V[\x91Pa\x1E!\x82a\x1D\xE2V[`\x17\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_a\x1E@\x82a\x1E,V[a\x1EJ\x81\x85a\x1D\xD8V[\x93Pa\x1EZ\x81\x85` \x86\x01a\x1D\x84V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x1E\x9A`\x11\x83a\x1D\xD8V[\x91Pa\x1E\xA5\x82a\x1EfV[`\x11\x82\x01\x90P\x91\x90PV[_a\x1E\xBA\x82a\x1E\nV[\x91Pa\x1E\xC6\x82\x85a\x1E6V[\x91Pa\x1E\xD1\x82a\x1E\x8EV[\x91Pa\x1E\xDD\x82\x84a\x1E6V[\x91P\x81\x90P\x93\x92PPPV[_a\x1E\xF3\x82a\x1E,V[a\x1E\xFD\x81\x85a\x18\xD4V[\x93Pa\x1F\r\x81\x85` \x86\x01a\x1D\x84V[a\x1F\x16\x81a\x1A\xDDV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1F9\x81\x84a\x1E\xE9V[\x90P\x92\x91PPV[\x7FAddress: call to non-contract\0\0\0_\x82\x01RPV[_a\x1Fu`\x1D\x83a\x18\xD4V[\x91Pa\x1F\x80\x82a\x1FAV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1F\xA2\x81a\x1FiV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x1F\xBC\x82a\x1F\xA9V[\x91Pa\x1F\xC7\x83a\x1F\xA9V[\x92P\x82\x82\x02a\x1F\xD5\x81a\x1F\xA9V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1F\xECWa\x1F\xEBa\x1CzV[[P\x92\x91PPV[_a\x1F\xFD\x82a\x1F\xA9V[\x91Pa \x08\x83a\x1F\xA9V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a  Wa \x1Fa\x1CzV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_a ]\x82a\x1F\xA9V[\x91P_\x82\x03a oWa na\x1CzV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient_\x82\x01RPV[_a \xAE` \x83a\x18\xD4V[\x91Pa \xB9\x82a zV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra \xDB\x81a \xA2V[\x90P\x91\x90PV\xFEAddress: low-level call with value failed\xA2dipfsX\"\x12 \xA8\xF46\xF0\t\xB9\x99\x1A\xE7[I\x99\\8y\x01\xAF^U\xA7\xD1\xB0\xC0H\r\xE5\x8C\x12\xCBt\x19|dsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100e0575f3560e01c80636a42b8f81161007e57806391d148541161005857806391d14854146102c1578063a217fddf146102fd578063c74f349b14610327578063d547741f14610365576100e7565b80636a42b8f8146102335780637958004c1461025d57806390bd1e6d14610299576100e7565b80632656227d116100ba5780632656227d1461018b5780632f2ff15d146101a757806336568abe146101cf57806357f525ed146101f7576100e7565b806301ffc9a7146100eb578063248a9ca31461012757806324adbc5b14610163576100e7565b366100e757005b5f5ffd5b3480156100f6575f5ffd5b50610111600480360381019061010c9190611372565b61038d565b60405161011e91906113b7565b60405180910390f35b348015610132575f5ffd5b5061014d60048036038101906101489190611403565b610406565b60405161015a919061143d565b60405180910390f35b34801561016e575f5ffd5b5061018960048036038101906101849190611493565b610422565b005b6101a560048036038101906101a091906115c9565b6104f9565b005b3480156101b2575f5ffd5b506101cd60048036038101906101c891906116e7565b610765565b005b3480156101da575f5ffd5b506101f560048036038101906101f091906116e7565b610786565b005b348015610202575f5ffd5b5061021d600480360381019061021891906115c9565b610809565b60405161022a919061143d565b60405180910390f35b34801561023e575f5ffd5b5061024761084a565b6040516102549190611734565b60405180910390f35b348015610268575f5ffd5b50610283600480360381019061027e9190611403565b610863565b60405161029091906117c0565b60405180910390f35b3480156102a4575f5ffd5b506102bf60048036038101906102ba91906115c9565b610937565b005b3480156102cc575f5ffd5b506102e760048036038101906102e291906116e7565b610b3f565b6040516102f491906113b7565b60405180910390f35b348015610308575f5ffd5b50610311610ba2565b60405161031e919061143d565b60405180910390f35b348015610332575f5ffd5b5061034d60048036038101906103489190611403565b610ba8565b60405161035c939291906117d9565b60405180910390f35b348015610370575f5ffd5b5061038b600480360381019061038691906116e7565b610bf9565b005b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806103ff57506103fe82610c1a565b5b9050919050565b5f5f5f8381526020019081526020015f20600101549050919050565b3073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610487576040517fdfb49e3100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b621275008167ffffffffffffffff1611156104ce576040517f78f424c400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8060025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff16021790555050565b5f8787905011610535576040517f576405a300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b848490508787905014610574576040517f2e2c60fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8282905087879050146105b3576040517f76cefbcb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6105c388888888888888610809565b90505f5f90505b888890508160ff1610156106be576106b285858360ff168181106105f1576105f061180e565b5b90506020028101906106039190611847565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505088888460ff1681811061065b5761065a61180e565b5b905060200201358b8b8560ff168181106106785761067761180e565b5b905060200201602081019061068d91906118a9565b73ffffffffffffffffffffffffffffffffffffffff16610c839092919063ffffffff16565b508060010190506105ca565b50600260038111156106d3576106d261174d565b5b6106dc82610863565b60038111156106ee576106ed61174d565b5b1461073057806040517f414afe48000000000000000000000000000000000000000000000000000000008152600401610727919061143d565b60405180910390fd5b6001805f8381526020019081526020015f205f0160096101000a81548160ff0219169083151502179055505050505050505050565b61076e82610406565b61077781610cb2565b6107818383610cc6565b505050565b61078e610da0565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146107fb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016107f290611954565b60405180910390fd5b6108058282610da7565b5050565b5f878787878787876040516020016108279796959493929190611c1c565b604051602081830303815290604052805190602001209050979650505050505050565b60025f9054906101000a900467ffffffffffffffff1681565b5f5f60015f8481526020019081526020015f206040518060600160405290815f82015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160089054906101000a900460ff161515151581526020015f820160099054906101000a900460ff161515151581525050905080602001511561092d578060400151156109045760039150610928565b805f015167ffffffffffffffff164210156109225760019150610927565b600291505b5b610931565b5f91505b50919050565b7fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc15f1b61096381610cb2565b5f88889050148061097957506101008888905010155b156109b0576040517f576405a300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8585905088889050146109ef576040517f2e2c60fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b838390508888905014610a2e576040517f76cefbcb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610a3e89898989898989610809565b90505f6003811115610a5357610a5261174d565b5b610a5c82610863565b6003811115610a6e57610a6d61174d565b5b14610ab057806040517f416333a2000000000000000000000000000000000000000000000000000000008152600401610aa7919061143d565b60405180910390fd5b60025f9054906101000a900467ffffffffffffffff1642610ad19190611ca7565b60015f8381526020019081526020015f205f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001805f8381526020019081526020015f205f0160086101000a81548160ff021916908315150217905550505050505050505050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b6001602052805f5260405f205f91509050805f015f9054906101000a900467ffffffffffffffff1690805f0160089054906101000a900460ff1690805f0160099054906101000a900460ff16905083565b610c0282610406565b610c0b81610cb2565b610c158383610da7565b505050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b6060610ca98484846040518060600160405280602981526020016120e360299139610e81565b90509392505050565b610cc381610cbe610da0565b610f4a565b50565b610cd08282610b3f565b610d9c5760015f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610d41610da0565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45b5050565b5f33905090565b610db18282610b3f565b15610e7d575f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610e22610da0565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16837ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a45b5050565b606082471015610ec6576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ebd90611d52565b60405180910390fd5b5f5f8673ffffffffffffffffffffffffffffffffffffffff168587604051610eee9190611dc2565b5f6040518083038185875af1925050503d805f8114610f28576040519150601f19603f3d011682016040523d82523d5f602084013e610f2d565b606091505b5091509150610f3e87838387610fce565b92505050949350505050565b610f548282610b3f565b610fca57610f6181611042565b610f6e835f1c602061106f565b604051602001610f7f929190611eb0565b6040516020818303038152906040526040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610fc19190611f21565b60405180910390fd5b5050565b6060831561102f575f83510361102757610fe7856112a4565b611026576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161101d90611f8b565b60405180910390fd5b5b82905061103a565b61103983836112c6565b5b949350505050565b60606110688273ffffffffffffffffffffffffffffffffffffffff16601460ff1661106f565b9050919050565b60605f60028360026110819190611fb2565b61108b9190611ff3565b67ffffffffffffffff8111156110a4576110a3612026565b5b6040519080825280601f01601f1916602001820160405280156110d65781602001600182028036833780820191505090505b5090507f3000000000000000000000000000000000000000000000000000000000000000815f8151811061110d5761110c61180e565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053507f7800000000000000000000000000000000000000000000000000000000000000816001815181106111705761116f61180e565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053505f60018460026111ae9190611fb2565b6111b89190611ff3565b90505b6001811115611257577f3031323334353637383961626364656600000000000000000000000000000000600f8616601081106111fa576111f961180e565b5b1a60f81b8282815181106112115761121061180e565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350600485901c94508061125090612053565b90506111bb565b505f841461129a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611291906120c4565b60405180910390fd5b8091505092915050565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f825111156112d85781518083602001fd5b806040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161130c9190611f21565b60405180910390fd5b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6113518161131d565b811461135b575f5ffd5b50565b5f8135905061136c81611348565b92915050565b5f6020828403121561138757611386611315565b5b5f6113948482850161135e565b91505092915050565b5f8115159050919050565b6113b18161139d565b82525050565b5f6020820190506113ca5f8301846113a8565b92915050565b5f819050919050565b6113e2816113d0565b81146113ec575f5ffd5b50565b5f813590506113fd816113d9565b92915050565b5f6020828403121561141857611417611315565b5b5f611425848285016113ef565b91505092915050565b611437816113d0565b82525050565b5f6020820190506114505f83018461142e565b92915050565b5f67ffffffffffffffff82169050919050565b61147281611456565b811461147c575f5ffd5b50565b5f8135905061148d81611469565b92915050565b5f602082840312156114a8576114a7611315565b5b5f6114b58482850161147f565b91505092915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126114df576114de6114be565b5b8235905067ffffffffffffffff8111156114fc576114fb6114c2565b5b602083019150836020820283011115611518576115176114c6565b5b9250929050565b5f5f83601f840112611534576115336114be565b5b8235905067ffffffffffffffff811115611551576115506114c2565b5b60208301915083602082028301111561156d5761156c6114c6565b5b9250929050565b5f5f83601f840112611589576115886114be565b5b8235905067ffffffffffffffff8111156115a6576115a56114c2565b5b6020830191508360208202830111156115c2576115c16114c6565b5b9250929050565b5f5f5f5f5f5f5f6080888a0312156115e4576115e3611315565b5b5f88013567ffffffffffffffff81111561160157611600611319565b5b61160d8a828b016114ca565b9750975050602088013567ffffffffffffffff8111156116305761162f611319565b5b61163c8a828b0161151f565b9550955050604088013567ffffffffffffffff81111561165f5761165e611319565b5b61166b8a828b01611574565b9350935050606061167e8a828b016113ef565b91505092959891949750929550565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6116b68261168d565b9050919050565b6116c6816116ac565b81146116d0575f5ffd5b50565b5f813590506116e1816116bd565b92915050565b5f5f604083850312156116fd576116fc611315565b5b5f61170a858286016113ef565b925050602061171b858286016116d3565b9150509250929050565b61172e81611456565b82525050565b5f6020820190506117475f830184611725565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061178b5761178a61174d565b5b50565b5f81905061179b8261177a565b919050565b5f6117aa8261178e565b9050919050565b6117ba816117a0565b82525050565b5f6020820190506117d35f8301846117b1565b92915050565b5f6060820190506117ec5f830186611725565b6117f960208301856113a8565b61180660408301846113a8565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f833560016020038436030381126118635761186261183b565b5b80840192508235915067ffffffffffffffff8211156118855761188461183f565b5b6020830192506001820236038313156118a1576118a0611843565b5b509250929050565b5f602082840312156118be576118bd611315565b5b5f6118cb848285016116d3565b91505092915050565b5f82825260208201905092915050565b7f416363657373436f6e74726f6c3a2063616e206f6e6c792072656e6f756e63655f8201527f20726f6c657320666f722073656c660000000000000000000000000000000000602082015250565b5f61193e602f836118d4565b9150611949826118e4565b604082019050919050565b5f6020820190508181035f83015261196b81611932565b9050919050565b5f82825260208201905092915050565b5f819050919050565b611994816116ac565b82525050565b5f6119a5838361198b565b60208301905092915050565b5f6119bf60208401846116d3565b905092915050565b5f602082019050919050565b5f6119de8385611972565b93506119e982611982565b805f5b85811015611a21576119fe82846119b1565b611a08888261199a565b9750611a13836119c7565b9250506001810190506119ec565b5085925050509392505050565b5f82825260208201905092915050565b5f5ffd5b82818337505050565b5f611a568385611a2e565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115611a8957611a88611a3e565b5b602083029250611a9a838584611a42565b82840190509392505050565b5f82825260208201905092915050565b5f819050919050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f611af88385611abf565b9350611b05838584611acf565b611b0e83611add565b840190509392505050565b5f611b25848484611aed565b90509392505050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83356001602003843603038112611b5657611b55611b36565b5b83810192508235915060208301925067ffffffffffffffff821115611b7e57611b7d611b2e565b5b600182023603831315611b9457611b93611b32565b5b509250929050565b5f602082019050919050565b5f611bb38385611aa6565b935083602084028501611bc584611ab6565b805f5b87811015611c0a578484038952611bdf8284611b3a565b611bea868284611b19565b9550611bf584611b9c565b935060208b019a505050600181019050611bc8565b50829750879450505050509392505050565b5f6080820190508181035f830152611c3581898b6119d3565b90508181036020830152611c4a818789611a4b565b90508181036040830152611c5f818587611ba8565b9050611c6e606083018461142e565b98975050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f611cb182611456565b9150611cbc83611456565b9250828201905067ffffffffffffffff811115611cdc57611cdb611c7a565b5b92915050565b7f416464726573733a20696e73756666696369656e742062616c616e636520666f5f8201527f722063616c6c0000000000000000000000000000000000000000000000000000602082015250565b5f611d3c6026836118d4565b9150611d4782611ce2565b604082019050919050565b5f6020820190508181035f830152611d6981611d30565b9050919050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f611d9c82611d70565b611da68185611d7a565b9350611db6818560208601611d84565b80840191505092915050565b5f611dcd8284611d92565b915081905092915050565b5f81905092915050565b7f416363657373436f6e74726f6c3a206163636f756e74200000000000000000005f82015250565b5f611e16601783611dd8565b9150611e2182611de2565b601782019050919050565b5f81519050919050565b5f611e4082611e2c565b611e4a8185611dd8565b9350611e5a818560208601611d84565b80840191505092915050565b7f206973206d697373696e6720726f6c65200000000000000000000000000000005f82015250565b5f611e9a601183611dd8565b9150611ea582611e66565b601182019050919050565b5f611eba82611e0a565b9150611ec68285611e36565b9150611ed182611e8e565b9150611edd8284611e36565b91508190509392505050565b5f611ef382611e2c565b611efd81856118d4565b9350611f0d818560208601611d84565b611f1681611add565b840191505092915050565b5f6020820190508181035f830152611f398184611ee9565b905092915050565b7f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000005f82015250565b5f611f75601d836118d4565b9150611f8082611f41565b602082019050919050565b5f6020820190508181035f830152611fa281611f69565b9050919050565b5f819050919050565b5f611fbc82611fa9565b9150611fc783611fa9565b9250828202611fd581611fa9565b91508282048414831517611fec57611feb611c7a565b5b5092915050565b5f611ffd82611fa9565b915061200883611fa9565b92508282019050808211156120205761201f611c7a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f61205d82611fa9565b91505f820361206f5761206e611c7a565b5b600182039050919050565b7f537472696e67733a20686578206c656e67746820696e73756666696369656e745f82015250565b5f6120ae6020836118d4565b91506120b98261207a565b602082019050919050565b5f6020820190508181035f8301526120db816120a2565b905091905056fe416464726573733a206c6f772d6c6576656c2063616c6c20776974682076616c7565206661696c6564a2646970667358221220a8f436f009b9991ae75b49995c387901af5e55a7d1b0c0480de58c12cb74197c64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xE0W_5`\xE0\x1C\x80cjB\xB8\xF8\x11a\0~W\x80c\x91\xD1HT\x11a\0XW\x80c\x91\xD1HT\x14a\x02\xC1W\x80c\xA2\x17\xFD\xDF\x14a\x02\xFDW\x80c\xC7O4\x9B\x14a\x03'W\x80c\xD5Gt\x1F\x14a\x03eWa\0\xE7V[\x80cjB\xB8\xF8\x14a\x023W\x80cyX\0L\x14a\x02]W\x80c\x90\xBD\x1Em\x14a\x02\x99Wa\0\xE7V[\x80c&V\"}\x11a\0\xBAW\x80c&V\"}\x14a\x01\x8BW\x80c//\xF1]\x14a\x01\xA7W\x80c6V\x8A\xBE\x14a\x01\xCFW\x80cW\xF5%\xED\x14a\x01\xF7Wa\0\xE7V[\x80c\x01\xFF\xC9\xA7\x14a\0\xEBW\x80c$\x8A\x9C\xA3\x14a\x01'W\x80c$\xAD\xBC[\x14a\x01cWa\0\xE7V[6a\0\xE7W\0[__\xFD[4\x80\x15a\0\xF6W__\xFD[Pa\x01\x11`\x04\x806\x03\x81\x01\x90a\x01\x0C\x91\x90a\x13rV[a\x03\x8DV[`@Qa\x01\x1E\x91\x90a\x13\xB7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x012W__\xFD[Pa\x01M`\x04\x806\x03\x81\x01\x90a\x01H\x91\x90a\x14\x03V[a\x04\x06V[`@Qa\x01Z\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01nW__\xFD[Pa\x01\x89`\x04\x806\x03\x81\x01\x90a\x01\x84\x91\x90a\x14\x93V[a\x04\"V[\0[a\x01\xA5`\x04\x806\x03\x81\x01\x90a\x01\xA0\x91\x90a\x15\xC9V[a\x04\xF9V[\0[4\x80\x15a\x01\xB2W__\xFD[Pa\x01\xCD`\x04\x806\x03\x81\x01\x90a\x01\xC8\x91\x90a\x16\xE7V[a\x07eV[\0[4\x80\x15a\x01\xDAW__\xFD[Pa\x01\xF5`\x04\x806\x03\x81\x01\x90a\x01\xF0\x91\x90a\x16\xE7V[a\x07\x86V[\0[4\x80\x15a\x02\x02W__\xFD[Pa\x02\x1D`\x04\x806\x03\x81\x01\x90a\x02\x18\x91\x90a\x15\xC9V[a\x08\tV[`@Qa\x02*\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02>W__\xFD[Pa\x02Ga\x08JV[`@Qa\x02T\x91\x90a\x174V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02hW__\xFD[Pa\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a\x14\x03V[a\x08cV[`@Qa\x02\x90\x91\x90a\x17\xC0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA4W__\xFD[Pa\x02\xBF`\x04\x806\x03\x81\x01\x90a\x02\xBA\x91\x90a\x15\xC9V[a\t7V[\0[4\x80\x15a\x02\xCCW__\xFD[Pa\x02\xE7`\x04\x806\x03\x81\x01\x90a\x02\xE2\x91\x90a\x16\xE7V[a\x0B?V[`@Qa\x02\xF4\x91\x90a\x13\xB7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x08W__\xFD[Pa\x03\x11a\x0B\xA2V[`@Qa\x03\x1E\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x032W__\xFD[Pa\x03M`\x04\x806\x03\x81\x01\x90a\x03H\x91\x90a\x14\x03V[a\x0B\xA8V[`@Qa\x03\\\x93\x92\x91\x90a\x17\xD9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03pW__\xFD[Pa\x03\x8B`\x04\x806\x03\x81\x01\x90a\x03\x86\x91\x90a\x16\xE7V[a\x0B\xF9V[\0[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03\xFFWPa\x03\xFE\x82a\x0C\x1AV[[\x90P\x91\x90PV[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x87W`@Q\x7F\xDF\xB4\x9E1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x12u\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x04\xCEW`@Q\x7Fx\xF4$\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[_\x87\x87\x90P\x11a\x055W`@Q\x7FWd\x05\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90P\x87\x87\x90P\x14a\x05tW`@Q\x7F.,`\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x82\x90P\x87\x87\x90P\x14a\x05\xB3W`@Q\x7Fv\xCE\xFB\xCB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x05\xC3\x88\x88\x88\x88\x88\x88\x88a\x08\tV[\x90P__\x90P[\x88\x88\x90P\x81`\xFF\x16\x10\x15a\x06\xBEWa\x06\xB2\x85\x85\x83`\xFF\x16\x81\x81\x10a\x05\xF1Wa\x05\xF0a\x18\x0EV[[\x90P` \x02\x81\x01\x90a\x06\x03\x91\x90a\x18GV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x88\x88\x84`\xFF\x16\x81\x81\x10a\x06[Wa\x06Za\x18\x0EV[[\x90P` \x02\x015\x8B\x8B\x85`\xFF\x16\x81\x81\x10a\x06xWa\x06wa\x18\x0EV[[\x90P` \x02\x01` \x81\x01\x90a\x06\x8D\x91\x90a\x18\xA9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0C\x83\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80`\x01\x01\x90Pa\x05\xCAV[P`\x02`\x03\x81\x11\x15a\x06\xD3Wa\x06\xD2a\x17MV[[a\x06\xDC\x82a\x08cV[`\x03\x81\x11\x15a\x06\xEEWa\x06\xEDa\x17MV[[\x14a\x070W\x80`@Q\x7FAJ\xFEH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07'\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xFD[`\x01\x80_\x83\x81R` \x01\x90\x81R` \x01_ _\x01`\ta\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPPPV[a\x07n\x82a\x04\x06V[a\x07w\x81a\x0C\xB2V[a\x07\x81\x83\x83a\x0C\xC6V[PPPV[a\x07\x8Ea\r\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xF2\x90a\x19TV[`@Q\x80\x91\x03\x90\xFD[a\x08\x05\x82\x82a\r\xA7V[PPV[_\x87\x87\x87\x87\x87\x87\x87`@Q` \x01a\x08'\x97\x96\x95\x94\x93\x92\x91\x90a\x1C\x1CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__`\x01_\x84\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x15\x15\x81R` \x01_\x82\x01`\t\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x15\x15\x81RPP\x90P\x80` \x01Q\x15a\t-W\x80`@\x01Q\x15a\t\x04W`\x03\x91Pa\t(V[\x80_\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x10\x15a\t\"W`\x01\x91Pa\t'V[`\x02\x91P[[a\t1V[_\x91P[P\x91\x90PV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1_\x1Ba\tc\x81a\x0C\xB2V[_\x88\x88\x90P\x14\x80a\tyWPa\x01\0\x88\x88\x90P\x10\x15[\x15a\t\xB0W`@Q\x7FWd\x05\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x85\x90P\x88\x88\x90P\x14a\t\xEFW`@Q\x7F.,`\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x83\x90P\x88\x88\x90P\x14a\n.W`@Q\x7Fv\xCE\xFB\xCB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n>\x89\x89\x89\x89\x89\x89\x89a\x08\tV[\x90P_`\x03\x81\x11\x15a\nSWa\nRa\x17MV[[a\n\\\x82a\x08cV[`\x03\x81\x11\x15a\nnWa\nma\x17MV[[\x14a\n\xB0W\x80`@Q\x7FAc3\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xA7\x91\x90a\x14=V[`@Q\x80\x91\x03\x90\xFD[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\n\xD1\x91\x90a\x1C\xA7V[`\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x80_\x83\x81R` \x01\x90\x81R` \x01_ _\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPPPPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[`\x01` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x80_\x01`\t\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x83V[a\x0C\x02\x82a\x04\x06V[a\x0C\x0B\x81a\x0C\xB2V[a\x0C\x15\x83\x83a\r\xA7V[PPPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[``a\x0C\xA9\x84\x84\x84`@Q\x80``\x01`@R\x80`)\x81R` \x01a \xE3`)\x919a\x0E\x81V[\x90P\x93\x92PPPV[a\x0C\xC3\x81a\x0C\xBEa\r\xA0V[a\x0FJV[PV[a\x0C\xD0\x82\x82a\x0B?V[a\r\x9CW`\x01__\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\rAa\r\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[_3\x90P\x90V[a\r\xB1\x82\x82a\x0B?V[\x15a\x0E}W___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0E\"a\r\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[``\x82G\x10\x15a\x0E\xC6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xBD\x90a\x1DRV[`@Q\x80\x91\x03\x90\xFD[__\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x87`@Qa\x0E\xEE\x91\x90a\x1D\xC2V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0F(W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0F-V[``\x91P[P\x91P\x91Pa\x0F>\x87\x83\x83\x87a\x0F\xCEV[\x92PPP\x94\x93PPPPV[a\x0FT\x82\x82a\x0B?V[a\x0F\xCAWa\x0Fa\x81a\x10BV[a\x0Fn\x83_\x1C` a\x10oV[`@Q` \x01a\x0F\x7F\x92\x91\x90a\x1E\xB0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xC1\x91\x90a\x1F!V[`@Q\x80\x91\x03\x90\xFD[PPV[``\x83\x15a\x10/W_\x83Q\x03a\x10'Wa\x0F\xE7\x85a\x12\xA4V[a\x10&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x1D\x90a\x1F\x8BV[`@Q\x80\x91\x03\x90\xFD[[\x82\x90Pa\x10:V[a\x109\x83\x83a\x12\xC6V[[\x94\x93PPPPV[``a\x10h\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x10oV[\x90P\x91\x90PV[``_`\x02\x83`\x02a\x10\x81\x91\x90a\x1F\xB2V[a\x10\x8B\x91\x90a\x1F\xF3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xA4Wa\x10\xA3a &V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10\xD6W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\x11\rWa\x11\x0Ca\x18\x0EV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x11pWa\x11oa\x18\x0EV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP_`\x01\x84`\x02a\x11\xAE\x91\x90a\x1F\xB2V[a\x11\xB8\x91\x90a\x1F\xF3V[\x90P[`\x01\x81\x11\x15a\x12WW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\x11\xFAWa\x11\xF9a\x18\x0EV[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x12\x11Wa\x12\x10a\x18\x0EV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x12P\x90a SV[\x90Pa\x11\xBBV[P_\x84\x14a\x12\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\x91\x90a \xC4V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_\x82Q\x11\x15a\x12\xD8W\x81Q\x80\x83` \x01\xFD[\x80`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x0C\x91\x90a\x1F!V[`@Q\x80\x91\x03\x90\xFD[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x13Q\x81a\x13\x1DV[\x81\x14a\x13[W__\xFD[PV[_\x815\x90Pa\x13l\x81a\x13HV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\x87Wa\x13\x86a\x13\x15V[[_a\x13\x94\x84\x82\x85\x01a\x13^V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x13\xB1\x81a\x13\x9DV[\x82RPPV[_` \x82\x01\x90Pa\x13\xCA_\x83\x01\x84a\x13\xA8V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x13\xE2\x81a\x13\xD0V[\x81\x14a\x13\xECW__\xFD[PV[_\x815\x90Pa\x13\xFD\x81a\x13\xD9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\x18Wa\x14\x17a\x13\x15V[[_a\x14%\x84\x82\x85\x01a\x13\xEFV[\x91PP\x92\x91PPV[a\x147\x81a\x13\xD0V[\x82RPPV[_` \x82\x01\x90Pa\x14P_\x83\x01\x84a\x14.V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x14r\x81a\x14VV[\x81\x14a\x14|W__\xFD[PV[_\x815\x90Pa\x14\x8D\x81a\x14iV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xA8Wa\x14\xA7a\x13\x15V[[_a\x14\xB5\x84\x82\x85\x01a\x14\x7FV[\x91PP\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a\x14\xDFWa\x14\xDEa\x14\xBEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xFCWa\x14\xFBa\x14\xC2V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x15\x18Wa\x15\x17a\x14\xC6V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x154Wa\x153a\x14\xBEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15QWa\x15Pa\x14\xC2V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x15mWa\x15la\x14\xC6V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x15\x89Wa\x15\x88a\x14\xBEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xA6Wa\x15\xA5a\x14\xC2V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x15\xC2Wa\x15\xC1a\x14\xC6V[[\x92P\x92\x90PV[_______`\x80\x88\x8A\x03\x12\x15a\x15\xE4Wa\x15\xE3a\x13\x15V[[_\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x01Wa\x16\0a\x13\x19V[[a\x16\r\x8A\x82\x8B\x01a\x14\xCAV[\x97P\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x160Wa\x16/a\x13\x19V[[a\x16<\x8A\x82\x8B\x01a\x15\x1FV[\x95P\x95PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16_Wa\x16^a\x13\x19V[[a\x16k\x8A\x82\x8B\x01a\x15tV[\x93P\x93PP``a\x16~\x8A\x82\x8B\x01a\x13\xEFV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x16\xB6\x82a\x16\x8DV[\x90P\x91\x90PV[a\x16\xC6\x81a\x16\xACV[\x81\x14a\x16\xD0W__\xFD[PV[_\x815\x90Pa\x16\xE1\x81a\x16\xBDV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x16\xFDWa\x16\xFCa\x13\x15V[[_a\x17\n\x85\x82\x86\x01a\x13\xEFV[\x92PP` a\x17\x1B\x85\x82\x86\x01a\x16\xD3V[\x91PP\x92P\x92\x90PV[a\x17.\x81a\x14VV[\x82RPPV[_` \x82\x01\x90Pa\x17G_\x83\x01\x84a\x17%V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x17\x8BWa\x17\x8Aa\x17MV[[PV[_\x81\x90Pa\x17\x9B\x82a\x17zV[\x91\x90PV[_a\x17\xAA\x82a\x17\x8EV[\x90P\x91\x90PV[a\x17\xBA\x81a\x17\xA0V[\x82RPPV[_` \x82\x01\x90Pa\x17\xD3_\x83\x01\x84a\x17\xB1V[\x92\x91PPV[_``\x82\x01\x90Pa\x17\xEC_\x83\x01\x86a\x17%V[a\x17\xF9` \x83\x01\x85a\x13\xA8V[a\x18\x06`@\x83\x01\x84a\x13\xA8V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[__\xFD[__\xFD[__\xFD[__\x835`\x01` \x03\x846\x03\x03\x81\x12a\x18cWa\x18ba\x18;V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\x85Wa\x18\x84a\x18?V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15a\x18\xA1Wa\x18\xA0a\x18CV[[P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x18\xBEWa\x18\xBDa\x13\x15V[[_a\x18\xCB\x84\x82\x85\x01a\x16\xD3V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FAccessControl: can only renounce_\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x19>`/\x83a\x18\xD4V[\x91Pa\x19I\x82a\x18\xE4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19k\x81a\x192V[\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x19\x94\x81a\x16\xACV[\x82RPPV[_a\x19\xA5\x83\x83a\x19\x8BV[` \x83\x01\x90P\x92\x91PPV[_a\x19\xBF` \x84\x01\x84a\x16\xD3V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x19\xDE\x83\x85a\x19rV[\x93Pa\x19\xE9\x82a\x19\x82V[\x80_[\x85\x81\x10\x15a\x1A!Wa\x19\xFE\x82\x84a\x19\xB1V[a\x1A\x08\x88\x82a\x19\x9AV[\x97Pa\x1A\x13\x83a\x19\xC7V[\x92PP`\x01\x81\x01\x90Pa\x19\xECV[P\x85\x92PPP\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[__\xFD[\x82\x81\x837PPPV[_a\x1AV\x83\x85a\x1A.V[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x1A\x89Wa\x1A\x88a\x1A>V[[` \x83\x02\x92Pa\x1A\x9A\x83\x85\x84a\x1ABV[\x82\x84\x01\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x837_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a\x1A\xF8\x83\x85a\x1A\xBFV[\x93Pa\x1B\x05\x83\x85\x84a\x1A\xCFV[a\x1B\x0E\x83a\x1A\xDDV[\x84\x01\x90P\x93\x92PPPV[_a\x1B%\x84\x84\x84a\x1A\xEDV[\x90P\x93\x92PPPV[__\xFD[__\xFD[__\xFD[__\x835`\x01` \x03\x846\x03\x03\x81\x12a\x1BVWa\x1BUa\x1B6V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1B~Wa\x1B}a\x1B.V[[`\x01\x82\x026\x03\x83\x13\x15a\x1B\x94Wa\x1B\x93a\x1B2V[[P\x92P\x92\x90PV[_` \x82\x01\x90P\x91\x90PV[_a\x1B\xB3\x83\x85a\x1A\xA6V[\x93P\x83` \x84\x02\x85\x01a\x1B\xC5\x84a\x1A\xB6V[\x80_[\x87\x81\x10\x15a\x1C\nW\x84\x84\x03\x89Ra\x1B\xDF\x82\x84a\x1B:V[a\x1B\xEA\x86\x82\x84a\x1B\x19V[\x95Pa\x1B\xF5\x84a\x1B\x9CV[\x93P` \x8B\x01\x9APPP`\x01\x81\x01\x90Pa\x1B\xC8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C5\x81\x89\x8Ba\x19\xD3V[\x90P\x81\x81\x03` \x83\x01Ra\x1CJ\x81\x87\x89a\x1AKV[\x90P\x81\x81\x03`@\x83\x01Ra\x1C_\x81\x85\x87a\x1B\xA8V[\x90Pa\x1Cn``\x83\x01\x84a\x14.V[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x1C\xB1\x82a\x14VV[\x91Pa\x1C\xBC\x83a\x14VV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xDCWa\x1C\xDBa\x1CzV[[\x92\x91PPV[\x7FAddress: insufficient balance fo_\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1D<`&\x83a\x18\xD4V[\x91Pa\x1DG\x82a\x1C\xE2V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1Di\x81a\x1D0V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x1D\x9C\x82a\x1DpV[a\x1D\xA6\x81\x85a\x1DzV[\x93Pa\x1D\xB6\x81\x85` \x86\x01a\x1D\x84V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x1D\xCD\x82\x84a\x1D\x92V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x1E\x16`\x17\x83a\x1D\xD8V[\x91Pa\x1E!\x82a\x1D\xE2V[`\x17\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_a\x1E@\x82a\x1E,V[a\x1EJ\x81\x85a\x1D\xD8V[\x93Pa\x1EZ\x81\x85` \x86\x01a\x1D\x84V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x1E\x9A`\x11\x83a\x1D\xD8V[\x91Pa\x1E\xA5\x82a\x1EfV[`\x11\x82\x01\x90P\x91\x90PV[_a\x1E\xBA\x82a\x1E\nV[\x91Pa\x1E\xC6\x82\x85a\x1E6V[\x91Pa\x1E\xD1\x82a\x1E\x8EV[\x91Pa\x1E\xDD\x82\x84a\x1E6V[\x91P\x81\x90P\x93\x92PPPV[_a\x1E\xF3\x82a\x1E,V[a\x1E\xFD\x81\x85a\x18\xD4V[\x93Pa\x1F\r\x81\x85` \x86\x01a\x1D\x84V[a\x1F\x16\x81a\x1A\xDDV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1F9\x81\x84a\x1E\xE9V[\x90P\x92\x91PPV[\x7FAddress: call to non-contract\0\0\0_\x82\x01RPV[_a\x1Fu`\x1D\x83a\x18\xD4V[\x91Pa\x1F\x80\x82a\x1FAV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1F\xA2\x81a\x1FiV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x1F\xBC\x82a\x1F\xA9V[\x91Pa\x1F\xC7\x83a\x1F\xA9V[\x92P\x82\x82\x02a\x1F\xD5\x81a\x1F\xA9V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1F\xECWa\x1F\xEBa\x1CzV[[P\x92\x91PPV[_a\x1F\xFD\x82a\x1F\xA9V[\x91Pa \x08\x83a\x1F\xA9V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a  Wa \x1Fa\x1CzV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_a ]\x82a\x1F\xA9V[\x91P_\x82\x03a oWa na\x1CzV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient_\x82\x01RPV[_a \xAE` \x83a\x18\xD4V[\x91Pa \xB9\x82a zV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra \xDB\x81a \xA2V[\x90P\x91\x90PV\xFEAddress: low-level call with value failed\xA2dipfsX\"\x12 \xA8\xF46\xF0\t\xB9\x99\x1A\xE7[I\x99\\8y\x01\xAF^U\xA7\xD1\xB0\xC0H\r\xE5\x8C\x12\xCBt\x19|dsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CallerNotTimelock()` and selector `0xdfb49e31`.
```solidity
error CallerNotTimelock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CallerNotTimelock;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CallerNotTimelock> for UnderlyingRustTuple<'_> {
            fn from(value: CallerNotTimelock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CallerNotTimelock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CallerNotTimelock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CallerNotTimelock()";
            const SELECTOR: [u8; 4] = [223u8, 180u8, 158u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidDataElementsCount()` and selector `0x76cefbcb`.
```solidity
error InvalidDataElementsCount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDataElementsCount;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidDataElementsCount>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDataElementsCount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidDataElementsCount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDataElementsCount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDataElementsCount()";
            const SELECTOR: [u8; 4] = [118u8, 206u8, 251u8, 203u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidTargetsCount()` and selector `0x576405a3`.
```solidity
error InvalidTargetsCount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTargetsCount;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidTargetsCount> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTargetsCount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTargetsCount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTargetsCount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTargetsCount()";
            const SELECTOR: [u8; 4] = [87u8, 100u8, 5u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidValuesCount()` and selector `0x2e2c60fe`.
```solidity
error InvalidValuesCount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidValuesCount;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidValuesCount> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidValuesCount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidValuesCount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidValuesCount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidValuesCount()";
            const SELECTOR: [u8; 4] = [46u8, 44u8, 96u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NewDelayAboveMax()` and selector `0x78f424c4`.
```solidity
error NewDelayAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NewDelayAboveMax;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NewDelayAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: NewDelayAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NewDelayAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NewDelayAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NewDelayAboveMax()";
            const SELECTOR: [u8; 4] = [120u8, 244u8, 36u8, 196u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotReadyForExecution(bytes32)` and selector `0x414afe48`.
```solidity
error NotReadyForExecution(bytes32 operationId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotReadyForExecution {
        #[allow(missing_docs)]
        pub operationId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotReadyForExecution> for UnderlyingRustTuple<'_> {
            fn from(value: NotReadyForExecution) -> Self {
                (value.operationId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotReadyForExecution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { operationId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotReadyForExecution {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotReadyForExecution(bytes32)";
            const SELECTOR: [u8; 4] = [65u8, 74u8, 254u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operationId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OperationAlreadyKnown(bytes32)` and selector `0x416333a2`.
```solidity
error OperationAlreadyKnown(bytes32 operationId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperationAlreadyKnown {
        #[allow(missing_docs)]
        pub operationId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperationAlreadyKnown> for UnderlyingRustTuple<'_> {
            fn from(value: OperationAlreadyKnown) -> Self {
                (value.operationId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperationAlreadyKnown {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { operationId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperationAlreadyKnown {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperationAlreadyKnown(bytes32)";
            const SELECTOR: [u8; 4] = [65u8, 99u8, 51u8, 162u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operationId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
```solidity
event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
```solidity
event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
```solidity
event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address admin, address proposer);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.admin, value.proposer)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        admin: tuple.0,
                        proposer: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.admin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delay()` and selector `0x6a42b8f8`.
```solidity
function delay() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`delay()`](delayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delayCall> for UnderlyingRustTuple<'_> {
                fn from(value: delayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delay()";
            const SELECTOR: [u8; 4] = [106u8, 66u8, 184u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: delayReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: delayReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`.
```solidity
function execute(address[] memory targets, uint256[] memory values, bytes[] memory dataElements, bytes32 salt) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub dataElements: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`execute(address[],uint256[],bytes[],bytes32)`](executeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeCall) -> Self {
                    (value.targets, value.values, value.dataElements, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        dataElements: tuple.2,
                        salt: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl executeReturn {
            fn _tokenize(
                &self,
            ) -> <executeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execute(address[],uint256[],bytes[],bytes32)";
            const SELECTOR: [u8; 4] = [38u8, 86u8, 34u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.dataElements),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                executeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperationId(address[],uint256[],bytes[],bytes32)` and selector `0x57f525ed`.
```solidity
function getOperationId(address[] memory targets, uint256[] memory values, bytes[] memory dataElements, bytes32 salt) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperationIdCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub dataElements: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperationId(address[],uint256[],bytes[],bytes32)`](getOperationIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperationIdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperationIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperationIdCall) -> Self {
                    (value.targets, value.values, value.dataElements, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperationIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        dataElements: tuple.2,
                        salt: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperationIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperationIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperationIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperationIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperationId(address[],uint256[],bytes[],bytes32)";
            const SELECTOR: [u8; 4] = [87u8, 245u8, 37u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.dataElements),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getOperationIdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getOperationIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperationState(bytes32)` and selector `0x7958004c`.
```solidity
function getOperationState(bytes32 id) external view returns (ClimberTimelockBase.OperationState state);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperationStateCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperationState(bytes32)`](getOperationStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperationStateReturn {
        #[allow(missing_docs)]
        pub state: <ClimberTimelockBase::OperationState as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperationStateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperationStateCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperationStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (ClimberTimelockBase::OperationState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ClimberTimelockBase::OperationState as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperationStateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperationStateReturn) -> Self {
                    (value.state,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperationStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { state: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperationStateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ClimberTimelockBase::OperationState as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ClimberTimelockBase::OperationState,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperationState(bytes32)";
            const SELECTOR: [u8; 4] = [121u8, 88u8, 0u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <ClimberTimelockBase::OperationState as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getOperationStateReturn = r.into();
                        r.state
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getOperationStateReturn = r.into();
                        r.state
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
```solidity
function getRoleAdmin(bytes32 role) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
```solidity
function grantRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl grantRoleReturn {
            fn _tokenize(
                &self,
            ) -> <grantRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                grantRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
```solidity
function hasRole(bytes32 role, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `operations(bytes32)` and selector `0xc74f349b`.
```solidity
function operations(bytes32) external view returns (uint64 readyAtTimestamp, bool known, bool executed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operationsCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`operations(bytes32)`](operationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operationsReturn {
        #[allow(missing_docs)]
        pub readyAtTimestamp: u64,
        #[allow(missing_docs)]
        pub known: bool,
        #[allow(missing_docs)]
        pub executed: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: operationsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, bool, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operationsReturn) -> Self {
                    (value.readyAtTimestamp, value.known, value.executed)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        readyAtTimestamp: tuple.0,
                        known: tuple.1,
                        executed: tuple.2,
                    }
                }
            }
        }
        impl operationsReturn {
            fn _tokenize(
                &self,
            ) -> <operationsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.readyAtTimestamp),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.known,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.executed,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operationsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operationsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operations(bytes32)";
            const SELECTOR: [u8; 4] = [199u8, 79u8, 52u8, 155u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                operationsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
```solidity
function renounceRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceRoleReturn {
            fn _tokenize(
                &self,
            ) -> <renounceRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
```solidity
function revokeRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeRoleReturn {
            fn _tokenize(
                &self,
            ) -> <revokeRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `schedule(address[],uint256[],bytes[],bytes32)` and selector `0x90bd1e6d`.
```solidity
function schedule(address[] memory targets, uint256[] memory values, bytes[] memory dataElements, bytes32 salt) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub dataElements: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`schedule(address[],uint256[],bytes[],bytes32)`](scheduleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<scheduleCall> for UnderlyingRustTuple<'_> {
                fn from(value: scheduleCall) -> Self {
                    (value.targets, value.values, value.dataElements, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scheduleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        dataElements: tuple.2,
                        salt: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<scheduleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: scheduleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scheduleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scheduleReturn {
            fn _tokenize(
                &self,
            ) -> <scheduleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scheduleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "schedule(address[],uint256[],bytes[],bytes32)";
            const SELECTOR: [u8; 4] = [144u8, 189u8, 30u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.dataElements),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scheduleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `updateDelay(uint64)` and selector `0x24adbc5b`.
```solidity
function updateDelay(uint64 newDelay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateDelayCall {
        #[allow(missing_docs)]
        pub newDelay: u64,
    }
    ///Container type for the return parameters of the [`updateDelay(uint64)`](updateDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateDelayReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateDelayCall) -> Self {
                    (value.newDelay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newDelay: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateDelayReturn {
            fn _tokenize(
                &self,
            ) -> <updateDelayCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateDelayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateDelay(uint64)";
            const SELECTOR: [u8; 4] = [36u8, 173u8, 188u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.newDelay),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateDelayReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`ClimberTimelock`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ClimberTimelockCalls {
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        delay(delayCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        getOperationId(getOperationIdCall),
        #[allow(missing_docs)]
        getOperationState(getOperationStateCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        operations(operationsCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        schedule(scheduleCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        updateDelay(updateDelayCall),
    }
    impl ClimberTimelockCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [36u8, 138u8, 156u8, 163u8],
            [36u8, 173u8, 188u8, 91u8],
            [38u8, 86u8, 34u8, 125u8],
            [47u8, 47u8, 241u8, 93u8],
            [54u8, 86u8, 138u8, 190u8],
            [87u8, 245u8, 37u8, 237u8],
            [106u8, 66u8, 184u8, 248u8],
            [121u8, 88u8, 0u8, 76u8],
            [144u8, 189u8, 30u8, 109u8],
            [145u8, 209u8, 72u8, 84u8],
            [162u8, 23u8, 253u8, 223u8],
            [199u8, 79u8, 52u8, 155u8],
            [213u8, 71u8, 116u8, 31u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(updateDelay),
            ::core::stringify!(execute),
            ::core::stringify!(grantRole),
            ::core::stringify!(renounceRole),
            ::core::stringify!(getOperationId),
            ::core::stringify!(delay),
            ::core::stringify!(getOperationState),
            ::core::stringify!(schedule),
            ::core::stringify!(hasRole),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(operations),
            ::core::stringify!(revokeRole),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateDelayCall as alloy_sol_types::SolCall>::SIGNATURE,
            <executeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getOperationIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <delayCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getOperationStateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scheduleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <operationsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ClimberTimelockCalls {
        const NAME: &'static str = "ClimberTimelockCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 14usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delay(_) => <delayCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getOperationId(_) => {
                    <getOperationIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperationState(_) => {
                    <getOperationStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::operations(_) => {
                    <operationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::schedule(_) => <scheduleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateDelay(_) => {
                    <updateDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ClimberTimelockCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn updateDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <updateDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::updateDelay)
                    }
                    updateDelay
                },
                {
                    fn execute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ClimberTimelockCalls::execute)
                    }
                    execute
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ClimberTimelockCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getOperationId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <getOperationIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::getOperationId)
                    }
                    getOperationId
                },
                {
                    fn delay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <delayCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ClimberTimelockCalls::delay)
                    }
                    delay
                },
                {
                    fn getOperationState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <getOperationStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::getOperationState)
                    }
                    getOperationState
                },
                {
                    fn schedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <scheduleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ClimberTimelockCalls::schedule)
                    }
                    schedule
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ClimberTimelockCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn operations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <operationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::operations)
                    }
                    operations
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockCalls::revokeRole)
                    }
                    revokeRole
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ClimberTimelockCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn updateDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <updateDelayCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::updateDelay)
                    }
                    updateDelay
                },
                {
                    fn execute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::execute)
                    }
                    execute
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getOperationId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <getOperationIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::getOperationId)
                    }
                    getOperationId
                },
                {
                    fn delay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <delayCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::delay)
                    }
                    delay
                },
                {
                    fn getOperationState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <getOperationStateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::getOperationState)
                    }
                    getOperationState
                },
                {
                    fn schedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <scheduleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::schedule)
                    }
                    schedule
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn operations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <operationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::operations)
                    }
                    operations
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockCalls::revokeRole)
                    }
                    revokeRole
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delay(inner) => {
                    <delayCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getOperationId(inner) => {
                    <getOperationIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperationState(inner) => {
                    <getOperationStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::operations(inner) => {
                    <operationsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::schedule(inner) => {
                    <scheduleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateDelay(inner) => {
                    <updateDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delay(inner) => {
                    <delayCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getOperationId(inner) => {
                    <getOperationIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperationState(inner) => {
                    <getOperationStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::operations(inner) => {
                    <operationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::schedule(inner) => {
                    <scheduleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateDelay(inner) => {
                    <updateDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ClimberTimelock`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ClimberTimelockErrors {
        #[allow(missing_docs)]
        CallerNotTimelock(CallerNotTimelock),
        #[allow(missing_docs)]
        InvalidDataElementsCount(InvalidDataElementsCount),
        #[allow(missing_docs)]
        InvalidTargetsCount(InvalidTargetsCount),
        #[allow(missing_docs)]
        InvalidValuesCount(InvalidValuesCount),
        #[allow(missing_docs)]
        NewDelayAboveMax(NewDelayAboveMax),
        #[allow(missing_docs)]
        NotReadyForExecution(NotReadyForExecution),
        #[allow(missing_docs)]
        OperationAlreadyKnown(OperationAlreadyKnown),
    }
    impl ClimberTimelockErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [46u8, 44u8, 96u8, 254u8],
            [65u8, 74u8, 254u8, 72u8],
            [65u8, 99u8, 51u8, 162u8],
            [87u8, 100u8, 5u8, 163u8],
            [118u8, 206u8, 251u8, 203u8],
            [120u8, 244u8, 36u8, 196u8],
            [223u8, 180u8, 158u8, 49u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(InvalidValuesCount),
            ::core::stringify!(NotReadyForExecution),
            ::core::stringify!(OperationAlreadyKnown),
            ::core::stringify!(InvalidTargetsCount),
            ::core::stringify!(InvalidDataElementsCount),
            ::core::stringify!(NewDelayAboveMax),
            ::core::stringify!(CallerNotTimelock),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <InvalidValuesCount as alloy_sol_types::SolError>::SIGNATURE,
            <NotReadyForExecution as alloy_sol_types::SolError>::SIGNATURE,
            <OperationAlreadyKnown as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTargetsCount as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDataElementsCount as alloy_sol_types::SolError>::SIGNATURE,
            <NewDelayAboveMax as alloy_sol_types::SolError>::SIGNATURE,
            <CallerNotTimelock as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ClimberTimelockErrors {
        const NAME: &'static str = "ClimberTimelockErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 7usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CallerNotTimelock(_) => {
                    <CallerNotTimelock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDataElementsCount(_) => {
                    <InvalidDataElementsCount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTargetsCount(_) => {
                    <InvalidTargetsCount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidValuesCount(_) => {
                    <InvalidValuesCount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NewDelayAboveMax(_) => {
                    <NewDelayAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotReadyForExecution(_) => {
                    <NotReadyForExecution as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperationAlreadyKnown(_) => {
                    <OperationAlreadyKnown as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ClimberTimelockErrors>] = &[
                {
                    fn InvalidValuesCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <InvalidValuesCount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockErrors::InvalidValuesCount)
                    }
                    InvalidValuesCount
                },
                {
                    fn NotReadyForExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <NotReadyForExecution as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockErrors::NotReadyForExecution)
                    }
                    NotReadyForExecution
                },
                {
                    fn OperationAlreadyKnown(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <OperationAlreadyKnown as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockErrors::OperationAlreadyKnown)
                    }
                    OperationAlreadyKnown
                },
                {
                    fn InvalidTargetsCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <InvalidTargetsCount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockErrors::InvalidTargetsCount)
                    }
                    InvalidTargetsCount
                },
                {
                    fn InvalidDataElementsCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <InvalidDataElementsCount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockErrors::InvalidDataElementsCount)
                    }
                    InvalidDataElementsCount
                },
                {
                    fn NewDelayAboveMax(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <NewDelayAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockErrors::NewDelayAboveMax)
                    }
                    NewDelayAboveMax
                },
                {
                    fn CallerNotTimelock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <CallerNotTimelock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ClimberTimelockErrors::CallerNotTimelock)
                    }
                    CallerNotTimelock
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ClimberTimelockErrors>] = &[
                {
                    fn InvalidValuesCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <InvalidValuesCount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockErrors::InvalidValuesCount)
                    }
                    InvalidValuesCount
                },
                {
                    fn NotReadyForExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <NotReadyForExecution as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockErrors::NotReadyForExecution)
                    }
                    NotReadyForExecution
                },
                {
                    fn OperationAlreadyKnown(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <OperationAlreadyKnown as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockErrors::OperationAlreadyKnown)
                    }
                    OperationAlreadyKnown
                },
                {
                    fn InvalidTargetsCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <InvalidTargetsCount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockErrors::InvalidTargetsCount)
                    }
                    InvalidTargetsCount
                },
                {
                    fn InvalidDataElementsCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <InvalidDataElementsCount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockErrors::InvalidDataElementsCount)
                    }
                    InvalidDataElementsCount
                },
                {
                    fn NewDelayAboveMax(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <NewDelayAboveMax as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockErrors::NewDelayAboveMax)
                    }
                    NewDelayAboveMax
                },
                {
                    fn CallerNotTimelock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ClimberTimelockErrors> {
                        <CallerNotTimelock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ClimberTimelockErrors::CallerNotTimelock)
                    }
                    CallerNotTimelock
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::CallerNotTimelock(inner) => {
                    <CallerNotTimelock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidDataElementsCount(inner) => {
                    <InvalidDataElementsCount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTargetsCount(inner) => {
                    <InvalidTargetsCount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidValuesCount(inner) => {
                    <InvalidValuesCount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NewDelayAboveMax(inner) => {
                    <NewDelayAboveMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotReadyForExecution(inner) => {
                    <NotReadyForExecution as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperationAlreadyKnown(inner) => {
                    <OperationAlreadyKnown as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CallerNotTimelock(inner) => {
                    <CallerNotTimelock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDataElementsCount(inner) => {
                    <InvalidDataElementsCount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTargetsCount(inner) => {
                    <InvalidTargetsCount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidValuesCount(inner) => {
                    <InvalidValuesCount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NewDelayAboveMax(inner) => {
                    <NewDelayAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotReadyForExecution(inner) => {
                    <NotReadyForExecution as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperationAlreadyKnown(inner) => {
                    <OperationAlreadyKnown as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ClimberTimelock`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ClimberTimelockEvents {
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
    }
    impl ClimberTimelockEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(RoleGranted),
            ::core::stringify!(RoleAdminChanged),
            ::core::stringify!(RoleRevoked),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ClimberTimelockEvents {
        const NAME: &'static str = "ClimberTimelockEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleRevoked)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for ClimberTimelockEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ClimberTimelock`](self) contract instance.

See the [wrapper's documentation](`ClimberTimelockInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ClimberTimelockInstance<P, N> {
        ClimberTimelockInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        admin: alloy::sol_types::private::Address,
        proposer: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ClimberTimelockInstance<P, N>>,
    > {
        ClimberTimelockInstance::<P, N>::deploy(__provider, admin, proposer)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        admin: alloy::sol_types::private::Address,
        proposer: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        ClimberTimelockInstance::<P, N>::deploy_builder(__provider, admin, proposer)
    }
    /**A [`ClimberTimelock`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ClimberTimelock`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ClimberTimelockInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ClimberTimelockInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ClimberTimelockInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ClimberTimelockInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ClimberTimelock`](self) contract instance.

See the [wrapper's documentation](`ClimberTimelockInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            admin: alloy::sol_types::private::Address,
            proposer: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ClimberTimelockInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, admin, proposer);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            admin: alloy::sol_types::private::Address,
            proposer: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { admin, proposer },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> ClimberTimelockInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ClimberTimelockInstance<P, N> {
            ClimberTimelockInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ClimberTimelockInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`delay`] function.
        pub fn delay(&self) -> alloy_contract::SolCallBuilder<&P, delayCall, N> {
            self.call_builder(&delayCall)
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            dataElements: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, executeCall, N> {
            self.call_builder(
                &executeCall {
                    targets,
                    values,
                    dataElements,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`getOperationId`] function.
        pub fn getOperationId(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            dataElements: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getOperationIdCall, N> {
            self.call_builder(
                &getOperationIdCall {
                    targets,
                    values,
                    dataElements,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`getOperationState`] function.
        pub fn getOperationState(
            &self,
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getOperationStateCall, N> {
            self.call_builder(&getOperationStateCall { id })
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`operations`] function.
        pub fn operations(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, operationsCall, N> {
            self.call_builder(&operationsCall(_0))
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, renounceRoleCall, N> {
            self.call_builder(&renounceRoleCall { role, account })
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`schedule`] function.
        pub fn schedule(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            dataElements: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, scheduleCall, N> {
            self.call_builder(
                &scheduleCall {
                    targets,
                    values,
                    dataElements,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<&P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`updateDelay`] function.
        pub fn updateDelay(
            &self,
            newDelay: u64,
        ) -> alloy_contract::SolCallBuilder<&P, updateDelayCall, N> {
            self.call_builder(&updateDelayCall { newDelay })
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ClimberTimelockInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<&P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<&P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
    }
}
