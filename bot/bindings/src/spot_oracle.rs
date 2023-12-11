pub use spot_oracle::*;
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
pub mod spot_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LTV"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LTV"),
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
                    ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ReserveOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPrice"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("getSpot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spot"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidLtv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidLtv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReserveOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidReserveOracle",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MathOverflowedMulDiv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MathOverflowedMulDiv",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SPOTORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct SpotOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SpotOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SpotOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SpotOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SpotOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SpotOracle)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SpotOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SPOTORACLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `LTV` (0x62965d8e) function
        pub fn ltv(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 150, 93, 142], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_ORACLE` (0x1f5155c4) function
        pub fn reserve_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([31, 81, 85, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPrice` (0x98d5fdca) function
        pub fn get_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 213, 253, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpot` (0x2b37269c) function
        pub fn get_spot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([43, 55, 38, 156], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SpotOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidLtv` with signature `InvalidLtv(uint256)` and selector `0x2e3fcf0a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidLtv", abi = "InvalidLtv(uint256)")]
    pub struct InvalidLtv {
        pub ltv: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidReserveOracle` with signature `InvalidReserveOracle()` and selector `0xc4322b81`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidReserveOracle", abi = "InvalidReserveOracle()")]
    pub struct InvalidReserveOracle;
    ///Custom Error type `MathOverflowedMulDiv` with signature `MathOverflowedMulDiv()` and selector `0x227bc153`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MathOverflowedMulDiv", abi = "MathOverflowedMulDiv()")]
    pub struct MathOverflowedMulDiv;
    ///Container type for all of the contract's custom errors
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
    pub enum SpotOracleErrors {
        InvalidLtv(InvalidLtv),
        InvalidReserveOracle(InvalidReserveOracle),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SpotOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidLtv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidLtv(decoded));
            }
            if let Ok(decoded) = <InvalidReserveOracle as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReserveOracle(decoded));
            }
            if let Ok(decoded) = <MathOverflowedMulDiv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MathOverflowedMulDiv(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SpotOracleErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidLtv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SpotOracleErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidLtv as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidReserveOracle as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MathOverflowedMulDiv as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SpotOracleErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidLtv(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReserveOracle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SpotOracleErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidLtv> for SpotOracleErrors {
        fn from(value: InvalidLtv) -> Self {
            Self::InvalidLtv(value)
        }
    }
    impl ::core::convert::From<InvalidReserveOracle> for SpotOracleErrors {
        fn from(value: InvalidReserveOracle) -> Self {
            Self::InvalidReserveOracle(value)
        }
    }
    impl ::core::convert::From<MathOverflowedMulDiv> for SpotOracleErrors {
        fn from(value: MathOverflowedMulDiv) -> Self {
            Self::MathOverflowedMulDiv(value)
        }
    }
    ///Container type for all input parameters for the `LTV` function with signature `LTV()` and selector `0x62965d8e`
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
    #[ethcall(name = "LTV", abi = "LTV()")]
    pub struct LtvCall;
    ///Container type for all input parameters for the `RESERVE_ORACLE` function with signature `RESERVE_ORACLE()` and selector `0x1f5155c4`
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
    #[ethcall(name = "RESERVE_ORACLE", abi = "RESERVE_ORACLE()")]
    pub struct ReserveOracleCall;
    ///Container type for all input parameters for the `getPrice` function with signature `getPrice()` and selector `0x98d5fdca`
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
    #[ethcall(name = "getPrice", abi = "getPrice()")]
    pub struct GetPriceCall;
    ///Container type for all input parameters for the `getSpot` function with signature `getSpot()` and selector `0x2b37269c`
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
    #[ethcall(name = "getSpot", abi = "getSpot()")]
    pub struct GetSpotCall;
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
    pub enum SpotOracleCalls {
        Ltv(LtvCall),
        ReserveOracle(ReserveOracleCall),
        GetPrice(GetPriceCall),
        GetSpot(GetSpotCall),
    }
    impl ::ethers::core::abi::AbiDecode for SpotOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LtvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ltv(decoded));
            }
            if let Ok(decoded) = <ReserveOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveOracle(decoded));
            }
            if let Ok(decoded) = <GetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPrice(decoded));
            }
            if let Ok(decoded) = <GetSpotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSpot(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SpotOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Ltv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpot(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SpotOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Ltv(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpot(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LtvCall> for SpotOracleCalls {
        fn from(value: LtvCall) -> Self {
            Self::Ltv(value)
        }
    }
    impl ::core::convert::From<ReserveOracleCall> for SpotOracleCalls {
        fn from(value: ReserveOracleCall) -> Self {
            Self::ReserveOracle(value)
        }
    }
    impl ::core::convert::From<GetPriceCall> for SpotOracleCalls {
        fn from(value: GetPriceCall) -> Self {
            Self::GetPrice(value)
        }
    }
    impl ::core::convert::From<GetSpotCall> for SpotOracleCalls {
        fn from(value: GetSpotCall) -> Self {
            Self::GetSpot(value)
        }
    }
    ///Container type for all return fields from the `LTV` function with signature `LTV()` and selector `0x62965d8e`
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
    pub struct LtvReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RESERVE_ORACLE` function with signature `RESERVE_ORACLE()` and selector `0x1f5155c4`
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
    pub struct ReserveOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPrice` function with signature `getPrice()` and selector `0x98d5fdca`
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
    pub struct GetPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getSpot` function with signature `getSpot()` and selector `0x2b37269c`
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
    pub struct GetSpotReturn {
        pub spot: ::ethers::core::types::U256,
    }
}
