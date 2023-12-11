pub use i_stader_config::*;
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
pub mod i_stader_config {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getMaxDepositAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMaxDepositAmount",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMinDepositAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMinDepositAmount",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStaderOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStaderOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
    pub static ISTADERCONFIG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IStaderConfig<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStaderConfig<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStaderConfig<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStaderConfig<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStaderConfig<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IStaderConfig))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStaderConfig<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISTADERCONFIG_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getMaxDepositAmount` (0x5726a356) function
        pub fn get_max_deposit_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([87, 38, 163, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinDepositAmount` (0xf4914d33) function
        pub fn get_min_deposit_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 145, 77, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStaderOracle` (0xdefd024d) function
        pub fn get_stader_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([222, 253, 2, 77], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IStaderConfig<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getMaxDepositAmount` function with signature `getMaxDepositAmount()` and selector `0x5726a356`
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
        Hash
    )]
    #[ethcall(name = "getMaxDepositAmount", abi = "getMaxDepositAmount()")]
    pub struct GetMaxDepositAmountCall;
    ///Container type for all input parameters for the `getMinDepositAmount` function with signature `getMinDepositAmount()` and selector `0xf4914d33`
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
        Hash
    )]
    #[ethcall(name = "getMinDepositAmount", abi = "getMinDepositAmount()")]
    pub struct GetMinDepositAmountCall;
    ///Container type for all input parameters for the `getStaderOracle` function with signature `getStaderOracle()` and selector `0xdefd024d`
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
        Hash
    )]
    #[ethcall(name = "getStaderOracle", abi = "getStaderOracle()")]
    pub struct GetStaderOracleCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum IStaderConfigCalls {
        GetMaxDepositAmount(GetMaxDepositAmountCall),
        GetMinDepositAmount(GetMinDepositAmountCall),
        GetStaderOracle(GetStaderOracleCall),
    }
    impl ::ethers::core::abi::AbiDecode for IStaderConfigCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetMaxDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMaxDepositAmount(decoded));
            }
            if let Ok(decoded) = <GetMinDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMinDepositAmount(decoded));
            }
            if let Ok(decoded) = <GetStaderOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStaderOracle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IStaderConfigCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetMaxDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMinDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStaderOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IStaderConfigCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetMaxDepositAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMinDepositAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStaderOracle(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetMaxDepositAmountCall> for IStaderConfigCalls {
        fn from(value: GetMaxDepositAmountCall) -> Self {
            Self::GetMaxDepositAmount(value)
        }
    }
    impl ::core::convert::From<GetMinDepositAmountCall> for IStaderConfigCalls {
        fn from(value: GetMinDepositAmountCall) -> Self {
            Self::GetMinDepositAmount(value)
        }
    }
    impl ::core::convert::From<GetStaderOracleCall> for IStaderConfigCalls {
        fn from(value: GetStaderOracleCall) -> Self {
            Self::GetStaderOracle(value)
        }
    }
    ///Container type for all return fields from the `getMaxDepositAmount` function with signature `getMaxDepositAmount()` and selector `0x5726a356`
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
        Hash
    )]
    pub struct GetMaxDepositAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMinDepositAmount` function with signature `getMinDepositAmount()` and selector `0xf4914d33`
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
        Hash
    )]
    pub struct GetMinDepositAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStaderOracle` function with signature `getStaderOracle()` and selector `0xdefd024d`
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
        Hash
    )]
    pub struct GetStaderOracleReturn(pub ::ethers::core::types::Address);
}
