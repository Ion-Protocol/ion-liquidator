pub use liquidation::*;
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
pub mod liquidation {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_ionPool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_protocol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reserveOracles"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_liquidationThresholds"),
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
                        name: ::std::borrow::ToOwned::to_owned("_targetHealth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reserveFactor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_maxDiscount"),
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
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BASE_DISCOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_DISCOUNT"),
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
                    ::std::borrow::ToOwned::to_owned("LIQUIDATION_THRESHOLD_0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LIQUIDATION_THRESHOLD_0",
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
                    ::std::borrow::ToOwned::to_owned("LIQUIDATION_THRESHOLD_1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LIQUIDATION_THRESHOLD_1",
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
                    ::std::borrow::ToOwned::to_owned("LIQUIDATION_THRESHOLD_2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LIQUIDATION_THRESHOLD_2",
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
                    ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT_0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT_0"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT_1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT_1"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT_2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT_2"),
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
                    ::std::borrow::ToOwned::to_owned("POOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IonPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PROTOCOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PROTOCOL"),
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
                    ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE_0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE_0"),
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
                    ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE_1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE_1"),
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
                    ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE_2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RESERVE_ORACLE_2"),
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
                    ::std::borrow::ToOwned::to_owned("TARGET_HEALTH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TARGET_HEALTH"),
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
                    ::std::borrow::ToOwned::to_owned("UNDERLYING"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("UNDERLYING"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRepayAmt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRepayAmt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vault"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repay"),
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
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vault"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kpr"),
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
                    ::std::borrow::ToOwned::to_owned("Liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("kpr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("repay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gemOut"),
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
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExchangeRateCannotBeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExchangeRateCannotBeZero",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "InvalidLiquidationThresholdsLength",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidLiquidationThresholdsLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidReserveOraclesLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidReserveOraclesLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
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
                    ::std::borrow::ToOwned::to_owned("LiquidationThresholdCannotBeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LiquidationThresholdCannotBeZero",
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
                (
                    ::std::borrow::ToOwned::to_owned("NotScalingUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotScalingUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("SafeCastOverflowedUintToInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeCastOverflowedUintToInt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VaultIsNotUnsafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("VaultIsNotUnsafe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("healthRatio"),
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
    pub static LIQUIDATION_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x02@`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\x000Y8\x03\x80b\x000Y\x839\x81\x81\x01`@R\x81\x01\x90b\0\08\x91\x90b\0\x08\x01V[`\0\x87\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0a\x02\0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB6N\0\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\x1E\x91\x90b\0\t\x11V[\x90P\x80\x87Q\x14b\0\x01iW\x86Q`@Q\x7FDh\xC4\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x01`\x91\x90b\0\tTV[`@Q\x80\x91\x03\x90\xFD[\x80\x86Q\x14b\0\x01\xB2W\x85Q`@Q\x7F6\xEB\x1E\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x01\xA9\x91\x90b\0\tTV[`@Q\x80\x91\x03\x90\xFD[\x84`\x80\x81\x81RPP\x83`\xA0\x81\x81RPP\x82`\0\x81Q\x81\x10b\0\x01\xD9Wb\0\x01\xD8b\0\tqV[[` \x02` \x01\x01Q`\xC0\x81\x81RPP\x82`\x01\x81Q\x81\x10b\0\x01\xFFWb\0\x01\xFEb\0\tqV[[` \x02` \x01\x01Q`\xE0\x81\x81RPP\x82`\x02\x81Q\x81\x10b\0\x02%Wb\0\x02$b\0\tqV[[` \x02` \x01\x01Qa\x01\0\x81\x81RPP`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x02\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02\xA9\x91\x90b\0\t\xE5V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x03\x08\x92\x91\x90b\0\n(V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x03(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03N\x91\x90b\0\n\x92V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02 \x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x86`\0\x81Q\x81\x10b\0\x03\x9BWb\0\x03\x9Ab\0\tqV[[` \x02` \x01\x01Qa\x01 \x81\x81RPP\x86`\x01\x81Q\x81\x10b\0\x03\xC2Wb\0\x03\xC1b\0\tqV[[` \x02` \x01\x01Qa\x01@\x81\x81RPP\x86`\x02\x81Q\x81\x10b\0\x03\xE9Wb\0\x03\xE8b\0\tqV[[` \x02` \x01\x01Qa\x01`\x81\x81RPP\x87`\0\x81Q\x81\x10b\0\x04\x10Wb\0\x04\x0Fb\0\tqV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x87`\x01\x81Q\x81\x10b\0\x04cWb\0\x04bb\0\tqV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x87`\x02\x81Q\x81\x10b\0\x04\xB6Wb\0\x04\xB5b\0\tqV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPPPPPPPPb\0\n\xC4V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x05C\x82b\0\x05\x16V[\x90P\x91\x90PV[b\0\x05U\x81b\0\x056V[\x81\x14b\0\x05aW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x05u\x81b\0\x05JV[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x05\xCB\x82b\0\x05\x80V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x05\xEDWb\0\x05\xECb\0\x05\x91V[[\x80`@RPPPV[`\0b\0\x06\x02b\0\x05\x02V[\x90Pb\0\x06\x10\x82\x82b\0\x05\xC0V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x063Wb\0\x062b\0\x05\x91V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0b\0\x06`b\0\x06Z\x84b\0\x06\x15V[b\0\x05\xF6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x06\x86Wb\0\x06\x85b\0\x06DV[[\x83[\x81\x81\x10\x15b\0\x06\xB3W\x80b\0\x06\x9E\x88\x82b\0\x05dV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x06\x88V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x06\xD5Wb\0\x06\xD4b\0\x05{V[[\x81Qb\0\x06\xE7\x84\x82` \x86\x01b\0\x06IV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x07\x0EWb\0\x07\rb\0\x05\x91V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[b\0\x074\x81b\0\x07\x1FV[\x81\x14b\0\x07@W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x07T\x81b\0\x07)V[\x92\x91PPV[`\0b\0\x07qb\0\x07k\x84b\0\x06\xF0V[b\0\x05\xF6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x07\x97Wb\0\x07\x96b\0\x06DV[[\x83[\x81\x81\x10\x15b\0\x07\xC4W\x80b\0\x07\xAF\x88\x82b\0\x07CV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x07\x99V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x07\xE6Wb\0\x07\xE5b\0\x05{V[[\x81Qb\0\x07\xF8\x84\x82` \x86\x01b\0\x07ZV[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15b\0\x08#Wb\0\x08\"b\0\x05\x0CV[[`\0b\0\x083\x8A\x82\x8B\x01b\0\x05dV[\x97PP` b\0\x08F\x8A\x82\x8B\x01b\0\x05dV[\x96PP`@\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x08jWb\0\x08ib\0\x05\x11V[[b\0\x08x\x8A\x82\x8B\x01b\0\x06\xBDV[\x95PP``\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x08\x9CWb\0\x08\x9Bb\0\x05\x11V[[b\0\x08\xAA\x8A\x82\x8B\x01b\0\x07\xCEV[\x94PP`\x80b\0\x08\xBD\x8A\x82\x8B\x01b\0\x07CV[\x93PP`\xA0b\0\x08\xD0\x8A\x82\x8B\x01b\0\x07CV[\x92PP`\xC0\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x08\xF4Wb\0\x08\xF3b\0\x05\x11V[[b\0\t\x02\x8A\x82\x8B\x01b\0\x07\xCEV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x82\x84\x03\x12\x15b\0\t*Wb\0\t)b\0\x05\x0CV[[`\0b\0\t:\x84\x82\x85\x01b\0\x07CV[\x91PP\x92\x91PPV[b\0\tN\x81b\0\x07\x1FV[\x82RPPV[`\0` \x82\x01\x90Pb\0\tk`\0\x83\x01\x84b\0\tCV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0b\0\t\xAD\x82b\0\x056V[\x90P\x91\x90PV[b\0\t\xBF\x81b\0\t\xA0V[\x81\x14b\0\t\xCBW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\t\xDF\x81b\0\t\xB4V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\t\xFEWb\0\t\xFDb\0\x05\x0CV[[`\0b\0\n\x0E\x84\x82\x85\x01b\0\t\xCEV[\x91PP\x92\x91PPV[b\0\n\"\x81b\0\x056V[\x82RPPV[`\0`@\x82\x01\x90Pb\0\n?`\0\x83\x01\x85b\0\n\x17V[b\0\nN` \x83\x01\x84b\0\tCV[\x93\x92PPPV[`\0\x81\x15\x15\x90P\x91\x90PV[b\0\nl\x81b\0\nUV[\x81\x14b\0\nxW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\n\x8C\x81b\0\naV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\n\xABWb\0\n\xAAb\0\x05\x0CV[[`\0b\0\n\xBB\x84\x82\x85\x01b\0\n{V[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x01\xE0Qa\x02\0Qa\x02 Qa$Eb\0\x0C\x14`\09`\0\x81\x81a\nB\x01Ra\x11E\x01R`\0\x81\x81a\x03\xE1\x01R\x81\x81a\x04\x82\x01R\x81\x81a\x07\x13\x01R\x81\x81a\x08j\x01R\x81\x81a\n\x89\x01R\x81\x81a\x0BH\x01R\x81\x81a\x0C\xF7\x01R\x81\x81a\r\xFE\x01Ra\x0E\x9F\x01R`\0\x81\x81a\x07Q\x01R\x81\x81a\x07r\x01Ra\r\x1B\x01R`\0\x81\x81a\x0Cg\x01Ra\x13=\x01R`\0\x81\x81a\x0C\xD3\x01Ra\x12\x81\x01R`\0\x81\x81a\x0C\x8B\x01Ra\x11\xC5\x01R`\0\x81\x81a\r?\x01Ra\x13\x95\x01R`\0\x81\x81a\x11i\x01Ra\x12\xD9\x01R`\0\x81\x81a\x11\x8D\x01Ra\x12\x1D\x01R`\0\x81\x81a\x02\xF7\x01Ra\x13\xBF\x01R`\0\x81\x81a\x0C\xAF\x01Ra\x13\x03\x01R`\0\x81\x81a\x03\x1B\x01Ra\x12G\x01R`\0\x81\x81a\x06H\x01R\x81\x81a\x0CC\x01Ra\x10e\x01R`\0\x81\x81a\x11!\x01R\x81\x81a\x14\x93\x01Ra\x14\xFB\x01Ra$E`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80cu5\xD2F\x11a\0\x97W\x80c\xAES\x953\x11a\0fW\x80c\xAES\x953\x14a\x02}W\x80c\xC5\xD6d\xC6\x14a\x02\x9BW\x80c\xE1S1\xCC\x14a\x02\xB9W\x80c\xE7-\xC8^\x14a\x02\xD7Wa\x01\0V[\x80cu5\xD2F\x14a\x01\xF3W\x80c\x91\xB9\xB8'\x14a\x02\x11W\x80c\x9A}\xDA\x8D\x14a\x02/W\x80c\xA0\xD5\xF5\x99\x14a\x02MWa\x01\0V[\x80cM\x19i\x84\x11a\0\xD3W\x80cM\x19i\x84\x14a\x01{W\x80cU)\xB7\x8A\x14a\x01\x99W\x80cU*\xBBm\x14a\x01\xB7W\x80cc Ua\x14a\x01\xD5Wa\x01\0V[\x80c\x1AY\x84\xB6\x14a\x01\x05W\x80c\x1A\xE7C\x18\x14a\x01#W\x80c%\x84\x0E\xDA\x14a\x01AW\x80cI\xEDd4\x14a\x01]W[`\0\x80\xFD[a\x01\ra\x02\xF5V[`@Qa\x01\x1A\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01+a\x03\x19V[`@Qa\x018\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01[`\x04\x806\x03\x81\x01\x90a\x01V\x91\x90a\x1B\xE8V[a\x03=V[\0[a\x01ea\x0CAV[`@Qa\x01r\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01\x83a\x0CeV[`@Qa\x01\x90\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x01\xA1a\x0C\x89V[`@Qa\x01\xAE\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x01\xBFa\x0C\xADV[`@Qa\x01\xCC\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDDa\x0C\xD1V[`@Qa\x01\xEA\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x01\xFBa\x0C\xF5V[`@Qa\x02\x08\x91\x90a\x1C\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02\x19a\r\x19V[`@Qa\x02&\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x027a\r=V[`@Qa\x02D\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02g`\x04\x806\x03\x81\x01\x90a\x02b\x91\x90a\x1C\xDFV[a\raV[`@Qa\x02t\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02\x85a\x11\x1FV[`@Qa\x02\x92\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA3a\x11CV[`@Qa\x02\xB0\x91\x90a\x1D@V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC1a\x11gV[`@Qa\x02\xCE\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDFa\x11\x8BV[`@Qa\x02\xEC\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ea\x1A\xB2V[`\0a\x03P\x85a\x11\xAFV[\x90P`\0a\x03\xDA`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCC\x91\x90a\x1D\x87V[a\x13\xF0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04:\x92\x91\x90a\x1D\xC3V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04z\x91\x90a\x1D\xECV[\x91P\x91P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xD9\x91\x90a\x1E,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1A\x91\x90a\x1D\x87V[\x90P`\0\x84\x03a\x05VW`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x85`\0\x01Q\x03a\x05\x94W`@Q\x7Fj\x87\xC8\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\xB8\x86`\0\x01Q\x86\x86a\x05\xAA\x91\x90a\x1EvV[a\x14\x06\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x05\xDA\x83\x85a\x05\xCB\x91\x90a\x1EvV[\x83a\x141\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x06,W\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06#\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xFD[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x06F\x91\x90a\x1E\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06q\x91\x90a\x1E\xECV[\x90P\x87` \x01Q\x81\x11\x15a\x06\x89W\x87` \x01Qa\x06\x8BV[\x80[\x90Pa\x06\xB7\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x06\xA8\x91\x90a\x1E\xB8V[\x88a\x14\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01\x81\x81RPPa\x06\xDB\x84\x86a\x06\xCF\x91\x90a\x1EvV[\x84\x8A`\0\x01Q\x84a\x14\x8AV[\x89`\0\x01\x81\x81RPPPP\x81\x83a\x06\xF2\x91\x90a\x1EvV[\x87`\0\x01Q\x11\x15a\x08hW\x82\x87`@\x01\x81\x81RPP\x83\x87` \x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8B\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C` \x01Qa\x07\x9F\x90a\x1F*V[\x8D`@\x01Qa\x07\xAD\x90a\x1F*V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xCE\x96\x95\x94\x93\x92\x91\x90a\x1F\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xFCW=`\0\x80>=`\0\xFD[PPPP\x89`\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x99\x87\xF2\x94\xB3\xBD\xD0P\xBD\x9At\xD3\xEBX\xD0>\x11qn\xAF\x87Vr_\xDE\xBCf\xD0T\xD9(\xE9\x89`@\x01Q\x8A` \x01Q`@Qa\x08T\x92\x91\x90a\x1F\xE2V[`@Q\x80\x91\x03\x90\xA3PPPPPPPa\x0C<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B\xA7e\x07\x8B`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xC1\x91\x90a\x1E,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x02\x91\x90a\x1D\x87V[\x87`\0\x01Q\x83\x85a\t\x13\x91\x90a\x1EvV[a\t\x1D\x91\x90a\x1E\xB8V[\x10\x15a\tkW\x81\x83a\t/\x91\x90a\x1EvV[\x87`\0\x01\x81\x81RPP\x82\x87`@\x01\x81\x81RPP\x86`\x80\x01Q\x82\x84a\tS\x91\x90a\x1EvV[a\t]\x91\x90a :V[\x87` \x01\x81\x81RPPa\t\xE9V[\x81\x87`\0\x01Qa\t{\x91\x90a :V[\x87`@\x01\x81\x81RPP`\0\x82\x88`\0\x01Qa\t\x96\x91\x90a kV[\x11\x15a\t\xB2W\x86`@\x01\x80Qa\t\xAB\x90a \x9CV[\x90\x81\x81RPP[\x86`\x80\x01Q\x87`\0\x01Qa\t\xC6\x91\x90a :V[\x87` \x01\x81\x81RPP\x81\x87`@\x01Qa\t\xDF\x91\x90a\x1EvV[\x87`\0\x01\x81\x81RPP[`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88`\0\x01Qa\n\x07\x91\x90a :V[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89`\0\x01Qa\n'\x91\x90a kV[\x11\x15a\n:W\x80a\n7\x90a \x9CV[\x90P[a\n\x8730\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15E\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8C\x8C\x8C0a\n\xD4\x8E` \x01Qa\x15\xC7V[a\n\xDD\x90a\x1F*V[a\n\xEA\x8F`@\x01Qa\x15\xC7V[a\n\xF3\x90a\x1F*V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x14\x96\x95\x94\x93\x92\x91\x90a\x1F\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B.W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0BBW=`\0\x80>=`\0\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD81\xEF\xD80\x8A`\0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xA5\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xD3W=`\0\x80>=`\0\xFD[PPPP\x8A`\xFF\x16\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x99\x87\xF2\x94\xB3\xBD\xD0P\xBD\x9At\xD3\xEBX\xD0>\x11qn\xAF\x87Vr_\xDE\xBCf\xD0T\xD9(\xE9\x8A`@\x01Q\x8B` \x01Q`@Qa\x0C+\x92\x91\x90a\x1F\xE2V[`@Q\x80\x91\x03\x90\xA3PPPPPPPP[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\rm\x84a\x11\xAFV[\x90P`\0a\r\xF7`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xE9\x91\x90a\x1D\x87V[a\x13\xF0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x88\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0EW\x92\x91\x90a\x1D\xC3V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x97\x91\x90a\x1D\xECV[\x91P\x91P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xF6\x91\x90a\x1E,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F7\x91\x90a\x1D\x87V[\x90P`\0\x84\x03a\x0FsW`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x85`\0\x01Q\x03a\x0F\xB1W`@Q\x7Fj\x87\xC8\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0F\xD5\x86`\0\x01Q\x86\x86a\x0F\xC7\x91\x90a\x1EvV[a\x14\x06\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0F\xF7\x83\x85a\x0F\xE8\x91\x90a\x1EvV[\x83a\x141\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x10IW\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10@\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xFD[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x10c\x91\x90a\x1E\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x8E\x91\x90a\x1E\xECV[\x90P\x87` \x01Q\x81\x11\x15a\x10\xA6W\x87` \x01Qa\x10\xA8V[\x80[\x90P`\0a\x10\xC7\x85\x87a\x10\xBB\x91\x90a\x1EvV[\x85\x8B`\0\x01Q\x85a\x14\x8AV[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81a\x10\xE1\x91\x90a :V[\x99P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82a\x10\xFD\x91\x90a kV[\x11\x15a\x11\x10W\x89a\x11\r\x90a \x9CV[\x99P[PPPPPPPPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x11\xB7a\x1A\xE1V[`\0\x80\x83`\xFF\x16\x03a\x12tW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01\x81\x81RPPa\x13\xEAV[`\x01\x83`\xFF\x16\x03a\x130W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01\x81\x81RPPa\x13\xE9V[`\x02\x83`\xFF\x16\x03a\x13\xE8W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01\x81\x81RPP[[[P\x91\x90PV[`\0a\x13\xFE\x83\x83`\x1Ba\x166V[\x90P\x92\x91PPV[`\0a\x14)\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x16\xAA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x14Tk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x16\xAA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x14\x82\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x17\xB1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80\x84a\x14\xC1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x14\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x14\xCB\x91\x90a\x1E\xB8V[\x90P`\0a\x14\xF9\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x14\xEA\x91\x90a\x1E\xB8V[\x86a\x18\x08\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x15$\x91\x90a\x1E\xB8V[\x90Pa\x159\x81\x83a\x18\x08\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x94\x93PPPPV[a\x15\xC1\x84\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a\x15z\x93\x92\x91\x90a!\rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x186V[PPPPV[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x16.W\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16%\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x81\x83\x10a\x16~W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16u\x92\x91\x90a\x1F\xE2V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x16\x8A\x91\x90a\x1E\xB8V[`\na\x16\x96\x91\x90a\"wV[\x84a\x16\xA1\x91\x90a\x1EvV[\x90P\x93\x92PPPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x16\xE5W\x83\x82\x81a\x16\xDBWa\x16\xDAa \x0BV[[\x04\x92PPPa\x17\xAAV[\x80\x84\x11a\x17\x1EW`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0\x80a\x17\xBF\x86\x86\x86a\x16\xAAV[\x90Pa\x17\xCA\x83a\x18\xCDV[\x80\x15a\x17\xE7WP`\0\x84\x80a\x17\xE2Wa\x17\xE1a \x0BV[[\x86\x88\t\x11[\x15a\x17\xFCW`\x01\x81a\x17\xF9\x91\x90a\x1E\xECV[\x90P[\x80\x91PP\x94\x93PPPPV[`\0a\x18.k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x17\xB1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x18a\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\x18\x86WP\x80\x80` \x01\x90Q\x81\x01\x90a\x18\x84\x91\x90a\"\xFAV[\x15[\x15a\x18\xC8W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xBF\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xFD[PPPV[`\0`\x01`\x02\x83`\x03\x81\x11\x15a\x18\xE6Wa\x18\xE5a#'V[[a\x18\xF0\x91\x90a#VV[`\xFF\x16\x14\x90P\x91\x90PV[``a\x19\t\x83\x83`\0a\x19\x11V[\x90P\x92\x91PPV[``\x81G\x10\x15a\x19XW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19O\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\x19\x81\x91\x90a#\xF8V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19\xBEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\xC3V[``\x91P[P\x91P\x91Pa\x19\xD3\x86\x83\x83a\x19\xDEV[\x92PPP\x93\x92PPPV[``\x82a\x19\xF3Wa\x19\xEE\x82a\x1AmV[a\x1AeV[`\0\x82Q\x14\x80\x15a\x1A\x1BWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x1A]W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1AT\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x1AfV[[\x93\x92PPPV[`\0\x81Q\x11\x15a\x1A\x80W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0\x81\x90P\x91\x90PV[a\x1B+\x81a\x1B\x18V[\x82RPPV[`\0` \x82\x01\x90Pa\x1BF`\0\x83\x01\x84a\x1B\"V[\x92\x91PPV[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x1Bg\x81a\x1BQV[\x81\x14a\x1BrW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\x84\x81a\x1B^V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1B\xB5\x82a\x1B\x8AV[\x90P\x91\x90PV[a\x1B\xC5\x81a\x1B\xAAV[\x81\x14a\x1B\xD0W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\xE2\x81a\x1B\xBCV[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\x01Wa\x1C\0a\x1BLV[[`\0a\x1C\x0F\x86\x82\x87\x01a\x1BuV[\x93PP` a\x1C \x86\x82\x87\x01a\x1B\xD3V[\x92PP`@a\x1C1\x86\x82\x87\x01a\x1B\xD3V[\x91PP\x92P\x92P\x92V[a\x1CD\x81a\x1B\xAAV[\x82RPPV[`\0` \x82\x01\x90Pa\x1C_`\0\x83\x01\x84a\x1C;V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x1C\x8Aa\x1C\x85a\x1C\x80\x84a\x1B\x8AV[a\x1CeV[a\x1B\x8AV[\x90P\x91\x90PV[`\0a\x1C\x9C\x82a\x1CoV[\x90P\x91\x90PV[`\0a\x1C\xAE\x82a\x1C\x91V[\x90P\x91\x90PV[a\x1C\xBE\x81a\x1C\xA3V[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\xD9`\0\x83\x01\x84a\x1C\xB5V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xF6Wa\x1C\xF5a\x1BLV[[`\0a\x1D\x04\x85\x82\x86\x01a\x1BuV[\x92PP` a\x1D\x15\x85\x82\x86\x01a\x1B\xD3V[\x91PP\x92P\x92\x90PV[`\0a\x1D*\x82a\x1C\x91V[\x90P\x91\x90PV[a\x1D:\x81a\x1D\x1FV[\x82RPPV[`\0` \x82\x01\x90Pa\x1DU`\0\x83\x01\x84a\x1D1V[\x92\x91PPV[a\x1Dd\x81a\x1B\x18V[\x81\x14a\x1DoW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x1D\x81\x81a\x1D[V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D\x9DWa\x1D\x9Ca\x1BLV[[`\0a\x1D\xAB\x84\x82\x85\x01a\x1DrV[\x91PP\x92\x91PPV[a\x1D\xBD\x81a\x1BQV[\x82RPPV[`\0`@\x82\x01\x90Pa\x1D\xD8`\0\x83\x01\x85a\x1D\xB4V[a\x1D\xE5` \x83\x01\x84a\x1C;V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\x03Wa\x1E\x02a\x1BLV[[`\0a\x1E\x11\x85\x82\x86\x01a\x1DrV[\x92PP` a\x1E\"\x85\x82\x86\x01a\x1DrV[\x91PP\x92P\x92\x90PV[`\0` \x82\x01\x90Pa\x1EA`\0\x83\x01\x84a\x1D\xB4V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x1E\x81\x82a\x1B\x18V[\x91Pa\x1E\x8C\x83a\x1B\x18V[\x92P\x82\x82\x02a\x1E\x9A\x81a\x1B\x18V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1E\xB1Wa\x1E\xB0a\x1EGV[[P\x92\x91PPV[`\0a\x1E\xC3\x82a\x1B\x18V[\x91Pa\x1E\xCE\x83a\x1B\x18V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1E\xE6Wa\x1E\xE5a\x1EGV[[\x92\x91PPV[`\0a\x1E\xF7\x82a\x1B\x18V[\x91Pa\x1F\x02\x83a\x1B\x18V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1F\x1AWa\x1F\x19a\x1EGV[[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x1F5\x82a\x1F V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1FgWa\x1Ffa\x1EGV[[\x81`\0\x03\x90P\x91\x90PV[a\x1F{\x81a\x1F V[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x1F\x96`\0\x83\x01\x89a\x1D\xB4V[a\x1F\xA3` \x83\x01\x88a\x1C;V[a\x1F\xB0`@\x83\x01\x87a\x1C;V[a\x1F\xBD``\x83\x01\x86a\x1C;V[a\x1F\xCA`\x80\x83\x01\x85a\x1FrV[a\x1F\xD7`\xA0\x83\x01\x84a\x1FrV[\x97\x96PPPPPPPV[`\0`@\x82\x01\x90Pa\x1F\xF7`\0\x83\x01\x85a\x1B\"V[a \x04` \x83\x01\x84a\x1B\"V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a E\x82a\x1B\x18V[\x91Pa P\x83a\x1B\x18V[\x92P\x82a `Wa _a \x0BV[[\x82\x82\x04\x90P\x92\x91PPV[`\0a v\x82a\x1B\x18V[\x91Pa \x81\x83a\x1B\x18V[\x92P\x82a \x91Wa \x90a \x0BV[[\x82\x82\x06\x90P\x92\x91PPV[`\0a \xA7\x82a\x1B\x18V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a \xD9Wa \xD8a\x1EGV[[`\x01\x82\x01\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa \xF9`\0\x83\x01\x85a\x1C;V[a!\x06` \x83\x01\x84a\x1B\"V[\x93\x92PPPV[`\0``\x82\x01\x90Pa!\"`\0\x83\x01\x86a\x1C;V[a!/` \x83\x01\x85a\x1C;V[a!<`@\x83\x01\x84a\x1B\"V[\x94\x93PPPPV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a!\x9BW\x80\x86\x04\x81\x11\x15a!wWa!va\x1EGV[[`\x01\x85\x16\x15a!\x86W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa!\x94\x85a!DV[\x94Pa![V[\x94P\x94\x92PPPV[`\0\x82a!\xB4W`\x01\x90Pa\"pV[\x81a!\xC2W`\0\x90Pa\"pV[\x81`\x01\x81\x14a!\xD8W`\x02\x81\x14a!\xE2Wa\"\x11V[`\x01\x91PPa\"pV[`\xFF\x84\x11\x15a!\xF4Wa!\xF3a\x1EGV[[\x83`\x02\n\x91P\x84\x82\x11\x15a\"\x0BWa\"\na\x1EGV[[Pa\"pV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\"FW\x82\x82\n\x90P\x83\x81\x11\x15a\"AWa\"@a\x1EGV[[a\"pV[a\"S\x84\x84\x84`\x01a!QV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\"jWa\"ia\x1EGV[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\"\x82\x82a\x1B\x18V[\x91Pa\"\x8D\x83a\x1B\x18V[\x92Pa\"\xBA\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a!\xA4V[\x90P\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\"\xD7\x81a\"\xC2V[\x81\x14a\"\xE2W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\"\xF4\x81a\"\xCEV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\x10Wa#\x0Fa\x1BLV[[`\0a#\x1E\x84\x82\x85\x01a\"\xE5V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0a#a\x82a\x1BQV[\x91Pa#l\x83a\x1BQV[\x92P\x82a#|Wa#{a \x0BV[[\x82\x82\x06\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a#\xBBW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa#\xA0V[`\0\x84\x84\x01RPPPPV[`\0a#\xD2\x82a#\x87V[a#\xDC\x81\x85a#\x92V[\x93Pa#\xEC\x81\x85` \x86\x01a#\x9DV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a$\x04\x82\x84a#\xC7V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 (\x8A\xC4\xD4\x17\x1E\xBA\xE6\xCB:\xB3O2P\x89\xC7\0\x02\xF2\xB2\x02\x9A\xC2\x8BX\xD9p\xF4\xDA\xD5TmdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80cu5\xD2F\x11a\0\x97W\x80c\xAES\x953\x11a\0fW\x80c\xAES\x953\x14a\x02}W\x80c\xC5\xD6d\xC6\x14a\x02\x9BW\x80c\xE1S1\xCC\x14a\x02\xB9W\x80c\xE7-\xC8^\x14a\x02\xD7Wa\x01\0V[\x80cu5\xD2F\x14a\x01\xF3W\x80c\x91\xB9\xB8'\x14a\x02\x11W\x80c\x9A}\xDA\x8D\x14a\x02/W\x80c\xA0\xD5\xF5\x99\x14a\x02MWa\x01\0V[\x80cM\x19i\x84\x11a\0\xD3W\x80cM\x19i\x84\x14a\x01{W\x80cU)\xB7\x8A\x14a\x01\x99W\x80cU*\xBBm\x14a\x01\xB7W\x80cc Ua\x14a\x01\xD5Wa\x01\0V[\x80c\x1AY\x84\xB6\x14a\x01\x05W\x80c\x1A\xE7C\x18\x14a\x01#W\x80c%\x84\x0E\xDA\x14a\x01AW\x80cI\xEDd4\x14a\x01]W[`\0\x80\xFD[a\x01\ra\x02\xF5V[`@Qa\x01\x1A\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01+a\x03\x19V[`@Qa\x018\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01[`\x04\x806\x03\x81\x01\x90a\x01V\x91\x90a\x1B\xE8V[a\x03=V[\0[a\x01ea\x0CAV[`@Qa\x01r\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01\x83a\x0CeV[`@Qa\x01\x90\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x01\xA1a\x0C\x89V[`@Qa\x01\xAE\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x01\xBFa\x0C\xADV[`@Qa\x01\xCC\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDDa\x0C\xD1V[`@Qa\x01\xEA\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x01\xFBa\x0C\xF5V[`@Qa\x02\x08\x91\x90a\x1C\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02\x19a\r\x19V[`@Qa\x02&\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xF3[a\x027a\r=V[`@Qa\x02D\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02g`\x04\x806\x03\x81\x01\x90a\x02b\x91\x90a\x1C\xDFV[a\raV[`@Qa\x02t\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02\x85a\x11\x1FV[`@Qa\x02\x92\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA3a\x11CV[`@Qa\x02\xB0\x91\x90a\x1D@V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC1a\x11gV[`@Qa\x02\xCE\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDFa\x11\x8BV[`@Qa\x02\xEC\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ea\x1A\xB2V[`\0a\x03P\x85a\x11\xAFV[\x90P`\0a\x03\xDA`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCC\x91\x90a\x1D\x87V[a\x13\xF0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04:\x92\x91\x90a\x1D\xC3V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04z\x91\x90a\x1D\xECV[\x91P\x91P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xD9\x91\x90a\x1E,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1A\x91\x90a\x1D\x87V[\x90P`\0\x84\x03a\x05VW`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x85`\0\x01Q\x03a\x05\x94W`@Q\x7Fj\x87\xC8\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\xB8\x86`\0\x01Q\x86\x86a\x05\xAA\x91\x90a\x1EvV[a\x14\x06\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x05\xDA\x83\x85a\x05\xCB\x91\x90a\x1EvV[\x83a\x141\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x06,W\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06#\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xFD[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x06F\x91\x90a\x1E\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06q\x91\x90a\x1E\xECV[\x90P\x87` \x01Q\x81\x11\x15a\x06\x89W\x87` \x01Qa\x06\x8BV[\x80[\x90Pa\x06\xB7\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x06\xA8\x91\x90a\x1E\xB8V[\x88a\x14\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01\x81\x81RPPa\x06\xDB\x84\x86a\x06\xCF\x91\x90a\x1EvV[\x84\x8A`\0\x01Q\x84a\x14\x8AV[\x89`\0\x01\x81\x81RPPPP\x81\x83a\x06\xF2\x91\x90a\x1EvV[\x87`\0\x01Q\x11\x15a\x08hW\x82\x87`@\x01\x81\x81RPP\x83\x87` \x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8B\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C` \x01Qa\x07\x9F\x90a\x1F*V[\x8D`@\x01Qa\x07\xAD\x90a\x1F*V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xCE\x96\x95\x94\x93\x92\x91\x90a\x1F\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xFCW=`\0\x80>=`\0\xFD[PPPP\x89`\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x99\x87\xF2\x94\xB3\xBD\xD0P\xBD\x9At\xD3\xEBX\xD0>\x11qn\xAF\x87Vr_\xDE\xBCf\xD0T\xD9(\xE9\x89`@\x01Q\x8A` \x01Q`@Qa\x08T\x92\x91\x90a\x1F\xE2V[`@Q\x80\x91\x03\x90\xA3PPPPPPPa\x0C<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B\xA7e\x07\x8B`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xC1\x91\x90a\x1E,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x02\x91\x90a\x1D\x87V[\x87`\0\x01Q\x83\x85a\t\x13\x91\x90a\x1EvV[a\t\x1D\x91\x90a\x1E\xB8V[\x10\x15a\tkW\x81\x83a\t/\x91\x90a\x1EvV[\x87`\0\x01\x81\x81RPP\x82\x87`@\x01\x81\x81RPP\x86`\x80\x01Q\x82\x84a\tS\x91\x90a\x1EvV[a\t]\x91\x90a :V[\x87` \x01\x81\x81RPPa\t\xE9V[\x81\x87`\0\x01Qa\t{\x91\x90a :V[\x87`@\x01\x81\x81RPP`\0\x82\x88`\0\x01Qa\t\x96\x91\x90a kV[\x11\x15a\t\xB2W\x86`@\x01\x80Qa\t\xAB\x90a \x9CV[\x90\x81\x81RPP[\x86`\x80\x01Q\x87`\0\x01Qa\t\xC6\x91\x90a :V[\x87` \x01\x81\x81RPP\x81\x87`@\x01Qa\t\xDF\x91\x90a\x1EvV[\x87`\0\x01\x81\x81RPP[`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88`\0\x01Qa\n\x07\x91\x90a :V[\x90P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89`\0\x01Qa\n'\x91\x90a kV[\x11\x15a\n:W\x80a\n7\x90a \x9CV[\x90P[a\n\x8730\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15E\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8C\x8C\x8C0a\n\xD4\x8E` \x01Qa\x15\xC7V[a\n\xDD\x90a\x1F*V[a\n\xEA\x8F`@\x01Qa\x15\xC7V[a\n\xF3\x90a\x1F*V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x14\x96\x95\x94\x93\x92\x91\x90a\x1F\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B.W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0BBW=`\0\x80>=`\0\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD81\xEF\xD80\x8A`\0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xA5\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xD3W=`\0\x80>=`\0\xFD[PPPP\x8A`\xFF\x16\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x99\x87\xF2\x94\xB3\xBD\xD0P\xBD\x9At\xD3\xEBX\xD0>\x11qn\xAF\x87Vr_\xDE\xBCf\xD0T\xD9(\xE9\x8A`@\x01Q\x8B` \x01Q`@Qa\x0C+\x92\x91\x90a\x1F\xE2V[`@Q\x80\x91\x03\x90\xA3PPPPPPPP[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\rm\x84a\x11\xAFV[\x90P`\0a\r\xF7`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xE9\x91\x90a\x1D\x87V[a\x13\xF0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x88\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0EW\x92\x91\x90a\x1D\xC3V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x97\x91\x90a\x1D\xECV[\x91P\x91P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xF6\x91\x90a\x1E,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F7\x91\x90a\x1D\x87V[\x90P`\0\x84\x03a\x0FsW`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x85`\0\x01Q\x03a\x0F\xB1W`@Q\x7Fj\x87\xC8\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0F\xD5\x86`\0\x01Q\x86\x86a\x0F\xC7\x91\x90a\x1EvV[a\x14\x06\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0F\xF7\x83\x85a\x0F\xE8\x91\x90a\x1EvV[\x83a\x141\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x10IW\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10@\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xFD[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x10c\x91\x90a\x1E\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x8E\x91\x90a\x1E\xECV[\x90P\x87` \x01Q\x81\x11\x15a\x10\xA6W\x87` \x01Qa\x10\xA8V[\x80[\x90P`\0a\x10\xC7\x85\x87a\x10\xBB\x91\x90a\x1EvV[\x85\x8B`\0\x01Q\x85a\x14\x8AV[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81a\x10\xE1\x91\x90a :V[\x99P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82a\x10\xFD\x91\x90a kV[\x11\x15a\x11\x10W\x89a\x11\r\x90a \x9CV[\x99P[PPPPPPPPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x11\xB7a\x1A\xE1V[`\0\x80\x83`\xFF\x16\x03a\x12tW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01\x81\x81RPPa\x13\xEAV[`\x01\x83`\xFF\x16\x03a\x130W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01\x81\x81RPPa\x13\xE9V[`\x02\x83`\xFF\x16\x03a\x13\xE8W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01\x81\x81RPP[[[P\x91\x90PV[`\0a\x13\xFE\x83\x83`\x1Ba\x166V[\x90P\x92\x91PPV[`\0a\x14)\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x16\xAA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x14Tk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x16\xAA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x14\x82\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x17\xB1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80\x84a\x14\xC1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x14\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x14\xCB\x91\x90a\x1E\xB8V[\x90P`\0a\x14\xF9\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x14\xEA\x91\x90a\x1E\xB8V[\x86a\x18\x08\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x15$\x91\x90a\x1E\xB8V[\x90Pa\x159\x81\x83a\x18\x08\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x94\x93PPPPV[a\x15\xC1\x84\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a\x15z\x93\x92\x91\x90a!\rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x186V[PPPPV[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x16.W\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16%\x91\x90a\x1B1V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x81\x83\x10a\x16~W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16u\x92\x91\x90a\x1F\xE2V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x16\x8A\x91\x90a\x1E\xB8V[`\na\x16\x96\x91\x90a\"wV[\x84a\x16\xA1\x91\x90a\x1EvV[\x90P\x93\x92PPPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x16\xE5W\x83\x82\x81a\x16\xDBWa\x16\xDAa \x0BV[[\x04\x92PPPa\x17\xAAV[\x80\x84\x11a\x17\x1EW`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0\x80a\x17\xBF\x86\x86\x86a\x16\xAAV[\x90Pa\x17\xCA\x83a\x18\xCDV[\x80\x15a\x17\xE7WP`\0\x84\x80a\x17\xE2Wa\x17\xE1a \x0BV[[\x86\x88\t\x11[\x15a\x17\xFCW`\x01\x81a\x17\xF9\x91\x90a\x1E\xECV[\x90P[\x80\x91PP\x94\x93PPPPV[`\0a\x18.k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x17\xB1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x18a\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\x18\x86WP\x80\x80` \x01\x90Q\x81\x01\x90a\x18\x84\x91\x90a\"\xFAV[\x15[\x15a\x18\xC8W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xBF\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xFD[PPPV[`\0`\x01`\x02\x83`\x03\x81\x11\x15a\x18\xE6Wa\x18\xE5a#'V[[a\x18\xF0\x91\x90a#VV[`\xFF\x16\x14\x90P\x91\x90PV[``a\x19\t\x83\x83`\0a\x19\x11V[\x90P\x92\x91PPV[``\x81G\x10\x15a\x19XW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19O\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\x19\x81\x91\x90a#\xF8V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19\xBEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\xC3V[``\x91P[P\x91P\x91Pa\x19\xD3\x86\x83\x83a\x19\xDEV[\x92PPP\x93\x92PPPV[``\x82a\x19\xF3Wa\x19\xEE\x82a\x1AmV[a\x1AeV[`\0\x82Q\x14\x80\x15a\x1A\x1BWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x1A]W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1AT\x91\x90a\x1CJV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x1AfV[[\x93\x92PPPV[`\0\x81Q\x11\x15a\x1A\x80W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0\x81\x90P\x91\x90PV[a\x1B+\x81a\x1B\x18V[\x82RPPV[`\0` \x82\x01\x90Pa\x1BF`\0\x83\x01\x84a\x1B\"V[\x92\x91PPV[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x1Bg\x81a\x1BQV[\x81\x14a\x1BrW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\x84\x81a\x1B^V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1B\xB5\x82a\x1B\x8AV[\x90P\x91\x90PV[a\x1B\xC5\x81a\x1B\xAAV[\x81\x14a\x1B\xD0W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\xE2\x81a\x1B\xBCV[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\x01Wa\x1C\0a\x1BLV[[`\0a\x1C\x0F\x86\x82\x87\x01a\x1BuV[\x93PP` a\x1C \x86\x82\x87\x01a\x1B\xD3V[\x92PP`@a\x1C1\x86\x82\x87\x01a\x1B\xD3V[\x91PP\x92P\x92P\x92V[a\x1CD\x81a\x1B\xAAV[\x82RPPV[`\0` \x82\x01\x90Pa\x1C_`\0\x83\x01\x84a\x1C;V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x1C\x8Aa\x1C\x85a\x1C\x80\x84a\x1B\x8AV[a\x1CeV[a\x1B\x8AV[\x90P\x91\x90PV[`\0a\x1C\x9C\x82a\x1CoV[\x90P\x91\x90PV[`\0a\x1C\xAE\x82a\x1C\x91V[\x90P\x91\x90PV[a\x1C\xBE\x81a\x1C\xA3V[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\xD9`\0\x83\x01\x84a\x1C\xB5V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xF6Wa\x1C\xF5a\x1BLV[[`\0a\x1D\x04\x85\x82\x86\x01a\x1BuV[\x92PP` a\x1D\x15\x85\x82\x86\x01a\x1B\xD3V[\x91PP\x92P\x92\x90PV[`\0a\x1D*\x82a\x1C\x91V[\x90P\x91\x90PV[a\x1D:\x81a\x1D\x1FV[\x82RPPV[`\0` \x82\x01\x90Pa\x1DU`\0\x83\x01\x84a\x1D1V[\x92\x91PPV[a\x1Dd\x81a\x1B\x18V[\x81\x14a\x1DoW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x1D\x81\x81a\x1D[V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D\x9DWa\x1D\x9Ca\x1BLV[[`\0a\x1D\xAB\x84\x82\x85\x01a\x1DrV[\x91PP\x92\x91PPV[a\x1D\xBD\x81a\x1BQV[\x82RPPV[`\0`@\x82\x01\x90Pa\x1D\xD8`\0\x83\x01\x85a\x1D\xB4V[a\x1D\xE5` \x83\x01\x84a\x1C;V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\x03Wa\x1E\x02a\x1BLV[[`\0a\x1E\x11\x85\x82\x86\x01a\x1DrV[\x92PP` a\x1E\"\x85\x82\x86\x01a\x1DrV[\x91PP\x92P\x92\x90PV[`\0` \x82\x01\x90Pa\x1EA`\0\x83\x01\x84a\x1D\xB4V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x1E\x81\x82a\x1B\x18V[\x91Pa\x1E\x8C\x83a\x1B\x18V[\x92P\x82\x82\x02a\x1E\x9A\x81a\x1B\x18V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1E\xB1Wa\x1E\xB0a\x1EGV[[P\x92\x91PPV[`\0a\x1E\xC3\x82a\x1B\x18V[\x91Pa\x1E\xCE\x83a\x1B\x18V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1E\xE6Wa\x1E\xE5a\x1EGV[[\x92\x91PPV[`\0a\x1E\xF7\x82a\x1B\x18V[\x91Pa\x1F\x02\x83a\x1B\x18V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1F\x1AWa\x1F\x19a\x1EGV[[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x1F5\x82a\x1F V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1FgWa\x1Ffa\x1EGV[[\x81`\0\x03\x90P\x91\x90PV[a\x1F{\x81a\x1F V[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x1F\x96`\0\x83\x01\x89a\x1D\xB4V[a\x1F\xA3` \x83\x01\x88a\x1C;V[a\x1F\xB0`@\x83\x01\x87a\x1C;V[a\x1F\xBD``\x83\x01\x86a\x1C;V[a\x1F\xCA`\x80\x83\x01\x85a\x1FrV[a\x1F\xD7`\xA0\x83\x01\x84a\x1FrV[\x97\x96PPPPPPPV[`\0`@\x82\x01\x90Pa\x1F\xF7`\0\x83\x01\x85a\x1B\"V[a \x04` \x83\x01\x84a\x1B\"V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a E\x82a\x1B\x18V[\x91Pa P\x83a\x1B\x18V[\x92P\x82a `Wa _a \x0BV[[\x82\x82\x04\x90P\x92\x91PPV[`\0a v\x82a\x1B\x18V[\x91Pa \x81\x83a\x1B\x18V[\x92P\x82a \x91Wa \x90a \x0BV[[\x82\x82\x06\x90P\x92\x91PPV[`\0a \xA7\x82a\x1B\x18V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a \xD9Wa \xD8a\x1EGV[[`\x01\x82\x01\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa \xF9`\0\x83\x01\x85a\x1C;V[a!\x06` \x83\x01\x84a\x1B\"V[\x93\x92PPPV[`\0``\x82\x01\x90Pa!\"`\0\x83\x01\x86a\x1C;V[a!/` \x83\x01\x85a\x1C;V[a!<`@\x83\x01\x84a\x1B\"V[\x94\x93PPPPV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a!\x9BW\x80\x86\x04\x81\x11\x15a!wWa!va\x1EGV[[`\x01\x85\x16\x15a!\x86W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa!\x94\x85a!DV[\x94Pa![V[\x94P\x94\x92PPPV[`\0\x82a!\xB4W`\x01\x90Pa\"pV[\x81a!\xC2W`\0\x90Pa\"pV[\x81`\x01\x81\x14a!\xD8W`\x02\x81\x14a!\xE2Wa\"\x11V[`\x01\x91PPa\"pV[`\xFF\x84\x11\x15a!\xF4Wa!\xF3a\x1EGV[[\x83`\x02\n\x91P\x84\x82\x11\x15a\"\x0BWa\"\na\x1EGV[[Pa\"pV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\"FW\x82\x82\n\x90P\x83\x81\x11\x15a\"AWa\"@a\x1EGV[[a\"pV[a\"S\x84\x84\x84`\x01a!QV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\"jWa\"ia\x1EGV[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\"\x82\x82a\x1B\x18V[\x91Pa\"\x8D\x83a\x1B\x18V[\x92Pa\"\xBA\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a!\xA4V[\x90P\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\"\xD7\x81a\"\xC2V[\x81\x14a\"\xE2W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\"\xF4\x81a\"\xCEV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\x10Wa#\x0Fa\x1BLV[[`\0a#\x1E\x84\x82\x85\x01a\"\xE5V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0a#a\x82a\x1BQV[\x91Pa#l\x83a\x1BQV[\x92P\x82a#|Wa#{a \x0BV[[\x82\x82\x06\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a#\xBBW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa#\xA0V[`\0\x84\x84\x01RPPPPV[`\0a#\xD2\x82a#\x87V[a#\xDC\x81\x85a#\x92V[\x93Pa#\xEC\x81\x85` \x86\x01a#\x9DV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a$\x04\x82\x84a#\xC7V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 (\x8A\xC4\xD4\x17\x1E\xBA\xE6\xCB:\xB3O2P\x89\xC7\0\x02\xF2\xB2\x02\x9A\xC2\x8BX\xD9p\xF4\xDA\xD5TmdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidation<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidation<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidation<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidation<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidation<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidation))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidation<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATION_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
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
                LIQUIDATION_ABI.clone(),
                LIQUIDATION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BASE_DISCOUNT` (0x49ed6434) function
        pub fn base_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([73, 237, 100, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LIQUIDATION_THRESHOLD_0` (0xe72dc85e) function
        pub fn liquidation_threshold_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 45, 200, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LIQUIDATION_THRESHOLD_1` (0xe15331cc) function
        pub fn liquidation_threshold_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([225, 83, 49, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LIQUIDATION_THRESHOLD_2` (0x9a7dda8d) function
        pub fn liquidation_threshold_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 125, 218, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_DISCOUNT_0` (0x1ae74318) function
        pub fn max_discount_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([26, 231, 67, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_DISCOUNT_1` (0x552abb6d) function
        pub fn max_discount_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 42, 187, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_DISCOUNT_2` (0x1a5984b6) function
        pub fn max_discount_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([26, 89, 132, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL` (0x7535d246) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROTOCOL` (0x91b9b827) function
        pub fn protocol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([145, 185, 184, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_ORACLE_0` (0x5529b78a) function
        pub fn reserve_oracle_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([85, 41, 183, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_ORACLE_1` (0x63205561) function
        pub fn reserve_oracle_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 32, 85, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_ORACLE_2` (0x4d196984) function
        pub fn reserve_oracle_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([77, 25, 105, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TARGET_HEALTH` (0xae539533) function
        pub fn target_health(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([174, 83, 149, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNDERLYING` (0xc5d664c6) function
        pub fn underlying(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 214, 100, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRepayAmt` (0xa0d5f599) function
        pub fn get_repay_amt(
            &self,
            ilk_index: u8,
            vault: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 213, 245, 153], (ilk_index, vault))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x25840eda) function
        pub fn liquidate(
            &self,
            ilk_index: u8,
            vault: ::ethers::core::types::Address,
            kpr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 132, 14, 218], (ilk_index, vault, kpr))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Liquidate` event
        pub fn liquidate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidateFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidateFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidation<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
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
    #[etherror(
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `ExchangeRateCannotBeZero` with signature `ExchangeRateCannotBeZero()` and selector `0x150c57e5`
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
    #[etherror(name = "ExchangeRateCannotBeZero", abi = "ExchangeRateCannotBeZero()")]
    pub struct ExchangeRateCannotBeZero;
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `InvalidLiquidationThresholdsLength` with signature `InvalidLiquidationThresholdsLength(uint256)` and selector `0x36eb1e90`
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
    #[etherror(
        name = "InvalidLiquidationThresholdsLength",
        abi = "InvalidLiquidationThresholdsLength(uint256)"
    )]
    pub struct InvalidLiquidationThresholdsLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidReserveOraclesLength` with signature `InvalidReserveOraclesLength(uint256)` and selector `0x4468c4c2`
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
    #[etherror(
        name = "InvalidReserveOraclesLength",
        abi = "InvalidReserveOraclesLength(uint256)"
    )]
    pub struct InvalidReserveOraclesLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `LiquidationThresholdCannotBeZero` with signature `LiquidationThresholdCannotBeZero()` and selector `0x6a87c81f`
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
    #[etherror(
        name = "LiquidationThresholdCannotBeZero",
        abi = "LiquidationThresholdCannotBeZero()"
    )]
    pub struct LiquidationThresholdCannotBeZero;
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
    ///Custom Error type `NotScalingUp` with signature `NotScalingUp(uint256,uint256)` and selector `0x1a065cf1`
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
    #[etherror(name = "NotScalingUp", abi = "NotScalingUp(uint256,uint256)")]
    pub struct NotScalingUp {
        pub from: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::U256,
    }
    ///Custom Error type `SafeCastOverflowedUintToInt` with signature `SafeCastOverflowedUintToInt(uint256)` and selector `0x24775e06`
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
    #[etherror(
        name = "SafeCastOverflowedUintToInt",
        abi = "SafeCastOverflowedUintToInt(uint256)"
    )]
    pub struct SafeCastOverflowedUintToInt {
        pub value: ::ethers::core::types::U256,
    }
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
    #[etherror(
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `VaultIsNotUnsafe` with signature `VaultIsNotUnsafe(uint256)` and selector `0x1bf23ef6`
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
    #[etherror(name = "VaultIsNotUnsafe", abi = "VaultIsNotUnsafe(uint256)")]
    pub struct VaultIsNotUnsafe {
        pub health_ratio: ::ethers::core::types::U256,
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
    pub enum LiquidationErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        ExchangeRateCannotBeZero(ExchangeRateCannotBeZero),
        FailedInnerCall(FailedInnerCall),
        InvalidLiquidationThresholdsLength(InvalidLiquidationThresholdsLength),
        InvalidReserveOraclesLength(InvalidReserveOraclesLength),
        LiquidationThresholdCannotBeZero(LiquidationThresholdCannotBeZero),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        NotScalingUp(NotScalingUp),
        SafeCastOverflowedUintToInt(SafeCastOverflowedUintToInt),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        VaultIsNotUnsafe(VaultIsNotUnsafe),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidationErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <ExchangeRateCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExchangeRateCannotBeZero(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <InvalidLiquidationThresholdsLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidLiquidationThresholdsLength(decoded));
            }
            if let Ok(decoded) = <InvalidReserveOraclesLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReserveOraclesLength(decoded));
            }
            if let Ok(decoded) = <LiquidationThresholdCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidationThresholdCannotBeZero(decoded));
            }
            if let Ok(decoded) = <MathOverflowedMulDiv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MathOverflowedMulDiv(decoded));
            }
            if let Ok(decoded) = <NotScalingUp as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotScalingUp(decoded));
            }
            if let Ok(decoded) = <SafeCastOverflowedUintToInt as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeCastOverflowedUintToInt(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <VaultIsNotUnsafe as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VaultIsNotUnsafe(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidationErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExchangeRateCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidLiquidationThresholdsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveOraclesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationThresholdCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotScalingUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastOverflowedUintToInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VaultIsNotUnsafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LiquidationErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExchangeRateCannotBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidLiquidationThresholdsLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidReserveOraclesLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LiquidationThresholdCannotBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MathOverflowedMulDiv as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotScalingUp as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SafeCastOverflowedUintToInt as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <VaultIsNotUnsafe as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LiquidationErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExchangeRateCannotBeZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidLiquidationThresholdsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidReserveOraclesLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationThresholdCannotBeZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotScalingUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeCastOverflowedUintToInt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VaultIsNotUnsafe(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidationErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LiquidationErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for LiquidationErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ExchangeRateCannotBeZero> for LiquidationErrors {
        fn from(value: ExchangeRateCannotBeZero) -> Self {
            Self::ExchangeRateCannotBeZero(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LiquidationErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InvalidLiquidationThresholdsLength>
    for LiquidationErrors {
        fn from(value: InvalidLiquidationThresholdsLength) -> Self {
            Self::InvalidLiquidationThresholdsLength(value)
        }
    }
    impl ::core::convert::From<InvalidReserveOraclesLength> for LiquidationErrors {
        fn from(value: InvalidReserveOraclesLength) -> Self {
            Self::InvalidReserveOraclesLength(value)
        }
    }
    impl ::core::convert::From<LiquidationThresholdCannotBeZero> for LiquidationErrors {
        fn from(value: LiquidationThresholdCannotBeZero) -> Self {
            Self::LiquidationThresholdCannotBeZero(value)
        }
    }
    impl ::core::convert::From<MathOverflowedMulDiv> for LiquidationErrors {
        fn from(value: MathOverflowedMulDiv) -> Self {
            Self::MathOverflowedMulDiv(value)
        }
    }
    impl ::core::convert::From<NotScalingUp> for LiquidationErrors {
        fn from(value: NotScalingUp) -> Self {
            Self::NotScalingUp(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflowedUintToInt> for LiquidationErrors {
        fn from(value: SafeCastOverflowedUintToInt) -> Self {
            Self::SafeCastOverflowedUintToInt(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for LiquidationErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<VaultIsNotUnsafe> for LiquidationErrors {
        fn from(value: VaultIsNotUnsafe) -> Self {
            Self::VaultIsNotUnsafe(value)
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
    #[ethevent(name = "Liquidate", abi = "Liquidate(address,uint8,uint256,uint256)")]
    pub struct LiquidateFilter {
        #[ethevent(indexed)]
        pub kpr: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ilk_index: u8,
        pub repay: ::ethers::core::types::U256,
        pub gem_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `BASE_DISCOUNT` function with signature `BASE_DISCOUNT()` and selector `0x49ed6434`
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
    #[ethcall(name = "BASE_DISCOUNT", abi = "BASE_DISCOUNT()")]
    pub struct BaseDiscountCall;
    ///Container type for all input parameters for the `LIQUIDATION_THRESHOLD_0` function with signature `LIQUIDATION_THRESHOLD_0()` and selector `0xe72dc85e`
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
    #[ethcall(name = "LIQUIDATION_THRESHOLD_0", abi = "LIQUIDATION_THRESHOLD_0()")]
    pub struct LiquidationThreshold0Call;
    ///Container type for all input parameters for the `LIQUIDATION_THRESHOLD_1` function with signature `LIQUIDATION_THRESHOLD_1()` and selector `0xe15331cc`
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
    #[ethcall(name = "LIQUIDATION_THRESHOLD_1", abi = "LIQUIDATION_THRESHOLD_1()")]
    pub struct LiquidationThreshold1Call;
    ///Container type for all input parameters for the `LIQUIDATION_THRESHOLD_2` function with signature `LIQUIDATION_THRESHOLD_2()` and selector `0x9a7dda8d`
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
    #[ethcall(name = "LIQUIDATION_THRESHOLD_2", abi = "LIQUIDATION_THRESHOLD_2()")]
    pub struct LiquidationThreshold2Call;
    ///Container type for all input parameters for the `MAX_DISCOUNT_0` function with signature `MAX_DISCOUNT_0()` and selector `0x1ae74318`
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
    #[ethcall(name = "MAX_DISCOUNT_0", abi = "MAX_DISCOUNT_0()")]
    pub struct MaxDiscount0Call;
    ///Container type for all input parameters for the `MAX_DISCOUNT_1` function with signature `MAX_DISCOUNT_1()` and selector `0x552abb6d`
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
    #[ethcall(name = "MAX_DISCOUNT_1", abi = "MAX_DISCOUNT_1()")]
    pub struct MaxDiscount1Call;
    ///Container type for all input parameters for the `MAX_DISCOUNT_2` function with signature `MAX_DISCOUNT_2()` and selector `0x1a5984b6`
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
    #[ethcall(name = "MAX_DISCOUNT_2", abi = "MAX_DISCOUNT_2()")]
    pub struct MaxDiscount2Call;
    ///Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `PROTOCOL` function with signature `PROTOCOL()` and selector `0x91b9b827`
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
    #[ethcall(name = "PROTOCOL", abi = "PROTOCOL()")]
    pub struct ProtocolCall;
    ///Container type for all input parameters for the `RESERVE_ORACLE_0` function with signature `RESERVE_ORACLE_0()` and selector `0x5529b78a`
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
    #[ethcall(name = "RESERVE_ORACLE_0", abi = "RESERVE_ORACLE_0()")]
    pub struct ReserveOracle0Call;
    ///Container type for all input parameters for the `RESERVE_ORACLE_1` function with signature `RESERVE_ORACLE_1()` and selector `0x63205561`
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
    #[ethcall(name = "RESERVE_ORACLE_1", abi = "RESERVE_ORACLE_1()")]
    pub struct ReserveOracle1Call;
    ///Container type for all input parameters for the `RESERVE_ORACLE_2` function with signature `RESERVE_ORACLE_2()` and selector `0x4d196984`
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
    #[ethcall(name = "RESERVE_ORACLE_2", abi = "RESERVE_ORACLE_2()")]
    pub struct ReserveOracle2Call;
    ///Container type for all input parameters for the `TARGET_HEALTH` function with signature `TARGET_HEALTH()` and selector `0xae539533`
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
    #[ethcall(name = "TARGET_HEALTH", abi = "TARGET_HEALTH()")]
    pub struct TargetHealthCall;
    ///Container type for all input parameters for the `UNDERLYING` function with signature `UNDERLYING()` and selector `0xc5d664c6`
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
    #[ethcall(name = "UNDERLYING", abi = "UNDERLYING()")]
    pub struct UnderlyingCall;
    ///Container type for all input parameters for the `getRepayAmt` function with signature `getRepayAmt(uint8,address)` and selector `0xa0d5f599`
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
    #[ethcall(name = "getRepayAmt", abi = "getRepayAmt(uint8,address)")]
    pub struct GetRepayAmtCall {
        pub ilk_index: u8,
        pub vault: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(uint8,address,address)` and selector `0x25840eda`
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
    #[ethcall(name = "liquidate", abi = "liquidate(uint8,address,address)")]
    pub struct LiquidateCall {
        pub ilk_index: u8,
        pub vault: ::ethers::core::types::Address,
        pub kpr: ::ethers::core::types::Address,
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
    pub enum LiquidationCalls {
        BaseDiscount(BaseDiscountCall),
        LiquidationThreshold0(LiquidationThreshold0Call),
        LiquidationThreshold1(LiquidationThreshold1Call),
        LiquidationThreshold2(LiquidationThreshold2Call),
        MaxDiscount0(MaxDiscount0Call),
        MaxDiscount1(MaxDiscount1Call),
        MaxDiscount2(MaxDiscount2Call),
        Pool(PoolCall),
        Protocol(ProtocolCall),
        ReserveOracle0(ReserveOracle0Call),
        ReserveOracle1(ReserveOracle1Call),
        ReserveOracle2(ReserveOracle2Call),
        TargetHealth(TargetHealthCall),
        Underlying(UnderlyingCall),
        GetRepayAmt(GetRepayAmtCall),
        Liquidate(LiquidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidationCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BaseDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseDiscount(decoded));
            }
            if let Ok(decoded) = <LiquidationThreshold0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidationThreshold0(decoded));
            }
            if let Ok(decoded) = <LiquidationThreshold1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidationThreshold1(decoded));
            }
            if let Ok(decoded) = <LiquidationThreshold2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidationThreshold2(decoded));
            }
            if let Ok(decoded) = <MaxDiscount0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxDiscount0(decoded));
            }
            if let Ok(decoded) = <MaxDiscount1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxDiscount1(decoded));
            }
            if let Ok(decoded) = <MaxDiscount2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxDiscount2(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) = <ProtocolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Protocol(decoded));
            }
            if let Ok(decoded) = <ReserveOracle0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveOracle0(decoded));
            }
            if let Ok(decoded) = <ReserveOracle1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveOracle1(decoded));
            }
            if let Ok(decoded) = <ReserveOracle2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveOracle2(decoded));
            }
            if let Ok(decoded) = <TargetHealthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetHealth(decoded));
            }
            if let Ok(decoded) = <UnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Underlying(decoded));
            }
            if let Ok(decoded) = <GetRepayAmtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRepayAmt(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BaseDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationThreshold0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationThreshold1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationThreshold2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxDiscount0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxDiscount1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxDiscount2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Protocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveOracle0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveOracle1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveOracle2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetHealth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Underlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRepayAmt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidationCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationThreshold0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationThreshold1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationThreshold2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxDiscount0(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDiscount1(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDiscount2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Protocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveOracle0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveOracle1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveOracle2(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetHealth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Underlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRepayAmt(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BaseDiscountCall> for LiquidationCalls {
        fn from(value: BaseDiscountCall) -> Self {
            Self::BaseDiscount(value)
        }
    }
    impl ::core::convert::From<LiquidationThreshold0Call> for LiquidationCalls {
        fn from(value: LiquidationThreshold0Call) -> Self {
            Self::LiquidationThreshold0(value)
        }
    }
    impl ::core::convert::From<LiquidationThreshold1Call> for LiquidationCalls {
        fn from(value: LiquidationThreshold1Call) -> Self {
            Self::LiquidationThreshold1(value)
        }
    }
    impl ::core::convert::From<LiquidationThreshold2Call> for LiquidationCalls {
        fn from(value: LiquidationThreshold2Call) -> Self {
            Self::LiquidationThreshold2(value)
        }
    }
    impl ::core::convert::From<MaxDiscount0Call> for LiquidationCalls {
        fn from(value: MaxDiscount0Call) -> Self {
            Self::MaxDiscount0(value)
        }
    }
    impl ::core::convert::From<MaxDiscount1Call> for LiquidationCalls {
        fn from(value: MaxDiscount1Call) -> Self {
            Self::MaxDiscount1(value)
        }
    }
    impl ::core::convert::From<MaxDiscount2Call> for LiquidationCalls {
        fn from(value: MaxDiscount2Call) -> Self {
            Self::MaxDiscount2(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidationCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ProtocolCall> for LiquidationCalls {
        fn from(value: ProtocolCall) -> Self {
            Self::Protocol(value)
        }
    }
    impl ::core::convert::From<ReserveOracle0Call> for LiquidationCalls {
        fn from(value: ReserveOracle0Call) -> Self {
            Self::ReserveOracle0(value)
        }
    }
    impl ::core::convert::From<ReserveOracle1Call> for LiquidationCalls {
        fn from(value: ReserveOracle1Call) -> Self {
            Self::ReserveOracle1(value)
        }
    }
    impl ::core::convert::From<ReserveOracle2Call> for LiquidationCalls {
        fn from(value: ReserveOracle2Call) -> Self {
            Self::ReserveOracle2(value)
        }
    }
    impl ::core::convert::From<TargetHealthCall> for LiquidationCalls {
        fn from(value: TargetHealthCall) -> Self {
            Self::TargetHealth(value)
        }
    }
    impl ::core::convert::From<UnderlyingCall> for LiquidationCalls {
        fn from(value: UnderlyingCall) -> Self {
            Self::Underlying(value)
        }
    }
    impl ::core::convert::From<GetRepayAmtCall> for LiquidationCalls {
        fn from(value: GetRepayAmtCall) -> Self {
            Self::GetRepayAmt(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidationCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    ///Container type for all return fields from the `BASE_DISCOUNT` function with signature `BASE_DISCOUNT()` and selector `0x49ed6434`
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
    pub struct BaseDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `LIQUIDATION_THRESHOLD_0` function with signature `LIQUIDATION_THRESHOLD_0()` and selector `0xe72dc85e`
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
    pub struct LiquidationThreshold0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `LIQUIDATION_THRESHOLD_1` function with signature `LIQUIDATION_THRESHOLD_1()` and selector `0xe15331cc`
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
    pub struct LiquidationThreshold1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `LIQUIDATION_THRESHOLD_2` function with signature `LIQUIDATION_THRESHOLD_2()` and selector `0x9a7dda8d`
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
    pub struct LiquidationThreshold2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_DISCOUNT_0` function with signature `MAX_DISCOUNT_0()` and selector `0x1ae74318`
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
    pub struct MaxDiscount0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_DISCOUNT_1` function with signature `MAX_DISCOUNT_1()` and selector `0x552abb6d`
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
    pub struct MaxDiscount1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_DISCOUNT_2` function with signature `MAX_DISCOUNT_2()` and selector `0x1a5984b6`
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
    pub struct MaxDiscount2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    pub struct PoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `PROTOCOL` function with signature `PROTOCOL()` and selector `0x91b9b827`
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
    pub struct ProtocolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `RESERVE_ORACLE_0` function with signature `RESERVE_ORACLE_0()` and selector `0x5529b78a`
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
    pub struct ReserveOracle0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `RESERVE_ORACLE_1` function with signature `RESERVE_ORACLE_1()` and selector `0x63205561`
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
    pub struct ReserveOracle1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `RESERVE_ORACLE_2` function with signature `RESERVE_ORACLE_2()` and selector `0x4d196984`
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
    pub struct ReserveOracle2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `TARGET_HEALTH` function with signature `TARGET_HEALTH()` and selector `0xae539533`
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
    pub struct TargetHealthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `UNDERLYING` function with signature `UNDERLYING()` and selector `0xc5d664c6`
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
    pub struct UnderlyingReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRepayAmt` function with signature `getRepayAmt(uint8,address)` and selector `0xa0d5f599`
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
    pub struct GetRepayAmtReturn {
        pub repay: ::ethers::core::types::U256,
    }
}
