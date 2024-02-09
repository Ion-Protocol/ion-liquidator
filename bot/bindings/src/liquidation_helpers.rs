pub use liquidation_helpers::*;
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
pub mod liquidation_helpers {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getHealthRatio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getHealthRatio"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exchangeRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalDebt"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidationTypeBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidationTypeBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("locs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LiquidationTypeArgs",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "partialLiquidationBound",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "fullLiquidationBound",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "protocolLiquidationBound",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRepayAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRepayAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("normalizedDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exchangeRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetHealth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repayWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repayRad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getRepayAndCollateralRewardAmount",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRepayAndCollateralRewardAmount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("normalizedDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exchangeRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseDiscount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetHealth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dust"),
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
                                    name: ::std::borrow::ToOwned::to_owned("repayWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "collateralRewardAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("NotScalingDown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotScalingDown"),
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
                    ::std::borrow::ToOwned::to_owned("PRBMath_UD60x18_Sqrt_Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMath_UD60x18_Sqrt_Overflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("UD60x18"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATIONHELPERS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\x1F\x91\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\r9c\xE3\x14a\0NW\x80c)5\xC8\x01\x14a\0~W\x80c\xAB\x8D\xFA\x80\x14a\0\xB0W\x80c\xFC\x8B\x1D\xDE\x14a\0\xE1W[_\x80\xFD[a\0h`\x04\x806\x03\x81\x01\x90a\0c\x91\x90a\x15=V[a\x01\x12V[`@Qa\0u\x91\x90a\x15\xB0V[`@Q\x80\x91\x03\x90\xF3[a\0\x98`\x04\x806\x03\x81\x01\x90a\0\x93\x91\x90a\x17\xA7V[a\x01RV[`@Qa\0\xA7\x93\x92\x91\x90a\x17\xD3V[`@Q\x80\x91\x03\x90\xF3[a\0\xCA`\x04\x806\x03\x81\x01\x90a\0\xC5\x91\x90a\x18\x08V[a\x07(V[`@Qa\0\xD8\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xF3[a\0\xFB`\x04\x806\x03\x81\x01\x90a\0\xF6\x91\x90a\x18\xE0V[a\x08\xCAV[`@Qa\x01\t\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xF3[_\x80a\x012\x84\x86\x88a\x01$\x91\x90a\x19\xD1V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x01G\x83\x82a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP\x94\x93PPPPV[_\x80_\x80\x84`@\x01Q\x85` \x01Qa\x01j\x91\x90a\x19\xD1V[\x90P_\x81\x03a\x01\xC2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93P\x93P\x93PPa\x07!V[a\x01\xFA`-a\x01\xEC\x87`\xE0\x01Q\x88_\x01Qa\x01\xDD\x91\x90a\x19\xD1V[\x84a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x02K`\x1Ba\x02\x1C\x87`\xA0\x01Q\x88`\xC0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x023\x88`\x80\x01Q\x85a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02=\x91\x90a\x1A\x12V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01\0\x01\x81\x81RPPa\x02r\x85a\x01\0\x01Q\x86a\x01\0\x01Qa\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01 \x01\x81\x81RPPa\x02\x86\x85\x82a\x0B\x07V[\x85a\x01@\x01\x81\x81RPP_`\x02a\x02\xC9a\x02\xC4a\x02\xBF\x89a\x01@\x01Qa\x02\xB0\x8Ba\x01 \x01Qa\x0C.V[a\x02\xBA\x91\x90a\x1AEV[a\x0C\x9CV[a\x0C\xA5V[a\rRV[\x87a\x01\0\x01Qa\x02\xD9\x91\x90a\x1A\x12V[a\x02\xE3\x91\x90a\x1A\xB2V[\x90Pa\x03\x11\x86`\xE0\x01Qa\x03\x03\x88_\x01Q\x84a\r[\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x03=`\x1Ba\x03/\x88`\x80\x01Q\x85a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01\0\x01\x81\x81RPPa\x03d\x86a\x01\0\x01Q\x87a\x01\0\x01Qa\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01 \x01\x81\x81RPPa\x03\xB2a\x03\xAD`\x1Ba\x03\x9F\x89`\xE0\x01Qa\x03\x91\x87\x88a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[`\x04a\x03\xBE\x91\x90a\x1A\xE2V[\x86a\x01@\x01\x81\x81RPP`\x02a\x04\0a\x03\xFBa\x03\xF6\x89a\x01@\x01Qa\x03\xE7\x8Ba\x01 \x01Qa\x0C.V[a\x03\xF1\x91\x90a\x1BXV[a\x0C\x9CV[a\x0C\xA5V[a\rRV[\x87a\x01\0\x01Qa\x04\x10\x91\x90a\x1A\x12V[a\x04\x1A\x91\x90a\x1A\xB2V[\x90Pa\x04H\x86`\xE0\x01Qa\x04:\x88_\x01Q\x84a\r[\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P_a\x04\x83`-a\x04u\x87\x8A`\xE0\x01Q\x8B_\x01Qa\x04g\x91\x90a\x19\xD1V[a\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\x04\x99\x84\x83a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x04\xB5WP\x85\x87\x14[\x15a\x04\xE5W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x05\x9CV[a\x04\xEF\x88\x87a\r\xC0V[\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x14a\x05\x9BWa\x05/\x86\x89`\xE0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88_\x01Qa\x05=\x91\x90a\x19\xD1V[\x91Pa\x05R\x84\x83a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x05nWP\x85\x87\x14[\x15a\x05\x9AW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[[\x84\x87\x03a\x05\xC7W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[_a\x06\0`-a\x05\xF2\x88\x8C`\xE0\x01Q\x8D_\x01Qa\x05\xE4\x91\x90a\x19\xD1V[a\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\x15\x85\x82a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x061WP\x85\x87\x14[\x15a\x06aW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x06\xF0V[a\x06k\x89\x87a\x0FEV[\x95Pa\x06\x84\x86\x8A`\xE0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89_\x01Qa\x06\x92\x91\x90a\x19\xD1V[\x90Pa\x06\xA7\x85\x82a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x06\xC3WP\x85\x87\x14[\x15a\x06\xEFW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[\x85\x88\x03a\x07\x1BW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97P[PPPPP[\x91\x93\x90\x92PV[_\x80a\x07>`\x12\x88a\x10w\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x96P_a\x07_\x84\x89\x8Da\x07Q\x91\x90a\x19\xD1V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x89\x8Ba\x07n\x91\x90a\x19\xD1V[\x90P_\x81\x03a\x07\x84W_\x80\x93P\x93PPPa\x08\xBDV[_a\x07\x98\x82\x84a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x07\xBBW_\x80\x94P\x94PPPPa\x08\xBDV[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x07\xD4\x91\x90a\x1B\x99V[\x89a\x07\xDF\x91\x90a\x1A\x12V[\x90P\x89\x81\x11\x15a\x07\xEFW\x89a\x07\xF1V[\x80[\x90P_\x8C\x8Ea\x08\0\x91\x90a\x19\xD1V[\x90P_\x85a\x08\x17\x8B\x84a\x10\x8C\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x08!\x91\x90a\x1B\x99V[\x90P_a\x08N\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x08?\x91\x90a\x1B\x99V[\x8Ba\x10\xB9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8Ba\x08Y\x91\x90a\x1B\x99V[\x90Pa\x08n\x81\x83a\x10\xB9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x97Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88a\x08\x88\x91\x90a\x1A\xB2V[\x98P_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89a\x08\xA3\x91\x90a\x1B\xCCV[\x14a\x08\xB5W\x88a\x08\xB2\x90a\x1B\xFCV[\x98P[PPPPPPP[\x98P\x98\x96PPPPPPPV[_\x80_a\x08\xDD\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x07(V[\x80\x92P\x81\x94PPPa\x08\xF9`\x12\x8Aa\x10w\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x98P_a\t\x1A\x86\x8B\x8Fa\t\x0C\x91\x90a\x19\xD1V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x8B\x8Da\t)\x91\x90a\x19\xD1V[\x90P_\x81\x03a\t@W_\x80\x94P\x94PPPPa\n2V[_a\tT\x82\x84a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\txW_\x80\x95P\x95PPPPPa\n2V[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\x91\x91\x90a\x1B\x99V[\x8Ba\t\x9C\x91\x90a\x1A\x12V[\x90P\x8B\x81\x11\x15a\t\xACW\x8Ba\t\xAEV[\x80[\x90P_a\t\xDB\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\xCC\x91\x90a\x1B\x99V[\x8Fa\x10\x8C\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x83\x86\x11\x15a\t\xF6W_\x80\x97P\x97PPPPPPPa\n2V[\x85\x89a\n\x02\x91\x90a\x1A\x12V[\x84\x10\x15a\n\x1CW\x80\x84a\n\x15\x91\x90a\x1A\xB2V[\x96Pa\n+V[\x80\x86a\n(\x91\x90a\x1A\xB2V[\x96P[PPPPPP[\x99P\x99\x97PPPPPPPPV[_a\nb\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\n\x93r,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x83\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\n\xA8\x83\x83`\x12a\x11\xE5V[\x90P\x92\x91PPV[_a\n\xD9\x82r,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\n\xFF\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x80a\x0B:a\x0B5\x85`\xE0\x01Qa\x0B'\x86\x87a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[a\x0Boa\x0Bj\x86`\xE0\x01Qa\x0B\\\x87\x89`\xC0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[a\x0B\xBAa\x0B\xB5\x87`\x80\x01Qa\x0B\xA7\x89`\xA0\x01Qa\x0B\x99\x8A\x8C`\xC0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[a\x0B\xC4\x91\x90a\x1BXV[a\x0B\xCE\x91\x90a\x1AEV[\x90P_\x80\x82\x12\x90P_\x81a\x0B\xE2W\x82a\x0B\xEDV[\x82a\x0B\xEC\x90a\x1CCV[[\x90P_a\x0C\n`\x04c;\x9A\xCA\0\x84a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x82a\x0C\x17W\x80a\x0C\"V[\x80a\x0C!\x90a\x1CCV[[\x94PPPPP\x92\x91PPV[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\x94W\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x8B\x91\x90a\x15\xB0V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_\x80a\x0C\xB0\x83a\rRV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81a\x0C\xEAWa\x0C\xE9a\x1A\x85V[[\x04\x81\x11\x15a\r/W\x82`@Q\x7F\xED\xC26\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r&\x91\x90a\x1C\xC2V[`@Q\x80\x91\x03\x90\xFD[a\rJa\rEg\r\xE0\xB6\xB3\xA7d\0\0\x83\x02a\x12XV[a\x13\xFFV[\x91PP\x91\x90PV[_\x81\x90P\x91\x90PV[_a\ryg\r\xE0\xB6\xB3\xA7d\0\0\x83\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\r\xA3k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\r\xB8\x83\x83`\x1Ba\x11\xE5V[\x90P\x92\x91PPV[_\x80_\x90P[`\x05\x81\x10\x15a\x0F\x1AW_a\x0E\x12\x85_\x01Q\x86` \x01Q\x87`@\x01Q\x85`\x01\x89a\r\xEF\x91\x90a\x1B\x99V[a\r\xF9\x91\x90a\x1A\x12V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07(V[\x91PP_a\x0EL\x86_\x01Q\x87` \x01Q\x88`@\x01Q\x86\x89a\x0E3\x91\x90a\x1A\x12V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07(V[\x91PP\x85`@\x01Q\x86` \x01Qa\x0Ec\x91\x90a\x19\xD1V[\x82\x11\x15\x80\x15a\x0E\x93WP\x81\x86`\xC0\x01Qa\x0E}\x91\x90a\x1A\x12V[\x86`@\x01Q\x87` \x01Qa\x0E\x91\x91\x90a\x19\xD1V[\x10[\x15a\x0F\x05W\x85`@\x01Q\x86` \x01Qa\x0E\xAC\x91\x90a\x19\xD1V[\x81\x11\x15\x80\x15a\x0E\xDDWP\x80\x86`\xC0\x01Qa\x0E\xC6\x91\x90a\x1A\x12V[\x86`@\x01Q\x87` \x01Qa\x0E\xDA\x91\x90a\x19\xD1V[\x10\x15[\x15a\x0F\x04W`\x01\x83\x86a\x0E\xF0\x91\x90a\x1A\x12V[a\x0E\xFA\x91\x90a\x1B\x99V[\x93PPPPa\x0F?V[[PP\x80\x80a\x0F\x12\x90a\x1B\xFCV[\x91PPa\r\xC6V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90P[\x92\x91PPV[_\x80_\x90P[`\n\x81\x10\x15a\x105W_a\x0F\x8B\x85_\x01Q\x86` \x01Q\x87`@\x01Q\x85\x88a\x0Fr\x91\x90a\x1A\x12V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07(V[\x91PP_a\x0F\xD1\x86_\x01Q\x87` \x01Q\x88`@\x01Q`\x01\x87\x8Aa\x0F\xAE\x91\x90a\x1A\x12V[a\x0F\xB8\x91\x90a\x1A\x12V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07(V[\x91PP\x85`@\x01Q\x86` \x01Qa\x0F\xE8\x91\x90a\x19\xD1V[\x82\x11\x15a\x10 W\x85`@\x01Q\x86` \x01Qa\x10\x03\x91\x90a\x19\xD1V[\x81\x11a\x10\x1FW\x82\x85a\x10\x15\x91\x90a\x1A\x12V[\x93PPPPa\x10qV[[PP\x80\x80a\x10-\x90a\x1B\xFCV[\x91PPa\x0FKV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10h\x90a\x1D[V[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[_a\x10\x84\x83\x83`\x1Ba\x14\x08V[\x90P\x92\x91PPV[_a\x10\xB1\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x14{\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x10\xDEk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x14{\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\x11\x1EW\x83\x82\x81a\x11\x14Wa\x11\x13a\x1A\x85V[[\x04\x92PPPa\x11\xDEV[\x80\x84\x11a\x11WW`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_\x81\x83\x11a\x12,W\x82\x82`@Q\x7F\xA5\xD1i!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12#\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x128\x91\x90a\x1B\x99V[`\na\x12D\x91\x90a\x1E\xA8V[\x84a\x12O\x91\x90a\x1A\xB2V[\x90P\x93\x92PPPV[_\x80\x82\x03a\x12hW_\x90Pa\x13\xFAV[_\x82\x90P`\x01\x91Pp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10a\x12\x97W`\x80\x81\x90\x1C\x90P`@\x82\x90\x1B\x91P[h\x01\0\0\0\0\0\0\0\0\x81\x10a\x12\xB6W`@\x81\x90\x1C\x90P` \x82\x90\x1B\x91P[d\x01\0\0\0\0\x81\x10a\x12\xD1W` \x81\x90\x1C\x90P`\x10\x82\x90\x1B\x91P[b\x01\0\0\x81\x10a\x12\xEAW`\x10\x81\x90\x1C\x90P`\x08\x82\x90\x1B\x91P[a\x01\0\x81\x10a\x13\x02W`\x08\x81\x90\x1C\x90P`\x04\x82\x90\x1B\x91P[`\x10\x81\x10a\x13\x19W`\x04\x81\x90\x1C\x90P`\x02\x82\x90\x1B\x91P[`\x04\x81\x10a\x13)W`\x01\x82\x90\x1B\x91P[`\x01\x82\x84\x81a\x13;Wa\x13:a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13TWa\x13Sa\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13mWa\x13la\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\x86Wa\x13\x85a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\x9FWa\x13\x9Ea\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xB8Wa\x13\xB7a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xD1Wa\x13\xD0a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P_\x82\x84\x81a\x13\xE9Wa\x13\xE8a\x1A\x85V[[\x04\x90P\x80\x83\x10a\x13\xF7W\x80\x92P[PP[\x91\x90PV[_\x81\x90P\x91\x90PV[_\x81\x83\x10a\x14OW\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14F\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x14[\x91\x90a\x1B\x99V[`\na\x14g\x91\x90a\x1E\xA8V[\x84a\x14r\x91\x90a\x19\xD1V[\x90P\x93\x92PPPV[_\x80a\x14\x88\x86\x86\x86a\x10\xE6V[\x90Pa\x14\x93\x83a\x14\xD0V[\x80\x15a\x14\xAFWP_\x84\x80a\x14\xAAWa\x14\xA9a\x1A\x85V[[\x86\x88\t\x11[\x15a\x14\xC4W`\x01\x81a\x14\xC1\x91\x90a\x1A\x12V[\x90P[\x80\x91PP\x94\x93PPPPV[_`\x01`\x02\x83`\x03\x81\x11\x15a\x14\xE8Wa\x14\xE7a\x1E\xF2V[[a\x14\xF2\x91\x90a\x1F+V[`\xFF\x16\x14\x90P\x91\x90PV[_`@Q\x90P\x90V[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x15\x1C\x81a\x15\nV[\x81\x14a\x15&W_\x80\xFD[PV[_\x815\x90Pa\x157\x81a\x15\x13V[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x15UWa\x15Ta\x15\x06V[[_a\x15b\x87\x82\x88\x01a\x15)V[\x94PP` a\x15s\x87\x82\x88\x01a\x15)V[\x93PP`@a\x15\x84\x87\x82\x88\x01a\x15)V[\x92PP``a\x15\x95\x87\x82\x88\x01a\x15)V[\x91PP\x92\x95\x91\x94P\x92PV[a\x15\xAA\x81a\x15\nV[\x82RPPV[_` \x82\x01\x90Pa\x15\xC3_\x83\x01\x84a\x15\xA1V[\x92\x91PPV[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x16\x13\x82a\x15\xCDV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x162Wa\x161a\x15\xDDV[[\x80`@RPPPV[_a\x16Da\x14\xFDV[\x90Pa\x16P\x82\x82a\x16\nV[\x91\x90PV[_\x81\x90P\x91\x90PV[a\x16g\x81a\x16UV[\x81\x14a\x16qW_\x80\xFD[PV[_\x815\x90Pa\x16\x82\x81a\x16^V[\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a\x16\x9EWa\x16\x9Da\x15\xC9V[[a\x16\xA9a\x01\x80a\x16;V[\x90P_a\x16\xB8\x84\x82\x85\x01a\x15)V[_\x83\x01RP` a\x16\xCB\x84\x82\x85\x01a\x15)V[` \x83\x01RP`@a\x16\xDF\x84\x82\x85\x01a\x15)V[`@\x83\x01RP``a\x16\xF3\x84\x82\x85\x01a\x15)V[``\x83\x01RP`\x80a\x17\x07\x84\x82\x85\x01a\x15)V[`\x80\x83\x01RP`\xA0a\x17\x1B\x84\x82\x85\x01a\x15)V[`\xA0\x83\x01RP`\xC0a\x17/\x84\x82\x85\x01a\x15)V[`\xC0\x83\x01RP`\xE0a\x17C\x84\x82\x85\x01a\x15)V[`\xE0\x83\x01RPa\x01\0a\x17X\x84\x82\x85\x01a\x15)V[a\x01\0\x83\x01RPa\x01 a\x17n\x84\x82\x85\x01a\x15)V[a\x01 \x83\x01RPa\x01@a\x17\x84\x84\x82\x85\x01a\x16tV[a\x01@\x83\x01RPa\x01`a\x17\x9A\x84\x82\x85\x01a\x15)V[a\x01`\x83\x01RP\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a\x17\xBDWa\x17\xBCa\x15\x06V[[_a\x17\xCA\x84\x82\x85\x01a\x16\x88V[\x91PP\x92\x91PPV[_``\x82\x01\x90Pa\x17\xE6_\x83\x01\x86a\x15\xA1V[a\x17\xF3` \x83\x01\x85a\x15\xA1V[a\x18\0`@\x83\x01\x84a\x15\xA1V[\x94\x93PPPPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\x18%Wa\x18$a\x15\x06V[[_a\x182\x8B\x82\x8C\x01a\x15)V[\x98PP` a\x18C\x8B\x82\x8C\x01a\x15)V[\x97PP`@a\x18T\x8B\x82\x8C\x01a\x15)V[\x96PP``a\x18e\x8B\x82\x8C\x01a\x15)V[\x95PP`\x80a\x18v\x8B\x82\x8C\x01a\x15)V[\x94PP`\xA0a\x18\x87\x8B\x82\x8C\x01a\x15)V[\x93PP`\xC0a\x18\x98\x8B\x82\x8C\x01a\x15)V[\x92PP`\xE0a\x18\xA9\x8B\x82\x8C\x01a\x15)V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_`@\x82\x01\x90Pa\x18\xCC_\x83\x01\x85a\x15\xA1V[a\x18\xD9` \x83\x01\x84a\x15\xA1V[\x93\x92PPPV[_\x80_\x80_\x80_\x80_a\x01 \x8A\x8C\x03\x12\x15a\x18\xFEWa\x18\xFDa\x15\x06V[[_a\x19\x0B\x8C\x82\x8D\x01a\x15)V[\x99PP` a\x19\x1C\x8C\x82\x8D\x01a\x15)V[\x98PP`@a\x19-\x8C\x82\x8D\x01a\x15)V[\x97PP``a\x19>\x8C\x82\x8D\x01a\x15)V[\x96PP`\x80a\x19O\x8C\x82\x8D\x01a\x15)V[\x95PP`\xA0a\x19`\x8C\x82\x8D\x01a\x15)V[\x94PP`\xC0a\x19q\x8C\x82\x8D\x01a\x15)V[\x93PP`\xE0a\x19\x82\x8C\x82\x8D\x01a\x15)V[\x92PPa\x01\0a\x19\x94\x8C\x82\x8D\x01a\x15)V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x19\xDB\x82a\x15\nV[\x91Pa\x19\xE6\x83a\x15\nV[\x92P\x82\x82\x02a\x19\xF4\x81a\x15\nV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1A\x0BWa\x1A\na\x19\xA4V[[P\x92\x91PPV[_a\x1A\x1C\x82a\x15\nV[\x91Pa\x1A'\x83a\x15\nV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1A?Wa\x1A>a\x19\xA4V[[\x92\x91PPV[_a\x1AO\x82a\x16UV[\x91Pa\x1AZ\x83a\x16UV[\x92P\x82\x82\x03\x90P\x81\x81\x12_\x84\x12\x16\x82\x82\x13_\x85\x12\x15\x16\x17\x15a\x1A\x7FWa\x1A~a\x19\xA4V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x1A\xBC\x82a\x15\nV[\x91Pa\x1A\xC7\x83a\x15\nV[\x92P\x82a\x1A\xD7Wa\x1A\xD6a\x1A\x85V[[\x82\x82\x04\x90P\x92\x91PPV[_a\x1A\xEC\x82a\x16UV[\x91Pa\x1A\xF7\x83a\x16UV[\x92P\x82\x82\x02a\x1B\x05\x81a\x16UV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14_\x84\x12\x16\x15a\x1B<Wa\x1B;a\x19\xA4V[[\x82\x82\x05\x84\x14\x83\x15\x17a\x1BQWa\x1BPa\x19\xA4V[[P\x92\x91PPV[_a\x1Bb\x82a\x16UV[\x91Pa\x1Bm\x83a\x16UV[\x92P\x82\x82\x01\x90P\x82\x81\x12\x15_\x83\x12\x16\x83\x82\x12_\x84\x12\x15\x16\x17\x15a\x1B\x93Wa\x1B\x92a\x19\xA4V[[\x92\x91PPV[_a\x1B\xA3\x82a\x15\nV[\x91Pa\x1B\xAE\x83a\x15\nV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1B\xC6Wa\x1B\xC5a\x19\xA4V[[\x92\x91PPV[_a\x1B\xD6\x82a\x15\nV[\x91Pa\x1B\xE1\x83a\x15\nV[\x92P\x82a\x1B\xF1Wa\x1B\xF0a\x1A\x85V[[\x82\x82\x06\x90P\x92\x91PPV[_a\x1C\x06\x82a\x15\nV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1C8Wa\x1C7a\x19\xA4V[[`\x01\x82\x01\x90P\x91\x90PV[_a\x1CM\x82a\x16UV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1C\x7FWa\x1C~a\x19\xA4V[[\x81_\x03\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x1C\xACa\x1C\xA7a\x1C\xA2\x84a\x15\nV[a\x1C\x89V[a\x15\nV[\x90P\x91\x90PV[a\x1C\xBC\x81a\x1C\x92V[\x82RPPV[_` \x82\x01\x90Pa\x1C\xD5_\x83\x01\x84a\x1C\xB3V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCould not find rounded protocol _\x82\x01R\x7Fliquidation bound\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1DE`1\x83a\x1C\xDBV[\x91Pa\x1DP\x82a\x1C\xEBV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1Dr\x81a\x1D9V[\x90P\x91\x90PV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x1D\xCEW\x80\x86\x04\x81\x11\x15a\x1D\xAAWa\x1D\xA9a\x19\xA4V[[`\x01\x85\x16\x15a\x1D\xB9W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x1D\xC7\x85a\x1DyV[\x94Pa\x1D\x8EV[\x94P\x94\x92PPPV[_\x82a\x1D\xE6W`\x01\x90Pa\x1E\xA1V[\x81a\x1D\xF3W_\x90Pa\x1E\xA1V[\x81`\x01\x81\x14a\x1E\tW`\x02\x81\x14a\x1E\x13Wa\x1EBV[`\x01\x91PPa\x1E\xA1V[`\xFF\x84\x11\x15a\x1E%Wa\x1E$a\x19\xA4V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x1E<Wa\x1E;a\x19\xA4V[[Pa\x1E\xA1V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1EwW\x82\x82\n\x90P\x83\x81\x11\x15a\x1ErWa\x1Eqa\x19\xA4V[[a\x1E\xA1V[a\x1E\x84\x84\x84\x84`\x01a\x1D\x85V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x1E\x9BWa\x1E\x9Aa\x19\xA4V[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x1E\xB2\x82a\x15\nV[\x91Pa\x1E\xBD\x83a\x15\nV[\x92Pa\x1E\xEA\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x1D\xD7V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_a\x1F5\x82a\x1F\x1FV[\x91Pa\x1F@\x83a\x1F\x1FV[\x92P\x82a\x1FPWa\x1FOa\x1A\x85V[[\x82\x82\x06\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xBC\x04`\xC9txM\xF2~\xBB[+>\xF2\x9D\x81\x06\x9E96\x15\x0C\x18\xC1:L1_g\xAC\x93\xE5dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATIONHELPERS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\r9c\xE3\x14a\0NW\x80c)5\xC8\x01\x14a\0~W\x80c\xAB\x8D\xFA\x80\x14a\0\xB0W\x80c\xFC\x8B\x1D\xDE\x14a\0\xE1W[_\x80\xFD[a\0h`\x04\x806\x03\x81\x01\x90a\0c\x91\x90a\x15=V[a\x01\x12V[`@Qa\0u\x91\x90a\x15\xB0V[`@Q\x80\x91\x03\x90\xF3[a\0\x98`\x04\x806\x03\x81\x01\x90a\0\x93\x91\x90a\x17\xA7V[a\x01RV[`@Qa\0\xA7\x93\x92\x91\x90a\x17\xD3V[`@Q\x80\x91\x03\x90\xF3[a\0\xCA`\x04\x806\x03\x81\x01\x90a\0\xC5\x91\x90a\x18\x08V[a\x07(V[`@Qa\0\xD8\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xF3[a\0\xFB`\x04\x806\x03\x81\x01\x90a\0\xF6\x91\x90a\x18\xE0V[a\x08\xCAV[`@Qa\x01\t\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xF3[_\x80a\x012\x84\x86\x88a\x01$\x91\x90a\x19\xD1V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x01G\x83\x82a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP\x94\x93PPPPV[_\x80_\x80\x84`@\x01Q\x85` \x01Qa\x01j\x91\x90a\x19\xD1V[\x90P_\x81\x03a\x01\xC2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93P\x93P\x93PPa\x07!V[a\x01\xFA`-a\x01\xEC\x87`\xE0\x01Q\x88_\x01Qa\x01\xDD\x91\x90a\x19\xD1V[\x84a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x02K`\x1Ba\x02\x1C\x87`\xA0\x01Q\x88`\xC0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x023\x88`\x80\x01Q\x85a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02=\x91\x90a\x1A\x12V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01\0\x01\x81\x81RPPa\x02r\x85a\x01\0\x01Q\x86a\x01\0\x01Qa\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01 \x01\x81\x81RPPa\x02\x86\x85\x82a\x0B\x07V[\x85a\x01@\x01\x81\x81RPP_`\x02a\x02\xC9a\x02\xC4a\x02\xBF\x89a\x01@\x01Qa\x02\xB0\x8Ba\x01 \x01Qa\x0C.V[a\x02\xBA\x91\x90a\x1AEV[a\x0C\x9CV[a\x0C\xA5V[a\rRV[\x87a\x01\0\x01Qa\x02\xD9\x91\x90a\x1A\x12V[a\x02\xE3\x91\x90a\x1A\xB2V[\x90Pa\x03\x11\x86`\xE0\x01Qa\x03\x03\x88_\x01Q\x84a\r[\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x03=`\x1Ba\x03/\x88`\x80\x01Q\x85a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01\0\x01\x81\x81RPPa\x03d\x86a\x01\0\x01Q\x87a\x01\0\x01Qa\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01 \x01\x81\x81RPPa\x03\xB2a\x03\xAD`\x1Ba\x03\x9F\x89`\xE0\x01Qa\x03\x91\x87\x88a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\x9B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[`\x04a\x03\xBE\x91\x90a\x1A\xE2V[\x86a\x01@\x01\x81\x81RPP`\x02a\x04\0a\x03\xFBa\x03\xF6\x89a\x01@\x01Qa\x03\xE7\x8Ba\x01 \x01Qa\x0C.V[a\x03\xF1\x91\x90a\x1BXV[a\x0C\x9CV[a\x0C\xA5V[a\rRV[\x87a\x01\0\x01Qa\x04\x10\x91\x90a\x1A\x12V[a\x04\x1A\x91\x90a\x1A\xB2V[\x90Pa\x04H\x86`\xE0\x01Qa\x04:\x88_\x01Q\x84a\r[\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P_a\x04\x83`-a\x04u\x87\x8A`\xE0\x01Q\x8B_\x01Qa\x04g\x91\x90a\x19\xD1V[a\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\x04\x99\x84\x83a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x04\xB5WP\x85\x87\x14[\x15a\x04\xE5W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x05\x9CV[a\x04\xEF\x88\x87a\r\xC0V[\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x14a\x05\x9BWa\x05/\x86\x89`\xE0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88_\x01Qa\x05=\x91\x90a\x19\xD1V[\x91Pa\x05R\x84\x83a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x05nWP\x85\x87\x14[\x15a\x05\x9AW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[[\x84\x87\x03a\x05\xC7W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[_a\x06\0`-a\x05\xF2\x88\x8C`\xE0\x01Q\x8D_\x01Qa\x05\xE4\x91\x90a\x19\xD1V[a\n\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\x15\x85\x82a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x061WP\x85\x87\x14[\x15a\x06aW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x06\xF0V[a\x06k\x89\x87a\x0FEV[\x95Pa\x06\x84\x86\x8A`\xE0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89_\x01Qa\x06\x92\x91\x90a\x19\xD1V[\x90Pa\x06\xA7\x85\x82a\nj\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x06\xC3WP\x85\x87\x14[\x15a\x06\xEFW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[\x85\x88\x03a\x07\x1BW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97P[PPPPP[\x91\x93\x90\x92PV[_\x80a\x07>`\x12\x88a\x10w\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x96P_a\x07_\x84\x89\x8Da\x07Q\x91\x90a\x19\xD1V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x89\x8Ba\x07n\x91\x90a\x19\xD1V[\x90P_\x81\x03a\x07\x84W_\x80\x93P\x93PPPa\x08\xBDV[_a\x07\x98\x82\x84a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x07\xBBW_\x80\x94P\x94PPPPa\x08\xBDV[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x07\xD4\x91\x90a\x1B\x99V[\x89a\x07\xDF\x91\x90a\x1A\x12V[\x90P\x89\x81\x11\x15a\x07\xEFW\x89a\x07\xF1V[\x80[\x90P_\x8C\x8Ea\x08\0\x91\x90a\x19\xD1V[\x90P_\x85a\x08\x17\x8B\x84a\x10\x8C\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x08!\x91\x90a\x1B\x99V[\x90P_a\x08N\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x08?\x91\x90a\x1B\x99V[\x8Ba\x10\xB9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8Ba\x08Y\x91\x90a\x1B\x99V[\x90Pa\x08n\x81\x83a\x10\xB9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x97Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88a\x08\x88\x91\x90a\x1A\xB2V[\x98P_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89a\x08\xA3\x91\x90a\x1B\xCCV[\x14a\x08\xB5W\x88a\x08\xB2\x90a\x1B\xFCV[\x98P[PPPPPPP[\x98P\x98\x96PPPPPPPV[_\x80_a\x08\xDD\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x07(V[\x80\x92P\x81\x94PPPa\x08\xF9`\x12\x8Aa\x10w\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x98P_a\t\x1A\x86\x8B\x8Fa\t\x0C\x91\x90a\x19\xD1V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x8B\x8Da\t)\x91\x90a\x19\xD1V[\x90P_\x81\x03a\t@W_\x80\x94P\x94PPPPa\n2V[_a\tT\x82\x84a\r\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\txW_\x80\x95P\x95PPPPPa\n2V[_\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\x91\x91\x90a\x1B\x99V[\x8Ba\t\x9C\x91\x90a\x1A\x12V[\x90P\x8B\x81\x11\x15a\t\xACW\x8Ba\t\xAEV[\x80[\x90P_a\t\xDB\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\xCC\x91\x90a\x1B\x99V[\x8Fa\x10\x8C\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x83\x86\x11\x15a\t\xF6W_\x80\x97P\x97PPPPPPPa\n2V[\x85\x89a\n\x02\x91\x90a\x1A\x12V[\x84\x10\x15a\n\x1CW\x80\x84a\n\x15\x91\x90a\x1A\xB2V[\x96Pa\n+V[\x80\x86a\n(\x91\x90a\x1A\xB2V[\x96P[PPPPPP[\x99P\x99\x97PPPPPPPPV[_a\nb\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\n\x93r,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x83\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\n\xA8\x83\x83`\x12a\x11\xE5V[\x90P\x92\x91PPV[_a\n\xD9\x82r,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\n\xFF\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x80a\x0B:a\x0B5\x85`\xE0\x01Qa\x0B'\x86\x87a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[a\x0Boa\x0Bj\x86`\xE0\x01Qa\x0B\\\x87\x89`\xC0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[a\x0B\xBAa\x0B\xB5\x87`\x80\x01Qa\x0B\xA7\x89`\xA0\x01Qa\x0B\x99\x8A\x8C`\xC0\x01Qa\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C.V[a\x0B\xC4\x91\x90a\x1BXV[a\x0B\xCE\x91\x90a\x1AEV[\x90P_\x80\x82\x12\x90P_\x81a\x0B\xE2W\x82a\x0B\xEDV[\x82a\x0B\xEC\x90a\x1CCV[[\x90P_a\x0C\n`\x04c;\x9A\xCA\0\x84a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x82a\x0C\x17W\x80a\x0C\"V[\x80a\x0C!\x90a\x1CCV[[\x94PPPPP\x92\x91PPV[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\x94W\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x8B\x91\x90a\x15\xB0V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_\x80a\x0C\xB0\x83a\rRV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81a\x0C\xEAWa\x0C\xE9a\x1A\x85V[[\x04\x81\x11\x15a\r/W\x82`@Q\x7F\xED\xC26\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r&\x91\x90a\x1C\xC2V[`@Q\x80\x91\x03\x90\xFD[a\rJa\rEg\r\xE0\xB6\xB3\xA7d\0\0\x83\x02a\x12XV[a\x13\xFFV[\x91PP\x91\x90PV[_\x81\x90P\x91\x90PV[_a\ryg\r\xE0\xB6\xB3\xA7d\0\0\x83\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\r\xA3k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x10\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\r\xB8\x83\x83`\x1Ba\x11\xE5V[\x90P\x92\x91PPV[_\x80_\x90P[`\x05\x81\x10\x15a\x0F\x1AW_a\x0E\x12\x85_\x01Q\x86` \x01Q\x87`@\x01Q\x85`\x01\x89a\r\xEF\x91\x90a\x1B\x99V[a\r\xF9\x91\x90a\x1A\x12V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07(V[\x91PP_a\x0EL\x86_\x01Q\x87` \x01Q\x88`@\x01Q\x86\x89a\x0E3\x91\x90a\x1A\x12V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07(V[\x91PP\x85`@\x01Q\x86` \x01Qa\x0Ec\x91\x90a\x19\xD1V[\x82\x11\x15\x80\x15a\x0E\x93WP\x81\x86`\xC0\x01Qa\x0E}\x91\x90a\x1A\x12V[\x86`@\x01Q\x87` \x01Qa\x0E\x91\x91\x90a\x19\xD1V[\x10[\x15a\x0F\x05W\x85`@\x01Q\x86` \x01Qa\x0E\xAC\x91\x90a\x19\xD1V[\x81\x11\x15\x80\x15a\x0E\xDDWP\x80\x86`\xC0\x01Qa\x0E\xC6\x91\x90a\x1A\x12V[\x86`@\x01Q\x87` \x01Qa\x0E\xDA\x91\x90a\x19\xD1V[\x10\x15[\x15a\x0F\x04W`\x01\x83\x86a\x0E\xF0\x91\x90a\x1A\x12V[a\x0E\xFA\x91\x90a\x1B\x99V[\x93PPPPa\x0F?V[[PP\x80\x80a\x0F\x12\x90a\x1B\xFCV[\x91PPa\r\xC6V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90P[\x92\x91PPV[_\x80_\x90P[`\n\x81\x10\x15a\x105W_a\x0F\x8B\x85_\x01Q\x86` \x01Q\x87`@\x01Q\x85\x88a\x0Fr\x91\x90a\x1A\x12V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07(V[\x91PP_a\x0F\xD1\x86_\x01Q\x87` \x01Q\x88`@\x01Q`\x01\x87\x8Aa\x0F\xAE\x91\x90a\x1A\x12V[a\x0F\xB8\x91\x90a\x1A\x12V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07(V[\x91PP\x85`@\x01Q\x86` \x01Qa\x0F\xE8\x91\x90a\x19\xD1V[\x82\x11\x15a\x10 W\x85`@\x01Q\x86` \x01Qa\x10\x03\x91\x90a\x19\xD1V[\x81\x11a\x10\x1FW\x82\x85a\x10\x15\x91\x90a\x1A\x12V[\x93PPPPa\x10qV[[PP\x80\x80a\x10-\x90a\x1B\xFCV[\x91PPa\x0FKV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10h\x90a\x1D[V[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[_a\x10\x84\x83\x83`\x1Ba\x14\x08V[\x90P\x92\x91PPV[_a\x10\xB1\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x14{\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x10\xDEk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x14{\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\x11\x1EW\x83\x82\x81a\x11\x14Wa\x11\x13a\x1A\x85V[[\x04\x92PPPa\x11\xDEV[\x80\x84\x11a\x11WW`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_\x81\x83\x11a\x12,W\x82\x82`@Q\x7F\xA5\xD1i!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12#\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x128\x91\x90a\x1B\x99V[`\na\x12D\x91\x90a\x1E\xA8V[\x84a\x12O\x91\x90a\x1A\xB2V[\x90P\x93\x92PPPV[_\x80\x82\x03a\x12hW_\x90Pa\x13\xFAV[_\x82\x90P`\x01\x91Pp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10a\x12\x97W`\x80\x81\x90\x1C\x90P`@\x82\x90\x1B\x91P[h\x01\0\0\0\0\0\0\0\0\x81\x10a\x12\xB6W`@\x81\x90\x1C\x90P` \x82\x90\x1B\x91P[d\x01\0\0\0\0\x81\x10a\x12\xD1W` \x81\x90\x1C\x90P`\x10\x82\x90\x1B\x91P[b\x01\0\0\x81\x10a\x12\xEAW`\x10\x81\x90\x1C\x90P`\x08\x82\x90\x1B\x91P[a\x01\0\x81\x10a\x13\x02W`\x08\x81\x90\x1C\x90P`\x04\x82\x90\x1B\x91P[`\x10\x81\x10a\x13\x19W`\x04\x81\x90\x1C\x90P`\x02\x82\x90\x1B\x91P[`\x04\x81\x10a\x13)W`\x01\x82\x90\x1B\x91P[`\x01\x82\x84\x81a\x13;Wa\x13:a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13TWa\x13Sa\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13mWa\x13la\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\x86Wa\x13\x85a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\x9FWa\x13\x9Ea\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xB8Wa\x13\xB7a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xD1Wa\x13\xD0a\x1A\x85V[[\x04\x83\x01\x90\x1C\x91P_\x82\x84\x81a\x13\xE9Wa\x13\xE8a\x1A\x85V[[\x04\x90P\x80\x83\x10a\x13\xF7W\x80\x92P[PP[\x91\x90PV[_\x81\x90P\x91\x90PV[_\x81\x83\x10a\x14OW\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14F\x92\x91\x90a\x18\xB9V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x14[\x91\x90a\x1B\x99V[`\na\x14g\x91\x90a\x1E\xA8V[\x84a\x14r\x91\x90a\x19\xD1V[\x90P\x93\x92PPPV[_\x80a\x14\x88\x86\x86\x86a\x10\xE6V[\x90Pa\x14\x93\x83a\x14\xD0V[\x80\x15a\x14\xAFWP_\x84\x80a\x14\xAAWa\x14\xA9a\x1A\x85V[[\x86\x88\t\x11[\x15a\x14\xC4W`\x01\x81a\x14\xC1\x91\x90a\x1A\x12V[\x90P[\x80\x91PP\x94\x93PPPPV[_`\x01`\x02\x83`\x03\x81\x11\x15a\x14\xE8Wa\x14\xE7a\x1E\xF2V[[a\x14\xF2\x91\x90a\x1F+V[`\xFF\x16\x14\x90P\x91\x90PV[_`@Q\x90P\x90V[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x15\x1C\x81a\x15\nV[\x81\x14a\x15&W_\x80\xFD[PV[_\x815\x90Pa\x157\x81a\x15\x13V[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x15UWa\x15Ta\x15\x06V[[_a\x15b\x87\x82\x88\x01a\x15)V[\x94PP` a\x15s\x87\x82\x88\x01a\x15)V[\x93PP`@a\x15\x84\x87\x82\x88\x01a\x15)V[\x92PP``a\x15\x95\x87\x82\x88\x01a\x15)V[\x91PP\x92\x95\x91\x94P\x92PV[a\x15\xAA\x81a\x15\nV[\x82RPPV[_` \x82\x01\x90Pa\x15\xC3_\x83\x01\x84a\x15\xA1V[\x92\x91PPV[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x16\x13\x82a\x15\xCDV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x162Wa\x161a\x15\xDDV[[\x80`@RPPPV[_a\x16Da\x14\xFDV[\x90Pa\x16P\x82\x82a\x16\nV[\x91\x90PV[_\x81\x90P\x91\x90PV[a\x16g\x81a\x16UV[\x81\x14a\x16qW_\x80\xFD[PV[_\x815\x90Pa\x16\x82\x81a\x16^V[\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a\x16\x9EWa\x16\x9Da\x15\xC9V[[a\x16\xA9a\x01\x80a\x16;V[\x90P_a\x16\xB8\x84\x82\x85\x01a\x15)V[_\x83\x01RP` a\x16\xCB\x84\x82\x85\x01a\x15)V[` \x83\x01RP`@a\x16\xDF\x84\x82\x85\x01a\x15)V[`@\x83\x01RP``a\x16\xF3\x84\x82\x85\x01a\x15)V[``\x83\x01RP`\x80a\x17\x07\x84\x82\x85\x01a\x15)V[`\x80\x83\x01RP`\xA0a\x17\x1B\x84\x82\x85\x01a\x15)V[`\xA0\x83\x01RP`\xC0a\x17/\x84\x82\x85\x01a\x15)V[`\xC0\x83\x01RP`\xE0a\x17C\x84\x82\x85\x01a\x15)V[`\xE0\x83\x01RPa\x01\0a\x17X\x84\x82\x85\x01a\x15)V[a\x01\0\x83\x01RPa\x01 a\x17n\x84\x82\x85\x01a\x15)V[a\x01 \x83\x01RPa\x01@a\x17\x84\x84\x82\x85\x01a\x16tV[a\x01@\x83\x01RPa\x01`a\x17\x9A\x84\x82\x85\x01a\x15)V[a\x01`\x83\x01RP\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a\x17\xBDWa\x17\xBCa\x15\x06V[[_a\x17\xCA\x84\x82\x85\x01a\x16\x88V[\x91PP\x92\x91PPV[_``\x82\x01\x90Pa\x17\xE6_\x83\x01\x86a\x15\xA1V[a\x17\xF3` \x83\x01\x85a\x15\xA1V[a\x18\0`@\x83\x01\x84a\x15\xA1V[\x94\x93PPPPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\x18%Wa\x18$a\x15\x06V[[_a\x182\x8B\x82\x8C\x01a\x15)V[\x98PP` a\x18C\x8B\x82\x8C\x01a\x15)V[\x97PP`@a\x18T\x8B\x82\x8C\x01a\x15)V[\x96PP``a\x18e\x8B\x82\x8C\x01a\x15)V[\x95PP`\x80a\x18v\x8B\x82\x8C\x01a\x15)V[\x94PP`\xA0a\x18\x87\x8B\x82\x8C\x01a\x15)V[\x93PP`\xC0a\x18\x98\x8B\x82\x8C\x01a\x15)V[\x92PP`\xE0a\x18\xA9\x8B\x82\x8C\x01a\x15)V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_`@\x82\x01\x90Pa\x18\xCC_\x83\x01\x85a\x15\xA1V[a\x18\xD9` \x83\x01\x84a\x15\xA1V[\x93\x92PPPV[_\x80_\x80_\x80_\x80_a\x01 \x8A\x8C\x03\x12\x15a\x18\xFEWa\x18\xFDa\x15\x06V[[_a\x19\x0B\x8C\x82\x8D\x01a\x15)V[\x99PP` a\x19\x1C\x8C\x82\x8D\x01a\x15)V[\x98PP`@a\x19-\x8C\x82\x8D\x01a\x15)V[\x97PP``a\x19>\x8C\x82\x8D\x01a\x15)V[\x96PP`\x80a\x19O\x8C\x82\x8D\x01a\x15)V[\x95PP`\xA0a\x19`\x8C\x82\x8D\x01a\x15)V[\x94PP`\xC0a\x19q\x8C\x82\x8D\x01a\x15)V[\x93PP`\xE0a\x19\x82\x8C\x82\x8D\x01a\x15)V[\x92PPa\x01\0a\x19\x94\x8C\x82\x8D\x01a\x15)V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x19\xDB\x82a\x15\nV[\x91Pa\x19\xE6\x83a\x15\nV[\x92P\x82\x82\x02a\x19\xF4\x81a\x15\nV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1A\x0BWa\x1A\na\x19\xA4V[[P\x92\x91PPV[_a\x1A\x1C\x82a\x15\nV[\x91Pa\x1A'\x83a\x15\nV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1A?Wa\x1A>a\x19\xA4V[[\x92\x91PPV[_a\x1AO\x82a\x16UV[\x91Pa\x1AZ\x83a\x16UV[\x92P\x82\x82\x03\x90P\x81\x81\x12_\x84\x12\x16\x82\x82\x13_\x85\x12\x15\x16\x17\x15a\x1A\x7FWa\x1A~a\x19\xA4V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x1A\xBC\x82a\x15\nV[\x91Pa\x1A\xC7\x83a\x15\nV[\x92P\x82a\x1A\xD7Wa\x1A\xD6a\x1A\x85V[[\x82\x82\x04\x90P\x92\x91PPV[_a\x1A\xEC\x82a\x16UV[\x91Pa\x1A\xF7\x83a\x16UV[\x92P\x82\x82\x02a\x1B\x05\x81a\x16UV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14_\x84\x12\x16\x15a\x1B<Wa\x1B;a\x19\xA4V[[\x82\x82\x05\x84\x14\x83\x15\x17a\x1BQWa\x1BPa\x19\xA4V[[P\x92\x91PPV[_a\x1Bb\x82a\x16UV[\x91Pa\x1Bm\x83a\x16UV[\x92P\x82\x82\x01\x90P\x82\x81\x12\x15_\x83\x12\x16\x83\x82\x12_\x84\x12\x15\x16\x17\x15a\x1B\x93Wa\x1B\x92a\x19\xA4V[[\x92\x91PPV[_a\x1B\xA3\x82a\x15\nV[\x91Pa\x1B\xAE\x83a\x15\nV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1B\xC6Wa\x1B\xC5a\x19\xA4V[[\x92\x91PPV[_a\x1B\xD6\x82a\x15\nV[\x91Pa\x1B\xE1\x83a\x15\nV[\x92P\x82a\x1B\xF1Wa\x1B\xF0a\x1A\x85V[[\x82\x82\x06\x90P\x92\x91PPV[_a\x1C\x06\x82a\x15\nV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1C8Wa\x1C7a\x19\xA4V[[`\x01\x82\x01\x90P\x91\x90PV[_a\x1CM\x82a\x16UV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1C\x7FWa\x1C~a\x19\xA4V[[\x81_\x03\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x1C\xACa\x1C\xA7a\x1C\xA2\x84a\x15\nV[a\x1C\x89V[a\x15\nV[\x90P\x91\x90PV[a\x1C\xBC\x81a\x1C\x92V[\x82RPPV[_` \x82\x01\x90Pa\x1C\xD5_\x83\x01\x84a\x1C\xB3V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCould not find rounded protocol _\x82\x01R\x7Fliquidation bound\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1DE`1\x83a\x1C\xDBV[\x91Pa\x1DP\x82a\x1C\xEBV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1Dr\x81a\x1D9V[\x90P\x91\x90PV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x1D\xCEW\x80\x86\x04\x81\x11\x15a\x1D\xAAWa\x1D\xA9a\x19\xA4V[[`\x01\x85\x16\x15a\x1D\xB9W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x1D\xC7\x85a\x1DyV[\x94Pa\x1D\x8EV[\x94P\x94\x92PPPV[_\x82a\x1D\xE6W`\x01\x90Pa\x1E\xA1V[\x81a\x1D\xF3W_\x90Pa\x1E\xA1V[\x81`\x01\x81\x14a\x1E\tW`\x02\x81\x14a\x1E\x13Wa\x1EBV[`\x01\x91PPa\x1E\xA1V[`\xFF\x84\x11\x15a\x1E%Wa\x1E$a\x19\xA4V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x1E<Wa\x1E;a\x19\xA4V[[Pa\x1E\xA1V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1EwW\x82\x82\n\x90P\x83\x81\x11\x15a\x1ErWa\x1Eqa\x19\xA4V[[a\x1E\xA1V[a\x1E\x84\x84\x84\x84`\x01a\x1D\x85V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x1E\x9BWa\x1E\x9Aa\x19\xA4V[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x1E\xB2\x82a\x15\nV[\x91Pa\x1E\xBD\x83a\x15\nV[\x92Pa\x1E\xEA\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x1D\xD7V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_a\x1F5\x82a\x1F\x1FV[\x91Pa\x1F@\x83a\x1F\x1FV[\x92P\x82a\x1FPWa\x1FOa\x1A\x85V[[\x82\x82\x06\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xBC\x04`\xC9txM\xF2~\xBB[+>\xF2\x9D\x81\x06\x9E96\x15\x0C\x18\xC1:L1_g\xAC\x93\xE5dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATIONHELPERS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LiquidationHelpers<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LiquidationHelpers<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LiquidationHelpers<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LiquidationHelpers<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LiquidationHelpers<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LiquidationHelpers))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LiquidationHelpers<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATIONHELPERS_ABI.clone(),
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
                LIQUIDATIONHELPERS_ABI.clone(),
                LIQUIDATIONHELPERS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getHealthRatio` (0x0d3963e3) function
        pub fn get_health_ratio(
            &self,
            collateral_amount: ::ethers::core::types::U256,
            exchange_rate: ::ethers::core::types::U256,
            liquidation_threshold: ::ethers::core::types::U256,
            total_debt: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [13, 57, 99, 227],
                    (collateral_amount, exchange_rate, liquidation_threshold, total_debt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationTypeBounds` (0x2935c801) function
        pub fn get_liquidation_type_bounds(
            &self,
            locs: LiquidationTypeArgs,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([41, 53, 200, 1], (locs,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRepayAmount` (0xab8dfa80) function
        pub fn get_repay_amount(
            &self,
            collateral_amount: ::ethers::core::types::U256,
            normalized_debt: ::ethers::core::types::U256,
            rate: ::ethers::core::types::U256,
            exchange_rate: ::ethers::core::types::U256,
            max_discount: ::ethers::core::types::U256,
            base_discount: ::ethers::core::types::U256,
            target_health: ::ethers::core::types::U256,
            liquidation_threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [171, 141, 250, 128],
                    (
                        collateral_amount,
                        normalized_debt,
                        rate,
                        exchange_rate,
                        max_discount,
                        base_discount,
                        target_health,
                        liquidation_threshold,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRepayAndCollateralRewardAmount` (0xfc8b1dde) function
        pub fn get_repay_and_collateral_reward_amount(
            &self,
            collateral_amount: ::ethers::core::types::U256,
            normalized_debt: ::ethers::core::types::U256,
            rate: ::ethers::core::types::U256,
            exchange_rate: ::ethers::core::types::U256,
            max_discount: ::ethers::core::types::U256,
            base_discount: ::ethers::core::types::U256,
            target_health: ::ethers::core::types::U256,
            liquidation_threshold: ::ethers::core::types::U256,
            dust: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [252, 139, 29, 222],
                    (
                        collateral_amount,
                        normalized_debt,
                        rate,
                        exchange_rate,
                        max_discount,
                        base_discount,
                        target_health,
                        liquidation_threshold,
                        dust,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LiquidationHelpers<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Custom Error type `NotScalingDown` with signature `NotScalingDown(uint256,uint256)` and selector `0xa5d16921`
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
    #[etherror(name = "NotScalingDown", abi = "NotScalingDown(uint256,uint256)")]
    pub struct NotScalingDown {
        pub from: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::U256,
    }
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
    ///Custom Error type `PRBMath_UD60x18_Sqrt_Overflow` with signature `PRBMath_UD60x18_Sqrt_Overflow(uint256)` and selector `0xedc236ad`
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
        name = "PRBMath_UD60x18_Sqrt_Overflow",
        abi = "PRBMath_UD60x18_Sqrt_Overflow(uint256)"
    )]
    pub struct PRBMath_UD60x18_Sqrt_Overflow {
        pub x: ::ethers::core::types::U256,
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
    pub enum LiquidationHelpersErrors {
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        NotScalingDown(NotScalingDown),
        NotScalingUp(NotScalingUp),
        PRBMath_UD60x18_Sqrt_Overflow(PRBMath_UD60x18_Sqrt_Overflow),
        SafeCastOverflowedUintToInt(SafeCastOverflowedUintToInt),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidationHelpersErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <MathOverflowedMulDiv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MathOverflowedMulDiv(decoded));
            }
            if let Ok(decoded) = <NotScalingDown as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotScalingDown(decoded));
            }
            if let Ok(decoded) = <NotScalingUp as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotScalingUp(decoded));
            }
            if let Ok(decoded) = <PRBMath_UD60x18_Sqrt_Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMath_UD60x18_Sqrt_Overflow(decoded));
            }
            if let Ok(decoded) = <SafeCastOverflowedUintToInt as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeCastOverflowedUintToInt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidationHelpersErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::MathOverflowedMulDiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotScalingDown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotScalingUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMath_UD60x18_Sqrt_Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastOverflowedUintToInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LiquidationHelpersErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <MathOverflowedMulDiv as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotScalingDown as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotScalingUp as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PRBMath_UD60x18_Sqrt_Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeCastOverflowedUintToInt as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LiquidationHelpersErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MathOverflowedMulDiv(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotScalingDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotScalingUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::PRBMath_UD60x18_Sqrt_Overflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeCastOverflowedUintToInt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidationHelpersErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<MathOverflowedMulDiv> for LiquidationHelpersErrors {
        fn from(value: MathOverflowedMulDiv) -> Self {
            Self::MathOverflowedMulDiv(value)
        }
    }
    impl ::core::convert::From<NotScalingDown> for LiquidationHelpersErrors {
        fn from(value: NotScalingDown) -> Self {
            Self::NotScalingDown(value)
        }
    }
    impl ::core::convert::From<NotScalingUp> for LiquidationHelpersErrors {
        fn from(value: NotScalingUp) -> Self {
            Self::NotScalingUp(value)
        }
    }
    impl ::core::convert::From<PRBMath_UD60x18_Sqrt_Overflow>
    for LiquidationHelpersErrors {
        fn from(value: PRBMath_UD60x18_Sqrt_Overflow) -> Self {
            Self::PRBMath_UD60x18_Sqrt_Overflow(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflowedUintToInt>
    for LiquidationHelpersErrors {
        fn from(value: SafeCastOverflowedUintToInt) -> Self {
            Self::SafeCastOverflowedUintToInt(value)
        }
    }
    ///Container type for all input parameters for the `getHealthRatio` function with signature `getHealthRatio(uint256,uint256,uint256,uint256)` and selector `0x0d3963e3`
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
        name = "getHealthRatio",
        abi = "getHealthRatio(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetHealthRatioCall {
        pub collateral_amount: ::ethers::core::types::U256,
        pub exchange_rate: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub total_debt: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLiquidationTypeBounds` function with signature `getLiquidationTypeBounds((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,int256,uint256))` and selector `0x2935c801`
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
        name = "getLiquidationTypeBounds",
        abi = "getLiquidationTypeBounds((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,int256,uint256))"
    )]
    pub struct GetLiquidationTypeBoundsCall {
        pub locs: LiquidationTypeArgs,
    }
    ///Container type for all input parameters for the `getRepayAmount` function with signature `getRepayAmount(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xab8dfa80`
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
        name = "getRepayAmount",
        abi = "getRepayAmount(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct GetRepayAmountCall {
        pub collateral_amount: ::ethers::core::types::U256,
        pub normalized_debt: ::ethers::core::types::U256,
        pub rate: ::ethers::core::types::U256,
        pub exchange_rate: ::ethers::core::types::U256,
        pub max_discount: ::ethers::core::types::U256,
        pub base_discount: ::ethers::core::types::U256,
        pub target_health: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRepayAndCollateralRewardAmount` function with signature `getRepayAndCollateralRewardAmount(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xfc8b1dde`
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
        name = "getRepayAndCollateralRewardAmount",
        abi = "getRepayAndCollateralRewardAmount(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct GetRepayAndCollateralRewardAmountCall {
        pub collateral_amount: ::ethers::core::types::U256,
        pub normalized_debt: ::ethers::core::types::U256,
        pub rate: ::ethers::core::types::U256,
        pub exchange_rate: ::ethers::core::types::U256,
        pub max_discount: ::ethers::core::types::U256,
        pub base_discount: ::ethers::core::types::U256,
        pub target_health: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub dust: ::ethers::core::types::U256,
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
    pub enum LiquidationHelpersCalls {
        GetHealthRatio(GetHealthRatioCall),
        GetLiquidationTypeBounds(GetLiquidationTypeBoundsCall),
        GetRepayAmount(GetRepayAmountCall),
        GetRepayAndCollateralRewardAmount(GetRepayAndCollateralRewardAmountCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidationHelpersCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetHealthRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetHealthRatio(decoded));
            }
            if let Ok(decoded) = <GetLiquidationTypeBoundsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidationTypeBounds(decoded));
            }
            if let Ok(decoded) = <GetRepayAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRepayAmount(decoded));
            }
            if let Ok(decoded) = <GetRepayAndCollateralRewardAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRepayAndCollateralRewardAmount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidationHelpersCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetHealthRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidationTypeBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRepayAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRepayAndCollateralRewardAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidationHelpersCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetHealthRatio(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidationTypeBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRepayAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRepayAndCollateralRewardAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetHealthRatioCall> for LiquidationHelpersCalls {
        fn from(value: GetHealthRatioCall) -> Self {
            Self::GetHealthRatio(value)
        }
    }
    impl ::core::convert::From<GetLiquidationTypeBoundsCall>
    for LiquidationHelpersCalls {
        fn from(value: GetLiquidationTypeBoundsCall) -> Self {
            Self::GetLiquidationTypeBounds(value)
        }
    }
    impl ::core::convert::From<GetRepayAmountCall> for LiquidationHelpersCalls {
        fn from(value: GetRepayAmountCall) -> Self {
            Self::GetRepayAmount(value)
        }
    }
    impl ::core::convert::From<GetRepayAndCollateralRewardAmountCall>
    for LiquidationHelpersCalls {
        fn from(value: GetRepayAndCollateralRewardAmountCall) -> Self {
            Self::GetRepayAndCollateralRewardAmount(value)
        }
    }
    ///Container type for all return fields from the `getHealthRatio` function with signature `getHealthRatio(uint256,uint256,uint256,uint256)` and selector `0x0d3963e3`
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
    pub struct GetHealthRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLiquidationTypeBounds` function with signature `getLiquidationTypeBounds((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,int256,uint256))` and selector `0x2935c801`
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
    pub struct GetLiquidationTypeBoundsReturn {
        pub partial_liquidation_bound: ::ethers::core::types::U256,
        pub full_liquidation_bound: ::ethers::core::types::U256,
        pub protocol_liquidation_bound: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getRepayAmount` function with signature `getRepayAmount(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xab8dfa80`
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
    pub struct GetRepayAmountReturn {
        pub repay_wad: ::ethers::core::types::U256,
        pub repay_rad: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getRepayAndCollateralRewardAmount` function with signature `getRepayAndCollateralRewardAmount(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xfc8b1dde`
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
    pub struct GetRepayAndCollateralRewardAmountReturn {
        pub repay_wad: ::ethers::core::types::U256,
        pub collateral_reward_amount: ::ethers::core::types::U256,
    }
    ///`LiquidationTypeArgs(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,int256,uint256)`
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
    pub struct LiquidationTypeArgs {
        pub collateral_amount: ::ethers::core::types::U256,
        pub normalized_debt: ::ethers::core::types::U256,
        pub rate: ::ethers::core::types::U256,
        pub max_discount: ::ethers::core::types::U256,
        pub base_discount: ::ethers::core::types::U256,
        pub target_health: ::ethers::core::types::U256,
        pub dust: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub negative_b: ::ethers::core::types::U256,
        pub b_squared: ::ethers::core::types::U256,
        pub four_ac: ::ethers::core::types::I256,
        pub total_debt: ::ethers::core::types::U256,
    }
}
