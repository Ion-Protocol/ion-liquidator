pub use mock_spot_oracle::*;
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
pub mod mock_spot_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("ltv"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("reserveOracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_price"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LTV"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LTV"),
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
                                        ::std::borrow::ToOwned::to_owned("contract ReserveOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPrice"),
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
                    ::std::borrow::ToOwned::to_owned("getSpot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spot"),
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
                    ::std::borrow::ToOwned::to_owned("setPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidLtv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidLtv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidReserveOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidReserveOracle",
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKSPOTORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\xCE8\x03\x80a\x07\xCE\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x01\xD1V[\x82\x82k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x11\x15a\0\x85W\x81`@Q\x7F.?\xCF\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0|\x91\x90a\x023V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xEBW`@Q\x7F\xC42+\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x81\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPP\x80`\0\x81\x90UPPPPa\x02NV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x01P\x81a\x01=V[\x81\x14a\x01[W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x01m\x81a\x01GV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x01\x9E\x82a\x01sV[\x90P\x91\x90PV[a\x01\xAE\x81a\x01\x93V[\x81\x14a\x01\xB9W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x01\xCB\x81a\x01\xA5V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x01\xEAWa\x01\xE9a\x018V[[`\0a\x01\xF8\x86\x82\x87\x01a\x01^V[\x93PP` a\x02\t\x86\x82\x87\x01a\x01\xBCV[\x92PP`@a\x02\x1A\x86\x82\x87\x01a\x01^V[\x91PP\x92P\x92P\x92V[a\x02-\x81a\x01=V[\x82RPPV[`\0` \x82\x01\x90Pa\x02H`\0\x83\x01\x84a\x02$V[\x92\x91PPV[`\x80Q`\xA0Qa\x05Na\x02\x80`\09`\0\x81\x81`\xF2\x01Ra\x01%\x01R`\0\x81\x81a\x01\xC8\x01Ra\x01\xFF\x01Ra\x05N`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x1FQU\xC4\x14a\0\\W\x80c+7&\x9C\x14a\0zW\x80cb\x96]\x8E\x14a\0\x98W\x80c\x91\xB7\xF5\xED\x14a\0\xB6W\x80c\x98\xD5\xFD\xCA\x14a\0\xD2W[`\0\x80\xFD[a\0da\0\xF0V[`@Qa\0q\x91\x90a\x03\xFAV[`@Q\x80\x91\x03\x90\xF3[a\0\x82a\x01\x14V[`@Qa\0\x8F\x91\x90a\x04.V[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\x01\xFDV[`@Qa\0\xAD\x91\x90a\x04.V[`@Q\x80\x91\x03\x90\xF3[a\0\xD0`\x04\x806\x03\x81\x01\x90a\0\xCB\x91\x90a\x04zV[a\x02!V[\0[a\0\xDAa\x02+V[`@Qa\0\xE7\x91\x90a\x04.V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x01\x1Fa\x02+V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB2\x91\x90a\x04\xBCV[\x90P`\0a\x01\xC0\x83\x83a\x024V[\x90Pa\x01\xF5\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93PPPP\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x80`\0\x81\x90UPPV[`\0\x80T\x90P\x90V[`\0\x81\x83\x10a\x02CW\x81a\x02EV[\x82[\x90P\x92\x91PPV[`\0a\x02l\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x02t\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x02\xAFW\x83\x82\x81a\x02\xA5Wa\x02\xA4a\x04\xE9V[[\x04\x92PPPa\x03tV[\x80\x84\x11a\x02\xE8W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x03\xC0a\x03\xBBa\x03\xB6\x84a\x03{V[a\x03\x9BV[a\x03{V[\x90P\x91\x90PV[`\0a\x03\xD2\x82a\x03\xA5V[\x90P\x91\x90PV[`\0a\x03\xE4\x82a\x03\xC7V[\x90P\x91\x90PV[a\x03\xF4\x81a\x03\xD9V[\x82RPPV[`\0` \x82\x01\x90Pa\x04\x0F`\0\x83\x01\x84a\x03\xEBV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x04(\x81a\x04\x15V[\x82RPPV[`\0` \x82\x01\x90Pa\x04C`\0\x83\x01\x84a\x04\x1FV[\x92\x91PPV[`\0\x80\xFD[a\x04W\x81a\x04\x15V[\x81\x14a\x04bW`\0\x80\xFD[PV[`\0\x815\x90Pa\x04t\x81a\x04NV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\x90Wa\x04\x8Fa\x04IV[[`\0a\x04\x9E\x84\x82\x85\x01a\x04eV[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa\x04\xB6\x81a\x04NV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\xD2Wa\x04\xD1a\x04IV[[`\0a\x04\xE0\x84\x82\x85\x01a\x04\xA7V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 kB]\x1F\x9D\x084\xEB4\x1F\xC2u6\x85\xBD\x9C\x8B*\xBDY\x8BvV\x95K\x01$\x9F\x9D`\x99\xE5dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKSPOTORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x1FQU\xC4\x14a\0\\W\x80c+7&\x9C\x14a\0zW\x80cb\x96]\x8E\x14a\0\x98W\x80c\x91\xB7\xF5\xED\x14a\0\xB6W\x80c\x98\xD5\xFD\xCA\x14a\0\xD2W[`\0\x80\xFD[a\0da\0\xF0V[`@Qa\0q\x91\x90a\x03\xFAV[`@Q\x80\x91\x03\x90\xF3[a\0\x82a\x01\x14V[`@Qa\0\x8F\x91\x90a\x04.V[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\x01\xFDV[`@Qa\0\xAD\x91\x90a\x04.V[`@Q\x80\x91\x03\x90\xF3[a\0\xD0`\x04\x806\x03\x81\x01\x90a\0\xCB\x91\x90a\x04zV[a\x02!V[\0[a\0\xDAa\x02+V[`@Qa\0\xE7\x91\x90a\x04.V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x01\x1Fa\x02+V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB2\x91\x90a\x04\xBCV[\x90P`\0a\x01\xC0\x83\x83a\x024V[\x90Pa\x01\xF5\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93PPPP\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x80`\0\x81\x90UPPV[`\0\x80T\x90P\x90V[`\0\x81\x83\x10a\x02CW\x81a\x02EV[\x82[\x90P\x92\x91PPV[`\0a\x02l\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x02t\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x02\xAFW\x83\x82\x81a\x02\xA5Wa\x02\xA4a\x04\xE9V[[\x04\x92PPPa\x03tV[\x80\x84\x11a\x02\xE8W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x03\xC0a\x03\xBBa\x03\xB6\x84a\x03{V[a\x03\x9BV[a\x03{V[\x90P\x91\x90PV[`\0a\x03\xD2\x82a\x03\xA5V[\x90P\x91\x90PV[`\0a\x03\xE4\x82a\x03\xC7V[\x90P\x91\x90PV[a\x03\xF4\x81a\x03\xD9V[\x82RPPV[`\0` \x82\x01\x90Pa\x04\x0F`\0\x83\x01\x84a\x03\xEBV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x04(\x81a\x04\x15V[\x82RPPV[`\0` \x82\x01\x90Pa\x04C`\0\x83\x01\x84a\x04\x1FV[\x92\x91PPV[`\0\x80\xFD[a\x04W\x81a\x04\x15V[\x81\x14a\x04bW`\0\x80\xFD[PV[`\0\x815\x90Pa\x04t\x81a\x04NV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\x90Wa\x04\x8Fa\x04IV[[`\0a\x04\x9E\x84\x82\x85\x01a\x04eV[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa\x04\xB6\x81a\x04NV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\xD2Wa\x04\xD1a\x04IV[[`\0a\x04\xE0\x84\x82\x85\x01a\x04\xA7V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 kB]\x1F\x9D\x084\xEB4\x1F\xC2u6\x85\xBD\x9C\x8B*\xBDY\x8BvV\x95K\x01$\x9F\x9D`\x99\xE5dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKSPOTORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockSpotOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockSpotOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockSpotOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockSpotOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockSpotOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockSpotOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockSpotOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKSPOTORACLE_ABI.clone(),
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
                MOCKSPOTORACLE_ABI.clone(),
                MOCKSPOTORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `LTV` (0x62965d8e) function
        pub fn ltv(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 150, 93, 142], ())
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
        ///Calls the contract's `getPrice` (0x98d5fdca) function
        pub fn get_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 213, 253, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpot` (0x2b37269c) function
        pub fn get_spot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([43, 55, 38, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPrice` (0x91b7f5ed) function
        pub fn set_price(
            &self,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 183, 245, 237], price)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockSpotOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidLtv` with signature `InvalidLtv(uint256)` and selector `0x2e3fcf0a`
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
    #[etherror(name = "InvalidLtv", abi = "InvalidLtv(uint256)")]
    pub struct InvalidLtv {
        pub ltv: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidReserveOracle` with signature `InvalidReserveOracle()` and selector `0xc4322b81`
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
    #[etherror(name = "InvalidReserveOracle", abi = "InvalidReserveOracle()")]
    pub struct InvalidReserveOracle;
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
    pub enum MockSpotOracleErrors {
        InvalidLtv(InvalidLtv),
        InvalidReserveOracle(InvalidReserveOracle),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockSpotOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidLtv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidLtv(decoded));
            }
            if let Ok(decoded) = <InvalidReserveOracle as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReserveOracle(decoded));
            }
            if let Ok(decoded) = <MathOverflowedMulDiv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MathOverflowedMulDiv(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockSpotOracleErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidLtv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockSpotOracleErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidLtv as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidReserveOracle as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MathOverflowedMulDiv as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockSpotOracleErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidLtv(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReserveOracle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockSpotOracleErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidLtv> for MockSpotOracleErrors {
        fn from(value: InvalidLtv) -> Self {
            Self::InvalidLtv(value)
        }
    }
    impl ::core::convert::From<InvalidReserveOracle> for MockSpotOracleErrors {
        fn from(value: InvalidReserveOracle) -> Self {
            Self::InvalidReserveOracle(value)
        }
    }
    impl ::core::convert::From<MathOverflowedMulDiv> for MockSpotOracleErrors {
        fn from(value: MathOverflowedMulDiv) -> Self {
            Self::MathOverflowedMulDiv(value)
        }
    }
    ///Container type for all input parameters for the `LTV` function with signature `LTV()` and selector `0x62965d8e`
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
    #[ethcall(name = "LTV", abi = "LTV()")]
    pub struct LtvCall;
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
    ///Container type for all input parameters for the `getPrice` function with signature `getPrice()` and selector `0x98d5fdca`
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
    #[ethcall(name = "getPrice", abi = "getPrice()")]
    pub struct GetPriceCall;
    ///Container type for all input parameters for the `getSpot` function with signature `getSpot()` and selector `0x2b37269c`
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
    #[ethcall(name = "getSpot", abi = "getSpot()")]
    pub struct GetSpotCall;
    ///Container type for all input parameters for the `setPrice` function with signature `setPrice(uint256)` and selector `0x91b7f5ed`
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
    #[ethcall(name = "setPrice", abi = "setPrice(uint256)")]
    pub struct SetPriceCall {
        pub price: ::ethers::core::types::U256,
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
    pub enum MockSpotOracleCalls {
        Ltv(LtvCall),
        ReserveOracle(ReserveOracleCall),
        GetPrice(GetPriceCall),
        GetSpot(GetSpotCall),
        SetPrice(SetPriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockSpotOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LtvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ltv(decoded));
            }
            if let Ok(decoded) = <ReserveOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReserveOracle(decoded));
            }
            if let Ok(decoded) = <GetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPrice(decoded));
            }
            if let Ok(decoded) = <GetSpotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSpot(decoded));
            }
            if let Ok(decoded) = <SetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockSpotOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Ltv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockSpotOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Ltv(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpot(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPrice(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LtvCall> for MockSpotOracleCalls {
        fn from(value: LtvCall) -> Self {
            Self::Ltv(value)
        }
    }
    impl ::core::convert::From<ReserveOracleCall> for MockSpotOracleCalls {
        fn from(value: ReserveOracleCall) -> Self {
            Self::ReserveOracle(value)
        }
    }
    impl ::core::convert::From<GetPriceCall> for MockSpotOracleCalls {
        fn from(value: GetPriceCall) -> Self {
            Self::GetPrice(value)
        }
    }
    impl ::core::convert::From<GetSpotCall> for MockSpotOracleCalls {
        fn from(value: GetSpotCall) -> Self {
            Self::GetSpot(value)
        }
    }
    impl ::core::convert::From<SetPriceCall> for MockSpotOracleCalls {
        fn from(value: SetPriceCall) -> Self {
            Self::SetPrice(value)
        }
    }
    ///Container type for all return fields from the `LTV` function with signature `LTV()` and selector `0x62965d8e`
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
    pub struct LtvReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `getPrice` function with signature `getPrice()` and selector `0x98d5fdca`
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
    pub struct GetPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSpot` function with signature `getSpot()` and selector `0x2b37269c`
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
    pub struct GetSpotReturn {
        pub spot: ::ethers::core::types::U256,
    }
}
