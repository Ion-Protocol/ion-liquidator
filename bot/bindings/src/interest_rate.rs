pub use interest_rate::*;
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
pub mod interest_rate {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("ilkDataList"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Tuple(
                                    ::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    ],
                                ),
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("struct IlkData[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_yieldOracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IYieldOracle"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("COLLATERAL_COUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("COLLATERAL_COUNT"),
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
                    ::std::borrow::ToOwned::to_owned("YIELD_ORACLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("YIELD_ORACLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IYieldOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateInterestRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateInterestRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalIlkDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalEthSupply"),
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
                    ::std::borrow::ToOwned::to_owned("unpackCollateralConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "unpackCollateralConfig",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                                    name: ::std::borrow::ToOwned::to_owned("ilkData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IlkData"),
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
                    ::std::borrow::ToOwned::to_owned("CollateralIndexOutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CollateralIndexOutOfBounds",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DistributionFactorsDoNotSumToOne"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DistributionFactorsDoNotSumToOne",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sum"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidIlkDataListLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidIlkDataListLength",
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
                    ::std::borrow::ToOwned::to_owned("InvalidMinimumKinkRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidMinimumKinkRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumKinkRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumBaseRate"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidOptimalUtilizationRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidOptimalUtilizationRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "optimalUtilizationRate",
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
                    ::std::borrow::ToOwned::to_owned("InvalidReserveFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidReserveFactor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveFactor"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidYieldOracleAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidYieldOracleAddress",
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
                    ::std::borrow::ToOwned::to_owned("TotalDebtsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TotalDebtsLength"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("COLLATERAL_COUNT"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "totalIlkDebtsLength",
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static INTERESTRATE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x03\xC0`@R4\x80\x15b\0\0\x11W_\x80\xFD[P`@Qb\0!)8\x03\x80b\0!)\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\t\xCFV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\x9DW`@Q\x7Fu>\xD5\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x82Q\x11\x15b\0\0\xE8W\x81Q`@Q\x7F\xEF%C\x8A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0\xDF\x91\x90b\0\nMV[`@Q\x80\x91\x03\x90\xFD[\x81Qa\x03\x80\x81\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x80[a\x03\x80Q\x81\x10\x15b\0\x03\x88W\x83\x81\x81Q\x81\x10b\0\x01LWb\0\x01Kb\0\nhV[[` \x02` \x01\x01Q`\xC0\x01Qa\xFF\xFF\x16\x82b\0\x01i\x91\x90b\0\n\xC2V[\x91P\x83\x81\x81Q\x81\x10b\0\x01\x81Wb\0\x01\x80b\0\nhV[[` \x02` \x01\x01Q`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x82\x81Q\x81\x10b\0\x01\xB1Wb\0\x01\xB0b\0\nhV[[` \x02` \x01\x01Q` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15b\0\x02UW\x83\x81\x81Q\x81\x10b\0\x01\xE8Wb\0\x01\xE7b\0\nhV[[` \x02` \x01\x01Q` \x01Q\x84\x82\x81Q\x81\x10b\0\x02\nWb\0\x02\tb\0\nhV[[` \x02` \x01\x01Q`\x80\x01Q`@Q\x7F~\xD9DB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x02L\x92\x91\x90b\0\x0B=V[`@Q\x80\x91\x03\x90\xFD[_\x84\x82\x81Q\x81\x10b\0\x02lWb\0\x02kb\0\nhV[[` \x02` \x01\x01Q`\xA0\x01Qa\xFF\xFF\x16\x03b\0\x02\xE2W\x83\x81\x81Q\x81\x10b\0\x02\x98Wb\0\x02\x97b\0\nhV[[` \x02` \x01\x01Q`\xA0\x01Q`@Q\x7F\xF2\xAEk=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x02\xD9\x91\x90b\0\x0B\xA0V[`@Q\x80\x91\x03\x90\xFD[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x84\x82\x81Q\x81\x10b\0\x03\x05Wb\0\x03\x04b\0\nhV[[` \x02` \x01\x01Q`@\x01Qa\xFF\xFF\x16\x11\x15b\0\x03|W\x83\x81\x81Q\x81\x10b\0\x032Wb\0\x031b\0\nhV[[` \x02` \x01\x01Q`@\x01Q`@Q\x7F\x19\xA6\x93\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x03s\x91\x90b\0\x0B\xA0V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01\x90Pb\0\x01*V[Pa'\x10\x81\x14b\0\x03\xD2W\x80`@Q\x7F\xB9w\x8C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x03\xC9\x91\x90b\0\nMV[`@Q\x80\x91\x03\x90\xFD[b\0\x03\xE4\x83_b\0\x05F` \x1B` \x1CV[\x80`\xC0\x90\x81RP\x81`\xA0\x90\x81RP\x82`\x80\x90\x81RPPPPb\0\x04\x0F\x83`\x01b\0\x05F` \x1B` \x1CV[\x80a\x01 \x90\x81RP\x81a\x01\0\x90\x81RP\x82`\xE0\x90\x81RPPPPb\0\x04<\x83`\x02b\0\x05F` \x1B` \x1CV[\x80a\x01\x80\x90\x81RP\x81a\x01`\x90\x81RP\x82a\x01@\x90\x81RPPPPb\0\x04j\x83`\x03b\0\x05F` \x1B` \x1CV[\x80a\x01\xE0\x90\x81RP\x81a\x01\xC0\x90\x81RP\x82a\x01\xA0\x90\x81RPPPPb\0\x04\x98\x83`\x04b\0\x05F` \x1B` \x1CV[\x80a\x02@\x90\x81RP\x81a\x02 \x90\x81RP\x82a\x02\0\x90\x81RPPPPb\0\x04\xC6\x83`\x05b\0\x05F` \x1B` \x1CV[\x80a\x02\xA0\x90\x81RP\x81a\x02\x80\x90\x81RP\x82a\x02`\x90\x81RPPPPb\0\x04\xF4\x83`\x06b\0\x05F` \x1B` \x1CV[\x80a\x03\0\x90\x81RP\x81a\x02\xE0\x90\x81RP\x82a\x02\xC0\x90\x81RPPPPb\0\x05\"\x83`\x07b\0\x05F` \x1B` \x1CV[\x80a\x03`\x90\x81RP\x81a\x03@\x90\x81RP\x82a\x03 \x90\x81RPPPPPPPb\0\x0B\xBBV[_\x80_a\x03\x80Q\x84\x10b\0\x05cW_\x80_\x92P\x92P\x92Pb\0\x06[V[_\x85\x85\x81Q\x81\x10b\0\x05zWb\0\x05yb\0\nhV[[` \x02` \x01\x01Q\x90P```\xFF\x16\x81` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B_`\xFF\x16\x82_\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x93P`\xE0`\xFF\x16\x81`\xC0\x01Qa\xFF\xFF\x16\x90\x1B`\xD0`\xFF\x16\x82`\xA0\x01Qa\xFF\xFF\x16\x90\x1B`p`\xFF\x16\x83`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`\x10`\xFF\x16\x84``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B_`\xFF\x16\x85`@\x01Qa\xFF\xFF\x16\x90\x1B\x17\x17\x17\x17\x92P```\xFF\x16\x81a\x01\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B_`\xFF\x16\x82`\xE0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x91PP[\x92P\x92P\x92V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[b\0\x06\xBF\x82b\0\x06wV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x06\xE1Wb\0\x06\xE0b\0\x06\x87V[[\x80`@RPPPV[_b\0\x06\xF5b\0\x06bV[\x90Pb\0\x07\x03\x82\x82b\0\x06\xB4V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x07%Wb\0\x07$b\0\x06\x87V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_\x80\xFD[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x07`\x81b\0\x07>V[\x81\x14b\0\x07kW_\x80\xFD[PV[_\x81Q\x90Pb\0\x07~\x81b\0\x07UV[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x07\x9C\x81b\0\x07\x84V[\x81\x14b\0\x07\xA7W_\x80\xFD[PV[_\x81Q\x90Pb\0\x07\xBA\x81b\0\x07\x91V[\x92\x91PPV[_a\x01 \x82\x84\x03\x12\x15b\0\x07\xD9Wb\0\x07\xD8b\0\x07:V[[b\0\x07\xE6a\x01 b\0\x06\xEAV[\x90P_b\0\x07\xF7\x84\x82\x85\x01b\0\x07nV[_\x83\x01RP` b\0\x08\x0C\x84\x82\x85\x01b\0\x07nV[` \x83\x01RP`@b\0\x08\"\x84\x82\x85\x01b\0\x07\xAAV[`@\x83\x01RP``b\0\x088\x84\x82\x85\x01b\0\x07nV[``\x83\x01RP`\x80b\0\x08N\x84\x82\x85\x01b\0\x07nV[`\x80\x83\x01RP`\xA0b\0\x08d\x84\x82\x85\x01b\0\x07\xAAV[`\xA0\x83\x01RP`\xC0b\0\x08z\x84\x82\x85\x01b\0\x07\xAAV[`\xC0\x83\x01RP`\xE0b\0\x08\x90\x84\x82\x85\x01b\0\x07nV[`\xE0\x83\x01RPa\x01\0b\0\x08\xA7\x84\x82\x85\x01b\0\x07nV[a\x01\0\x83\x01RP\x92\x91PPV[_b\0\x08\xCAb\0\x08\xC4\x84b\0\x07\x08V[b\0\x06\xEAV[\x90P\x80\x83\x82R` \x82\x01\x90Pa\x01 \x84\x02\x83\x01\x85\x81\x11\x15b\0\x08\xF1Wb\0\x08\xF0b\0\x076V[[\x83[\x81\x81\x10\x15b\0\t\x1FW\x80b\0\t\t\x88\x82b\0\x07\xC0V[\x84R` \x84\x01\x93PPa\x01 \x81\x01\x90Pb\0\x08\xF3V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12b\0\t@Wb\0\t?b\0\x06sV[[\x81Qb\0\tR\x84\x82` \x86\x01b\0\x08\xB4V[\x91PP\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\t\x86\x82b\0\t[V[\x90P\x91\x90PV[_b\0\t\x99\x82b\0\tzV[\x90P\x91\x90PV[b\0\t\xAB\x81b\0\t\x8DV[\x81\x14b\0\t\xB6W_\x80\xFD[PV[_\x81Q\x90Pb\0\t\xC9\x81b\0\t\xA0V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15b\0\t\xE8Wb\0\t\xE7b\0\x06kV[[_\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\n\x08Wb\0\n\x07b\0\x06oV[[b\0\n\x16\x85\x82\x86\x01b\0\t)V[\x92PP` b\0\n)\x85\x82\x86\x01b\0\t\xB9V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[b\0\nG\x81b\0\n3V[\x82RPPV[_` \x82\x01\x90Pb\0\nb_\x83\x01\x84b\0\n<V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_b\0\n\xCE\x82b\0\n3V[\x91Pb\0\n\xDB\x83b\0\n3V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15b\0\n\xF6Wb\0\n\xF5b\0\n\x95V[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_b\0\x0B%b\0\x0B\x1Fb\0\x0B\x19\x84b\0\x07>V[b\0\n\xFCV[b\0\n3V[\x90P\x91\x90PV[b\0\x0B7\x81b\0\x0B\x05V[\x82RPPV[_`@\x82\x01\x90Pb\0\x0BR_\x83\x01\x85b\0\x0B,V[b\0\x0Ba` \x83\x01\x84b\0\x0B,V[\x93\x92PPPV[_b\0\x0B\x88b\0\x0B\x82b\0\x0B|\x84b\0\x07\x84V[b\0\n\xFCV[b\0\n3V[\x90P\x91\x90PV[b\0\x0B\x9A\x81b\0\x0BhV[\x82RPPV[_` \x82\x01\x90Pb\0\x0B\xB5_\x83\x01\x84b\0\x0B\x8FV[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x01\xE0Qa\x02\0Qa\x02 Qa\x02@Qa\x02`Qa\x02\x80Qa\x02\xA0Qa\x02\xC0Qa\x02\xE0Qa\x03\0Qa\x03 Qa\x03@Qa\x03`Qa\x03\x80Qa\x03\xA0Qa\x14Qb\0\x0C\xD8_9_\x81\x81`\xED\x01Ra\x01\x8D\x01R_\x81\x81a\x01)\x01Ra\x05\x88\x01R_a\tv\x01R_a\tS\x01R_a\t0\x01R_a\t\0\x01R_a\x08\xDD\x01R_a\x08\xBA\x01R_a\x08\x8A\x01R_a\x08g\x01R_a\x08D\x01R_a\x08\x14\x01R_a\x07\xF1\x01R_a\x07\xCE\x01R_a\x07\x9E\x01R_a\x07{\x01R_a\x07X\x01R_a\x07(\x01R_a\x07\x05\x01R_a\x06\xE2\x01R_a\x06\xB2\x01R_a\x06\x8F\x01R_a\x06l\x01R_a\x06<\x01R_a\x06\x19\x01R_a\x05\xF6\x01Ra\x14Q_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\t\xEAQ\x9F\x14a\0NW\x80c\x16\x1F\xA6V\x14a\0lW\x80cH\xD4\xB4\x87\x14a\0\x9CW\x80c\xFEK\xABC\x14a\0\xBAW[_\x80\xFD[a\0Va\0\xEBV[`@Qa\0c\x91\x90a\x0E\xA3V[`@Q\x80\x91\x03\x90\xF3[a\0\x86`\x04\x806\x03\x81\x01\x90a\0\x81\x91\x90a\x0E\xF3V[a\x01\x0FV[`@Qa\0\x93\x91\x90a\x10\x15V[`@Q\x80\x91\x03\x90\xF3[a\0\xA4a\x01'V[`@Qa\0\xB1\x91\x90a\x10>V[`@Q\x80\x91\x03\x90\xF3[a\0\xD4`\x04\x806\x03\x81\x01\x90a\0\xCF\x91\x90a\x10WV[a\x01KV[`@Qa\0\xE2\x92\x91\x90a\x10\xA7V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x17a\r\x86V[a\x01 \x82a\x05|V[\x90P\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_\x80_a\x01W\x86a\x05|V[\x90P_a\x01v`\x04\x83`\xA0\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_c\x01\xE13\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x027`\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x81\xCE\x1C#\x8C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xE4\x91\x90a\x10>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xFFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02#\x91\x90a\x11\x07V[c\xFF\xFF\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02A\x91\x90a\x11\x8CV[\x90P_\x83`\xC0\x01Qa\xFF\xFF\x16\x90P_\x81\x03a\x02\x95W\x83` \x01Qa\x02w`\x04\x86`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x95P\x95PPPPPa\x05tV[_\x80\x88\x14a\x02\xD3Wa\x02\xC3a\x02\xB4`\x04\x84a\x0B\x85\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x0B\x9A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x02\xCE\x91\x90a\x11\x8CV[a\x02\xD5V[_[\x90P_\x80\x86``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x03\x03\x90P\x84\x81\x11\x15a\x03\x0FW_\x90P[a\x03\"\x86\x82a\x0B\xC0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP_a\x03Z\x86\x88`\x80\x01Q\x89` \x01Qa\x03>\x91\x90a\x11\xBCV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xC0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x85\x83\x10\x15a\x040W_\x87``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\x8B\x85\x85a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\x95\x91\x90a\x11\xFBV[\x90P_\x88`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xBE\x86\x85a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\xC8\x91\x90a\x11\xFBV[\x90P\x80\x82\x10\x15a\x04\x01W\x80a\x03\xEF`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x05tV[\x81a\x04\x1E`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x05tV[_\x86\x84a\x04=\x91\x90a\x12.V[\x90P_\x88``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04f\x89\x86a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04p\x91\x90a\x11\xFBV[\x90P_\x89`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\x99\x8A\x86a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xA3\x91\x90a\x11\xFBV[\x90P_\x82a\x04\xCC\x85\x8D`\xE0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xD6\x91\x90a\x11\xFBV[\x90P_\x82a\x05\0\x86\x8Ea\x01\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x05\n\x91\x90a\x11\xFBV[\x90P\x80\x82\x10\x15a\x05FW\x80a\x051`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPPa\x05tV[\x81a\x05c`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPP[\x93P\x93\x91PPV[a\x05\x84a\r\x86V[`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xB1\x91\x90a\x12.V[\x82\x11\x15a\x05\xEAW`@Q\x7F\xCA\x89\xFCI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80_\x80\x85\x03a\x06bW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9FV[`\x01\x85\x03a\x06\xD8W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9EV[`\x02\x85\x03a\x07NW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9DV[`\x03\x85\x03a\x07\xC4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9CV[`\x04\x85\x03a\x08:W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9BV[`\x05\x85\x03a\x08\xB0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9AV[`\x06\x85\x03a\t&W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x99V[`\x07\x85\x03a\t\x98W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P[[[[[[[[_\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x1C\x90P_```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x86\x16\x90\x1C\x90P_\x80`\xFF\x16a\xFF\xFF\x86\x16\x90\x1C\x90P_`\x10`\xFF\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x87\x16\x90\x1C\x90P_`p`\xFF\x16y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x90\x1C\x90P_`\xD0`\xFF\x16{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x16\x90\x1C\x90P_`\xE0`\xFF\x16}\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x16\x90\x1C\x90P_\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x90\x1C\x90P_```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x16\x90\x1C\x90P`@Q\x80a\x01 \x01`@R\x80\x8Ak\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88a\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85a\xFF\xFF\x16\x81R` \x01\x84a\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x9CPPPPPPPPPPPPP\x91\x90PV[_a\x0B}\x83\x83`\x1Ba\x0C\x14V[\x90P\x92\x91PPV[_a\x0B\x92\x83\x83`\x12a\x0C\x14V[\x90P\x92\x91PPV[_a\x0B\xB8\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0C\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x0B\xE2k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x0C\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x0C\x0C\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x0C\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x81\x83\x10a\x0C[W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0CR\x92\x91\x90a\x10\xA7V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x0Cg\x91\x90a\x12.V[`\na\x0Cs\x91\x90a\x13\x90V[\x84a\x0C~\x91\x90a\x13\xDAV[\x90P\x93\x92PPPV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\x0C\xBFW\x83\x82\x81a\x0C\xB5Wa\x0C\xB4a\x112V[[\x04\x92PPPa\r\x7FV[\x80\x84\x11a\x0C\xF8W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`@Q\x80a\x01 \x01`@R\x80_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_a\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_a\xFF\xFF\x16\x81R` \x01_a\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x0Eka\x0Efa\x0Ea\x84a\x0E)V[a\x0EHV[a\x0E)V[\x90P\x91\x90PV[_a\x0E|\x82a\x0EQV[\x90P\x91\x90PV[_a\x0E\x8D\x82a\x0ErV[\x90P\x91\x90PV[a\x0E\x9D\x81a\x0E\x83V[\x82RPPV[_` \x82\x01\x90Pa\x0E\xB6_\x83\x01\x84a\x0E\x94V[\x92\x91PPV[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x0E\xD2\x81a\x0E\xC0V[\x81\x14a\x0E\xDCW_\x80\xFD[PV[_\x815\x90Pa\x0E\xED\x81a\x0E\xC9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\x08Wa\x0F\x07a\x0E\xBCV[[_a\x0F\x15\x84\x82\x85\x01a\x0E\xDFV[\x91PP\x92\x91PPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F>\x81a\x0F\x1EV[\x82RPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0FZ\x81a\x0FDV[\x82RPPV[a\x01 \x82\x01_\x82\x01Qa\x0Fu_\x85\x01\x82a\x0F5V[P` \x82\x01Qa\x0F\x88` \x85\x01\x82a\x0F5V[P`@\x82\x01Qa\x0F\x9B`@\x85\x01\x82a\x0FQV[P``\x82\x01Qa\x0F\xAE``\x85\x01\x82a\x0F5V[P`\x80\x82\x01Qa\x0F\xC1`\x80\x85\x01\x82a\x0F5V[P`\xA0\x82\x01Qa\x0F\xD4`\xA0\x85\x01\x82a\x0FQV[P`\xC0\x82\x01Qa\x0F\xE7`\xC0\x85\x01\x82a\x0FQV[P`\xE0\x82\x01Qa\x0F\xFA`\xE0\x85\x01\x82a\x0F5V[Pa\x01\0\x82\x01Qa\x10\x0Fa\x01\0\x85\x01\x82a\x0F5V[PPPPV[_a\x01 \x82\x01\x90Pa\x10)_\x83\x01\x84a\x0F`V[\x92\x91PPV[a\x108\x81a\x0E\xC0V[\x82RPPV[_` \x82\x01\x90Pa\x10Q_\x83\x01\x84a\x10/V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x10nWa\x10ma\x0E\xBCV[[_a\x10{\x86\x82\x87\x01a\x0E\xDFV[\x93PP` a\x10\x8C\x86\x82\x87\x01a\x0E\xDFV[\x92PP`@a\x10\x9D\x86\x82\x87\x01a\x0E\xDFV[\x91PP\x92P\x92P\x92V[_`@\x82\x01\x90Pa\x10\xBA_\x83\x01\x85a\x10/V[a\x10\xC7` \x83\x01\x84a\x10/V[\x93\x92PPPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10\xE6\x81a\x10\xCEV[\x81\x14a\x10\xF0W_\x80\xFD[PV[_\x81Q\x90Pa\x11\x01\x81a\x10\xDDV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11\x1CWa\x11\x1Ba\x0E\xBCV[[_a\x11)\x84\x82\x85\x01a\x10\xF3V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x11\x96\x82a\x0E\xC0V[\x91Pa\x11\xA1\x83a\x0E\xC0V[\x92P\x82a\x11\xB1Wa\x11\xB0a\x112V[[\x82\x82\x04\x90P\x92\x91PPV[_a\x11\xC6\x82a\x0F\x1EV[\x91Pa\x11\xD1\x83a\x0F\x1EV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xF5Wa\x11\xF4a\x11_V[[\x92\x91PPV[_a\x12\x05\x82a\x0E\xC0V[\x91Pa\x12\x10\x83a\x0E\xC0V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x12(Wa\x12'a\x11_V[[\x92\x91PPV[_a\x128\x82a\x0E\xC0V[\x91Pa\x12C\x83a\x0E\xC0V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x12[Wa\x12Za\x11_V[[\x92\x91PPV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x12\xB6W\x80\x86\x04\x81\x11\x15a\x12\x92Wa\x12\x91a\x11_V[[`\x01\x85\x16\x15a\x12\xA1W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x12\xAF\x85a\x12aV[\x94Pa\x12vV[\x94P\x94\x92PPPV[_\x82a\x12\xCEW`\x01\x90Pa\x13\x89V[\x81a\x12\xDBW_\x90Pa\x13\x89V[\x81`\x01\x81\x14a\x12\xF1W`\x02\x81\x14a\x12\xFBWa\x13*V[`\x01\x91PPa\x13\x89V[`\xFF\x84\x11\x15a\x13\rWa\x13\x0Ca\x11_V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x13$Wa\x13#a\x11_V[[Pa\x13\x89V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x13_W\x82\x82\n\x90P\x83\x81\x11\x15a\x13ZWa\x13Ya\x11_V[[a\x13\x89V[a\x13l\x84\x84\x84`\x01a\x12mV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x13\x83Wa\x13\x82a\x11_V[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x13\x9A\x82a\x0E\xC0V[\x91Pa\x13\xA5\x83a\x0E\xC0V[\x92Pa\x13\xD2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x12\xBFV[\x90P\x92\x91PPV[_a\x13\xE4\x82a\x0E\xC0V[\x91Pa\x13\xEF\x83a\x0E\xC0V[\x92P\x82\x82\x02a\x13\xFD\x81a\x0E\xC0V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x14\x14Wa\x14\x13a\x11_V[[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x14B\xE4_5\xB9[\xE1\xA7\x1D\xF01\xD2\xEE\x0EY\xE2\x15\x07\xD2H\x9D\xF5PRR\xF2\x853@a\xCEdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static INTERESTRATE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\t\xEAQ\x9F\x14a\0NW\x80c\x16\x1F\xA6V\x14a\0lW\x80cH\xD4\xB4\x87\x14a\0\x9CW\x80c\xFEK\xABC\x14a\0\xBAW[_\x80\xFD[a\0Va\0\xEBV[`@Qa\0c\x91\x90a\x0E\xA3V[`@Q\x80\x91\x03\x90\xF3[a\0\x86`\x04\x806\x03\x81\x01\x90a\0\x81\x91\x90a\x0E\xF3V[a\x01\x0FV[`@Qa\0\x93\x91\x90a\x10\x15V[`@Q\x80\x91\x03\x90\xF3[a\0\xA4a\x01'V[`@Qa\0\xB1\x91\x90a\x10>V[`@Q\x80\x91\x03\x90\xF3[a\0\xD4`\x04\x806\x03\x81\x01\x90a\0\xCF\x91\x90a\x10WV[a\x01KV[`@Qa\0\xE2\x92\x91\x90a\x10\xA7V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x17a\r\x86V[a\x01 \x82a\x05|V[\x90P\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_\x80_a\x01W\x86a\x05|V[\x90P_a\x01v`\x04\x83`\xA0\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_c\x01\xE13\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x027`\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x81\xCE\x1C#\x8C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xE4\x91\x90a\x10>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xFFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02#\x91\x90a\x11\x07V[c\xFF\xFF\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02A\x91\x90a\x11\x8CV[\x90P_\x83`\xC0\x01Qa\xFF\xFF\x16\x90P_\x81\x03a\x02\x95W\x83` \x01Qa\x02w`\x04\x86`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x95P\x95PPPPPa\x05tV[_\x80\x88\x14a\x02\xD3Wa\x02\xC3a\x02\xB4`\x04\x84a\x0B\x85\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x0B\x9A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x02\xCE\x91\x90a\x11\x8CV[a\x02\xD5V[_[\x90P_\x80\x86``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x03\x03\x90P\x84\x81\x11\x15a\x03\x0FW_\x90P[a\x03\"\x86\x82a\x0B\xC0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP_a\x03Z\x86\x88`\x80\x01Q\x89` \x01Qa\x03>\x91\x90a\x11\xBCV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xC0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x85\x83\x10\x15a\x040W_\x87``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\x8B\x85\x85a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\x95\x91\x90a\x11\xFBV[\x90P_\x88`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xBE\x86\x85a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\xC8\x91\x90a\x11\xFBV[\x90P\x80\x82\x10\x15a\x04\x01W\x80a\x03\xEF`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x05tV[\x81a\x04\x1E`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x05tV[_\x86\x84a\x04=\x91\x90a\x12.V[\x90P_\x88``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04f\x89\x86a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04p\x91\x90a\x11\xFBV[\x90P_\x89`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\x99\x8A\x86a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xA3\x91\x90a\x11\xFBV[\x90P_\x82a\x04\xCC\x85\x8D`\xE0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xD6\x91\x90a\x11\xFBV[\x90P_\x82a\x05\0\x86\x8Ea\x01\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xEA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x05\n\x91\x90a\x11\xFBV[\x90P\x80\x82\x10\x15a\x05FW\x80a\x051`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPPa\x05tV[\x81a\x05c`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0Bp\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPP[\x93P\x93\x91PPV[a\x05\x84a\r\x86V[`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xB1\x91\x90a\x12.V[\x82\x11\x15a\x05\xEAW`@Q\x7F\xCA\x89\xFCI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80_\x80\x85\x03a\x06bW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9FV[`\x01\x85\x03a\x06\xD8W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9EV[`\x02\x85\x03a\x07NW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9DV[`\x03\x85\x03a\x07\xC4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9CV[`\x04\x85\x03a\x08:W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9BV[`\x05\x85\x03a\x08\xB0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x9AV[`\x06\x85\x03a\t&W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\x99V[`\x07\x85\x03a\t\x98W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P[[[[[[[[_\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x1C\x90P_```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x86\x16\x90\x1C\x90P_\x80`\xFF\x16a\xFF\xFF\x86\x16\x90\x1C\x90P_`\x10`\xFF\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x87\x16\x90\x1C\x90P_`p`\xFF\x16y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x90\x1C\x90P_`\xD0`\xFF\x16{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x16\x90\x1C\x90P_`\xE0`\xFF\x16}\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x16\x90\x1C\x90P_\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x90\x1C\x90P_```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x16\x90\x1C\x90P`@Q\x80a\x01 \x01`@R\x80\x8Ak\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88a\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85a\xFF\xFF\x16\x81R` \x01\x84a\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x9CPPPPPPPPPPPPP\x91\x90PV[_a\x0B}\x83\x83`\x1Ba\x0C\x14V[\x90P\x92\x91PPV[_a\x0B\x92\x83\x83`\x12a\x0C\x14V[\x90P\x92\x91PPV[_a\x0B\xB8\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0C\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x0B\xE2k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x0C\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_a\x0C\x0C\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x0C\x87\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[_\x81\x83\x10a\x0C[W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0CR\x92\x91\x90a\x10\xA7V[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x0Cg\x91\x90a\x12.V[`\na\x0Cs\x91\x90a\x13\x90V[\x84a\x0C~\x91\x90a\x13\xDAV[\x90P\x93\x92PPPV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\x0C\xBFW\x83\x82\x81a\x0C\xB5Wa\x0C\xB4a\x112V[[\x04\x92PPPa\r\x7FV[\x80\x84\x11a\x0C\xF8W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`@Q\x80a\x01 \x01`@R\x80_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_a\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_a\xFF\xFF\x16\x81R` \x01_a\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x0Eka\x0Efa\x0Ea\x84a\x0E)V[a\x0EHV[a\x0E)V[\x90P\x91\x90PV[_a\x0E|\x82a\x0EQV[\x90P\x91\x90PV[_a\x0E\x8D\x82a\x0ErV[\x90P\x91\x90PV[a\x0E\x9D\x81a\x0E\x83V[\x82RPPV[_` \x82\x01\x90Pa\x0E\xB6_\x83\x01\x84a\x0E\x94V[\x92\x91PPV[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x0E\xD2\x81a\x0E\xC0V[\x81\x14a\x0E\xDCW_\x80\xFD[PV[_\x815\x90Pa\x0E\xED\x81a\x0E\xC9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\x08Wa\x0F\x07a\x0E\xBCV[[_a\x0F\x15\x84\x82\x85\x01a\x0E\xDFV[\x91PP\x92\x91PPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F>\x81a\x0F\x1EV[\x82RPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0FZ\x81a\x0FDV[\x82RPPV[a\x01 \x82\x01_\x82\x01Qa\x0Fu_\x85\x01\x82a\x0F5V[P` \x82\x01Qa\x0F\x88` \x85\x01\x82a\x0F5V[P`@\x82\x01Qa\x0F\x9B`@\x85\x01\x82a\x0FQV[P``\x82\x01Qa\x0F\xAE``\x85\x01\x82a\x0F5V[P`\x80\x82\x01Qa\x0F\xC1`\x80\x85\x01\x82a\x0F5V[P`\xA0\x82\x01Qa\x0F\xD4`\xA0\x85\x01\x82a\x0FQV[P`\xC0\x82\x01Qa\x0F\xE7`\xC0\x85\x01\x82a\x0FQV[P`\xE0\x82\x01Qa\x0F\xFA`\xE0\x85\x01\x82a\x0F5V[Pa\x01\0\x82\x01Qa\x10\x0Fa\x01\0\x85\x01\x82a\x0F5V[PPPPV[_a\x01 \x82\x01\x90Pa\x10)_\x83\x01\x84a\x0F`V[\x92\x91PPV[a\x108\x81a\x0E\xC0V[\x82RPPV[_` \x82\x01\x90Pa\x10Q_\x83\x01\x84a\x10/V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x10nWa\x10ma\x0E\xBCV[[_a\x10{\x86\x82\x87\x01a\x0E\xDFV[\x93PP` a\x10\x8C\x86\x82\x87\x01a\x0E\xDFV[\x92PP`@a\x10\x9D\x86\x82\x87\x01a\x0E\xDFV[\x91PP\x92P\x92P\x92V[_`@\x82\x01\x90Pa\x10\xBA_\x83\x01\x85a\x10/V[a\x10\xC7` \x83\x01\x84a\x10/V[\x93\x92PPPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10\xE6\x81a\x10\xCEV[\x81\x14a\x10\xF0W_\x80\xFD[PV[_\x81Q\x90Pa\x11\x01\x81a\x10\xDDV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11\x1CWa\x11\x1Ba\x0E\xBCV[[_a\x11)\x84\x82\x85\x01a\x10\xF3V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x11\x96\x82a\x0E\xC0V[\x91Pa\x11\xA1\x83a\x0E\xC0V[\x92P\x82a\x11\xB1Wa\x11\xB0a\x112V[[\x82\x82\x04\x90P\x92\x91PPV[_a\x11\xC6\x82a\x0F\x1EV[\x91Pa\x11\xD1\x83a\x0F\x1EV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xF5Wa\x11\xF4a\x11_V[[\x92\x91PPV[_a\x12\x05\x82a\x0E\xC0V[\x91Pa\x12\x10\x83a\x0E\xC0V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x12(Wa\x12'a\x11_V[[\x92\x91PPV[_a\x128\x82a\x0E\xC0V[\x91Pa\x12C\x83a\x0E\xC0V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x12[Wa\x12Za\x11_V[[\x92\x91PPV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x12\xB6W\x80\x86\x04\x81\x11\x15a\x12\x92Wa\x12\x91a\x11_V[[`\x01\x85\x16\x15a\x12\xA1W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x12\xAF\x85a\x12aV[\x94Pa\x12vV[\x94P\x94\x92PPPV[_\x82a\x12\xCEW`\x01\x90Pa\x13\x89V[\x81a\x12\xDBW_\x90Pa\x13\x89V[\x81`\x01\x81\x14a\x12\xF1W`\x02\x81\x14a\x12\xFBWa\x13*V[`\x01\x91PPa\x13\x89V[`\xFF\x84\x11\x15a\x13\rWa\x13\x0Ca\x11_V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x13$Wa\x13#a\x11_V[[Pa\x13\x89V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x13_W\x82\x82\n\x90P\x83\x81\x11\x15a\x13ZWa\x13Ya\x11_V[[a\x13\x89V[a\x13l\x84\x84\x84`\x01a\x12mV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x13\x83Wa\x13\x82a\x11_V[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x13\x9A\x82a\x0E\xC0V[\x91Pa\x13\xA5\x83a\x0E\xC0V[\x92Pa\x13\xD2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x12\xBFV[\x90P\x92\x91PPV[_a\x13\xE4\x82a\x0E\xC0V[\x91Pa\x13\xEF\x83a\x0E\xC0V[\x92P\x82\x82\x02a\x13\xFD\x81a\x0E\xC0V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x14\x14Wa\x14\x13a\x11_V[[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x14B\xE4_5\xB9[\xE1\xA7\x1D\xF01\xD2\xEE\x0EY\xE2\x15\x07\xD2H\x9D\xF5PRR\xF2\x853@a\xCEdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static INTERESTRATE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InterestRate<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InterestRate<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InterestRate<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InterestRate<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InterestRate<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InterestRate))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InterestRate<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INTERESTRATE_ABI.clone(),
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
                INTERESTRATE_ABI.clone(),
                INTERESTRATE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `COLLATERAL_COUNT` (0x48d4b487) function
        pub fn collateral_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([72, 212, 180, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `YIELD_ORACLE` (0x09ea519f) function
        pub fn yield_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([9, 234, 81, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateInterestRate` (0xfe4bab43) function
        pub fn calculate_interest_rate(
            &self,
            ilk_index: ::ethers::core::types::U256,
            total_ilk_debt: ::ethers::core::types::U256,
            total_eth_supply: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [254, 75, 171, 67],
                    (ilk_index, total_ilk_debt, total_eth_supply),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpackCollateralConfig` (0x161fa656) function
        pub fn unpack_collateral_config(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, IlkData> {
            self.0
                .method_hash([22, 31, 166, 86], index)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InterestRate<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CollateralIndexOutOfBounds` with signature `CollateralIndexOutOfBounds()` and selector `0xca89fc49`
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
        name = "CollateralIndexOutOfBounds",
        abi = "CollateralIndexOutOfBounds()"
    )]
    pub struct CollateralIndexOutOfBounds;
    ///Custom Error type `DistributionFactorsDoNotSumToOne` with signature `DistributionFactorsDoNotSumToOne(uint256)` and selector `0xb9778c09`
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
        name = "DistributionFactorsDoNotSumToOne",
        abi = "DistributionFactorsDoNotSumToOne(uint256)"
    )]
    pub struct DistributionFactorsDoNotSumToOne {
        pub sum: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidIlkDataListLength` with signature `InvalidIlkDataListLength(uint256)` and selector `0xef25438a`
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
        name = "InvalidIlkDataListLength",
        abi = "InvalidIlkDataListLength(uint256)"
    )]
    pub struct InvalidIlkDataListLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidMinimumKinkRate` with signature `InvalidMinimumKinkRate(uint256,uint256)` and selector `0x7ed94442`
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
        name = "InvalidMinimumKinkRate",
        abi = "InvalidMinimumKinkRate(uint256,uint256)"
    )]
    pub struct InvalidMinimumKinkRate {
        pub minimum_kink_rate: ::ethers::core::types::U256,
        pub minimum_base_rate: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidOptimalUtilizationRate` with signature `InvalidOptimalUtilizationRate(uint256)` and selector `0xf2ae6b3d`
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
        name = "InvalidOptimalUtilizationRate",
        abi = "InvalidOptimalUtilizationRate(uint256)"
    )]
    pub struct InvalidOptimalUtilizationRate {
        pub optimal_utilization_rate: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidReserveFactor` with signature `InvalidReserveFactor(uint256)` and selector `0x19a6930e`
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
    #[etherror(name = "InvalidReserveFactor", abi = "InvalidReserveFactor(uint256)")]
    pub struct InvalidReserveFactor {
        pub reserve_factor: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidYieldOracleAddress` with signature `InvalidYieldOracleAddress()` and selector `0x753ed5ee`
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
    #[etherror(name = "InvalidYieldOracleAddress", abi = "InvalidYieldOracleAddress()")]
    pub struct InvalidYieldOracleAddress;
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
    ///Custom Error type `TotalDebtsLength` with signature `TotalDebtsLength(uint256,uint256)` and selector `0xf561668b`
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
    #[etherror(name = "TotalDebtsLength", abi = "TotalDebtsLength(uint256,uint256)")]
    pub struct TotalDebtsLength {
        pub collateral_count: ::ethers::core::types::U256,
        pub total_ilk_debts_length: ::ethers::core::types::U256,
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
    pub enum InterestRateErrors {
        CollateralIndexOutOfBounds(CollateralIndexOutOfBounds),
        DistributionFactorsDoNotSumToOne(DistributionFactorsDoNotSumToOne),
        InvalidIlkDataListLength(InvalidIlkDataListLength),
        InvalidMinimumKinkRate(InvalidMinimumKinkRate),
        InvalidOptimalUtilizationRate(InvalidOptimalUtilizationRate),
        InvalidReserveFactor(InvalidReserveFactor),
        InvalidYieldOracleAddress(InvalidYieldOracleAddress),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        NotScalingUp(NotScalingUp),
        TotalDebtsLength(TotalDebtsLength),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for InterestRateErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CollateralIndexOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollateralIndexOutOfBounds(decoded));
            }
            if let Ok(decoded) = <DistributionFactorsDoNotSumToOne as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DistributionFactorsDoNotSumToOne(decoded));
            }
            if let Ok(decoded) = <InvalidIlkDataListLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidIlkDataListLength(decoded));
            }
            if let Ok(decoded) = <InvalidMinimumKinkRate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMinimumKinkRate(decoded));
            }
            if let Ok(decoded) = <InvalidOptimalUtilizationRate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOptimalUtilizationRate(decoded));
            }
            if let Ok(decoded) = <InvalidReserveFactor as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReserveFactor(decoded));
            }
            if let Ok(decoded) = <InvalidYieldOracleAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidYieldOracleAddress(decoded));
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
            if let Ok(decoded) = <TotalDebtsLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalDebtsLength(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InterestRateErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CollateralIndexOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DistributionFactorsDoNotSumToOne(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidIlkDataListLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinimumKinkRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOptimalUtilizationRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidYieldOracleAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotScalingUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalDebtsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for InterestRateErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CollateralIndexOutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DistributionFactorsDoNotSumToOne as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidIlkDataListLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinimumKinkRate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOptimalUtilizationRate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidReserveFactor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidYieldOracleAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MathOverflowedMulDiv as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotScalingUp as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <TotalDebtsLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for InterestRateErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollateralIndexOutOfBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DistributionFactorsDoNotSumToOne(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidIlkDataListLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinimumKinkRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOptimalUtilizationRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidReserveFactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidYieldOracleAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotScalingUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalDebtsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for InterestRateErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CollateralIndexOutOfBounds> for InterestRateErrors {
        fn from(value: CollateralIndexOutOfBounds) -> Self {
            Self::CollateralIndexOutOfBounds(value)
        }
    }
    impl ::core::convert::From<DistributionFactorsDoNotSumToOne> for InterestRateErrors {
        fn from(value: DistributionFactorsDoNotSumToOne) -> Self {
            Self::DistributionFactorsDoNotSumToOne(value)
        }
    }
    impl ::core::convert::From<InvalidIlkDataListLength> for InterestRateErrors {
        fn from(value: InvalidIlkDataListLength) -> Self {
            Self::InvalidIlkDataListLength(value)
        }
    }
    impl ::core::convert::From<InvalidMinimumKinkRate> for InterestRateErrors {
        fn from(value: InvalidMinimumKinkRate) -> Self {
            Self::InvalidMinimumKinkRate(value)
        }
    }
    impl ::core::convert::From<InvalidOptimalUtilizationRate> for InterestRateErrors {
        fn from(value: InvalidOptimalUtilizationRate) -> Self {
            Self::InvalidOptimalUtilizationRate(value)
        }
    }
    impl ::core::convert::From<InvalidReserveFactor> for InterestRateErrors {
        fn from(value: InvalidReserveFactor) -> Self {
            Self::InvalidReserveFactor(value)
        }
    }
    impl ::core::convert::From<InvalidYieldOracleAddress> for InterestRateErrors {
        fn from(value: InvalidYieldOracleAddress) -> Self {
            Self::InvalidYieldOracleAddress(value)
        }
    }
    impl ::core::convert::From<MathOverflowedMulDiv> for InterestRateErrors {
        fn from(value: MathOverflowedMulDiv) -> Self {
            Self::MathOverflowedMulDiv(value)
        }
    }
    impl ::core::convert::From<NotScalingUp> for InterestRateErrors {
        fn from(value: NotScalingUp) -> Self {
            Self::NotScalingUp(value)
        }
    }
    impl ::core::convert::From<TotalDebtsLength> for InterestRateErrors {
        fn from(value: TotalDebtsLength) -> Self {
            Self::TotalDebtsLength(value)
        }
    }
    ///Container type for all input parameters for the `COLLATERAL_COUNT` function with signature `COLLATERAL_COUNT()` and selector `0x48d4b487`
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
    #[ethcall(name = "COLLATERAL_COUNT", abi = "COLLATERAL_COUNT()")]
    pub struct CollateralCountCall;
    ///Container type for all input parameters for the `YIELD_ORACLE` function with signature `YIELD_ORACLE()` and selector `0x09ea519f`
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
    #[ethcall(name = "YIELD_ORACLE", abi = "YIELD_ORACLE()")]
    pub struct YieldOracleCall;
    ///Container type for all input parameters for the `calculateInterestRate` function with signature `calculateInterestRate(uint256,uint256,uint256)` and selector `0xfe4bab43`
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
        name = "calculateInterestRate",
        abi = "calculateInterestRate(uint256,uint256,uint256)"
    )]
    pub struct CalculateInterestRateCall {
        pub ilk_index: ::ethers::core::types::U256,
        pub total_ilk_debt: ::ethers::core::types::U256,
        pub total_eth_supply: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `unpackCollateralConfig` function with signature `unpackCollateralConfig(uint256)` and selector `0x161fa656`
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
    #[ethcall(name = "unpackCollateralConfig", abi = "unpackCollateralConfig(uint256)")]
    pub struct UnpackCollateralConfigCall {
        pub index: ::ethers::core::types::U256,
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
    pub enum InterestRateCalls {
        CollateralCount(CollateralCountCall),
        YieldOracle(YieldOracleCall),
        CalculateInterestRate(CalculateInterestRateCall),
        UnpackCollateralConfig(UnpackCollateralConfigCall),
    }
    impl ::ethers::core::abi::AbiDecode for InterestRateCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CollateralCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollateralCount(decoded));
            }
            if let Ok(decoded) = <YieldOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::YieldOracle(decoded));
            }
            if let Ok(decoded) = <CalculateInterestRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateInterestRate(decoded));
            }
            if let Ok(decoded) = <UnpackCollateralConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnpackCollateralConfig(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InterestRateCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollateralCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::YieldOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateInterestRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnpackCollateralConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for InterestRateCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollateralCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::YieldOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateInterestRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpackCollateralConfig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CollateralCountCall> for InterestRateCalls {
        fn from(value: CollateralCountCall) -> Self {
            Self::CollateralCount(value)
        }
    }
    impl ::core::convert::From<YieldOracleCall> for InterestRateCalls {
        fn from(value: YieldOracleCall) -> Self {
            Self::YieldOracle(value)
        }
    }
    impl ::core::convert::From<CalculateInterestRateCall> for InterestRateCalls {
        fn from(value: CalculateInterestRateCall) -> Self {
            Self::CalculateInterestRate(value)
        }
    }
    impl ::core::convert::From<UnpackCollateralConfigCall> for InterestRateCalls {
        fn from(value: UnpackCollateralConfigCall) -> Self {
            Self::UnpackCollateralConfig(value)
        }
    }
    ///Container type for all return fields from the `COLLATERAL_COUNT` function with signature `COLLATERAL_COUNT()` and selector `0x48d4b487`
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
    pub struct CollateralCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `YIELD_ORACLE` function with signature `YIELD_ORACLE()` and selector `0x09ea519f`
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
    pub struct YieldOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `calculateInterestRate` function with signature `calculateInterestRate(uint256,uint256,uint256)` and selector `0xfe4bab43`
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
    pub struct CalculateInterestRateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `unpackCollateralConfig` function with signature `unpackCollateralConfig(uint256)` and selector `0x161fa656`
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
    pub struct UnpackCollateralConfigReturn {
        pub ilk_data: IlkData,
    }
    ///`IlkData(uint96,uint96,uint16,uint96,uint96,uint16,uint16,uint96,uint96)`
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
    pub struct IlkData {
        pub adjusted_profit_margin: u128,
        pub minimum_kink_rate: u128,
        pub reserve_factor: u16,
        pub adjusted_base_rate: u128,
        pub minimum_base_rate: u128,
        pub optimal_utilization_rate: u16,
        pub distribution_factor: u16,
        pub adjusted_above_kink_slope: u128,
        pub minimum_above_kink_slope: u128,
    }
}
