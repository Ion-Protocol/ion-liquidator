pub use i_st_eth::*;
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
pub mod i_st_eth {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentStakeLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentStakeLimit",
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
                    ::std::borrow::ToOwned::to_owned("getSharesByPooledEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSharesByPooledEth",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ethAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getTotalPooledEther"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalPooledEther",
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
                    ::std::borrow::ToOwned::to_owned("getTotalShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTotalShares"),
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
                    ::std::borrow::ToOwned::to_owned("submit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_referral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
    pub static ISTETH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IStEth<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStEth<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStEth<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStEth<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStEth<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IStEth)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStEth<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISTETH_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getCurrentStakeLimit` (0x609c4c6c) function
        pub fn get_current_stake_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([96, 156, 76, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSharesByPooledEth` (0x19208451) function
        pub fn get_shares_by_pooled_eth(
            &self,
            eth_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 32, 132, 81], eth_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalPooledEther` (0x37cfdaca) function
        pub fn get_total_pooled_ether(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 207, 218, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalShares` (0xd5002f2e) function
        pub fn get_total_shares(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([213, 0, 47, 46], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submit` (0xa1903eab) function
        pub fn submit(
            &self,
            referral: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 144, 62, 171], referral)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IStEth<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getCurrentStakeLimit` function with signature `getCurrentStakeLimit()` and selector `0x609c4c6c`
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
    #[ethcall(name = "getCurrentStakeLimit", abi = "getCurrentStakeLimit()")]
    pub struct GetCurrentStakeLimitCall;
    ///Container type for all input parameters for the `getSharesByPooledEth` function with signature `getSharesByPooledEth(uint256)` and selector `0x19208451`
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
    #[ethcall(name = "getSharesByPooledEth", abi = "getSharesByPooledEth(uint256)")]
    pub struct GetSharesByPooledEthCall {
        pub eth_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalPooledEther` function with signature `getTotalPooledEther()` and selector `0x37cfdaca`
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
    #[ethcall(name = "getTotalPooledEther", abi = "getTotalPooledEther()")]
    pub struct GetTotalPooledEtherCall;
    ///Container type for all input parameters for the `getTotalShares` function with signature `getTotalShares()` and selector `0xd5002f2e`
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
    #[ethcall(name = "getTotalShares", abi = "getTotalShares()")]
    pub struct GetTotalSharesCall;
    ///Container type for all input parameters for the `submit` function with signature `submit(address)` and selector `0xa1903eab`
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
    #[ethcall(name = "submit", abi = "submit(address)")]
    pub struct SubmitCall {
        pub referral: ::ethers::core::types::Address,
    }
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
    pub enum IStEthCalls {
        GetCurrentStakeLimit(GetCurrentStakeLimitCall),
        GetSharesByPooledEth(GetSharesByPooledEthCall),
        GetTotalPooledEther(GetTotalPooledEtherCall),
        GetTotalShares(GetTotalSharesCall),
        Submit(SubmitCall),
    }
    impl ::ethers::core::abi::AbiDecode for IStEthCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCurrentStakeLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentStakeLimit(decoded));
            }
            if let Ok(decoded) = <GetSharesByPooledEthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSharesByPooledEth(decoded));
            }
            if let Ok(decoded) = <GetTotalPooledEtherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalPooledEther(decoded));
            }
            if let Ok(decoded) = <GetTotalSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalShares(decoded));
            }
            if let Ok(decoded) = <SubmitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Submit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IStEthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCurrentStakeLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSharesByPooledEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalPooledEther(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Submit(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IStEthCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCurrentStakeLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSharesByPooledEth(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalPooledEther(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Submit(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCurrentStakeLimitCall> for IStEthCalls {
        fn from(value: GetCurrentStakeLimitCall) -> Self {
            Self::GetCurrentStakeLimit(value)
        }
    }
    impl ::core::convert::From<GetSharesByPooledEthCall> for IStEthCalls {
        fn from(value: GetSharesByPooledEthCall) -> Self {
            Self::GetSharesByPooledEth(value)
        }
    }
    impl ::core::convert::From<GetTotalPooledEtherCall> for IStEthCalls {
        fn from(value: GetTotalPooledEtherCall) -> Self {
            Self::GetTotalPooledEther(value)
        }
    }
    impl ::core::convert::From<GetTotalSharesCall> for IStEthCalls {
        fn from(value: GetTotalSharesCall) -> Self {
            Self::GetTotalShares(value)
        }
    }
    impl ::core::convert::From<SubmitCall> for IStEthCalls {
        fn from(value: SubmitCall) -> Self {
            Self::Submit(value)
        }
    }
    ///Container type for all return fields from the `getCurrentStakeLimit` function with signature `getCurrentStakeLimit()` and selector `0x609c4c6c`
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
    pub struct GetCurrentStakeLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSharesByPooledEth` function with signature `getSharesByPooledEth(uint256)` and selector `0x19208451`
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
    pub struct GetSharesByPooledEthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalPooledEther` function with signature `getTotalPooledEther()` and selector `0x37cfdaca`
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
    pub struct GetTotalPooledEtherReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalShares` function with signature `getTotalShares()` and selector `0xd5002f2e`
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
    pub struct GetTotalSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submit` function with signature `submit(address)` and selector `0xa1903eab`
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
    pub struct SubmitReturn(pub ::ethers::core::types::U256);
}
