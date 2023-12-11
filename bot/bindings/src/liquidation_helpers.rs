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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa 7\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\r9c\xE3\x14a\0QW\x80c)5\xC8\x01\x14a\0\x81W\x80c\xAB\x8D\xFA\x80\x14a\0\xB3W\x80c\xFC\x8B\x1D\xDE\x14a\0\xE4W[`\0\x80\xFD[a\0k`\x04\x806\x03\x81\x01\x90a\0f\x91\x90a\x15\x9CV[a\x01\x15V[`@Qa\0x\x91\x90a\x16\x12V[`@Q\x80\x91\x03\x90\xF3[a\0\x9B`\x04\x806\x03\x81\x01\x90a\0\x96\x91\x90a\x18\x16V[a\x01VV[`@Qa\0\xAA\x93\x92\x91\x90a\x18DV[`@Q\x80\x91\x03\x90\xF3[a\0\xCD`\x04\x806\x03\x81\x01\x90a\0\xC8\x91\x90a\x18{V[a\x07:V[`@Qa\0\xDB\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xF3[a\0\xFE`\x04\x806\x03\x81\x01\x90a\0\xF9\x91\x90a\x19ZV[a\x08\xE8V[`@Qa\x01\x0C\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xF3[`\0\x80a\x016\x84\x86\x88a\x01(\x91\x90a\x1ASV[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x01K\x83\x82a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP\x94\x93PPPPV[`\0\x80`\0\x80\x84`@\x01Q\x85` \x01Qa\x01p\x91\x90a\x1ASV[\x90P`\0\x81\x03a\x01\xC9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93P\x93P\x93PPa\x073V[a\x02\x02`-a\x01\xF4\x87`\xE0\x01Q\x88`\0\x01Qa\x01\xE5\x91\x90a\x1ASV[\x84a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x02S`\x1Ba\x02$\x87`\xA0\x01Q\x88`\xC0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02;\x88`\x80\x01Q\x85a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02E\x91\x90a\x1A\x95V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01\0\x01\x81\x81RPPa\x02z\x85a\x01\0\x01Q\x86a\x01\0\x01Qa\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01 \x01\x81\x81RPPa\x02\x8E\x85\x82a\x0B5V[\x85a\x01@\x01\x81\x81RPP`\0`\x02a\x02\xD2a\x02\xCDa\x02\xC8\x89a\x01@\x01Qa\x02\xB9\x8Ba\x01 \x01Qa\x0C`V[a\x02\xC3\x91\x90a\x1A\xC9V[a\x0C\xCFV[a\x0C\xD9V[a\r\x87V[\x87a\x01\0\x01Qa\x02\xE2\x91\x90a\x1A\x95V[a\x02\xEC\x91\x90a\x1B;V[\x90Pa\x03\x1B\x86`\xE0\x01Qa\x03\r\x88`\0\x01Q\x84a\r\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x03G`\x1Ba\x039\x88`\x80\x01Q\x85a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01\0\x01\x81\x81RPPa\x03n\x86a\x01\0\x01Q\x87a\x01\0\x01Qa\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01 \x01\x81\x81RPPa\x03\xBCa\x03\xB7`\x1Ba\x03\xA9\x89`\xE0\x01Qa\x03\x9B\x87\x88a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[`\x04a\x03\xC8\x91\x90a\x1BlV[\x86a\x01@\x01\x81\x81RPP`\x02a\x04\na\x04\x05a\x04\0\x89a\x01@\x01Qa\x03\xF1\x8Ba\x01 \x01Qa\x0C`V[a\x03\xFB\x91\x90a\x1B\xE4V[a\x0C\xCFV[a\x0C\xD9V[a\r\x87V[\x87a\x01\0\x01Qa\x04\x1A\x91\x90a\x1A\x95V[a\x04$\x91\x90a\x1B;V[\x90Pa\x04S\x86`\xE0\x01Qa\x04E\x88`\0\x01Q\x84a\r\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P`\0a\x04\x90`-a\x04\x82\x87\x8A`\xE0\x01Q\x8B`\0\x01Qa\x04t\x91\x90a\x1ASV[a\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x04\xA7\x84\x83a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x04\xC3WP\x85\x87\x14[\x15a\x04\xF3W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x05\xABV[a\x04\xFD\x88\x87a\r\xF9V[\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x14a\x05\xAAWa\x05=\x86\x89`\xE0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\0\x01Qa\x05L\x91\x90a\x1ASV[\x91Pa\x05a\x84\x83a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x05}WP\x85\x87\x14[\x15a\x05\xA9W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[[\x84\x87\x03a\x05\xD6W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[`\0a\x06\x11`-a\x06\x03\x88\x8C`\xE0\x01Q\x8D`\0\x01Qa\x05\xF5\x91\x90a\x1ASV[a\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06&\x85\x82a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x06BWP\x85\x87\x14[\x15a\x06rW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x07\x02V[a\x06|\x89\x87a\x0F\x84V[\x95Pa\x06\x95\x86\x8A`\xE0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\0\x01Qa\x06\xA4\x91\x90a\x1ASV[\x90Pa\x06\xB9\x85\x82a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x06\xD5WP\x85\x87\x14[\x15a\x07\x01W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[\x85\x88\x03a\x07-W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97P[PPPPP[\x91\x93\x90\x92PV[`\0\x80a\x07Q`\x12\x88a\x10\xBC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x96P`\0a\x07s\x84\x89\x8Da\x07e\x91\x90a\x1ASV[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x89\x8Ba\x07\x83\x91\x90a\x1ASV[\x90P`\0\x81\x03a\x07\x9BW`\0\x80\x93P\x93PPPa\x08\xDBV[`\0a\x07\xB0\x82\x84a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x07\xD4W`\0\x80\x94P\x94PPPPa\x08\xDBV[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x07\xEE\x91\x90a\x1C(V[\x89a\x07\xF9\x91\x90a\x1A\x95V[\x90P\x89\x81\x11\x15a\x08\tW\x89a\x08\x0BV[\x80[\x90P`\0\x8C\x8Ea\x08\x1B\x91\x90a\x1ASV[\x90P`\0\x85a\x083\x8B\x84a\x10\xD2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x08=\x91\x90a\x1C(V[\x90P`\0a\x08k\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x08\\\x91\x90a\x1C(V[\x8Ba\x11\0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8Ba\x08v\x91\x90a\x1C(V[\x90Pa\x08\x8B\x81\x83a\x11\0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x97Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88a\x08\xA5\x91\x90a\x1B;V[\x98P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89a\x08\xC1\x91\x90a\x1C\\V[\x14a\x08\xD3W\x88a\x08\xD0\x90a\x1C\x8DV[\x98P[PPPPPPP[\x98P\x98\x96PPPPPPPV[`\0\x80`\0a\x08\xFD\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x07:V[\x80\x92P\x81\x94PPPa\t\x19`\x12\x8Aa\x10\xBC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x98P`\0a\t;\x86\x8B\x8Fa\t-\x91\x90a\x1ASV[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x8B\x8Da\tK\x91\x90a\x1ASV[\x90P`\0\x81\x03a\tdW`\0\x80\x94P\x94PPPPa\n[V[`\0a\ty\x82\x84a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\t\x9EW`\0\x80\x95P\x95PPPPPa\n[V[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\xB8\x91\x90a\x1C(V[\x8Ba\t\xC3\x91\x90a\x1A\x95V[\x90P\x8B\x81\x11\x15a\t\xD3W\x8Ba\t\xD5V[\x80[\x90P`\0a\n\x03\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\xF4\x91\x90a\x1C(V[\x8Fa\x10\xD2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x83\x86\x11\x15a\n\x1FW`\0\x80\x97P\x97PPPPPPPa\n[V[\x85\x89a\n+\x91\x90a\x1A\x95V[\x84\x10\x15a\nEW\x80\x84a\n>\x91\x90a\x1B;V[\x96Pa\nTV[\x80\x86a\nQ\x91\x90a\x1B;V[\x96P[PPPPPP[\x99P\x99\x97PPPPPPPPV[`\0a\n\x8C\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\n\xBEr,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x83\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\n\xD4\x83\x83`\x12a\x125V[\x90P\x92\x91PPV[`\0a\x0B\x06\x82r,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x0B-\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80a\x0Bia\x0Bd\x85`\xE0\x01Qa\x0BV\x86\x87a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[a\x0B\x9Ea\x0B\x99\x86`\xE0\x01Qa\x0B\x8B\x87\x89`\xC0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[a\x0B\xE9a\x0B\xE4\x87`\x80\x01Qa\x0B\xD6\x89`\xA0\x01Qa\x0B\xC8\x8A\x8C`\xC0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[a\x0B\xF3\x91\x90a\x1B\xE4V[a\x0B\xFD\x91\x90a\x1A\xC9V[\x90P`\0\x80\x82\x12\x90P`\0\x81a\x0C\x13W\x82a\x0C\x1EV[\x82a\x0C\x1D\x90a\x1C\xD5V[[\x90P`\0a\x0C<`\x04c;\x9A\xCA\0\x84a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x82a\x0CIW\x80a\x0CTV[\x80a\x0CS\x90a\x1C\xD5V[[\x94PPPPP\x92\x91PPV[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\xC7W\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xBE\x91\x90a\x16\x12V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0\x80a\x0C\xE5\x83a\r\x87V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81a\r\x1FWa\r\x1Ea\x1B\x0CV[[\x04\x81\x11\x15a\rdW\x82`@Q\x7F\xED\xC26\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r[\x91\x90a\x1DXV[`@Q\x80\x91\x03\x90\xFD[a\r\x7Fa\rzg\r\xE0\xB6\xB3\xA7d\0\0\x83\x02a\x12\xA9V[a\x14TV[\x91PP\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\r\xB0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\r\xDBk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\r\xF1\x83\x83`\x1Ba\x125V[\x90P\x92\x91PPV[`\0\x80`\0\x90P[`\x05\x81\x10\x15a\x0FYW`\0a\x0EO\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x85`\x01\x89a\x0E,\x91\x90a\x1C(V[a\x0E6\x91\x90a\x1A\x95V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07:V[\x91PP`\0a\x0E\x8B\x86`\0\x01Q\x87` \x01Q\x88`@\x01Q\x86\x89a\x0Er\x91\x90a\x1A\x95V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07:V[\x91PP\x85`@\x01Q\x86` \x01Qa\x0E\xA2\x91\x90a\x1ASV[\x82\x11\x15\x80\x15a\x0E\xD2WP\x81\x86`\xC0\x01Qa\x0E\xBC\x91\x90a\x1A\x95V[\x86`@\x01Q\x87` \x01Qa\x0E\xD0\x91\x90a\x1ASV[\x10[\x15a\x0FDW\x85`@\x01Q\x86` \x01Qa\x0E\xEB\x91\x90a\x1ASV[\x81\x11\x15\x80\x15a\x0F\x1CWP\x80\x86`\xC0\x01Qa\x0F\x05\x91\x90a\x1A\x95V[\x86`@\x01Q\x87` \x01Qa\x0F\x19\x91\x90a\x1ASV[\x10\x15[\x15a\x0FCW`\x01\x83\x86a\x0F/\x91\x90a\x1A\x95V[a\x0F9\x91\x90a\x1C(V[\x93PPPPa\x0F~V[[PP\x80\x80a\x0FQ\x90a\x1C\x8DV[\x91PPa\x0E\x01V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90P[\x92\x91PPV[`\0\x80`\0\x90P[`\n\x81\x10\x15a\x10zW`\0a\x0F\xCE\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x85\x88a\x0F\xB5\x91\x90a\x1A\x95V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07:V[\x91PP`\0a\x10\x16\x86`\0\x01Q\x87` \x01Q\x88`@\x01Q`\x01\x87\x8Aa\x0F\xF3\x91\x90a\x1A\x95V[a\x0F\xFD\x91\x90a\x1A\x95V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07:V[\x91PP\x85`@\x01Q\x86` \x01Qa\x10-\x91\x90a\x1ASV[\x82\x11\x15a\x10eW\x85`@\x01Q\x86` \x01Qa\x10H\x91\x90a\x1ASV[\x81\x11a\x10dW\x82\x85a\x10Z\x91\x90a\x1A\x95V[\x93PPPPa\x10\xB6V[[PP\x80\x80a\x10r\x90a\x1C\x8DV[\x91PPa\x0F\x8CV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xAD\x90a\x1D\xF6V[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[`\0a\x10\xCA\x83\x83`\x1Ba\x14^V[\x90P\x92\x91PPV[`\0a\x10\xF8\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x14\xD2\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x11&k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x14\xD2\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x11iW\x83\x82\x81a\x11_Wa\x11^a\x1B\x0CV[[\x04\x92PPPa\x12.V[\x80\x84\x11a\x11\xA2W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0\x81\x83\x11a\x12}W\x82\x82`@Q\x7F\xA5\xD1i!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12t\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x12\x89\x91\x90a\x1C(V[`\na\x12\x95\x91\x90a\x1FIV[\x84a\x12\xA0\x91\x90a\x1B;V[\x90P\x93\x92PPPV[`\0\x80\x82\x03a\x12\xBBW`\0\x90Pa\x14OV[`\0\x82\x90P`\x01\x91Pp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10a\x12\xEBW`\x80\x81\x90\x1C\x90P`@\x82\x90\x1B\x91P[h\x01\0\0\0\0\0\0\0\0\x81\x10a\x13\nW`@\x81\x90\x1C\x90P` \x82\x90\x1B\x91P[d\x01\0\0\0\0\x81\x10a\x13%W` \x81\x90\x1C\x90P`\x10\x82\x90\x1B\x91P[b\x01\0\0\x81\x10a\x13>W`\x10\x81\x90\x1C\x90P`\x08\x82\x90\x1B\x91P[a\x01\0\x81\x10a\x13VW`\x08\x81\x90\x1C\x90P`\x04\x82\x90\x1B\x91P[`\x10\x81\x10a\x13mW`\x04\x81\x90\x1C\x90P`\x02\x82\x90\x1B\x91P[`\x04\x81\x10a\x13}W`\x01\x82\x90\x1B\x91P[`\x01\x82\x84\x81a\x13\x8FWa\x13\x8Ea\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xA8Wa\x13\xA7a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xC1Wa\x13\xC0a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xDAWa\x13\xD9a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xF3Wa\x13\xF2a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x14\x0CWa\x14\x0Ba\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x14%Wa\x14$a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\0\x82\x84\x81a\x14>Wa\x14=a\x1B\x0CV[[\x04\x90P\x80\x83\x10a\x14LW\x80\x92P[PP[\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0\x81\x83\x10a\x14\xA6W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x9D\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x14\xB2\x91\x90a\x1C(V[`\na\x14\xBE\x91\x90a\x1FIV[\x84a\x14\xC9\x91\x90a\x1ASV[\x90P\x93\x92PPPV[`\0\x80a\x14\xE0\x86\x86\x86a\x11.V[\x90Pa\x14\xEB\x83a\x15)V[\x80\x15a\x15\x08WP`\0\x84\x80a\x15\x03Wa\x15\x02a\x1B\x0CV[[\x86\x88\t\x11[\x15a\x15\x1DW`\x01\x81a\x15\x1A\x91\x90a\x1A\x95V[\x90P[\x80\x91PP\x94\x93PPPPV[`\0`\x01`\x02\x83`\x03\x81\x11\x15a\x15BWa\x15Aa\x1F\x94V[[a\x15L\x91\x90a\x1F\xD0V[`\xFF\x16\x14\x90P\x91\x90PV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x15y\x81a\x15fV[\x81\x14a\x15\x84W`\0\x80\xFD[PV[`\0\x815\x90Pa\x15\x96\x81a\x15pV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\xB6Wa\x15\xB5a\x15aV[[`\0a\x15\xC4\x87\x82\x88\x01a\x15\x87V[\x94PP` a\x15\xD5\x87\x82\x88\x01a\x15\x87V[\x93PP`@a\x15\xE6\x87\x82\x88\x01a\x15\x87V[\x92PP``a\x15\xF7\x87\x82\x88\x01a\x15\x87V[\x91PP\x92\x95\x91\x94P\x92PV[a\x16\x0C\x81a\x15fV[\x82RPPV[`\0` \x82\x01\x90Pa\x16'`\0\x83\x01\x84a\x16\x03V[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x16{\x82a\x162V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x16\x9AWa\x16\x99a\x16CV[[\x80`@RPPPV[`\0a\x16\xADa\x15WV[\x90Pa\x16\xB9\x82\x82a\x16rV[\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x16\xD1\x81a\x16\xBEV[\x81\x14a\x16\xDCW`\0\x80\xFD[PV[`\0\x815\x90Pa\x16\xEE\x81a\x16\xC8V[\x92\x91PPV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x17\x0BWa\x17\na\x16-V[[a\x17\x16a\x01\x80a\x16\xA3V[\x90P`\0a\x17&\x84\x82\x85\x01a\x15\x87V[`\0\x83\x01RP` a\x17:\x84\x82\x85\x01a\x15\x87V[` \x83\x01RP`@a\x17N\x84\x82\x85\x01a\x15\x87V[`@\x83\x01RP``a\x17b\x84\x82\x85\x01a\x15\x87V[``\x83\x01RP`\x80a\x17v\x84\x82\x85\x01a\x15\x87V[`\x80\x83\x01RP`\xA0a\x17\x8A\x84\x82\x85\x01a\x15\x87V[`\xA0\x83\x01RP`\xC0a\x17\x9E\x84\x82\x85\x01a\x15\x87V[`\xC0\x83\x01RP`\xE0a\x17\xB2\x84\x82\x85\x01a\x15\x87V[`\xE0\x83\x01RPa\x01\0a\x17\xC7\x84\x82\x85\x01a\x15\x87V[a\x01\0\x83\x01RPa\x01 a\x17\xDD\x84\x82\x85\x01a\x15\x87V[a\x01 \x83\x01RPa\x01@a\x17\xF3\x84\x82\x85\x01a\x16\xDFV[a\x01@\x83\x01RPa\x01`a\x18\t\x84\x82\x85\x01a\x15\x87V[a\x01`\x83\x01RP\x92\x91PPV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x18-Wa\x18,a\x15aV[[`\0a\x18;\x84\x82\x85\x01a\x16\xF4V[\x91PP\x92\x91PPV[`\0``\x82\x01\x90Pa\x18Y`\0\x83\x01\x86a\x16\x03V[a\x18f` \x83\x01\x85a\x16\x03V[a\x18s`@\x83\x01\x84a\x16\x03V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x18\x9CWa\x18\x9Ba\x15aV[[`\0a\x18\xAA\x8B\x82\x8C\x01a\x15\x87V[\x98PP` a\x18\xBB\x8B\x82\x8C\x01a\x15\x87V[\x97PP`@a\x18\xCC\x8B\x82\x8C\x01a\x15\x87V[\x96PP``a\x18\xDD\x8B\x82\x8C\x01a\x15\x87V[\x95PP`\x80a\x18\xEE\x8B\x82\x8C\x01a\x15\x87V[\x94PP`\xA0a\x18\xFF\x8B\x82\x8C\x01a\x15\x87V[\x93PP`\xC0a\x19\x10\x8B\x82\x8C\x01a\x15\x87V[\x92PP`\xE0a\x19!\x8B\x82\x8C\x01a\x15\x87V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0`@\x82\x01\x90Pa\x19F`\0\x83\x01\x85a\x16\x03V[a\x19S` \x83\x01\x84a\x16\x03V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a\x19}Wa\x19|a\x15aV[[`\0a\x19\x8B\x8C\x82\x8D\x01a\x15\x87V[\x99PP` a\x19\x9C\x8C\x82\x8D\x01a\x15\x87V[\x98PP`@a\x19\xAD\x8C\x82\x8D\x01a\x15\x87V[\x97PP``a\x19\xBE\x8C\x82\x8D\x01a\x15\x87V[\x96PP`\x80a\x19\xCF\x8C\x82\x8D\x01a\x15\x87V[\x95PP`\xA0a\x19\xE0\x8C\x82\x8D\x01a\x15\x87V[\x94PP`\xC0a\x19\xF1\x8C\x82\x8D\x01a\x15\x87V[\x93PP`\xE0a\x1A\x02\x8C\x82\x8D\x01a\x15\x87V[\x92PPa\x01\0a\x1A\x14\x8C\x82\x8D\x01a\x15\x87V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x1A^\x82a\x15fV[\x91Pa\x1Ai\x83a\x15fV[\x92P\x82\x82\x02a\x1Aw\x81a\x15fV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1A\x8EWa\x1A\x8Da\x1A$V[[P\x92\x91PPV[`\0a\x1A\xA0\x82a\x15fV[\x91Pa\x1A\xAB\x83a\x15fV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1A\xC3Wa\x1A\xC2a\x1A$V[[\x92\x91PPV[`\0a\x1A\xD4\x82a\x16\xBEV[\x91Pa\x1A\xDF\x83a\x16\xBEV[\x92P\x82\x82\x03\x90P\x81\x81\x12`\0\x84\x12\x16\x82\x82\x13`\0\x85\x12\x15\x16\x17\x15a\x1B\x06Wa\x1B\x05a\x1A$V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a\x1BF\x82a\x15fV[\x91Pa\x1BQ\x83a\x15fV[\x92P\x82a\x1BaWa\x1B`a\x1B\x0CV[[\x82\x82\x04\x90P\x92\x91PPV[`\0a\x1Bw\x82a\x16\xBEV[\x91Pa\x1B\x82\x83a\x16\xBEV[\x92P\x82\x82\x02a\x1B\x90\x81a\x16\xBEV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14`\0\x84\x12\x16\x15a\x1B\xC8Wa\x1B\xC7a\x1A$V[[\x82\x82\x05\x84\x14\x83\x15\x17a\x1B\xDDWa\x1B\xDCa\x1A$V[[P\x92\x91PPV[`\0a\x1B\xEF\x82a\x16\xBEV[\x91Pa\x1B\xFA\x83a\x16\xBEV[\x92P\x82\x82\x01\x90P\x82\x81\x12\x15`\0\x83\x12\x16\x83\x82\x12`\0\x84\x12\x15\x16\x17\x15a\x1C\"Wa\x1C!a\x1A$V[[\x92\x91PPV[`\0a\x1C3\x82a\x15fV[\x91Pa\x1C>\x83a\x15fV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1CVWa\x1CUa\x1A$V[[\x92\x91PPV[`\0a\x1Cg\x82a\x15fV[\x91Pa\x1Cr\x83a\x15fV[\x92P\x82a\x1C\x82Wa\x1C\x81a\x1B\x0CV[[\x82\x82\x06\x90P\x92\x91PPV[`\0a\x1C\x98\x82a\x15fV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1C\xCAWa\x1C\xC9a\x1A$V[[`\x01\x82\x01\x90P\x91\x90PV[`\0a\x1C\xE0\x82a\x16\xBEV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1D\x12Wa\x1D\x11a\x1A$V[[\x81`\0\x03\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x1DBa\x1D=a\x1D8\x84a\x15fV[a\x1D\x1DV[a\x15fV[\x90P\x91\x90PV[a\x1DR\x81a\x1D'V[\x82RPPV[`\0` \x82\x01\x90Pa\x1Dm`\0\x83\x01\x84a\x1DIV[\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCould not find rounded protocol `\0\x82\x01R\x7Fliquidation bound\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1D\xE0`1\x83a\x1DsV[\x91Pa\x1D\xEB\x82a\x1D\x84V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1E\x0F\x81a\x1D\xD3V[\x90P\x91\x90PV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x1EmW\x80\x86\x04\x81\x11\x15a\x1EIWa\x1EHa\x1A$V[[`\x01\x85\x16\x15a\x1EXW\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x1Ef\x85a\x1E\x16V[\x94Pa\x1E-V[\x94P\x94\x92PPPV[`\0\x82a\x1E\x86W`\x01\x90Pa\x1FBV[\x81a\x1E\x94W`\0\x90Pa\x1FBV[\x81`\x01\x81\x14a\x1E\xAAW`\x02\x81\x14a\x1E\xB4Wa\x1E\xE3V[`\x01\x91PPa\x1FBV[`\xFF\x84\x11\x15a\x1E\xC6Wa\x1E\xC5a\x1A$V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x1E\xDDWa\x1E\xDCa\x1A$V[[Pa\x1FBV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1F\x18W\x82\x82\n\x90P\x83\x81\x11\x15a\x1F\x13Wa\x1F\x12a\x1A$V[[a\x1FBV[a\x1F%\x84\x84\x84`\x01a\x1E#V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x1F<Wa\x1F;a\x1A$V[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\x1FT\x82a\x15fV[\x91Pa\x1F_\x83a\x15fV[\x92Pa\x1F\x8C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x1EvV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1F\xDB\x82a\x1F\xC3V[\x91Pa\x1F\xE6\x83a\x1F\xC3V[\x92P\x82a\x1F\xF6Wa\x1F\xF5a\x1B\x0CV[[\x82\x82\x06\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 h/\xA5=\x97z\xAE\x84\x88\xF9\xEAb\xF9\xEAE^o0ws+[5\xD3\xD7\x96\x82@\xE2\x9DSMdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATIONHELPERS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\r9c\xE3\x14a\0QW\x80c)5\xC8\x01\x14a\0\x81W\x80c\xAB\x8D\xFA\x80\x14a\0\xB3W\x80c\xFC\x8B\x1D\xDE\x14a\0\xE4W[`\0\x80\xFD[a\0k`\x04\x806\x03\x81\x01\x90a\0f\x91\x90a\x15\x9CV[a\x01\x15V[`@Qa\0x\x91\x90a\x16\x12V[`@Q\x80\x91\x03\x90\xF3[a\0\x9B`\x04\x806\x03\x81\x01\x90a\0\x96\x91\x90a\x18\x16V[a\x01VV[`@Qa\0\xAA\x93\x92\x91\x90a\x18DV[`@Q\x80\x91\x03\x90\xF3[a\0\xCD`\x04\x806\x03\x81\x01\x90a\0\xC8\x91\x90a\x18{V[a\x07:V[`@Qa\0\xDB\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xF3[a\0\xFE`\x04\x806\x03\x81\x01\x90a\0\xF9\x91\x90a\x19ZV[a\x08\xE8V[`@Qa\x01\x0C\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xF3[`\0\x80a\x016\x84\x86\x88a\x01(\x91\x90a\x1ASV[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x01K\x83\x82a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP\x94\x93PPPPV[`\0\x80`\0\x80\x84`@\x01Q\x85` \x01Qa\x01p\x91\x90a\x1ASV[\x90P`\0\x81\x03a\x01\xC9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93P\x93P\x93PPa\x073V[a\x02\x02`-a\x01\xF4\x87`\xE0\x01Q\x88`\0\x01Qa\x01\xE5\x91\x90a\x1ASV[\x84a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x02S`\x1Ba\x02$\x87`\xA0\x01Q\x88`\xC0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02;\x88`\x80\x01Q\x85a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02E\x91\x90a\x1A\x95V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01\0\x01\x81\x81RPPa\x02z\x85a\x01\0\x01Q\x86a\x01\0\x01Qa\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85a\x01 \x01\x81\x81RPPa\x02\x8E\x85\x82a\x0B5V[\x85a\x01@\x01\x81\x81RPP`\0`\x02a\x02\xD2a\x02\xCDa\x02\xC8\x89a\x01@\x01Qa\x02\xB9\x8Ba\x01 \x01Qa\x0C`V[a\x02\xC3\x91\x90a\x1A\xC9V[a\x0C\xCFV[a\x0C\xD9V[a\r\x87V[\x87a\x01\0\x01Qa\x02\xE2\x91\x90a\x1A\x95V[a\x02\xEC\x91\x90a\x1B;V[\x90Pa\x03\x1B\x86`\xE0\x01Qa\x03\r\x88`\0\x01Q\x84a\r\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93Pa\x03G`\x1Ba\x039\x88`\x80\x01Q\x85a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01\0\x01\x81\x81RPPa\x03n\x86a\x01\0\x01Q\x87a\x01\0\x01Qa\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x01 \x01\x81\x81RPPa\x03\xBCa\x03\xB7`\x1Ba\x03\xA9\x89`\xE0\x01Qa\x03\x9B\x87\x88a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xC6\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[`\x04a\x03\xC8\x91\x90a\x1BlV[\x86a\x01@\x01\x81\x81RPP`\x02a\x04\na\x04\x05a\x04\0\x89a\x01@\x01Qa\x03\xF1\x8Ba\x01 \x01Qa\x0C`V[a\x03\xFB\x91\x90a\x1B\xE4V[a\x0C\xCFV[a\x0C\xD9V[a\r\x87V[\x87a\x01\0\x01Qa\x04\x1A\x91\x90a\x1A\x95V[a\x04$\x91\x90a\x1B;V[\x90Pa\x04S\x86`\xE0\x01Qa\x04E\x88`\0\x01Q\x84a\r\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92P`\0a\x04\x90`-a\x04\x82\x87\x8A`\xE0\x01Q\x8B`\0\x01Qa\x04t\x91\x90a\x1ASV[a\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x04\xA7\x84\x83a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x04\xC3WP\x85\x87\x14[\x15a\x04\xF3W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x05\xABV[a\x04\xFD\x88\x87a\r\xF9V[\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x14a\x05\xAAWa\x05=\x86\x89`\xE0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\0\x01Qa\x05L\x91\x90a\x1ASV[\x91Pa\x05a\x84\x83a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10\x15\x80a\x05}WP\x85\x87\x14[\x15a\x05\xA9W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[[\x84\x87\x03a\x05\xD6W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[`\0a\x06\x11`-a\x06\x03\x88\x8C`\xE0\x01Q\x8D`\0\x01Qa\x05\xF5\x91\x90a\x1ASV[a\x0B\x0E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06&\x85\x82a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x06BWP\x85\x87\x14[\x15a\x06rW\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96Pa\x07\x02V[a\x06|\x89\x87a\x0F\x84V[\x95Pa\x06\x95\x86\x8A`\xE0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\0\x01Qa\x06\xA4\x91\x90a\x1ASV[\x90Pa\x06\xB9\x85\x82a\n\x94\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x10\x15\x80a\x06\xD5WP\x85\x87\x14[\x15a\x07\x01W\x86\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96P[[\x85\x88\x03a\x07-W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97P[PPPPP[\x91\x93\x90\x92PV[`\0\x80a\x07Q`\x12\x88a\x10\xBC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x96P`\0a\x07s\x84\x89\x8Da\x07e\x91\x90a\x1ASV[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x89\x8Ba\x07\x83\x91\x90a\x1ASV[\x90P`\0\x81\x03a\x07\x9BW`\0\x80\x93P\x93PPPa\x08\xDBV[`\0a\x07\xB0\x82\x84a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\x07\xD4W`\0\x80\x94P\x94PPPPa\x08\xDBV[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x07\xEE\x91\x90a\x1C(V[\x89a\x07\xF9\x91\x90a\x1A\x95V[\x90P\x89\x81\x11\x15a\x08\tW\x89a\x08\x0BV[\x80[\x90P`\0\x8C\x8Ea\x08\x1B\x91\x90a\x1ASV[\x90P`\0\x85a\x083\x8B\x84a\x10\xD2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x08=\x91\x90a\x1C(V[\x90P`\0a\x08k\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x08\\\x91\x90a\x1C(V[\x8Ba\x11\0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x8Ba\x08v\x91\x90a\x1C(V[\x90Pa\x08\x8B\x81\x83a\x11\0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x97Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x88a\x08\xA5\x91\x90a\x1B;V[\x98P`\0k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x89a\x08\xC1\x91\x90a\x1C\\V[\x14a\x08\xD3W\x88a\x08\xD0\x90a\x1C\x8DV[\x98P[PPPPPPP[\x98P\x98\x96PPPPPPPV[`\0\x80`\0a\x08\xFD\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x07:V[\x80\x92P\x81\x94PPPa\t\x19`\x12\x8Aa\x10\xBC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x98P`\0a\t;\x86\x8B\x8Fa\t-\x91\x90a\x1ASV[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x8B\x8Da\tK\x91\x90a\x1ASV[\x90P`\0\x81\x03a\tdW`\0\x80\x94P\x94PPPPa\n[V[`\0a\ty\x82\x84a\r\xB8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x10a\t\x9EW`\0\x80\x95P\x95PPPPPa\n[V[`\0\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\xB8\x91\x90a\x1C(V[\x8Ba\t\xC3\x91\x90a\x1A\x95V[\x90P\x8B\x81\x11\x15a\t\xD3W\x8Ba\t\xD5V[\x80[\x90P`\0a\n\x03\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\t\xF4\x91\x90a\x1C(V[\x8Fa\x10\xD2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x83\x86\x11\x15a\n\x1FW`\0\x80\x97P\x97PPPPPPPa\n[V[\x85\x89a\n+\x91\x90a\x1A\x95V[\x84\x10\x15a\nEW\x80\x84a\n>\x91\x90a\x1B;V[\x96Pa\nTV[\x80\x86a\nQ\x91\x90a\x1B;V[\x96P[PPPPPP[\x99P\x99\x97PPPPPPPPV[`\0a\n\x8C\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\n\xBEr,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x83\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\n\xD4\x83\x83`\x12a\x125V[\x90P\x92\x91PPV[`\0a\x0B\x06\x82r,\xD7o\xE0\x86\xB9<\xE2\xF7h\xA0\x0B\"\xA0\0\0\0\0\0\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x0B-\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80a\x0Bia\x0Bd\x85`\xE0\x01Qa\x0BV\x86\x87a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[a\x0B\x9Ea\x0B\x99\x86`\xE0\x01Qa\x0B\x8B\x87\x89`\xC0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[a\x0B\xE9a\x0B\xE4\x87`\x80\x01Qa\x0B\xD6\x89`\xA0\x01Qa\x0B\xC8\x8A\x8C`\xC0\x01Qa\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\ni\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0C`V[a\x0B\xF3\x91\x90a\x1B\xE4V[a\x0B\xFD\x91\x90a\x1A\xC9V[\x90P`\0\x80\x82\x12\x90P`\0\x81a\x0C\x13W\x82a\x0C\x1EV[\x82a\x0C\x1D\x90a\x1C\xD5V[[\x90P`\0a\x0C<`\x04c;\x9A\xCA\0\x84a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x82a\x0CIW\x80a\x0CTV[\x80a\x0CS\x90a\x1C\xD5V[[\x94PPPPP\x92\x91PPV[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\xC7W\x81`@Q\x7F$w^\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xBE\x91\x90a\x16\x12V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0\x80a\x0C\xE5\x83a\r\x87V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81a\r\x1FWa\r\x1Ea\x1B\x0CV[[\x04\x81\x11\x15a\rdW\x82`@Q\x7F\xED\xC26\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r[\x91\x90a\x1DXV[`@Q\x80\x91\x03\x90\xFD[a\r\x7Fa\rzg\r\xE0\xB6\xB3\xA7d\0\0\x83\x02a\x12\xA9V[a\x14TV[\x91PP\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\r\xB0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\r\xDBk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x11.\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\r\xF1\x83\x83`\x1Ba\x125V[\x90P\x92\x91PPV[`\0\x80`\0\x90P[`\x05\x81\x10\x15a\x0FYW`\0a\x0EO\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x85`\x01\x89a\x0E,\x91\x90a\x1C(V[a\x0E6\x91\x90a\x1A\x95V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07:V[\x91PP`\0a\x0E\x8B\x86`\0\x01Q\x87` \x01Q\x88`@\x01Q\x86\x89a\x0Er\x91\x90a\x1A\x95V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07:V[\x91PP\x85`@\x01Q\x86` \x01Qa\x0E\xA2\x91\x90a\x1ASV[\x82\x11\x15\x80\x15a\x0E\xD2WP\x81\x86`\xC0\x01Qa\x0E\xBC\x91\x90a\x1A\x95V[\x86`@\x01Q\x87` \x01Qa\x0E\xD0\x91\x90a\x1ASV[\x10[\x15a\x0FDW\x85`@\x01Q\x86` \x01Qa\x0E\xEB\x91\x90a\x1ASV[\x81\x11\x15\x80\x15a\x0F\x1CWP\x80\x86`\xC0\x01Qa\x0F\x05\x91\x90a\x1A\x95V[\x86`@\x01Q\x87` \x01Qa\x0F\x19\x91\x90a\x1ASV[\x10\x15[\x15a\x0FCW`\x01\x83\x86a\x0F/\x91\x90a\x1A\x95V[a\x0F9\x91\x90a\x1C(V[\x93PPPPa\x0F~V[[PP\x80\x80a\x0FQ\x90a\x1C\x8DV[\x91PPa\x0E\x01V[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90P[\x92\x91PPV[`\0\x80`\0\x90P[`\n\x81\x10\x15a\x10zW`\0a\x0F\xCE\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x85\x88a\x0F\xB5\x91\x90a\x1A\x95V[\x89``\x01Q\x8A`\x80\x01Q\x8B`\xA0\x01Q\x8C`\xE0\x01Qa\x07:V[\x91PP`\0a\x10\x16\x86`\0\x01Q\x87` \x01Q\x88`@\x01Q`\x01\x87\x8Aa\x0F\xF3\x91\x90a\x1A\x95V[a\x0F\xFD\x91\x90a\x1A\x95V[\x8A``\x01Q\x8B`\x80\x01Q\x8C`\xA0\x01Q\x8D`\xE0\x01Qa\x07:V[\x91PP\x85`@\x01Q\x86` \x01Qa\x10-\x91\x90a\x1ASV[\x82\x11\x15a\x10eW\x85`@\x01Q\x86` \x01Qa\x10H\x91\x90a\x1ASV[\x81\x11a\x10dW\x82\x85a\x10Z\x91\x90a\x1A\x95V[\x93PPPPa\x10\xB6V[[PP\x80\x80a\x10r\x90a\x1C\x8DV[\x91PPa\x0F\x8CV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xAD\x90a\x1D\xF6V[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[`\0a\x10\xCA\x83\x83`\x1Ba\x14^V[\x90P\x92\x91PPV[`\0a\x10\xF8\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x01\x86a\x14\xD2\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x11&k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83`\x01\x86a\x14\xD2\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x11iW\x83\x82\x81a\x11_Wa\x11^a\x1B\x0CV[[\x04\x92PPPa\x12.V[\x80\x84\x11a\x11\xA2W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0\x81\x83\x11a\x12}W\x82\x82`@Q\x7F\xA5\xD1i!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12t\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x12\x89\x91\x90a\x1C(V[`\na\x12\x95\x91\x90a\x1FIV[\x84a\x12\xA0\x91\x90a\x1B;V[\x90P\x93\x92PPPV[`\0\x80\x82\x03a\x12\xBBW`\0\x90Pa\x14OV[`\0\x82\x90P`\x01\x91Pp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10a\x12\xEBW`\x80\x81\x90\x1C\x90P`@\x82\x90\x1B\x91P[h\x01\0\0\0\0\0\0\0\0\x81\x10a\x13\nW`@\x81\x90\x1C\x90P` \x82\x90\x1B\x91P[d\x01\0\0\0\0\x81\x10a\x13%W` \x81\x90\x1C\x90P`\x10\x82\x90\x1B\x91P[b\x01\0\0\x81\x10a\x13>W`\x10\x81\x90\x1C\x90P`\x08\x82\x90\x1B\x91P[a\x01\0\x81\x10a\x13VW`\x08\x81\x90\x1C\x90P`\x04\x82\x90\x1B\x91P[`\x10\x81\x10a\x13mW`\x04\x81\x90\x1C\x90P`\x02\x82\x90\x1B\x91P[`\x04\x81\x10a\x13}W`\x01\x82\x90\x1B\x91P[`\x01\x82\x84\x81a\x13\x8FWa\x13\x8Ea\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xA8Wa\x13\xA7a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xC1Wa\x13\xC0a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xDAWa\x13\xD9a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x13\xF3Wa\x13\xF2a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x14\x0CWa\x14\x0Ba\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\x01\x82\x84\x81a\x14%Wa\x14$a\x1B\x0CV[[\x04\x83\x01\x90\x1C\x91P`\0\x82\x84\x81a\x14>Wa\x14=a\x1B\x0CV[[\x04\x90P\x80\x83\x10a\x14LW\x80\x92P[PP[\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0\x81\x83\x10a\x14\xA6W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x9D\x92\x91\x90a\x191V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x14\xB2\x91\x90a\x1C(V[`\na\x14\xBE\x91\x90a\x1FIV[\x84a\x14\xC9\x91\x90a\x1ASV[\x90P\x93\x92PPPV[`\0\x80a\x14\xE0\x86\x86\x86a\x11.V[\x90Pa\x14\xEB\x83a\x15)V[\x80\x15a\x15\x08WP`\0\x84\x80a\x15\x03Wa\x15\x02a\x1B\x0CV[[\x86\x88\t\x11[\x15a\x15\x1DW`\x01\x81a\x15\x1A\x91\x90a\x1A\x95V[\x90P[\x80\x91PP\x94\x93PPPPV[`\0`\x01`\x02\x83`\x03\x81\x11\x15a\x15BWa\x15Aa\x1F\x94V[[a\x15L\x91\x90a\x1F\xD0V[`\xFF\x16\x14\x90P\x91\x90PV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x15y\x81a\x15fV[\x81\x14a\x15\x84W`\0\x80\xFD[PV[`\0\x815\x90Pa\x15\x96\x81a\x15pV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\xB6Wa\x15\xB5a\x15aV[[`\0a\x15\xC4\x87\x82\x88\x01a\x15\x87V[\x94PP` a\x15\xD5\x87\x82\x88\x01a\x15\x87V[\x93PP`@a\x15\xE6\x87\x82\x88\x01a\x15\x87V[\x92PP``a\x15\xF7\x87\x82\x88\x01a\x15\x87V[\x91PP\x92\x95\x91\x94P\x92PV[a\x16\x0C\x81a\x15fV[\x82RPPV[`\0` \x82\x01\x90Pa\x16'`\0\x83\x01\x84a\x16\x03V[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x16{\x82a\x162V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x16\x9AWa\x16\x99a\x16CV[[\x80`@RPPPV[`\0a\x16\xADa\x15WV[\x90Pa\x16\xB9\x82\x82a\x16rV[\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x16\xD1\x81a\x16\xBEV[\x81\x14a\x16\xDCW`\0\x80\xFD[PV[`\0\x815\x90Pa\x16\xEE\x81a\x16\xC8V[\x92\x91PPV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x17\x0BWa\x17\na\x16-V[[a\x17\x16a\x01\x80a\x16\xA3V[\x90P`\0a\x17&\x84\x82\x85\x01a\x15\x87V[`\0\x83\x01RP` a\x17:\x84\x82\x85\x01a\x15\x87V[` \x83\x01RP`@a\x17N\x84\x82\x85\x01a\x15\x87V[`@\x83\x01RP``a\x17b\x84\x82\x85\x01a\x15\x87V[``\x83\x01RP`\x80a\x17v\x84\x82\x85\x01a\x15\x87V[`\x80\x83\x01RP`\xA0a\x17\x8A\x84\x82\x85\x01a\x15\x87V[`\xA0\x83\x01RP`\xC0a\x17\x9E\x84\x82\x85\x01a\x15\x87V[`\xC0\x83\x01RP`\xE0a\x17\xB2\x84\x82\x85\x01a\x15\x87V[`\xE0\x83\x01RPa\x01\0a\x17\xC7\x84\x82\x85\x01a\x15\x87V[a\x01\0\x83\x01RPa\x01 a\x17\xDD\x84\x82\x85\x01a\x15\x87V[a\x01 \x83\x01RPa\x01@a\x17\xF3\x84\x82\x85\x01a\x16\xDFV[a\x01@\x83\x01RPa\x01`a\x18\t\x84\x82\x85\x01a\x15\x87V[a\x01`\x83\x01RP\x92\x91PPV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x18-Wa\x18,a\x15aV[[`\0a\x18;\x84\x82\x85\x01a\x16\xF4V[\x91PP\x92\x91PPV[`\0``\x82\x01\x90Pa\x18Y`\0\x83\x01\x86a\x16\x03V[a\x18f` \x83\x01\x85a\x16\x03V[a\x18s`@\x83\x01\x84a\x16\x03V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x18\x9CWa\x18\x9Ba\x15aV[[`\0a\x18\xAA\x8B\x82\x8C\x01a\x15\x87V[\x98PP` a\x18\xBB\x8B\x82\x8C\x01a\x15\x87V[\x97PP`@a\x18\xCC\x8B\x82\x8C\x01a\x15\x87V[\x96PP``a\x18\xDD\x8B\x82\x8C\x01a\x15\x87V[\x95PP`\x80a\x18\xEE\x8B\x82\x8C\x01a\x15\x87V[\x94PP`\xA0a\x18\xFF\x8B\x82\x8C\x01a\x15\x87V[\x93PP`\xC0a\x19\x10\x8B\x82\x8C\x01a\x15\x87V[\x92PP`\xE0a\x19!\x8B\x82\x8C\x01a\x15\x87V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0`@\x82\x01\x90Pa\x19F`\0\x83\x01\x85a\x16\x03V[a\x19S` \x83\x01\x84a\x16\x03V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a\x19}Wa\x19|a\x15aV[[`\0a\x19\x8B\x8C\x82\x8D\x01a\x15\x87V[\x99PP` a\x19\x9C\x8C\x82\x8D\x01a\x15\x87V[\x98PP`@a\x19\xAD\x8C\x82\x8D\x01a\x15\x87V[\x97PP``a\x19\xBE\x8C\x82\x8D\x01a\x15\x87V[\x96PP`\x80a\x19\xCF\x8C\x82\x8D\x01a\x15\x87V[\x95PP`\xA0a\x19\xE0\x8C\x82\x8D\x01a\x15\x87V[\x94PP`\xC0a\x19\xF1\x8C\x82\x8D\x01a\x15\x87V[\x93PP`\xE0a\x1A\x02\x8C\x82\x8D\x01a\x15\x87V[\x92PPa\x01\0a\x1A\x14\x8C\x82\x8D\x01a\x15\x87V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x1A^\x82a\x15fV[\x91Pa\x1Ai\x83a\x15fV[\x92P\x82\x82\x02a\x1Aw\x81a\x15fV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1A\x8EWa\x1A\x8Da\x1A$V[[P\x92\x91PPV[`\0a\x1A\xA0\x82a\x15fV[\x91Pa\x1A\xAB\x83a\x15fV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1A\xC3Wa\x1A\xC2a\x1A$V[[\x92\x91PPV[`\0a\x1A\xD4\x82a\x16\xBEV[\x91Pa\x1A\xDF\x83a\x16\xBEV[\x92P\x82\x82\x03\x90P\x81\x81\x12`\0\x84\x12\x16\x82\x82\x13`\0\x85\x12\x15\x16\x17\x15a\x1B\x06Wa\x1B\x05a\x1A$V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a\x1BF\x82a\x15fV[\x91Pa\x1BQ\x83a\x15fV[\x92P\x82a\x1BaWa\x1B`a\x1B\x0CV[[\x82\x82\x04\x90P\x92\x91PPV[`\0a\x1Bw\x82a\x16\xBEV[\x91Pa\x1B\x82\x83a\x16\xBEV[\x92P\x82\x82\x02a\x1B\x90\x81a\x16\xBEV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14`\0\x84\x12\x16\x15a\x1B\xC8Wa\x1B\xC7a\x1A$V[[\x82\x82\x05\x84\x14\x83\x15\x17a\x1B\xDDWa\x1B\xDCa\x1A$V[[P\x92\x91PPV[`\0a\x1B\xEF\x82a\x16\xBEV[\x91Pa\x1B\xFA\x83a\x16\xBEV[\x92P\x82\x82\x01\x90P\x82\x81\x12\x15`\0\x83\x12\x16\x83\x82\x12`\0\x84\x12\x15\x16\x17\x15a\x1C\"Wa\x1C!a\x1A$V[[\x92\x91PPV[`\0a\x1C3\x82a\x15fV[\x91Pa\x1C>\x83a\x15fV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1CVWa\x1CUa\x1A$V[[\x92\x91PPV[`\0a\x1Cg\x82a\x15fV[\x91Pa\x1Cr\x83a\x15fV[\x92P\x82a\x1C\x82Wa\x1C\x81a\x1B\x0CV[[\x82\x82\x06\x90P\x92\x91PPV[`\0a\x1C\x98\x82a\x15fV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1C\xCAWa\x1C\xC9a\x1A$V[[`\x01\x82\x01\x90P\x91\x90PV[`\0a\x1C\xE0\x82a\x16\xBEV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1D\x12Wa\x1D\x11a\x1A$V[[\x81`\0\x03\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x1DBa\x1D=a\x1D8\x84a\x15fV[a\x1D\x1DV[a\x15fV[\x90P\x91\x90PV[a\x1DR\x81a\x1D'V[\x82RPPV[`\0` \x82\x01\x90Pa\x1Dm`\0\x83\x01\x84a\x1DIV[\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCould not find rounded protocol `\0\x82\x01R\x7Fliquidation bound\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1D\xE0`1\x83a\x1DsV[\x91Pa\x1D\xEB\x82a\x1D\x84V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1E\x0F\x81a\x1D\xD3V[\x90P\x91\x90PV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x1EmW\x80\x86\x04\x81\x11\x15a\x1EIWa\x1EHa\x1A$V[[`\x01\x85\x16\x15a\x1EXW\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x1Ef\x85a\x1E\x16V[\x94Pa\x1E-V[\x94P\x94\x92PPPV[`\0\x82a\x1E\x86W`\x01\x90Pa\x1FBV[\x81a\x1E\x94W`\0\x90Pa\x1FBV[\x81`\x01\x81\x14a\x1E\xAAW`\x02\x81\x14a\x1E\xB4Wa\x1E\xE3V[`\x01\x91PPa\x1FBV[`\xFF\x84\x11\x15a\x1E\xC6Wa\x1E\xC5a\x1A$V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x1E\xDDWa\x1E\xDCa\x1A$V[[Pa\x1FBV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1F\x18W\x82\x82\n\x90P\x83\x81\x11\x15a\x1F\x13Wa\x1F\x12a\x1A$V[[a\x1FBV[a\x1F%\x84\x84\x84`\x01a\x1E#V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x1F<Wa\x1F;a\x1A$V[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\x1FT\x82a\x15fV[\x91Pa\x1F_\x83a\x15fV[\x92Pa\x1F\x8C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x1EvV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1F\xDB\x82a\x1F\xC3V[\x91Pa\x1F\xE6\x83a\x1F\xC3V[\x92P\x82a\x1F\xF6Wa\x1F\xF5a\x1B\x0CV[[\x82\x82\x06\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 h/\xA5=\x97z\xAE\x84\x88\xF9\xEAb\xF9\xEAE^o0ws+[5\xD3\xD7\x96\x82@\xE2\x9DSMdsolcC\0\x08\x15\x003";
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
