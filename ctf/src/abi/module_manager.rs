///Module containing a contract's types and functions.
/**

```solidity
library Enum {
    type Operation is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Enum {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operation(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Operation> for u8 {
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
        impl Operation {
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
        impl From<u8> for Operation {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Operation> for u8 {
            fn from(value: Operation) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Operation {
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
        impl alloy_sol_types::EventTopic for Operation {
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
    /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, __provider: P) -> EnumInstance<P, N> {
        EnumInstance::<P, N>::new(address, __provider)
    }
    /**A [`Enum`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Enum`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EnumInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for EnumInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EnumInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > EnumInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> EnumInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EnumInstance<P, N> {
            EnumInstance {
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
    > EnumInstance<P, N> {
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
    > EnumInstance<P, N> {
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
library Enum {
    type Operation is uint8;
}

interface ModuleManager {
    event DisabledModule(address module);
    event EnabledModule(address module);
    event ExecutionFromModuleFailure(address indexed module);
    event ExecutionFromModuleSuccess(address indexed module);

    function disableModule(address prevModule, address module) external;
    function enableModule(address module) external;
    function execTransactionFromModule(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success);
    function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success, bytes memory returnData);
    function getModulesPaginated(address start, uint256 pageSize) external view returns (address[] memory array, address next);
    function isModuleEnabled(address module) external view returns (bool);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "disableModule",
    "inputs": [
      {
        "name": "prevModule",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "module",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "enableModule",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "execTransactionFromModule",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "operation",
        "type": "uint8",
        "internalType": "enum Enum.Operation"
      }
    ],
    "outputs": [
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "execTransactionFromModuleReturnData",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "operation",
        "type": "uint8",
        "internalType": "enum Enum.Operation"
      }
    ],
    "outputs": [
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "returnData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getModulesPaginated",
    "inputs": [
      {
        "name": "start",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "pageSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "array",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "next",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isModuleEnabled",
    "inputs": [
      {
        "name": "module",
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
    "type": "event",
    "name": "DisabledModule",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EnabledModule",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionFromModuleFailure",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionFromModuleSuccess",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
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
pub mod ModuleManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506114df8061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610060575f3560e01c80632d9ad53d14610064578063468721a7146100945780635229073f146100c4578063610b5925146100f5578063cc2f845214610111578063e009cfde14610142575b5f5ffd5b61007e60048036038101906100799190610d3c565b61015e565b60405161008b9190610d81565b60405180910390f35b6100ae60048036038101906100a99190610f2c565b61022a565b6040516100bb9190610d81565b60405180910390f35b6100de60048036038101906100d99190610f2c565b6103d7565b6040516100ec92919061100c565b60405180910390f35b61010f600480360381019061010a9190610d3c565b61040b565b005b61012b6004803603810190610126919061103a565b61070f565b60405161013992919061113e565b60405180910390f35b61015c6004803603810190610157919061116c565b610908565b005b5f8173ffffffffffffffffffffffffffffffffffffffff16600173ffffffffffffffffffffffffffffffffffffffff161415801561022357505f73ffffffffffffffffffffffffffffffffffffffff165f5f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614155b9050919050565b5f600173ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141580156102ef57505f73ffffffffffffffffffffffffffffffffffffffff165f5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614155b61032e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161032590611204565b60405180910390fd5b61033b858585855a610c0b565b9050801561038b573373ffffffffffffffffffffffffffffffffffffffff167f6895c13664aa4f67288b25d7a21d7aaa34916e355fb9b6fae0a139a9085becb860405160405180910390a26103cf565b3373ffffffffffffffffffffffffffffffffffffffff167facd2c8702804128fdb0db2bb49f6d127dd0181c13fd45dbfe16de0930e2bd37560405160405180910390a25b949350505050565b5f60606103e68686868661022a565b915060405160203d0181016040523d81523d5f602083013e8091505094509492505050565b610413610c61565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161415801561047c5750600173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614155b6104bb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104b29061126c565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff165f5f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610584576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161057b906112d4565b60405180910390fd5b5f5f600173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f5f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550805f5f600173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507fecdf3a3effea5783a3c4c2140e677577666428d44ed9d474a0b3a4c9943f84408160405161070491906112f2565b60405180910390a150565b60605f8267ffffffffffffffff81111561072c5761072b610de5565b5b60405190808252806020026020018201604052801561075a5781602001602082028036833780820191505090505b5091505f5f90505f5f5f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690505b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161415801561082a5750600173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614155b801561083557508482105b156108f9578084838151811061084e5761084d61130b565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250505f5f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081806108f190611365565b9250506107c1565b80925081845250509250929050565b610910610c61565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16141580156109795750600173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614155b6109b8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016109af9061126c565b60405180910390fd5b8073ffffffffffffffffffffffffffffffffffffffff165f5f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610a81576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a78906113f6565b60405180910390fd5b5f5f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f5f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f5f5f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507faab4fa2b463f581b2b32cb3b7e3b704b9ce37cc209b5fb4d77e593ace405427681604051610bff91906112f2565b60405180910390a15050565b5f600180811115610c1f57610c1e611414565b5b836001811115610c3257610c31611414565b5b03610c49575f5f8551602087018986f49050610c58565b5f5f855160208701888a87f190505b95945050505050565b3073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610ccf576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610cc69061148b565b60405180910390fd5b565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610d0b82610ce2565b9050919050565b610d1b81610d01565b8114610d25575f5ffd5b50565b5f81359050610d3681610d12565b92915050565b5f60208284031215610d5157610d50610cda565b5b5f610d5e84828501610d28565b91505092915050565b5f8115159050919050565b610d7b81610d67565b82525050565b5f602082019050610d945f830184610d72565b92915050565b5f819050919050565b610dac81610d9a565b8114610db6575f5ffd5b50565b5f81359050610dc781610da3565b92915050565b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610e1b82610dd5565b810181811067ffffffffffffffff82111715610e3a57610e39610de5565b5b80604052505050565b5f610e4c610cd1565b9050610e588282610e12565b919050565b5f67ffffffffffffffff821115610e7757610e76610de5565b5b610e8082610dd5565b9050602081019050919050565b828183375f83830152505050565b5f610ead610ea884610e5d565b610e43565b905082815260208101848484011115610ec957610ec8610dd1565b5b610ed4848285610e8d565b509392505050565b5f82601f830112610ef057610eef610dcd565b5b8135610f00848260208601610e9b565b91505092915050565b60028110610f15575f5ffd5b50565b5f81359050610f2681610f09565b92915050565b5f5f5f5f60808587031215610f4457610f43610cda565b5b5f610f5187828801610d28565b9450506020610f6287828801610db9565b935050604085013567ffffffffffffffff811115610f8357610f82610cde565b5b610f8f87828801610edc565b9250506060610fa087828801610f18565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f610fde82610fac565b610fe88185610fb6565b9350610ff8818560208601610fc6565b61100181610dd5565b840191505092915050565b5f60408201905061101f5f830185610d72565b81810360208301526110318184610fd4565b90509392505050565b5f5f604083850312156110505761104f610cda565b5b5f61105d85828601610d28565b925050602061106e85828601610db9565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6110aa81610d01565b82525050565b5f6110bb83836110a1565b60208301905092915050565b5f602082019050919050565b5f6110dd82611078565b6110e78185611082565b93506110f283611092565b805f5b8381101561112257815161110988826110b0565b9750611114836110c7565b9250506001810190506110f5565b5085935050505092915050565b61113881610d01565b82525050565b5f6040820190508181035f83015261115681856110d3565b9050611165602083018461112f565b9392505050565b5f5f6040838503121561118257611181610cda565b5b5f61118f85828601610d28565b92505060206111a085828601610d28565b9150509250929050565b5f82825260208201905092915050565b7f47533130340000000000000000000000000000000000000000000000000000005f82015250565b5f6111ee6005836111aa565b91506111f9826111ba565b602082019050919050565b5f6020820190508181035f83015261121b816111e2565b9050919050565b7f47533130310000000000000000000000000000000000000000000000000000005f82015250565b5f6112566005836111aa565b915061126182611222565b602082019050919050565b5f6020820190508181035f8301526112838161124a565b9050919050565b7f47533130320000000000000000000000000000000000000000000000000000005f82015250565b5f6112be6005836111aa565b91506112c98261128a565b602082019050919050565b5f6020820190508181035f8301526112eb816112b2565b9050919050565b5f6020820190506113055f83018461112f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61136f82610d9a565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036113a1576113a0611338565b5b600182019050919050565b7f47533130330000000000000000000000000000000000000000000000000000005f82015250565b5f6113e06005836111aa565b91506113eb826113ac565b602082019050919050565b5f6020820190508181035f83015261140d816113d4565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b7f47533033310000000000000000000000000000000000000000000000000000005f82015250565b5f6114756005836111aa565b915061148082611441565b602082019050919050565b5f6020820190508181035f8301526114a281611469565b905091905056fea2646970667358221220a9aa1cab9898203f872d317471961cdb4a87f3847aa848c14fabf1ac3ca303c264736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x14\xDF\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0`W_5`\xE0\x1C\x80c-\x9A\xD5=\x14a\0dW\x80cF\x87!\xA7\x14a\0\x94W\x80cR)\x07?\x14a\0\xC4W\x80ca\x0BY%\x14a\0\xF5W\x80c\xCC/\x84R\x14a\x01\x11W\x80c\xE0\t\xCF\xDE\x14a\x01BW[__\xFD[a\0~`\x04\x806\x03\x81\x01\x90a\0y\x91\x90a\r<V[a\x01^V[`@Qa\0\x8B\x91\x90a\r\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xAE`\x04\x806\x03\x81\x01\x90a\0\xA9\x91\x90a\x0F,V[a\x02*V[`@Qa\0\xBB\x91\x90a\r\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xDE`\x04\x806\x03\x81\x01\x90a\0\xD9\x91\x90a\x0F,V[a\x03\xD7V[`@Qa\0\xEC\x92\x91\x90a\x10\x0CV[`@Q\x80\x91\x03\x90\xF3[a\x01\x0F`\x04\x806\x03\x81\x01\x90a\x01\n\x91\x90a\r<V[a\x04\x0BV[\0[a\x01+`\x04\x806\x03\x81\x01\x90a\x01&\x91\x90a\x10:V[a\x07\x0FV[`@Qa\x019\x92\x91\x90a\x11>V[`@Q\x80\x91\x03\x90\xF3[a\x01\\`\x04\x806\x03\x81\x01\x90a\x01W\x91\x90a\x11lV[a\t\x08V[\0[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x02#WP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x90P\x91\x90PV[_`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x02\xEFWP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[a\x03.W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03%\x90a\x12\x04V[`@Q\x80\x91\x03\x90\xFD[a\x03;\x85\x85\x85\x85Za\x0C\x0BV[\x90P\x80\x15a\x03\x8BW3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8`@Q`@Q\x80\x91\x03\x90\xA2a\x03\xCFV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u`@Q`@Q\x80\x91\x03\x90\xA2[\x94\x93PPPPV[_``a\x03\xE6\x86\x86\x86\x86a\x02*V[\x91P`@Q` =\x01\x81\x01`@R=\x81R=_` \x83\x01>\x80\x91PP\x94P\x94\x92PPPV[a\x04\x13a\x0CaV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x04|WP`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[a\x04\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB2\x90a\x12lV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05{\x90a\x12\xD4V[`@Q\x80\x91\x03\x90\xFD[__`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80__`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x81`@Qa\x07\x04\x91\x90a\x12\xF2V[`@Q\x80\x91\x03\x90\xA1PV[``_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07,Wa\x07+a\r\xE5V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07ZW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P__\x90P___\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x08*WP`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x80\x15a\x085WP\x84\x82\x10[\x15a\x08\xF9W\x80\x84\x83\x81Q\x81\x10a\x08NWa\x08Ma\x13\x0BV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81\x80a\x08\xF1\x90a\x13eV[\x92PPa\x07\xC1V[\x80\x92P\x81\x84RPP\x92P\x92\x90PV[a\t\x10a\x0CaV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\tyWP`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[a\t\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xAF\x90a\x12lV[`@Q\x80\x91\x03\x90\xFD[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nx\x90a\x13\xF6V[`@Q\x80\x91\x03\x90\xFD[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP___\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x81`@Qa\x0B\xFF\x91\x90a\x12\xF2V[`@Q\x80\x91\x03\x90\xA1PPV[_`\x01\x80\x81\x11\x15a\x0C\x1FWa\x0C\x1Ea\x14\x14V[[\x83`\x01\x81\x11\x15a\x0C2Wa\x0C1a\x14\x14V[[\x03a\x0CIW__\x85Q` \x87\x01\x89\x86\xF4\x90Pa\x0CXV[__\x85Q` \x87\x01\x88\x8A\x87\xF1\x90P[\x95\x94PPPPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xC6\x90a\x14\x8BV[`@Q\x80\x91\x03\x90\xFD[V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\r\x0B\x82a\x0C\xE2V[\x90P\x91\x90PV[a\r\x1B\x81a\r\x01V[\x81\x14a\r%W__\xFD[PV[_\x815\x90Pa\r6\x81a\r\x12V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\rQWa\rPa\x0C\xDAV[[_a\r^\x84\x82\x85\x01a\r(V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\r{\x81a\rgV[\x82RPPV[_` \x82\x01\x90Pa\r\x94_\x83\x01\x84a\rrV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\xAC\x81a\r\x9AV[\x81\x14a\r\xB6W__\xFD[PV[_\x815\x90Pa\r\xC7\x81a\r\xA3V[\x92\x91PPV[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0E\x1B\x82a\r\xD5V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E:Wa\x0E9a\r\xE5V[[\x80`@RPPPV[_a\x0ELa\x0C\xD1V[\x90Pa\x0EX\x82\x82a\x0E\x12V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0EwWa\x0Eva\r\xE5V[[a\x0E\x80\x82a\r\xD5V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0E\xADa\x0E\xA8\x84a\x0E]V[a\x0ECV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0E\xC9Wa\x0E\xC8a\r\xD1V[[a\x0E\xD4\x84\x82\x85a\x0E\x8DV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0E\xF0Wa\x0E\xEFa\r\xCDV[[\x815a\x0F\0\x84\x82` \x86\x01a\x0E\x9BV[\x91PP\x92\x91PPV[`\x02\x81\x10a\x0F\x15W__\xFD[PV[_\x815\x90Pa\x0F&\x81a\x0F\tV[\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x0FDWa\x0FCa\x0C\xDAV[[_a\x0FQ\x87\x82\x88\x01a\r(V[\x94PP` a\x0Fb\x87\x82\x88\x01a\r\xB9V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x83Wa\x0F\x82a\x0C\xDEV[[a\x0F\x8F\x87\x82\x88\x01a\x0E\xDCV[\x92PP``a\x0F\xA0\x87\x82\x88\x01a\x0F\x18V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0F\xDE\x82a\x0F\xACV[a\x0F\xE8\x81\x85a\x0F\xB6V[\x93Pa\x0F\xF8\x81\x85` \x86\x01a\x0F\xC6V[a\x10\x01\x81a\r\xD5V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\x10\x1F_\x83\x01\x85a\rrV[\x81\x81\x03` \x83\x01Ra\x101\x81\x84a\x0F\xD4V[\x90P\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x10PWa\x10Oa\x0C\xDAV[[_a\x10]\x85\x82\x86\x01a\r(V[\x92PP` a\x10n\x85\x82\x86\x01a\r\xB9V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x10\xAA\x81a\r\x01V[\x82RPPV[_a\x10\xBB\x83\x83a\x10\xA1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x10\xDD\x82a\x10xV[a\x10\xE7\x81\x85a\x10\x82V[\x93Pa\x10\xF2\x83a\x10\x92V[\x80_[\x83\x81\x10\x15a\x11\"W\x81Qa\x11\t\x88\x82a\x10\xB0V[\x97Pa\x11\x14\x83a\x10\xC7V[\x92PP`\x01\x81\x01\x90Pa\x10\xF5V[P\x85\x93PPPP\x92\x91PPV[a\x118\x81a\r\x01V[\x82RPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11V\x81\x85a\x10\xD3V[\x90Pa\x11e` \x83\x01\x84a\x11/V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x11\x82Wa\x11\x81a\x0C\xDAV[[_a\x11\x8F\x85\x82\x86\x01a\r(V[\x92PP` a\x11\xA0\x85\x82\x86\x01a\r(V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FGS104\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x11\xEE`\x05\x83a\x11\xAAV[\x91Pa\x11\xF9\x82a\x11\xBAV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\x1B\x81a\x11\xE2V[\x90P\x91\x90PV[\x7FGS101\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x12V`\x05\x83a\x11\xAAV[\x91Pa\x12a\x82a\x12\"V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\x83\x81a\x12JV[\x90P\x91\x90PV[\x7FGS102\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x12\xBE`\x05\x83a\x11\xAAV[\x91Pa\x12\xC9\x82a\x12\x8AV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\xEB\x81a\x12\xB2V[\x90P\x91\x90PV[_` \x82\x01\x90Pa\x13\x05_\x83\x01\x84a\x11/V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x13o\x82a\r\x9AV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x13\xA1Wa\x13\xA0a\x138V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FGS103\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x13\xE0`\x05\x83a\x11\xAAV[\x91Pa\x13\xEB\x82a\x13\xACV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x14\r\x81a\x13\xD4V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x7FGS031\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x14u`\x05\x83a\x11\xAAV[\x91Pa\x14\x80\x82a\x14AV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x14\xA2\x81a\x14iV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xA9\xAA\x1C\xAB\x98\x98 ?\x87-1tq\x96\x1C\xDBJ\x87\xF3\x84z\xA8H\xC1O\xAB\xF1\xAC<\xA3\x03\xC2dsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610060575f3560e01c80632d9ad53d14610064578063468721a7146100945780635229073f146100c4578063610b5925146100f5578063cc2f845214610111578063e009cfde14610142575b5f5ffd5b61007e60048036038101906100799190610d3c565b61015e565b60405161008b9190610d81565b60405180910390f35b6100ae60048036038101906100a99190610f2c565b61022a565b6040516100bb9190610d81565b60405180910390f35b6100de60048036038101906100d99190610f2c565b6103d7565b6040516100ec92919061100c565b60405180910390f35b61010f600480360381019061010a9190610d3c565b61040b565b005b61012b6004803603810190610126919061103a565b61070f565b60405161013992919061113e565b60405180910390f35b61015c6004803603810190610157919061116c565b610908565b005b5f8173ffffffffffffffffffffffffffffffffffffffff16600173ffffffffffffffffffffffffffffffffffffffff161415801561022357505f73ffffffffffffffffffffffffffffffffffffffff165f5f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614155b9050919050565b5f600173ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141580156102ef57505f73ffffffffffffffffffffffffffffffffffffffff165f5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614155b61032e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161032590611204565b60405180910390fd5b61033b858585855a610c0b565b9050801561038b573373ffffffffffffffffffffffffffffffffffffffff167f6895c13664aa4f67288b25d7a21d7aaa34916e355fb9b6fae0a139a9085becb860405160405180910390a26103cf565b3373ffffffffffffffffffffffffffffffffffffffff167facd2c8702804128fdb0db2bb49f6d127dd0181c13fd45dbfe16de0930e2bd37560405160405180910390a25b949350505050565b5f60606103e68686868661022a565b915060405160203d0181016040523d81523d5f602083013e8091505094509492505050565b610413610c61565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161415801561047c5750600173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614155b6104bb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104b29061126c565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff165f5f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610584576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161057b906112d4565b60405180910390fd5b5f5f600173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f5f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550805f5f600173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507fecdf3a3effea5783a3c4c2140e677577666428d44ed9d474a0b3a4c9943f84408160405161070491906112f2565b60405180910390a150565b60605f8267ffffffffffffffff81111561072c5761072b610de5565b5b60405190808252806020026020018201604052801561075a5781602001602082028036833780820191505090505b5091505f5f90505f5f5f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690505b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161415801561082a5750600173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614155b801561083557508482105b156108f9578084838151811061084e5761084d61130b565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250505f5f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081806108f190611365565b9250506107c1565b80925081845250509250929050565b610910610c61565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16141580156109795750600173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614155b6109b8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016109af9061126c565b60405180910390fd5b8073ffffffffffffffffffffffffffffffffffffffff165f5f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610a81576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a78906113f6565b60405180910390fd5b5f5f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f5f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f5f5f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507faab4fa2b463f581b2b32cb3b7e3b704b9ce37cc209b5fb4d77e593ace405427681604051610bff91906112f2565b60405180910390a15050565b5f600180811115610c1f57610c1e611414565b5b836001811115610c3257610c31611414565b5b03610c49575f5f8551602087018986f49050610c58565b5f5f855160208701888a87f190505b95945050505050565b3073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610ccf576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610cc69061148b565b60405180910390fd5b565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610d0b82610ce2565b9050919050565b610d1b81610d01565b8114610d25575f5ffd5b50565b5f81359050610d3681610d12565b92915050565b5f60208284031215610d5157610d50610cda565b5b5f610d5e84828501610d28565b91505092915050565b5f8115159050919050565b610d7b81610d67565b82525050565b5f602082019050610d945f830184610d72565b92915050565b5f819050919050565b610dac81610d9a565b8114610db6575f5ffd5b50565b5f81359050610dc781610da3565b92915050565b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610e1b82610dd5565b810181811067ffffffffffffffff82111715610e3a57610e39610de5565b5b80604052505050565b5f610e4c610cd1565b9050610e588282610e12565b919050565b5f67ffffffffffffffff821115610e7757610e76610de5565b5b610e8082610dd5565b9050602081019050919050565b828183375f83830152505050565b5f610ead610ea884610e5d565b610e43565b905082815260208101848484011115610ec957610ec8610dd1565b5b610ed4848285610e8d565b509392505050565b5f82601f830112610ef057610eef610dcd565b5b8135610f00848260208601610e9b565b91505092915050565b60028110610f15575f5ffd5b50565b5f81359050610f2681610f09565b92915050565b5f5f5f5f60808587031215610f4457610f43610cda565b5b5f610f5187828801610d28565b9450506020610f6287828801610db9565b935050604085013567ffffffffffffffff811115610f8357610f82610cde565b5b610f8f87828801610edc565b9250506060610fa087828801610f18565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f610fde82610fac565b610fe88185610fb6565b9350610ff8818560208601610fc6565b61100181610dd5565b840191505092915050565b5f60408201905061101f5f830185610d72565b81810360208301526110318184610fd4565b90509392505050565b5f5f604083850312156110505761104f610cda565b5b5f61105d85828601610d28565b925050602061106e85828601610db9565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6110aa81610d01565b82525050565b5f6110bb83836110a1565b60208301905092915050565b5f602082019050919050565b5f6110dd82611078565b6110e78185611082565b93506110f283611092565b805f5b8381101561112257815161110988826110b0565b9750611114836110c7565b9250506001810190506110f5565b5085935050505092915050565b61113881610d01565b82525050565b5f6040820190508181035f83015261115681856110d3565b9050611165602083018461112f565b9392505050565b5f5f6040838503121561118257611181610cda565b5b5f61118f85828601610d28565b92505060206111a085828601610d28565b9150509250929050565b5f82825260208201905092915050565b7f47533130340000000000000000000000000000000000000000000000000000005f82015250565b5f6111ee6005836111aa565b91506111f9826111ba565b602082019050919050565b5f6020820190508181035f83015261121b816111e2565b9050919050565b7f47533130310000000000000000000000000000000000000000000000000000005f82015250565b5f6112566005836111aa565b915061126182611222565b602082019050919050565b5f6020820190508181035f8301526112838161124a565b9050919050565b7f47533130320000000000000000000000000000000000000000000000000000005f82015250565b5f6112be6005836111aa565b91506112c98261128a565b602082019050919050565b5f6020820190508181035f8301526112eb816112b2565b9050919050565b5f6020820190506113055f83018461112f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61136f82610d9a565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036113a1576113a0611338565b5b600182019050919050565b7f47533130330000000000000000000000000000000000000000000000000000005f82015250565b5f6113e06005836111aa565b91506113eb826113ac565b602082019050919050565b5f6020820190508181035f83015261140d816113d4565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b7f47533033310000000000000000000000000000000000000000000000000000005f82015250565b5f6114756005836111aa565b915061148082611441565b602082019050919050565b5f6020820190508181035f8301526114a281611469565b905091905056fea2646970667358221220a9aa1cab9898203f872d317471961cdb4a87f3847aa848c14fabf1ac3ca303c264736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0`W_5`\xE0\x1C\x80c-\x9A\xD5=\x14a\0dW\x80cF\x87!\xA7\x14a\0\x94W\x80cR)\x07?\x14a\0\xC4W\x80ca\x0BY%\x14a\0\xF5W\x80c\xCC/\x84R\x14a\x01\x11W\x80c\xE0\t\xCF\xDE\x14a\x01BW[__\xFD[a\0~`\x04\x806\x03\x81\x01\x90a\0y\x91\x90a\r<V[a\x01^V[`@Qa\0\x8B\x91\x90a\r\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xAE`\x04\x806\x03\x81\x01\x90a\0\xA9\x91\x90a\x0F,V[a\x02*V[`@Qa\0\xBB\x91\x90a\r\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xDE`\x04\x806\x03\x81\x01\x90a\0\xD9\x91\x90a\x0F,V[a\x03\xD7V[`@Qa\0\xEC\x92\x91\x90a\x10\x0CV[`@Q\x80\x91\x03\x90\xF3[a\x01\x0F`\x04\x806\x03\x81\x01\x90a\x01\n\x91\x90a\r<V[a\x04\x0BV[\0[a\x01+`\x04\x806\x03\x81\x01\x90a\x01&\x91\x90a\x10:V[a\x07\x0FV[`@Qa\x019\x92\x91\x90a\x11>V[`@Q\x80\x91\x03\x90\xF3[a\x01\\`\x04\x806\x03\x81\x01\x90a\x01W\x91\x90a\x11lV[a\t\x08V[\0[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x02#WP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x90P\x91\x90PV[_`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x02\xEFWP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[a\x03.W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03%\x90a\x12\x04V[`@Q\x80\x91\x03\x90\xFD[a\x03;\x85\x85\x85\x85Za\x0C\x0BV[\x90P\x80\x15a\x03\x8BW3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8`@Q`@Q\x80\x91\x03\x90\xA2a\x03\xCFV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u`@Q`@Q\x80\x91\x03\x90\xA2[\x94\x93PPPPV[_``a\x03\xE6\x86\x86\x86\x86a\x02*V[\x91P`@Q` =\x01\x81\x01`@R=\x81R=_` \x83\x01>\x80\x91PP\x94P\x94\x92PPPV[a\x04\x13a\x0CaV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x04|WP`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[a\x04\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB2\x90a\x12lV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05{\x90a\x12\xD4V[`@Q\x80\x91\x03\x90\xFD[__`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80__`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x81`@Qa\x07\x04\x91\x90a\x12\xF2V[`@Q\x80\x91\x03\x90\xA1PV[``_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07,Wa\x07+a\r\xE5V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07ZW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P__\x90P___\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x08*WP`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x80\x15a\x085WP\x84\x82\x10[\x15a\x08\xF9W\x80\x84\x83\x81Q\x81\x10a\x08NWa\x08Ma\x13\x0BV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81\x80a\x08\xF1\x90a\x13eV[\x92PPa\x07\xC1V[\x80\x92P\x81\x84RPP\x92P\x92\x90PV[a\t\x10a\x0CaV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\tyWP`\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[a\t\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xAF\x90a\x12lV[`@Q\x80\x91\x03\x90\xFD[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nx\x90a\x13\xF6V[`@Q\x80\x91\x03\x90\xFD[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP___\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x81`@Qa\x0B\xFF\x91\x90a\x12\xF2V[`@Q\x80\x91\x03\x90\xA1PPV[_`\x01\x80\x81\x11\x15a\x0C\x1FWa\x0C\x1Ea\x14\x14V[[\x83`\x01\x81\x11\x15a\x0C2Wa\x0C1a\x14\x14V[[\x03a\x0CIW__\x85Q` \x87\x01\x89\x86\xF4\x90Pa\x0CXV[__\x85Q` \x87\x01\x88\x8A\x87\xF1\x90P[\x95\x94PPPPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xC6\x90a\x14\x8BV[`@Q\x80\x91\x03\x90\xFD[V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\r\x0B\x82a\x0C\xE2V[\x90P\x91\x90PV[a\r\x1B\x81a\r\x01V[\x81\x14a\r%W__\xFD[PV[_\x815\x90Pa\r6\x81a\r\x12V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\rQWa\rPa\x0C\xDAV[[_a\r^\x84\x82\x85\x01a\r(V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\r{\x81a\rgV[\x82RPPV[_` \x82\x01\x90Pa\r\x94_\x83\x01\x84a\rrV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\xAC\x81a\r\x9AV[\x81\x14a\r\xB6W__\xFD[PV[_\x815\x90Pa\r\xC7\x81a\r\xA3V[\x92\x91PPV[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0E\x1B\x82a\r\xD5V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E:Wa\x0E9a\r\xE5V[[\x80`@RPPPV[_a\x0ELa\x0C\xD1V[\x90Pa\x0EX\x82\x82a\x0E\x12V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0EwWa\x0Eva\r\xE5V[[a\x0E\x80\x82a\r\xD5V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0E\xADa\x0E\xA8\x84a\x0E]V[a\x0ECV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0E\xC9Wa\x0E\xC8a\r\xD1V[[a\x0E\xD4\x84\x82\x85a\x0E\x8DV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0E\xF0Wa\x0E\xEFa\r\xCDV[[\x815a\x0F\0\x84\x82` \x86\x01a\x0E\x9BV[\x91PP\x92\x91PPV[`\x02\x81\x10a\x0F\x15W__\xFD[PV[_\x815\x90Pa\x0F&\x81a\x0F\tV[\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x0FDWa\x0FCa\x0C\xDAV[[_a\x0FQ\x87\x82\x88\x01a\r(V[\x94PP` a\x0Fb\x87\x82\x88\x01a\r\xB9V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x83Wa\x0F\x82a\x0C\xDEV[[a\x0F\x8F\x87\x82\x88\x01a\x0E\xDCV[\x92PP``a\x0F\xA0\x87\x82\x88\x01a\x0F\x18V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0F\xDE\x82a\x0F\xACV[a\x0F\xE8\x81\x85a\x0F\xB6V[\x93Pa\x0F\xF8\x81\x85` \x86\x01a\x0F\xC6V[a\x10\x01\x81a\r\xD5V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\x10\x1F_\x83\x01\x85a\rrV[\x81\x81\x03` \x83\x01Ra\x101\x81\x84a\x0F\xD4V[\x90P\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x10PWa\x10Oa\x0C\xDAV[[_a\x10]\x85\x82\x86\x01a\r(V[\x92PP` a\x10n\x85\x82\x86\x01a\r\xB9V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x10\xAA\x81a\r\x01V[\x82RPPV[_a\x10\xBB\x83\x83a\x10\xA1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x10\xDD\x82a\x10xV[a\x10\xE7\x81\x85a\x10\x82V[\x93Pa\x10\xF2\x83a\x10\x92V[\x80_[\x83\x81\x10\x15a\x11\"W\x81Qa\x11\t\x88\x82a\x10\xB0V[\x97Pa\x11\x14\x83a\x10\xC7V[\x92PP`\x01\x81\x01\x90Pa\x10\xF5V[P\x85\x93PPPP\x92\x91PPV[a\x118\x81a\r\x01V[\x82RPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11V\x81\x85a\x10\xD3V[\x90Pa\x11e` \x83\x01\x84a\x11/V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x11\x82Wa\x11\x81a\x0C\xDAV[[_a\x11\x8F\x85\x82\x86\x01a\r(V[\x92PP` a\x11\xA0\x85\x82\x86\x01a\r(V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FGS104\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x11\xEE`\x05\x83a\x11\xAAV[\x91Pa\x11\xF9\x82a\x11\xBAV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\x1B\x81a\x11\xE2V[\x90P\x91\x90PV[\x7FGS101\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x12V`\x05\x83a\x11\xAAV[\x91Pa\x12a\x82a\x12\"V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\x83\x81a\x12JV[\x90P\x91\x90PV[\x7FGS102\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x12\xBE`\x05\x83a\x11\xAAV[\x91Pa\x12\xC9\x82a\x12\x8AV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\xEB\x81a\x12\xB2V[\x90P\x91\x90PV[_` \x82\x01\x90Pa\x13\x05_\x83\x01\x84a\x11/V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x13o\x82a\r\x9AV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x13\xA1Wa\x13\xA0a\x138V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FGS103\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x13\xE0`\x05\x83a\x11\xAAV[\x91Pa\x13\xEB\x82a\x13\xACV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x14\r\x81a\x13\xD4V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x7FGS031\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x14u`\x05\x83a\x11\xAAV[\x91Pa\x14\x80\x82a\x14AV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x14\xA2\x81a\x14iV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xA9\xAA\x1C\xAB\x98\x98 ?\x87-1tq\x96\x1C\xDBJ\x87\xF3\x84z\xA8H\xC1O\xAB\xF1\xAC<\xA3\x03\xC2dsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DisabledModule(address)` and selector `0xaab4fa2b463f581b2b32cb3b7e3b704b9ce37cc209b5fb4d77e593ace4054276`.
```solidity
event DisabledModule(address module);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DisabledModule {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for DisabledModule {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DisabledModule(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                170u8, 180u8, 250u8, 43u8, 70u8, 63u8, 88u8, 27u8, 43u8, 50u8, 203u8,
                59u8, 126u8, 59u8, 112u8, 75u8, 156u8, 227u8, 124u8, 194u8, 9u8, 181u8,
                251u8, 77u8, 119u8, 229u8, 147u8, 172u8, 228u8, 5u8, 66u8, 118u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: data.0 }
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.module,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DisabledModule {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DisabledModule> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DisabledModule) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EnabledModule(address)` and selector `0xecdf3a3effea5783a3c4c2140e677577666428d44ed9d474a0b3a4c9943f8440`.
```solidity
event EnabledModule(address module);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EnabledModule {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for EnabledModule {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EnabledModule(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                236u8, 223u8, 58u8, 62u8, 255u8, 234u8, 87u8, 131u8, 163u8, 196u8, 194u8,
                20u8, 14u8, 103u8, 117u8, 119u8, 102u8, 100u8, 40u8, 212u8, 78u8, 217u8,
                212u8, 116u8, 160u8, 179u8, 164u8, 201u8, 148u8, 63u8, 132u8, 64u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: data.0 }
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.module,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EnabledModule {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EnabledModule> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EnabledModule) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ExecutionFromModuleFailure(address)` and selector `0xacd2c8702804128fdb0db2bb49f6d127dd0181c13fd45dbfe16de0930e2bd375`.
```solidity
event ExecutionFromModuleFailure(address indexed module);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExecutionFromModuleFailure {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ExecutionFromModuleFailure {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ExecutionFromModuleFailure(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                172u8, 210u8, 200u8, 112u8, 40u8, 4u8, 18u8, 143u8, 219u8, 13u8, 178u8,
                187u8, 73u8, 246u8, 209u8, 39u8, 221u8, 1u8, 129u8, 193u8, 63u8, 212u8,
                93u8, 191u8, 225u8, 109u8, 224u8, 147u8, 14u8, 43u8, 211u8, 117u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.module.clone())
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
                    &self.module,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ExecutionFromModuleFailure {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionFromModuleFailure> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ExecutionFromModuleFailure,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ExecutionFromModuleSuccess(address)` and selector `0x6895c13664aa4f67288b25d7a21d7aaa34916e355fb9b6fae0a139a9085becb8`.
```solidity
event ExecutionFromModuleSuccess(address indexed module);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExecutionFromModuleSuccess {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ExecutionFromModuleSuccess {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ExecutionFromModuleSuccess(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                104u8, 149u8, 193u8, 54u8, 100u8, 170u8, 79u8, 103u8, 40u8, 139u8, 37u8,
                215u8, 162u8, 29u8, 122u8, 170u8, 52u8, 145u8, 110u8, 53u8, 95u8, 185u8,
                182u8, 250u8, 224u8, 161u8, 57u8, 169u8, 8u8, 91u8, 236u8, 184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.module.clone())
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
                    &self.module,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ExecutionFromModuleSuccess {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionFromModuleSuccess> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ExecutionFromModuleSuccess,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disableModule(address,address)` and selector `0xe009cfde`.
```solidity
function disableModule(address prevModule, address module) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableModuleCall {
        #[allow(missing_docs)]
        pub prevModule: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`disableModule(address,address)`](disableModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableModuleReturn {}
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
            impl ::core::convert::From<disableModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: disableModuleCall) -> Self {
                    (value.prevModule, value.module)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        prevModule: tuple.0,
                        module: tuple.1,
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
            impl ::core::convert::From<disableModuleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disableModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl disableModuleReturn {
            fn _tokenize(
                &self,
            ) -> <disableModuleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableModuleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableModule(address,address)";
            const SELECTOR: [u8; 4] = [224u8, 9u8, 207u8, 222u8];
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
                        &self.prevModule,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.module,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                disableModuleReturn::_tokenize(ret)
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
    /**Function with signature `enableModule(address)` and selector `0x610b5925`.
```solidity
function enableModule(address module) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableModuleCall {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`enableModule(address)`](enableModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableModuleReturn {}
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
            impl ::core::convert::From<enableModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: enableModuleCall) -> Self {
                    (value.module,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { module: tuple.0 }
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
            impl ::core::convert::From<enableModuleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: enableModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl enableModuleReturn {
            fn _tokenize(
                &self,
            ) -> <enableModuleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableModuleCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enableModule(address)";
            const SELECTOR: [u8; 4] = [97u8, 11u8, 89u8, 37u8];
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
                        &self.module,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                enableModuleReturn::_tokenize(ret)
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
    /**Function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`.
```solidity
function execTransactionFromModule(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`execTransactionFromModule(address,uint256,bytes,uint8)`](execTransactionFromModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturn {
        #[allow(missing_docs)]
        pub success: bool,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<execTransactionFromModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleCall) -> Self {
                    (value.to, value.value, value.data, value.operation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        operation: tuple.3,
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
            impl ::core::convert::From<execTransactionFromModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturn) -> Self {
                    (value.success,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { success: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for execTransactionFromModuleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execTransactionFromModule(address,uint256,bytes,uint8)";
            const SELECTOR: [u8; 4] = [70u8, 135u8, 33u8, 167u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self.operation,
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
                        let r: execTransactionFromModuleReturn = r.into();
                        r.success
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
                        let r: execTransactionFromModuleReturn = r.into();
                        r.success
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`.
```solidity
function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success, bytes memory returnData);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturnDataCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`execTransactionFromModuleReturnData(address,uint256,bytes,uint8)`](execTransactionFromModuleReturnDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturnDataReturn {
        #[allow(missing_docs)]
        pub success: bool,
        #[allow(missing_docs)]
        pub returnData: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<execTransactionFromModuleReturnDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturnDataCall) -> Self {
                    (value.to, value.value, value.data, value.operation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleReturnDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        operation: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, alloy::sol_types::private::Bytes);
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
            impl ::core::convert::From<execTransactionFromModuleReturnDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturnDataReturn) -> Self {
                    (value.success, value.returnData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleReturnDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        success: tuple.0,
                        returnData: tuple.1,
                    }
                }
            }
        }
        impl execTransactionFromModuleReturnDataReturn {
            fn _tokenize(
                &self,
            ) -> <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.success,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.returnData,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for execTransactionFromModuleReturnDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = execTransactionFromModuleReturnDataReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)";
            const SELECTOR: [u8; 4] = [82u8, 41u8, 7u8, 63u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self.operation,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                execTransactionFromModuleReturnDataReturn::_tokenize(ret)
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
    /**Function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`.
```solidity
function getModulesPaginated(address start, uint256 pageSize) external view returns (address[] memory array, address next);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getModulesPaginatedCall {
        #[allow(missing_docs)]
        pub start: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub pageSize: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getModulesPaginated(address,uint256)`](getModulesPaginatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getModulesPaginatedReturn {
        #[allow(missing_docs)]
        pub array: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub next: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getModulesPaginatedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getModulesPaginatedCall) -> Self {
                    (value.start, value.pageSize)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getModulesPaginatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        start: tuple.0,
                        pageSize: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getModulesPaginatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getModulesPaginatedReturn) -> Self {
                    (value.array, value.next)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getModulesPaginatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        array: tuple.0,
                        next: tuple.1,
                    }
                }
            }
        }
        impl getModulesPaginatedReturn {
            fn _tokenize(
                &self,
            ) -> <getModulesPaginatedCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.array),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.next,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getModulesPaginatedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getModulesPaginatedReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getModulesPaginated(address,uint256)";
            const SELECTOR: [u8; 4] = [204u8, 47u8, 132u8, 82u8];
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
                        &self.start,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.pageSize),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getModulesPaginatedReturn::_tokenize(ret)
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
    /**Function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`.
```solidity
function isModuleEnabled(address module) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isModuleEnabledCall {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isModuleEnabled(address)`](isModuleEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isModuleEnabledReturn {
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
            impl ::core::convert::From<isModuleEnabledCall> for UnderlyingRustTuple<'_> {
                fn from(value: isModuleEnabledCall) -> Self {
                    (value.module,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isModuleEnabledCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { module: tuple.0 }
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
            impl ::core::convert::From<isModuleEnabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isModuleEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isModuleEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isModuleEnabledCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isModuleEnabled(address)";
            const SELECTOR: [u8; 4] = [45u8, 154u8, 213u8, 61u8];
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
                        &self.module,
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
                        let r: isModuleEnabledReturn = r.into();
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
                        let r: isModuleEnabledReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`ModuleManager`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ModuleManagerCalls {
        #[allow(missing_docs)]
        disableModule(disableModuleCall),
        #[allow(missing_docs)]
        enableModule(enableModuleCall),
        #[allow(missing_docs)]
        execTransactionFromModule(execTransactionFromModuleCall),
        #[allow(missing_docs)]
        execTransactionFromModuleReturnData(execTransactionFromModuleReturnDataCall),
        #[allow(missing_docs)]
        getModulesPaginated(getModulesPaginatedCall),
        #[allow(missing_docs)]
        isModuleEnabled(isModuleEnabledCall),
    }
    impl ModuleManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [45u8, 154u8, 213u8, 61u8],
            [70u8, 135u8, 33u8, 167u8],
            [82u8, 41u8, 7u8, 63u8],
            [97u8, 11u8, 89u8, 37u8],
            [204u8, 47u8, 132u8, 82u8],
            [224u8, 9u8, 207u8, 222u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(isModuleEnabled),
            ::core::stringify!(execTransactionFromModule),
            ::core::stringify!(execTransactionFromModuleReturnData),
            ::core::stringify!(enableModule),
            ::core::stringify!(getModulesPaginated),
            ::core::stringify!(disableModule),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <isModuleEnabledCall as alloy_sol_types::SolCall>::SIGNATURE,
            <execTransactionFromModuleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <enableModuleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getModulesPaginatedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disableModuleCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for ModuleManagerCalls {
        const NAME: &'static str = "ModuleManagerCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::disableModule(_) => {
                    <disableModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::enableModule(_) => {
                    <enableModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execTransactionFromModule(_) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execTransactionFromModuleReturnData(_) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getModulesPaginated(_) => {
                    <getModulesPaginatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isModuleEnabled(_) => {
                    <isModuleEnabledCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ModuleManagerCalls>] = &[
                {
                    fn isModuleEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <isModuleEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ModuleManagerCalls::isModuleEnabled)
                    }
                    isModuleEnabled
                },
                {
                    fn execTransactionFromModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ModuleManagerCalls::execTransactionFromModule)
                    }
                    execTransactionFromModule
                },
                {
                    fn execTransactionFromModuleReturnData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ModuleManagerCalls::execTransactionFromModuleReturnData)
                    }
                    execTransactionFromModuleReturnData
                },
                {
                    fn enableModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <enableModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ModuleManagerCalls::enableModule)
                    }
                    enableModule
                },
                {
                    fn getModulesPaginated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <getModulesPaginatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ModuleManagerCalls::getModulesPaginated)
                    }
                    getModulesPaginated
                },
                {
                    fn disableModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <disableModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ModuleManagerCalls::disableModule)
                    }
                    disableModule
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
            ) -> alloy_sol_types::Result<ModuleManagerCalls>] = &[
                {
                    fn isModuleEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <isModuleEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ModuleManagerCalls::isModuleEnabled)
                    }
                    isModuleEnabled
                },
                {
                    fn execTransactionFromModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ModuleManagerCalls::execTransactionFromModule)
                    }
                    execTransactionFromModule
                },
                {
                    fn execTransactionFromModuleReturnData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ModuleManagerCalls::execTransactionFromModuleReturnData)
                    }
                    execTransactionFromModuleReturnData
                },
                {
                    fn enableModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <enableModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ModuleManagerCalls::enableModule)
                    }
                    enableModule
                },
                {
                    fn getModulesPaginated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <getModulesPaginatedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ModuleManagerCalls::getModulesPaginated)
                    }
                    getModulesPaginated
                },
                {
                    fn disableModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ModuleManagerCalls> {
                        <disableModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ModuleManagerCalls::disableModule)
                    }
                    disableModule
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
                Self::disableModule(inner) => {
                    <disableModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::enableModule(inner) => {
                    <enableModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execTransactionFromModule(inner) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execTransactionFromModuleReturnData(inner) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getModulesPaginated(inner) => {
                    <getModulesPaginatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isModuleEnabled(inner) => {
                    <isModuleEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::disableModule(inner) => {
                    <disableModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::enableModule(inner) => {
                    <enableModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execTransactionFromModule(inner) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execTransactionFromModuleReturnData(inner) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getModulesPaginated(inner) => {
                    <getModulesPaginatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isModuleEnabled(inner) => {
                    <isModuleEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ModuleManager`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ModuleManagerEvents {
        #[allow(missing_docs)]
        DisabledModule(DisabledModule),
        #[allow(missing_docs)]
        EnabledModule(EnabledModule),
        #[allow(missing_docs)]
        ExecutionFromModuleFailure(ExecutionFromModuleFailure),
        #[allow(missing_docs)]
        ExecutionFromModuleSuccess(ExecutionFromModuleSuccess),
    }
    impl ModuleManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                104u8, 149u8, 193u8, 54u8, 100u8, 170u8, 79u8, 103u8, 40u8, 139u8, 37u8,
                215u8, 162u8, 29u8, 122u8, 170u8, 52u8, 145u8, 110u8, 53u8, 95u8, 185u8,
                182u8, 250u8, 224u8, 161u8, 57u8, 169u8, 8u8, 91u8, 236u8, 184u8,
            ],
            [
                170u8, 180u8, 250u8, 43u8, 70u8, 63u8, 88u8, 27u8, 43u8, 50u8, 203u8,
                59u8, 126u8, 59u8, 112u8, 75u8, 156u8, 227u8, 124u8, 194u8, 9u8, 181u8,
                251u8, 77u8, 119u8, 229u8, 147u8, 172u8, 228u8, 5u8, 66u8, 118u8,
            ],
            [
                172u8, 210u8, 200u8, 112u8, 40u8, 4u8, 18u8, 143u8, 219u8, 13u8, 178u8,
                187u8, 73u8, 246u8, 209u8, 39u8, 221u8, 1u8, 129u8, 193u8, 63u8, 212u8,
                93u8, 191u8, 225u8, 109u8, 224u8, 147u8, 14u8, 43u8, 211u8, 117u8,
            ],
            [
                236u8, 223u8, 58u8, 62u8, 255u8, 234u8, 87u8, 131u8, 163u8, 196u8, 194u8,
                20u8, 14u8, 103u8, 117u8, 119u8, 102u8, 100u8, 40u8, 212u8, 78u8, 217u8,
                212u8, 116u8, 160u8, 179u8, 164u8, 201u8, 148u8, 63u8, 132u8, 64u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ExecutionFromModuleSuccess),
            ::core::stringify!(DisabledModule),
            ::core::stringify!(ExecutionFromModuleFailure),
            ::core::stringify!(EnabledModule),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ExecutionFromModuleSuccess as alloy_sol_types::SolEvent>::SIGNATURE,
            <DisabledModule as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExecutionFromModuleFailure as alloy_sol_types::SolEvent>::SIGNATURE,
            <EnabledModule as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for ModuleManagerEvents {
        const NAME: &'static str = "ModuleManagerEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<DisabledModule as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DisabledModule as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DisabledModule)
                }
                Some(<EnabledModule as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EnabledModule as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EnabledModule)
                }
                Some(
                    <ExecutionFromModuleFailure as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ExecutionFromModuleFailure as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ExecutionFromModuleFailure)
                }
                Some(
                    <ExecutionFromModuleSuccess as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ExecutionFromModuleSuccess as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ExecutionFromModuleSuccess)
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
    impl alloy_sol_types::private::IntoLogData for ModuleManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DisabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EnabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionFromModuleFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionFromModuleSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DisabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EnabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionFromModuleFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionFromModuleSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ModuleManager`](self) contract instance.

See the [wrapper's documentation](`ModuleManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ModuleManagerInstance<P, N> {
        ModuleManagerInstance::<P, N>::new(address, __provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ModuleManagerInstance<P, N>>,
    > {
        ModuleManagerInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        ModuleManagerInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`ModuleManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ModuleManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ModuleManagerInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ModuleManagerInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ModuleManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ModuleManagerInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ModuleManager`](self) contract instance.

See the [wrapper's documentation](`ModuleManagerInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ModuleManagerInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<P: ::core::clone::Clone, N> ModuleManagerInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ModuleManagerInstance<P, N> {
            ModuleManagerInstance {
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
    > ModuleManagerInstance<P, N> {
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
        ///Creates a new call builder for the [`disableModule`] function.
        pub fn disableModule(
            &self,
            prevModule: alloy::sol_types::private::Address,
            module: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, disableModuleCall, N> {
            self.call_builder(
                &disableModuleCall {
                    prevModule,
                    module,
                },
            )
        }
        ///Creates a new call builder for the [`enableModule`] function.
        pub fn enableModule(
            &self,
            module: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, enableModuleCall, N> {
            self.call_builder(&enableModuleCall { module })
        }
        ///Creates a new call builder for the [`execTransactionFromModule`] function.
        pub fn execTransactionFromModule(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, execTransactionFromModuleCall, N> {
            self.call_builder(
                &execTransactionFromModuleCall {
                    to,
                    value,
                    data,
                    operation,
                },
            )
        }
        ///Creates a new call builder for the [`execTransactionFromModuleReturnData`] function.
        pub fn execTransactionFromModuleReturnData(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            execTransactionFromModuleReturnDataCall,
            N,
        > {
            self.call_builder(
                &execTransactionFromModuleReturnDataCall {
                    to,
                    value,
                    data,
                    operation,
                },
            )
        }
        ///Creates a new call builder for the [`getModulesPaginated`] function.
        pub fn getModulesPaginated(
            &self,
            start: alloy::sol_types::private::Address,
            pageSize: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getModulesPaginatedCall, N> {
            self.call_builder(
                &getModulesPaginatedCall {
                    start,
                    pageSize,
                },
            )
        }
        ///Creates a new call builder for the [`isModuleEnabled`] function.
        pub fn isModuleEnabled(
            &self,
            module: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isModuleEnabledCall, N> {
            self.call_builder(&isModuleEnabledCall { module })
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ModuleManagerInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DisabledModule`] event.
        pub fn DisabledModule_filter(
            &self,
        ) -> alloy_contract::Event<&P, DisabledModule, N> {
            self.event_filter::<DisabledModule>()
        }
        ///Creates a new event filter for the [`EnabledModule`] event.
        pub fn EnabledModule_filter(
            &self,
        ) -> alloy_contract::Event<&P, EnabledModule, N> {
            self.event_filter::<EnabledModule>()
        }
        ///Creates a new event filter for the [`ExecutionFromModuleFailure`] event.
        pub fn ExecutionFromModuleFailure_filter(
            &self,
        ) -> alloy_contract::Event<&P, ExecutionFromModuleFailure, N> {
            self.event_filter::<ExecutionFromModuleFailure>()
        }
        ///Creates a new event filter for the [`ExecutionFromModuleSuccess`] event.
        pub fn ExecutionFromModuleSuccess_filter(
            &self,
        ) -> alloy_contract::Event<&P, ExecutionFromModuleSuccess, N> {
            self.event_filter::<ExecutionFromModuleSuccess>()
        }
    }
}
