pub use reserve_oracle::*;
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
pub mod reserve_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FEED0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FEED0"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IReserveFeed"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FEED1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FEED1"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IReserveFeed"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FEED2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FEED2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IReserveFeed"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ILK_INDEX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ILK_INDEX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_CHANGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_CHANGE"),
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
                    ::std::borrow::ToOwned::to_owned("QUORUM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("QUORUM"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentExchangeRate",
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
                    ::std::borrow::ToOwned::to_owned("getProtocolExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProtocolExchangeRate",
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
                    ::std::borrow::ToOwned::to_owned("lastUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastUpdated"),
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
                    ::std::borrow::ToOwned::to_owned("updateExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateExchangeRate"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("UpdateExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UpdateExchangeRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("exchangeRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeedLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFeedLength"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invalidLength"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidInitialization",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "invalidExchangeRate",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("InvalidMaxChange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMaxChange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invalidMaxChange"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidMinMax"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMinMax"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invalidMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invalidMax"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidQuorum"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invalidQuorum"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
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
                (
                    ::std::borrow::ToOwned::to_owned("UpdateCooldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UpdateCooldown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastUpdated"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RESERVEORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ReserveOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ReserveOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ReserveOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ReserveOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ReserveOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ReserveOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ReserveOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESERVEORACLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `FEED0` (0x36c03936) function
        pub fn feed0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([54, 192, 57, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FEED1` (0xbc04cb0f) function
        pub fn feed1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([188, 4, 203, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FEED2` (0x4be39f2f) function
        pub fn feed2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([75, 227, 159, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ILK_INDEX` (0xed0cee97) function
        pub fn ilk_index(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([237, 12, 238, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_CHANGE` (0x954a53f9) function
        pub fn max_change(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([149, 74, 83, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `QUORUM` (0x2e80d9b6) function
        pub fn quorum(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([46, 128, 217, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentExchangeRate` (0xa3684977) function
        pub fn current_exchange_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([163, 104, 73, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProtocolExchangeRate` (0xf13597a6) function
        pub fn get_protocol_exchange_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 53, 151, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdated` (0xd0b06f5d) function
        pub fn last_updated(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([208, 176, 111, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateExchangeRate` (0x02ce728f) function
        pub fn update_exchange_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 206, 114, 143], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `UpdateExchangeRate` event
        pub fn update_exchange_rate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateExchangeRateFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateExchangeRateFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ReserveOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidFeedLength` with signature `InvalidFeedLength(uint256)` and selector `0xb52571da`
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
    #[etherror(name = "InvalidFeedLength", abi = "InvalidFeedLength(uint256)")]
    pub struct InvalidFeedLength {
        pub invalid_length: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization(uint256)` and selector `0x02f35f83`
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
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization(uint256)")]
    pub struct InvalidInitialization {
        pub invalid_exchange_rate: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidMaxChange` with signature `InvalidMaxChange(uint256)` and selector `0xd8912ac5`
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
    #[etherror(name = "InvalidMaxChange", abi = "InvalidMaxChange(uint256)")]
    pub struct InvalidMaxChange {
        pub invalid_max_change: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidMinMax` with signature `InvalidMinMax(uint256,uint256)` and selector `0xf60db6cc`
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
    #[etherror(name = "InvalidMinMax", abi = "InvalidMinMax(uint256,uint256)")]
    pub struct InvalidMinMax {
        pub invalid_min: ::ethers::core::types::U256,
        pub invalid_max: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidQuorum` with signature `InvalidQuorum(uint8)` and selector `0x4c7795ff`
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
    #[etherror(name = "InvalidQuorum", abi = "InvalidQuorum(uint8)")]
    pub struct InvalidQuorum {
        pub invalid_quorum: u8,
    }
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
    ///Custom Error type `UpdateCooldown` with signature `UpdateCooldown(uint256)` and selector `0x00d42941`
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
    #[etherror(name = "UpdateCooldown", abi = "UpdateCooldown(uint256)")]
    pub struct UpdateCooldown {
        pub last_updated: ::ethers::core::types::U256,
    }
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
    pub enum ReserveOracleErrors {
        InvalidFeedLength(InvalidFeedLength),
        InvalidInitialization(InvalidInitialization),
        InvalidMaxChange(InvalidMaxChange),
        InvalidMinMax(InvalidMinMax),
        InvalidQuorum(InvalidQuorum),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        UpdateCooldown(UpdateCooldown),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ReserveOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidFeedLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFeedLength(decoded));
            }
            if let Ok(decoded) = <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <InvalidMaxChange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxChange(decoded));
            }
            if let Ok(decoded) = <InvalidMinMax as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinMax(decoded));
            }
            if let Ok(decoded) = <InvalidQuorum as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidQuorum(decoded));
            }
            if let Ok(decoded) = <MathOverflowedMulDiv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MathOverflowedMulDiv(decoded));
            }
            if let Ok(decoded) = <UpdateCooldown as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateCooldown(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReserveOracleErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidFeedLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxChange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinMax(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateCooldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ReserveOracleErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidFeedLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxChange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinMax as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidQuorum as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MathOverflowedMulDiv as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UpdateCooldown as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ReserveOracleErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidFeedLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMaxChange(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMinMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::MathOverflowedMulDiv(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateCooldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ReserveOracleErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidFeedLength> for ReserveOracleErrors {
        fn from(value: InvalidFeedLength) -> Self {
            Self::InvalidFeedLength(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for ReserveOracleErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<InvalidMaxChange> for ReserveOracleErrors {
        fn from(value: InvalidMaxChange) -> Self {
            Self::InvalidMaxChange(value)
        }
    }
    impl ::core::convert::From<InvalidMinMax> for ReserveOracleErrors {
        fn from(value: InvalidMinMax) -> Self {
            Self::InvalidMinMax(value)
        }
    }
    impl ::core::convert::From<InvalidQuorum> for ReserveOracleErrors {
        fn from(value: InvalidQuorum) -> Self {
            Self::InvalidQuorum(value)
        }
    }
    impl ::core::convert::From<MathOverflowedMulDiv> for ReserveOracleErrors {
        fn from(value: MathOverflowedMulDiv) -> Self {
            Self::MathOverflowedMulDiv(value)
        }
    }
    impl ::core::convert::From<UpdateCooldown> for ReserveOracleErrors {
        fn from(value: UpdateCooldown) -> Self {
            Self::UpdateCooldown(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "UpdateExchangeRate", abi = "UpdateExchangeRate(uint256)")]
    pub struct UpdateExchangeRateFilter {
        pub exchange_rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `FEED0` function with signature `FEED0()` and selector `0x36c03936`
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
    #[ethcall(name = "FEED0", abi = "FEED0()")]
    pub struct Feed0Call;
    ///Container type for all input parameters for the `FEED1` function with signature `FEED1()` and selector `0xbc04cb0f`
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
    #[ethcall(name = "FEED1", abi = "FEED1()")]
    pub struct Feed1Call;
    ///Container type for all input parameters for the `FEED2` function with signature `FEED2()` and selector `0x4be39f2f`
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
    #[ethcall(name = "FEED2", abi = "FEED2()")]
    pub struct Feed2Call;
    ///Container type for all input parameters for the `ILK_INDEX` function with signature `ILK_INDEX()` and selector `0xed0cee97`
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
    #[ethcall(name = "ILK_INDEX", abi = "ILK_INDEX()")]
    pub struct IlkIndexCall;
    ///Container type for all input parameters for the `MAX_CHANGE` function with signature `MAX_CHANGE()` and selector `0x954a53f9`
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
    #[ethcall(name = "MAX_CHANGE", abi = "MAX_CHANGE()")]
    pub struct MaxChangeCall;
    ///Container type for all input parameters for the `QUORUM` function with signature `QUORUM()` and selector `0x2e80d9b6`
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
    #[ethcall(name = "QUORUM", abi = "QUORUM()")]
    pub struct QuorumCall;
    ///Container type for all input parameters for the `currentExchangeRate` function with signature `currentExchangeRate()` and selector `0xa3684977`
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
    #[ethcall(name = "currentExchangeRate", abi = "currentExchangeRate()")]
    pub struct CurrentExchangeRateCall;
    ///Container type for all input parameters for the `getProtocolExchangeRate` function with signature `getProtocolExchangeRate()` and selector `0xf13597a6`
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
    #[ethcall(name = "getProtocolExchangeRate", abi = "getProtocolExchangeRate()")]
    pub struct GetProtocolExchangeRateCall;
    ///Container type for all input parameters for the `lastUpdated` function with signature `lastUpdated()` and selector `0xd0b06f5d`
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
    #[ethcall(name = "lastUpdated", abi = "lastUpdated()")]
    pub struct LastUpdatedCall;
    ///Container type for all input parameters for the `updateExchangeRate` function with signature `updateExchangeRate()` and selector `0x02ce728f`
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
    #[ethcall(name = "updateExchangeRate", abi = "updateExchangeRate()")]
    pub struct UpdateExchangeRateCall;
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
    pub enum ReserveOracleCalls {
        Feed0(Feed0Call),
        Feed1(Feed1Call),
        Feed2(Feed2Call),
        IlkIndex(IlkIndexCall),
        MaxChange(MaxChangeCall),
        Quorum(QuorumCall),
        CurrentExchangeRate(CurrentExchangeRateCall),
        GetProtocolExchangeRate(GetProtocolExchangeRateCall),
        LastUpdated(LastUpdatedCall),
        UpdateExchangeRate(UpdateExchangeRateCall),
    }
    impl ::ethers::core::abi::AbiDecode for ReserveOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Feed0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Feed0(decoded));
            }
            if let Ok(decoded) = <Feed1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Feed1(decoded));
            }
            if let Ok(decoded) = <Feed2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Feed2(decoded));
            }
            if let Ok(decoded) = <IlkIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IlkIndex(decoded));
            }
            if let Ok(decoded) = <MaxChangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxChange(decoded));
            }
            if let Ok(decoded) = <QuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quorum(decoded));
            }
            if let Ok(decoded) = <CurrentExchangeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentExchangeRate(decoded));
            }
            if let Ok(decoded) = <GetProtocolExchangeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProtocolExchangeRate(decoded));
            }
            if let Ok(decoded) = <LastUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastUpdated(decoded));
            }
            if let Ok(decoded) = <UpdateExchangeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateExchangeRate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReserveOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Feed0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Feed1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Feed2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IlkIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxChange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Quorum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CurrentExchangeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProtocolExchangeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateExchangeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ReserveOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Feed0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Feed1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Feed2(element) => ::core::fmt::Display::fmt(element, f),
                Self::IlkIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxChange(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentExchangeRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProtocolExchangeRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastUpdated(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateExchangeRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Feed0Call> for ReserveOracleCalls {
        fn from(value: Feed0Call) -> Self {
            Self::Feed0(value)
        }
    }
    impl ::core::convert::From<Feed1Call> for ReserveOracleCalls {
        fn from(value: Feed1Call) -> Self {
            Self::Feed1(value)
        }
    }
    impl ::core::convert::From<Feed2Call> for ReserveOracleCalls {
        fn from(value: Feed2Call) -> Self {
            Self::Feed2(value)
        }
    }
    impl ::core::convert::From<IlkIndexCall> for ReserveOracleCalls {
        fn from(value: IlkIndexCall) -> Self {
            Self::IlkIndex(value)
        }
    }
    impl ::core::convert::From<MaxChangeCall> for ReserveOracleCalls {
        fn from(value: MaxChangeCall) -> Self {
            Self::MaxChange(value)
        }
    }
    impl ::core::convert::From<QuorumCall> for ReserveOracleCalls {
        fn from(value: QuorumCall) -> Self {
            Self::Quorum(value)
        }
    }
    impl ::core::convert::From<CurrentExchangeRateCall> for ReserveOracleCalls {
        fn from(value: CurrentExchangeRateCall) -> Self {
            Self::CurrentExchangeRate(value)
        }
    }
    impl ::core::convert::From<GetProtocolExchangeRateCall> for ReserveOracleCalls {
        fn from(value: GetProtocolExchangeRateCall) -> Self {
            Self::GetProtocolExchangeRate(value)
        }
    }
    impl ::core::convert::From<LastUpdatedCall> for ReserveOracleCalls {
        fn from(value: LastUpdatedCall) -> Self {
            Self::LastUpdated(value)
        }
    }
    impl ::core::convert::From<UpdateExchangeRateCall> for ReserveOracleCalls {
        fn from(value: UpdateExchangeRateCall) -> Self {
            Self::UpdateExchangeRate(value)
        }
    }
    ///Container type for all return fields from the `FEED0` function with signature `FEED0()` and selector `0x36c03936`
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
    pub struct Feed0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `FEED1` function with signature `FEED1()` and selector `0xbc04cb0f`
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
    pub struct Feed1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `FEED2` function with signature `FEED2()` and selector `0x4be39f2f`
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
    pub struct Feed2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ILK_INDEX` function with signature `ILK_INDEX()` and selector `0xed0cee97`
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
    pub struct IlkIndexReturn(pub u8);
    ///Container type for all return fields from the `MAX_CHANGE` function with signature `MAX_CHANGE()` and selector `0x954a53f9`
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
    pub struct MaxChangeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `QUORUM` function with signature `QUORUM()` and selector `0x2e80d9b6`
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
    pub struct QuorumReturn(pub u8);
    ///Container type for all return fields from the `currentExchangeRate` function with signature `currentExchangeRate()` and selector `0xa3684977`
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
    pub struct CurrentExchangeRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getProtocolExchangeRate` function with signature `getProtocolExchangeRate()` and selector `0xf13597a6`
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
    pub struct GetProtocolExchangeRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastUpdated` function with signature `lastUpdated()` and selector `0xd0b06f5d`
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
    pub struct LastUpdatedReturn(pub ::ethers::core::types::U256);
}
