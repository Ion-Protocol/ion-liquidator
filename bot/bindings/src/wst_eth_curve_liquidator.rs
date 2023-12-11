pub use wst_eth_curve_liquidator::*;
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
pub mod wst_eth_curve_liquidator {
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
                        name: ::std::borrow::ToOwned::to_owned("wstEth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IWstEth"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static WSTETHCURVELIQUIDATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1E\xCB8\x03\x80b\0\x1E\xCB\x839\x81\x81\x01`@R\x81\x01\x90b\0\08\x91\x90b\0\x03~V[\x84\x84\x84\x83\x83\x83\x83\x83\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\xC0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3`\xA0Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x01q\x92\x91\x90b\0\x042V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x01\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xB7\x91\x90b\0\x04\x9CV[PPPPPPPPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPPPb\0\x04\xCEV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x022\x82b\0\x02\x05V[\x90P\x91\x90PV[`\0b\0\x02F\x82b\0\x02%V[\x90P\x91\x90PV[b\0\x02X\x81b\0\x029V[\x81\x14b\0\x02dW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02x\x81b\0\x02MV[\x92\x91PPV[`\0b\0\x02\x8B\x82b\0\x02%V[\x90P\x91\x90PV[b\0\x02\x9D\x81b\0\x02~V[\x81\x14b\0\x02\xA9W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\xBD\x81b\0\x02\x92V[\x92\x91PPV[`\0b\0\x02\xD0\x82b\0\x02%V[\x90P\x91\x90PV[b\0\x02\xE2\x81b\0\x02\xC3V[\x81\x14b\0\x02\xEEW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\x02\x81b\0\x02\xD7V[\x92\x91PPV[`\0b\0\x03\x15\x82b\0\x02%V[\x90P\x91\x90PV[b\0\x03'\x81b\0\x03\x08V[\x81\x14b\0\x033W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03G\x81b\0\x03\x1CV[\x92\x91PPV[b\0\x03X\x81b\0\x02%V[\x81\x14b\0\x03dW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03x\x81b\0\x03MV[\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x03\x9DWb\0\x03\x9Cb\0\x02\0V[[`\0b\0\x03\xAD\x88\x82\x89\x01b\0\x02gV[\x95PP` b\0\x03\xC0\x88\x82\x89\x01b\0\x02\xACV[\x94PP`@b\0\x03\xD3\x88\x82\x89\x01b\0\x02\xF1V[\x93PP``b\0\x03\xE6\x88\x82\x89\x01b\0\x036V[\x92PP`\x80b\0\x03\xF9\x88\x82\x89\x01b\0\x03gV[\x91PP\x92\x95P\x92\x95\x90\x93PV[b\0\x04\x11\x81b\0\x02%V[\x82RPPV[`\0\x81\x90P\x91\x90PV[b\0\x04,\x81b\0\x04\x17V[\x82RPPV[`\0`@\x82\x01\x90Pb\0\x04I`\0\x83\x01\x85b\0\x04\x06V[b\0\x04X` \x83\x01\x84b\0\x04!V[\x93\x92PPPV[`\0\x81\x15\x15\x90P\x91\x90PV[b\0\x04v\x81b\0\x04_V[\x81\x14b\0\x04\x82W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x04\x96\x81b\0\x04kV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x04\xB5Wb\0\x04\xB4b\0\x02\0V[[`\0b\0\x04\xC5\x84\x82\x85\x01b\0\x04\x85V[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x19\xA0b\0\x05+`\09`\0a\x06V\x01R`\0a\x05\xDA\x01R`\0\x81\x81`\xDB\x01R\x81\x81a\x05\x91\x01R\x81\x81a\x05\xFC\x01Ra\x07\x04\x01R`\0\x81\x81a\x01\x98\x01Ra\x03\xD3\x01R`\0PPa\x19\xA0`\0\xF3\xFE`\x80`@R`\x046\x10a\0-W`\x005`\xE0\x1C\x80cl\xB0D)\x14a\09W\x80c\xF0O'\x07\x14a\0bWa\x004V[6a\x004W\0[`\0\x80\xFD[4\x80\x15a\0EW`\0\x80\xFD[Pa\0``\x04\x806\x03\x81\x01\x90a\0[\x91\x90a\r*V[a\0\x8BV[\0[4\x80\x15a\0nW`\0\x80\xFD[Pa\0\x89`\x04\x806\x03\x81\x01\x90a\0\x84\x91\x90a\x10\xFCV[a\x03\nV[\0[`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xA8Wa\0\xA7a\r\xCDV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xD6W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x01\x0EWa\x01\ra\x11\xD3V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01eWa\x01da\r\xCDV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x93W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xF1\x92\x91\x90a\x12 V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x022\x91\x90a\x12^V[\x81`\0\x81Q\x81\x10a\x02FWa\x02Ea\x11\xD3V[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\xA0\x96\x95\x94\x93\x92\x91\x90a\x13)V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xCE\x94\x93\x92\x91\x90a\x15\xB8V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xFCW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x83`\0\x81Q\x81\x10a\x03 Wa\x03\x1Fa\x11\xD3V[[` \x02` \x01\x01Q\x90P`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03G\x91\x90a\x16\xB9V[\x95P\x95P\x95P\x95P\x95P\x95P`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x8E\x91\x90a\x17FV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCF\x91\x90a\x12^V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04.\x93\x92\x91\x90a\x17aV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04HW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\\W=`\0\x80>=`\0\xFD[PPPP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x9B\x92\x91\x90a\x17\xA7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xB5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xC9W=`\0\x80>=`\0\xFD[PPPP`\0\x81\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\t\x91\x90a\x17FV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05J\x91\x90a\x12^V[a\x05T\x91\x90a\x17\xFFV[\x90P`\0a\x05d\x85\x88\x84\x87a\x06QV[\x90P`\0\x8A\x82a\x05t\x91\x90a\x17\xFFV[\x90Pa\x05\xD5s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x90\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06@\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x90\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDE\x0E\x9A>\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xAD\x91\x90a\x183V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF0\x91\x90a\x12^V[\x90P`\0a\x07\0\x87\x87\x84\x87a\x08\x0FV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x07jW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07~W=`\0\x80>=`\0\xFD[PPPPP\x80\x92PPP\x94\x93PPPPV[a\x08\n\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x07\xC3\x92\x91\x90a\x17\xA7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\tGV[PPPV[`\0\x80\x85`\0\x1C\x90P`\0\x81\x90P`\0\x80\x85a\x08.W`\0`\x01a\x083V[`\x01`\0[\x91P\x91P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x85\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08r\x92\x91\x90a\x17\xA7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB5\x91\x90a\x18NV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\xF0!$\x83\x83\x8A`\0`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xF6\x94\x93\x92\x91\x90a\x18\xD2V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t9\x91\x90a\x12^V[\x94PPPPP\x94\x93PPPPV[`\0a\tr\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xDE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\t\x97WP\x80\x80` \x01\x90Q\x81\x01\x90a\t\x95\x91\x90a\x18NV[\x15[\x15a\t\xD9W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xD0\x91\x90a\x17FV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\t\xEC\x83\x83`\0a\t\xF4V[\x90P\x92\x91PPV[``\x81G\x10\x15a\n;W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n2\x91\x90a\x17FV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\nd\x91\x90a\x19SV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xA6V[``\x91P[P\x91P\x91Pa\n\xB6\x86\x83\x83a\n\xC1V[\x92PPP\x93\x92PPPV[``\x82a\n\xD6Wa\n\xD1\x82a\x0BPV[a\x0BHV[`\0\x82Q\x14\x80\x15a\n\xFEWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x0B@W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B7\x91\x90a\x17FV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x0BIV[[\x93\x92PPPV[`\0\x81Q\x11\x15a\x0BcW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x0B\xBF\x81a\x0B\xA9V[\x81\x14a\x0B\xCAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\xDC\x81a\x0B\xB6V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0C\r\x82a\x0B\xE2V[\x90P\x91\x90PV[a\x0C\x1D\x81a\x0C\x02V[\x81\x14a\x0C(W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C:\x81a\x0C\x14V[\x92\x91PPV[`\0a\x0CK\x82a\x0C\x02V[\x90P\x91\x90PV[a\x0C[\x81a\x0C@V[\x81\x14a\x0CfW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0Cx\x81a\x0CRV[\x92\x91PPV[`\0a\x0C\x89\x82a\x0C\x02V[\x90P\x91\x90PV[a\x0C\x99\x81a\x0C~V[\x81\x14a\x0C\xA4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\xB6\x81a\x0C\x90V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0C\xCF\x81a\x0C\xBCV[\x81\x14a\x0C\xDAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\xEC\x81a\x0C\xC6V[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\r\x07\x81a\x0C\xF2V[\x81\x14a\r\x12W`\0\x80\xFD[PV[`\0\x815\x90Pa\r$\x81a\x0C\xFEV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\rGWa\rFa\x0B\x9FV[[`\0a\rU\x89\x82\x8A\x01a\x0B\xCDV[\x96PP` a\rf\x89\x82\x8A\x01a\x0C+V[\x95PP`@a\rw\x89\x82\x8A\x01a\x0CiV[\x94PP``a\r\x88\x89\x82\x8A\x01a\x0C\xA7V[\x93PP`\x80a\r\x99\x89\x82\x8A\x01a\x0C\xDDV[\x92PP`\xA0a\r\xAA\x89\x82\x8A\x01a\r\x15V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x0E\x05\x82a\r\xBCV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E$Wa\x0E#a\r\xCDV[[\x80`@RPPPV[`\0a\x0E7a\x0B\x95V[\x90Pa\x0EC\x82\x82a\r\xFCV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0EcWa\x0Eba\r\xCDV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a\x0E\x84\x82a\x0C\x02V[\x90P\x91\x90PV[a\x0E\x94\x81a\x0EyV[\x81\x14a\x0E\x9FW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xB1\x81a\x0E\x8BV[\x92\x91PPV[`\0a\x0E\xCAa\x0E\xC5\x84a\x0EHV[a\x0E-V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0E\xEDWa\x0E\xECa\x0EtV[[\x83[\x81\x81\x10\x15a\x0F\x16W\x80a\x0F\x02\x88\x82a\x0E\xA2V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0E\xEFV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F5Wa\x0F4a\r\xB7V[[\x815a\x0FE\x84\x82` \x86\x01a\x0E\xB7V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FiWa\x0Fha\r\xCDV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x0F\x8D\x81a\x0FzV[\x81\x14a\x0F\x98W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\xAA\x81a\x0F\x84V[\x92\x91PPV[`\0a\x0F\xC3a\x0F\xBE\x84a\x0FNV[a\x0E-V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0F\xE6Wa\x0F\xE5a\x0EtV[[\x83[\x81\x81\x10\x15a\x10\x0FW\x80a\x0F\xFB\x88\x82a\x0F\x9BV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0F\xE8V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x10.Wa\x10-a\r\xB7V[[\x815a\x10>\x84\x82` \x86\x01a\x0F\xB0V[\x91PP\x92\x91PPV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10gWa\x10fa\r\xCDV[[a\x10p\x82a\r\xBCV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x10\x9Fa\x10\x9A\x84a\x10LV[a\x0E-V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x10\xBBWa\x10\xBAa\x10GV[[a\x10\xC6\x84\x82\x85a\x10}V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x10\xE3Wa\x10\xE2a\r\xB7V[[\x815a\x10\xF3\x84\x82` \x86\x01a\x10\x8CV[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x11\x16Wa\x11\x15a\x0B\x9FV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x114Wa\x113a\x0B\xA4V[[a\x11@\x87\x82\x88\x01a\x0F V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11aWa\x11`a\x0B\xA4V[[a\x11m\x87\x82\x88\x01a\x10\x19V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x8EWa\x11\x8Da\x0B\xA4V[[a\x11\x9A\x87\x82\x88\x01a\x10\x19V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xBBWa\x11\xBAa\x0B\xA4V[[a\x11\xC7\x87\x82\x88\x01a\x10\xCEV[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x12\x0B\x81a\x0B\xA9V[\x82RPPV[a\x12\x1A\x81a\x0C\x02V[\x82RPPV[`\0`@\x82\x01\x90Pa\x125`\0\x83\x01\x85a\x12\x02V[a\x12B` \x83\x01\x84a\x12\x11V[\x93\x92PPPV[`\0\x81Q\x90Pa\x12X\x81a\x0F\x84V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12tWa\x12sa\x0B\x9FV[[`\0a\x12\x82\x84\x82\x85\x01a\x12IV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x12\xB0a\x12\xABa\x12\xA6\x84a\x0B\xE2V[a\x12\x8BV[a\x0B\xE2V[\x90P\x91\x90PV[`\0a\x12\xC2\x82a\x12\x95V[\x90P\x91\x90PV[`\0a\x12\xD4\x82a\x12\xB7V[\x90P\x91\x90PV[a\x12\xE4\x81a\x12\xC9V[\x82RPPV[`\0a\x12\xF5\x82a\x12\xB7V[\x90P\x91\x90PV[a\x13\x05\x81a\x12\xEAV[\x82RPPV[a\x13\x14\x81a\x0C\xBCV[\x82RPPV[a\x13#\x81a\x0C\xF2V[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x13>`\0\x83\x01\x89a\x12\x02V[a\x13K` \x83\x01\x88a\x12\x11V[a\x13X`@\x83\x01\x87a\x12\xDBV[a\x13e``\x83\x01\x86a\x12\xFCV[a\x13r`\x80\x83\x01\x85a\x13\x0BV[a\x13\x7F`\xA0\x83\x01\x84a\x13\x1AV[\x97\x96PPPPPPPV[`\0a\x13\x95\x82a\x12\xB7V[\x90P\x91\x90PV[a\x13\xA5\x81a\x13\x8AV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x13\xE2\x82a\x12\xB7V[\x90P\x91\x90PV[a\x13\xF2\x81a\x13\xD7V[\x82RPPV[`\0a\x14\x04\x83\x83a\x13\xE9V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x14(\x82a\x13\xABV[a\x142\x81\x85a\x13\xB6V[\x93Pa\x14=\x83a\x13\xC7V[\x80`\0[\x83\x81\x10\x15a\x14nW\x81Qa\x14U\x88\x82a\x13\xF8V[\x97Pa\x14`\x83a\x14\x10V[\x92PP`\x01\x81\x01\x90Pa\x14AV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x14\xB0\x81a\x0FzV[\x82RPPV[`\0a\x14\xC2\x83\x83a\x14\xA7V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x14\xE6\x82a\x14{V[a\x14\xF0\x81\x85a\x14\x86V[\x93Pa\x14\xFB\x83a\x14\x97V[\x80`\0[\x83\x81\x10\x15a\x15,W\x81Qa\x15\x13\x88\x82a\x14\xB6V[\x97Pa\x15\x1E\x83a\x14\xCEV[\x92PP`\x01\x81\x01\x90Pa\x14\xFFV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x15sW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x15XV[`\0\x84\x84\x01RPPPPV[`\0a\x15\x8A\x82a\x159V[a\x15\x94\x81\x85a\x15DV[\x93Pa\x15\xA4\x81\x85` \x86\x01a\x15UV[a\x15\xAD\x81a\r\xBCV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x15\xCD`\0\x83\x01\x87a\x13\x9CV[\x81\x81\x03` \x83\x01Ra\x15\xDF\x81\x86a\x14\x1DV[\x90P\x81\x81\x03`@\x83\x01Ra\x15\xF3\x81\x85a\x14\xDBV[\x90P\x81\x81\x03``\x83\x01Ra\x16\x07\x81\x84a\x15\x7FV[\x90P\x95\x94PPPPPV[`\0\x81Q\x90Pa\x16!\x81a\x0B\xB6V[\x92\x91PPV[`\0a\x162\x82a\x0B\xE2V[\x90P\x91\x90PV[a\x16B\x81a\x16'V[\x81\x14a\x16MW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x16_\x81a\x169V[\x92\x91PPV[`\0\x81Q\x90Pa\x16t\x81a\x0CRV[\x92\x91PPV[`\0\x81Q\x90Pa\x16\x89\x81a\x0C\x90V[\x92\x91PPV[`\0\x81Q\x90Pa\x16\x9E\x81a\x0C\xC6V[\x92\x91PPV[`\0\x81Q\x90Pa\x16\xB3\x81a\x0C\xFEV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x16\xD6Wa\x16\xD5a\x0B\x9FV[[`\0a\x16\xE4\x89\x82\x8A\x01a\x16\x12V[\x96PP` a\x16\xF5\x89\x82\x8A\x01a\x16PV[\x95PP`@a\x17\x06\x89\x82\x8A\x01a\x16eV[\x94PP``a\x17\x17\x89\x82\x8A\x01a\x16zV[\x93PP`\x80a\x17(\x89\x82\x8A\x01a\x16\x8FV[\x92PP`\xA0a\x179\x89\x82\x8A\x01a\x16\xA4V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x01\x90Pa\x17[`\0\x83\x01\x84a\x12\x11V[\x92\x91PPV[`\0``\x82\x01\x90Pa\x17v`\0\x83\x01\x86a\x12\x02V[a\x17\x83` \x83\x01\x85a\x12\x11V[a\x17\x90`@\x83\x01\x84a\x12\x11V[\x94\x93PPPPV[a\x17\xA1\x81a\x0FzV[\x82RPPV[`\0`@\x82\x01\x90Pa\x17\xBC`\0\x83\x01\x85a\x12\x11V[a\x17\xC9` \x83\x01\x84a\x17\x98V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x18\n\x82a\x0FzV[\x91Pa\x18\x15\x83a\x0FzV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x18-Wa\x18,a\x17\xD0V[[\x92\x91PPV[`\0` \x82\x01\x90Pa\x18H`\0\x83\x01\x84a\x17\x98V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x18dWa\x18ca\x0B\x9FV[[`\0a\x18r\x84\x82\x85\x01a\x16\xA4V[\x91PP\x92\x91PPV[`\0\x81`\x0F\x0B\x90P\x91\x90PV[a\x18\x91\x81a\x18{V[\x82RPPV[`\0\x81\x90P\x91\x90PV[`\0a\x18\xBCa\x18\xB7a\x18\xB2\x84a\x18\x97V[a\x12\x8BV[a\x0FzV[\x90P\x91\x90PV[a\x18\xCC\x81a\x18\xA1V[\x82RPPV[`\0`\x80\x82\x01\x90Pa\x18\xE7`\0\x83\x01\x87a\x18\x88V[a\x18\xF4` \x83\x01\x86a\x18\x88V[a\x19\x01`@\x83\x01\x85a\x17\x98V[a\x19\x0E``\x83\x01\x84a\x18\xC3V[\x95\x94PPPPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x19-\x82a\x159V[a\x197\x81\x85a\x19\x17V[\x93Pa\x19G\x81\x85` \x86\x01a\x15UV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x19_\x82\x84a\x19\"V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 j\x0E+C\x8D\xD6\xA3yn\xE2e\x82z\x05\x19-\\a|Nh;\x85\xCA\x80\x88\xD8\xDC:\xF2\xC1\xF9dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static WSTETHCURVELIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0-W`\x005`\xE0\x1C\x80cl\xB0D)\x14a\09W\x80c\xF0O'\x07\x14a\0bWa\x004V[6a\x004W\0[`\0\x80\xFD[4\x80\x15a\0EW`\0\x80\xFD[Pa\0``\x04\x806\x03\x81\x01\x90a\0[\x91\x90a\r*V[a\0\x8BV[\0[4\x80\x15a\0nW`\0\x80\xFD[Pa\0\x89`\x04\x806\x03\x81\x01\x90a\0\x84\x91\x90a\x10\xFCV[a\x03\nV[\0[`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xA8Wa\0\xA7a\r\xCDV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xD6W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x01\x0EWa\x01\ra\x11\xD3V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\0`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01eWa\x01da\r\xCDV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x93W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xD5\xF5\x99\x89\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xF1\x92\x91\x90a\x12 V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x022\x91\x90a\x12^V[\x81`\0\x81Q\x81\x10a\x02FWa\x02Ea\x11\xD3V[[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\8D\x9E0\x84\x84\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x02\xA0\x96\x95\x94\x93\x92\x91\x90a\x13)V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xCE\x94\x93\x92\x91\x90a\x15\xB8V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xFCW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x83`\0\x81Q\x81\x10a\x03 Wa\x03\x1Fa\x11\xD3V[[` \x02` \x01\x01Q\x90P`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x03G\x91\x90a\x16\xB9V[\x95P\x95P\x95P\x95P\x95P\x95P`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x8E\x91\x90a\x17FV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCF\x91\x90a\x12^V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c%\x84\x0E\xDA\x88\x880`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04.\x93\x92\x91\x90a\x17aV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04HW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\\W=`\0\x80>=`\0\xFD[PPPP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEFi;\xED0\x8A`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x9B\x92\x91\x90a\x17\xA7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xB5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xC9W=`\0\x80>=`\0\xFD[PPPP`\0\x81\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\t\x91\x90a\x17FV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05J\x91\x90a\x12^V[a\x05T\x91\x90a\x17\xFFV[\x90P`\0a\x05d\x85\x88\x84\x87a\x06QV[\x90P`\0\x8A\x82a\x05t\x91\x90a\x17\xFFV[\x90Pa\x05\xD5s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x90\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x06@\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x90\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPPPPPPPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDE\x0E\x9A>\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xAD\x91\x90a\x183V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF0\x91\x90a\x12^V[\x90P`\0a\x07\0\x87\x87\x84\x87a\x08\x0FV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x07jW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07~W=`\0\x80>=`\0\xFD[PPPPP\x80\x92PPP\x94\x93PPPPV[a\x08\n\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\x07\xC3\x92\x91\x90a\x17\xA7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\tGV[PPPV[`\0\x80\x85`\0\x1C\x90P`\0\x81\x90P`\0\x80\x85a\x08.W`\0`\x01a\x083V[`\x01`\0[\x91P\x91P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t^\xA7\xB3\x85\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08r\x92\x91\x90a\x17\xA7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB5\x91\x90a\x18NV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\xF0!$\x83\x83\x8A`\0`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xF6\x94\x93\x92\x91\x90a\x18\xD2V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t9\x91\x90a\x12^V[\x94PPPPP\x94\x93PPPPV[`\0a\tr\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xDE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\t\x97WP\x80\x80` \x01\x90Q\x81\x01\x90a\t\x95\x91\x90a\x18NV[\x15[\x15a\t\xD9W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xD0\x91\x90a\x17FV[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\t\xEC\x83\x83`\0a\t\xF4V[\x90P\x92\x91PPV[``\x81G\x10\x15a\n;W0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n2\x91\x90a\x17FV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\nd\x91\x90a\x19SV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xA6V[``\x91P[P\x91P\x91Pa\n\xB6\x86\x83\x83a\n\xC1V[\x92PPP\x93\x92PPPV[``\x82a\n\xD6Wa\n\xD1\x82a\x0BPV[a\x0BHV[`\0\x82Q\x14\x80\x15a\n\xFEWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x0B@W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B7\x91\x90a\x17FV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x0BIV[[\x93\x92PPPV[`\0\x81Q\x11\x15a\x0BcW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x0B\xBF\x81a\x0B\xA9V[\x81\x14a\x0B\xCAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0B\xDC\x81a\x0B\xB6V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0C\r\x82a\x0B\xE2V[\x90P\x91\x90PV[a\x0C\x1D\x81a\x0C\x02V[\x81\x14a\x0C(W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C:\x81a\x0C\x14V[\x92\x91PPV[`\0a\x0CK\x82a\x0C\x02V[\x90P\x91\x90PV[a\x0C[\x81a\x0C@V[\x81\x14a\x0CfW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0Cx\x81a\x0CRV[\x92\x91PPV[`\0a\x0C\x89\x82a\x0C\x02V[\x90P\x91\x90PV[a\x0C\x99\x81a\x0C~V[\x81\x14a\x0C\xA4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\xB6\x81a\x0C\x90V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0C\xCF\x81a\x0C\xBCV[\x81\x14a\x0C\xDAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\xEC\x81a\x0C\xC6V[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\r\x07\x81a\x0C\xF2V[\x81\x14a\r\x12W`\0\x80\xFD[PV[`\0\x815\x90Pa\r$\x81a\x0C\xFEV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\rGWa\rFa\x0B\x9FV[[`\0a\rU\x89\x82\x8A\x01a\x0B\xCDV[\x96PP` a\rf\x89\x82\x8A\x01a\x0C+V[\x95PP`@a\rw\x89\x82\x8A\x01a\x0CiV[\x94PP``a\r\x88\x89\x82\x8A\x01a\x0C\xA7V[\x93PP`\x80a\r\x99\x89\x82\x8A\x01a\x0C\xDDV[\x92PP`\xA0a\r\xAA\x89\x82\x8A\x01a\r\x15V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x0E\x05\x82a\r\xBCV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E$Wa\x0E#a\r\xCDV[[\x80`@RPPPV[`\0a\x0E7a\x0B\x95V[\x90Pa\x0EC\x82\x82a\r\xFCV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0EcWa\x0Eba\r\xCDV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a\x0E\x84\x82a\x0C\x02V[\x90P\x91\x90PV[a\x0E\x94\x81a\x0EyV[\x81\x14a\x0E\x9FW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xB1\x81a\x0E\x8BV[\x92\x91PPV[`\0a\x0E\xCAa\x0E\xC5\x84a\x0EHV[a\x0E-V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0E\xEDWa\x0E\xECa\x0EtV[[\x83[\x81\x81\x10\x15a\x0F\x16W\x80a\x0F\x02\x88\x82a\x0E\xA2V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0E\xEFV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F5Wa\x0F4a\r\xB7V[[\x815a\x0FE\x84\x82` \x86\x01a\x0E\xB7V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FiWa\x0Fha\r\xCDV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x0F\x8D\x81a\x0FzV[\x81\x14a\x0F\x98W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\xAA\x81a\x0F\x84V[\x92\x91PPV[`\0a\x0F\xC3a\x0F\xBE\x84a\x0FNV[a\x0E-V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0F\xE6Wa\x0F\xE5a\x0EtV[[\x83[\x81\x81\x10\x15a\x10\x0FW\x80a\x0F\xFB\x88\x82a\x0F\x9BV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0F\xE8V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x10.Wa\x10-a\r\xB7V[[\x815a\x10>\x84\x82` \x86\x01a\x0F\xB0V[\x91PP\x92\x91PPV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10gWa\x10fa\r\xCDV[[a\x10p\x82a\r\xBCV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x10\x9Fa\x10\x9A\x84a\x10LV[a\x0E-V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x10\xBBWa\x10\xBAa\x10GV[[a\x10\xC6\x84\x82\x85a\x10}V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x10\xE3Wa\x10\xE2a\r\xB7V[[\x815a\x10\xF3\x84\x82` \x86\x01a\x10\x8CV[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x11\x16Wa\x11\x15a\x0B\x9FV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x114Wa\x113a\x0B\xA4V[[a\x11@\x87\x82\x88\x01a\x0F V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11aWa\x11`a\x0B\xA4V[[a\x11m\x87\x82\x88\x01a\x10\x19V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x8EWa\x11\x8Da\x0B\xA4V[[a\x11\x9A\x87\x82\x88\x01a\x10\x19V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xBBWa\x11\xBAa\x0B\xA4V[[a\x11\xC7\x87\x82\x88\x01a\x10\xCEV[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x12\x0B\x81a\x0B\xA9V[\x82RPPV[a\x12\x1A\x81a\x0C\x02V[\x82RPPV[`\0`@\x82\x01\x90Pa\x125`\0\x83\x01\x85a\x12\x02V[a\x12B` \x83\x01\x84a\x12\x11V[\x93\x92PPPV[`\0\x81Q\x90Pa\x12X\x81a\x0F\x84V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12tWa\x12sa\x0B\x9FV[[`\0a\x12\x82\x84\x82\x85\x01a\x12IV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x12\xB0a\x12\xABa\x12\xA6\x84a\x0B\xE2V[a\x12\x8BV[a\x0B\xE2V[\x90P\x91\x90PV[`\0a\x12\xC2\x82a\x12\x95V[\x90P\x91\x90PV[`\0a\x12\xD4\x82a\x12\xB7V[\x90P\x91\x90PV[a\x12\xE4\x81a\x12\xC9V[\x82RPPV[`\0a\x12\xF5\x82a\x12\xB7V[\x90P\x91\x90PV[a\x13\x05\x81a\x12\xEAV[\x82RPPV[a\x13\x14\x81a\x0C\xBCV[\x82RPPV[a\x13#\x81a\x0C\xF2V[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x13>`\0\x83\x01\x89a\x12\x02V[a\x13K` \x83\x01\x88a\x12\x11V[a\x13X`@\x83\x01\x87a\x12\xDBV[a\x13e``\x83\x01\x86a\x12\xFCV[a\x13r`\x80\x83\x01\x85a\x13\x0BV[a\x13\x7F`\xA0\x83\x01\x84a\x13\x1AV[\x97\x96PPPPPPPV[`\0a\x13\x95\x82a\x12\xB7V[\x90P\x91\x90PV[a\x13\xA5\x81a\x13\x8AV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x13\xE2\x82a\x12\xB7V[\x90P\x91\x90PV[a\x13\xF2\x81a\x13\xD7V[\x82RPPV[`\0a\x14\x04\x83\x83a\x13\xE9V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x14(\x82a\x13\xABV[a\x142\x81\x85a\x13\xB6V[\x93Pa\x14=\x83a\x13\xC7V[\x80`\0[\x83\x81\x10\x15a\x14nW\x81Qa\x14U\x88\x82a\x13\xF8V[\x97Pa\x14`\x83a\x14\x10V[\x92PP`\x01\x81\x01\x90Pa\x14AV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x14\xB0\x81a\x0FzV[\x82RPPV[`\0a\x14\xC2\x83\x83a\x14\xA7V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x14\xE6\x82a\x14{V[a\x14\xF0\x81\x85a\x14\x86V[\x93Pa\x14\xFB\x83a\x14\x97V[\x80`\0[\x83\x81\x10\x15a\x15,W\x81Qa\x15\x13\x88\x82a\x14\xB6V[\x97Pa\x15\x1E\x83a\x14\xCEV[\x92PP`\x01\x81\x01\x90Pa\x14\xFFV[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x15sW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x15XV[`\0\x84\x84\x01RPPPPV[`\0a\x15\x8A\x82a\x159V[a\x15\x94\x81\x85a\x15DV[\x93Pa\x15\xA4\x81\x85` \x86\x01a\x15UV[a\x15\xAD\x81a\r\xBCV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x15\xCD`\0\x83\x01\x87a\x13\x9CV[\x81\x81\x03` \x83\x01Ra\x15\xDF\x81\x86a\x14\x1DV[\x90P\x81\x81\x03`@\x83\x01Ra\x15\xF3\x81\x85a\x14\xDBV[\x90P\x81\x81\x03``\x83\x01Ra\x16\x07\x81\x84a\x15\x7FV[\x90P\x95\x94PPPPPV[`\0\x81Q\x90Pa\x16!\x81a\x0B\xB6V[\x92\x91PPV[`\0a\x162\x82a\x0B\xE2V[\x90P\x91\x90PV[a\x16B\x81a\x16'V[\x81\x14a\x16MW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x16_\x81a\x169V[\x92\x91PPV[`\0\x81Q\x90Pa\x16t\x81a\x0CRV[\x92\x91PPV[`\0\x81Q\x90Pa\x16\x89\x81a\x0C\x90V[\x92\x91PPV[`\0\x81Q\x90Pa\x16\x9E\x81a\x0C\xC6V[\x92\x91PPV[`\0\x81Q\x90Pa\x16\xB3\x81a\x0C\xFEV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x16\xD6Wa\x16\xD5a\x0B\x9FV[[`\0a\x16\xE4\x89\x82\x8A\x01a\x16\x12V[\x96PP` a\x16\xF5\x89\x82\x8A\x01a\x16PV[\x95PP`@a\x17\x06\x89\x82\x8A\x01a\x16eV[\x94PP``a\x17\x17\x89\x82\x8A\x01a\x16zV[\x93PP`\x80a\x17(\x89\x82\x8A\x01a\x16\x8FV[\x92PP`\xA0a\x179\x89\x82\x8A\x01a\x16\xA4V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x01\x90Pa\x17[`\0\x83\x01\x84a\x12\x11V[\x92\x91PPV[`\0``\x82\x01\x90Pa\x17v`\0\x83\x01\x86a\x12\x02V[a\x17\x83` \x83\x01\x85a\x12\x11V[a\x17\x90`@\x83\x01\x84a\x12\x11V[\x94\x93PPPPV[a\x17\xA1\x81a\x0FzV[\x82RPPV[`\0`@\x82\x01\x90Pa\x17\xBC`\0\x83\x01\x85a\x12\x11V[a\x17\xC9` \x83\x01\x84a\x17\x98V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x18\n\x82a\x0FzV[\x91Pa\x18\x15\x83a\x0FzV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x18-Wa\x18,a\x17\xD0V[[\x92\x91PPV[`\0` \x82\x01\x90Pa\x18H`\0\x83\x01\x84a\x17\x98V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x18dWa\x18ca\x0B\x9FV[[`\0a\x18r\x84\x82\x85\x01a\x16\xA4V[\x91PP\x92\x91PPV[`\0\x81`\x0F\x0B\x90P\x91\x90PV[a\x18\x91\x81a\x18{V[\x82RPPV[`\0\x81\x90P\x91\x90PV[`\0a\x18\xBCa\x18\xB7a\x18\xB2\x84a\x18\x97V[a\x12\x8BV[a\x0FzV[\x90P\x91\x90PV[a\x18\xCC\x81a\x18\xA1V[\x82RPPV[`\0`\x80\x82\x01\x90Pa\x18\xE7`\0\x83\x01\x87a\x18\x88V[a\x18\xF4` \x83\x01\x86a\x18\x88V[a\x19\x01`@\x83\x01\x85a\x17\x98V[a\x19\x0E``\x83\x01\x84a\x18\xC3V[\x95\x94PPPPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x19-\x82a\x159V[a\x197\x81\x85a\x19\x17V[\x93Pa\x19G\x81\x85` \x86\x01a\x15UV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x19_\x82\x84a\x19\"V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 j\x0E+C\x8D\xD6\xA3yn\xE2e\x82z\x05\x19-\\a|Nh;\x85\xCA\x80\x88\xD8\xDC:\xF2\xC1\xF9dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static WSTETHCURVELIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct WstEthCurveLiquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WstEthCurveLiquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WstEthCurveLiquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WstEthCurveLiquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WstEthCurveLiquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WstEthCurveLiquidator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WstEthCurveLiquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    WSTETHCURVELIQUIDATOR_ABI.clone(),
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
                WSTETHCURVELIQUIDATOR_ABI.clone(),
                WSTETHCURVELIQUIDATOR_BYTECODE.clone().into(),
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
    for WstEthCurveLiquidator<M> {
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
    pub enum WstEthCurveLiquidatorErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for WstEthCurveLiquidatorErrors {
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
    impl ::ethers::core::abi::AbiEncode for WstEthCurveLiquidatorErrors {
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
    impl ::ethers::contract::ContractRevert for WstEthCurveLiquidatorErrors {
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
    impl ::core::fmt::Display for WstEthCurveLiquidatorErrors {
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
    impl ::core::convert::From<::std::string::String> for WstEthCurveLiquidatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for WstEthCurveLiquidatorErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance>
    for WstEthCurveLiquidatorErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for WstEthCurveLiquidatorErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation>
    for WstEthCurveLiquidatorErrors {
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
    pub enum WstEthCurveLiquidatorCalls {
        Liquidate(LiquidateCall),
        ReceiveFlashLoan(ReceiveFlashLoanCall),
    }
    impl ::ethers::core::abi::AbiDecode for WstEthCurveLiquidatorCalls {
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
    impl ::ethers::core::abi::AbiEncode for WstEthCurveLiquidatorCalls {
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
    impl ::core::fmt::Display for WstEthCurveLiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceiveFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LiquidateCall> for WstEthCurveLiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<ReceiveFlashLoanCall> for WstEthCurveLiquidatorCalls {
        fn from(value: ReceiveFlashLoanCall) -> Self {
            Self::ReceiveFlashLoan(value)
        }
    }
}
