pub use i_wst_eth::*;
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
pub mod i_wst_eth {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getStETHByWstETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStETHByWstETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ETHAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getWstETHByStETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getWstETHByStETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stETHAmount"),
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
                    ::std::borrow::ToOwned::to_owned("stETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stETH"),
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
                (
                    ::std::borrow::ToOwned::to_owned("stEthPerToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stEthPerToken"),
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
                    ::std::borrow::ToOwned::to_owned("unwrap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_wstETHAmount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wrap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wrap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stETHAmount"),
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
    pub static IWSTETH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IWstEth<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IWstEth<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IWstEth<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IWstEth<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IWstEth<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IWstEth)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IWstEth<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IWSTETH_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getStETHByWstETH` (0xbb2952fc) function
        pub fn get_st_eth_by_wst_eth(
            &self,
            eth_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([187, 41, 82, 252], eth_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWstETHByStETH` (0xb0e38900) function
        pub fn get_wst_eth_by_st_eth(
            &self,
            st_eth_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([176, 227, 137, 0], st_eth_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stETH` (0xc1fe3e48) function
        pub fn st_eth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([193, 254, 62, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stEthPerToken` (0x035faf82) function
        pub fn st_eth_per_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([3, 95, 175, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrap` (0xde0e9a3e) function
        pub fn unwrap(
            &self,
            wst_eth_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([222, 14, 154, 62], wst_eth_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrap` (0xea598cb0) function
        pub fn wrap(
            &self,
            st_eth_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([234, 89, 140, 176], st_eth_amount)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IWstEth<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getStETHByWstETH` function with signature `getStETHByWstETH(uint256)` and selector `0xbb2952fc`
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
    #[ethcall(name = "getStETHByWstETH", abi = "getStETHByWstETH(uint256)")]
    pub struct GetStETHByWstETHCall {
        pub eth_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getWstETHByStETH` function with signature `getWstETHByStETH(uint256)` and selector `0xb0e38900`
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
    #[ethcall(name = "getWstETHByStETH", abi = "getWstETHByStETH(uint256)")]
    pub struct GetWstETHByStETHCall {
        pub st_eth_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stETH` function with signature `stETH()` and selector `0xc1fe3e48`
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
    #[ethcall(name = "stETH", abi = "stETH()")]
    pub struct StETHCall;
    ///Container type for all input parameters for the `stEthPerToken` function with signature `stEthPerToken()` and selector `0x035faf82`
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
    #[ethcall(name = "stEthPerToken", abi = "stEthPerToken()")]
    pub struct StEthPerTokenCall;
    ///Container type for all input parameters for the `unwrap` function with signature `unwrap(uint256)` and selector `0xde0e9a3e`
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
    #[ethcall(name = "unwrap", abi = "unwrap(uint256)")]
    pub struct UnwrapCall {
        pub wst_eth_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `wrap` function with signature `wrap(uint256)` and selector `0xea598cb0`
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
    #[ethcall(name = "wrap", abi = "wrap(uint256)")]
    pub struct WrapCall {
        pub st_eth_amount: ::ethers::core::types::U256,
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
    pub enum IWstEthCalls {
        GetStETHByWstETH(GetStETHByWstETHCall),
        GetWstETHByStETH(GetWstETHByStETHCall),
        StETH(StETHCall),
        StEthPerToken(StEthPerTokenCall),
        Unwrap(UnwrapCall),
        Wrap(WrapCall),
    }
    impl ::ethers::core::abi::AbiDecode for IWstEthCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetStETHByWstETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStETHByWstETH(decoded));
            }
            if let Ok(decoded) = <GetWstETHByStETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetWstETHByStETH(decoded));
            }
            if let Ok(decoded) = <StETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StETH(decoded));
            }
            if let Ok(decoded) = <StEthPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StEthPerToken(decoded));
            }
            if let Ok(decoded) = <UnwrapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unwrap(decoded));
            }
            if let Ok(decoded) = <WrapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Wrap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IWstEthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetStETHByWstETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWstETHByStETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StEthPerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unwrap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Wrap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IWstEthCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetStETHByWstETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWstETHByStETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::StETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::StEthPerToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unwrap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wrap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetStETHByWstETHCall> for IWstEthCalls {
        fn from(value: GetStETHByWstETHCall) -> Self {
            Self::GetStETHByWstETH(value)
        }
    }
    impl ::core::convert::From<GetWstETHByStETHCall> for IWstEthCalls {
        fn from(value: GetWstETHByStETHCall) -> Self {
            Self::GetWstETHByStETH(value)
        }
    }
    impl ::core::convert::From<StETHCall> for IWstEthCalls {
        fn from(value: StETHCall) -> Self {
            Self::StETH(value)
        }
    }
    impl ::core::convert::From<StEthPerTokenCall> for IWstEthCalls {
        fn from(value: StEthPerTokenCall) -> Self {
            Self::StEthPerToken(value)
        }
    }
    impl ::core::convert::From<UnwrapCall> for IWstEthCalls {
        fn from(value: UnwrapCall) -> Self {
            Self::Unwrap(value)
        }
    }
    impl ::core::convert::From<WrapCall> for IWstEthCalls {
        fn from(value: WrapCall) -> Self {
            Self::Wrap(value)
        }
    }
    ///Container type for all return fields from the `getStETHByWstETH` function with signature `getStETHByWstETH(uint256)` and selector `0xbb2952fc`
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
    pub struct GetStETHByWstETHReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getWstETHByStETH` function with signature `getWstETHByStETH(uint256)` and selector `0xb0e38900`
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
    pub struct GetWstETHByStETHReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stETH` function with signature `stETH()` and selector `0xc1fe3e48`
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
    pub struct StETHReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stEthPerToken` function with signature `stEthPerToken()` and selector `0x035faf82`
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
    pub struct StEthPerTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `unwrap` function with signature `unwrap(uint256)` and selector `0xde0e9a3e`
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
    pub struct UnwrapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `wrap` function with signature `wrap(uint256)` and selector `0xea598cb0`
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
    pub struct WrapReturn(pub ::ethers::core::types::U256);
}
