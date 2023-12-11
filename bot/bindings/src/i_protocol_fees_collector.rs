pub use i_protocol_fees_collector::*;
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
pub mod i_protocol_fees_collector {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAuthorizer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAuthorizer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAuthorizer"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCollectedFeeAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCollectedFeeAmounts",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeAmounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFlashLoanFeePercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getFlashLoanFeePercentage",
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
                    ::std::borrow::ToOwned::to_owned("getSwapFeePercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSwapFeePercentage",
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
                    ::std::borrow::ToOwned::to_owned("setFlashLoanFeePercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setFlashLoanFeePercentage",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFlashLoanFeePercentage",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSwapFeePercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setSwapFeePercentage",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSwapFeePercentage",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vault"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVault"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawCollectedFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawCollectedFees",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FlashLoanFeePercentageChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FlashLoanFeePercentageChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFlashLoanFeePercentage",
                                    ),
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
                (
                    ::std::borrow::ToOwned::to_owned("SwapFeePercentageChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapFeePercentageChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSwapFeePercentage",
                                    ),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IPROTOCOLFEESCOLLECTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IProtocolFeesCollector<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IProtocolFeesCollector<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IProtocolFeesCollector<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IProtocolFeesCollector<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IProtocolFeesCollector<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IProtocolFeesCollector))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IProtocolFeesCollector<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPROTOCOLFEESCOLLECTOR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getAuthorizer` (0xaaabadc5) function
        pub fn get_authorizer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([170, 171, 173, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCollectedFeeAmounts` (0xe42abf35) function
        pub fn get_collected_fee_amounts(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([228, 42, 191, 53], tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFlashLoanFeePercentage` (0xd877845c) function
        pub fn get_flash_loan_fee_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([216, 119, 132, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapFeePercentage` (0x55c67628) function
        pub fn get_swap_fee_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 198, 118, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFlashLoanFeePercentage` (0x6b6b9f69) function
        pub fn set_flash_loan_fee_percentage(
            &self,
            new_flash_loan_fee_percentage: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 107, 159, 105], new_flash_loan_fee_percentage)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSwapFeePercentage` (0x38e9922e) function
        pub fn set_swap_fee_percentage(
            &self,
            new_swap_fee_percentage: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 233, 146, 46], new_swap_fee_percentage)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vault` (0xfbfa77cf) function
        pub fn vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([251, 250, 119, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawCollectedFees` (0x6daefab6) function
        pub fn withdraw_collected_fees(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 174, 250, 182], (tokens, amounts, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FlashLoanFeePercentageChanged` event
        pub fn flash_loan_fee_percentage_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FlashLoanFeePercentageChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SwapFeePercentageChanged` event
        pub fn swap_fee_percentage_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapFeePercentageChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IProtocolFeesCollectorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IProtocolFeesCollector<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(
        name = "FlashLoanFeePercentageChanged",
        abi = "FlashLoanFeePercentageChanged(uint256)"
    )]
    pub struct FlashLoanFeePercentageChangedFilter {
        pub new_flash_loan_fee_percentage: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "SwapFeePercentageChanged",
        abi = "SwapFeePercentageChanged(uint256)"
    )]
    pub struct SwapFeePercentageChangedFilter {
        pub new_swap_fee_percentage: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
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
    pub enum IProtocolFeesCollectorEvents {
        FlashLoanFeePercentageChangedFilter(FlashLoanFeePercentageChangedFilter),
        SwapFeePercentageChangedFilter(SwapFeePercentageChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IProtocolFeesCollectorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FlashLoanFeePercentageChangedFilter::decode_log(log) {
                return Ok(
                    IProtocolFeesCollectorEvents::FlashLoanFeePercentageChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = SwapFeePercentageChangedFilter::decode_log(log) {
                return Ok(
                    IProtocolFeesCollectorEvents::SwapFeePercentageChangedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IProtocolFeesCollectorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FlashLoanFeePercentageChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFeePercentageChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FlashLoanFeePercentageChangedFilter>
    for IProtocolFeesCollectorEvents {
        fn from(value: FlashLoanFeePercentageChangedFilter) -> Self {
            Self::FlashLoanFeePercentageChangedFilter(value)
        }
    }
    impl ::core::convert::From<SwapFeePercentageChangedFilter>
    for IProtocolFeesCollectorEvents {
        fn from(value: SwapFeePercentageChangedFilter) -> Self {
            Self::SwapFeePercentageChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
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
    #[ethcall(name = "getAuthorizer", abi = "getAuthorizer()")]
    pub struct GetAuthorizerCall;
    ///Container type for all input parameters for the `getCollectedFeeAmounts` function with signature `getCollectedFeeAmounts(address[])` and selector `0xe42abf35`
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
    #[ethcall(
        name = "getCollectedFeeAmounts",
        abi = "getCollectedFeeAmounts(address[])"
    )]
    pub struct GetCollectedFeeAmountsCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getFlashLoanFeePercentage` function with signature `getFlashLoanFeePercentage()` and selector `0xd877845c`
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
    #[ethcall(name = "getFlashLoanFeePercentage", abi = "getFlashLoanFeePercentage()")]
    pub struct GetFlashLoanFeePercentageCall;
    ///Container type for all input parameters for the `getSwapFeePercentage` function with signature `getSwapFeePercentage()` and selector `0x55c67628`
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
    #[ethcall(name = "getSwapFeePercentage", abi = "getSwapFeePercentage()")]
    pub struct GetSwapFeePercentageCall;
    ///Container type for all input parameters for the `setFlashLoanFeePercentage` function with signature `setFlashLoanFeePercentage(uint256)` and selector `0x6b6b9f69`
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
    #[ethcall(
        name = "setFlashLoanFeePercentage",
        abi = "setFlashLoanFeePercentage(uint256)"
    )]
    pub struct SetFlashLoanFeePercentageCall {
        pub new_flash_loan_fee_percentage: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setSwapFeePercentage` function with signature `setSwapFeePercentage(uint256)` and selector `0x38e9922e`
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
    #[ethcall(name = "setSwapFeePercentage", abi = "setSwapFeePercentage(uint256)")]
    pub struct SetSwapFeePercentageCall {
        pub new_swap_fee_percentage: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `vault` function with signature `vault()` and selector `0xfbfa77cf`
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
    #[ethcall(name = "vault", abi = "vault()")]
    pub struct VaultCall;
    ///Container type for all input parameters for the `withdrawCollectedFees` function with signature `withdrawCollectedFees(address[],uint256[],address)` and selector `0x6daefab6`
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
    #[ethcall(
        name = "withdrawCollectedFees",
        abi = "withdrawCollectedFees(address[],uint256[],address)"
    )]
    pub struct WithdrawCollectedFeesCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub recipient: ::ethers::core::types::Address,
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
    pub enum IProtocolFeesCollectorCalls {
        GetAuthorizer(GetAuthorizerCall),
        GetCollectedFeeAmounts(GetCollectedFeeAmountsCall),
        GetFlashLoanFeePercentage(GetFlashLoanFeePercentageCall),
        GetSwapFeePercentage(GetSwapFeePercentageCall),
        SetFlashLoanFeePercentage(SetFlashLoanFeePercentageCall),
        SetSwapFeePercentage(SetSwapFeePercentageCall),
        Vault(VaultCall),
        WithdrawCollectedFees(WithdrawCollectedFeesCall),
    }
    impl ::ethers::core::abi::AbiDecode for IProtocolFeesCollectorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAuthorizer(decoded));
            }
            if let Ok(decoded) = <GetCollectedFeeAmountsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCollectedFeeAmounts(decoded));
            }
            if let Ok(decoded) = <GetFlashLoanFeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFlashLoanFeePercentage(decoded));
            }
            if let Ok(decoded) = <GetSwapFeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapFeePercentage(decoded));
            }
            if let Ok(decoded) = <SetFlashLoanFeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFlashLoanFeePercentage(decoded));
            }
            if let Ok(decoded) = <SetSwapFeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSwapFeePercentage(decoded));
            }
            if let Ok(decoded) = <VaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Vault(decoded));
            }
            if let Ok(decoded) = <WithdrawCollectedFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawCollectedFees(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IProtocolFeesCollectorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAuthorizer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollectedFeeAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFlashLoanFeePercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapFeePercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFlashLoanFeePercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSwapFeePercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vault(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawCollectedFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IProtocolFeesCollectorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAuthorizer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollectedFeeAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFlashLoanFeePercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSwapFeePercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFlashLoanFeePercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSwapFeePercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Vault(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawCollectedFees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetAuthorizerCall> for IProtocolFeesCollectorCalls {
        fn from(value: GetAuthorizerCall) -> Self {
            Self::GetAuthorizer(value)
        }
    }
    impl ::core::convert::From<GetCollectedFeeAmountsCall>
    for IProtocolFeesCollectorCalls {
        fn from(value: GetCollectedFeeAmountsCall) -> Self {
            Self::GetCollectedFeeAmounts(value)
        }
    }
    impl ::core::convert::From<GetFlashLoanFeePercentageCall>
    for IProtocolFeesCollectorCalls {
        fn from(value: GetFlashLoanFeePercentageCall) -> Self {
            Self::GetFlashLoanFeePercentage(value)
        }
    }
    impl ::core::convert::From<GetSwapFeePercentageCall>
    for IProtocolFeesCollectorCalls {
        fn from(value: GetSwapFeePercentageCall) -> Self {
            Self::GetSwapFeePercentage(value)
        }
    }
    impl ::core::convert::From<SetFlashLoanFeePercentageCall>
    for IProtocolFeesCollectorCalls {
        fn from(value: SetFlashLoanFeePercentageCall) -> Self {
            Self::SetFlashLoanFeePercentage(value)
        }
    }
    impl ::core::convert::From<SetSwapFeePercentageCall>
    for IProtocolFeesCollectorCalls {
        fn from(value: SetSwapFeePercentageCall) -> Self {
            Self::SetSwapFeePercentage(value)
        }
    }
    impl ::core::convert::From<VaultCall> for IProtocolFeesCollectorCalls {
        fn from(value: VaultCall) -> Self {
            Self::Vault(value)
        }
    }
    impl ::core::convert::From<WithdrawCollectedFeesCall>
    for IProtocolFeesCollectorCalls {
        fn from(value: WithdrawCollectedFeesCall) -> Self {
            Self::WithdrawCollectedFees(value)
        }
    }
    ///Container type for all return fields from the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
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
    pub struct GetAuthorizerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getCollectedFeeAmounts` function with signature `getCollectedFeeAmounts(address[])` and selector `0xe42abf35`
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
    pub struct GetCollectedFeeAmountsReturn {
        pub fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getFlashLoanFeePercentage` function with signature `getFlashLoanFeePercentage()` and selector `0xd877845c`
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
    pub struct GetFlashLoanFeePercentageReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSwapFeePercentage` function with signature `getSwapFeePercentage()` and selector `0x55c67628`
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
    pub struct GetSwapFeePercentageReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vault` function with signature `vault()` and selector `0xfbfa77cf`
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
    pub struct VaultReturn(pub ::ethers::core::types::Address);
}
