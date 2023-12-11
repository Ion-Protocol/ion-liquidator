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
    pub use super::super::shared_types::*;
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
    const __BYTECODE: &[u8] = b"a\x03\xC0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1DC8\x03\x80b\0\x1DC\x839\x81\x81\x01`@R\x81\x01\x90b\0\08\x91\x90b\0\x07\x9BV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\x9FW`@Q\x7Fu>\xD5\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Qa\x03\x80\x81\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0\x80[a\x03\x80Q\x81\x10\x15b\0\x01/W\x83\x81\x81Q\x81\x10b\0\x01\x04Wb\0\x01\x03b\0\x08\x01V[[` \x02` \x01\x01Q`\xC0\x01Qa\xFF\xFF\x16\x82b\0\x01!\x91\x90b\0\x08iV[\x91P\x80`\x01\x01\x90Pb\0\0\xE2V[Pa'\x10\x81\x14b\0\x01yW\x80`@Q\x7F\xB9w\x8C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x01p\x91\x90b\0\x08\xB5V[`@Q\x80\x91\x03\x90\xFD[b\0\x01\x8C\x83`\0b\0\x02\xEE` \x1B` \x1CV[\x80`\xC0\x90\x81RP\x81`\xA0\x90\x81RP\x82`\x80\x90\x81RPPPPb\0\x01\xB7\x83`\x01b\0\x02\xEE` \x1B` \x1CV[\x80a\x01 \x90\x81RP\x81a\x01\0\x90\x81RP\x82`\xE0\x90\x81RPPPPb\0\x01\xE4\x83`\x02b\0\x02\xEE` \x1B` \x1CV[\x80a\x01\x80\x90\x81RP\x81a\x01`\x90\x81RP\x82a\x01@\x90\x81RPPPPb\0\x02\x12\x83`\x03b\0\x02\xEE` \x1B` \x1CV[\x80a\x01\xE0\x90\x81RP\x81a\x01\xC0\x90\x81RP\x82a\x01\xA0\x90\x81RPPPPb\0\x02@\x83`\x04b\0\x02\xEE` \x1B` \x1CV[\x80a\x02@\x90\x81RP\x81a\x02 \x90\x81RP\x82a\x02\0\x90\x81RPPPPb\0\x02n\x83`\x05b\0\x02\xEE` \x1B` \x1CV[\x80a\x02\xA0\x90\x81RP\x81a\x02\x80\x90\x81RP\x82a\x02`\x90\x81RPPPPb\0\x02\x9C\x83`\x06b\0\x02\xEE` \x1B` \x1CV[\x80a\x03\0\x90\x81RP\x81a\x02\xE0\x90\x81RP\x82a\x02\xC0\x90\x81RPPPPb\0\x02\xCA\x83`\x07b\0\x02\xEE` \x1B` \x1CV[\x80a\x03`\x90\x81RP\x81a\x03@\x90\x81RP\x82a\x03 \x90\x81RPPPPPPPb\0\x08\xD2V[`\0\x80`\0a\x03\x80Q\x84\x10b\0\x03\x0FW`\0\x80`\0\x92P\x92P\x92Pb\0\x04\x0CV[`\0\x85\x85\x81Q\x81\x10b\0\x03'Wb\0\x03&b\0\x08\x01V[[` \x02` \x01\x01Q\x90P```\xFF\x16\x81` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`\0`\xFF\x16\x82`\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x93P`\xE0`\xFF\x16\x81`\xC0\x01Qa\xFF\xFF\x16\x90\x1B`\xD0`\xFF\x16\x82`\xA0\x01Qa\xFF\xFF\x16\x90\x1B`p`\xFF\x16\x83`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`\x10`\xFF\x16\x84``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`\0`\xFF\x16\x85`@\x01Qa\xFF\xFF\x16\x90\x1B\x17\x17\x17\x17\x92P```\xFF\x16\x81a\x01\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`\0`\xFF\x16\x82`\xE0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B\x17\x91PP[\x92P\x92P\x92V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x04w\x82b\0\x04,V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x04\x99Wb\0\x04\x98b\0\x04=V[[\x80`@RPPPV[`\0b\0\x04\xAEb\0\x04\x13V[\x90Pb\0\x04\xBC\x82\x82b\0\x04lV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x04\xDFWb\0\x04\xDEb\0\x04=V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x05\x1D\x81b\0\x04\xFAV[\x81\x14b\0\x05)W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x05=\x81b\0\x05\x12V[\x92\x91PPV[`\0a\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x05\\\x81b\0\x05CV[\x81\x14b\0\x05hW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x05|\x81b\0\x05QV[\x92\x91PPV[`\0a\x01 \x82\x84\x03\x12\x15b\0\x05\x9CWb\0\x05\x9Bb\0\x04\xF5V[[b\0\x05\xA9a\x01 b\0\x04\xA2V[\x90P`\0b\0\x05\xBB\x84\x82\x85\x01b\0\x05,V[`\0\x83\x01RP` b\0\x05\xD1\x84\x82\x85\x01b\0\x05,V[` \x83\x01RP`@b\0\x05\xE7\x84\x82\x85\x01b\0\x05kV[`@\x83\x01RP``b\0\x05\xFD\x84\x82\x85\x01b\0\x05,V[``\x83\x01RP`\x80b\0\x06\x13\x84\x82\x85\x01b\0\x05,V[`\x80\x83\x01RP`\xA0b\0\x06)\x84\x82\x85\x01b\0\x05kV[`\xA0\x83\x01RP`\xC0b\0\x06?\x84\x82\x85\x01b\0\x05kV[`\xC0\x83\x01RP`\xE0b\0\x06U\x84\x82\x85\x01b\0\x05,V[`\xE0\x83\x01RPa\x01\0b\0\x06l\x84\x82\x85\x01b\0\x05,V[a\x01\0\x83\x01RP\x92\x91PPV[`\0b\0\x06\x90b\0\x06\x8A\x84b\0\x04\xC1V[b\0\x04\xA2V[\x90P\x80\x83\x82R` \x82\x01\x90Pa\x01 \x84\x02\x83\x01\x85\x81\x11\x15b\0\x06\xB7Wb\0\x06\xB6b\0\x04\xF0V[[\x83[\x81\x81\x10\x15b\0\x06\xE5W\x80b\0\x06\xCF\x88\x82b\0\x05\x82V[\x84R` \x84\x01\x93PPa\x01 \x81\x01\x90Pb\0\x06\xB9V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x07\x07Wb\0\x07\x06b\0\x04'V[[\x81Qb\0\x07\x19\x84\x82` \x86\x01b\0\x06yV[\x91PP\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x07O\x82b\0\x07\"V[\x90P\x91\x90PV[`\0b\0\x07c\x82b\0\x07BV[\x90P\x91\x90PV[b\0\x07u\x81b\0\x07VV[\x81\x14b\0\x07\x81W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x07\x95\x81b\0\x07jV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x07\xB5Wb\0\x07\xB4b\0\x04\x1DV[[`\0\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x07\xD6Wb\0\x07\xD5b\0\x04\"V[[b\0\x07\xE4\x85\x82\x86\x01b\0\x06\xEFV[\x92PP` b\0\x07\xF7\x85\x82\x86\x01b\0\x07\x84V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0b\0\x08v\x82b\0\x080V[\x91Pb\0\x08\x83\x83b\0\x080V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15b\0\x08\x9EWb\0\x08\x9Db\0\x08:V[[\x92\x91PPV[b\0\x08\xAF\x81b\0\x080V[\x82RPPV[`\0` \x82\x01\x90Pb\0\x08\xCC`\0\x83\x01\x84b\0\x08\xA4V[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x01\xE0Qa\x02\0Qa\x02 Qa\x02@Qa\x02`Qa\x02\x80Qa\x02\xA0Qa\x02\xC0Qa\x02\xE0Qa\x03\0Qa\x03 Qa\x03@Qa\x03`Qa\x03\x80Qa\x03\xA0Qa\x139b\0\n\n`\09`\0\x81\x81`\xB5\x01Ra\x01A\x01R`\0\x81\x81`\xD9\x01Ra\x05M\x01R`\0a\t=\x01R`\0a\t\x1A\x01R`\0a\x08\xF7\x01R`\0a\x08\xC7\x01R`\0a\x08\xA4\x01R`\0a\x08\x81\x01R`\0a\x08Q\x01R`\0a\x08.\x01R`\0a\x08\x0B\x01R`\0a\x07\xDB\x01R`\0a\x07\xB8\x01R`\0a\x07\x95\x01R`\0a\x07e\x01R`\0a\x07B\x01R`\0a\x07\x1F\x01R`\0a\x06\xEF\x01R`\0a\x06\xCC\x01R`\0a\x06\xA9\x01R`\0a\x06y\x01R`\0a\x06V\x01R`\0a\x063\x01R`\0a\x06\x03\x01R`\0a\x05\xE0\x01R`\0a\x05\xBD\x01Ra\x139`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\t\xEAQ\x9F\x14a\0FW\x80cH\xD4\xB4\x87\x14a\0dW\x80c\xFEK\xABC\x14a\0\x82W[`\0\x80\xFD[a\0Na\0\xB3V[`@Qa\0[\x91\x90a\x0E\x8FV[`@Q\x80\x91\x03\x90\xF3[a\0la\0\xD7V[`@Qa\0y\x91\x90a\x0E\xC3V[`@Q\x80\x91\x03\x90\xF3[a\0\x9C`\x04\x806\x03\x81\x01\x90a\0\x97\x91\x90a\x0F\x0FV[a\0\xFBV[`@Qa\0\xAA\x92\x91\x90a\x0FbV[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0a\x01\t\x86a\x05AV[\x90P`\0a\x01)`\x04\x83`\xA0\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0c\x01\xE13\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xED`\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x81\xCE\x1C#\x8C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x98\x91\x90a\x0E\xC3V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xD9\x91\x90a\x0F\xC7V[c\xFF\xFF\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x01\xF7\x91\x90a\x10RV[\x90P`\0\x83`\xC0\x01Qa\xFF\xFF\x16\x90P`\0\x81\x03a\x02MW\x83` \x01Qa\x02/`\x04\x86`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x95P\x95PPPPPa\x059V[`\0\x80\x88\x14a\x02\x8CWa\x02|a\x02m`\x04\x84a\x0BV\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x0Bl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x02\x87\x91\x90a\x10RV[a\x02\x8FV[`\0[\x90P`\0\x80\x86``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87`\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x03\x03\x90P\x84\x81\x11\x15a\x02\xCCW`\0\x90P[a\x02\xDF\x86\x82a\x0B\x93\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP`\0a\x03\x18\x86\x88`\x80\x01Q\x89` \x01Qa\x02\xFC\x91\x90a\x10\x9BV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\x93\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x85\x83\x10\x15a\x03\xF0W`\0\x87``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03J\x85\x85a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03T\x91\x90a\x10\xDBV[\x90P`\0\x88`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03~\x86\x85a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\x88\x91\x90a\x10\xDBV[\x90P\x80\x82\x10\x15a\x03\xC1W\x80a\x03\xAF`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x059V[\x81a\x03\xDE`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x059V[`\0\x86\x84a\x03\xFE\x91\x90a\x11\x0FV[\x90P`\0\x88``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04(\x89\x86a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x042\x91\x90a\x10\xDBV[\x90P`\0\x89`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\\\x8A\x86a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04f\x91\x90a\x10\xDBV[\x90P`\0\x82a\x04\x90\x85\x8D`\xE0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\x9A\x91\x90a\x10\xDBV[\x90P`\0\x82a\x04\xC5\x86\x8Ea\x01\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xCF\x91\x90a\x10\xDBV[\x90P\x80\x82\x10\x15a\x05\x0BW\x80a\x04\xF6`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPPa\x059V[\x81a\x05(`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPP[\x93P\x93\x91PPV[a\x05Ia\rdV[`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05v\x91\x90a\x11\x0FV[\x82\x11\x15a\x05\xAFW`@Q\x7F\xCA\x89\xFCI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80\x85\x03a\x06)W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tfV[`\x01\x85\x03a\x06\x9FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\teV[`\x02\x85\x03a\x07\x15W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tdV[`\x03\x85\x03a\x07\x8BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tcV[`\x04\x85\x03a\x08\x01W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tbV[`\x05\x85\x03a\x08wW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\taV[`\x06\x85\x03a\x08\xEDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t`V[`\x07\x85\x03a\t_W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P[[[[[[[[`\0\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x1C\x90P`\0```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x86\x16\x90\x1C\x90P`\0\x80`\xFF\x16a\xFF\xFF\x86\x16\x90\x1C\x90P`\0`\x10`\xFF\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x87\x16\x90\x1C\x90P`\0`p`\xFF\x16y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x90\x1C\x90P`\0`\xD0`\xFF\x16{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x16\x90\x1C\x90P`\0`\xE0`\xFF\x16}\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x16\x90\x1C\x90P`\0\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x90\x1C\x90P`\0```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x16\x90\x1C\x90P`@Q\x80a\x01 \x01`@R\x80\x8Ak\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88a\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85a\xFF\xFF\x16\x81R` \x01\x84a\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x9CPPPPPPPPPPPPP\x91\x90PV[`\0a\x0BN\x83\x83`\x1Ba\x0B\xE9V[\x90P\x92\x91PPV[`\0a\x0Bd\x83\x83`\x12a\x0B\xE9V[\x90P\x92\x91PPV[`\0a\x0B\x8B\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0C]\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x0B\xB6k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x0C]\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x0B\xE1\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x0C]\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x81\x83\x10a\x0C1W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C(\x92\x91\x90a\x0FbV[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x0C=\x91\x90a\x11\x0FV[`\na\x0CI\x91\x90a\x12vV[\x84a\x0CT\x91\x90a\x12\xC1V[\x90P\x93\x92PPPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x0C\x98W\x83\x82\x81a\x0C\x8EWa\x0C\x8Da\x0F\xF4V[[\x04\x92PPPa\r]V[\x80\x84\x11a\x0C\xD1W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`@Q\x80a\x01 \x01`@R\x80`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0a\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0a\xFF\xFF\x16\x81R` \x01`\0a\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x0EUa\x0EPa\x0EK\x84a\x0E\x10V[a\x0E0V[a\x0E\x10V[\x90P\x91\x90PV[`\0a\x0Eg\x82a\x0E:V[\x90P\x91\x90PV[`\0a\x0Ey\x82a\x0E\\V[\x90P\x91\x90PV[a\x0E\x89\x81a\x0EnV[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xA4`\0\x83\x01\x84a\x0E\x80V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0E\xBD\x81a\x0E\xAAV[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xD8`\0\x83\x01\x84a\x0E\xB4V[\x92\x91PPV[`\0\x80\xFD[a\x0E\xEC\x81a\x0E\xAAV[\x81\x14a\x0E\xF7W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\t\x81a\x0E\xE3V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F(Wa\x0F'a\x0E\xDEV[[`\0a\x0F6\x86\x82\x87\x01a\x0E\xFAV[\x93PP` a\x0FG\x86\x82\x87\x01a\x0E\xFAV[\x92PP`@a\x0FX\x86\x82\x87\x01a\x0E\xFAV[\x91PP\x92P\x92P\x92V[`\0`@\x82\x01\x90Pa\x0Fw`\0\x83\x01\x85a\x0E\xB4V[a\x0F\x84` \x83\x01\x84a\x0E\xB4V[\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xA4\x81a\x0F\x8BV[\x81\x14a\x0F\xAFW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x0F\xC1\x81a\x0F\x9BV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\xDDWa\x0F\xDCa\x0E\xDEV[[`\0a\x0F\xEB\x84\x82\x85\x01a\x0F\xB2V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x10]\x82a\x0E\xAAV[\x91Pa\x10h\x83a\x0E\xAAV[\x92P\x82a\x10xWa\x10wa\x0F\xF4V[[\x82\x82\x04\x90P\x92\x91PPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x10\xA6\x82a\x10\x83V[\x91Pa\x10\xB1\x83a\x10\x83V[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xD5Wa\x10\xD4a\x10#V[[\x92\x91PPV[`\0a\x10\xE6\x82a\x0E\xAAV[\x91Pa\x10\xF1\x83a\x0E\xAAV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x11\tWa\x11\x08a\x10#V[[\x92\x91PPV[`\0a\x11\x1A\x82a\x0E\xAAV[\x91Pa\x11%\x83a\x0E\xAAV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x11=Wa\x11<a\x10#V[[\x92\x91PPV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x11\x9AW\x80\x86\x04\x81\x11\x15a\x11vWa\x11ua\x10#V[[`\x01\x85\x16\x15a\x11\x85W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x11\x93\x85a\x11CV[\x94Pa\x11ZV[\x94P\x94\x92PPPV[`\0\x82a\x11\xB3W`\x01\x90Pa\x12oV[\x81a\x11\xC1W`\0\x90Pa\x12oV[\x81`\x01\x81\x14a\x11\xD7W`\x02\x81\x14a\x11\xE1Wa\x12\x10V[`\x01\x91PPa\x12oV[`\xFF\x84\x11\x15a\x11\xF3Wa\x11\xF2a\x10#V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x12\nWa\x12\ta\x10#V[[Pa\x12oV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x12EW\x82\x82\n\x90P\x83\x81\x11\x15a\x12@Wa\x12?a\x10#V[[a\x12oV[a\x12R\x84\x84\x84`\x01a\x11PV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x12iWa\x12ha\x10#V[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\x12\x81\x82a\x0E\xAAV[\x91Pa\x12\x8C\x83a\x0E\xAAV[\x92Pa\x12\xB9\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x11\xA3V[\x90P\x92\x91PPV[`\0a\x12\xCC\x82a\x0E\xAAV[\x91Pa\x12\xD7\x83a\x0E\xAAV[\x92P\x82\x82\x02a\x12\xE5\x81a\x0E\xAAV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x12\xFCWa\x12\xFBa\x10#V[[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 :\x0B5\xA2\xCEL\xA3n\xF1.2\x18\xFD\xF4\x01R\x81|\x94\x81\xCC\xECy\x08\xD2\xE5\x80\x90\xD0\x01 \xCCdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static INTERESTRATE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\t\xEAQ\x9F\x14a\0FW\x80cH\xD4\xB4\x87\x14a\0dW\x80c\xFEK\xABC\x14a\0\x82W[`\0\x80\xFD[a\0Na\0\xB3V[`@Qa\0[\x91\x90a\x0E\x8FV[`@Q\x80\x91\x03\x90\xF3[a\0la\0\xD7V[`@Qa\0y\x91\x90a\x0E\xC3V[`@Q\x80\x91\x03\x90\xF3[a\0\x9C`\x04\x806\x03\x81\x01\x90a\0\x97\x91\x90a\x0F\x0FV[a\0\xFBV[`@Qa\0\xAA\x92\x91\x90a\x0FbV[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\0a\x01\t\x86a\x05AV[\x90P`\0a\x01)`\x04\x83`\xA0\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0c\x01\xE13\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xED`\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x81\xCE\x1C#\x8C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x98\x91\x90a\x0E\xC3V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xD9\x91\x90a\x0F\xC7V[c\xFF\xFF\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x01\xF7\x91\x90a\x10RV[\x90P`\0\x83`\xC0\x01Qa\xFF\xFF\x16\x90P`\0\x81\x03a\x02MW\x83` \x01Qa\x02/`\x04\x86`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x95P\x95PPPPPa\x059V[`\0\x80\x88\x14a\x02\x8CWa\x02|a\x02m`\x04\x84a\x0BV\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x0Bl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89a\x02\x87\x91\x90a\x10RV[a\x02\x8FV[`\0[\x90P`\0\x80\x86``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87`\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x03\x03\x90P\x84\x81\x11\x15a\x02\xCCW`\0\x90P[a\x02\xDF\x86\x82a\x0B\x93\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PP`\0a\x03\x18\x86\x88`\x80\x01Q\x89` \x01Qa\x02\xFC\x91\x90a\x10\x9BV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\x93\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x85\x83\x10\x15a\x03\xF0W`\0\x87``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03J\x85\x85a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03T\x91\x90a\x10\xDBV[\x90P`\0\x88`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03~\x86\x85a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\x88\x91\x90a\x10\xDBV[\x90P\x80\x82\x10\x15a\x03\xC1W\x80a\x03\xAF`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x059V[\x81a\x03\xDE`\x04\x8B`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9AP\x9APPPPPPPPPPa\x059V[`\0\x86\x84a\x03\xFE\x91\x90a\x11\x0FV[\x90P`\0\x88``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04(\x89\x86a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x042\x91\x90a\x10\xDBV[\x90P`\0\x89`\x80\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\\\x8A\x86a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04f\x91\x90a\x10\xDBV[\x90P`\0\x82a\x04\x90\x85\x8D`\xE0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\x9A\x91\x90a\x10\xDBV[\x90P`\0\x82a\x04\xC5\x86\x8Ea\x01\0\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xCF\x91\x90a\x10\xDBV[\x90P\x80\x82\x10\x15a\x05\x0BW\x80a\x04\xF6`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPPa\x059V[\x81a\x05(`\x04\x8E`@\x01Qa\xFF\xFF\x16a\x0B@\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x9DP\x9DPPPPPPPPPPPPP[\x93P\x93\x91PPV[a\x05Ia\rdV[`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05v\x91\x90a\x11\x0FV[\x82\x11\x15a\x05\xAFW`@Q\x7F\xCA\x89\xFCI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80\x85\x03a\x06)W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tfV[`\x01\x85\x03a\x06\x9FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\teV[`\x02\x85\x03a\x07\x15W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tdV[`\x03\x85\x03a\x07\x8BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tcV[`\x04\x85\x03a\x08\x01W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\tbV[`\x05\x85\x03a\x08wW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\taV[`\x06\x85\x03a\x08\xEDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t`V[`\x07\x85\x03a\t_W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P[[[[[[[[`\0\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x1C\x90P`\0```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x86\x16\x90\x1C\x90P`\0\x80`\xFF\x16a\xFF\xFF\x86\x16\x90\x1C\x90P`\0`\x10`\xFF\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x87\x16\x90\x1C\x90P`\0`p`\xFF\x16y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x90\x1C\x90P`\0`\xD0`\xFF\x16{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x16\x90\x1C\x90P`\0`\xE0`\xFF\x16}\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x16\x90\x1C\x90P`\0\x80`\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x90\x1C\x90P`\0```\xFF\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x16\x90\x1C\x90P`@Q\x80a\x01 \x01`@R\x80\x8Ak\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88a\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85a\xFF\xFF\x16\x81R` \x01\x84a\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x9CPPPPPPPPPPPPP\x91\x90PV[`\0a\x0BN\x83\x83`\x1Ba\x0B\xE9V[\x90P\x92\x91PPV[`\0a\x0Bd\x83\x83`\x12a\x0B\xE9V[\x90P\x92\x91PPV[`\0a\x0B\x8B\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0C]\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x0B\xB6k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x83\x85a\x0C]\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0a\x0B\xE1\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a\x0C]\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x81\x83\x10a\x0C1W\x82\x82`@Q\x7F\x1A\x06\\\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C(\x92\x91\x90a\x0FbV[`@Q\x80\x91\x03\x90\xFD[\x82\x82a\x0C=\x91\x90a\x11\x0FV[`\na\x0CI\x91\x90a\x12vV[\x84a\x0CT\x91\x90a\x12\xC1V[\x90P\x93\x92PPPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x0C\x98W\x83\x82\x81a\x0C\x8EWa\x0C\x8Da\x0F\xF4V[[\x04\x92PPPa\r]V[\x80\x84\x11a\x0C\xD1W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`@Q\x80a\x01 \x01`@R\x80`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0a\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0a\xFF\xFF\x16\x81R` \x01`\0a\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x0EUa\x0EPa\x0EK\x84a\x0E\x10V[a\x0E0V[a\x0E\x10V[\x90P\x91\x90PV[`\0a\x0Eg\x82a\x0E:V[\x90P\x91\x90PV[`\0a\x0Ey\x82a\x0E\\V[\x90P\x91\x90PV[a\x0E\x89\x81a\x0EnV[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xA4`\0\x83\x01\x84a\x0E\x80V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0E\xBD\x81a\x0E\xAAV[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xD8`\0\x83\x01\x84a\x0E\xB4V[\x92\x91PPV[`\0\x80\xFD[a\x0E\xEC\x81a\x0E\xAAV[\x81\x14a\x0E\xF7W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\t\x81a\x0E\xE3V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F(Wa\x0F'a\x0E\xDEV[[`\0a\x0F6\x86\x82\x87\x01a\x0E\xFAV[\x93PP` a\x0FG\x86\x82\x87\x01a\x0E\xFAV[\x92PP`@a\x0FX\x86\x82\x87\x01a\x0E\xFAV[\x91PP\x92P\x92P\x92V[`\0`@\x82\x01\x90Pa\x0Fw`\0\x83\x01\x85a\x0E\xB4V[a\x0F\x84` \x83\x01\x84a\x0E\xB4V[\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xA4\x81a\x0F\x8BV[\x81\x14a\x0F\xAFW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x0F\xC1\x81a\x0F\x9BV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\xDDWa\x0F\xDCa\x0E\xDEV[[`\0a\x0F\xEB\x84\x82\x85\x01a\x0F\xB2V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x10]\x82a\x0E\xAAV[\x91Pa\x10h\x83a\x0E\xAAV[\x92P\x82a\x10xWa\x10wa\x0F\xF4V[[\x82\x82\x04\x90P\x92\x91PPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x10\xA6\x82a\x10\x83V[\x91Pa\x10\xB1\x83a\x10\x83V[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xD5Wa\x10\xD4a\x10#V[[\x92\x91PPV[`\0a\x10\xE6\x82a\x0E\xAAV[\x91Pa\x10\xF1\x83a\x0E\xAAV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x11\tWa\x11\x08a\x10#V[[\x92\x91PPV[`\0a\x11\x1A\x82a\x0E\xAAV[\x91Pa\x11%\x83a\x0E\xAAV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x11=Wa\x11<a\x10#V[[\x92\x91PPV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x11\x9AW\x80\x86\x04\x81\x11\x15a\x11vWa\x11ua\x10#V[[`\x01\x85\x16\x15a\x11\x85W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x11\x93\x85a\x11CV[\x94Pa\x11ZV[\x94P\x94\x92PPPV[`\0\x82a\x11\xB3W`\x01\x90Pa\x12oV[\x81a\x11\xC1W`\0\x90Pa\x12oV[\x81`\x01\x81\x14a\x11\xD7W`\x02\x81\x14a\x11\xE1Wa\x12\x10V[`\x01\x91PPa\x12oV[`\xFF\x84\x11\x15a\x11\xF3Wa\x11\xF2a\x10#V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x12\nWa\x12\ta\x10#V[[Pa\x12oV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x12EW\x82\x82\n\x90P\x83\x81\x11\x15a\x12@Wa\x12?a\x10#V[[a\x12oV[a\x12R\x84\x84\x84`\x01a\x11PV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x12iWa\x12ha\x10#V[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\x12\x81\x82a\x0E\xAAV[\x91Pa\x12\x8C\x83a\x0E\xAAV[\x92Pa\x12\xB9\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x11\xA3V[\x90P\x92\x91PPV[`\0a\x12\xCC\x82a\x0E\xAAV[\x91Pa\x12\xD7\x83a\x0E\xAAV[\x92P\x82\x82\x02a\x12\xE5\x81a\x0E\xAAV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x12\xFCWa\x12\xFBa\x10#V[[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 :\x0B5\xA2\xCEL\xA3n\xF1.2\x18\xFD\xF4\x01R\x81|\x94\x81\xCC\xECy\x08\xD2\xE5\x80\x90\xD0\x01 \xCCdsolcC\0\x08\x15\x003";
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
}
