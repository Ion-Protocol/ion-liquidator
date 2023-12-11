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
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1C\xAE8\x03\x80b\0\x1C\xAE\x839\x81\x81\x01`@R\x81\x01\x90b\0\08\x91\x90b\0\x02\xFBV[\x83\x83\x83\x83\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\xC0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3`\xA0Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x01m\x92\x91\x90b\0\x03\x99V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x01\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xB3\x91\x90b\0\x04\x03V[PPPPPPPPPb\0\x045V[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x01\xF4\x82b\0\x01\xC7V[\x90P\x91\x90PV[`\0b\0\x02\x08\x82b\0\x01\xE7V[\x90P\x91\x90PV[b\0\x02\x1A\x81b\0\x01\xFBV[\x81\x14b\0\x02&W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02:\x81b\0\x02\x0FV[\x92\x91PPV[`\0b\0\x02M\x82b\0\x01\xE7V[\x90P\x91\x90PV[b\0\x02_\x81b\0\x02@V[\x81\x14b\0\x02kW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\x7F\x81b\0\x02TV[\x92\x91PPV[`\0b\0\x02\x92\x82b\0\x01\xE7V[\x90P\x91\x90PV[b\0\x02\xA4\x81b\0\x02\x85V[\x81\x14b\0\x02\xB0W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\xC4\x81b\0\x02\x99V[\x92\x91PPV[b\0\x02\xD5\x81b\0\x01\xE7V[\x81\x14b\0\x02\xE1W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\xF5\x81b\0\x02\xCAV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x03\x18Wb\0\x03\x17b\0\x01\xC2V[[`\0b\0\x03(\x87\x82\x88\x01b\0\x02)V[\x94PP` b\0\x03;\x87\x82\x88\x01b\0\x02nV[\x93PP`@b\0\x03N\x87\x82\x88\x01b\0\x02\xB3V[\x92PP``b\0\x03a\x87\x82\x88\x01b\0\x02\xE4V[\x91PP\x92\x95\x91\x94P\x92PV[b\0\x03x\x81b\0\x01\xE7V[\x82RPPV[`\0\x81\x90P\x91\x90PV[b\0\x03\x93\x81b\0\x03~V[\x82RPPV[`\0`@\x82\x01\x90Pb\0\x03\xB0`\0\x83\x01\x85b\0\x03mV[b\0\x03\xBF` \x83\x01\x84b\0\x03\x88V[\x93\x92PPPV[`\0\x81\x15\x15\x90P\x91\x90PV[b\0\x03\xDD\x81b\0\x03\xC6V[\x81\x14b\0\x03\xE9W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\xFD\x81b\0\x03\xD2V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x04\x1CWb\0\x04\x1Bb\0\x01\xC2V[[`\0b\0\x04,\x84\x82\x85\x01b\0\x03\xECV[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x18.b\0\x04\x80`\09`\0a\x05\xC2\x01R`\0\x81\x81`\xC3\x01R\x81\x81a\x05y\x01Ra\x05\xE4\x01R`\0\x81\x81a\x01\x80\x01Ra\x03\xBB\x01R`\0PPa\x18.`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cl\xB0D)\x14a\0;W\x80c\xF0O'\x07\x14a\0WW[`\0\x80\xFD[a\0U`\x04\x806\x03\x81\x01\x90a\0P\x91\x90a\x0B\xD3V[a\0sV[\0[a\0q`\x04\x806\x03\x81\x01\x90a\0l\x91\x90a\x0F\xA5V[a\x02\xF2V[\0[`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x90Wa\0\x8Fa\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xBEW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\0\xF6Wa\0\xF5a\x10|V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01MWa\x01La\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01{W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xD9\x92\x91\x90a\x10\xC9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x1A\x91\x90a\x11\x07V[\x81`\0\x81Q\x81\x10a\x02.Wa\x02-a\x10|V[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\x88\x96\x95\x94\x93\x92\x91\x90a\x11\xD2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xB6\x94\x93\x92\x91\x90a\x14aV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xE4W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x83`\0\x81Q\x81\x10a\x03\x08Wa\x03\x07a\x10|V[[` \x02` \x01\x01Q\x90P`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03/\x91\x90a\x15bV[\x95P\x95P\x95P\x95P\x95P\x95P`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03v\x91\x90a\x15\xEFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xB7\x91\x90a\x11\x07V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x16\x93\x92\x91\x90a\x16\nV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x040W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04DW=`\0\x80>=`\0\xFD[PPPP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x83\x92\x91\x90a\x16PV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x9DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xB1W=`\0\x80>=`\0\xFD[PPPP`\0\x81\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xF1\x91\x90a\x15\xEFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x052\x91\x90a\x11\x07V[a\x05<\x91\x90a\x16\xA8V[\x90P`\0a\x05L\x85\x88\x84\x87a\x069V[\x90P`\0\x8A\x82a\x05\\\x91\x90a\x16\xA8V[\x90Pa\x05\xBDs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07q\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06(\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07q\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[`\0\x80\x85`\0\x1C\x90P`\0\x81\x90P`\0\x80\x85a\x06XW`\0`\x01a\x06]V[`\x01`\0[\x91P\x91P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x85\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\x9C\x92\x91\x90a\x16PV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xDF\x91\x90a\x16\xDCV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\xF0!$\x83\x83\x8A`\0`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07 \x94\x93\x92\x91\x90a\x17`V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07c\x91\x90a\x11\x07V[\x94PPPPP\x94\x93PPPPV[a\x07\xEB\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x07\xA4\x92\x91\x90a\x16PV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x07\xF0V[PPPV[`\0a\x08\x1B\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\x87\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\x08@WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08>\x91\x90a\x16\xDCV[\x15[\x15a\x08\x82W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08y\x91\x90a\x15\xEFV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x08\x95\x83\x83`\0a\x08\x9DV[\x90P\x92\x91PPV[``\x81G\x10\x15a\x08\xE4W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xDB\x91\x90a\x15\xEFV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t\r\x91\x90a\x17\xE1V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\tJW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tOV[``\x91P[P\x91P\x91Pa\t_\x86\x83\x83a\tjV[\x92PPP\x93\x92PPPV[``\x82a\t\x7FWa\tz\x82a\t\xF9V[a\t\xF1V[`\0\x82Q\x14\x80\x15a\t\xA7WP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\t\xE9W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xE0\x91\x90a\x15\xEFV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\t\xF2V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\n\x0CW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\nh\x81a\nRV[\x81\x14a\nsW`\0\x80\xFD[PV[`\0\x815\x90Pa\n\x85\x81a\n_V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\n\xB6\x82a\n\x8BV[\x90P\x91\x90PV[a\n\xC6\x81a\n\xABV[\x81\x14a\n\xD1W`\0\x80\xFD[PV[`\0\x815\x90Pa\n\xE3\x81a\n\xBDV[\x92\x91PPV[`\0a\n\xF4\x82a\n\xABV[\x90P\x91\x90PV[a\x0B\x04\x81a\n\xE9V[\x81\x14a\x0B\x0FW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B!\x81a\n\xFBV[\x92\x91PPV[`\0a\x0B2\x82a\n\xABV[\x90P\x91\x90PV[a\x0BB\x81a\x0B'V[\x81\x14a\x0BMW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B_\x81a\x0B9V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0Bx\x81a\x0BeV[\x81\x14a\x0B\x83W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\x95\x81a\x0BoV[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0B\xB0\x81a\x0B\x9BV[\x81\x14a\x0B\xBBW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\xCD\x81a\x0B\xA7V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0B\xF0Wa\x0B\xEFa\nHV[[`\0a\x0B\xFE\x89\x82\x8A\x01a\nvV[\x96PP` a\x0C\x0F\x89\x82\x8A\x01a\n\xD4V[\x95PP`@a\x0C \x89\x82\x8A\x01a\x0B\x12V[\x94PP``a\x0C1\x89\x82\x8A\x01a\x0BPV[\x93PP`\x80a\x0CB\x89\x82\x8A\x01a\x0B\x86V[\x92PP`\xA0a\x0CS\x89\x82\x8A\x01a\x0B\xBEV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x0C\xAE\x82a\x0CeV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\xCDWa\x0C\xCCa\x0CvV[[\x80`@RPPPV[`\0a\x0C\xE0a\n>V[\x90Pa\x0C\xEC\x82\x82a\x0C\xA5V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\x0CWa\r\x0Ba\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a\r-\x82a\n\xABV[\x90P\x91\x90PV[a\r=\x81a\r\"V[\x81\x14a\rHW`\0\x80\xFD[PV[`\0\x815\x90Pa\rZ\x81a\r4V[\x92\x91PPV[`\0a\rsa\rn\x84a\x0C\xF1V[a\x0C\xD6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\r\x96Wa\r\x95a\r\x1DV[[\x83[\x81\x81\x10\x15a\r\xBFW\x80a\r\xAB\x88\x82a\rKV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\r\x98V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\r\xDEWa\r\xDDa\x0C`V[[\x815a\r\xEE\x84\x82` \x86\x01a\r`V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x12Wa\x0E\x11a\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x0E6\x81a\x0E#V[\x81\x14a\x0EAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0ES\x81a\x0E-V[\x92\x91PPV[`\0a\x0Ela\x0Eg\x84a\r\xF7V[a\x0C\xD6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0E\x8FWa\x0E\x8Ea\r\x1DV[[\x83[\x81\x81\x10\x15a\x0E\xB8W\x80a\x0E\xA4\x88\x82a\x0EDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0E\x91V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\xD7Wa\x0E\xD6a\x0C`V[[\x815a\x0E\xE7\x84\x82` \x86\x01a\x0EYV[\x91PP\x92\x91PPV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\x10Wa\x0F\x0Fa\x0CvV[[a\x0F\x19\x82a\x0CeV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x0FHa\x0FC\x84a\x0E\xF5V[a\x0C\xD6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0FdWa\x0Fca\x0E\xF0V[[a\x0Fo\x84\x82\x85a\x0F&V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\x8CWa\x0F\x8Ba\x0C`V[[\x815a\x0F\x9C\x84\x82` \x86\x01a\x0F5V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0F\xBFWa\x0F\xBEa\nHV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xDDWa\x0F\xDCa\nMV[[a\x0F\xE9\x87\x82\x88\x01a\r\xC9V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\nWa\x10\ta\nMV[[a\x10\x16\x87\x82\x88\x01a\x0E\xC2V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x107Wa\x106a\nMV[[a\x10C\x87\x82\x88\x01a\x0E\xC2V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10dWa\x10ca\nMV[[a\x10p\x87\x82\x88\x01a\x0FwV[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x10\xB4\x81a\nRV[\x82RPPV[a\x10\xC3\x81a\n\xABV[\x82RPPV[`\0`@\x82\x01\x90Pa\x10\xDE`\0\x83\x01\x85a\x10\xABV[a\x10\xEB` \x83\x01\x84a\x10\xBAV[\x93\x92PPPV[`\0\x81Q\x90Pa\x11\x01\x81a\x0E-V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\x1DWa\x11\x1Ca\nHV[[`\0a\x11+\x84\x82\x85\x01a\x10\xF2V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x11Ya\x11Ta\x11O\x84a\n\x8BV[a\x114V[a\n\x8BV[\x90P\x91\x90PV[`\0a\x11k\x82a\x11>V[\x90P\x91\x90PV[`\0a\x11}\x82a\x11`V[\x90P\x91\x90PV[a\x11\x8D\x81a\x11rV[\x82RPPV[`\0a\x11\x9E\x82a\x11`V[\x90P\x91\x90PV[a\x11\xAE\x81a\x11\x93V[\x82RPPV[a\x11\xBD\x81a\x0BeV[\x82RPPV[a\x11\xCC\x81a\x0B\x9BV[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x11\xE7`\0\x83\x01\x89a\x10\xABV[a\x11\xF4` \x83\x01\x88a\x10\xBAV[a\x12\x01`@\x83\x01\x87a\x11\x84V[a\x12\x0E``\x83\x01\x86a\x11\xA5V[a\x12\x1B`\x80\x83\x01\x85a\x11\xB4V[a\x12(`\xA0\x83\x01\x84a\x11\xC3V[\x97\x96PPPPPPPV[`\0a\x12>\x82a\x11`V[\x90P\x91\x90PV[a\x12N\x81a\x123V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x12\x8B\x82a\x11`V[\x90P\x91\x90PV[a\x12\x9B\x81a\x12\x80V[\x82RPPV[`\0a\x12\xAD\x83\x83a\x12\x92V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x12\xD1\x82a\x12TV[a\x12\xDB\x81\x85a\x12_V[\x93Pa\x12\xE6\x83a\x12pV[\x80`\0[\x83\x81\x10\x15a\x13\x17W\x81Qa\x12\xFE\x88\x82a\x12\xA1V[\x97Pa\x13\t\x83a\x12\xB9V[\x92PP`\x01\x81\x01\x90Pa\x12\xEAV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x13Y\x81a\x0E#V[\x82RPPV[`\0a\x13k\x83\x83a\x13PV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x13\x8F\x82a\x13$V[a\x13\x99\x81\x85a\x13/V[\x93Pa\x13\xA4\x83a\x13@V[\x80`\0[\x83\x81\x10\x15a\x13\xD5W\x81Qa\x13\xBC\x88\x82a\x13_V[\x97Pa\x13\xC7\x83a\x13wV[\x92PP`\x01\x81\x01\x90Pa\x13\xA8V[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x14\x1CW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x14\x01V[`\0\x84\x84\x01RPPPPV[`\0a\x143\x82a\x13\xE2V[a\x14=\x81\x85a\x13\xEDV[\x93Pa\x14M\x81\x85` \x86\x01a\x13\xFEV[a\x14V\x81a\x0CeV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x14v`\0\x83\x01\x87a\x12EV[\x81\x81\x03` \x83\x01Ra\x14\x88\x81\x86a\x12\xC6V[\x90P\x81\x81\x03`@\x83\x01Ra\x14\x9C\x81\x85a\x13\x84V[\x90P\x81\x81\x03``\x83\x01Ra\x14\xB0\x81\x84a\x14(V[\x90P\x95\x94PPPPPV[`\0\x81Q\x90Pa\x14\xCA\x81a\n_V[\x92\x91PPV[`\0a\x14\xDB\x82a\n\x8BV[\x90P\x91\x90PV[a\x14\xEB\x81a\x14\xD0V[\x81\x14a\x14\xF6W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x15\x08\x81a\x14\xE2V[\x92\x91PPV[`\0\x81Q\x90Pa\x15\x1D\x81a\n\xFBV[\x92\x91PPV[`\0\x81Q\x90Pa\x152\x81a\x0B9V[\x92\x91PPV[`\0\x81Q\x90Pa\x15G\x81a\x0BoV[\x92\x91PPV[`\0\x81Q\x90Pa\x15\\\x81a\x0B\xA7V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x15\x7FWa\x15~a\nHV[[`\0a\x15\x8D\x89\x82\x8A\x01a\x14\xBBV[\x96PP` a\x15\x9E\x89\x82\x8A\x01a\x14\xF9V[\x95PP`@a\x15\xAF\x89\x82\x8A\x01a\x15\x0EV[\x94PP``a\x15\xC0\x89\x82\x8A\x01a\x15#V[\x93PP`\x80a\x15\xD1\x89\x82\x8A\x01a\x158V[\x92PP`\xA0a\x15\xE2\x89\x82\x8A\x01a\x15MV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x01\x90Pa\x16\x04`\0\x83\x01\x84a\x10\xBAV[\x92\x91PPV[`\0``\x82\x01\x90Pa\x16\x1F`\0\x83\x01\x86a\x10\xABV[a\x16,` \x83\x01\x85a\x10\xBAV[a\x169`@\x83\x01\x84a\x10\xBAV[\x94\x93PPPPV[a\x16J\x81a\x0E#V[\x82RPPV[`\0`@\x82\x01\x90Pa\x16e`\0\x83\x01\x85a\x10\xBAV[a\x16r` \x83\x01\x84a\x16AV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x16\xB3\x82a\x0E#V[\x91Pa\x16\xBE\x83a\x0E#V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x16\xD6Wa\x16\xD5a\x16yV[[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xF2Wa\x16\xF1a\nHV[[`\0a\x17\0\x84\x82\x85\x01a\x15MV[\x91PP\x92\x91PPV[`\0\x81`\x0F\x0B\x90P\x91\x90PV[a\x17\x1F\x81a\x17\tV[\x82RPPV[`\0\x81\x90P\x91\x90PV[`\0a\x17Ja\x17Ea\x17@\x84a\x17%V[a\x114V[a\x0E#V[\x90P\x91\x90PV[a\x17Z\x81a\x17/V[\x82RPPV[`\0`\x80\x82\x01\x90Pa\x17u`\0\x83\x01\x87a\x17\x16V[a\x17\x82` \x83\x01\x86a\x17\x16V[a\x17\x8F`@\x83\x01\x85a\x16AV[a\x17\x9C``\x83\x01\x84a\x17QV[\x95\x94PPPPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x17\xBB\x82a\x13\xE2V[a\x17\xC5\x81\x85a\x17\xA5V[\x93Pa\x17\xD5\x81\x85` \x86\x01a\x13\xFEV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x17\xED\x82\x84a\x17\xB0V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 rZ\x1D\xBADq\xBE\xD8\xCE\x17\x9DL\x0C&\xD4\xA8\x84\xB9?\x8C\x10vv,\xD3\x8B\x18A\xD1e\xBD\xEAdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static CURVELIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cl\xB0D)\x14a\0;W\x80c\xF0O'\x07\x14a\0WW[`\0\x80\xFD[a\0U`\x04\x806\x03\x81\x01\x90a\0P\x91\x90a\x0B\xD3V[a\0sV[\0[a\0q`\x04\x806\x03\x81\x01\x90a\0l\x91\x90a\x0F\xA5V[a\x02\xF2V[\0[`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x90Wa\0\x8Fa\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xBEW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\0\xF6Wa\0\xF5a\x10|V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01MWa\x01La\x0CvV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01{W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xD9\x92\x91\x90a\x10\xC9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x1A\x91\x90a\x11\x07V[\x81`\0\x81Q\x81\x10a\x02.Wa\x02-a\x10|V[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\x88\x96\x95\x94\x93\x92\x91\x90a\x11\xD2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xB6\x94\x93\x92\x91\x90a\x14aV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xE4W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x83`\0\x81Q\x81\x10a\x03\x08Wa\x03\x07a\x10|V[[` \x02` \x01\x01Q\x90P`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03/\x91\x90a\x15bV[\x95P\x95P\x95P\x95P\x95P\x95P`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03v\x91\x90a\x15\xEFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xB7\x91\x90a\x11\x07V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x16\x93\x92\x91\x90a\x16\nV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x040W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04DW=`\0\x80>=`\0\xFD[PPPP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x83\x92\x91\x90a\x16PV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x9DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xB1W=`\0\x80>=`\0\xFD[PPPP`\0\x81\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xF1\x91\x90a\x15\xEFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x052\x91\x90a\x11\x07V[a\x05<\x91\x90a\x16\xA8V[\x90P`\0a\x05L\x85\x88\x84\x87a\x069V[\x90P`\0\x8A\x82a\x05\\\x91\x90a\x16\xA8V[\x90Pa\x05\xBDs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07q\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06(\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07q\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[`\0\x80\x85`\0\x1C\x90P`\0\x81\x90P`\0\x80\x85a\x06XW`\0`\x01a\x06]V[`\x01`\0[\x91P\x91P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x85\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\x9C\x92\x91\x90a\x16PV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xDF\x91\x90a\x16\xDCV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\xF0!$\x83\x83\x8A`\0`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07 \x94\x93\x92\x91\x90a\x17`V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07c\x91\x90a\x11\x07V[\x94PPPPP\x94\x93PPPPV[a\x07\xEB\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x07\xA4\x92\x91\x90a\x16PV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\x07\xF0V[PPPV[`\0a\x08\x1B\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\x87\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\x08@WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08>\x91\x90a\x16\xDCV[\x15[\x15a\x08\x82W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08y\x91\x90a\x15\xEFV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x08\x95\x83\x83`\0a\x08\x9DV[\x90P\x92\x91PPV[``\x81G\x10\x15a\x08\xE4W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xDB\x91\x90a\x15\xEFV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\t\r\x91\x90a\x17\xE1V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\tJW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tOV[``\x91P[P\x91P\x91Pa\t_\x86\x83\x83a\tjV[\x92PPP\x93\x92PPPV[``\x82a\t\x7FWa\tz\x82a\t\xF9V[a\t\xF1V[`\0\x82Q\x14\x80\x15a\t\xA7WP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\t\xE9W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xE0\x91\x90a\x15\xEFV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\t\xF2V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\n\x0CW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\nh\x81a\nRV[\x81\x14a\nsW`\0\x80\xFD[PV[`\0\x815\x90Pa\n\x85\x81a\n_V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\n\xB6\x82a\n\x8BV[\x90P\x91\x90PV[a\n\xC6\x81a\n\xABV[\x81\x14a\n\xD1W`\0\x80\xFD[PV[`\0\x815\x90Pa\n\xE3\x81a\n\xBDV[\x92\x91PPV[`\0a\n\xF4\x82a\n\xABV[\x90P\x91\x90PV[a\x0B\x04\x81a\n\xE9V[\x81\x14a\x0B\x0FW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B!\x81a\n\xFBV[\x92\x91PPV[`\0a\x0B2\x82a\n\xABV[\x90P\x91\x90PV[a\x0BB\x81a\x0B'V[\x81\x14a\x0BMW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B_\x81a\x0B9V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0Bx\x81a\x0BeV[\x81\x14a\x0B\x83W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\x95\x81a\x0BoV[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0B\xB0\x81a\x0B\x9BV[\x81\x14a\x0B\xBBW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\xCD\x81a\x0B\xA7V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0B\xF0Wa\x0B\xEFa\nHV[[`\0a\x0B\xFE\x89\x82\x8A\x01a\nvV[\x96PP` a\x0C\x0F\x89\x82\x8A\x01a\n\xD4V[\x95PP`@a\x0C \x89\x82\x8A\x01a\x0B\x12V[\x94PP``a\x0C1\x89\x82\x8A\x01a\x0BPV[\x93PP`\x80a\x0CB\x89\x82\x8A\x01a\x0B\x86V[\x92PP`\xA0a\x0CS\x89\x82\x8A\x01a\x0B\xBEV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x0C\xAE\x82a\x0CeV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\xCDWa\x0C\xCCa\x0CvV[[\x80`@RPPPV[`\0a\x0C\xE0a\n>V[\x90Pa\x0C\xEC\x82\x82a\x0C\xA5V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\x0CWa\r\x0Ba\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a\r-\x82a\n\xABV[\x90P\x91\x90PV[a\r=\x81a\r\"V[\x81\x14a\rHW`\0\x80\xFD[PV[`\0\x815\x90Pa\rZ\x81a\r4V[\x92\x91PPV[`\0a\rsa\rn\x84a\x0C\xF1V[a\x0C\xD6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\r\x96Wa\r\x95a\r\x1DV[[\x83[\x81\x81\x10\x15a\r\xBFW\x80a\r\xAB\x88\x82a\rKV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\r\x98V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\r\xDEWa\r\xDDa\x0C`V[[\x815a\r\xEE\x84\x82` \x86\x01a\r`V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x12Wa\x0E\x11a\x0CvV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x0E6\x81a\x0E#V[\x81\x14a\x0EAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0ES\x81a\x0E-V[\x92\x91PPV[`\0a\x0Ela\x0Eg\x84a\r\xF7V[a\x0C\xD6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0E\x8FWa\x0E\x8Ea\r\x1DV[[\x83[\x81\x81\x10\x15a\x0E\xB8W\x80a\x0E\xA4\x88\x82a\x0EDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0E\x91V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\xD7Wa\x0E\xD6a\x0C`V[[\x815a\x0E\xE7\x84\x82` \x86\x01a\x0EYV[\x91PP\x92\x91PPV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\x10Wa\x0F\x0Fa\x0CvV[[a\x0F\x19\x82a\x0CeV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x0FHa\x0FC\x84a\x0E\xF5V[a\x0C\xD6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0FdWa\x0Fca\x0E\xF0V[[a\x0Fo\x84\x82\x85a\x0F&V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\x8CWa\x0F\x8Ba\x0C`V[[\x815a\x0F\x9C\x84\x82` \x86\x01a\x0F5V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0F\xBFWa\x0F\xBEa\nHV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xDDWa\x0F\xDCa\nMV[[a\x0F\xE9\x87\x82\x88\x01a\r\xC9V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\nWa\x10\ta\nMV[[a\x10\x16\x87\x82\x88\x01a\x0E\xC2V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x107Wa\x106a\nMV[[a\x10C\x87\x82\x88\x01a\x0E\xC2V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10dWa\x10ca\nMV[[a\x10p\x87\x82\x88\x01a\x0FwV[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x10\xB4\x81a\nRV[\x82RPPV[a\x10\xC3\x81a\n\xABV[\x82RPPV[`\0`@\x82\x01\x90Pa\x10\xDE`\0\x83\x01\x85a\x10\xABV[a\x10\xEB` \x83\x01\x84a\x10\xBAV[\x93\x92PPPV[`\0\x81Q\x90Pa\x11\x01\x81a\x0E-V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\x1DWa\x11\x1Ca\nHV[[`\0a\x11+\x84\x82\x85\x01a\x10\xF2V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x11Ya\x11Ta\x11O\x84a\n\x8BV[a\x114V[a\n\x8BV[\x90P\x91\x90PV[`\0a\x11k\x82a\x11>V[\x90P\x91\x90PV[`\0a\x11}\x82a\x11`V[\x90P\x91\x90PV[a\x11\x8D\x81a\x11rV[\x82RPPV[`\0a\x11\x9E\x82a\x11`V[\x90P\x91\x90PV[a\x11\xAE\x81a\x11\x93V[\x82RPPV[a\x11\xBD\x81a\x0BeV[\x82RPPV[a\x11\xCC\x81a\x0B\x9BV[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x11\xE7`\0\x83\x01\x89a\x10\xABV[a\x11\xF4` \x83\x01\x88a\x10\xBAV[a\x12\x01`@\x83\x01\x87a\x11\x84V[a\x12\x0E``\x83\x01\x86a\x11\xA5V[a\x12\x1B`\x80\x83\x01\x85a\x11\xB4V[a\x12(`\xA0\x83\x01\x84a\x11\xC3V[\x97\x96PPPPPPPV[`\0a\x12>\x82a\x11`V[\x90P\x91\x90PV[a\x12N\x81a\x123V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x12\x8B\x82a\x11`V[\x90P\x91\x90PV[a\x12\x9B\x81a\x12\x80V[\x82RPPV[`\0a\x12\xAD\x83\x83a\x12\x92V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x12\xD1\x82a\x12TV[a\x12\xDB\x81\x85a\x12_V[\x93Pa\x12\xE6\x83a\x12pV[\x80`\0[\x83\x81\x10\x15a\x13\x17W\x81Qa\x12\xFE\x88\x82a\x12\xA1V[\x97Pa\x13\t\x83a\x12\xB9V[\x92PP`\x01\x81\x01\x90Pa\x12\xEAV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x13Y\x81a\x0E#V[\x82RPPV[`\0a\x13k\x83\x83a\x13PV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x13\x8F\x82a\x13$V[a\x13\x99\x81\x85a\x13/V[\x93Pa\x13\xA4\x83a\x13@V[\x80`\0[\x83\x81\x10\x15a\x13\xD5W\x81Qa\x13\xBC\x88\x82a\x13_V[\x97Pa\x13\xC7\x83a\x13wV[\x92PP`\x01\x81\x01\x90Pa\x13\xA8V[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x14\x1CW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x14\x01V[`\0\x84\x84\x01RPPPPV[`\0a\x143\x82a\x13\xE2V[a\x14=\x81\x85a\x13\xEDV[\x93Pa\x14M\x81\x85` \x86\x01a\x13\xFEV[a\x14V\x81a\x0CeV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x14v`\0\x83\x01\x87a\x12EV[\x81\x81\x03` \x83\x01Ra\x14\x88\x81\x86a\x12\xC6V[\x90P\x81\x81\x03`@\x83\x01Ra\x14\x9C\x81\x85a\x13\x84V[\x90P\x81\x81\x03``\x83\x01Ra\x14\xB0\x81\x84a\x14(V[\x90P\x95\x94PPPPPV[`\0\x81Q\x90Pa\x14\xCA\x81a\n_V[\x92\x91PPV[`\0a\x14\xDB\x82a\n\x8BV[\x90P\x91\x90PV[a\x14\xEB\x81a\x14\xD0V[\x81\x14a\x14\xF6W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x15\x08\x81a\x14\xE2V[\x92\x91PPV[`\0\x81Q\x90Pa\x15\x1D\x81a\n\xFBV[\x92\x91PPV[`\0\x81Q\x90Pa\x152\x81a\x0B9V[\x92\x91PPV[`\0\x81Q\x90Pa\x15G\x81a\x0BoV[\x92\x91PPV[`\0\x81Q\x90Pa\x15\\\x81a\x0B\xA7V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x15\x7FWa\x15~a\nHV[[`\0a\x15\x8D\x89\x82\x8A\x01a\x14\xBBV[\x96PP` a\x15\x9E\x89\x82\x8A\x01a\x14\xF9V[\x95PP`@a\x15\xAF\x89\x82\x8A\x01a\x15\x0EV[\x94PP``a\x15\xC0\x89\x82\x8A\x01a\x15#V[\x93PP`\x80a\x15\xD1\x89\x82\x8A\x01a\x158V[\x92PP`\xA0a\x15\xE2\x89\x82\x8A\x01a\x15MV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x01\x90Pa\x16\x04`\0\x83\x01\x84a\x10\xBAV[\x92\x91PPV[`\0``\x82\x01\x90Pa\x16\x1F`\0\x83\x01\x86a\x10\xABV[a\x16,` \x83\x01\x85a\x10\xBAV[a\x169`@\x83\x01\x84a\x10\xBAV[\x94\x93PPPPV[a\x16J\x81a\x0E#V[\x82RPPV[`\0`@\x82\x01\x90Pa\x16e`\0\x83\x01\x85a\x10\xBAV[a\x16r` \x83\x01\x84a\x16AV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x16\xB3\x82a\x0E#V[\x91Pa\x16\xBE\x83a\x0E#V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x16\xD6Wa\x16\xD5a\x16yV[[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xF2Wa\x16\xF1a\nHV[[`\0a\x17\0\x84\x82\x85\x01a\x15MV[\x91PP\x92\x91PPV[`\0\x81`\x0F\x0B\x90P\x91\x90PV[a\x17\x1F\x81a\x17\tV[\x82RPPV[`\0\x81\x90P\x91\x90PV[`\0a\x17Ja\x17Ea\x17@\x84a\x17%V[a\x114V[a\x0E#V[\x90P\x91\x90PV[a\x17Z\x81a\x17/V[\x82RPPV[`\0`\x80\x82\x01\x90Pa\x17u`\0\x83\x01\x87a\x17\x16V[a\x17\x82` \x83\x01\x86a\x17\x16V[a\x17\x8F`@\x83\x01\x85a\x16AV[a\x17\x9C``\x83\x01\x84a\x17QV[\x95\x94PPPPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x17\xBB\x82a\x13\xE2V[a\x17\xC5\x81\x85a\x17\xA5V[\x93Pa\x17\xD5\x81\x85` \x86\x01a\x13\xFEV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x17\xED\x82\x84a\x17\xB0V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 rZ\x1D\xBADq\xBE\xD8\xCE\x17\x9DL\x0C&\xD4\xA8\x84\xB9?\x8C\x10vv,\xD3\x8B\x18A\xD1e\xBD\xEAdsolcC\0\x08\x15\x003";
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
