pub use uniswap_v3_liquidator::*;
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
pub mod uniswap_v3_liquidator {
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
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
    pub static UNISWAPV3LIQUIDATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0 ]8\x03\x80b\0 ]\x839\x81\x81\x01`@R\x81\x01\x90b\0\08\x91\x90b\0\x02\xFBV[\x83\x83\x83\x83\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\xC0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3`\xA0Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x01m\x92\x91\x90b\0\x03\x99V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x01\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xB3\x91\x90b\0\x04\x03V[PPPPPPPPPb\0\x045V[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x01\xF4\x82b\0\x01\xC7V[\x90P\x91\x90PV[`\0b\0\x02\x08\x82b\0\x01\xE7V[\x90P\x91\x90PV[b\0\x02\x1A\x81b\0\x01\xFBV[\x81\x14b\0\x02&W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02:\x81b\0\x02\x0FV[\x92\x91PPV[`\0b\0\x02M\x82b\0\x01\xE7V[\x90P\x91\x90PV[b\0\x02_\x81b\0\x02@V[\x81\x14b\0\x02kW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\x7F\x81b\0\x02TV[\x92\x91PPV[`\0b\0\x02\x92\x82b\0\x01\xE7V[\x90P\x91\x90PV[b\0\x02\xA4\x81b\0\x02\x85V[\x81\x14b\0\x02\xB0W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\xC4\x81b\0\x02\x99V[\x92\x91PPV[b\0\x02\xD5\x81b\0\x01\xE7V[\x81\x14b\0\x02\xE1W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\xF5\x81b\0\x02\xCAV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x03\x18Wb\0\x03\x17b\0\x01\xC2V[[`\0b\0\x03(\x87\x82\x88\x01b\0\x02)V[\x94PP` b\0\x03;\x87\x82\x88\x01b\0\x02nV[\x93PP`@b\0\x03N\x87\x82\x88\x01b\0\x02\xB3V[\x92PP``b\0\x03a\x87\x82\x88\x01b\0\x02\xE4V[\x91PP\x92\x95\x91\x94P\x92PV[b\0\x03x\x81b\0\x01\xE7V[\x82RPPV[`\0\x81\x90P\x91\x90PV[b\0\x03\x93\x81b\0\x03~V[\x82RPPV[`\0`@\x82\x01\x90Pb\0\x03\xB0`\0\x83\x01\x85b\0\x03mV[b\0\x03\xBF` \x83\x01\x84b\0\x03\x88V[\x93\x92PPPV[`\0\x81\x15\x15\x90P\x91\x90PV[b\0\x03\xDD\x81b\0\x03\xC6V[\x81\x14b\0\x03\xE9W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\xFD\x81b\0\x03\xD2V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x04\x1CWb\0\x04\x1Bb\0\x01\xC2V[[`\0b\0\x04,\x84\x82\x85\x01b\0\x03\xECV[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x1B\xDDb\0\x04\x80`\09`\0a\x05\xE9\x01R`\0\x81\x81`\xEA\x01R\x81\x81a\x05\xA0\x01Ra\x06\x0B\x01R`\0\x81\x81a\x01\xA7\x01Ra\x03\xE2\x01R`\0PPa\x1B\xDD`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cl\xB0D)\x14a\0FW\x80c\xF0O'\x07\x14a\0bW\x80c\xFAF\x1E3\x14a\0~W[`\0\x80\xFD[a\0``\x04\x806\x03\x81\x01\x90a\0[\x91\x90a\x0C\x89V[a\0\x9AV[\0[a\0|`\x04\x806\x03\x81\x01\x90a\0w\x91\x90a\x10[V[a\x03\x19V[\0[a\0\x98`\x04\x806\x03\x81\x01\x90a\0\x93\x91\x90a\x11\xC3V[a\x06`V[\0[`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xB7Wa\0\xB6a\r,V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xE5W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x01\x1DWa\x01\x1Ca\x127V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01tWa\x01sa\r,V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xA2W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\0\x92\x91\x90a\x12\x84V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02A\x91\x90a\x12\xC2V[\x81`\0\x81Q\x81\x10a\x02UWa\x02Ta\x127V[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\xAF\x96\x95\x94\x93\x92\x91\x90a\x13\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xDD\x94\x93\x92\x91\x90a\x16\x1CV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x0BW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x83`\0\x81Q\x81\x10a\x03/Wa\x03.a\x127V[[` \x02` \x01\x01Q\x90P`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03V\x91\x90a\x17\x1DV[\x95P\x95P\x95P\x95P\x95P\x95P`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x9D\x91\x90a\x17\xAAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDE\x91\x90a\x12\xC2V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04=\x93\x92\x91\x90a\x17\xC5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04kW=`\0\x80>=`\0\xFD[PPPP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xAA\x92\x91\x90a\x18\x0BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xD8W=`\0\x80>=`\0\xFD[PPPP`\0\x81\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x18\x91\x90a\x17\xAAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x055W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05Y\x91\x90a\x12\xC2V[a\x05c\x91\x90a\x18cV[\x90P`\0a\x05s\x85\x88\x84\x87a\x07\x05V[\x90P`\0\x8A\x82a\x05\x83\x91\x90a\x18cV[\x90Pa\x05\xE4s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08'\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06O\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08'\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[`\0\x82\x82\x81\x01\x90a\x06q\x91\x90a\x18\x97V[\x90P`\0\x80\x86\x13\x15a\x06\x85W\x85\x90Pa\x06\xD2V[`\0\x85\x13\x15a\x06\x96W\x84\x90Pa\x06\xD1V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xC8\x90a\x19GV[`@Q\x80\x91\x03\x90\xFD[[a\x06\xFD3\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08'\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPV[`\0\x80\x85`\0\x1C\x90P`\0\x83\x15\x90P`\0\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x12\x8A\xCB\x080\x85\x8A\x87a\x07aW`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x07\\\x91\x90a\x19gV[a\x07tV[`\x01d\x01\0\x02v\xA3a\x07s\x91\x90a\x19\xAFV[[\x8D`@Q` \x01a\x07\x85\x91\x90a\x19\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xB4\x95\x94\x93\x92\x91\x90a\x1A0V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF6\x91\x90a\x1A\x9FV[\x91P\x91P\x82a\x08\x0EW\x81a\x08\t\x90a\x1A\xDFV[a\x08\x19V[\x80a\x08\x18\x90a\x1A\xDFV[[\x94PPPPP\x94\x93PPPPV[a\x08\xA1\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x08Z\x92\x91\x90a\x18\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x08\xA6V[PPPV[`\0a\x08\xD1\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\x08\xF6WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\xF4\x91\x90a\x1B'V[\x15[\x15a\t8W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t/\x91\x90a\x17\xAAV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\tK\x83\x83`\0a\tSV[\x90P\x92\x91PPV[``\x81G\x10\x15a\t\x9AW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x91\x91\x90a\x17\xAAV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t\xC3\x91\x90a\x1B\x90V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x05V[``\x91P[P\x91P\x91Pa\n\x15\x86\x83\x83a\n V[\x92PPP\x93\x92PPPV[``\x82a\n5Wa\n0\x82a\n\xAFV[a\n\xA7V[`\0\x82Q\x14\x80\x15a\n]WP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\n\x9FW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x96\x91\x90a\x17\xAAV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\n\xA8V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\n\xC2W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x0B\x1E\x81a\x0B\x08V[\x81\x14a\x0B)W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B;\x81a\x0B\x15V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0Bl\x82a\x0BAV[\x90P\x91\x90PV[a\x0B|\x81a\x0BaV[\x81\x14a\x0B\x87W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\x99\x81a\x0BsV[\x92\x91PPV[`\0a\x0B\xAA\x82a\x0BaV[\x90P\x91\x90PV[a\x0B\xBA\x81a\x0B\x9FV[\x81\x14a\x0B\xC5W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\xD7\x81a\x0B\xB1V[\x92\x91PPV[`\0a\x0B\xE8\x82a\x0BaV[\x90P\x91\x90PV[a\x0B\xF8\x81a\x0B\xDDV[\x81\x14a\x0C\x03W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\x15\x81a\x0B\xEFV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0C.\x81a\x0C\x1BV[\x81\x14a\x0C9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0CK\x81a\x0C%V[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0Cf\x81a\x0CQV[\x81\x14a\x0CqW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\x83\x81a\x0C]V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0C\xA6Wa\x0C\xA5a\n\xFEV[[`\0a\x0C\xB4\x89\x82\x8A\x01a\x0B,V[\x96PP` a\x0C\xC5\x89\x82\x8A\x01a\x0B\x8AV[\x95PP`@a\x0C\xD6\x89\x82\x8A\x01a\x0B\xC8V[\x94PP``a\x0C\xE7\x89\x82\x8A\x01a\x0C\x06V[\x93PP`\x80a\x0C\xF8\x89\x82\x8A\x01a\x0C<V[\x92PP`\xA0a\r\t\x89\x82\x8A\x01a\x0CtV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\rd\x82a\r\x1BV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\x83Wa\r\x82a\r,V[[\x80`@RPPPV[`\0a\r\x96a\n\xF4V[\x90Pa\r\xA2\x82\x82a\r[V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xC2Wa\r\xC1a\r,V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a\r\xE3\x82a\x0BaV[\x90P\x91\x90PV[a\r\xF3\x81a\r\xD8V[\x81\x14a\r\xFEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\x10\x81a\r\xEAV[\x92\x91PPV[`\0a\x0E)a\x0E$\x84a\r\xA7V[a\r\x8CV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0ELWa\x0EKa\r\xD3V[[\x83[\x81\x81\x10\x15a\x0EuW\x80a\x0Ea\x88\x82a\x0E\x01V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0ENV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\x94Wa\x0E\x93a\r\x16V[[\x815a\x0E\xA4\x84\x82` \x86\x01a\x0E\x16V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xC8Wa\x0E\xC7a\r,V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x0E\xEC\x81a\x0E\xD9V[\x81\x14a\x0E\xF7W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\t\x81a\x0E\xE3V[\x92\x91PPV[`\0a\x0F\"a\x0F\x1D\x84a\x0E\xADV[a\r\x8CV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0FEWa\x0FDa\r\xD3V[[\x83[\x81\x81\x10\x15a\x0FnW\x80a\x0FZ\x88\x82a\x0E\xFAV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0FGV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\x8DWa\x0F\x8Ca\r\x16V[[\x815a\x0F\x9D\x84\x82` \x86\x01a\x0F\x0FV[\x91PP\x92\x91PPV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\xC6Wa\x0F\xC5a\r,V[[a\x0F\xCF\x82a\r\x1BV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x0F\xFEa\x0F\xF9\x84a\x0F\xABV[a\r\x8CV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x10\x1AWa\x10\x19a\x0F\xA6V[[a\x10%\x84\x82\x85a\x0F\xDCV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x10BWa\x10Aa\r\x16V[[\x815a\x10R\x84\x82` \x86\x01a\x0F\xEBV[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10uWa\x10ta\n\xFEV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x93Wa\x10\x92a\x0B\x03V[[a\x10\x9F\x87\x82\x88\x01a\x0E\x7FV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xC0Wa\x10\xBFa\x0B\x03V[[a\x10\xCC\x87\x82\x88\x01a\x0FxV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xEDWa\x10\xECa\x0B\x03V[[a\x10\xF9\x87\x82\x88\x01a\x0FxV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x1AWa\x11\x19a\x0B\x03V[[a\x11&\x87\x82\x88\x01a\x10-V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81\x90P\x91\x90PV[a\x11E\x81a\x112V[\x81\x14a\x11PW`\0\x80\xFD[PV[`\0\x815\x90Pa\x11b\x81a\x11<V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x11\x83Wa\x11\x82a\r\x16V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xA0Wa\x11\x9Fa\x11hV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x11\xBCWa\x11\xBBa\r\xD3V[[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x11\xDDWa\x11\xDCa\n\xFEV[[`\0a\x11\xEB\x87\x82\x88\x01a\x11SV[\x94PP` a\x11\xFC\x87\x82\x88\x01a\x11SV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x1DWa\x12\x1Ca\x0B\x03V[[a\x12)\x87\x82\x88\x01a\x11mV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x12o\x81a\x0B\x08V[\x82RPPV[a\x12~\x81a\x0BaV[\x82RPPV[`\0`@\x82\x01\x90Pa\x12\x99`\0\x83\x01\x85a\x12fV[a\x12\xA6` \x83\x01\x84a\x12uV[\x93\x92PPPV[`\0\x81Q\x90Pa\x12\xBC\x81a\x0E\xE3V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\xD8Wa\x12\xD7a\n\xFEV[[`\0a\x12\xE6\x84\x82\x85\x01a\x12\xADV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x13\x14a\x13\x0Fa\x13\n\x84a\x0BAV[a\x12\xEFV[a\x0BAV[\x90P\x91\x90PV[`\0a\x13&\x82a\x12\xF9V[\x90P\x91\x90PV[`\0a\x138\x82a\x13\x1BV[\x90P\x91\x90PV[a\x13H\x81a\x13-V[\x82RPPV[`\0a\x13Y\x82a\x13\x1BV[\x90P\x91\x90PV[a\x13i\x81a\x13NV[\x82RPPV[a\x13x\x81a\x0C\x1BV[\x82RPPV[a\x13\x87\x81a\x0CQV[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x13\xA2`\0\x83\x01\x89a\x12fV[a\x13\xAF` \x83\x01\x88a\x12uV[a\x13\xBC`@\x83\x01\x87a\x13?V[a\x13\xC9``\x83\x01\x86a\x13`V[a\x13\xD6`\x80\x83\x01\x85a\x13oV[a\x13\xE3`\xA0\x83\x01\x84a\x13~V[\x97\x96PPPPPPPV[`\0a\x13\xF9\x82a\x13\x1BV[\x90P\x91\x90PV[a\x14\t\x81a\x13\xEEV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x14F\x82a\x13\x1BV[\x90P\x91\x90PV[a\x14V\x81a\x14;V[\x82RPPV[`\0a\x14h\x83\x83a\x14MV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x14\x8C\x82a\x14\x0FV[a\x14\x96\x81\x85a\x14\x1AV[\x93Pa\x14\xA1\x83a\x14+V[\x80`\0[\x83\x81\x10\x15a\x14\xD2W\x81Qa\x14\xB9\x88\x82a\x14\\V[\x97Pa\x14\xC4\x83a\x14tV[\x92PP`\x01\x81\x01\x90Pa\x14\xA5V[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x15\x14\x81a\x0E\xD9V[\x82RPPV[`\0a\x15&\x83\x83a\x15\x0BV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x15J\x82a\x14\xDFV[a\x15T\x81\x85a\x14\xEAV[\x93Pa\x15_\x83a\x14\xFBV[\x80`\0[\x83\x81\x10\x15a\x15\x90W\x81Qa\x15w\x88\x82a\x15\x1AV[\x97Pa\x15\x82\x83a\x152V[\x92PP`\x01\x81\x01\x90Pa\x15cV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x15\xD7W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x15\xBCV[`\0\x84\x84\x01RPPPPV[`\0a\x15\xEE\x82a\x15\x9DV[a\x15\xF8\x81\x85a\x15\xA8V[\x93Pa\x16\x08\x81\x85` \x86\x01a\x15\xB9V[a\x16\x11\x81a\r\x1BV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x161`\0\x83\x01\x87a\x14\0V[\x81\x81\x03` \x83\x01Ra\x16C\x81\x86a\x14\x81V[\x90P\x81\x81\x03`@\x83\x01Ra\x16W\x81\x85a\x15?V[\x90P\x81\x81\x03``\x83\x01Ra\x16k\x81\x84a\x15\xE3V[\x90P\x95\x94PPPPPV[`\0\x81Q\x90Pa\x16\x85\x81a\x0B\x15V[\x92\x91PPV[`\0a\x16\x96\x82a\x0BAV[\x90P\x91\x90PV[a\x16\xA6\x81a\x16\x8BV[\x81\x14a\x16\xB1W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x16\xC3\x81a\x16\x9DV[\x92\x91PPV[`\0\x81Q\x90Pa\x16\xD8\x81a\x0B\xB1V[\x92\x91PPV[`\0\x81Q\x90Pa\x16\xED\x81a\x0B\xEFV[\x92\x91PPV[`\0\x81Q\x90Pa\x17\x02\x81a\x0C%V[\x92\x91PPV[`\0\x81Q\x90Pa\x17\x17\x81a\x0C]V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x17:Wa\x179a\n\xFEV[[`\0a\x17H\x89\x82\x8A\x01a\x16vV[\x96PP` a\x17Y\x89\x82\x8A\x01a\x16\xB4V[\x95PP`@a\x17j\x89\x82\x8A\x01a\x16\xC9V[\x94PP``a\x17{\x89\x82\x8A\x01a\x16\xDEV[\x93PP`\x80a\x17\x8C\x89\x82\x8A\x01a\x16\xF3V[\x92PP`\xA0a\x17\x9D\x89\x82\x8A\x01a\x17\x08V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x01\x90Pa\x17\xBF`\0\x83\x01\x84a\x12uV[\x92\x91PPV[`\0``\x82\x01\x90Pa\x17\xDA`\0\x83\x01\x86a\x12fV[a\x17\xE7` \x83\x01\x85a\x12uV[a\x17\xF4`@\x83\x01\x84a\x12uV[\x94\x93PPPPV[a\x18\x05\x81a\x0E\xD9V[\x82RPPV[`\0`@\x82\x01\x90Pa\x18 `\0\x83\x01\x85a\x12uV[a\x18-` \x83\x01\x84a\x17\xFCV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x18n\x82a\x0E\xD9V[\x91Pa\x18y\x83a\x0E\xD9V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x18\x91Wa\x18\x90a\x184V[[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x18\xADWa\x18\xACa\n\xFEV[[`\0a\x18\xBB\x84\x82\x85\x01a\x0B\xC8V[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FUniswapV3Liquidator: no tokens r`\0\x82\x01R\x7Feceived\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x191`'\x83a\x18\xC4V[\x91Pa\x19<\x82a\x18\xD5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x19`\x81a\x19$V[\x90P\x91\x90PV[`\0a\x19r\x82a\x0BAV[\x91Pa\x19}\x83a\x0BAV[\x92P\x82\x82\x03\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xA9Wa\x19\xA8a\x184V[[\x92\x91PPV[`\0a\x19\xBA\x82a\x0BAV[\x91Pa\x19\xC5\x83a\x0BAV[\x92P\x82\x82\x01\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xF1Wa\x19\xF0a\x184V[[\x92\x91PPV[`\0` \x82\x01\x90Pa\x1A\x0C`\0\x83\x01\x84a\x13?V[\x92\x91PPV[a\x1A\x1B\x81a\x112V[\x82RPPV[a\x1A*\x81a\x0BAV[\x82RPPV[`\0`\xA0\x82\x01\x90Pa\x1AE`\0\x83\x01\x88a\x12uV[a\x1AR` \x83\x01\x87a\x13~V[a\x1A_`@\x83\x01\x86a\x1A\x12V[a\x1Al``\x83\x01\x85a\x1A!V[\x81\x81\x03`\x80\x83\x01Ra\x1A~\x81\x84a\x15\xE3V[\x90P\x96\x95PPPPPPV[`\0\x81Q\x90Pa\x1A\x99\x81a\x11<V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xB6Wa\x1A\xB5a\n\xFEV[[`\0a\x1A\xC4\x85\x82\x86\x01a\x1A\x8AV[\x92PP` a\x1A\xD5\x85\x82\x86\x01a\x1A\x8AV[\x91PP\x92P\x92\x90PV[`\0a\x1A\xEA\x82a\x112V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1B\x1CWa\x1B\x1Ba\x184V[[\x81`\0\x03\x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1B=Wa\x1B<a\n\xFEV[[`\0a\x1BK\x84\x82\x85\x01a\x17\x08V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[`\0a\x1Bj\x82a\x15\x9DV[a\x1Bt\x81\x85a\x1BTV[\x93Pa\x1B\x84\x81\x85` \x86\x01a\x15\xB9V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x1B\x9C\x82\x84a\x1B_V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x1A\xDA\xE7\x92\xBE\x15%]\xFC\xF9hGF$\xE5\xD6C\x1EF+GA\x864\xC3\xA6\x84\x04\x0B\x88\xD7CdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static UNISWAPV3LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cl\xB0D)\x14a\0FW\x80c\xF0O'\x07\x14a\0bW\x80c\xFAF\x1E3\x14a\0~W[`\0\x80\xFD[a\0``\x04\x806\x03\x81\x01\x90a\0[\x91\x90a\x0C\x89V[a\0\x9AV[\0[a\0|`\x04\x806\x03\x81\x01\x90a\0w\x91\x90a\x10[V[a\x03\x19V[\0[a\0\x98`\x04\x806\x03\x81\x01\x90a\0\x93\x91\x90a\x11\xC3V[a\x06`V[\0[`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xB7Wa\0\xB6a\r,V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xE5W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x01\x1DWa\x01\x1Ca\x127V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01tWa\x01sa\r,V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xA2W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\0\x92\x91\x90a\x12\x84V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02A\x91\x90a\x12\xC2V[\x81`\0\x81Q\x81\x10a\x02UWa\x02Ta\x127V[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\xAF\x96\x95\x94\x93\x92\x91\x90a\x13\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xDD\x94\x93\x92\x91\x90a\x16\x1CV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x0BW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x83`\0\x81Q\x81\x10a\x03/Wa\x03.a\x127V[[` \x02` \x01\x01Q\x90P`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03V\x91\x90a\x17\x1DV[\x95P\x95P\x95P\x95P\x95P\x95P`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x9D\x91\x90a\x17\xAAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDE\x91\x90a\x12\xC2V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04=\x93\x92\x91\x90a\x17\xC5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04kW=`\0\x80>=`\0\xFD[PPPP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xAA\x92\x91\x90a\x18\x0BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xD8W=`\0\x80>=`\0\xFD[PPPP`\0\x81\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x18\x91\x90a\x17\xAAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x055W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05Y\x91\x90a\x12\xC2V[a\x05c\x91\x90a\x18cV[\x90P`\0a\x05s\x85\x88\x84\x87a\x07\x05V[\x90P`\0\x8A\x82a\x05\x83\x91\x90a\x18cV[\x90Pa\x05\xE4s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08'\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06O\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08'\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[`\0\x82\x82\x81\x01\x90a\x06q\x91\x90a\x18\x97V[\x90P`\0\x80\x86\x13\x15a\x06\x85W\x85\x90Pa\x06\xD2V[`\0\x85\x13\x15a\x06\x96W\x84\x90Pa\x06\xD1V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xC8\x90a\x19GV[`@Q\x80\x91\x03\x90\xFD[[a\x06\xFD3\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08'\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPV[`\0\x80\x85`\0\x1C\x90P`\0\x83\x15\x90P`\0\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x12\x8A\xCB\x080\x85\x8A\x87a\x07aW`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x07\\\x91\x90a\x19gV[a\x07tV[`\x01d\x01\0\x02v\xA3a\x07s\x91\x90a\x19\xAFV[[\x8D`@Q` \x01a\x07\x85\x91\x90a\x19\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xB4\x95\x94\x93\x92\x91\x90a\x1A0V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF6\x91\x90a\x1A\x9FV[\x91P\x91P\x82a\x08\x0EW\x81a\x08\t\x90a\x1A\xDFV[a\x08\x19V[\x80a\x08\x18\x90a\x1A\xDFV[[\x94PPPPP\x94\x93PPPPV[a\x08\xA1\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x08Z\x92\x91\x90a\x18\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x08\xA6V[PPPV[`\0a\x08\xD1\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t=\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\x08\xF6WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\xF4\x91\x90a\x1B'V[\x15[\x15a\t8W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t/\x91\x90a\x17\xAAV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\tK\x83\x83`\0a\tSV[\x90P\x92\x91PPV[``\x81G\x10\x15a\t\x9AW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x91\x91\x90a\x17\xAAV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t\xC3\x91\x90a\x1B\x90V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x05V[``\x91P[P\x91P\x91Pa\n\x15\x86\x83\x83a\n V[\x92PPP\x93\x92PPPV[``\x82a\n5Wa\n0\x82a\n\xAFV[a\n\xA7V[`\0\x82Q\x14\x80\x15a\n]WP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\n\x9FW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x96\x91\x90a\x17\xAAV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\n\xA8V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\n\xC2W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x0B\x1E\x81a\x0B\x08V[\x81\x14a\x0B)W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B;\x81a\x0B\x15V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0Bl\x82a\x0BAV[\x90P\x91\x90PV[a\x0B|\x81a\x0BaV[\x81\x14a\x0B\x87W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\x99\x81a\x0BsV[\x92\x91PPV[`\0a\x0B\xAA\x82a\x0BaV[\x90P\x91\x90PV[a\x0B\xBA\x81a\x0B\x9FV[\x81\x14a\x0B\xC5W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\xD7\x81a\x0B\xB1V[\x92\x91PPV[`\0a\x0B\xE8\x82a\x0BaV[\x90P\x91\x90PV[a\x0B\xF8\x81a\x0B\xDDV[\x81\x14a\x0C\x03W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\x15\x81a\x0B\xEFV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0C.\x81a\x0C\x1BV[\x81\x14a\x0C9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0CK\x81a\x0C%V[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0Cf\x81a\x0CQV[\x81\x14a\x0CqW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\x83\x81a\x0C]V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0C\xA6Wa\x0C\xA5a\n\xFEV[[`\0a\x0C\xB4\x89\x82\x8A\x01a\x0B,V[\x96PP` a\x0C\xC5\x89\x82\x8A\x01a\x0B\x8AV[\x95PP`@a\x0C\xD6\x89\x82\x8A\x01a\x0B\xC8V[\x94PP``a\x0C\xE7\x89\x82\x8A\x01a\x0C\x06V[\x93PP`\x80a\x0C\xF8\x89\x82\x8A\x01a\x0C<V[\x92PP`\xA0a\r\t\x89\x82\x8A\x01a\x0CtV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\rd\x82a\r\x1BV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\x83Wa\r\x82a\r,V[[\x80`@RPPPV[`\0a\r\x96a\n\xF4V[\x90Pa\r\xA2\x82\x82a\r[V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xC2Wa\r\xC1a\r,V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a\r\xE3\x82a\x0BaV[\x90P\x91\x90PV[a\r\xF3\x81a\r\xD8V[\x81\x14a\r\xFEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\x10\x81a\r\xEAV[\x92\x91PPV[`\0a\x0E)a\x0E$\x84a\r\xA7V[a\r\x8CV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0ELWa\x0EKa\r\xD3V[[\x83[\x81\x81\x10\x15a\x0EuW\x80a\x0Ea\x88\x82a\x0E\x01V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0ENV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\x94Wa\x0E\x93a\r\x16V[[\x815a\x0E\xA4\x84\x82` \x86\x01a\x0E\x16V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xC8Wa\x0E\xC7a\r,V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x0E\xEC\x81a\x0E\xD9V[\x81\x14a\x0E\xF7W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\t\x81a\x0E\xE3V[\x92\x91PPV[`\0a\x0F\"a\x0F\x1D\x84a\x0E\xADV[a\r\x8CV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0FEWa\x0FDa\r\xD3V[[\x83[\x81\x81\x10\x15a\x0FnW\x80a\x0FZ\x88\x82a\x0E\xFAV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0FGV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\x8DWa\x0F\x8Ca\r\x16V[[\x815a\x0F\x9D\x84\x82` \x86\x01a\x0F\x0FV[\x91PP\x92\x91PPV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\xC6Wa\x0F\xC5a\r,V[[a\x0F\xCF\x82a\r\x1BV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x0F\xFEa\x0F\xF9\x84a\x0F\xABV[a\r\x8CV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x10\x1AWa\x10\x19a\x0F\xA6V[[a\x10%\x84\x82\x85a\x0F\xDCV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x10BWa\x10Aa\r\x16V[[\x815a\x10R\x84\x82` \x86\x01a\x0F\xEBV[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10uWa\x10ta\n\xFEV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x93Wa\x10\x92a\x0B\x03V[[a\x10\x9F\x87\x82\x88\x01a\x0E\x7FV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xC0Wa\x10\xBFa\x0B\x03V[[a\x10\xCC\x87\x82\x88\x01a\x0FxV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xEDWa\x10\xECa\x0B\x03V[[a\x10\xF9\x87\x82\x88\x01a\x0FxV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x1AWa\x11\x19a\x0B\x03V[[a\x11&\x87\x82\x88\x01a\x10-V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81\x90P\x91\x90PV[a\x11E\x81a\x112V[\x81\x14a\x11PW`\0\x80\xFD[PV[`\0\x815\x90Pa\x11b\x81a\x11<V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x11\x83Wa\x11\x82a\r\x16V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xA0Wa\x11\x9Fa\x11hV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x11\xBCWa\x11\xBBa\r\xD3V[[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x11\xDDWa\x11\xDCa\n\xFEV[[`\0a\x11\xEB\x87\x82\x88\x01a\x11SV[\x94PP` a\x11\xFC\x87\x82\x88\x01a\x11SV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x1DWa\x12\x1Ca\x0B\x03V[[a\x12)\x87\x82\x88\x01a\x11mV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x12o\x81a\x0B\x08V[\x82RPPV[a\x12~\x81a\x0BaV[\x82RPPV[`\0`@\x82\x01\x90Pa\x12\x99`\0\x83\x01\x85a\x12fV[a\x12\xA6` \x83\x01\x84a\x12uV[\x93\x92PPPV[`\0\x81Q\x90Pa\x12\xBC\x81a\x0E\xE3V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\xD8Wa\x12\xD7a\n\xFEV[[`\0a\x12\xE6\x84\x82\x85\x01a\x12\xADV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x13\x14a\x13\x0Fa\x13\n\x84a\x0BAV[a\x12\xEFV[a\x0BAV[\x90P\x91\x90PV[`\0a\x13&\x82a\x12\xF9V[\x90P\x91\x90PV[`\0a\x138\x82a\x13\x1BV[\x90P\x91\x90PV[a\x13H\x81a\x13-V[\x82RPPV[`\0a\x13Y\x82a\x13\x1BV[\x90P\x91\x90PV[a\x13i\x81a\x13NV[\x82RPPV[a\x13x\x81a\x0C\x1BV[\x82RPPV[a\x13\x87\x81a\x0CQV[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x13\xA2`\0\x83\x01\x89a\x12fV[a\x13\xAF` \x83\x01\x88a\x12uV[a\x13\xBC`@\x83\x01\x87a\x13?V[a\x13\xC9``\x83\x01\x86a\x13`V[a\x13\xD6`\x80\x83\x01\x85a\x13oV[a\x13\xE3`\xA0\x83\x01\x84a\x13~V[\x97\x96PPPPPPPV[`\0a\x13\xF9\x82a\x13\x1BV[\x90P\x91\x90PV[a\x14\t\x81a\x13\xEEV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x14F\x82a\x13\x1BV[\x90P\x91\x90PV[a\x14V\x81a\x14;V[\x82RPPV[`\0a\x14h\x83\x83a\x14MV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x14\x8C\x82a\x14\x0FV[a\x14\x96\x81\x85a\x14\x1AV[\x93Pa\x14\xA1\x83a\x14+V[\x80`\0[\x83\x81\x10\x15a\x14\xD2W\x81Qa\x14\xB9\x88\x82a\x14\\V[\x97Pa\x14\xC4\x83a\x14tV[\x92PP`\x01\x81\x01\x90Pa\x14\xA5V[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x15\x14\x81a\x0E\xD9V[\x82RPPV[`\0a\x15&\x83\x83a\x15\x0BV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x15J\x82a\x14\xDFV[a\x15T\x81\x85a\x14\xEAV[\x93Pa\x15_\x83a\x14\xFBV[\x80`\0[\x83\x81\x10\x15a\x15\x90W\x81Qa\x15w\x88\x82a\x15\x1AV[\x97Pa\x15\x82\x83a\x152V[\x92PP`\x01\x81\x01\x90Pa\x15cV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x15\xD7W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x15\xBCV[`\0\x84\x84\x01RPPPPV[`\0a\x15\xEE\x82a\x15\x9DV[a\x15\xF8\x81\x85a\x15\xA8V[\x93Pa\x16\x08\x81\x85` \x86\x01a\x15\xB9V[a\x16\x11\x81a\r\x1BV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x161`\0\x83\x01\x87a\x14\0V[\x81\x81\x03` \x83\x01Ra\x16C\x81\x86a\x14\x81V[\x90P\x81\x81\x03`@\x83\x01Ra\x16W\x81\x85a\x15?V[\x90P\x81\x81\x03``\x83\x01Ra\x16k\x81\x84a\x15\xE3V[\x90P\x95\x94PPPPPV[`\0\x81Q\x90Pa\x16\x85\x81a\x0B\x15V[\x92\x91PPV[`\0a\x16\x96\x82a\x0BAV[\x90P\x91\x90PV[a\x16\xA6\x81a\x16\x8BV[\x81\x14a\x16\xB1W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x16\xC3\x81a\x16\x9DV[\x92\x91PPV[`\0\x81Q\x90Pa\x16\xD8\x81a\x0B\xB1V[\x92\x91PPV[`\0\x81Q\x90Pa\x16\xED\x81a\x0B\xEFV[\x92\x91PPV[`\0\x81Q\x90Pa\x17\x02\x81a\x0C%V[\x92\x91PPV[`\0\x81Q\x90Pa\x17\x17\x81a\x0C]V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x17:Wa\x179a\n\xFEV[[`\0a\x17H\x89\x82\x8A\x01a\x16vV[\x96PP` a\x17Y\x89\x82\x8A\x01a\x16\xB4V[\x95PP`@a\x17j\x89\x82\x8A\x01a\x16\xC9V[\x94PP``a\x17{\x89\x82\x8A\x01a\x16\xDEV[\x93PP`\x80a\x17\x8C\x89\x82\x8A\x01a\x16\xF3V[\x92PP`\xA0a\x17\x9D\x89\x82\x8A\x01a\x17\x08V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x01\x90Pa\x17\xBF`\0\x83\x01\x84a\x12uV[\x92\x91PPV[`\0``\x82\x01\x90Pa\x17\xDA`\0\x83\x01\x86a\x12fV[a\x17\xE7` \x83\x01\x85a\x12uV[a\x17\xF4`@\x83\x01\x84a\x12uV[\x94\x93PPPPV[a\x18\x05\x81a\x0E\xD9V[\x82RPPV[`\0`@\x82\x01\x90Pa\x18 `\0\x83\x01\x85a\x12uV[a\x18-` \x83\x01\x84a\x17\xFCV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x18n\x82a\x0E\xD9V[\x91Pa\x18y\x83a\x0E\xD9V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x18\x91Wa\x18\x90a\x184V[[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x18\xADWa\x18\xACa\n\xFEV[[`\0a\x18\xBB\x84\x82\x85\x01a\x0B\xC8V[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FUniswapV3Liquidator: no tokens r`\0\x82\x01R\x7Feceived\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x191`'\x83a\x18\xC4V[\x91Pa\x19<\x82a\x18\xD5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x19`\x81a\x19$V[\x90P\x91\x90PV[`\0a\x19r\x82a\x0BAV[\x91Pa\x19}\x83a\x0BAV[\x92P\x82\x82\x03\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xA9Wa\x19\xA8a\x184V[[\x92\x91PPV[`\0a\x19\xBA\x82a\x0BAV[\x91Pa\x19\xC5\x83a\x0BAV[\x92P\x82\x82\x01\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xF1Wa\x19\xF0a\x184V[[\x92\x91PPV[`\0` \x82\x01\x90Pa\x1A\x0C`\0\x83\x01\x84a\x13?V[\x92\x91PPV[a\x1A\x1B\x81a\x112V[\x82RPPV[a\x1A*\x81a\x0BAV[\x82RPPV[`\0`\xA0\x82\x01\x90Pa\x1AE`\0\x83\x01\x88a\x12uV[a\x1AR` \x83\x01\x87a\x13~V[a\x1A_`@\x83\x01\x86a\x1A\x12V[a\x1Al``\x83\x01\x85a\x1A!V[\x81\x81\x03`\x80\x83\x01Ra\x1A~\x81\x84a\x15\xE3V[\x90P\x96\x95PPPPPPV[`\0\x81Q\x90Pa\x1A\x99\x81a\x11<V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xB6Wa\x1A\xB5a\n\xFEV[[`\0a\x1A\xC4\x85\x82\x86\x01a\x1A\x8AV[\x92PP` a\x1A\xD5\x85\x82\x86\x01a\x1A\x8AV[\x91PP\x92P\x92\x90PV[`\0a\x1A\xEA\x82a\x112V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1B\x1CWa\x1B\x1Ba\x184V[[\x81`\0\x03\x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1B=Wa\x1B<a\n\xFEV[[`\0a\x1BK\x84\x82\x85\x01a\x17\x08V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[`\0a\x1Bj\x82a\x15\x9DV[a\x1Bt\x81\x85a\x1BTV[\x93Pa\x1B\x84\x81\x85` \x86\x01a\x15\xB9V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x1B\x9C\x82\x84a\x1B_V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x1A\xDA\xE7\x92\xBE\x15%]\xFC\xF9hGF$\xE5\xD6C\x1EF+GA\x864\xC3\xA6\x84\x04\x0B\x88\xD7CdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static UNISWAPV3LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniswapV3Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV3Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV3Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV3Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV3Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapV3Liquidator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV3Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNISWAPV3LIQUIDATOR_ABI.clone(),
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
                UNISWAPV3LIQUIDATOR_ABI.clone(),
                UNISWAPV3LIQUIDATOR_BYTECODE.clone().into(),
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
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniswapV3Liquidator<M> {
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
    pub enum UniswapV3LiquidatorErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV3LiquidatorErrors {
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
    impl ::ethers::core::abi::AbiEncode for UniswapV3LiquidatorErrors {
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
    impl ::ethers::contract::ContractRevert for UniswapV3LiquidatorErrors {
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
    impl ::core::fmt::Display for UniswapV3LiquidatorErrors {
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
    impl ::core::convert::From<::std::string::String> for UniswapV3LiquidatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for UniswapV3LiquidatorErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance>
    for UniswapV3LiquidatorErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for UniswapV3LiquidatorErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for UniswapV3LiquidatorErrors {
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
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum UniswapV3LiquidatorCalls {
        Liquidate(LiquidateCall),
        ReceiveFlashLoan(ReceiveFlashLoanCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV3LiquidatorCalls {
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
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapV3LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReceiveFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UniswapV3LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceiveFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<LiquidateCall> for UniswapV3LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<ReceiveFlashLoanCall> for UniswapV3LiquidatorCalls {
        fn from(value: ReceiveFlashLoanCall) -> Self {
            Self::ReceiveFlashLoan(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for UniswapV3LiquidatorCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
}
