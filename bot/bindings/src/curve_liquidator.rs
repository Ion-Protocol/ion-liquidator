pub use curve_liquidator::*;
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
pub mod curve_liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("ionPool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IonPool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("liquidation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Liquidation"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("weth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IWETH9"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_treasury"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gemJoin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GemJoin"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wethIsToken0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("receiveFlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("receiveFlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::borrow::ToOwned::to_owned("userData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
            events: ::std::collections::BTreeMap::new(),
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
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                            inputs: ::std::vec![],
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CURVELIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x11W_\x80\xFD[P`@Qb\0\x1Cw8\x03\x80b\0\x1Cw\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x02\xE9V[\x83\x83\x83\x83\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\xC0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3`\xA0Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x01l\x92\x91\x90b\0\x03\x83V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15b\0\x01\x89W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xAF\x91\x90b\0\x03\xE8V[PPPPPPPPPb\0\x04\x18V[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\x01\xED\x82b\0\x01\xC2V[\x90P\x91\x90PV[_b\0\x02\0\x82b\0\x01\xE1V[\x90P\x91\x90PV[b\0\x02\x12\x81b\0\x01\xF4V[\x81\x14b\0\x02\x1DW_\x80\xFD[PV[_\x81Q\x90Pb\0\x020\x81b\0\x02\x07V[\x92\x91PPV[_b\0\x02B\x82b\0\x01\xE1V[\x90P\x91\x90PV[b\0\x02T\x81b\0\x026V[\x81\x14b\0\x02_W_\x80\xFD[PV[_\x81Q\x90Pb\0\x02r\x81b\0\x02IV[\x92\x91PPV[_b\0\x02\x84\x82b\0\x01\xE1V[\x90P\x91\x90PV[b\0\x02\x96\x81b\0\x02xV[\x81\x14b\0\x02\xA1W_\x80\xFD[PV[_\x81Q\x90Pb\0\x02\xB4\x81b\0\x02\x8BV[\x92\x91PPV[b\0\x02\xC5\x81b\0\x01\xE1V[\x81\x14b\0\x02\xD0W_\x80\xFD[PV[_\x81Q\x90Pb\0\x02\xE3\x81b\0\x02\xBAV[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15b\0\x03\x04Wb\0\x03\x03b\0\x01\xBEV[[_b\0\x03\x13\x87\x82\x88\x01b\0\x02 V[\x94PP` b\0\x03&\x87\x82\x88\x01b\0\x02bV[\x93PP`@b\0\x039\x87\x82\x88\x01b\0\x02\xA4V[\x92PP``b\0\x03L\x87\x82\x88\x01b\0\x02\xD3V[\x91PP\x92\x95\x91\x94P\x92PV[b\0\x03c\x81b\0\x01\xE1V[\x82RPPV[_\x81\x90P\x91\x90PV[b\0\x03}\x81b\0\x03iV[\x82RPPV[_`@\x82\x01\x90Pb\0\x03\x98_\x83\x01\x85b\0\x03XV[b\0\x03\xA7` \x83\x01\x84b\0\x03rV[\x93\x92PPPV[_\x81\x15\x15\x90P\x91\x90PV[b\0\x03\xC4\x81b\0\x03\xAEV[\x81\x14b\0\x03\xCFW_\x80\xFD[PV[_\x81Q\x90Pb\0\x03\xE2\x81b\0\x03\xB9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15b\0\x04\0Wb\0\x03\xFFb\0\x01\xBEV[[_b\0\x04\x0F\x84\x82\x85\x01b\0\x03\xD2V[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x18\x10b\0\x04g_9_a\x05\xF5\x01R_\x81\x81`\xBF\x01R\x81\x81a\x05\xAC\x01Ra\x06\x17\x01R_\x81\x81a\x01z\x01Ra\x03\xC7\x01R_\x81\x81a\x03+\x01Ra\x04g\x01Ra\x18\x10_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cl\xB0D)\x14a\08W\x80c\xF0O'\x07\x14a\0TW[_\x80\xFD[a\0R`\x04\x806\x03\x81\x01\x90a\0M\x91\x90a\x0B\xD9V[a\0pV[\0[a\0n`\x04\x806\x03\x81\x01\x90a\0i\x91\x90a\x0F\x90V[a\x02\xE4V[\0[_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x8CWa\0\x8Ba\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xBAW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\0\xF1Wa\0\xF0a\x10dV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01GWa\x01Fa\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01uW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xD3\x92\x91\x90a\x10\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xEEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x12\x91\x90a\x10\xEAV[\x81_\x81Q\x81\x10a\x02%Wa\x02$a\x10dV[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\x7F\x96\x95\x94\x93\x92\x91\x90a\x11\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xAD\x94\x93\x92\x91\x90a\x14&V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xD6W=_\x80>=_\xFD[PPPPPPPPPPPPV[_\x83_\x81Q\x81\x10a\x02\xF8Wa\x02\xF7a\x10dV[[` \x02` \x01\x01Q\x90P_\x80_\x80_\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03\x1C\x91\x90a\x15\x1DV[\x95P\x95P\x95P\x95P\x95P\x95P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x880`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x84\x92\x91\x90a\x10\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x9FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC3\x91\x90a\x10\xEAV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\"\x93\x92\x91\x90a\x15\xA6V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04=W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04a\x91\x90a\x15\xDBV[PP_\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x8A0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xC0\x92\x91\x90a\x10\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFF\x91\x90a\x10\xEAV[a\x05\t\x91\x90a\x16FV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05F\x92\x91\x90a\x16\x88V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05]W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05oW=_\x80>=_\xFD[PPPP_a\x05\x80\x85\x88\x84\x87a\x06lV[\x90P_\x8A\x82a\x05\x8F\x91\x90a\x16FV[\x90Pa\x05\xF0s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x97\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x97\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[_\x80\x85_\x1C\x90P_\x81\x90P_\x80\x85a\x06\x86W_`\x01a\x06\x8AV[`\x01_[\x91P\x91P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x85\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xC9\x92\x91\x90a\x16\x88V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\t\x91\x90a\x16\xAFV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\xF0!$\x83\x83\x8A_`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07I\x94\x93\x92\x91\x90a\x17.V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07eW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x89\x91\x90a\x10\xEAV[\x94PPPPP\x94\x93PPPPV[a\x08\x11\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x07\xCA\x92\x91\x90a\x16\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x08\x16V[PPPV[_a\x08@\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x81Q\x14\x15\x80\x15a\x08dWP\x80\x80` \x01\x90Q\x81\x01\x90a\x08b\x91\x90a\x16\xAFV[\x15[\x15a\x08\xA6W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x9D\x91\x90a\x17qV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x08\xB8\x83\x83_a\x08\xC0V[\x90P\x92\x91PPV[``\x81G\x10\x15a\t\x07W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xFE\x91\x90a\x17qV[`@Q\x80\x91\x03\x90\xFD[_\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t/\x91\x90a\x17\xC4V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\tiW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\tnV[``\x91P[P\x91P\x91Pa\t~\x86\x83\x83a\t\x89V[\x92PPP\x93\x92PPPV[``\x82a\t\x9EWa\t\x99\x82a\n\x16V[a\n\x0EV[_\x82Q\x14\x80\x15a\t\xC4WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\n\x06W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFD\x91\x90a\x17qV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\n\x0FV[[\x93\x92PPPV[_\x81Q\x11\x15a\n(W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\n\x80\x81a\nkV[\x81\x14a\n\x8AW_\x80\xFD[PV[_\x815\x90Pa\n\x9B\x81a\nwV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\n\xCA\x82a\n\xA1V[\x90P\x91\x90PV[a\n\xDA\x81a\n\xC0V[\x81\x14a\n\xE4W_\x80\xFD[PV[_\x815\x90Pa\n\xF5\x81a\n\xD1V[\x92\x91PPV[_a\x0B\x05\x82a\n\xC0V[\x90P\x91\x90PV[a\x0B\x15\x81a\n\xFBV[\x81\x14a\x0B\x1FW_\x80\xFD[PV[_\x815\x90Pa\x0B0\x81a\x0B\x0CV[\x92\x91PPV[_a\x0B@\x82a\n\xC0V[\x90P\x91\x90PV[a\x0BP\x81a\x0B6V[\x81\x14a\x0BZW_\x80\xFD[PV[_\x815\x90Pa\x0Bk\x81a\x0BGV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0B\x83\x81a\x0BqV[\x81\x14a\x0B\x8DW_\x80\xFD[PV[_\x815\x90Pa\x0B\x9E\x81a\x0BzV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0B\xB8\x81a\x0B\xA4V[\x81\x14a\x0B\xC2W_\x80\xFD[PV[_\x815\x90Pa\x0B\xD3\x81a\x0B\xAFV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x0B\xF3Wa\x0B\xF2a\ncV[[_a\x0C\0\x89\x82\x8A\x01a\n\x8DV[\x96PP` a\x0C\x11\x89\x82\x8A\x01a\n\xE7V[\x95PP`@a\x0C\"\x89\x82\x8A\x01a\x0B\"V[\x94PP``a\x0C3\x89\x82\x8A\x01a\x0B]V[\x93PP`\x80a\x0CD\x89\x82\x8A\x01a\x0B\x90V[\x92PP`\xA0a\x0CU\x89\x82\x8A\x01a\x0B\xC5V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0C\xAC\x82a\x0CfV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\xCBWa\x0C\xCAa\x0CvV[[\x80`@RPPPV[_a\x0C\xDDa\nZV[\x90Pa\x0C\xE9\x82\x82a\x0C\xA3V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\x08Wa\r\x07a\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_a\r'\x82a\n\xC0V[\x90P\x91\x90PV[a\r7\x81a\r\x1DV[\x81\x14a\rAW_\x80\xFD[PV[_\x815\x90Pa\rR\x81a\r.V[\x92\x91PPV[_a\rja\re\x84a\x0C\xEEV[a\x0C\xD4V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\r\x8DWa\r\x8Ca\r\x19V[[\x83[\x81\x81\x10\x15a\r\xB6W\x80a\r\xA2\x88\x82a\rDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\r\x8FV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\r\xD4Wa\r\xD3a\x0CbV[[\x815a\r\xE4\x84\x82` \x86\x01a\rXV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x07Wa\x0E\x06a\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x0E*\x81a\x0E\x18V[\x81\x14a\x0E4W_\x80\xFD[PV[_\x815\x90Pa\x0EE\x81a\x0E!V[\x92\x91PPV[_a\x0E]a\x0EX\x84a\r\xEDV[a\x0C\xD4V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0E\x80Wa\x0E\x7Fa\r\x19V[[\x83[\x81\x81\x10\x15a\x0E\xA9W\x80a\x0E\x95\x88\x82a\x0E7V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0E\x82V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0E\xC7Wa\x0E\xC6a\x0CbV[[\x815a\x0E\xD7\x84\x82` \x86\x01a\x0EKV[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xFEWa\x0E\xFDa\x0CvV[[a\x0F\x07\x82a\x0CfV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0F4a\x0F/\x84a\x0E\xE4V[a\x0C\xD4V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0FPWa\x0FOa\x0E\xE0V[[a\x0F[\x84\x82\x85a\x0F\x14V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0FwWa\x0Fva\x0CbV[[\x815a\x0F\x87\x84\x82` \x86\x01a\x0F\"V[\x91PP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x0F\xA8Wa\x0F\xA7a\ncV[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xC5Wa\x0F\xC4a\ngV[[a\x0F\xD1\x87\x82\x88\x01a\r\xC0V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xF2Wa\x0F\xF1a\ngV[[a\x0F\xFE\x87\x82\x88\x01a\x0E\xB3V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x1FWa\x10\x1Ea\ngV[[a\x10+\x87\x82\x88\x01a\x0E\xB3V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10LWa\x10Ka\ngV[[a\x10X\x87\x82\x88\x01a\x0FcV[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x10\x9A\x81a\nkV[\x82RPPV[a\x10\xA9\x81a\n\xC0V[\x82RPPV[_`@\x82\x01\x90Pa\x10\xC2_\x83\x01\x85a\x10\x91V[a\x10\xCF` \x83\x01\x84a\x10\xA0V[\x93\x92PPPV[_\x81Q\x90Pa\x10\xE4\x81a\x0E!V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10\xFFWa\x10\xFEa\ncV[[_a\x11\x0C\x84\x82\x85\x01a\x10\xD6V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x118a\x113a\x11.\x84a\n\xA1V[a\x11\x15V[a\n\xA1V[\x90P\x91\x90PV[_a\x11I\x82a\x11\x1EV[\x90P\x91\x90PV[_a\x11Z\x82a\x11?V[\x90P\x91\x90PV[a\x11j\x81a\x11PV[\x82RPPV[_a\x11z\x82a\x11?V[\x90P\x91\x90PV[a\x11\x8A\x81a\x11pV[\x82RPPV[a\x11\x99\x81a\x0BqV[\x82RPPV[a\x11\xA8\x81a\x0B\xA4V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x11\xC1_\x83\x01\x89a\x10\x91V[a\x11\xCE` \x83\x01\x88a\x10\xA0V[a\x11\xDB`@\x83\x01\x87a\x11aV[a\x11\xE8``\x83\x01\x86a\x11\x81V[a\x11\xF5`\x80\x83\x01\x85a\x11\x90V[a\x12\x02`\xA0\x83\x01\x84a\x11\x9FV[\x97\x96PPPPPPPV[_a\x12\x17\x82a\x11?V[\x90P\x91\x90PV[a\x12'\x81a\x12\rV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\x12`\x82a\x11?V[\x90P\x91\x90PV[a\x12p\x81a\x12VV[\x82RPPV[_a\x12\x81\x83\x83a\x12gV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x12\xA3\x82a\x12-V[a\x12\xAD\x81\x85a\x127V[\x93Pa\x12\xB8\x83a\x12GV[\x80_[\x83\x81\x10\x15a\x12\xE8W\x81Qa\x12\xCF\x88\x82a\x12vV[\x97Pa\x12\xDA\x83a\x12\x8DV[\x92PP`\x01\x81\x01\x90Pa\x12\xBBV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x13'\x81a\x0E\x18V[\x82RPPV[_a\x138\x83\x83a\x13\x1EV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x13Z\x82a\x12\xF5V[a\x13d\x81\x85a\x12\xFFV[\x93Pa\x13o\x83a\x13\x0FV[\x80_[\x83\x81\x10\x15a\x13\x9FW\x81Qa\x13\x86\x88\x82a\x13-V[\x97Pa\x13\x91\x83a\x13DV[\x92PP`\x01\x81\x01\x90Pa\x13rV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x13\xE3W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x13\xC8V[_\x84\x84\x01RPPPPV[_a\x13\xF8\x82a\x13\xACV[a\x14\x02\x81\x85a\x13\xB6V[\x93Pa\x14\x12\x81\x85` \x86\x01a\x13\xC6V[a\x14\x1B\x81a\x0CfV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa\x149_\x83\x01\x87a\x12\x1EV[\x81\x81\x03` \x83\x01Ra\x14K\x81\x86a\x12\x99V[\x90P\x81\x81\x03`@\x83\x01Ra\x14_\x81\x85a\x13PV[\x90P\x81\x81\x03``\x83\x01Ra\x14s\x81\x84a\x13\xEEV[\x90P\x95\x94PPPPPV[_\x81Q\x90Pa\x14\x8C\x81a\nwV[\x92\x91PPV[_a\x14\x9C\x82a\n\xA1V[\x90P\x91\x90PV[a\x14\xAC\x81a\x14\x92V[\x81\x14a\x14\xB6W_\x80\xFD[PV[_\x81Q\x90Pa\x14\xC7\x81a\x14\xA3V[\x92\x91PPV[_\x81Q\x90Pa\x14\xDB\x81a\x0B\x0CV[\x92\x91PPV[_\x81Q\x90Pa\x14\xEF\x81a\x0BGV[\x92\x91PPV[_\x81Q\x90Pa\x15\x03\x81a\x0BzV[\x92\x91PPV[_\x81Q\x90Pa\x15\x17\x81a\x0B\xAFV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x157Wa\x156a\ncV[[_a\x15D\x89\x82\x8A\x01a\x14~V[\x96PP` a\x15U\x89\x82\x8A\x01a\x14\xB9V[\x95PP`@a\x15f\x89\x82\x8A\x01a\x14\xCDV[\x94PP``a\x15w\x89\x82\x8A\x01a\x14\xE1V[\x93PP`\x80a\x15\x88\x89\x82\x8A\x01a\x14\xF5V[\x92PP`\xA0a\x15\x99\x89\x82\x8A\x01a\x15\tV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_``\x82\x01\x90Pa\x15\xB9_\x83\x01\x86a\x10\x91V[a\x15\xC6` \x83\x01\x85a\x10\xA0V[a\x15\xD3`@\x83\x01\x84a\x10\xA0V[\x94\x93PPPPV[_\x80`@\x83\x85\x03\x12\x15a\x15\xF1Wa\x15\xF0a\ncV[[_a\x15\xFE\x85\x82\x86\x01a\x10\xD6V[\x92PP` a\x16\x0F\x85\x82\x86\x01a\x10\xD6V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x16P\x82a\x0E\x18V[\x91Pa\x16[\x83a\x0E\x18V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x16sWa\x16ra\x16\x19V[[\x92\x91PPV[a\x16\x82\x81a\x0E\x18V[\x82RPPV[_`@\x82\x01\x90Pa\x16\x9B_\x83\x01\x85a\x10\xA0V[a\x16\xA8` \x83\x01\x84a\x16yV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x16\xC4Wa\x16\xC3a\ncV[[_a\x16\xD1\x84\x82\x85\x01a\x15\tV[\x91PP\x92\x91PPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x16\xEF\x81a\x16\xDAV[\x82RPPV[_\x81\x90P\x91\x90PV[_a\x17\x18a\x17\x13a\x17\x0E\x84a\x16\xF5V[a\x11\x15V[a\x0E\x18V[\x90P\x91\x90PV[a\x17(\x81a\x16\xFEV[\x82RPPV[_`\x80\x82\x01\x90Pa\x17A_\x83\x01\x87a\x16\xE6V[a\x17N` \x83\x01\x86a\x16\xE6V[a\x17[`@\x83\x01\x85a\x16yV[a\x17h``\x83\x01\x84a\x17\x1FV[\x95\x94PPPPPV[_` \x82\x01\x90Pa\x17\x84_\x83\x01\x84a\x10\xA0V[\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a\x17\x9E\x82a\x13\xACV[a\x17\xA8\x81\x85a\x17\x8AV[\x93Pa\x17\xB8\x81\x85` \x86\x01a\x13\xC6V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x17\xCF\x82\x84a\x17\x94V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD5\x95\x90\xA6\xA2\xF5})_\xBE\x08\xEF-\x1D\x01q\xC0\xD07\x80k\x8Bs2\x86\xDE0\xC53\xDAgtdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static CURVELIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cl\xB0D)\x14a\08W\x80c\xF0O'\x07\x14a\0TW[_\x80\xFD[a\0R`\x04\x806\x03\x81\x01\x90a\0M\x91\x90a\x0B\xD9V[a\0pV[\0[a\0n`\x04\x806\x03\x81\x01\x90a\0i\x91\x90a\x0F\x90V[a\x02\xE4V[\0[_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x8CWa\0\x8Ba\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xBAW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\0\xF1Wa\0\xF0a\x10dV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01GWa\x01Fa\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01uW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xD3\x92\x91\x90a\x10\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xEEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x12\x91\x90a\x10\xEAV[\x81_\x81Q\x81\x10a\x02%Wa\x02$a\x10dV[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\x7F\x96\x95\x94\x93\x92\x91\x90a\x11\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xAD\x94\x93\x92\x91\x90a\x14&V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xD6W=_\x80>=_\xFD[PPPPPPPPPPPPV[_\x83_\x81Q\x81\x10a\x02\xF8Wa\x02\xF7a\x10dV[[` \x02` \x01\x01Q\x90P_\x80_\x80_\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03\x1C\x91\x90a\x15\x1DV[\x95P\x95P\x95P\x95P\x95P\x95P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x880`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x84\x92\x91\x90a\x10\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x9FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC3\x91\x90a\x10\xEAV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\"\x93\x92\x91\x90a\x15\xA6V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04=W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04a\x91\x90a\x15\xDBV[PP_\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x8A0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xC0\x92\x91\x90a\x10\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFF\x91\x90a\x10\xEAV[a\x05\t\x91\x90a\x16FV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05F\x92\x91\x90a\x16\x88V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05]W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05oW=_\x80>=_\xFD[PPPP_a\x05\x80\x85\x88\x84\x87a\x06lV[\x90P_\x8A\x82a\x05\x8F\x91\x90a\x16FV[\x90Pa\x05\xF0s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x97\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x97\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[_\x80\x85_\x1C\x90P_\x81\x90P_\x80\x85a\x06\x86W_`\x01a\x06\x8AV[`\x01_[\x91P\x91P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x85\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xC9\x92\x91\x90a\x16\x88V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\t\x91\x90a\x16\xAFV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\xF0!$\x83\x83\x8A_`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07I\x94\x93\x92\x91\x90a\x17.V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07eW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x89\x91\x90a\x10\xEAV[\x94PPPPP\x94\x93PPPPV[a\x08\x11\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x07\xCA\x92\x91\x90a\x16\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x08\x16V[PPPV[_a\x08@\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x81Q\x14\x15\x80\x15a\x08dWP\x80\x80` \x01\x90Q\x81\x01\x90a\x08b\x91\x90a\x16\xAFV[\x15[\x15a\x08\xA6W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x9D\x91\x90a\x17qV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x08\xB8\x83\x83_a\x08\xC0V[\x90P\x92\x91PPV[``\x81G\x10\x15a\t\x07W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xFE\x91\x90a\x17qV[`@Q\x80\x91\x03\x90\xFD[_\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t/\x91\x90a\x17\xC4V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\tiW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\tnV[``\x91P[P\x91P\x91Pa\t~\x86\x83\x83a\t\x89V[\x92PPP\x93\x92PPPV[``\x82a\t\x9EWa\t\x99\x82a\n\x16V[a\n\x0EV[_\x82Q\x14\x80\x15a\t\xC4WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\n\x06W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFD\x91\x90a\x17qV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\n\x0FV[[\x93\x92PPPV[_\x81Q\x11\x15a\n(W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\n\x80\x81a\nkV[\x81\x14a\n\x8AW_\x80\xFD[PV[_\x815\x90Pa\n\x9B\x81a\nwV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\n\xCA\x82a\n\xA1V[\x90P\x91\x90PV[a\n\xDA\x81a\n\xC0V[\x81\x14a\n\xE4W_\x80\xFD[PV[_\x815\x90Pa\n\xF5\x81a\n\xD1V[\x92\x91PPV[_a\x0B\x05\x82a\n\xC0V[\x90P\x91\x90PV[a\x0B\x15\x81a\n\xFBV[\x81\x14a\x0B\x1FW_\x80\xFD[PV[_\x815\x90Pa\x0B0\x81a\x0B\x0CV[\x92\x91PPV[_a\x0B@\x82a\n\xC0V[\x90P\x91\x90PV[a\x0BP\x81a\x0B6V[\x81\x14a\x0BZW_\x80\xFD[PV[_\x815\x90Pa\x0Bk\x81a\x0BGV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0B\x83\x81a\x0BqV[\x81\x14a\x0B\x8DW_\x80\xFD[PV[_\x815\x90Pa\x0B\x9E\x81a\x0BzV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0B\xB8\x81a\x0B\xA4V[\x81\x14a\x0B\xC2W_\x80\xFD[PV[_\x815\x90Pa\x0B\xD3\x81a\x0B\xAFV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x0B\xF3Wa\x0B\xF2a\ncV[[_a\x0C\0\x89\x82\x8A\x01a\n\x8DV[\x96PP` a\x0C\x11\x89\x82\x8A\x01a\n\xE7V[\x95PP`@a\x0C\"\x89\x82\x8A\x01a\x0B\"V[\x94PP``a\x0C3\x89\x82\x8A\x01a\x0B]V[\x93PP`\x80a\x0CD\x89\x82\x8A\x01a\x0B\x90V[\x92PP`\xA0a\x0CU\x89\x82\x8A\x01a\x0B\xC5V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0C\xAC\x82a\x0CfV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\xCBWa\x0C\xCAa\x0CvV[[\x80`@RPPPV[_a\x0C\xDDa\nZV[\x90Pa\x0C\xE9\x82\x82a\x0C\xA3V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\x08Wa\r\x07a\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_a\r'\x82a\n\xC0V[\x90P\x91\x90PV[a\r7\x81a\r\x1DV[\x81\x14a\rAW_\x80\xFD[PV[_\x815\x90Pa\rR\x81a\r.V[\x92\x91PPV[_a\rja\re\x84a\x0C\xEEV[a\x0C\xD4V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\r\x8DWa\r\x8Ca\r\x19V[[\x83[\x81\x81\x10\x15a\r\xB6W\x80a\r\xA2\x88\x82a\rDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\r\x8FV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\r\xD4Wa\r\xD3a\x0CbV[[\x815a\r\xE4\x84\x82` \x86\x01a\rXV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x07Wa\x0E\x06a\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x0E*\x81a\x0E\x18V[\x81\x14a\x0E4W_\x80\xFD[PV[_\x815\x90Pa\x0EE\x81a\x0E!V[\x92\x91PPV[_a\x0E]a\x0EX\x84a\r\xEDV[a\x0C\xD4V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0E\x80Wa\x0E\x7Fa\r\x19V[[\x83[\x81\x81\x10\x15a\x0E\xA9W\x80a\x0E\x95\x88\x82a\x0E7V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0E\x82V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0E\xC7Wa\x0E\xC6a\x0CbV[[\x815a\x0E\xD7\x84\x82` \x86\x01a\x0EKV[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xFEWa\x0E\xFDa\x0CvV[[a\x0F\x07\x82a\x0CfV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0F4a\x0F/\x84a\x0E\xE4V[a\x0C\xD4V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0FPWa\x0FOa\x0E\xE0V[[a\x0F[\x84\x82\x85a\x0F\x14V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0FwWa\x0Fva\x0CbV[[\x815a\x0F\x87\x84\x82` \x86\x01a\x0F\"V[\x91PP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x0F\xA8Wa\x0F\xA7a\ncV[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xC5Wa\x0F\xC4a\ngV[[a\x0F\xD1\x87\x82\x88\x01a\r\xC0V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xF2Wa\x0F\xF1a\ngV[[a\x0F\xFE\x87\x82\x88\x01a\x0E\xB3V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x1FWa\x10\x1Ea\ngV[[a\x10+\x87\x82\x88\x01a\x0E\xB3V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10LWa\x10Ka\ngV[[a\x10X\x87\x82\x88\x01a\x0FcV[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x10\x9A\x81a\nkV[\x82RPPV[a\x10\xA9\x81a\n\xC0V[\x82RPPV[_`@\x82\x01\x90Pa\x10\xC2_\x83\x01\x85a\x10\x91V[a\x10\xCF` \x83\x01\x84a\x10\xA0V[\x93\x92PPPV[_\x81Q\x90Pa\x10\xE4\x81a\x0E!V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10\xFFWa\x10\xFEa\ncV[[_a\x11\x0C\x84\x82\x85\x01a\x10\xD6V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x118a\x113a\x11.\x84a\n\xA1V[a\x11\x15V[a\n\xA1V[\x90P\x91\x90PV[_a\x11I\x82a\x11\x1EV[\x90P\x91\x90PV[_a\x11Z\x82a\x11?V[\x90P\x91\x90PV[a\x11j\x81a\x11PV[\x82RPPV[_a\x11z\x82a\x11?V[\x90P\x91\x90PV[a\x11\x8A\x81a\x11pV[\x82RPPV[a\x11\x99\x81a\x0BqV[\x82RPPV[a\x11\xA8\x81a\x0B\xA4V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x11\xC1_\x83\x01\x89a\x10\x91V[a\x11\xCE` \x83\x01\x88a\x10\xA0V[a\x11\xDB`@\x83\x01\x87a\x11aV[a\x11\xE8``\x83\x01\x86a\x11\x81V[a\x11\xF5`\x80\x83\x01\x85a\x11\x90V[a\x12\x02`\xA0\x83\x01\x84a\x11\x9FV[\x97\x96PPPPPPPV[_a\x12\x17\x82a\x11?V[\x90P\x91\x90PV[a\x12'\x81a\x12\rV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\x12`\x82a\x11?V[\x90P\x91\x90PV[a\x12p\x81a\x12VV[\x82RPPV[_a\x12\x81\x83\x83a\x12gV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x12\xA3\x82a\x12-V[a\x12\xAD\x81\x85a\x127V[\x93Pa\x12\xB8\x83a\x12GV[\x80_[\x83\x81\x10\x15a\x12\xE8W\x81Qa\x12\xCF\x88\x82a\x12vV[\x97Pa\x12\xDA\x83a\x12\x8DV[\x92PP`\x01\x81\x01\x90Pa\x12\xBBV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x13'\x81a\x0E\x18V[\x82RPPV[_a\x138\x83\x83a\x13\x1EV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x13Z\x82a\x12\xF5V[a\x13d\x81\x85a\x12\xFFV[\x93Pa\x13o\x83a\x13\x0FV[\x80_[\x83\x81\x10\x15a\x13\x9FW\x81Qa\x13\x86\x88\x82a\x13-V[\x97Pa\x13\x91\x83a\x13DV[\x92PP`\x01\x81\x01\x90Pa\x13rV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x13\xE3W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x13\xC8V[_\x84\x84\x01RPPPPV[_a\x13\xF8\x82a\x13\xACV[a\x14\x02\x81\x85a\x13\xB6V[\x93Pa\x14\x12\x81\x85` \x86\x01a\x13\xC6V[a\x14\x1B\x81a\x0CfV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa\x149_\x83\x01\x87a\x12\x1EV[\x81\x81\x03` \x83\x01Ra\x14K\x81\x86a\x12\x99V[\x90P\x81\x81\x03`@\x83\x01Ra\x14_\x81\x85a\x13PV[\x90P\x81\x81\x03``\x83\x01Ra\x14s\x81\x84a\x13\xEEV[\x90P\x95\x94PPPPPV[_\x81Q\x90Pa\x14\x8C\x81a\nwV[\x92\x91PPV[_a\x14\x9C\x82a\n\xA1V[\x90P\x91\x90PV[a\x14\xAC\x81a\x14\x92V[\x81\x14a\x14\xB6W_\x80\xFD[PV[_\x81Q\x90Pa\x14\xC7\x81a\x14\xA3V[\x92\x91PPV[_\x81Q\x90Pa\x14\xDB\x81a\x0B\x0CV[\x92\x91PPV[_\x81Q\x90Pa\x14\xEF\x81a\x0BGV[\x92\x91PPV[_\x81Q\x90Pa\x15\x03\x81a\x0BzV[\x92\x91PPV[_\x81Q\x90Pa\x15\x17\x81a\x0B\xAFV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x157Wa\x156a\ncV[[_a\x15D\x89\x82\x8A\x01a\x14~V[\x96PP` a\x15U\x89\x82\x8A\x01a\x14\xB9V[\x95PP`@a\x15f\x89\x82\x8A\x01a\x14\xCDV[\x94PP``a\x15w\x89\x82\x8A\x01a\x14\xE1V[\x93PP`\x80a\x15\x88\x89\x82\x8A\x01a\x14\xF5V[\x92PP`\xA0a\x15\x99\x89\x82\x8A\x01a\x15\tV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_``\x82\x01\x90Pa\x15\xB9_\x83\x01\x86a\x10\x91V[a\x15\xC6` \x83\x01\x85a\x10\xA0V[a\x15\xD3`@\x83\x01\x84a\x10\xA0V[\x94\x93PPPPV[_\x80`@\x83\x85\x03\x12\x15a\x15\xF1Wa\x15\xF0a\ncV[[_a\x15\xFE\x85\x82\x86\x01a\x10\xD6V[\x92PP` a\x16\x0F\x85\x82\x86\x01a\x10\xD6V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x16P\x82a\x0E\x18V[\x91Pa\x16[\x83a\x0E\x18V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x16sWa\x16ra\x16\x19V[[\x92\x91PPV[a\x16\x82\x81a\x0E\x18V[\x82RPPV[_`@\x82\x01\x90Pa\x16\x9B_\x83\x01\x85a\x10\xA0V[a\x16\xA8` \x83\x01\x84a\x16yV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x16\xC4Wa\x16\xC3a\ncV[[_a\x16\xD1\x84\x82\x85\x01a\x15\tV[\x91PP\x92\x91PPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x16\xEF\x81a\x16\xDAV[\x82RPPV[_\x81\x90P\x91\x90PV[_a\x17\x18a\x17\x13a\x17\x0E\x84a\x16\xF5V[a\x11\x15V[a\x0E\x18V[\x90P\x91\x90PV[a\x17(\x81a\x16\xFEV[\x82RPPV[_`\x80\x82\x01\x90Pa\x17A_\x83\x01\x87a\x16\xE6V[a\x17N` \x83\x01\x86a\x16\xE6V[a\x17[`@\x83\x01\x85a\x16yV[a\x17h``\x83\x01\x84a\x17\x1FV[\x95\x94PPPPPV[_` \x82\x01\x90Pa\x17\x84_\x83\x01\x84a\x10\xA0V[\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a\x17\x9E\x82a\x13\xACV[a\x17\xA8\x81\x85a\x17\x8AV[\x93Pa\x17\xB8\x81\x85` \x86\x01a\x13\xC6V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x17\xCF\x82\x84a\x17\x94V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD5\x95\x90\xA6\xA2\xF5})_\xBE\x08\xEF-\x1D\x01q\xC0\xD07\x80k\x8Bs2\x86\xDE0\xC53\xDAgtdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static CURVELIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CurveLiquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CurveLiquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CurveLiquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CurveLiquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CurveLiquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CurveLiquidator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CurveLiquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CURVELIQUIDATOR_ABI.clone(),
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
                CURVELIQUIDATOR_ABI.clone(),
                CURVELIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `liquidate` (0x6cb04429) function
        pub fn liquidate(
            &self,
            ilk_index: u8,
            user: ::ethers::core::types::Address,
            collateral_token: ::ethers::core::types::Address,
            gem_join: ::ethers::core::types::Address,
            pool_id: [u8; 32],
            weth_is_token_0: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [108, 176, 68, 41],
                    (
                        ilk_index,
                        user,
                        collateral_token,
                        gem_join,
                        pool_id,
                        weth_is_token_0,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `receiveFlashLoan` (0xf04f2707) function
        pub fn receive_flash_loan(
            &self,
            p0: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 79, 39, 7], (p0, amounts, p2, user_data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CurveLiquidator<M> {
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
    pub enum CurveLiquidatorErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CurveLiquidatorErrors {
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
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CurveLiquidatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for CurveLiquidatorErrors {
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
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CurveLiquidatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CurveLiquidatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for CurveLiquidatorErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for CurveLiquidatorErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for CurveLiquidatorErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for CurveLiquidatorErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(uint8,address,address,address,bytes32,bool)` and selector `0x6cb04429`
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
        name = "liquidate",
        abi = "liquidate(uint8,address,address,address,bytes32,bool)"
    )]
    pub struct LiquidateCall {
        pub ilk_index: u8,
        pub user: ::ethers::core::types::Address,
        pub collateral_token: ::ethers::core::types::Address,
        pub gem_join: ::ethers::core::types::Address,
        pub pool_id: [u8; 32],
        pub weth_is_token_0: bool,
    }
    ///Container type for all input parameters for the `receiveFlashLoan` function with signature `receiveFlashLoan(address[],uint256[],uint256[],bytes)` and selector `0xf04f2707`
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
        name = "receiveFlashLoan",
        abi = "receiveFlashLoan(address[],uint256[],uint256[],bytes)"
    )]
    pub struct ReceiveFlashLoanCall {
        pub p0: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub p2: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
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
    pub enum CurveLiquidatorCalls {
        Liquidate(LiquidateCall),
        ReceiveFlashLoan(ReceiveFlashLoanCall),
    }
    impl ::ethers::core::abi::AbiDecode for CurveLiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <ReceiveFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReceiveFlashLoan(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CurveLiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReceiveFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CurveLiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceiveFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LiquidateCall> for CurveLiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<ReceiveFlashLoanCall> for CurveLiquidatorCalls {
        fn from(value: ReceiveFlashLoanCall) -> Self {
            Self::ReceiveFlashLoan(value)
        }
    }
}
