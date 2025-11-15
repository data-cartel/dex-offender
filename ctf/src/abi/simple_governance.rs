///Module containing a contract's types and functions.
/**

```solidity
library ISimpleGovernance {
    struct GovernanceAction { uint128 value; uint64 proposedAt; uint64 executedAt; address target; bytes data; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISimpleGovernance {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct GovernanceAction { uint128 value; uint64 proposedAt; uint64 executedAt; address target; bytes data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernanceAction {
        #[allow(missing_docs)]
        pub value: u128,
        #[allow(missing_docs)]
        pub proposedAt: u64,
        #[allow(missing_docs)]
        pub executedAt: u64,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u128,
            u64,
            u64,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<GovernanceAction> for UnderlyingRustTuple<'_> {
            fn from(value: GovernanceAction) -> Self {
                (
                    value.value,
                    value.proposedAt,
                    value.executedAt,
                    value.target,
                    value.data,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GovernanceAction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    value: tuple.0,
                    proposedAt: tuple.1,
                    executedAt: tuple.2,
                    target: tuple.3,
                    data: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GovernanceAction {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GovernanceAction {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposedAt),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.executedAt),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GovernanceAction {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for GovernanceAction {
            const NAME: &'static str = "GovernanceAction";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GovernanceAction(uint128 value,uint64 proposedAt,uint64 executedAt,address target,bytes data)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.value)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.proposedAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.executedAt)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.target,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GovernanceAction {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.value)
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposedAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.executedAt,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.target,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.value,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposedAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.executedAt,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.target,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISimpleGovernance`](self) contract instance.

See the [wrapper's documentation](`ISimpleGovernanceInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ISimpleGovernanceInstance<P, N> {
        ISimpleGovernanceInstance::<P, N>::new(address, __provider)
    }
    /**A [`ISimpleGovernance`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISimpleGovernance`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISimpleGovernanceInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ISimpleGovernanceInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISimpleGovernanceInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ISimpleGovernanceInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ISimpleGovernance`](self) contract instance.

See the [wrapper's documentation](`ISimpleGovernanceInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> ISimpleGovernanceInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISimpleGovernanceInstance<P, N> {
            ISimpleGovernanceInstance {
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
    > ISimpleGovernanceInstance<P, N> {
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
    > ISimpleGovernanceInstance<P, N> {
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
library ISimpleGovernance {
    struct GovernanceAction {
        uint128 value;
        uint64 proposedAt;
        uint64 executedAt;
        address target;
        bytes data;
    }
}

interface SimpleGovernance {
    error ActionFailed(uint256 actionId);
    error CannotExecute(uint256 actionId);
    error InvalidTarget();
    error NotEnoughVotes(address who);
    error TargetMustHaveCode();

    event ActionExecuted(uint256 actionId, address indexed caller);
    event ActionQueued(uint256 actionId, address indexed caller);

    constructor(address governanceToken);

    function executeAction(uint256 actionId) external payable returns (bytes memory);
    function getAction(uint256 actionId) external view returns (ISimpleGovernance.GovernanceAction memory);
    function getActionCounter() external view returns (uint256);
    function getActionDelay() external pure returns (uint256);
    function getGovernanceToken() external view returns (address);
    function queueAction(address target, uint128 value, bytes memory data) external returns (uint256 actionId);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "governanceToken",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "executeAction",
    "inputs": [
      {
        "name": "actionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getAction",
    "inputs": [
      {
        "name": "actionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ISimpleGovernance.GovernanceAction",
        "components": [
          {
            "name": "value",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "proposedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "executedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "target",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getActionCounter",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getActionDelay",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getGovernanceToken",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "queueAction",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "actionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "ActionExecuted",
    "inputs": [
      {
        "name": "actionId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "caller",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ActionQueued",
    "inputs": [
      {
        "name": "actionId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "caller",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "ActionFailed",
    "inputs": [
      {
        "name": "actionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CannotExecute",
    "inputs": [
      {
        "name": "actionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidTarget",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotEnoughVotes",
    "inputs": [
      {
        "name": "who",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "TargetMustHaveCode",
    "inputs": []
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
pub mod SimpleGovernance {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50604051611599380380611599833981810160405281019061003191906100db565b805f5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001808190555050610106565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6100aa82610081565b9050919050565b6100ba816100a0565b81146100c4575f5ffd5b50565b5f815190506100d5816100b1565b92915050565b5f602082840312156100f0576100ef61007d565b5b5f6100fd848285016100c7565b91505092915050565b611486806101135f395ff3fe608060405260043610610054575f3560e01c806312057a14146100585780633f8a037d1461008257806352ecb90a146100ac5780639aca08d4146100e8578063b6e7687314610112578063c0c1cf551461014e575b5f5ffd5b348015610063575f5ffd5b5061006c61017e565b6040516100799190610c61565b60405180910390f35b34801561008d575f5ffd5b50610096610188565b6040516100a39190610cb9565b60405180910390f35b3480156100b7575f5ffd5b506100d260048036038101906100cd9190610daa565b6101af565b6040516100df9190610c61565b60405180910390f35b3480156100f3575f5ffd5b506100fc6104d8565b6040516101099190610c61565b60405180910390f35b34801561011d575f5ffd5b5061013860048036038101906101339190610e45565b6104e1565b6040516101459190610f93565b60405180910390f35b61016860048036038101906101639190610e45565b61069b565b6040516101759190610ffb565b60405180910390f35b5f6202a300905090565b5f5f5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6101b933610893565b6101fa57336040517ffb124aea0000000000000000000000000000000000000000000000000000000081526004016101f19190610cb9565b60405180910390fd5b3073ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff160361025f576040517f82d5d76a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8383905011801561028757505f8573ffffffffffffffffffffffffffffffffffffffff163b145b156102be576040517f6dd4aa6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60015490506040518060a00160405280856fffffffffffffffffffffffffffffffff1681526020014267ffffffffffffffff1681526020015f67ffffffffffffffff1681526020018673ffffffffffffffffffffffffffffffffffffffff16815260200184848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525060025f8381526020019081526020015f205f820151815f015f6101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055506020820151815f0160106101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506040820151815f0160186101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506060820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550608082015181600201908161046d9190611245565b5090505060015f81548092919060010191905055503373ffffffffffffffffffffffffffffffffffffffff167f4dfd92f69e02f82c8f6705b2e4a364465b588fa4773cda26786c3ca8df43a195826040516104c89190610c61565b60405180910390a2949350505050565b5f600154905090565b6104e9610be2565b60025f8381526020019081526020015f206040518060a00160405290815f82015f9054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020015f820160109054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160189054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461061490611075565b80601f016020809104026020016040519081016040528092919081815260200182805461064090611075565b801561068b5780601f106106625761010080835404028352916020019161068b565b820191905f5260205f20905b81548152906001019060200180831161066e57829003601f168201915b5050505050815250509050919050565b60606106a6826109d7565b6106e757816040517fb452faaf0000000000000000000000000000000000000000000000000000000081526004016106de9190610c61565b60405180910390fd5b5f60025f8481526020019081526020015f20905042815f0160186101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055503373ffffffffffffffffffffffffffffffffffffffff167f78a7d64f64ef6891bedfc1b6854447113cca07ace8747b8a6cf32a0fd5a80b178460405161076b9190610c61565b60405180910390a25f5f826001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16835f015f9054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16846002016040516107f1919061139e565b5f6040518083038185875af1925050503d805f811461082b576040519150601f19603f3d011682016040523d82523d5f602084013e610830565b606091505b509150915081610888575f8151111561084b57805181602001fd5b846040517fa6a7dbbd00000000000000000000000000000000000000000000000000000000815260040161087f9190610c61565b60405180910390fd5b809350505050919050565b5f5f5f5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16634591164e846040518263ffffffff1660e01b81526004016108ee9190610cb9565b602060405180830381865afa158015610909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061092d91906113c8565b90505f60025f5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a3ec73fb6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561099b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109bf91906113c8565b6109c99190611420565b905080821192505050919050565b5f5f60025f8481526020019081526020015f206040518060a00160405290815f82015f9054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020015f820160109054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160189054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054610b0490611075565b80601f0160208091040260200160405190810160405280929190818152602001828054610b3090611075565b8015610b7b5780601f10610b5257610100808354040283529160200191610b7b565b820191905f5260205f20905b815481529060010190602001808311610b5e57829003601f168201915b50505050508152505090505f816020015167ffffffffffffffff1603610ba4575f915050610bdd565b5f8160200151420390505f826040015167ffffffffffffffff16148015610bd857506202a3008167ffffffffffffffff1610155b925050505b919050565b6040518060a001604052805f6fffffffffffffffffffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff168152602001606081525090565b5f819050919050565b610c5b81610c49565b82525050565b5f602082019050610c745f830184610c52565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ca382610c7a565b9050919050565b610cb381610c99565b82525050565b5f602082019050610ccc5f830184610caa565b92915050565b5f5ffd5b5f5ffd5b610ce381610c99565b8114610ced575f5ffd5b50565b5f81359050610cfe81610cda565b92915050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b610d2881610d04565b8114610d32575f5ffd5b50565b5f81359050610d4381610d1f565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112610d6a57610d69610d49565b5b8235905067ffffffffffffffff811115610d8757610d86610d4d565b5b602083019150836001820283011115610da357610da2610d51565b5b9250929050565b5f5f5f5f60608587031215610dc257610dc1610cd2565b5b5f610dcf87828801610cf0565b9450506020610de087828801610d35565b935050604085013567ffffffffffffffff811115610e0157610e00610cd6565b5b610e0d87828801610d55565b925092505092959194509250565b610e2481610c49565b8114610e2e575f5ffd5b50565b5f81359050610e3f81610e1b565b92915050565b5f60208284031215610e5a57610e59610cd2565b5b5f610e6784828501610e31565b91505092915050565b610e7981610d04565b82525050565b5f67ffffffffffffffff82169050919050565b610e9b81610e7f565b82525050565b610eaa81610c99565b82525050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f610ef282610eb0565b610efc8185610eba565b9350610f0c818560208601610eca565b610f1581610ed8565b840191505092915050565b5f60a083015f830151610f355f860182610e70565b506020830151610f486020860182610e92565b506040830151610f5b6040860182610e92565b506060830151610f6e6060860182610ea1565b5060808301518482036080860152610f868282610ee8565b9150508091505092915050565b5f6020820190508181035f830152610fab8184610f20565b905092915050565b5f82825260208201905092915050565b5f610fcd82610eb0565b610fd78185610fb3565b9350610fe7818560208601610eca565b610ff081610ed8565b840191505092915050565b5f6020820190508181035f8301526110138184610fc3565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061108c57607f821691505b60208210810361109f5761109e611048565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026111017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826110c6565b61110b86836110c6565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61114661114161113c84610c49565b611123565b610c49565b9050919050565b5f819050919050565b61115f8361112c565b61117361116b8261114d565b8484546110d2565b825550505050565b5f5f905090565b61118a61117b565b611195818484611156565b505050565b5b818110156111b8576111ad5f82611182565b60018101905061119b565b5050565b601f8211156111fd576111ce816110a5565b6111d7846110b7565b810160208510156111e6578190505b6111fa6111f2856110b7565b83018261119a565b50505b505050565b5f82821c905092915050565b5f61121d5f1984600802611202565b1980831691505092915050565b5f611235838361120e565b9150826002028217905092915050565b61124e82610eb0565b67ffffffffffffffff8111156112675761126661101b565b5b6112718254611075565b61127c8282856111bc565b5f60209050601f8311600181146112ad575f841561129b578287015190505b6112a5858261122a565b86555061130c565b601f1984166112bb866110a5565b5f5b828110156112e2578489015182556001820191506020850194506020810190506112bd565b868310156112ff57848901516112fb601f89168261120e565b8355505b6001600288020188555050505b505050505050565b5f81905092915050565b5f815461132a81611075565b6113348186611314565b9450600182165f811461134e576001811461136357611395565b60ff1983168652811515820286019350611395565b61136c856110a5565b5f5b8381101561138d5781548189015260018201915060208101905061136e565b838801955050505b50505092915050565b5f6113a9828461131e565b915081905092915050565b5f815190506113c281610e1b565b92915050565b5f602082840312156113dd576113dc610cd2565b5b5f6113ea848285016113b4565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61142a82610c49565b915061143583610c49565b925082611445576114446113f3565b5b82820490509291505056fea26469706673582212206ed0745a9b4a5479c7811e29813bdd8fd42c948e64a1234a5c802441aac7e22864736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x15\x998\x03\x80a\x15\x99\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\0\xDBV[\x80__a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x80\x81\x90UPPa\x01\x06V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\0\xAA\x82a\0\x81V[\x90P\x91\x90PV[a\0\xBA\x81a\0\xA0V[\x81\x14a\0\xC4W__\xFD[PV[_\x81Q\x90Pa\0\xD5\x81a\0\xB1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\0\xF0Wa\0\xEFa\0}V[[_a\0\xFD\x84\x82\x85\x01a\0\xC7V[\x91PP\x92\x91PPV[a\x14\x86\x80a\x01\x13_9_\xF3\xFE`\x80`@R`\x046\x10a\0TW_5`\xE0\x1C\x80c\x12\x05z\x14\x14a\0XW\x80c?\x8A\x03}\x14a\0\x82W\x80cR\xEC\xB9\n\x14a\0\xACW\x80c\x9A\xCA\x08\xD4\x14a\0\xE8W\x80c\xB6\xE7hs\x14a\x01\x12W\x80c\xC0\xC1\xCFU\x14a\x01NW[__\xFD[4\x80\x15a\0cW__\xFD[Pa\0la\x01~V[`@Qa\0y\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x8DW__\xFD[Pa\0\x96a\x01\x88V[`@Qa\0\xA3\x91\x90a\x0C\xB9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xB7W__\xFD[Pa\0\xD2`\x04\x806\x03\x81\x01\x90a\0\xCD\x91\x90a\r\xAAV[a\x01\xAFV[`@Qa\0\xDF\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xF3W__\xFD[Pa\0\xFCa\x04\xD8V[`@Qa\x01\t\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x1DW__\xFD[Pa\x018`\x04\x806\x03\x81\x01\x90a\x013\x91\x90a\x0EEV[a\x04\xE1V[`@Qa\x01E\x91\x90a\x0F\x93V[`@Q\x80\x91\x03\x90\xF3[a\x01h`\x04\x806\x03\x81\x01\x90a\x01c\x91\x90a\x0EEV[a\x06\x9BV[`@Qa\x01u\x91\x90a\x0F\xFBV[`@Q\x80\x91\x03\x90\xF3[_b\x02\xA3\0\x90P\x90V[___\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x01\xB93a\x08\x93V[a\x01\xFAW3`@Q\x7F\xFB\x12J\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xF1\x91\x90a\x0C\xB9V[`@Q\x80\x91\x03\x90\xFD[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x02_W`@Q\x7F\x82\xD5\xD7j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x83\x90P\x11\x80\x15a\x02\x87WP_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x02\xBEW`@Q\x7Fm\xD4\xAAe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x90P`@Q\x80`\xA0\x01`@R\x80\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x10a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x02\x01\x90\x81a\x04m\x91\x90a\x12EV[P\x90PP`\x01_\x81T\x80\x92\x91\x90`\x01\x01\x91\x90PUP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FM\xFD\x92\xF6\x9E\x02\xF8,\x8Fg\x05\xB2\xE4\xA3dF[X\x8F\xA4w<\xDA&xl<\xA8\xDFC\xA1\x95\x82`@Qa\x04\xC8\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[_`\x01T\x90P\x90V[a\x04\xE9a\x0B\xE2V[`\x02_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x06\x14\x90a\x10uV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06@\x90a\x10uV[\x80\x15a\x06\x8BW\x80`\x1F\x10a\x06bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x8BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``a\x06\xA6\x82a\t\xD7V[a\x06\xE7W\x81`@Q\x7F\xB4R\xFA\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xDE\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xFD[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ \x90PB\x81_\x01`\x18a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fx\xA7\xD6Od\xEFh\x91\xBE\xDF\xC1\xB6\x85DG\x11<\xCA\x07\xAC\xE8t{\x8Al\xF3*\x0F\xD5\xA8\x0B\x17\x84`@Qa\x07k\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xA2__\x82`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83_\x01_\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`\x02\x01`@Qa\x07\xF1\x91\x90a\x13\x9EV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x08+W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x080V[``\x91P[P\x91P\x91P\x81a\x08\x88W_\x81Q\x11\x15a\x08KW\x80Q\x81` \x01\xFD[\x84`@Q\x7F\xA6\xA7\xDB\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x7F\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xFD[\x80\x93PPPP\x91\x90PV[____\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cE\x91\x16N\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xEE\x91\x90a\x0C\xB9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t-\x91\x90a\x13\xC8V[\x90P_`\x02__\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3\xECs\xFB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xBF\x91\x90a\x13\xC8V[a\t\xC9\x91\x90a\x14 V[\x90P\x80\x82\x11\x92PPP\x91\x90PV[__`\x02_\x84\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x0B\x04\x90a\x10uV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B0\x90a\x10uV[\x80\x15a\x0B{W\x80`\x1F\x10a\x0BRWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B{V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P_\x81` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B\xA4W_\x91PPa\x0B\xDDV[_\x81` \x01QB\x03\x90P_\x82`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\x0B\xD8WPb\x02\xA3\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x92PPP[\x91\x90PV[`@Q\x80`\xA0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81RP\x90V[_\x81\x90P\x91\x90PV[a\x0C[\x81a\x0CIV[\x82RPPV[_` \x82\x01\x90Pa\x0Ct_\x83\x01\x84a\x0CRV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0C\xA3\x82a\x0CzV[\x90P\x91\x90PV[a\x0C\xB3\x81a\x0C\x99V[\x82RPPV[_` \x82\x01\x90Pa\x0C\xCC_\x83\x01\x84a\x0C\xAAV[\x92\x91PPV[__\xFD[__\xFD[a\x0C\xE3\x81a\x0C\x99V[\x81\x14a\x0C\xEDW__\xFD[PV[_\x815\x90Pa\x0C\xFE\x81a\x0C\xDAV[\x92\x91PPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\r(\x81a\r\x04V[\x81\x14a\r2W__\xFD[PV[_\x815\x90Pa\rC\x81a\r\x1FV[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a\rjWa\ria\rIV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x87Wa\r\x86a\rMV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\r\xA3Wa\r\xA2a\rQV[[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\r\xC2Wa\r\xC1a\x0C\xD2V[[_a\r\xCF\x87\x82\x88\x01a\x0C\xF0V[\x94PP` a\r\xE0\x87\x82\x88\x01a\r5V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x01Wa\x0E\0a\x0C\xD6V[[a\x0E\r\x87\x82\x88\x01a\rUV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[a\x0E$\x81a\x0CIV[\x81\x14a\x0E.W__\xFD[PV[_\x815\x90Pa\x0E?\x81a\x0E\x1BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0EZWa\x0EYa\x0C\xD2V[[_a\x0Eg\x84\x82\x85\x01a\x0E1V[\x91PP\x92\x91PPV[a\x0Ey\x81a\r\x04V[\x82RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0E\x9B\x81a\x0E\x7FV[\x82RPPV[a\x0E\xAA\x81a\x0C\x99V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a\x0E\xF2\x82a\x0E\xB0V[a\x0E\xFC\x81\x85a\x0E\xBAV[\x93Pa\x0F\x0C\x81\x85` \x86\x01a\x0E\xCAV[a\x0F\x15\x81a\x0E\xD8V[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa\x0F5_\x86\x01\x82a\x0EpV[P` \x83\x01Qa\x0FH` \x86\x01\x82a\x0E\x92V[P`@\x83\x01Qa\x0F[`@\x86\x01\x82a\x0E\x92V[P``\x83\x01Qa\x0Fn``\x86\x01\x82a\x0E\xA1V[P`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x0F\x86\x82\x82a\x0E\xE8V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F\xAB\x81\x84a\x0F V[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x0F\xCD\x82a\x0E\xB0V[a\x0F\xD7\x81\x85a\x0F\xB3V[\x93Pa\x0F\xE7\x81\x85` \x86\x01a\x0E\xCAV[a\x0F\xF0\x81a\x0E\xD8V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\x13\x81\x84a\x0F\xC3V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x10\x8CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\x9FWa\x10\x9Ea\x10HV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a\x11\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x10\xC6V[a\x11\x0B\x86\x83a\x10\xC6V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x11Fa\x11Aa\x11<\x84a\x0CIV[a\x11#V[a\x0CIV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x11_\x83a\x11,V[a\x11sa\x11k\x82a\x11MV[\x84\x84Ta\x10\xD2V[\x82UPPPPV[__\x90P\x90V[a\x11\x8Aa\x11{V[a\x11\x95\x81\x84\x84a\x11VV[PPPV[[\x81\x81\x10\x15a\x11\xB8Wa\x11\xAD_\x82a\x11\x82V[`\x01\x81\x01\x90Pa\x11\x9BV[PPV[`\x1F\x82\x11\x15a\x11\xFDWa\x11\xCE\x81a\x10\xA5V[a\x11\xD7\x84a\x10\xB7V[\x81\x01` \x85\x10\x15a\x11\xE6W\x81\x90P[a\x11\xFAa\x11\xF2\x85a\x10\xB7V[\x83\x01\x82a\x11\x9AV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a\x12\x1D_\x19\x84`\x08\x02a\x12\x02V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a\x125\x83\x83a\x12\x0EV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x12N\x82a\x0E\xB0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12gWa\x12fa\x10\x1BV[[a\x12q\x82Ta\x10uV[a\x12|\x82\x82\x85a\x11\xBCV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a\x12\xADW_\x84\x15a\x12\x9BW\x82\x87\x01Q\x90P[a\x12\xA5\x85\x82a\x12*V[\x86UPa\x13\x0CV[`\x1F\x19\x84\x16a\x12\xBB\x86a\x10\xA5V[_[\x82\x81\x10\x15a\x12\xE2W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x12\xBDV[\x86\x83\x10\x15a\x12\xFFW\x84\x89\x01Qa\x12\xFB`\x1F\x89\x16\x82a\x12\x0EV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81\x90P\x92\x91PPV[_\x81Ta\x13*\x81a\x10uV[a\x134\x81\x86a\x13\x14V[\x94P`\x01\x82\x16_\x81\x14a\x13NW`\x01\x81\x14a\x13cWa\x13\x95V[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x13\x95V[a\x13l\x85a\x10\xA5V[_[\x83\x81\x10\x15a\x13\x8DW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x13nV[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[_a\x13\xA9\x82\x84a\x13\x1EV[\x91P\x81\x90P\x92\x91PPV[_\x81Q\x90Pa\x13\xC2\x81a\x0E\x1BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\xDDWa\x13\xDCa\x0C\xD2V[[_a\x13\xEA\x84\x82\x85\x01a\x13\xB4V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x14*\x82a\x0CIV[\x91Pa\x145\x83a\x0CIV[\x92P\x82a\x14EWa\x14Da\x13\xF3V[[\x82\x82\x04\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 n\xD0tZ\x9BJTy\xC7\x81\x1E)\x81;\xDD\x8F\xD4,\x94\x8Ed\xA1#J\\\x80$A\xAA\xC7\xE2(dsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610054575f3560e01c806312057a14146100585780633f8a037d1461008257806352ecb90a146100ac5780639aca08d4146100e8578063b6e7687314610112578063c0c1cf551461014e575b5f5ffd5b348015610063575f5ffd5b5061006c61017e565b6040516100799190610c61565b60405180910390f35b34801561008d575f5ffd5b50610096610188565b6040516100a39190610cb9565b60405180910390f35b3480156100b7575f5ffd5b506100d260048036038101906100cd9190610daa565b6101af565b6040516100df9190610c61565b60405180910390f35b3480156100f3575f5ffd5b506100fc6104d8565b6040516101099190610c61565b60405180910390f35b34801561011d575f5ffd5b5061013860048036038101906101339190610e45565b6104e1565b6040516101459190610f93565b60405180910390f35b61016860048036038101906101639190610e45565b61069b565b6040516101759190610ffb565b60405180910390f35b5f6202a300905090565b5f5f5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6101b933610893565b6101fa57336040517ffb124aea0000000000000000000000000000000000000000000000000000000081526004016101f19190610cb9565b60405180910390fd5b3073ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff160361025f576040517f82d5d76a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8383905011801561028757505f8573ffffffffffffffffffffffffffffffffffffffff163b145b156102be576040517f6dd4aa6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60015490506040518060a00160405280856fffffffffffffffffffffffffffffffff1681526020014267ffffffffffffffff1681526020015f67ffffffffffffffff1681526020018673ffffffffffffffffffffffffffffffffffffffff16815260200184848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525060025f8381526020019081526020015f205f820151815f015f6101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055506020820151815f0160106101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506040820151815f0160186101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506060820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550608082015181600201908161046d9190611245565b5090505060015f81548092919060010191905055503373ffffffffffffffffffffffffffffffffffffffff167f4dfd92f69e02f82c8f6705b2e4a364465b588fa4773cda26786c3ca8df43a195826040516104c89190610c61565b60405180910390a2949350505050565b5f600154905090565b6104e9610be2565b60025f8381526020019081526020015f206040518060a00160405290815f82015f9054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020015f820160109054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160189054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461061490611075565b80601f016020809104026020016040519081016040528092919081815260200182805461064090611075565b801561068b5780601f106106625761010080835404028352916020019161068b565b820191905f5260205f20905b81548152906001019060200180831161066e57829003601f168201915b5050505050815250509050919050565b60606106a6826109d7565b6106e757816040517fb452faaf0000000000000000000000000000000000000000000000000000000081526004016106de9190610c61565b60405180910390fd5b5f60025f8481526020019081526020015f20905042815f0160186101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055503373ffffffffffffffffffffffffffffffffffffffff167f78a7d64f64ef6891bedfc1b6854447113cca07ace8747b8a6cf32a0fd5a80b178460405161076b9190610c61565b60405180910390a25f5f826001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16835f015f9054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16846002016040516107f1919061139e565b5f6040518083038185875af1925050503d805f811461082b576040519150601f19603f3d011682016040523d82523d5f602084013e610830565b606091505b509150915081610888575f8151111561084b57805181602001fd5b846040517fa6a7dbbd00000000000000000000000000000000000000000000000000000000815260040161087f9190610c61565b60405180910390fd5b809350505050919050565b5f5f5f5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16634591164e846040518263ffffffff1660e01b81526004016108ee9190610cb9565b602060405180830381865afa158015610909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061092d91906113c8565b90505f60025f5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a3ec73fb6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561099b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109bf91906113c8565b6109c99190611420565b905080821192505050919050565b5f5f60025f8481526020019081526020015f206040518060a00160405290815f82015f9054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020015f820160109054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160189054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054610b0490611075565b80601f0160208091040260200160405190810160405280929190818152602001828054610b3090611075565b8015610b7b5780601f10610b5257610100808354040283529160200191610b7b565b820191905f5260205f20905b815481529060010190602001808311610b5e57829003601f168201915b50505050508152505090505f816020015167ffffffffffffffff1603610ba4575f915050610bdd565b5f8160200151420390505f826040015167ffffffffffffffff16148015610bd857506202a3008167ffffffffffffffff1610155b925050505b919050565b6040518060a001604052805f6fffffffffffffffffffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff168152602001606081525090565b5f819050919050565b610c5b81610c49565b82525050565b5f602082019050610c745f830184610c52565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ca382610c7a565b9050919050565b610cb381610c99565b82525050565b5f602082019050610ccc5f830184610caa565b92915050565b5f5ffd5b5f5ffd5b610ce381610c99565b8114610ced575f5ffd5b50565b5f81359050610cfe81610cda565b92915050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b610d2881610d04565b8114610d32575f5ffd5b50565b5f81359050610d4381610d1f565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112610d6a57610d69610d49565b5b8235905067ffffffffffffffff811115610d8757610d86610d4d565b5b602083019150836001820283011115610da357610da2610d51565b5b9250929050565b5f5f5f5f60608587031215610dc257610dc1610cd2565b5b5f610dcf87828801610cf0565b9450506020610de087828801610d35565b935050604085013567ffffffffffffffff811115610e0157610e00610cd6565b5b610e0d87828801610d55565b925092505092959194509250565b610e2481610c49565b8114610e2e575f5ffd5b50565b5f81359050610e3f81610e1b565b92915050565b5f60208284031215610e5a57610e59610cd2565b5b5f610e6784828501610e31565b91505092915050565b610e7981610d04565b82525050565b5f67ffffffffffffffff82169050919050565b610e9b81610e7f565b82525050565b610eaa81610c99565b82525050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f610ef282610eb0565b610efc8185610eba565b9350610f0c818560208601610eca565b610f1581610ed8565b840191505092915050565b5f60a083015f830151610f355f860182610e70565b506020830151610f486020860182610e92565b506040830151610f5b6040860182610e92565b506060830151610f6e6060860182610ea1565b5060808301518482036080860152610f868282610ee8565b9150508091505092915050565b5f6020820190508181035f830152610fab8184610f20565b905092915050565b5f82825260208201905092915050565b5f610fcd82610eb0565b610fd78185610fb3565b9350610fe7818560208601610eca565b610ff081610ed8565b840191505092915050565b5f6020820190508181035f8301526110138184610fc3565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061108c57607f821691505b60208210810361109f5761109e611048565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026111017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826110c6565b61110b86836110c6565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61114661114161113c84610c49565b611123565b610c49565b9050919050565b5f819050919050565b61115f8361112c565b61117361116b8261114d565b8484546110d2565b825550505050565b5f5f905090565b61118a61117b565b611195818484611156565b505050565b5b818110156111b8576111ad5f82611182565b60018101905061119b565b5050565b601f8211156111fd576111ce816110a5565b6111d7846110b7565b810160208510156111e6578190505b6111fa6111f2856110b7565b83018261119a565b50505b505050565b5f82821c905092915050565b5f61121d5f1984600802611202565b1980831691505092915050565b5f611235838361120e565b9150826002028217905092915050565b61124e82610eb0565b67ffffffffffffffff8111156112675761126661101b565b5b6112718254611075565b61127c8282856111bc565b5f60209050601f8311600181146112ad575f841561129b578287015190505b6112a5858261122a565b86555061130c565b601f1984166112bb866110a5565b5f5b828110156112e2578489015182556001820191506020850194506020810190506112bd565b868310156112ff57848901516112fb601f89168261120e565b8355505b6001600288020188555050505b505050505050565b5f81905092915050565b5f815461132a81611075565b6113348186611314565b9450600182165f811461134e576001811461136357611395565b60ff1983168652811515820286019350611395565b61136c856110a5565b5f5b8381101561138d5781548189015260018201915060208101905061136e565b838801955050505b50505092915050565b5f6113a9828461131e565b915081905092915050565b5f815190506113c281610e1b565b92915050565b5f602082840312156113dd576113dc610cd2565b5b5f6113ea848285016113b4565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61142a82610c49565b915061143583610c49565b925082611445576114446113f3565b5b82820490509291505056fea26469706673582212206ed0745a9b4a5479c7811e29813bdd8fd42c948e64a1234a5c802441aac7e22864736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0TW_5`\xE0\x1C\x80c\x12\x05z\x14\x14a\0XW\x80c?\x8A\x03}\x14a\0\x82W\x80cR\xEC\xB9\n\x14a\0\xACW\x80c\x9A\xCA\x08\xD4\x14a\0\xE8W\x80c\xB6\xE7hs\x14a\x01\x12W\x80c\xC0\xC1\xCFU\x14a\x01NW[__\xFD[4\x80\x15a\0cW__\xFD[Pa\0la\x01~V[`@Qa\0y\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x8DW__\xFD[Pa\0\x96a\x01\x88V[`@Qa\0\xA3\x91\x90a\x0C\xB9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xB7W__\xFD[Pa\0\xD2`\x04\x806\x03\x81\x01\x90a\0\xCD\x91\x90a\r\xAAV[a\x01\xAFV[`@Qa\0\xDF\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xF3W__\xFD[Pa\0\xFCa\x04\xD8V[`@Qa\x01\t\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x1DW__\xFD[Pa\x018`\x04\x806\x03\x81\x01\x90a\x013\x91\x90a\x0EEV[a\x04\xE1V[`@Qa\x01E\x91\x90a\x0F\x93V[`@Q\x80\x91\x03\x90\xF3[a\x01h`\x04\x806\x03\x81\x01\x90a\x01c\x91\x90a\x0EEV[a\x06\x9BV[`@Qa\x01u\x91\x90a\x0F\xFBV[`@Q\x80\x91\x03\x90\xF3[_b\x02\xA3\0\x90P\x90V[___\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x01\xB93a\x08\x93V[a\x01\xFAW3`@Q\x7F\xFB\x12J\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xF1\x91\x90a\x0C\xB9V[`@Q\x80\x91\x03\x90\xFD[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x02_W`@Q\x7F\x82\xD5\xD7j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x83\x90P\x11\x80\x15a\x02\x87WP_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x02\xBEW`@Q\x7Fm\xD4\xAAe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x90P`@Q\x80`\xA0\x01`@R\x80\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x10a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x02\x01\x90\x81a\x04m\x91\x90a\x12EV[P\x90PP`\x01_\x81T\x80\x92\x91\x90`\x01\x01\x91\x90PUP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FM\xFD\x92\xF6\x9E\x02\xF8,\x8Fg\x05\xB2\xE4\xA3dF[X\x8F\xA4w<\xDA&xl<\xA8\xDFC\xA1\x95\x82`@Qa\x04\xC8\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[_`\x01T\x90P\x90V[a\x04\xE9a\x0B\xE2V[`\x02_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x06\x14\x90a\x10uV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06@\x90a\x10uV[\x80\x15a\x06\x8BW\x80`\x1F\x10a\x06bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x8BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``a\x06\xA6\x82a\t\xD7V[a\x06\xE7W\x81`@Q\x7F\xB4R\xFA\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xDE\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xFD[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ \x90PB\x81_\x01`\x18a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fx\xA7\xD6Od\xEFh\x91\xBE\xDF\xC1\xB6\x85DG\x11<\xCA\x07\xAC\xE8t{\x8Al\xF3*\x0F\xD5\xA8\x0B\x17\x84`@Qa\x07k\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xA2__\x82`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83_\x01_\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`\x02\x01`@Qa\x07\xF1\x91\x90a\x13\x9EV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x08+W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x080V[``\x91P[P\x91P\x91P\x81a\x08\x88W_\x81Q\x11\x15a\x08KW\x80Q\x81` \x01\xFD[\x84`@Q\x7F\xA6\xA7\xDB\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x7F\x91\x90a\x0CaV[`@Q\x80\x91\x03\x90\xFD[\x80\x93PPPP\x91\x90PV[____\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cE\x91\x16N\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xEE\x91\x90a\x0C\xB9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t-\x91\x90a\x13\xC8V[\x90P_`\x02__\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3\xECs\xFB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xBF\x91\x90a\x13\xC8V[a\t\xC9\x91\x90a\x14 V[\x90P\x80\x82\x11\x92PPP\x91\x90PV[__`\x02_\x84\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x0B\x04\x90a\x10uV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B0\x90a\x10uV[\x80\x15a\x0B{W\x80`\x1F\x10a\x0BRWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B{V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P_\x81` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B\xA4W_\x91PPa\x0B\xDDV[_\x81` \x01QB\x03\x90P_\x82`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\x0B\xD8WPb\x02\xA3\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x92PPP[\x91\x90PV[`@Q\x80`\xA0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81RP\x90V[_\x81\x90P\x91\x90PV[a\x0C[\x81a\x0CIV[\x82RPPV[_` \x82\x01\x90Pa\x0Ct_\x83\x01\x84a\x0CRV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0C\xA3\x82a\x0CzV[\x90P\x91\x90PV[a\x0C\xB3\x81a\x0C\x99V[\x82RPPV[_` \x82\x01\x90Pa\x0C\xCC_\x83\x01\x84a\x0C\xAAV[\x92\x91PPV[__\xFD[__\xFD[a\x0C\xE3\x81a\x0C\x99V[\x81\x14a\x0C\xEDW__\xFD[PV[_\x815\x90Pa\x0C\xFE\x81a\x0C\xDAV[\x92\x91PPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\r(\x81a\r\x04V[\x81\x14a\r2W__\xFD[PV[_\x815\x90Pa\rC\x81a\r\x1FV[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a\rjWa\ria\rIV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x87Wa\r\x86a\rMV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\r\xA3Wa\r\xA2a\rQV[[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\r\xC2Wa\r\xC1a\x0C\xD2V[[_a\r\xCF\x87\x82\x88\x01a\x0C\xF0V[\x94PP` a\r\xE0\x87\x82\x88\x01a\r5V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x01Wa\x0E\0a\x0C\xD6V[[a\x0E\r\x87\x82\x88\x01a\rUV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[a\x0E$\x81a\x0CIV[\x81\x14a\x0E.W__\xFD[PV[_\x815\x90Pa\x0E?\x81a\x0E\x1BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0EZWa\x0EYa\x0C\xD2V[[_a\x0Eg\x84\x82\x85\x01a\x0E1V[\x91PP\x92\x91PPV[a\x0Ey\x81a\r\x04V[\x82RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0E\x9B\x81a\x0E\x7FV[\x82RPPV[a\x0E\xAA\x81a\x0C\x99V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a\x0E\xF2\x82a\x0E\xB0V[a\x0E\xFC\x81\x85a\x0E\xBAV[\x93Pa\x0F\x0C\x81\x85` \x86\x01a\x0E\xCAV[a\x0F\x15\x81a\x0E\xD8V[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa\x0F5_\x86\x01\x82a\x0EpV[P` \x83\x01Qa\x0FH` \x86\x01\x82a\x0E\x92V[P`@\x83\x01Qa\x0F[`@\x86\x01\x82a\x0E\x92V[P``\x83\x01Qa\x0Fn``\x86\x01\x82a\x0E\xA1V[P`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x0F\x86\x82\x82a\x0E\xE8V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F\xAB\x81\x84a\x0F V[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x0F\xCD\x82a\x0E\xB0V[a\x0F\xD7\x81\x85a\x0F\xB3V[\x93Pa\x0F\xE7\x81\x85` \x86\x01a\x0E\xCAV[a\x0F\xF0\x81a\x0E\xD8V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\x13\x81\x84a\x0F\xC3V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x10\x8CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\x9FWa\x10\x9Ea\x10HV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a\x11\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x10\xC6V[a\x11\x0B\x86\x83a\x10\xC6V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x11Fa\x11Aa\x11<\x84a\x0CIV[a\x11#V[a\x0CIV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x11_\x83a\x11,V[a\x11sa\x11k\x82a\x11MV[\x84\x84Ta\x10\xD2V[\x82UPPPPV[__\x90P\x90V[a\x11\x8Aa\x11{V[a\x11\x95\x81\x84\x84a\x11VV[PPPV[[\x81\x81\x10\x15a\x11\xB8Wa\x11\xAD_\x82a\x11\x82V[`\x01\x81\x01\x90Pa\x11\x9BV[PPV[`\x1F\x82\x11\x15a\x11\xFDWa\x11\xCE\x81a\x10\xA5V[a\x11\xD7\x84a\x10\xB7V[\x81\x01` \x85\x10\x15a\x11\xE6W\x81\x90P[a\x11\xFAa\x11\xF2\x85a\x10\xB7V[\x83\x01\x82a\x11\x9AV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a\x12\x1D_\x19\x84`\x08\x02a\x12\x02V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a\x125\x83\x83a\x12\x0EV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x12N\x82a\x0E\xB0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12gWa\x12fa\x10\x1BV[[a\x12q\x82Ta\x10uV[a\x12|\x82\x82\x85a\x11\xBCV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a\x12\xADW_\x84\x15a\x12\x9BW\x82\x87\x01Q\x90P[a\x12\xA5\x85\x82a\x12*V[\x86UPa\x13\x0CV[`\x1F\x19\x84\x16a\x12\xBB\x86a\x10\xA5V[_[\x82\x81\x10\x15a\x12\xE2W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x12\xBDV[\x86\x83\x10\x15a\x12\xFFW\x84\x89\x01Qa\x12\xFB`\x1F\x89\x16\x82a\x12\x0EV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81\x90P\x92\x91PPV[_\x81Ta\x13*\x81a\x10uV[a\x134\x81\x86a\x13\x14V[\x94P`\x01\x82\x16_\x81\x14a\x13NW`\x01\x81\x14a\x13cWa\x13\x95V[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x13\x95V[a\x13l\x85a\x10\xA5V[_[\x83\x81\x10\x15a\x13\x8DW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x13nV[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[_a\x13\xA9\x82\x84a\x13\x1EV[\x91P\x81\x90P\x92\x91PPV[_\x81Q\x90Pa\x13\xC2\x81a\x0E\x1BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\xDDWa\x13\xDCa\x0C\xD2V[[_a\x13\xEA\x84\x82\x85\x01a\x13\xB4V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x14*\x82a\x0CIV[\x91Pa\x145\x83a\x0CIV[\x92P\x82a\x14EWa\x14Da\x13\xF3V[[\x82\x82\x04\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 n\xD0tZ\x9BJTy\xC7\x81\x1E)\x81;\xDD\x8F\xD4,\x94\x8Ed\xA1#J\\\x80$A\xAA\xC7\xE2(dsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ActionFailed(uint256)` and selector `0xa6a7dbbd`.
```solidity
error ActionFailed(uint256 actionId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ActionFailed {
        #[allow(missing_docs)]
        pub actionId: alloy::sol_types::private::primitives::aliases::U256,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ActionFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ActionFailed) -> Self {
                (value.actionId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ActionFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { actionId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ActionFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ActionFailed(uint256)";
            const SELECTOR: [u8; 4] = [166u8, 167u8, 219u8, 189u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionId),
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
    /**Custom error with signature `CannotExecute(uint256)` and selector `0xb452faaf`.
```solidity
error CannotExecute(uint256 actionId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotExecute {
        #[allow(missing_docs)]
        pub actionId: alloy::sol_types::private::primitives::aliases::U256,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CannotExecute> for UnderlyingRustTuple<'_> {
            fn from(value: CannotExecute) -> Self {
                (value.actionId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotExecute {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { actionId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotExecute {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotExecute(uint256)";
            const SELECTOR: [u8; 4] = [180u8, 82u8, 250u8, 175u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionId),
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
    /**Custom error with signature `InvalidTarget()` and selector `0x82d5d76a`.
```solidity
error InvalidTarget();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTarget;
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
        impl ::core::convert::From<InvalidTarget> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTarget) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTarget {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTarget {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTarget()";
            const SELECTOR: [u8; 4] = [130u8, 213u8, 215u8, 106u8];
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
    /**Custom error with signature `NotEnoughVotes(address)` and selector `0xfb124aea`.
```solidity
error NotEnoughVotes(address who);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotEnoughVotes {
        #[allow(missing_docs)]
        pub who: alloy::sol_types::private::Address,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<NotEnoughVotes> for UnderlyingRustTuple<'_> {
            fn from(value: NotEnoughVotes) -> Self {
                (value.who,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotEnoughVotes {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { who: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotEnoughVotes {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotEnoughVotes(address)";
            const SELECTOR: [u8; 4] = [251u8, 18u8, 74u8, 234u8];
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
                        &self.who,
                    ),
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
    /**Custom error with signature `TargetMustHaveCode()` and selector `0x6dd4aa65`.
```solidity
error TargetMustHaveCode();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TargetMustHaveCode;
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
        impl ::core::convert::From<TargetMustHaveCode> for UnderlyingRustTuple<'_> {
            fn from(value: TargetMustHaveCode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TargetMustHaveCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TargetMustHaveCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TargetMustHaveCode()";
            const SELECTOR: [u8; 4] = [109u8, 212u8, 170u8, 101u8];
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
    /**Event with signature `ActionExecuted(uint256,address)` and selector `0x78a7d64f64ef6891bedfc1b6854447113cca07ace8747b8a6cf32a0fd5a80b17`.
```solidity
event ActionExecuted(uint256 actionId, address indexed caller);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActionExecuted {
        #[allow(missing_docs)]
        pub actionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ActionExecuted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ActionExecuted(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                120u8, 167u8, 214u8, 79u8, 100u8, 239u8, 104u8, 145u8, 190u8, 223u8,
                193u8, 182u8, 133u8, 68u8, 71u8, 17u8, 60u8, 202u8, 7u8, 172u8, 232u8,
                116u8, 123u8, 138u8, 108u8, 243u8, 42u8, 15u8, 213u8, 168u8, 11u8, 23u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    actionId: data.0,
                    caller: topics.1,
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
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionId),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.caller.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.caller,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ActionExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActionExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActionExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ActionQueued(uint256,address)` and selector `0x4dfd92f69e02f82c8f6705b2e4a364465b588fa4773cda26786c3ca8df43a195`.
```solidity
event ActionQueued(uint256 actionId, address indexed caller);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActionQueued {
        #[allow(missing_docs)]
        pub actionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ActionQueued {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ActionQueued(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                77u8, 253u8, 146u8, 246u8, 158u8, 2u8, 248u8, 44u8, 143u8, 103u8, 5u8,
                178u8, 228u8, 163u8, 100u8, 70u8, 91u8, 88u8, 143u8, 164u8, 119u8, 60u8,
                218u8, 38u8, 120u8, 108u8, 60u8, 168u8, 223u8, 67u8, 161u8, 149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    actionId: data.0,
                    caller: topics.1,
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
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionId),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.caller.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.caller,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ActionQueued {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActionQueued> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActionQueued) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address governanceToken);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub governanceToken: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
                    (value.governanceToken,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { governanceToken: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                        &self.governanceToken,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `executeAction(uint256)` and selector `0xc0c1cf55`.
```solidity
function executeAction(uint256 actionId) external payable returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeActionCall {
        #[allow(missing_docs)]
        pub actionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`executeAction(uint256)`](executeActionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeActionReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<executeActionCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeActionCall) -> Self {
                    (value.actionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeActionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { actionId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<executeActionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeActionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeActionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeActionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeAction(uint256)";
            const SELECTOR: [u8; 4] = [192u8, 193u8, 207u8, 85u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
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
                        let r: executeActionReturn = r.into();
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
                        let r: executeActionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAction(uint256)` and selector `0xb6e76873`.
```solidity
function getAction(uint256 actionId) external view returns (ISimpleGovernance.GovernanceAction memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActionCall {
        #[allow(missing_docs)]
        pub actionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAction(uint256)`](getActionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActionReturn {
        #[allow(missing_docs)]
        pub _0: <ISimpleGovernance::GovernanceAction as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getActionCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActionCall) -> Self {
                    (value.actionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { actionId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (ISimpleGovernance::GovernanceAction,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISimpleGovernance::GovernanceAction as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getActionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getActionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ISimpleGovernance::GovernanceAction as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ISimpleGovernance::GovernanceAction,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAction(uint256)";
            const SELECTOR: [u8; 4] = [182u8, 231u8, 104u8, 115u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <ISimpleGovernance::GovernanceAction as alloy_sol_types::SolType>::tokenize(
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
                        let r: getActionReturn = r.into();
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
                        let r: getActionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getActionCounter()` and selector `0x9aca08d4`.
```solidity
function getActionCounter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActionCounterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActionCounter()`](getActionCounterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActionCounterReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getActionCounterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActionCounterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActionCounterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getActionCounterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActionCounterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActionCounterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActionCounterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActionCounter()";
            const SELECTOR: [u8; 4] = [154u8, 202u8, 8u8, 212u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getActionCounterReturn = r.into();
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
                        let r: getActionCounterReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getActionDelay()` and selector `0x12057a14`.
```solidity
function getActionDelay() external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActionDelayCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActionDelay()`](getActionDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActionDelayReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getActionDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActionDelayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActionDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getActionDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActionDelayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActionDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActionDelayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActionDelay()";
            const SELECTOR: [u8; 4] = [18u8, 5u8, 122u8, 20u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getActionDelayReturn = r.into();
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
                        let r: getActionDelayReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGovernanceToken()` and selector `0x3f8a037d`.
```solidity
function getGovernanceToken() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGovernanceTokenCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGovernanceToken()`](getGovernanceTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGovernanceTokenReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getGovernanceTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGovernanceTokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGovernanceTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<getGovernanceTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGovernanceTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGovernanceTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGovernanceTokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGovernanceToken()";
            const SELECTOR: [u8; 4] = [63u8, 138u8, 3u8, 125u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: getGovernanceTokenReturn = r.into();
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
                        let r: getGovernanceTokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `queueAction(address,uint128,bytes)` and selector `0x52ecb90a`.
```solidity
function queueAction(address target, uint128 value, bytes memory data) external returns (uint256 actionId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueActionCall {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: u128,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`queueAction(address,uint128,bytes)`](queueActionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueActionReturn {
        #[allow(missing_docs)]
        pub actionId: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u128,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<queueActionCall> for UnderlyingRustTuple<'_> {
                fn from(value: queueActionCall) -> Self {
                    (value.target, value.value, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueActionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        target: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<queueActionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: queueActionReturn) -> Self {
                    (value.actionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueActionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { actionId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueActionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queueAction(address,uint128,bytes)";
            const SELECTOR: [u8; 4] = [82u8, 236u8, 185u8, 10u8];
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
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: queueActionReturn = r.into();
                        r.actionId
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
                        let r: queueActionReturn = r.into();
                        r.actionId
                    })
            }
        }
    };
    ///Container for all the [`SimpleGovernance`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SimpleGovernanceCalls {
        #[allow(missing_docs)]
        executeAction(executeActionCall),
        #[allow(missing_docs)]
        getAction(getActionCall),
        #[allow(missing_docs)]
        getActionCounter(getActionCounterCall),
        #[allow(missing_docs)]
        getActionDelay(getActionDelayCall),
        #[allow(missing_docs)]
        getGovernanceToken(getGovernanceTokenCall),
        #[allow(missing_docs)]
        queueAction(queueActionCall),
    }
    impl SimpleGovernanceCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [18u8, 5u8, 122u8, 20u8],
            [63u8, 138u8, 3u8, 125u8],
            [82u8, 236u8, 185u8, 10u8],
            [154u8, 202u8, 8u8, 212u8],
            [182u8, 231u8, 104u8, 115u8],
            [192u8, 193u8, 207u8, 85u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(getActionDelay),
            ::core::stringify!(getGovernanceToken),
            ::core::stringify!(queueAction),
            ::core::stringify!(getActionCounter),
            ::core::stringify!(getAction),
            ::core::stringify!(executeAction),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <getActionDelayCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getGovernanceTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <queueActionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getActionCounterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getActionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <executeActionCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SimpleGovernanceCalls {
        const NAME: &'static str = "SimpleGovernanceCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::executeAction(_) => {
                    <executeActionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAction(_) => {
                    <getActionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActionCounter(_) => {
                    <getActionCounterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActionDelay(_) => {
                    <getActionDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGovernanceToken(_) => {
                    <getGovernanceTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queueAction(_) => {
                    <queueActionCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SimpleGovernanceCalls>] = &[
                {
                    fn getActionDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getActionDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceCalls::getActionDelay)
                    }
                    getActionDelay
                },
                {
                    fn getGovernanceToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getGovernanceTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceCalls::getGovernanceToken)
                    }
                    getGovernanceToken
                },
                {
                    fn queueAction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <queueActionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceCalls::queueAction)
                    }
                    queueAction
                },
                {
                    fn getActionCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getActionCounterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceCalls::getActionCounter)
                    }
                    getActionCounter
                },
                {
                    fn getAction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getActionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SimpleGovernanceCalls::getAction)
                    }
                    getAction
                },
                {
                    fn executeAction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <executeActionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceCalls::executeAction)
                    }
                    executeAction
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
            ) -> alloy_sol_types::Result<SimpleGovernanceCalls>] = &[
                {
                    fn getActionDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getActionDelayCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceCalls::getActionDelay)
                    }
                    getActionDelay
                },
                {
                    fn getGovernanceToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getGovernanceTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceCalls::getGovernanceToken)
                    }
                    getGovernanceToken
                },
                {
                    fn queueAction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <queueActionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceCalls::queueAction)
                    }
                    queueAction
                },
                {
                    fn getActionCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getActionCounterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceCalls::getActionCounter)
                    }
                    getActionCounter
                },
                {
                    fn getAction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <getActionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceCalls::getAction)
                    }
                    getAction
                },
                {
                    fn executeAction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceCalls> {
                        <executeActionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceCalls::executeAction)
                    }
                    executeAction
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
                Self::executeAction(inner) => {
                    <executeActionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAction(inner) => {
                    <getActionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getActionCounter(inner) => {
                    <getActionCounterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActionDelay(inner) => {
                    <getActionDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGovernanceToken(inner) => {
                    <getGovernanceTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queueAction(inner) => {
                    <queueActionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::executeAction(inner) => {
                    <executeActionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAction(inner) => {
                    <getActionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActionCounter(inner) => {
                    <getActionCounterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActionDelay(inner) => {
                    <getActionDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGovernanceToken(inner) => {
                    <getGovernanceTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queueAction(inner) => {
                    <queueActionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SimpleGovernance`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SimpleGovernanceErrors {
        #[allow(missing_docs)]
        ActionFailed(ActionFailed),
        #[allow(missing_docs)]
        CannotExecute(CannotExecute),
        #[allow(missing_docs)]
        InvalidTarget(InvalidTarget),
        #[allow(missing_docs)]
        NotEnoughVotes(NotEnoughVotes),
        #[allow(missing_docs)]
        TargetMustHaveCode(TargetMustHaveCode),
    }
    impl SimpleGovernanceErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [109u8, 212u8, 170u8, 101u8],
            [130u8, 213u8, 215u8, 106u8],
            [166u8, 167u8, 219u8, 189u8],
            [180u8, 82u8, 250u8, 175u8],
            [251u8, 18u8, 74u8, 234u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(TargetMustHaveCode),
            ::core::stringify!(InvalidTarget),
            ::core::stringify!(ActionFailed),
            ::core::stringify!(CannotExecute),
            ::core::stringify!(NotEnoughVotes),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <TargetMustHaveCode as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTarget as alloy_sol_types::SolError>::SIGNATURE,
            <ActionFailed as alloy_sol_types::SolError>::SIGNATURE,
            <CannotExecute as alloy_sol_types::SolError>::SIGNATURE,
            <NotEnoughVotes as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SimpleGovernanceErrors {
        const NAME: &'static str = "SimpleGovernanceErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ActionFailed(_) => {
                    <ActionFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotExecute(_) => {
                    <CannotExecute as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTarget(_) => {
                    <InvalidTarget as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotEnoughVotes(_) => {
                    <NotEnoughVotes as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TargetMustHaveCode(_) => {
                    <TargetMustHaveCode as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SimpleGovernanceErrors>] = &[
                {
                    fn TargetMustHaveCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <TargetMustHaveCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceErrors::TargetMustHaveCode)
                    }
                    TargetMustHaveCode
                },
                {
                    fn InvalidTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <InvalidTarget as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceErrors::InvalidTarget)
                    }
                    InvalidTarget
                },
                {
                    fn ActionFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <ActionFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SimpleGovernanceErrors::ActionFailed)
                    }
                    ActionFailed
                },
                {
                    fn CannotExecute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <CannotExecute as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceErrors::CannotExecute)
                    }
                    CannotExecute
                },
                {
                    fn NotEnoughVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <NotEnoughVotes as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SimpleGovernanceErrors::NotEnoughVotes)
                    }
                    NotEnoughVotes
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
            ) -> alloy_sol_types::Result<SimpleGovernanceErrors>] = &[
                {
                    fn TargetMustHaveCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <TargetMustHaveCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceErrors::TargetMustHaveCode)
                    }
                    TargetMustHaveCode
                },
                {
                    fn InvalidTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <InvalidTarget as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceErrors::InvalidTarget)
                    }
                    InvalidTarget
                },
                {
                    fn ActionFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <ActionFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceErrors::ActionFailed)
                    }
                    ActionFailed
                },
                {
                    fn CannotExecute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <CannotExecute as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceErrors::CannotExecute)
                    }
                    CannotExecute
                },
                {
                    fn NotEnoughVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SimpleGovernanceErrors> {
                        <NotEnoughVotes as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SimpleGovernanceErrors::NotEnoughVotes)
                    }
                    NotEnoughVotes
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
                Self::ActionFailed(inner) => {
                    <ActionFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CannotExecute(inner) => {
                    <CannotExecute as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidTarget(inner) => {
                    <InvalidTarget as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotEnoughVotes(inner) => {
                    <NotEnoughVotes as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TargetMustHaveCode(inner) => {
                    <TargetMustHaveCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ActionFailed(inner) => {
                    <ActionFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotExecute(inner) => {
                    <CannotExecute as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTarget(inner) => {
                    <InvalidTarget as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotEnoughVotes(inner) => {
                    <NotEnoughVotes as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TargetMustHaveCode(inner) => {
                    <TargetMustHaveCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SimpleGovernance`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SimpleGovernanceEvents {
        #[allow(missing_docs)]
        ActionExecuted(ActionExecuted),
        #[allow(missing_docs)]
        ActionQueued(ActionQueued),
    }
    impl SimpleGovernanceEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                77u8, 253u8, 146u8, 246u8, 158u8, 2u8, 248u8, 44u8, 143u8, 103u8, 5u8,
                178u8, 228u8, 163u8, 100u8, 70u8, 91u8, 88u8, 143u8, 164u8, 119u8, 60u8,
                218u8, 38u8, 120u8, 108u8, 60u8, 168u8, 223u8, 67u8, 161u8, 149u8,
            ],
            [
                120u8, 167u8, 214u8, 79u8, 100u8, 239u8, 104u8, 145u8, 190u8, 223u8,
                193u8, 182u8, 133u8, 68u8, 71u8, 17u8, 60u8, 202u8, 7u8, 172u8, 232u8,
                116u8, 123u8, 138u8, 108u8, 243u8, 42u8, 15u8, 213u8, 168u8, 11u8, 23u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ActionQueued),
            ::core::stringify!(ActionExecuted),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ActionQueued as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for SimpleGovernanceEvents {
        const NAME: &'static str = "SimpleGovernanceEvents";
        const COUNT: usize = 2usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActionExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActionExecuted)
                }
                Some(<ActionQueued as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActionQueued as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActionQueued)
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
    impl alloy_sol_types::private::IntoLogData for SimpleGovernanceEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ActionQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ActionQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SimpleGovernance`](self) contract instance.

See the [wrapper's documentation](`SimpleGovernanceInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> SimpleGovernanceInstance<P, N> {
        SimpleGovernanceInstance::<P, N>::new(address, __provider)
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
        governanceToken: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SimpleGovernanceInstance<P, N>>,
    > {
        SimpleGovernanceInstance::<P, N>::deploy(__provider, governanceToken)
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
        governanceToken: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        SimpleGovernanceInstance::<P, N>::deploy_builder(__provider, governanceToken)
    }
    /**A [`SimpleGovernance`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SimpleGovernance`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SimpleGovernanceInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SimpleGovernanceInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SimpleGovernanceInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SimpleGovernanceInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SimpleGovernance`](self) contract instance.

See the [wrapper's documentation](`SimpleGovernanceInstance`) for more details.*/
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
            governanceToken: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SimpleGovernanceInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, governanceToken);
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
            governanceToken: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { governanceToken },
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
    impl<P: ::core::clone::Clone, N> SimpleGovernanceInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SimpleGovernanceInstance<P, N> {
            SimpleGovernanceInstance {
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
    > SimpleGovernanceInstance<P, N> {
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
        ///Creates a new call builder for the [`executeAction`] function.
        pub fn executeAction(
            &self,
            actionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, executeActionCall, N> {
            self.call_builder(&executeActionCall { actionId })
        }
        ///Creates a new call builder for the [`getAction`] function.
        pub fn getAction(
            &self,
            actionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getActionCall, N> {
            self.call_builder(&getActionCall { actionId })
        }
        ///Creates a new call builder for the [`getActionCounter`] function.
        pub fn getActionCounter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActionCounterCall, N> {
            self.call_builder(&getActionCounterCall)
        }
        ///Creates a new call builder for the [`getActionDelay`] function.
        pub fn getActionDelay(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActionDelayCall, N> {
            self.call_builder(&getActionDelayCall)
        }
        ///Creates a new call builder for the [`getGovernanceToken`] function.
        pub fn getGovernanceToken(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getGovernanceTokenCall, N> {
            self.call_builder(&getGovernanceTokenCall)
        }
        ///Creates a new call builder for the [`queueAction`] function.
        pub fn queueAction(
            &self,
            target: alloy::sol_types::private::Address,
            value: u128,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, queueActionCall, N> {
            self.call_builder(
                &queueActionCall {
                    target,
                    value,
                    data,
                },
            )
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SimpleGovernanceInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ActionExecuted`] event.
        pub fn ActionExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, ActionExecuted, N> {
            self.event_filter::<ActionExecuted>()
        }
        ///Creates a new event filter for the [`ActionQueued`] event.
        pub fn ActionQueued_filter(&self) -> alloy_contract::Event<&P, ActionQueued, N> {
            self.event_filter::<ActionQueued>()
        }
    }
}
