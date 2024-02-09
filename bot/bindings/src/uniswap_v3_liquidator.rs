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
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x11W_\x80\xFD[P`@Qb\0 \x148\x03\x80b\0 \x14\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x02\xE9V[\x83\x83\x83\x83\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\xC0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3`\xA0Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x01l\x92\x91\x90b\0\x03\x83V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15b\0\x01\x89W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xAF\x91\x90b\0\x03\xE8V[PPPPPPPPPb\0\x04\x18V[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\x01\xED\x82b\0\x01\xC2V[\x90P\x91\x90PV[_b\0\x02\0\x82b\0\x01\xE1V[\x90P\x91\x90PV[b\0\x02\x12\x81b\0\x01\xF4V[\x81\x14b\0\x02\x1DW_\x80\xFD[PV[_\x81Q\x90Pb\0\x020\x81b\0\x02\x07V[\x92\x91PPV[_b\0\x02B\x82b\0\x01\xE1V[\x90P\x91\x90PV[b\0\x02T\x81b\0\x026V[\x81\x14b\0\x02_W_\x80\xFD[PV[_\x81Q\x90Pb\0\x02r\x81b\0\x02IV[\x92\x91PPV[_b\0\x02\x84\x82b\0\x01\xE1V[\x90P\x91\x90PV[b\0\x02\x96\x81b\0\x02xV[\x81\x14b\0\x02\xA1W_\x80\xFD[PV[_\x81Q\x90Pb\0\x02\xB4\x81b\0\x02\x8BV[\x92\x91PPV[b\0\x02\xC5\x81b\0\x01\xE1V[\x81\x14b\0\x02\xD0W_\x80\xFD[PV[_\x81Q\x90Pb\0\x02\xE3\x81b\0\x02\xBAV[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15b\0\x03\x04Wb\0\x03\x03b\0\x01\xBEV[[_b\0\x03\x13\x87\x82\x88\x01b\0\x02 V[\x94PP` b\0\x03&\x87\x82\x88\x01b\0\x02bV[\x93PP`@b\0\x039\x87\x82\x88\x01b\0\x02\xA4V[\x92PP``b\0\x03L\x87\x82\x88\x01b\0\x02\xD3V[\x91PP\x92\x95\x91\x94P\x92PV[b\0\x03c\x81b\0\x01\xE1V[\x82RPPV[_\x81\x90P\x91\x90PV[b\0\x03}\x81b\0\x03iV[\x82RPPV[_`@\x82\x01\x90Pb\0\x03\x98_\x83\x01\x85b\0\x03XV[b\0\x03\xA7` \x83\x01\x84b\0\x03rV[\x93\x92PPPV[_\x81\x15\x15\x90P\x91\x90PV[b\0\x03\xC4\x81b\0\x03\xAEV[\x81\x14b\0\x03\xCFW_\x80\xFD[PV[_\x81Q\x90Pb\0\x03\xE2\x81b\0\x03\xB9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15b\0\x04\0Wb\0\x03\xFFb\0\x01\xBEV[[_b\0\x04\x0F\x84\x82\x85\x01b\0\x03\xD2V[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x1B\xADb\0\x04g_9_a\x06\x1C\x01R_\x81\x81`\xE6\x01R\x81\x81a\x05\xD3\x01Ra\x06>\x01R_\x81\x81a\x01\xA1\x01Ra\x03\xEE\x01R_\x81\x81a\x03R\x01Ra\x04\x8E\x01Ra\x1B\xAD_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80cl\xB0D)\x14a\0CW\x80c\xF0O'\x07\x14a\0_W\x80c\xFAF\x1E3\x14a\0{W[_\x80\xFD[a\0]`\x04\x806\x03\x81\x01\x90a\0X\x91\x90a\x0C\x92V[a\0\x97V[\0[a\0y`\x04\x806\x03\x81\x01\x90a\0t\x91\x90a\x10IV[a\x03\x0BV[\0[a\0\x95`\x04\x806\x03\x81\x01\x90a\0\x90\x91\x90a\x11\xA9V[a\x06\x93V[\0[_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xB3Wa\0\xB2a\r/V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xE1W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\x01\x18Wa\x01\x17a\x12\x1AV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01nWa\x01ma\r/V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x9CW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xFA\x92\x91\x90a\x12eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x15W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x029\x91\x90a\x12\xA0V[\x81_\x81Q\x81\x10a\x02LWa\x02Ka\x12\x1AV[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\xA6\x96\x95\x94\x93\x92\x91\x90a\x13dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xD4\x94\x93\x92\x91\x90a\x15\xDCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xEBW_\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xFDW=_\x80>=_\xFD[PPPPPPPPPPPPV[_\x83_\x81Q\x81\x10a\x03\x1FWa\x03\x1Ea\x12\x1AV[[` \x02` \x01\x01Q\x90P_\x80_\x80_\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03C\x91\x90a\x16\xD3V[\x95P\x95P\x95P\x95P\x95P\x95P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x880`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xAB\x92\x91\x90a\x12eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEA\x91\x90a\x12\xA0V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04I\x93\x92\x91\x90a\x17\\V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04dW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x88\x91\x90a\x17\x91V[PP_\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x8A0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xE7\x92\x91\x90a\x12eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x02W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05&\x91\x90a\x12\xA0V[a\x050\x91\x90a\x17\xFCV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05m\x92\x91\x90a\x18>V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x84W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x96W=_\x80>=_\xFD[PPPP_a\x05\xA7\x85\x88\x84\x87a\x075V[\x90P_\x8A\x82a\x05\xB6\x91\x90a\x17\xFCV[\x90Pa\x06\x17s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08P\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08P\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[_\x82\x82\x81\x01\x90a\x06\xA3\x91\x90a\x18eV[\x90P_\x80\x86\x13\x15a\x06\xB6W\x85\x90Pa\x07\x02V[_\x85\x13\x15a\x06\xC6W\x84\x90Pa\x07\x01V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xF8\x90a\x19\x10V[`@Q\x80\x91\x03\x90\xFD[[a\x07-3\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08P\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPV[_\x80\x85_\x1C\x90P_\x83\x15\x90P_\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x12\x8A\xCB\x080\x85\x8A\x87a\x07\x8DW`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x07\x88\x91\x90a\x19.V[a\x07\xA0V[`\x01d\x01\0\x02v\xA3a\x07\x9F\x91\x90a\x19uV[[\x8D`@Q` \x01a\x07\xB1\x91\x90a\x19\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xE0\x95\x94\x93\x92\x91\x90a\x19\xF3V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\xFBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1F\x91\x90a\x1A_V[\x91P\x91P\x82a\x087W\x81a\x082\x90a\x1A\x9DV[a\x08BV[\x80a\x08A\x90a\x1A\x9DV[[\x94PPPPP\x94\x93PPPPV[a\x08\xCA\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x08\x83\x92\x91\x90a\x18>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x08\xCFV[PPPV[_a\x08\xF9\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\td\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x81Q\x14\x15\x80\x15a\t\x1DWP\x80\x80` \x01\x90Q\x81\x01\x90a\t\x1B\x91\x90a\x1A\xE3V[\x15[\x15a\t_W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\tV\x91\x90a\x1B\x0EV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\tq\x83\x83_a\tyV[\x90P\x92\x91PPV[``\x81G\x10\x15a\t\xC0W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xB7\x91\x90a\x1B\x0EV[`@Q\x80\x91\x03\x90\xFD[_\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t\xE8\x91\x90a\x1BaV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\n\"W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\n'V[``\x91P[P\x91P\x91Pa\n7\x86\x83\x83a\nBV[\x92PPP\x93\x92PPPV[``\x82a\nWWa\nR\x82a\n\xCFV[a\n\xC7V[_\x82Q\x14\x80\x15a\n}WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\n\xBFW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xB6\x91\x90a\x1B\x0EV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\n\xC8V[[\x93\x92PPPV[_\x81Q\x11\x15a\n\xE1W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0B9\x81a\x0B$V[\x81\x14a\x0BCW_\x80\xFD[PV[_\x815\x90Pa\x0BT\x81a\x0B0V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0B\x83\x82a\x0BZV[\x90P\x91\x90PV[a\x0B\x93\x81a\x0ByV[\x81\x14a\x0B\x9DW_\x80\xFD[PV[_\x815\x90Pa\x0B\xAE\x81a\x0B\x8AV[\x92\x91PPV[_a\x0B\xBE\x82a\x0ByV[\x90P\x91\x90PV[a\x0B\xCE\x81a\x0B\xB4V[\x81\x14a\x0B\xD8W_\x80\xFD[PV[_\x815\x90Pa\x0B\xE9\x81a\x0B\xC5V[\x92\x91PPV[_a\x0B\xF9\x82a\x0ByV[\x90P\x91\x90PV[a\x0C\t\x81a\x0B\xEFV[\x81\x14a\x0C\x13W_\x80\xFD[PV[_\x815\x90Pa\x0C$\x81a\x0C\0V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0C<\x81a\x0C*V[\x81\x14a\x0CFW_\x80\xFD[PV[_\x815\x90Pa\x0CW\x81a\x0C3V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0Cq\x81a\x0C]V[\x81\x14a\x0C{W_\x80\xFD[PV[_\x815\x90Pa\x0C\x8C\x81a\x0ChV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x0C\xACWa\x0C\xABa\x0B\x1CV[[_a\x0C\xB9\x89\x82\x8A\x01a\x0BFV[\x96PP` a\x0C\xCA\x89\x82\x8A\x01a\x0B\xA0V[\x95PP`@a\x0C\xDB\x89\x82\x8A\x01a\x0B\xDBV[\x94PP``a\x0C\xEC\x89\x82\x8A\x01a\x0C\x16V[\x93PP`\x80a\x0C\xFD\x89\x82\x8A\x01a\x0CIV[\x92PP`\xA0a\r\x0E\x89\x82\x8A\x01a\x0C~V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\re\x82a\r\x1FV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\x84Wa\r\x83a\r/V[[\x80`@RPPPV[_a\r\x96a\x0B\x13V[\x90Pa\r\xA2\x82\x82a\r\\V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xC1Wa\r\xC0a\r/V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_a\r\xE0\x82a\x0ByV[\x90P\x91\x90PV[a\r\xF0\x81a\r\xD6V[\x81\x14a\r\xFAW_\x80\xFD[PV[_\x815\x90Pa\x0E\x0B\x81a\r\xE7V[\x92\x91PPV[_a\x0E#a\x0E\x1E\x84a\r\xA7V[a\r\x8DV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0EFWa\x0EEa\r\xD2V[[\x83[\x81\x81\x10\x15a\x0EoW\x80a\x0E[\x88\x82a\r\xFDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0EHV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0E\x8DWa\x0E\x8Ca\r\x1BV[[\x815a\x0E\x9D\x84\x82` \x86\x01a\x0E\x11V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xC0Wa\x0E\xBFa\r/V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x0E\xE3\x81a\x0E\xD1V[\x81\x14a\x0E\xEDW_\x80\xFD[PV[_\x815\x90Pa\x0E\xFE\x81a\x0E\xDAV[\x92\x91PPV[_a\x0F\x16a\x0F\x11\x84a\x0E\xA6V[a\r\x8DV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0F9Wa\x0F8a\r\xD2V[[\x83[\x81\x81\x10\x15a\x0FbW\x80a\x0FN\x88\x82a\x0E\xF0V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0F;V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0F\x80Wa\x0F\x7Fa\r\x1BV[[\x815a\x0F\x90\x84\x82` \x86\x01a\x0F\x04V[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\xB7Wa\x0F\xB6a\r/V[[a\x0F\xC0\x82a\r\x1FV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0F\xEDa\x0F\xE8\x84a\x0F\x9DV[a\r\x8DV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x10\tWa\x10\x08a\x0F\x99V[[a\x10\x14\x84\x82\x85a\x0F\xCDV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x100Wa\x10/a\r\x1BV[[\x815a\x10@\x84\x82` \x86\x01a\x0F\xDBV[\x91PP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x10aWa\x10`a\x0B\x1CV[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10~Wa\x10}a\x0B V[[a\x10\x8A\x87\x82\x88\x01a\x0EyV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xABWa\x10\xAAa\x0B V[[a\x10\xB7\x87\x82\x88\x01a\x0FlV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xD8Wa\x10\xD7a\x0B V[[a\x10\xE4\x87\x82\x88\x01a\x0FlV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x05Wa\x11\x04a\x0B V[[a\x11\x11\x87\x82\x88\x01a\x10\x1CV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81\x90P\x91\x90PV[a\x11/\x81a\x11\x1DV[\x81\x14a\x119W_\x80\xFD[PV[_\x815\x90Pa\x11J\x81a\x11&V[\x92\x91PPV[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a\x11iWa\x11ha\r\x1BV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x86Wa\x11\x85a\x11PV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x11\xA2Wa\x11\xA1a\r\xD2V[[\x92P\x92\x90PV[_\x80_\x80``\x85\x87\x03\x12\x15a\x11\xC1Wa\x11\xC0a\x0B\x1CV[[_a\x11\xCE\x87\x82\x88\x01a\x11<V[\x94PP` a\x11\xDF\x87\x82\x88\x01a\x11<V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\0Wa\x11\xFFa\x0B V[[a\x12\x0C\x87\x82\x88\x01a\x11TV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x12P\x81a\x0B$V[\x82RPPV[a\x12_\x81a\x0ByV[\x82RPPV[_`@\x82\x01\x90Pa\x12x_\x83\x01\x85a\x12GV[a\x12\x85` \x83\x01\x84a\x12VV[\x93\x92PPPV[_\x81Q\x90Pa\x12\x9A\x81a\x0E\xDAV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12\xB5Wa\x12\xB4a\x0B\x1CV[[_a\x12\xC2\x84\x82\x85\x01a\x12\x8CV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x12\xEEa\x12\xE9a\x12\xE4\x84a\x0BZV[a\x12\xCBV[a\x0BZV[\x90P\x91\x90PV[_a\x12\xFF\x82a\x12\xD4V[\x90P\x91\x90PV[_a\x13\x10\x82a\x12\xF5V[\x90P\x91\x90PV[a\x13 \x81a\x13\x06V[\x82RPPV[_a\x130\x82a\x12\xF5V[\x90P\x91\x90PV[a\x13@\x81a\x13&V[\x82RPPV[a\x13O\x81a\x0C*V[\x82RPPV[a\x13^\x81a\x0C]V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x13w_\x83\x01\x89a\x12GV[a\x13\x84` \x83\x01\x88a\x12VV[a\x13\x91`@\x83\x01\x87a\x13\x17V[a\x13\x9E``\x83\x01\x86a\x137V[a\x13\xAB`\x80\x83\x01\x85a\x13FV[a\x13\xB8`\xA0\x83\x01\x84a\x13UV[\x97\x96PPPPPPPV[_a\x13\xCD\x82a\x12\xF5V[\x90P\x91\x90PV[a\x13\xDD\x81a\x13\xC3V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\x14\x16\x82a\x12\xF5V[\x90P\x91\x90PV[a\x14&\x81a\x14\x0CV[\x82RPPV[_a\x147\x83\x83a\x14\x1DV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x14Y\x82a\x13\xE3V[a\x14c\x81\x85a\x13\xEDV[\x93Pa\x14n\x83a\x13\xFDV[\x80_[\x83\x81\x10\x15a\x14\x9EW\x81Qa\x14\x85\x88\x82a\x14,V[\x97Pa\x14\x90\x83a\x14CV[\x92PP`\x01\x81\x01\x90Pa\x14qV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x14\xDD\x81a\x0E\xD1V[\x82RPPV[_a\x14\xEE\x83\x83a\x14\xD4V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x15\x10\x82a\x14\xABV[a\x15\x1A\x81\x85a\x14\xB5V[\x93Pa\x15%\x83a\x14\xC5V[\x80_[\x83\x81\x10\x15a\x15UW\x81Qa\x15<\x88\x82a\x14\xE3V[\x97Pa\x15G\x83a\x14\xFAV[\x92PP`\x01\x81\x01\x90Pa\x15(V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x15\x99W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x15~V[_\x84\x84\x01RPPPPV[_a\x15\xAE\x82a\x15bV[a\x15\xB8\x81\x85a\x15lV[\x93Pa\x15\xC8\x81\x85` \x86\x01a\x15|V[a\x15\xD1\x81a\r\x1FV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa\x15\xEF_\x83\x01\x87a\x13\xD4V[\x81\x81\x03` \x83\x01Ra\x16\x01\x81\x86a\x14OV[\x90P\x81\x81\x03`@\x83\x01Ra\x16\x15\x81\x85a\x15\x06V[\x90P\x81\x81\x03``\x83\x01Ra\x16)\x81\x84a\x15\xA4V[\x90P\x95\x94PPPPPV[_\x81Q\x90Pa\x16B\x81a\x0B0V[\x92\x91PPV[_a\x16R\x82a\x0BZV[\x90P\x91\x90PV[a\x16b\x81a\x16HV[\x81\x14a\x16lW_\x80\xFD[PV[_\x81Q\x90Pa\x16}\x81a\x16YV[\x92\x91PPV[_\x81Q\x90Pa\x16\x91\x81a\x0B\xC5V[\x92\x91PPV[_\x81Q\x90Pa\x16\xA5\x81a\x0C\0V[\x92\x91PPV[_\x81Q\x90Pa\x16\xB9\x81a\x0C3V[\x92\x91PPV[_\x81Q\x90Pa\x16\xCD\x81a\x0ChV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x16\xEDWa\x16\xECa\x0B\x1CV[[_a\x16\xFA\x89\x82\x8A\x01a\x164V[\x96PP` a\x17\x0B\x89\x82\x8A\x01a\x16oV[\x95PP`@a\x17\x1C\x89\x82\x8A\x01a\x16\x83V[\x94PP``a\x17-\x89\x82\x8A\x01a\x16\x97V[\x93PP`\x80a\x17>\x89\x82\x8A\x01a\x16\xABV[\x92PP`\xA0a\x17O\x89\x82\x8A\x01a\x16\xBFV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_``\x82\x01\x90Pa\x17o_\x83\x01\x86a\x12GV[a\x17|` \x83\x01\x85a\x12VV[a\x17\x89`@\x83\x01\x84a\x12VV[\x94\x93PPPPV[_\x80`@\x83\x85\x03\x12\x15a\x17\xA7Wa\x17\xA6a\x0B\x1CV[[_a\x17\xB4\x85\x82\x86\x01a\x12\x8CV[\x92PP` a\x17\xC5\x85\x82\x86\x01a\x12\x8CV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x18\x06\x82a\x0E\xD1V[\x91Pa\x18\x11\x83a\x0E\xD1V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x18)Wa\x18(a\x17\xCFV[[\x92\x91PPV[a\x188\x81a\x0E\xD1V[\x82RPPV[_`@\x82\x01\x90Pa\x18Q_\x83\x01\x85a\x12VV[a\x18^` \x83\x01\x84a\x18/V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x18zWa\x18ya\x0B\x1CV[[_a\x18\x87\x84\x82\x85\x01a\x0B\xDBV[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FUniswapV3Liquidator: no tokens r_\x82\x01R\x7Feceived\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x18\xFA`'\x83a\x18\x90V[\x91Pa\x19\x05\x82a\x18\xA0V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19'\x81a\x18\xEEV[\x90P\x91\x90PV[_a\x198\x82a\x0BZV[\x91Pa\x19C\x83a\x0BZV[\x92P\x82\x82\x03\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19oWa\x19na\x17\xCFV[[\x92\x91PPV[_a\x19\x7F\x82a\x0BZV[\x91Pa\x19\x8A\x83a\x0BZV[\x92P\x82\x82\x01\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xB6Wa\x19\xB5a\x17\xCFV[[\x92\x91PPV[_` \x82\x01\x90Pa\x19\xCF_\x83\x01\x84a\x13\x17V[\x92\x91PPV[a\x19\xDE\x81a\x11\x1DV[\x82RPPV[a\x19\xED\x81a\x0BZV[\x82RPPV[_`\xA0\x82\x01\x90Pa\x1A\x06_\x83\x01\x88a\x12VV[a\x1A\x13` \x83\x01\x87a\x13UV[a\x1A `@\x83\x01\x86a\x19\xD5V[a\x1A-``\x83\x01\x85a\x19\xE4V[\x81\x81\x03`\x80\x83\x01Ra\x1A?\x81\x84a\x15\xA4V[\x90P\x96\x95PPPPPPV[_\x81Q\x90Pa\x1AY\x81a\x11&V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x1AuWa\x1Ata\x0B\x1CV[[_a\x1A\x82\x85\x82\x86\x01a\x1AKV[\x92PP` a\x1A\x93\x85\x82\x86\x01a\x1AKV[\x91PP\x92P\x92\x90PV[_a\x1A\xA7\x82a\x11\x1DV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1A\xD9Wa\x1A\xD8a\x17\xCFV[[\x81_\x03\x90P\x91\x90PV[_` \x82\x84\x03\x12\x15a\x1A\xF8Wa\x1A\xF7a\x0B\x1CV[[_a\x1B\x05\x84\x82\x85\x01a\x16\xBFV[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa\x1B!_\x83\x01\x84a\x12VV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a\x1B;\x82a\x15bV[a\x1BE\x81\x85a\x1B'V[\x93Pa\x1BU\x81\x85` \x86\x01a\x15|V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x1Bl\x82\x84a\x1B1V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE7\xB5\x15\xF8J\xC8\xB4\x19\x17\x14\x8A\xBC5\x92\xA8q}\x8EqU\xE7\xBF\xBB\x88\xDE\xE1\xA2`*/v(dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static UNISWAPV3LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80cl\xB0D)\x14a\0CW\x80c\xF0O'\x07\x14a\0_W\x80c\xFAF\x1E3\x14a\0{W[_\x80\xFD[a\0]`\x04\x806\x03\x81\x01\x90a\0X\x91\x90a\x0C\x92V[a\0\x97V[\0[a\0y`\x04\x806\x03\x81\x01\x90a\0t\x91\x90a\x10IV[a\x03\x0BV[\0[a\0\x95`\x04\x806\x03\x81\x01\x90a\0\x90\x91\x90a\x11\xA9V[a\x06\x93V[\0[_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xB3Wa\0\xB2a\r/V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xE1W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81_\x81Q\x81\x10a\x01\x18Wa\x01\x17a\x12\x1AV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01nWa\x01ma\r/V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x9CW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xFA\x92\x91\x90a\x12eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x15W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x029\x91\x90a\x12\xA0V[\x81_\x81Q\x81\x10a\x02LWa\x02Ka\x12\x1AV[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\xA6\x96\x95\x94\x93\x92\x91\x90a\x13dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xD4\x94\x93\x92\x91\x90a\x15\xDCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xEBW_\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xFDW=_\x80>=_\xFD[PPPPPPPPPPPPV[_\x83_\x81Q\x81\x10a\x03\x1FWa\x03\x1Ea\x12\x1AV[[` \x02` \x01\x01Q\x90P_\x80_\x80_\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03C\x91\x90a\x16\xD3V[\x95P\x95P\x95P\x95P\x95P\x95P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x880`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xAB\x92\x91\x90a\x12eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEA\x91\x90a\x12\xA0V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04I\x93\x92\x91\x90a\x17\\V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04dW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x88\x91\x90a\x17\x91V[PP_\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xD8h\r\x8A0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xE7\x92\x91\x90a\x12eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x02W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05&\x91\x90a\x12\xA0V[a\x050\x91\x90a\x17\xFCV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05m\x92\x91\x90a\x18>V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x84W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x96W=_\x80>=_\xFD[PPPP_a\x05\xA7\x85\x88\x84\x87a\x075V[\x90P_\x8A\x82a\x05\xB6\x91\x90a\x17\xFCV[\x90Pa\x06\x17s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08P\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08P\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[_\x82\x82\x81\x01\x90a\x06\xA3\x91\x90a\x18eV[\x90P_\x80\x86\x13\x15a\x06\xB6W\x85\x90Pa\x07\x02V[_\x85\x13\x15a\x06\xC6W\x84\x90Pa\x07\x01V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xF8\x90a\x19\x10V[`@Q\x80\x91\x03\x90\xFD[[a\x07-3\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08P\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPV[_\x80\x85_\x1C\x90P_\x83\x15\x90P_\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x12\x8A\xCB\x080\x85\x8A\x87a\x07\x8DW`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x07\x88\x91\x90a\x19.V[a\x07\xA0V[`\x01d\x01\0\x02v\xA3a\x07\x9F\x91\x90a\x19uV[[\x8D`@Q` \x01a\x07\xB1\x91\x90a\x19\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xE0\x95\x94\x93\x92\x91\x90a\x19\xF3V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\xFBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1F\x91\x90a\x1A_V[\x91P\x91P\x82a\x087W\x81a\x082\x90a\x1A\x9DV[a\x08BV[\x80a\x08A\x90a\x1A\x9DV[[\x94PPPPP\x94\x93PPPPV[a\x08\xCA\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x08\x83\x92\x91\x90a\x18>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x08\xCFV[PPPV[_a\x08\xF9\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\td\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x81Q\x14\x15\x80\x15a\t\x1DWP\x80\x80` \x01\x90Q\x81\x01\x90a\t\x1B\x91\x90a\x1A\xE3V[\x15[\x15a\t_W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\tV\x91\x90a\x1B\x0EV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\tq\x83\x83_a\tyV[\x90P\x92\x91PPV[``\x81G\x10\x15a\t\xC0W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xB7\x91\x90a\x1B\x0EV[`@Q\x80\x91\x03\x90\xFD[_\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t\xE8\x91\x90a\x1BaV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\n\"W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\n'V[``\x91P[P\x91P\x91Pa\n7\x86\x83\x83a\nBV[\x92PPP\x93\x92PPPV[``\x82a\nWWa\nR\x82a\n\xCFV[a\n\xC7V[_\x82Q\x14\x80\x15a\n}WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\n\xBFW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xB6\x91\x90a\x1B\x0EV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\n\xC8V[[\x93\x92PPPV[_\x81Q\x11\x15a\n\xE1W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0B9\x81a\x0B$V[\x81\x14a\x0BCW_\x80\xFD[PV[_\x815\x90Pa\x0BT\x81a\x0B0V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0B\x83\x82a\x0BZV[\x90P\x91\x90PV[a\x0B\x93\x81a\x0ByV[\x81\x14a\x0B\x9DW_\x80\xFD[PV[_\x815\x90Pa\x0B\xAE\x81a\x0B\x8AV[\x92\x91PPV[_a\x0B\xBE\x82a\x0ByV[\x90P\x91\x90PV[a\x0B\xCE\x81a\x0B\xB4V[\x81\x14a\x0B\xD8W_\x80\xFD[PV[_\x815\x90Pa\x0B\xE9\x81a\x0B\xC5V[\x92\x91PPV[_a\x0B\xF9\x82a\x0ByV[\x90P\x91\x90PV[a\x0C\t\x81a\x0B\xEFV[\x81\x14a\x0C\x13W_\x80\xFD[PV[_\x815\x90Pa\x0C$\x81a\x0C\0V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0C<\x81a\x0C*V[\x81\x14a\x0CFW_\x80\xFD[PV[_\x815\x90Pa\x0CW\x81a\x0C3V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0Cq\x81a\x0C]V[\x81\x14a\x0C{W_\x80\xFD[PV[_\x815\x90Pa\x0C\x8C\x81a\x0ChV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x0C\xACWa\x0C\xABa\x0B\x1CV[[_a\x0C\xB9\x89\x82\x8A\x01a\x0BFV[\x96PP` a\x0C\xCA\x89\x82\x8A\x01a\x0B\xA0V[\x95PP`@a\x0C\xDB\x89\x82\x8A\x01a\x0B\xDBV[\x94PP``a\x0C\xEC\x89\x82\x8A\x01a\x0C\x16V[\x93PP`\x80a\x0C\xFD\x89\x82\x8A\x01a\x0CIV[\x92PP`\xA0a\r\x0E\x89\x82\x8A\x01a\x0C~V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\re\x82a\r\x1FV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\x84Wa\r\x83a\r/V[[\x80`@RPPPV[_a\r\x96a\x0B\x13V[\x90Pa\r\xA2\x82\x82a\r\\V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xC1Wa\r\xC0a\r/V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_a\r\xE0\x82a\x0ByV[\x90P\x91\x90PV[a\r\xF0\x81a\r\xD6V[\x81\x14a\r\xFAW_\x80\xFD[PV[_\x815\x90Pa\x0E\x0B\x81a\r\xE7V[\x92\x91PPV[_a\x0E#a\x0E\x1E\x84a\r\xA7V[a\r\x8DV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0EFWa\x0EEa\r\xD2V[[\x83[\x81\x81\x10\x15a\x0EoW\x80a\x0E[\x88\x82a\r\xFDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0EHV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0E\x8DWa\x0E\x8Ca\r\x1BV[[\x815a\x0E\x9D\x84\x82` \x86\x01a\x0E\x11V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xC0Wa\x0E\xBFa\r/V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x0E\xE3\x81a\x0E\xD1V[\x81\x14a\x0E\xEDW_\x80\xFD[PV[_\x815\x90Pa\x0E\xFE\x81a\x0E\xDAV[\x92\x91PPV[_a\x0F\x16a\x0F\x11\x84a\x0E\xA6V[a\r\x8DV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0F9Wa\x0F8a\r\xD2V[[\x83[\x81\x81\x10\x15a\x0FbW\x80a\x0FN\x88\x82a\x0E\xF0V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0F;V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0F\x80Wa\x0F\x7Fa\r\x1BV[[\x815a\x0F\x90\x84\x82` \x86\x01a\x0F\x04V[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\xB7Wa\x0F\xB6a\r/V[[a\x0F\xC0\x82a\r\x1FV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0F\xEDa\x0F\xE8\x84a\x0F\x9DV[a\r\x8DV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x10\tWa\x10\x08a\x0F\x99V[[a\x10\x14\x84\x82\x85a\x0F\xCDV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x100Wa\x10/a\r\x1BV[[\x815a\x10@\x84\x82` \x86\x01a\x0F\xDBV[\x91PP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x10aWa\x10`a\x0B\x1CV[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10~Wa\x10}a\x0B V[[a\x10\x8A\x87\x82\x88\x01a\x0EyV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xABWa\x10\xAAa\x0B V[[a\x10\xB7\x87\x82\x88\x01a\x0FlV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xD8Wa\x10\xD7a\x0B V[[a\x10\xE4\x87\x82\x88\x01a\x0FlV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x05Wa\x11\x04a\x0B V[[a\x11\x11\x87\x82\x88\x01a\x10\x1CV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81\x90P\x91\x90PV[a\x11/\x81a\x11\x1DV[\x81\x14a\x119W_\x80\xFD[PV[_\x815\x90Pa\x11J\x81a\x11&V[\x92\x91PPV[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a\x11iWa\x11ha\r\x1BV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x86Wa\x11\x85a\x11PV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x11\xA2Wa\x11\xA1a\r\xD2V[[\x92P\x92\x90PV[_\x80_\x80``\x85\x87\x03\x12\x15a\x11\xC1Wa\x11\xC0a\x0B\x1CV[[_a\x11\xCE\x87\x82\x88\x01a\x11<V[\x94PP` a\x11\xDF\x87\x82\x88\x01a\x11<V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\0Wa\x11\xFFa\x0B V[[a\x12\x0C\x87\x82\x88\x01a\x11TV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x12P\x81a\x0B$V[\x82RPPV[a\x12_\x81a\x0ByV[\x82RPPV[_`@\x82\x01\x90Pa\x12x_\x83\x01\x85a\x12GV[a\x12\x85` \x83\x01\x84a\x12VV[\x93\x92PPPV[_\x81Q\x90Pa\x12\x9A\x81a\x0E\xDAV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12\xB5Wa\x12\xB4a\x0B\x1CV[[_a\x12\xC2\x84\x82\x85\x01a\x12\x8CV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x12\xEEa\x12\xE9a\x12\xE4\x84a\x0BZV[a\x12\xCBV[a\x0BZV[\x90P\x91\x90PV[_a\x12\xFF\x82a\x12\xD4V[\x90P\x91\x90PV[_a\x13\x10\x82a\x12\xF5V[\x90P\x91\x90PV[a\x13 \x81a\x13\x06V[\x82RPPV[_a\x130\x82a\x12\xF5V[\x90P\x91\x90PV[a\x13@\x81a\x13&V[\x82RPPV[a\x13O\x81a\x0C*V[\x82RPPV[a\x13^\x81a\x0C]V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x13w_\x83\x01\x89a\x12GV[a\x13\x84` \x83\x01\x88a\x12VV[a\x13\x91`@\x83\x01\x87a\x13\x17V[a\x13\x9E``\x83\x01\x86a\x137V[a\x13\xAB`\x80\x83\x01\x85a\x13FV[a\x13\xB8`\xA0\x83\x01\x84a\x13UV[\x97\x96PPPPPPPV[_a\x13\xCD\x82a\x12\xF5V[\x90P\x91\x90PV[a\x13\xDD\x81a\x13\xC3V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\x14\x16\x82a\x12\xF5V[\x90P\x91\x90PV[a\x14&\x81a\x14\x0CV[\x82RPPV[_a\x147\x83\x83a\x14\x1DV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x14Y\x82a\x13\xE3V[a\x14c\x81\x85a\x13\xEDV[\x93Pa\x14n\x83a\x13\xFDV[\x80_[\x83\x81\x10\x15a\x14\x9EW\x81Qa\x14\x85\x88\x82a\x14,V[\x97Pa\x14\x90\x83a\x14CV[\x92PP`\x01\x81\x01\x90Pa\x14qV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x14\xDD\x81a\x0E\xD1V[\x82RPPV[_a\x14\xEE\x83\x83a\x14\xD4V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x15\x10\x82a\x14\xABV[a\x15\x1A\x81\x85a\x14\xB5V[\x93Pa\x15%\x83a\x14\xC5V[\x80_[\x83\x81\x10\x15a\x15UW\x81Qa\x15<\x88\x82a\x14\xE3V[\x97Pa\x15G\x83a\x14\xFAV[\x92PP`\x01\x81\x01\x90Pa\x15(V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x15\x99W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x15~V[_\x84\x84\x01RPPPPV[_a\x15\xAE\x82a\x15bV[a\x15\xB8\x81\x85a\x15lV[\x93Pa\x15\xC8\x81\x85` \x86\x01a\x15|V[a\x15\xD1\x81a\r\x1FV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa\x15\xEF_\x83\x01\x87a\x13\xD4V[\x81\x81\x03` \x83\x01Ra\x16\x01\x81\x86a\x14OV[\x90P\x81\x81\x03`@\x83\x01Ra\x16\x15\x81\x85a\x15\x06V[\x90P\x81\x81\x03``\x83\x01Ra\x16)\x81\x84a\x15\xA4V[\x90P\x95\x94PPPPPV[_\x81Q\x90Pa\x16B\x81a\x0B0V[\x92\x91PPV[_a\x16R\x82a\x0BZV[\x90P\x91\x90PV[a\x16b\x81a\x16HV[\x81\x14a\x16lW_\x80\xFD[PV[_\x81Q\x90Pa\x16}\x81a\x16YV[\x92\x91PPV[_\x81Q\x90Pa\x16\x91\x81a\x0B\xC5V[\x92\x91PPV[_\x81Q\x90Pa\x16\xA5\x81a\x0C\0V[\x92\x91PPV[_\x81Q\x90Pa\x16\xB9\x81a\x0C3V[\x92\x91PPV[_\x81Q\x90Pa\x16\xCD\x81a\x0ChV[\x92\x91PPV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x16\xEDWa\x16\xECa\x0B\x1CV[[_a\x16\xFA\x89\x82\x8A\x01a\x164V[\x96PP` a\x17\x0B\x89\x82\x8A\x01a\x16oV[\x95PP`@a\x17\x1C\x89\x82\x8A\x01a\x16\x83V[\x94PP``a\x17-\x89\x82\x8A\x01a\x16\x97V[\x93PP`\x80a\x17>\x89\x82\x8A\x01a\x16\xABV[\x92PP`\xA0a\x17O\x89\x82\x8A\x01a\x16\xBFV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_``\x82\x01\x90Pa\x17o_\x83\x01\x86a\x12GV[a\x17|` \x83\x01\x85a\x12VV[a\x17\x89`@\x83\x01\x84a\x12VV[\x94\x93PPPPV[_\x80`@\x83\x85\x03\x12\x15a\x17\xA7Wa\x17\xA6a\x0B\x1CV[[_a\x17\xB4\x85\x82\x86\x01a\x12\x8CV[\x92PP` a\x17\xC5\x85\x82\x86\x01a\x12\x8CV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x18\x06\x82a\x0E\xD1V[\x91Pa\x18\x11\x83a\x0E\xD1V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x18)Wa\x18(a\x17\xCFV[[\x92\x91PPV[a\x188\x81a\x0E\xD1V[\x82RPPV[_`@\x82\x01\x90Pa\x18Q_\x83\x01\x85a\x12VV[a\x18^` \x83\x01\x84a\x18/V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x18zWa\x18ya\x0B\x1CV[[_a\x18\x87\x84\x82\x85\x01a\x0B\xDBV[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FUniswapV3Liquidator: no tokens r_\x82\x01R\x7Feceived\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x18\xFA`'\x83a\x18\x90V[\x91Pa\x19\x05\x82a\x18\xA0V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19'\x81a\x18\xEEV[\x90P\x91\x90PV[_a\x198\x82a\x0BZV[\x91Pa\x19C\x83a\x0BZV[\x92P\x82\x82\x03\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19oWa\x19na\x17\xCFV[[\x92\x91PPV[_a\x19\x7F\x82a\x0BZV[\x91Pa\x19\x8A\x83a\x0BZV[\x92P\x82\x82\x01\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xB6Wa\x19\xB5a\x17\xCFV[[\x92\x91PPV[_` \x82\x01\x90Pa\x19\xCF_\x83\x01\x84a\x13\x17V[\x92\x91PPV[a\x19\xDE\x81a\x11\x1DV[\x82RPPV[a\x19\xED\x81a\x0BZV[\x82RPPV[_`\xA0\x82\x01\x90Pa\x1A\x06_\x83\x01\x88a\x12VV[a\x1A\x13` \x83\x01\x87a\x13UV[a\x1A `@\x83\x01\x86a\x19\xD5V[a\x1A-``\x83\x01\x85a\x19\xE4V[\x81\x81\x03`\x80\x83\x01Ra\x1A?\x81\x84a\x15\xA4V[\x90P\x96\x95PPPPPPV[_\x81Q\x90Pa\x1AY\x81a\x11&V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x1AuWa\x1Ata\x0B\x1CV[[_a\x1A\x82\x85\x82\x86\x01a\x1AKV[\x92PP` a\x1A\x93\x85\x82\x86\x01a\x1AKV[\x91PP\x92P\x92\x90PV[_a\x1A\xA7\x82a\x11\x1DV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x1A\xD9Wa\x1A\xD8a\x17\xCFV[[\x81_\x03\x90P\x91\x90PV[_` \x82\x84\x03\x12\x15a\x1A\xF8Wa\x1A\xF7a\x0B\x1CV[[_a\x1B\x05\x84\x82\x85\x01a\x16\xBFV[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa\x1B!_\x83\x01\x84a\x12VV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a\x1B;\x82a\x15bV[a\x1BE\x81\x85a\x1B'V[\x93Pa\x1BU\x81\x85` \x86\x01a\x15|V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x1Bl\x82\x84a\x1B1V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE7\xB5\x15\xF8J\xC8\xB4\x19\x17\x14\x8A\xBC5\x92\xA8q}\x8EqU\xE7\xBF\xBB\x88\xDE\xE1\xA2`*/v(dsolcC\0\x08\x15\x003";
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
