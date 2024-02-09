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
                        name: ::std::borrow::ToOwned::to_owned("_maxDiscounts"),
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
                    ::std::borrow::ToOwned::to_owned("LIQUIDATION_THRESHOLD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LIQUIDATION_THRESHOLD",
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
                    ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_DISCOUNT"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repayAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gemOut"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
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
                    ::std::borrow::ToOwned::to_owned("InvalidLiquidationThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidLiquidationThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidationThreshold",
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
                    ::std::borrow::ToOwned::to_owned("InvalidMaxDiscount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMaxDiscount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDiscount"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidMaxDiscountsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMaxDiscountsLength",
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
                    ::std::borrow::ToOwned::to_owned("InvalidTargetHealth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidTargetHealth",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetHealth"),
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
    const __BYTECODE: &[u8] = b"a\x01\x80`@R4\x80\x15b\0\0\x11W_\x80\xFD[P`@Qb\0/\xB78\x03\x80b\0/\xB7\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x0B\x01V[_\x87\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01@\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_a\x01@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB6N\0\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xF3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\x19\x91\x90b\0\x0C\x0CV[\x90P_\x83Q\x90P\x81\x81\x14b\0\x01hW\x83Q`@Q\x7F\xD7X\x9A\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x01_\x91\x90b\0\x0CMV[`@Q\x80\x91\x03\x90\xFD[\x81\x88Q\x14b\0\x01\xB1W\x87Q`@Q\x7FDh\xC4\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x01\xA8\x91\x90b\0\x0CMV[`@Q\x80\x91\x03\x90\xFD[_\x87Q\x90P\x82\x81\x14b\0\x01\xFEW\x87Q`@Q\x7F6\xEB\x1E\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x01\xF5\x91\x90b\0\x0CMV[`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15b\0\x02\xA2Wk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x86\x82\x81Q\x81\x10b\0\x02,Wb\0\x02+b\0\x0ChV[[` \x02` \x01\x01Q\x10b\0\x02\x96W\x85\x81\x81Q\x81\x10b\0\x02PWb\0\x02Ob\0\x0ChV[[` \x02` \x01\x01Q`@Q\x7F\xE1\xA8\".\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x02\x8D\x91\x90b\0\x0CMV[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01\x90Pb\0\x02\0V[P_[\x81\x81\x10\x15b\0\x03\xEAW_\x89\x82\x81Q\x81\x10b\0\x02\xC5Wb\0\x02\xC4b\0\x0ChV[[` \x02` \x01\x01Q\x03b\0\x03/W\x88\x81\x81Q\x81\x10b\0\x02\xE9Wb\0\x02\xE8b\0\x0ChV[[` \x02` \x01\x01Q`@Q\x7F\xCA\x8CE*\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x03&\x91\x90b\0\x0CMV[`@Q\x80\x91\x03\x90\xFD[b\0\x03\x97\x86\x82\x81Q\x81\x10b\0\x03IWb\0\x03Hb\0\x0ChV[[` \x02` \x01\x01Qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0b\0\x03j\x91\x90b\0\x0C\xC2V[\x8A\x83\x81Q\x81\x10b\0\x03\x80Wb\0\x03\x7Fb\0\x0ChV[[` \x02` \x01\x01Qb\0\x06F` \x1B\x90\x91\x90` \x1CV[\x88\x10\x15b\0\x03\xDEW\x87`@Q\x7F\x14!E\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x03\xD5\x91\x90b\0\x0CMV[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01\x90Pb\0\x02\xA5V[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x87\x10\x15b\0\x04?W\x86`@Q\x7F\x14!E\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x046\x91\x90b\0\x0CMV[`@Q\x80\x91\x03\x90\xFD[\x86`\x80\x81\x81RPP\x85`\xA0\x81\x81RPP\x84_\x81Q\x81\x10b\0\x04eWb\0\x04db\0\x0ChV[[` \x02` \x01\x01Q`\xC0\x81\x81RPP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x04\xBFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04\xE5\x91\x90b\0\r>V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x86\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x05D\x92\x91\x90b\0\r\x7FV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15b\0\x05aW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x05\x87\x91\x90b\0\r\xE4V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x88_\x81Q\x81\x10b\0\x05\xD3Wb\0\x05\xD2b\0\x0ChV[[` \x02` \x01\x01Q`\xE0\x81\x81RPP\x89_\x81Q\x81\x10b\0\x05\xF8Wb\0\x05\xF7b\0\x0ChV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPPPPPPPPPPb\0\x0E\xEBV[_b\0\x06mk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86b\0\x06u` \x1B\x90\x93\x92\x91\x90` \x1CV[\x90P\x92\x91PPV[_\x80b\0\x06\x8A\x86\x86\x86b\0\x06\xE1` \x1B` \x1CV[\x90Pb\0\x06\x9D\x83b\0\x07\xE6` \x1B` \x1CV[\x80\x15b\0\x06\xBDWP_\x84\x80b\0\x06\xB8Wb\0\x06\xB7b\0\x0E\x14V[[\x86\x88\t\x11[\x15b\0\x06\xD5W`\x01\x81b\0\x06\xD2\x91\x90b\0\x0EAV[\x90P[\x80\x91PP\x94\x93PPPPV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03b\0\x07\x1EW\x83\x82\x81b\0\x07\x13Wb\0\x07\x12b\0\x0E\x14V[[\x04\x92PPPb\0\x07\xDFV[\x80\x84\x11b\0\x07XW`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_`\x01`\x02\x83`\x03\x81\x11\x15b\0\x08\x01Wb\0\x08\0b\0\x0E{V[[b\0\x08\r\x91\x90b\0\x0E\xB4V[`\xFF\x16\x14\x90P\x91\x90PV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\x08T\x82b\0\x08)V[\x90P\x91\x90PV[b\0\x08f\x81b\0\x08HV[\x81\x14b\0\x08qW_\x80\xFD[PV[_\x81Q\x90Pb\0\x08\x84\x81b\0\x08[V[\x92\x91PPV[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[b\0\x08\xD6\x82b\0\x08\x8EV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x08\xF8Wb\0\x08\xF7b\0\x08\x9EV[[\x80`@RPPPV[_b\0\t\x0Cb\0\x08\x18V[\x90Pb\0\t\x1A\x82\x82b\0\x08\xCBV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\t<Wb\0\t;b\0\x08\x9EV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_b\0\tgb\0\ta\x84b\0\t\x1FV[b\0\t\x01V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\t\x8DWb\0\t\x8Cb\0\tMV[[\x83[\x81\x81\x10\x15b\0\t\xBAW\x80b\0\t\xA5\x88\x82b\0\x08tV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\t\x8FV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12b\0\t\xDBWb\0\t\xDAb\0\x08\x8AV[[\x81Qb\0\t\xED\x84\x82` \x86\x01b\0\tQV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\n\x13Wb\0\n\x12b\0\x08\x9EV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[b\0\n8\x81b\0\n$V[\x81\x14b\0\nCW_\x80\xFD[PV[_\x81Q\x90Pb\0\nV\x81b\0\n-V[\x92\x91PPV[_b\0\nrb\0\nl\x84b\0\t\xF6V[b\0\t\x01V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\n\x98Wb\0\n\x97b\0\tMV[[\x83[\x81\x81\x10\x15b\0\n\xC5W\x80b\0\n\xB0\x88\x82b\0\nFV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\n\x9AV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12b\0\n\xE6Wb\0\n\xE5b\0\x08\x8AV[[\x81Qb\0\n\xF8\x84\x82` \x86\x01b\0\n\\V[\x91PP\x92\x91PPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15b\0\x0B\x1FWb\0\x0B\x1Eb\0\x08!V[[_b\0\x0B.\x8A\x82\x8B\x01b\0\x08tV[\x97PP` b\0\x0BA\x8A\x82\x8B\x01b\0\x08tV[\x96PP`@\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0BeWb\0\x0Bdb\0\x08%V[[b\0\x0Bs\x8A\x82\x8B\x01b\0\t\xC4V[\x95PP``\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0B\x97Wb\0\x0B\x96b\0\x08%V[[b\0\x0B\xA5\x8A\x82\x8B\x01b\0\n\xCFV[\x94PP`\x80b\0\x0B\xB8\x8A\x82\x8B\x01b\0\nFV[\x93PP`\xA0b\0\x0B\xCB\x8A\x82\x8B\x01b\0\nFV[\x92PP`\xC0\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0B\xEFWb\0\x0B\xEEb\0\x08%V[[b\0\x0B\xFD\x8A\x82\x8B\x01b\0\n\xCFV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_` \x82\x84\x03\x12\x15b\0\x0C$Wb\0\x0C#b\0\x08!V[[_b\0\x0C3\x84\x82\x85\x01b\0\nFV[\x91PP\x92\x91PPV[b\0\x0CG\x81b\0\n$V[\x82RPPV[_` \x82\x01\x90Pb\0\x0Cb_\x83\x01\x84b\0\x0C<V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_b\0\x0C\xCE\x82b\0\n$V[\x91Pb\0\x0C\xDB\x83b\0\n$V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15b\0\x0C\xF6Wb\0\x0C\xF5b\0\x0C\x95V[[\x92\x91PPV[_b\0\r\x08\x82b\0\x08HV[\x90P\x91\x90PV[b\0\r\x1A\x81b\0\x0C\xFCV[\x81\x14b\0\r%W_\x80\xFD[PV[_\x81Q\x90Pb\0\r8\x81b\0\r\x0FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15b\0\rVWb\0\rUb\0\x08!V[[_b\0\re\x84\x82\x85\x01b\0\r(V[\x91PP\x92\x91PPV[b\0\ry\x81b\0\x08HV[\x82RPPV[_`@\x82\x01\x90Pb\0\r\x94_\x83\x01\x85b\0\rnV[b\0\r\xA3` \x83\x01\x84b\0\x0C<V[\x93\x92PPPV[_\x81\x15\x15\x90P\x91\x90PV[b\0\r\xC0\x81b\0\r\xAAV[\x81\x14b\0\r\xCBW_\x80\xFD[PV[_\x81Q\x90Pb\0\r\xDE\x81b\0\r\xB5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15b\0\r\xFCWb\0\r\xFBb\0\x08!V[[_b\0\x0E\x0B\x84\x82\x85\x01b\0\r\xCEV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_b\0\x0EM\x82b\0\n$V[\x91Pb\0\x0EZ\x83b\0\n$V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15b\0\x0EuWb\0\x0Etb\0\x0C\x95V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_b\0\x0E\xC0\x82b\0\x0E\xA8V[\x91Pb\0\x0E\xCD\x83b\0\x0E\xA8V[\x92P\x82b\0\x0E\xE0Wb\0\x0E\xDFb\0\x0E\x14V[[\x82\x82\x06\x90P\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x1F\xF2b\0\x0F\xC5_9_\x81\x81a\x08\xD6\x01Ra\x0F\x14\x01R_\x81\x81a\x02\xB5\x01R\x81\x81a\x03S\x01R\x81\x81a\x05\x9C\x01R\x81\x81a\x07\x0B\x01R\x81\x81a\t\x1D\x01R\x81\x81a\t\xD7\x01R\x81\x81a\x0B\x17\x01R\x81\x81a\x0C\x18\x01Ra\x0C\xB6\x01R_\x81\x81a\x05\xDA\x01R\x81\x81a\x05\xFB\x01Ra\x0B_\x01R_\x81\x81a\x01\xF3\x01Ra\x0Fd\x01R_\x81\x81a\x0B;\x01Ra\x0F\xBC\x01R_\x81\x81a\x0F8\x01Ra\x0F\xE5\x01R_\x81\x81a\x04\xD4\x01R\x81\x81a\n\xF3\x01Ra\x0E7\x01R_\x81\x81a\x0E\xF0\x01R\x81\x81a\x10\xAE\x01Ra\x11\x15\x01Ra\x1F\xF2_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x9CW_5`\xE0\x1C\x80c\x91\xB9\xB8'\x11a\0dW\x80c\x91\xB9\xB8'\x14a\x01IW\x80c\xA0\xD5\xF5\x99\x14a\x01gW\x80c\xAES\x953\x14a\x01\x97W\x80c\xC5\xD6d\xC6\x14a\x01\xB5W\x80c\xE3IUi\x14a\x01\xD3Wa\0\x9CV[\x80c\x1FQU\xC4\x14a\0\xA0W\x80c%\x84\x0E\xDA\x14a\0\xBEW\x80cI\xEDd4\x14a\0\xEFW\x80cu5\xD2F\x14a\x01\rW\x80c\x90\xA8\xAE\x9B\x14a\x01+W[_\x80\xFD[a\0\xA8a\x01\xF1V[`@Qa\0\xB5\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xF3[a\0\xD8`\x04\x806\x03\x81\x01\x90a\0\xD3\x91\x90a\x17\xCEV[a\x02\x15V[`@Qa\0\xE6\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xF3[a\0\xF7a\n\xF1V[`@Qa\x01\x04\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01\x15a\x0B\x15V[`@Qa\x01\"\x91\x90a\x18\xD1V[`@Q\x80\x91\x03\x90\xF3[a\x013a\x0B9V[`@Qa\x01@\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x0B]V[`@Qa\x01^\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xF3[a\x01\x81`\x04\x806\x03\x81\x01\x90a\x01|\x91\x90a\x18\xEAV[a\x0B\x81V[`@Qa\x01\x8E\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Fa\x0E\xEEV[`@Qa\x01\xAC\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01\xBDa\x0F\x12V[`@Qa\x01\xCA\x91\x90a\x19HV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDBa\x0F6V[`@Qa\x01\xE8\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_\x80a\x02\x1Fa\x16\xB4V[_a\x02(a\x0FZV[\x90P_a\x02\xAF`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA1\x91\x90a\x19\x8BV[a\x10\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x8B\x8B`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x0E\x92\x91\x90a\x19\xC5V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03L\x91\x90a\x19\xECV[\x91P\x91P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x8C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xAA\x91\x90a\x1A*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE9\x91\x90a\x19\x8BV[\x90P_\x84\x03a\x04$W`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04F\x86_\x01Q\x86\x86a\x048\x91\x90a\x1ApV[a\x10%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\x04g\x83\x85a\x04X\x91\x90a\x1ApV[\x83a\x10O\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x04\xB9W\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB0\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xFD[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x04\xD2\x91\x90a\x1A\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xFD\x91\x90a\x1A\xE4V[\x90P\x87` \x01Q\x81\x11\x15a\x05\x15W\x87` \x01Qa\x05\x17V[\x80[\x90Pa\x05C\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x054\x91\x90a\x1A\xB1V[\x88a\x10y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01\x81\x81RPPa\x05f\x84\x86a\x05[\x91\x90a\x1ApV[\x84\x8A_\x01Q\x84a\x10\xA6V[\x89_\x01\x81\x81RPPPP\x81\x83a\x05|\x91\x90a\x1ApV[\x87_\x01Q\x11\x15a\x07\tW\x82\x87`@\x01\x81\x81RPP\x83\x87` \x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8D\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C` \x01Qa\x06(\x90a\x1B V[\x8D`@\x01Qa\x066\x90a\x1B V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06W\x96\x95\x94\x93\x92\x91\x90a\x1BuV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06nW_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x80W=_\x80>=_\xFD[PPPP\x8B`\xFF\x16\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FT\xF9\xB0\x06\xB1\x9ADiNV\xB0\xD6\xA5\x7F4\xC4 \xBF\x88\xDAss\xF9\xD0\xF38\x0E_Kp\x95\x1E\x8A`@\x01Q\x8B` \x01Q`@Qa\x06\xEF\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xA4_\x80\x98P\x98PPPPPPPPa\n\xE9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B\xA7e\x07\x8D`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07b\x91\x90a\x1A*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA1\x91\x90a\x19\x8BV[\x87_\x01Q\x83\x85a\x07\xB1\x91\x90a\x1ApV[a\x07\xBB\x91\x90a\x1A\xB1V[\x10\x15a\x08\x08W\x81\x83a\x07\xCD\x91\x90a\x1ApV[\x87_\x01\x81\x81RPP\x82\x87`@\x01\x81\x81RPP\x86`\x80\x01Q\x82\x84a\x07\xF0\x91\x90a\x1ApV[a\x07\xFA\x91\x90a\x1C\x01V[\x87` \x01\x81\x81RPPa\x08\x81V[\x81\x87_\x01Qa\x08\x17\x91\x90a\x1C\x01V[\x87`@\x01\x81\x81RPP_\x82\x88_\x01Qa\x080\x91\x90a\x1C1V[\x11\x15a\x08LW\x86`@\x01\x80Qa\x08E\x90a\x1CaV[\x90\x81\x81RPP[\x86`\x80\x01Q\x87_\x01Qa\x08_\x91\x90a\x1C\x01V[\x87` \x01\x81\x81RPP\x81\x87`@\x01Qa\x08x\x91\x90a\x1ApV[\x87_\x01\x81\x81RPP[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88_\x01Qa\x08\x9D\x91\x90a\x1C\x01V[\x90P_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89_\x01Qa\x08\xBB\x91\x90a\x1C1V[\x11\x15a\x08\xCEW\x80a\x08\xCB\x90a\x1CaV[\x90P[a\t\x1B30\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11_\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8E\x8E\x8E0a\th\x8E` \x01Qa\x11\xE1V[a\tq\x90a\x1B V[a\t~\x8F`@\x01Qa\x11\xE1V[a\t\x87\x90a\x1B V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xA8\x96\x95\x94\x93\x92\x91\x90a\x1BuV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xBFW_\x80\xFD[PZ\xF1\x15\x80\x15a\t\xD1W=_\x80>=_\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD81\xEF\xD80\x8A_\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n3\x92\x91\x90a\x1C\xA8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\nJW_\x80\xFD[PZ\xF1\x15\x80\x15a\n\\W=_\x80>=_\xFD[PPPP\x8C`\xFF\x16\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FT\xF9\xB0\x06\xB1\x9ADiNV\xB0\xD6\xA5\x7F4\xC4 \xBF\x88\xDAss\xF9\xD0\xF38\x0E_Kp\x95\x1E\x8B`@\x01Q\x8C` \x01Q`@Qa\n\xCB\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xA4\x87_\x01Q\x88` \x01Q\x99P\x99PPPPPPPPP[\x93P\x93\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_\x80a\x0B\x8Ba\x0FZV[\x90P_a\x0C\x12`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xE0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x04\x91\x90a\x19\x8BV[a\x10\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x88\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Cq\x92\x91\x90a\x19\xC5V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x8BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAF\x91\x90a\x19\xECV[\x91P\x91P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\r\x91\x90a\x1A*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rL\x91\x90a\x19\x8BV[\x90P_\x84\x03a\r\x87W`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xA9\x86_\x01Q\x86\x86a\r\x9B\x91\x90a\x1ApV[a\x10%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\r\xCA\x83\x85a\r\xBB\x91\x90a\x1ApV[\x83a\x10O\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x0E\x1CW\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x13\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xFD[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E5\x91\x90a\x1A\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0E`\x91\x90a\x1A\xE4V[\x90P\x87` \x01Q\x81\x11\x15a\x0ExW\x87` \x01Qa\x0EzV[\x80[\x90P_a\x0E\x97\x85\x87a\x0E\x8C\x91\x90a\x1ApV[\x85\x8B_\x01Q\x85a\x10\xA6V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81a\x0E\xB1\x91\x90a\x1C\x01V[\x99P_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82a\x0E\xCC\x91\x90a\x1C1V[\x11\x15a\x0E\xDFW\x89a\x0E\xDC\x90a\x1CaV[\x99P[PPPPPPPPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x0Fba\x16\xDEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81` \x01\x81\x81RPP\x90V[_a\x10\x1D\x83\x83`\x1Ba\x12OV[\x90P\x92\x91PPV[_a\x10G\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x12\xC2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x10qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x12\xC2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x10\x9E\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x13\xC1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x80\x84a\x10\xDC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x10y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x10\xE6\x91\x90a\x1A\xB1V[\x90P_a\x11\x13\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x11\x04\x91\x90a\x1A\xB1V[\x86a\x14\x16\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11>\x91\x90a\x1A\xB1V[\x90Pa\x11S\x81\x83a\x14\x16\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x94\x93PPPPV[a\x11\xDB\x84\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a\x11\x94\x93\x92\x91\x90a\x1C\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x14CV[PPPPV[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12GW\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12>\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x81\x83\x10a\x12\x96W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\x8D\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x12\xA2\x91\x90a\x1A\xB1V[`\na\x12\xAE\x91\x90a\x1E3V[\x84a\x12\xB9\x91\x90a\x1ApV[\x90P\x93\x92PPPV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\x12\xFAW\x83\x82\x81a\x12\xF0Wa\x12\xEFa\x1B\xD4V[[\x04\x92PPPa\x13\xBAV[\x80\x84\x11a\x133W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_\x80a\x13\xCE\x86\x86\x86a\x12\xC2V[\x90Pa\x13\xD9\x83a\x14\xD8V[\x80\x15a\x13\xF5WP_\x84\x80a\x13\xF0Wa\x13\xEFa\x1B\xD4V[[\x86\x88\t\x11[\x15a\x14\nW`\x01\x81a\x14\x07\x91\x90a\x1A\xE4V[\x90P[\x80\x91PP\x94\x93PPPPV[_a\x14;k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x13\xC1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x14m\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x81Q\x14\x15\x80\x15a\x14\x91WP\x80\x80` \x01\x90Q\x81\x01\x90a\x14\x8F\x91\x90a\x1E\xB2V[\x15[\x15a\x14\xD3W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xCA\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xFD[PPPV[_`\x01`\x02\x83`\x03\x81\x11\x15a\x14\xF0Wa\x14\xEFa\x1E\xDDV[[a\x14\xFA\x91\x90a\x1F\nV[`\xFF\x16\x14\x90P\x91\x90PV[``a\x15\x12\x83\x83_a\x15\x1AV[\x90P\x92\x91PPV[``\x81G\x10\x15a\x15aW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15X\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xFD[_\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\x15\x89\x91\x90a\x1F\xA6V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x15\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x15\xC8V[``\x91P[P\x91P\x91Pa\x15\xD8\x86\x83\x83a\x15\xE3V[\x92PPP\x93\x92PPPV[``\x82a\x15\xF8Wa\x15\xF3\x82a\x16pV[a\x16hV[_\x82Q\x14\x80\x15a\x16\x1EWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x16`W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16W\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x16iV[[\x93\x92PPPV[_\x81Q\x11\x15a\x16\x82W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x17;\x82a\x17\x12V[\x90P\x91\x90PV[a\x17K\x81a\x171V[\x82RPPV[_` \x82\x01\x90Pa\x17d_\x83\x01\x84a\x17BV[\x92\x91PPV[_\x80\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\x17\x83\x81a\x17nV[\x81\x14a\x17\x8DW_\x80\xFD[PV[_\x815\x90Pa\x17\x9E\x81a\x17zV[\x92\x91PPV[a\x17\xAD\x81a\x171V[\x81\x14a\x17\xB7W_\x80\xFD[PV[_\x815\x90Pa\x17\xC8\x81a\x17\xA4V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x17\xE5Wa\x17\xE4a\x17jV[[_a\x17\xF2\x86\x82\x87\x01a\x17\x90V[\x93PP` a\x18\x03\x86\x82\x87\x01a\x17\xBAV[\x92PP`@a\x18\x14\x86\x82\x87\x01a\x17\xBAV[\x91PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[a\x180\x81a\x18\x1EV[\x82RPPV[_`@\x82\x01\x90Pa\x18I_\x83\x01\x85a\x18'V[a\x18V` \x83\x01\x84a\x18'V[\x93\x92PPPV[_` \x82\x01\x90Pa\x18p_\x83\x01\x84a\x18'V[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x18\x99a\x18\x94a\x18\x8F\x84a\x17\x12V[a\x18vV[a\x17\x12V[\x90P\x91\x90PV[_a\x18\xAA\x82a\x18\x7FV[\x90P\x91\x90PV[_a\x18\xBB\x82a\x18\xA0V[\x90P\x91\x90PV[a\x18\xCB\x81a\x18\xB1V[\x82RPPV[_` \x82\x01\x90Pa\x18\xE4_\x83\x01\x84a\x18\xC2V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x19\0Wa\x18\xFFa\x17jV[[_a\x19\r\x85\x82\x86\x01a\x17\x90V[\x92PP` a\x19\x1E\x85\x82\x86\x01a\x17\xBAV[\x91PP\x92P\x92\x90PV[_a\x192\x82a\x18\xA0V[\x90P\x91\x90PV[a\x19B\x81a\x19(V[\x82RPPV[_` \x82\x01\x90Pa\x19[_\x83\x01\x84a\x199V[\x92\x91PPV[a\x19j\x81a\x18\x1EV[\x81\x14a\x19tW_\x80\xFD[PV[_\x81Q\x90Pa\x19\x85\x81a\x19aV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x19\xA0Wa\x19\x9Fa\x17jV[[_a\x19\xAD\x84\x82\x85\x01a\x19wV[\x91PP\x92\x91PPV[a\x19\xBF\x81a\x17nV[\x82RPPV[_`@\x82\x01\x90Pa\x19\xD8_\x83\x01\x85a\x19\xB6V[a\x19\xE5` \x83\x01\x84a\x17BV[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x1A\x02Wa\x1A\x01a\x17jV[[_a\x1A\x0F\x85\x82\x86\x01a\x19wV[\x92PP` a\x1A \x85\x82\x86\x01a\x19wV[\x91PP\x92P\x92\x90PV[_` \x82\x01\x90Pa\x1A=_\x83\x01\x84a\x19\xB6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x1Az\x82a\x18\x1EV[\x91Pa\x1A\x85\x83a\x18\x1EV[\x92P\x82\x82\x02a\x1A\x93\x81a\x18\x1EV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1A\xAAWa\x1A\xA9a\x1ACV[[P\x92\x91PPV[_a\x1A\xBB\x82a\x18\x1EV[\x91Pa\x1A\xC6\x83a\x18\x1EV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1A\xDEWa\x1A\xDDa\x1ACV[[\x92\x91PPV[_a\x1A\xEE\x82a\x18\x1EV[\x91Pa\x1A\xF9\x83a\x18\x1EV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1B\x11Wa\x1B\x10a\x1ACV[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x1B*\x82a\x1B\x17V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1B\\Wa\x1B[a\x1ACV[[\x81_\x03\x90P\x91\x90PV[a\x1Bo\x81a\x1B\x17V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x1B\x88_\x83\x01\x89a\x19\xB6V[a\x1B\x95` \x83\x01\x88a\x17BV[a\x1B\xA2`@\x83\x01\x87a\x17BV[a\x1B\xAF``\x83\x01\x86a\x17BV[a\x1B\xBC`\x80\x83\x01\x85a\x1BfV[a\x1B\xC9`\xA0\x83\x01\x84a\x1BfV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x1C\x0B\x82a\x18\x1EV[\x91Pa\x1C\x16\x83a\x18\x1EV[\x92P\x82a\x1C&Wa\x1C%a\x1B\xD4V[[\x82\x82\x04\x90P\x92\x91PPV[_a\x1C;\x82a\x18\x1EV[\x91Pa\x1CF\x83a\x18\x1EV[\x92P\x82a\x1CVWa\x1CUa\x1B\xD4V[[\x82\x82\x06\x90P\x92\x91PPV[_a\x1Ck\x82a\x18\x1EV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1C\x9DWa\x1C\x9Ca\x1ACV[[`\x01\x82\x01\x90P\x91\x90PV[_`@\x82\x01\x90Pa\x1C\xBB_\x83\x01\x85a\x17BV[a\x1C\xC8` \x83\x01\x84a\x18'V[\x93\x92PPPV[_``\x82\x01\x90Pa\x1C\xE2_\x83\x01\x86a\x17BV[a\x1C\xEF` \x83\x01\x85a\x17BV[a\x1C\xFC`@\x83\x01\x84a\x18'V[\x94\x93PPPPV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x1DYW\x80\x86\x04\x81\x11\x15a\x1D5Wa\x1D4a\x1ACV[[`\x01\x85\x16\x15a\x1DDW\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x1DR\x85a\x1D\x04V[\x94Pa\x1D\x19V[\x94P\x94\x92PPPV[_\x82a\x1DqW`\x01\x90Pa\x1E,V[\x81a\x1D~W_\x90Pa\x1E,V[\x81`\x01\x81\x14a\x1D\x94W`\x02\x81\x14a\x1D\x9EWa\x1D\xCDV[`\x01\x91PPa\x1E,V[`\xFF\x84\x11\x15a\x1D\xB0Wa\x1D\xAFa\x1ACV[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x1D\xC7Wa\x1D\xC6a\x1ACV[[Pa\x1E,V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1E\x02W\x82\x82\n\x90P\x83\x81\x11\x15a\x1D\xFDWa\x1D\xFCa\x1ACV[[a\x1E,V[a\x1E\x0F\x84\x84\x84`\x01a\x1D\x10V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x1E&Wa\x1E%a\x1ACV[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x1E=\x82a\x18\x1EV[\x91Pa\x1EH\x83a\x18\x1EV[\x92Pa\x1Eu\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x1DbV[\x90P\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x1E\x91\x81a\x1E}V[\x81\x14a\x1E\x9BW_\x80\xFD[PV[_\x81Q\x90Pa\x1E\xAC\x81a\x1E\x88V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1E\xC7Wa\x1E\xC6a\x17jV[[_a\x1E\xD4\x84\x82\x85\x01a\x1E\x9EV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_a\x1F\x14\x82a\x17nV[\x91Pa\x1F\x1F\x83a\x17nV[\x92P\x82a\x1F/Wa\x1F.a\x1B\xD4V[[\x82\x82\x06\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x1FkW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1FPV[_\x84\x84\x01RPPPPV[_a\x1F\x80\x82a\x1F:V[a\x1F\x8A\x81\x85a\x1FDV[\x93Pa\x1F\x9A\x81\x85` \x86\x01a\x1FNV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x1F\xB1\x82\x84a\x1FvV[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xFCG\x12\xBF\x17(\xC0\xCE!\x01\xC1{#\xFAg\xF6\xA1\x89K\x06\x9D9Z2\xDAR\x96C\xC6}\xDB\xEFdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x9CW_5`\xE0\x1C\x80c\x91\xB9\xB8'\x11a\0dW\x80c\x91\xB9\xB8'\x14a\x01IW\x80c\xA0\xD5\xF5\x99\x14a\x01gW\x80c\xAES\x953\x14a\x01\x97W\x80c\xC5\xD6d\xC6\x14a\x01\xB5W\x80c\xE3IUi\x14a\x01\xD3Wa\0\x9CV[\x80c\x1FQU\xC4\x14a\0\xA0W\x80c%\x84\x0E\xDA\x14a\0\xBEW\x80cI\xEDd4\x14a\0\xEFW\x80cu5\xD2F\x14a\x01\rW\x80c\x90\xA8\xAE\x9B\x14a\x01+W[_\x80\xFD[a\0\xA8a\x01\xF1V[`@Qa\0\xB5\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xF3[a\0\xD8`\x04\x806\x03\x81\x01\x90a\0\xD3\x91\x90a\x17\xCEV[a\x02\x15V[`@Qa\0\xE6\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xF3[a\0\xF7a\n\xF1V[`@Qa\x01\x04\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01\x15a\x0B\x15V[`@Qa\x01\"\x91\x90a\x18\xD1V[`@Q\x80\x91\x03\x90\xF3[a\x013a\x0B9V[`@Qa\x01@\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x0B]V[`@Qa\x01^\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xF3[a\x01\x81`\x04\x806\x03\x81\x01\x90a\x01|\x91\x90a\x18\xEAV[a\x0B\x81V[`@Qa\x01\x8E\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Fa\x0E\xEEV[`@Qa\x01\xAC\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[a\x01\xBDa\x0F\x12V[`@Qa\x01\xCA\x91\x90a\x19HV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDBa\x0F6V[`@Qa\x01\xE8\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_\x80a\x02\x1Fa\x16\xB4V[_a\x02(a\x0FZV[\x90P_a\x02\xAF`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xA1\x91\x90a\x19\x8BV[a\x10\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x8B\x8B`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x0E\x92\x91\x90a\x19\xC5V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03L\x91\x90a\x19\xECV[\x91P\x91P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x8C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xAA\x91\x90a\x1A*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE9\x91\x90a\x19\x8BV[\x90P_\x84\x03a\x04$W`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04F\x86_\x01Q\x86\x86a\x048\x91\x90a\x1ApV[a\x10%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\x04g\x83\x85a\x04X\x91\x90a\x1ApV[\x83a\x10O\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x04\xB9W\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB0\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xFD[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x04\xD2\x91\x90a\x1A\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xFD\x91\x90a\x1A\xE4V[\x90P\x87` \x01Q\x81\x11\x15a\x05\x15W\x87` \x01Qa\x05\x17V[\x80[\x90Pa\x05C\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x054\x91\x90a\x1A\xB1V[\x88a\x10y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01\x81\x81RPPa\x05f\x84\x86a\x05[\x91\x90a\x1ApV[\x84\x8A_\x01Q\x84a\x10\xA6V[\x89_\x01\x81\x81RPPPP\x81\x83a\x05|\x91\x90a\x1ApV[\x87_\x01Q\x11\x15a\x07\tW\x82\x87`@\x01\x81\x81RPP\x83\x87` \x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8D\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C` \x01Qa\x06(\x90a\x1B V[\x8D`@\x01Qa\x066\x90a\x1B V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06W\x96\x95\x94\x93\x92\x91\x90a\x1BuV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06nW_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x80W=_\x80>=_\xFD[PPPP\x8B`\xFF\x16\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FT\xF9\xB0\x06\xB1\x9ADiNV\xB0\xD6\xA5\x7F4\xC4 \xBF\x88\xDAss\xF9\xD0\xF38\x0E_Kp\x95\x1E\x8A`@\x01Q\x8B` \x01Q`@Qa\x06\xEF\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xA4_\x80\x98P\x98PPPPPPPPa\n\xE9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B\xA7e\x07\x8D`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07b\x91\x90a\x1A*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA1\x91\x90a\x19\x8BV[\x87_\x01Q\x83\x85a\x07\xB1\x91\x90a\x1ApV[a\x07\xBB\x91\x90a\x1A\xB1V[\x10\x15a\x08\x08W\x81\x83a\x07\xCD\x91\x90a\x1ApV[\x87_\x01\x81\x81RPP\x82\x87`@\x01\x81\x81RPP\x86`\x80\x01Q\x82\x84a\x07\xF0\x91\x90a\x1ApV[a\x07\xFA\x91\x90a\x1C\x01V[\x87` \x01\x81\x81RPPa\x08\x81V[\x81\x87_\x01Qa\x08\x17\x91\x90a\x1C\x01V[\x87`@\x01\x81\x81RPP_\x82\x88_\x01Qa\x080\x91\x90a\x1C1V[\x11\x15a\x08LW\x86`@\x01\x80Qa\x08E\x90a\x1CaV[\x90\x81\x81RPP[\x86`\x80\x01Q\x87_\x01Qa\x08_\x91\x90a\x1C\x01V[\x87` \x01\x81\x81RPP\x81\x87`@\x01Qa\x08x\x91\x90a\x1ApV[\x87_\x01\x81\x81RPP[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88_\x01Qa\x08\x9D\x91\x90a\x1C\x01V[\x90P_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89_\x01Qa\x08\xBB\x91\x90a\x1C1V[\x11\x15a\x08\xCEW\x80a\x08\xCB\x90a\x1CaV[\x90P[a\t\x1B30\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11_\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97\x93\x97C\x8E\x8E\x8E0a\th\x8E` \x01Qa\x11\xE1V[a\tq\x90a\x1B V[a\t~\x8F`@\x01Qa\x11\xE1V[a\t\x87\x90a\x1B V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xA8\x96\x95\x94\x93\x92\x91\x90a\x1BuV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xBFW_\x80\xFD[PZ\xF1\x15\x80\x15a\t\xD1W=_\x80>=_\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD81\xEF\xD80\x8A_\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n3\x92\x91\x90a\x1C\xA8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\nJW_\x80\xFD[PZ\xF1\x15\x80\x15a\n\\W=_\x80>=_\xFD[PPPP\x8C`\xFF\x16\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FT\xF9\xB0\x06\xB1\x9ADiNV\xB0\xD6\xA5\x7F4\xC4 \xBF\x88\xDAss\xF9\xD0\xF38\x0E_Kp\x95\x1E\x8B`@\x01Q\x8C` \x01Q`@Qa\n\xCB\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xA4\x87_\x01Q\x88` \x01Q\x99P\x99PPPPPPPPP[\x93P\x93\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_\x80a\x0B\x8Ba\x0FZV[\x90P_a\x0C\x12`\x12\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xE0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x04\x91\x90a\x19\x8BV[a\x10\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A=\xB7\x9B\x88\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Cq\x92\x91\x90a\x19\xC5V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x8BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAF\x91\x90a\x19\xECV[\x91P\x91P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\r\x91\x90a\x1A*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rL\x91\x90a\x19\x8BV[\x90P_\x84\x03a\r\x87W`@Q\x7F\x15\x0CW\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xA9\x86_\x01Q\x86\x86a\r\x9B\x91\x90a\x1ApV[a\x10%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\r\xCA\x83\x85a\r\xBB\x91\x90a\x1ApV[\x83a\x10O\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x0E\x1CW\x80`@Q\x7F\x1B\xF2>\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x13\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xFD[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E5\x91\x90a\x1A\xB1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0E`\x91\x90a\x1A\xE4V[\x90P\x87` \x01Q\x81\x11\x15a\x0ExW\x87` \x01Qa\x0EzV[\x80[\x90P_a\x0E\x97\x85\x87a\x0E\x8C\x91\x90a\x1ApV[\x85\x8B_\x01Q\x85a\x10\xA6V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81a\x0E\xB1\x91\x90a\x1C\x01V[\x99P_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82a\x0E\xCC\x91\x90a\x1C1V[\x11\x15a\x0E\xDFW\x89a\x0E\xDC\x90a\x1CaV[\x99P[PPPPPPPPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x0Fba\x16\xDEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x01\x81\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81` \x01\x81\x81RPP\x90V[_a\x10\x1D\x83\x83`\x1Ba\x12OV[\x90P\x92\x91PPV[_a\x10G\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x12\xC2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x10qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x12\xC2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x10\x9E\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x13\xC1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x80\x84a\x10\xDC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x10y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x10\xE6\x91\x90a\x1A\xB1V[\x90P_a\x11\x13\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x11\x04\x91\x90a\x1A\xB1V[\x86a\x14\x16\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11>\x91\x90a\x1A\xB1V[\x90Pa\x11S\x81\x83a\x14\x16\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x94\x93PPPPV[a\x11\xDB\x84\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a\x11\x94\x93\x92\x91\x90a\x1C\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x14CV[PPPPV[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12GW\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12>\x91\x90a\x18]V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x81\x83\x10a\x12\x96W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\x8D\x92\x91\x90a\x186V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x12\xA2\x91\x90a\x1A\xB1V[`\na\x12\xAE\x91\x90a\x1E3V[\x84a\x12\xB9\x91\x90a\x1ApV[\x90P\x93\x92PPPV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\x12\xFAW\x83\x82\x81a\x12\xF0Wa\x12\xEFa\x1B\xD4V[[\x04\x92PPPa\x13\xBAV[\x80\x84\x11a\x133W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_\x80a\x13\xCE\x86\x86\x86a\x12\xC2V[\x90Pa\x13\xD9\x83a\x14\xD8V[\x80\x15a\x13\xF5WP_\x84\x80a\x13\xF0Wa\x13\xEFa\x1B\xD4V[[\x86\x88\t\x11[\x15a\x14\nW`\x01\x81a\x14\x07\x91\x90a\x1A\xE4V[\x90P[\x80\x91PP\x94\x93PPPPV[_a\x14;k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x13\xC1\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x14m\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x81Q\x14\x15\x80\x15a\x14\x91WP\x80\x80` \x01\x90Q\x81\x01\x90a\x14\x8F\x91\x90a\x1E\xB2V[\x15[\x15a\x14\xD3W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xCA\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xFD[PPPV[_`\x01`\x02\x83`\x03\x81\x11\x15a\x14\xF0Wa\x14\xEFa\x1E\xDDV[[a\x14\xFA\x91\x90a\x1F\nV[`\xFF\x16\x14\x90P\x91\x90PV[``a\x15\x12\x83\x83_a\x15\x1AV[\x90P\x92\x91PPV[``\x81G\x10\x15a\x15aW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15X\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xFD[_\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\x15\x89\x91\x90a\x1F\xA6V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x15\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x15\xC8V[``\x91P[P\x91P\x91Pa\x15\xD8\x86\x83\x83a\x15\xE3V[\x92PPP\x93\x92PPPV[``\x82a\x15\xF8Wa\x15\xF3\x82a\x16pV[a\x16hV[_\x82Q\x14\x80\x15a\x16\x1EWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x16`W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16W\x91\x90a\x17QV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x16iV[[\x93\x92PPPV[_\x81Q\x11\x15a\x16\x82W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x17;\x82a\x17\x12V[\x90P\x91\x90PV[a\x17K\x81a\x171V[\x82RPPV[_` \x82\x01\x90Pa\x17d_\x83\x01\x84a\x17BV[\x92\x91PPV[_\x80\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\x17\x83\x81a\x17nV[\x81\x14a\x17\x8DW_\x80\xFD[PV[_\x815\x90Pa\x17\x9E\x81a\x17zV[\x92\x91PPV[a\x17\xAD\x81a\x171V[\x81\x14a\x17\xB7W_\x80\xFD[PV[_\x815\x90Pa\x17\xC8\x81a\x17\xA4V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x17\xE5Wa\x17\xE4a\x17jV[[_a\x17\xF2\x86\x82\x87\x01a\x17\x90V[\x93PP` a\x18\x03\x86\x82\x87\x01a\x17\xBAV[\x92PP`@a\x18\x14\x86\x82\x87\x01a\x17\xBAV[\x91PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[a\x180\x81a\x18\x1EV[\x82RPPV[_`@\x82\x01\x90Pa\x18I_\x83\x01\x85a\x18'V[a\x18V` \x83\x01\x84a\x18'V[\x93\x92PPPV[_` \x82\x01\x90Pa\x18p_\x83\x01\x84a\x18'V[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x18\x99a\x18\x94a\x18\x8F\x84a\x17\x12V[a\x18vV[a\x17\x12V[\x90P\x91\x90PV[_a\x18\xAA\x82a\x18\x7FV[\x90P\x91\x90PV[_a\x18\xBB\x82a\x18\xA0V[\x90P\x91\x90PV[a\x18\xCB\x81a\x18\xB1V[\x82RPPV[_` \x82\x01\x90Pa\x18\xE4_\x83\x01\x84a\x18\xC2V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x19\0Wa\x18\xFFa\x17jV[[_a\x19\r\x85\x82\x86\x01a\x17\x90V[\x92PP` a\x19\x1E\x85\x82\x86\x01a\x17\xBAV[\x91PP\x92P\x92\x90PV[_a\x192\x82a\x18\xA0V[\x90P\x91\x90PV[a\x19B\x81a\x19(V[\x82RPPV[_` \x82\x01\x90Pa\x19[_\x83\x01\x84a\x199V[\x92\x91PPV[a\x19j\x81a\x18\x1EV[\x81\x14a\x19tW_\x80\xFD[PV[_\x81Q\x90Pa\x19\x85\x81a\x19aV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x19\xA0Wa\x19\x9Fa\x17jV[[_a\x19\xAD\x84\x82\x85\x01a\x19wV[\x91PP\x92\x91PPV[a\x19\xBF\x81a\x17nV[\x82RPPV[_`@\x82\x01\x90Pa\x19\xD8_\x83\x01\x85a\x19\xB6V[a\x19\xE5` \x83\x01\x84a\x17BV[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x1A\x02Wa\x1A\x01a\x17jV[[_a\x1A\x0F\x85\x82\x86\x01a\x19wV[\x92PP` a\x1A \x85\x82\x86\x01a\x19wV[\x91PP\x92P\x92\x90PV[_` \x82\x01\x90Pa\x1A=_\x83\x01\x84a\x19\xB6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x1Az\x82a\x18\x1EV[\x91Pa\x1A\x85\x83a\x18\x1EV[\x92P\x82\x82\x02a\x1A\x93\x81a\x18\x1EV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1A\xAAWa\x1A\xA9a\x1ACV[[P\x92\x91PPV[_a\x1A\xBB\x82a\x18\x1EV[\x91Pa\x1A\xC6\x83a\x18\x1EV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1A\xDEWa\x1A\xDDa\x1ACV[[\x92\x91PPV[_a\x1A\xEE\x82a\x18\x1EV[\x91Pa\x1A\xF9\x83a\x18\x1EV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1B\x11Wa\x1B\x10a\x1ACV[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x1B*\x82a\x1B\x17V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1B\\Wa\x1B[a\x1ACV[[\x81_\x03\x90P\x91\x90PV[a\x1Bo\x81a\x1B\x17V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x1B\x88_\x83\x01\x89a\x19\xB6V[a\x1B\x95` \x83\x01\x88a\x17BV[a\x1B\xA2`@\x83\x01\x87a\x17BV[a\x1B\xAF``\x83\x01\x86a\x17BV[a\x1B\xBC`\x80\x83\x01\x85a\x1BfV[a\x1B\xC9`\xA0\x83\x01\x84a\x1BfV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x1C\x0B\x82a\x18\x1EV[\x91Pa\x1C\x16\x83a\x18\x1EV[\x92P\x82a\x1C&Wa\x1C%a\x1B\xD4V[[\x82\x82\x04\x90P\x92\x91PPV[_a\x1C;\x82a\x18\x1EV[\x91Pa\x1CF\x83a\x18\x1EV[\x92P\x82a\x1CVWa\x1CUa\x1B\xD4V[[\x82\x82\x06\x90P\x92\x91PPV[_a\x1Ck\x82a\x18\x1EV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1C\x9DWa\x1C\x9Ca\x1ACV[[`\x01\x82\x01\x90P\x91\x90PV[_`@\x82\x01\x90Pa\x1C\xBB_\x83\x01\x85a\x17BV[a\x1C\xC8` \x83\x01\x84a\x18'V[\x93\x92PPPV[_``\x82\x01\x90Pa\x1C\xE2_\x83\x01\x86a\x17BV[a\x1C\xEF` \x83\x01\x85a\x17BV[a\x1C\xFC`@\x83\x01\x84a\x18'V[\x94\x93PPPPV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x1DYW\x80\x86\x04\x81\x11\x15a\x1D5Wa\x1D4a\x1ACV[[`\x01\x85\x16\x15a\x1DDW\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x1DR\x85a\x1D\x04V[\x94Pa\x1D\x19V[\x94P\x94\x92PPPV[_\x82a\x1DqW`\x01\x90Pa\x1E,V[\x81a\x1D~W_\x90Pa\x1E,V[\x81`\x01\x81\x14a\x1D\x94W`\x02\x81\x14a\x1D\x9EWa\x1D\xCDV[`\x01\x91PPa\x1E,V[`\xFF\x84\x11\x15a\x1D\xB0Wa\x1D\xAFa\x1ACV[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x1D\xC7Wa\x1D\xC6a\x1ACV[[Pa\x1E,V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1E\x02W\x82\x82\n\x90P\x83\x81\x11\x15a\x1D\xFDWa\x1D\xFCa\x1ACV[[a\x1E,V[a\x1E\x0F\x84\x84\x84`\x01a\x1D\x10V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x1E&Wa\x1E%a\x1ACV[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x1E=\x82a\x18\x1EV[\x91Pa\x1EH\x83a\x18\x1EV[\x92Pa\x1Eu\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x1DbV[\x90P\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x1E\x91\x81a\x1E}V[\x81\x14a\x1E\x9BW_\x80\xFD[PV[_\x81Q\x90Pa\x1E\xAC\x81a\x1E\x88V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1E\xC7Wa\x1E\xC6a\x17jV[[_a\x1E\xD4\x84\x82\x85\x01a\x1E\x9EV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_a\x1F\x14\x82a\x17nV[\x91Pa\x1F\x1F\x83a\x17nV[\x92P\x82a\x1F/Wa\x1F.a\x1B\xD4V[[\x82\x82\x06\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x1FkW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1FPV[_\x84\x84\x01RPPPPV[_a\x1F\x80\x82a\x1F:V[a\x1F\x8A\x81\x85a\x1FDV[\x93Pa\x1F\x9A\x81\x85` \x86\x01a\x1FNV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x1F\xB1\x82\x84a\x1FvV[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xFCG\x12\xBF\x17(\xC0\xCE!\x01\xC1{#\xFAg\xF6\xA1\x89K\x06\x9D9Z2\xDAR\x96C\xC6}\xDB\xEFdsolcC\0\x08\x15\x003";
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
        ///Calls the contract's `LIQUIDATION_THRESHOLD` (0x90a8ae9b) function
        pub fn liquidation_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 168, 174, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_DISCOUNT` (0xe3495569) function
        pub fn max_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 73, 85, 105], ())
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
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
    ///Custom Error type `InvalidLiquidationThreshold` with signature `InvalidLiquidationThreshold(uint256)` and selector `0xca8c452a`
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
        name = "InvalidLiquidationThreshold",
        abi = "InvalidLiquidationThreshold(uint256)"
    )]
    pub struct InvalidLiquidationThreshold {
        pub liquidation_threshold: ::ethers::core::types::U256,
    }
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
    ///Custom Error type `InvalidMaxDiscount` with signature `InvalidMaxDiscount(uint256)` and selector `0xe1a8222e`
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
    #[etherror(name = "InvalidMaxDiscount", abi = "InvalidMaxDiscount(uint256)")]
    pub struct InvalidMaxDiscount {
        pub max_discount: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidMaxDiscountsLength` with signature `InvalidMaxDiscountsLength(uint256)` and selector `0xd7589af6`
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
        name = "InvalidMaxDiscountsLength",
        abi = "InvalidMaxDiscountsLength(uint256)"
    )]
    pub struct InvalidMaxDiscountsLength {
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
    ///Custom Error type `InvalidTargetHealth` with signature `InvalidTargetHealth(uint256)` and selector `0x14214512`
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
    #[etherror(name = "InvalidTargetHealth", abi = "InvalidTargetHealth(uint256)")]
    pub struct InvalidTargetHealth {
        pub target_health: ::ethers::core::types::U256,
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
        InvalidLiquidationThreshold(InvalidLiquidationThreshold),
        InvalidLiquidationThresholdsLength(InvalidLiquidationThresholdsLength),
        InvalidMaxDiscount(InvalidMaxDiscount),
        InvalidMaxDiscountsLength(InvalidMaxDiscountsLength),
        InvalidReserveOraclesLength(InvalidReserveOraclesLength),
        InvalidTargetHealth(InvalidTargetHealth),
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
            if let Ok(decoded) = <InvalidLiquidationThreshold as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <InvalidLiquidationThresholdsLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidLiquidationThresholdsLength(decoded));
            }
            if let Ok(decoded) = <InvalidMaxDiscount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxDiscount(decoded));
            }
            if let Ok(decoded) = <InvalidMaxDiscountsLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMaxDiscountsLength(decoded));
            }
            if let Ok(decoded) = <InvalidReserveOraclesLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReserveOraclesLength(decoded));
            }
            if let Ok(decoded) = <InvalidTargetHealth as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTargetHealth(decoded));
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
                Self::InvalidLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidLiquidationThresholdsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMaxDiscountsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveOraclesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTargetHealth(element) => {
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
                    == <InvalidLiquidationThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidLiquidationThresholdsLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxDiscount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaxDiscountsLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidReserveOraclesLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTargetHealth as ::ethers::contract::EthError>::selector() => {
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
                Self::InvalidLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidLiquidationThresholdsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMaxDiscount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMaxDiscountsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidReserveOraclesLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTargetHealth(element) => {
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
    impl ::core::convert::From<InvalidLiquidationThreshold> for LiquidationErrors {
        fn from(value: InvalidLiquidationThreshold) -> Self {
            Self::InvalidLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<InvalidLiquidationThresholdsLength>
    for LiquidationErrors {
        fn from(value: InvalidLiquidationThresholdsLength) -> Self {
            Self::InvalidLiquidationThresholdsLength(value)
        }
    }
    impl ::core::convert::From<InvalidMaxDiscount> for LiquidationErrors {
        fn from(value: InvalidMaxDiscount) -> Self {
            Self::InvalidMaxDiscount(value)
        }
    }
    impl ::core::convert::From<InvalidMaxDiscountsLength> for LiquidationErrors {
        fn from(value: InvalidMaxDiscountsLength) -> Self {
            Self::InvalidMaxDiscountsLength(value)
        }
    }
    impl ::core::convert::From<InvalidReserveOraclesLength> for LiquidationErrors {
        fn from(value: InvalidReserveOraclesLength) -> Self {
            Self::InvalidReserveOraclesLength(value)
        }
    }
    impl ::core::convert::From<InvalidTargetHealth> for LiquidationErrors {
        fn from(value: InvalidTargetHealth) -> Self {
            Self::InvalidTargetHealth(value)
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
    #[ethevent(
        name = "Liquidate",
        abi = "Liquidate(address,address,uint8,uint256,uint256)"
    )]
    pub struct LiquidateFilter {
        #[ethevent(indexed)]
        pub initiator: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `LIQUIDATION_THRESHOLD` function with signature `LIQUIDATION_THRESHOLD()` and selector `0x90a8ae9b`
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
    #[ethcall(name = "LIQUIDATION_THRESHOLD", abi = "LIQUIDATION_THRESHOLD()")]
    pub struct LiquidationThresholdCall;
    ///Container type for all input parameters for the `MAX_DISCOUNT` function with signature `MAX_DISCOUNT()` and selector `0xe3495569`
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
    #[ethcall(name = "MAX_DISCOUNT", abi = "MAX_DISCOUNT()")]
    pub struct MaxDiscountCall;
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
        LiquidationThreshold(LiquidationThresholdCall),
        MaxDiscount(MaxDiscountCall),
        Pool(PoolCall),
        Protocol(ProtocolCall),
        ReserveOracle(ReserveOracleCall),
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
            if let Ok(decoded) = <LiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <MaxDiscountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxDiscount(decoded));
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
            if let Ok(decoded) = <ReserveOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveOracle(decoded));
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
                Self::LiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxDiscount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Protocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveOracle(element) => {
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
                Self::LiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Protocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveOracle(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<LiquidationThresholdCall> for LiquidationCalls {
        fn from(value: LiquidationThresholdCall) -> Self {
            Self::LiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<MaxDiscountCall> for LiquidationCalls {
        fn from(value: MaxDiscountCall) -> Self {
            Self::MaxDiscount(value)
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
    impl ::core::convert::From<ReserveOracleCall> for LiquidationCalls {
        fn from(value: ReserveOracleCall) -> Self {
            Self::ReserveOracle(value)
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
    ///Container type for all return fields from the `LIQUIDATION_THRESHOLD` function with signature `LIQUIDATION_THRESHOLD()` and selector `0x90a8ae9b`
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
    pub struct LiquidationThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_DISCOUNT` function with signature `MAX_DISCOUNT()` and selector `0xe3495569`
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
    pub struct MaxDiscountReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(uint8,address,address)` and selector `0x25840eda`
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
    pub struct LiquidateReturn {
        pub repay_amount: ::ethers::core::types::U256,
        pub gem_out: ::ethers::core::types::U256,
    }
}
