pub use yield_oracle::*;
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
pub mod yield_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_historicalExchangeRates",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ),
                                    3usize,
                                ),
                            ),
                            7usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64[3][7]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_wstEth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stader"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_swell"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESS0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADDRESS0"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESS1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADDRESS1"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESS2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADDRESS2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("apys"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("apys"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("currentIndex"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("historicalExchangeRates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "historicalExchangeRates",
                            ),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ionPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ionPool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IonPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastUpdated"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("updateAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateIonPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateIonPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ionPool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IonPool"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ApyUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApyUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newApy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferStarted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferStarted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadyUpdated"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidExchangeRate",
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
                            ],
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
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
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
                    ::std::borrow::ToOwned::to_owned("SafeCastOverflowedUintDowncast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeCastOverflowedUintDowncast",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
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
    pub static YIELDORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0+\x828\x03\x80b\0+\x82\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\r\xA5V[\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\xADW`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0\xA4\x91\x90b\0\x0ECV[`@Q\x80\x91\x03\x90\xFD[b\0\0\xBE\x81b\0\x01\x8A` \x1B` \x1CV[P\x84`\x03\x90`\x07b\0\0\xD2\x92\x91\x90b\0\tmV[P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPb\0\x01\x7Fb\0\x01\xC3` \x1B` \x1CV[PPPPPb\0\x14;V[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ub\0\x01\xC0\x81b\0\x05\x18` \x1B` \x1CV[PV[Bb\x01Jx`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\x01\xF1\x91\x90b\0\x0E\x99V[\x11\x15b\0\x02*W`@Q\x7F\xE5=\xFD\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x90P`\0`\x03\x82`\x07\x81\x10b\0\x02`Wb\0\x02_b\0\x0E\xD4V[[\x01\x90P`\0[`\x03c\xFF\xFF\xFF\xFF\x16\x81`\xFF\x16\x10\x15b\0\x04\x9FW`\0b\0\x02\x8F\x82`\xFF\x16b\0\x05\xDC` \x1B` \x1CV[\x90P`\0\x83\x83`\xFF\x16`\x03\x81\x10b\0\x02\xACWb\0\x02\xABb\0\x0E\xD4V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80b\0\x02\xFBWP\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10[\x15b\0\x03@W\x82`@Q\x7F\xD8\xD9\xAE\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x037\x91\x90b\0\x0FSV[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x83b\0\x03P\x91\x90b\0\x0FpV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0b\0\x03\xC1b\0\x03\xB5`\x07c\xFF\xFF\xFF\xFF\x16`\x08`\nb\0\x03~\x91\x90b\0\x11\x07V[a\x01mb\0\x03\x8D\x91\x90b\0\x11XV[b\0\x03\x99\x91\x90b\0\x11\xD2V[\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85b\0\x07\xA6` \x1B\x90\x92\x91\x90` \x1CV[b\0\x08\xB3` \x1B` \x1CV[\x90P\x80`\x02\x86`\xFF\x16`\x03\x81\x10b\0\x03\xDEWb\0\x03\xDDb\0\x0E\xD4V[[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x86\x86`\xFF\x16`\x03\x81\x10b\0\x04!Wb\0\x04 b\0\x0E\xD4V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\xFF\x16\x7F\xD2\x0CoF\xFFO\x97t2\x15\x1B\xD6,\xF6[\x9F\x90\xDC~\x0CFc\xF8#\xB64\xCC\x94\xC0MU\x9E\x82`@Qb\0\x04\x87\x91\x90b\0\x12SV[`@Q\x80\x91\x03\x90\xA2\x84`\x01\x01\x94PPPPPb\0\x02fV[P`\x07`\x01`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16b\0\x04\xC3\x91\x90b\0\x12pV[b\0\x04\xCF\x91\x90b\0\x12\xAFV[`\n`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPB`\n`\x18a\x01\0\n\x81T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80\x82\x03b\0\x06xW`\0`\x80Q\x90Pb\0\x06o\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x03_\xAF\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06c\x91\x90b\0\x13\x18V[b\0\t\x0E` \x1B` \x1CV[\x91PPb\0\x07\xA1V[`\x01\x82\x03b\0\x07\x13W`\0`\xA0Q\x90Pb\0\x07\n\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE6\xAA!l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\xFE\x91\x90b\0\x13\x18V[b\0\t\x0E` \x1B` \x1CV[\x91PPb\0\x07\xA0V[`\0`\xC0Q\x90Pb\0\x07\x9C\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\x8B,\xB6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x07jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x07\x90\x91\x90b\0\x13\x18V[b\0\t\x0E` \x1B` \x1CV[\x91PP[[\x91\x90PV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03b\0\x07\xE6W\x83\x82\x81b\0\x07\xDBWb\0\x07\xDAb\0\x11\xA3V[[\x04\x92PPPb\0\x08\xACV[\x80\x84\x11b\0\x08 W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15b\0\t\x06W` \x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x08\xFD\x92\x91\x90b\0\x13\x9EV[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15b\0\teW`@\x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\t\\\x92\x91\x90b\0\x14\x0EV[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[\x82`\x07\x81\x01\x92\x82\x15b\0\t\xAFW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\t\xAEW\x82Q\x82\x90`\x03b\0\t\x9D\x92\x91\x90b\0\t\xC2V[P\x91` \x01\x91\x90`\x01\x01\x90b\0\t\x81V[[P\x90Pb\0\t\xBE\x91\x90b\0\nwV[P\x90V[\x82`\x03\x80\x01`\x04\x90\x04\x81\x01\x92\x82\x15b\0\ndW\x91` \x02\x82\x01`\0[\x83\x82\x11\x15b\0\n,W\x83Q\x83\x82a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x92` \x01\x92`\x08\x01` \x81`\x07\x01\x04\x92\x83\x01\x92`\x01\x03\x02b\0\t\xDEV[\x80\x15b\0\nbW\x82\x81a\x01\0\n\x81T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x08\x01` \x81`\x07\x01\x04\x92\x83\x01\x92`\x01\x03\x02b\0\n,V[P[P\x90Pb\0\ns\x91\x90b\0\n\x9FV[P\x90V[[\x80\x82\x11\x15b\0\n\x9BW`\0\x81\x81b\0\n\x91\x91\x90b\0\n\xBEV[P`\x01\x01b\0\nxV[P\x90V[[\x80\x82\x11\x15b\0\n\xBAW`\0\x81`\0\x90UP`\x01\x01b\0\n\xA0V[P\x90V[P`\0\x90UV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x0B$\x82b\0\n\xD9V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x0BFWb\0\x0BEb\0\n\xEAV[[\x80`@RPPPV[`\0b\0\x0B[b\0\n\xC5V[\x90Pb\0\x0Bi\x82\x82b\0\x0B\x19V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x0B\x8CWb\0\x0B\x8Bb\0\n\xEAV[[` \x82\x02\x90P\x91\x90PV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x0B\xBAWb\0\x0B\xB9b\0\n\xEAV[[` \x82\x02\x90P\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x0B\xE4\x81b\0\x0B\xC5V[\x81\x14b\0\x0B\xF0W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x0C\x04\x81b\0\x0B\xD9V[\x92\x91PPV[`\0b\0\x0C!b\0\x0C\x1B\x84b\0\x0B\x9CV[b\0\x0BOV[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x0C>Wb\0\x0C=b\0\x0B\x97V[[\x83[\x81\x81\x10\x15b\0\x0CkW\x80b\0\x0CV\x88\x82b\0\x0B\xF3V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x0C@V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x0C\x8DWb\0\x0C\x8Cb\0\n\xD4V[[`\x03b\0\x0C\x9C\x84\x82\x85b\0\x0C\nV[\x91PP\x92\x91PPV[`\0b\0\x0C\xBCb\0\x0C\xB6\x84b\0\x0BnV[b\0\x0BOV[\x90P\x80``\x84\x02\x83\x01\x85\x81\x11\x15b\0\x0C\xD9Wb\0\x0C\xD8b\0\x0B\x97V[[\x83[\x81\x81\x10\x15b\0\r\x06W\x80b\0\x0C\xF1\x88\x82b\0\x0CuV[\x84R` \x84\x01\x93PP``\x81\x01\x90Pb\0\x0C\xDBV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\r(Wb\0\r'b\0\n\xD4V[[`\x07b\0\r7\x84\x82\x85b\0\x0C\xA5V[\x91PP\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\rm\x82b\0\r@V[\x90P\x91\x90PV[b\0\r\x7F\x81b\0\r`V[\x81\x14b\0\r\x8BW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\r\x9F\x81b\0\rtV[\x92\x91PPV[`\0\x80`\0\x80`\0a\x03 \x86\x88\x03\x12\x15b\0\r\xC5Wb\0\r\xC4b\0\n\xCFV[[`\0b\0\r\xD5\x88\x82\x89\x01b\0\r\x10V[\x95PPa\x02\xA0b\0\r\xE9\x88\x82\x89\x01b\0\r\x8EV[\x94PPa\x02\xC0b\0\r\xFD\x88\x82\x89\x01b\0\r\x8EV[\x93PPa\x02\xE0b\0\x0E\x11\x88\x82\x89\x01b\0\r\x8EV[\x92PPa\x03\0b\0\x0E%\x88\x82\x89\x01b\0\r\x8EV[\x91PP\x92\x95P\x92\x95\x90\x93PV[b\0\x0E=\x81b\0\r`V[\x82RPPV[`\0` \x82\x01\x90Pb\0\x0EZ`\0\x83\x01\x84b\0\x0E2V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0b\0\x0E\xA6\x82b\0\x0E`V[\x91Pb\0\x0E\xB3\x83b\0\x0E`V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15b\0\x0E\xCEWb\0\x0E\xCDb\0\x0EjV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0b\0\x0F;b\0\x0F5b\0\x0F/\x84b\0\x0F\x03V[b\0\x0F\x10V[b\0\x0E`V[\x90P\x91\x90PV[b\0\x0FM\x81b\0\x0F\x1AV[\x82RPPV[`\0` \x82\x01\x90Pb\0\x0Fj`\0\x83\x01\x84b\0\x0FBV[\x92\x91PPV[`\0b\0\x0F}\x82b\0\x0B\xC5V[\x91Pb\0\x0F\x8A\x83b\0\x0B\xC5V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0F\xADWb\0\x0F\xACb\0\x0EjV[[\x92\x91PPV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15b\0\x10\x12W\x80\x86\x04\x81\x11\x15b\0\x0F\xEAWb\0\x0F\xE9b\0\x0EjV[[`\x01\x85\x16\x15b\0\x0F\xFAW\x80\x82\x02\x91P[\x80\x81\x02\x90Pb\0\x10\n\x85b\0\x0F\xB3V[\x94Pb\0\x0F\xCAV[\x94P\x94\x92PPPV[`\0\x82b\0\x10-W`\x01\x90Pb\0\x11\0V[\x81b\0\x10=W`\0\x90Pb\0\x11\0V[\x81`\x01\x81\x14b\0\x10VW`\x02\x81\x14b\0\x10aWb\0\x10\x97V[`\x01\x91PPb\0\x11\0V[`\xFF\x84\x11\x15b\0\x10vWb\0\x10ub\0\x0EjV[[\x83`\x02\n\x91P\x84\x82\x11\x15b\0\x10\x90Wb\0\x10\x8Fb\0\x0EjV[[Pb\0\x11\0V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\0\x10\xD1W\x82\x82\n\x90P\x83\x81\x11\x15b\0\x10\xCBWb\0\x10\xCAb\0\x0EjV[[b\0\x11\0V[b\0\x10\xE0\x84\x84\x84`\x01b\0\x0F\xC0V[\x92P\x90P\x81\x84\x04\x81\x11\x15b\0\x10\xFAWb\0\x10\xF9b\0\x0EjV[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0b\0\x11\x14\x82b\0\x0E`V[\x91Pb\0\x11!\x83b\0\x0F\x03V[\x92Pb\0\x11P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84b\0\x10\x1BV[\x90P\x92\x91PPV[`\0b\0\x11e\x82b\0\x0E`V[\x91Pb\0\x11r\x83b\0\x0E`V[\x92P\x82\x82\x02b\0\x11\x82\x81b\0\x0E`V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17b\0\x11\x9CWb\0\x11\x9Bb\0\x0EjV[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0b\0\x11\xDF\x82b\0\x0E`V[\x91Pb\0\x11\xEC\x83b\0\x0E`V[\x92P\x82b\0\x11\xFFWb\0\x11\xFEb\0\x11\xA3V[[\x82\x82\x04\x90P\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x12;b\0\x125b\0\x12/\x84b\0\x12\nV[b\0\x0F\x10V[b\0\x0E`V[\x90P\x91\x90PV[b\0\x12M\x81b\0\x12\x1AV[\x82RPPV[`\0` \x82\x01\x90Pb\0\x12j`\0\x83\x01\x84b\0\x12BV[\x92\x91PPV[`\0b\0\x12}\x82b\0\x12\nV[\x91Pb\0\x12\x8A\x83b\0\x12\nV[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x12\xA9Wb\0\x12\xA8b\0\x0EjV[[\x92\x91PPV[`\0b\0\x12\xBC\x82b\0\x12\nV[\x91Pb\0\x12\xC9\x83b\0\x12\nV[\x92P\x82b\0\x12\xDCWb\0\x12\xDBb\0\x11\xA3V[[\x82\x82\x06\x90P\x92\x91PPV[b\0\x12\xF2\x81b\0\x0E`V[\x81\x14b\0\x12\xFEW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x13\x12\x81b\0\x12\xE7V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x131Wb\0\x130b\0\n\xCFV[[`\0b\0\x13A\x84\x82\x85\x01b\0\x13\x01V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0b\0\x13ub\0\x13ob\0\x13i\x84b\0\x13JV[b\0\x0F\x10V[b\0\x0F\x03V[\x90P\x91\x90PV[b\0\x13\x87\x81b\0\x13TV[\x82RPPV[b\0\x13\x98\x81b\0\x0E`V[\x82RPPV[`\0`@\x82\x01\x90Pb\0\x13\xB5`\0\x83\x01\x85b\0\x13|V[b\0\x13\xC4` \x83\x01\x84b\0\x13\x8DV[\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0b\0\x13\xF6b\0\x13\xF0b\0\x13\xEA\x84b\0\x13\xCBV[b\0\x0F\x10V[b\0\x0F\x03V[\x90P\x91\x90PV[b\0\x14\x08\x81b\0\x13\xD5V[\x82RPPV[`\0`@\x82\x01\x90Pb\0\x14%`\0\x83\x01\x85b\0\x13\xFDV[b\0\x144` \x83\x01\x84b\0\x13\x8DV[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Qa\x17\x02b\0\x14\x80`\09`\0\x81\x81a\x02\xE8\x01Ra\x0B\xA0\x01R`\0\x81\x81a\x03F\x01Ra\n\xFC\x01R`\0\x81\x81a\x03\x0C\x01Ra\nP\x01Ra\x17\x02`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80cy\xBAP\x97\x11a\0\x97W\x80c\xB5\x1AO$\x11a\0fW\x80c\xB5\x1AO$\x14a\x02*W\x80c\xD0\xB0o]\x14a\x02HW\x80c\xE3\x0C9x\x14a\x02fW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x84Wa\0\xF5V[\x80cy\xBAP\x97\x14a\x01\xB6W\x80c\x81\xCE\x1C#\x14a\x01\xC0W\x80c\x8D\xA5\xCB[\x14a\x01\xF0W\x80c\xB1\xA0\xFE\xEF\x14a\x02\x0EWa\0\xF5V[\x80c&\x98{`\x11a\0\xD3W\x80c&\x98{`\x14a\x01fW\x80c1\xC9IG\x14a\x01\x84W\x80cS\xD7\x86\x93\x14a\x01\xA2W\x80cqP\x18\xA6\x14a\x01\xACWa\0\xF5V[\x80c\x04\xDEG\xE4\x14a\0\xFAW\x80c\x08Oci\x14a\x01*W\x80c#\x12\xC4\xBB\x14a\x01HW[`\0\x80\xFD[a\x01\x14`\x04\x806\x03\x81\x01\x90a\x01\x0F\x91\x90a\x0E\xFCV[a\x02\xA0V[`@Qa\x01!\x91\x90a\x0F_V[`@Q\x80\x91\x03\x90\xF3[a\x012a\x02\xE6V[`@Qa\x01?\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01Pa\x03\nV[`@Qa\x01]\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01na\x03.V[`@Qa\x01{\x91\x90a\x0F\xF5V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ca\x03DV[`@Qa\x01\x99\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01\xAAa\x03hV[\0[a\x01\xB4a\x04\x06V[\0[a\x01\xBEa\x04\x1AV[\0[a\x01\xDA`\x04\x806\x03\x81\x01\x90a\x01\xD5\x91\x90a\x10\x10V[a\x04\xA9V[`@Qa\x01\xE7\x91\x90a\x0F\xF5V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF8a\x04\xD9V[`@Qa\x02\x05\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x02(`\x04\x806\x03\x81\x01\x90a\x02#\x91\x90a\x10{V[a\x05\x02V[\0[a\x022a\x05NV[`@Qa\x02?\x91\x90a\x11\x07V[`@Q\x80\x91\x03\x90\xF3[a\x02Pa\x05tV[`@Qa\x02]\x91\x90a\x11CV[`@Q\x80\x91\x03\x90\xF3[a\x02na\x05\x8CV[`@Qa\x02{\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x02\x9E`\x04\x806\x03\x81\x01\x90a\x02\x99\x91\x90a\x11\x8AV[a\x05\xB6V[\0[`\x03\x82`\x07\x81\x10a\x02\xB0W`\0\x80\xFD[\x01\x81`\x03\x81\x10a\x02\xBFW`\0\x80\xFD[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA6\xAF\xED\x95`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFB\x91\x90a\x11\xCCV[Pa\x04\x04a\x06cV[V[a\x04\x0Ea\t\x83V[a\x04\x18`\0a\n\nV[V[`\0a\x04$a\n;V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04Ea\x05\x8CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x9DW\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x94\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xFD[a\x04\xA6\x81a\n\nV[PV[`\x02\x81`\x03\x81\x10a\x04\xB9W`\0\x80\xFD[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\na\t\x83V[\x80`\n`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[`\n`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\xBEa\t\x83V[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\x1Ea\x04\xD9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[Bb\x01Jx`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\x8F\x91\x90a\x12(V[\x11\x15a\x06\xC7W`@Q\x7F\xE5=\xFD\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x90P`\0`\x03\x82`\x07\x81\x10a\x06\xFAWa\x06\xF9a\x12\\V[[\x01\x90P`\0[`\x03c\xFF\xFF\xFF\xFF\x16\x81`\xFF\x16\x10\x15a\t\x0EW`\0a\x07 \x82`\xFF\x16a\nCV[\x90P`\0\x83\x83`\xFF\x16`\x03\x81\x10a\x07:Wa\x079a\x12\\V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x07\x88WP\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10[\x15a\x07\xCAW\x82`@Q\x7F\xD8\xD9\xAE\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xC1\x91\x90a\x12\xC9V[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x83a\x07\xD8\x91\x90a\x12\xE4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a\x089a\x084`\x07c\xFF\xFF\xFF\xFF\x16`\x08`\na\x08\x02\x91\x90a\x14SV[a\x01ma\x08\x0F\x91\x90a\x14\x9EV[a\x08\x19\x91\x90a\x15\x0FV[\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85a\x0CB\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\rIV[\x90P\x80`\x02\x86`\xFF\x16`\x03\x81\x10a\x08SWa\x08Ra\x12\\V[[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x86\x86`\xFF\x16`\x03\x81\x10a\x08\x93Wa\x08\x92a\x12\\V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\xFF\x16\x7F\xD2\x0CoF\xFFO\x97t2\x15\x1B\xD6,\xF6[\x9F\x90\xDC~\x0CFc\xF8#\xB64\xCC\x94\xC0MU\x9E\x82`@Qa\x08\xF7\x91\x90a\x15qV[`@Q\x80\x91\x03\x90\xA2\x84`\x01\x01\x94PPPPPa\x07\0V[P`\x07`\x01`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\t0\x91\x90a\x15\x8CV[a\t:\x91\x90a\x15\xC4V[`\n`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPB`\n`\x18a\x01\0\n\x81T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\t\x8Ba\n;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xA9a\x04\xD9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x08Wa\t\xCCa\n;V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFF\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xFD[V[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\n8\x81a\r\xA1V[PV[`\x003\x90P\x90V[`\0\x80\x82\x03a\n\xF0W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\n\xE8\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x03_\xAF\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xE3\x91\x90a\x11\xCCV[a\x0EeV[\x91PPa\x0C=V[`\x01\x82\x03a\x0B\x9CW`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0B\x94\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE6\xAA!l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x8F\x91\x90a\x11\xCCV[a\x0EeV[\x91PPa\x0C<V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0C8\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\x8B,\xB6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C3\x91\x90a\x11\xCCV[a\x0EeV[\x91PP[[\x91\x90PV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x0C}W\x83\x82\x81a\x0CsWa\x0Cra\x14\xE0V[[\x04\x92PPPa\rBV[\x80\x84\x11a\x0C\xB6W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\r\x99W` \x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x90\x92\x91\x90a\x16?V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\x0E\xB9W`@\x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xB0\x92\x91\x90a\x16\xA3V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x0E\xD9\x81a\x0E\xC6V[\x81\x14a\x0E\xE4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xF6\x81a\x0E\xD0V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x13Wa\x0F\x12a\x0E\xC1V[[`\0a\x0F!\x85\x82\x86\x01a\x0E\xE7V[\x92PP` a\x0F2\x85\x82\x86\x01a\x0E\xE7V[\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0FY\x81a\x0F<V[\x82RPPV[`\0` \x82\x01\x90Pa\x0Ft`\0\x83\x01\x84a\x0FPV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0F\xA5\x82a\x0FzV[\x90P\x91\x90PV[a\x0F\xB5\x81a\x0F\x9AV[\x82RPPV[`\0` \x82\x01\x90Pa\x0F\xD0`\0\x83\x01\x84a\x0F\xACV[\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xEF\x81a\x0F\xD6V[\x82RPPV[`\0` \x82\x01\x90Pa\x10\n`\0\x83\x01\x84a\x0F\xE6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10&Wa\x10%a\x0E\xC1V[[`\0a\x104\x84\x82\x85\x01a\x0E\xE7V[\x91PP\x92\x91PPV[`\0a\x10H\x82a\x0F\x9AV[\x90P\x91\x90PV[a\x10X\x81a\x10=V[\x81\x14a\x10cW`\0\x80\xFD[PV[`\0\x815\x90Pa\x10u\x81a\x10OV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10\x91Wa\x10\x90a\x0E\xC1V[[`\0a\x10\x9F\x84\x82\x85\x01a\x10fV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x10\xCDa\x10\xC8a\x10\xC3\x84a\x0FzV[a\x10\xA8V[a\x0FzV[\x90P\x91\x90PV[`\0a\x10\xDF\x82a\x10\xB2V[\x90P\x91\x90PV[`\0a\x10\xF1\x82a\x10\xD4V[\x90P\x91\x90PV[a\x11\x01\x81a\x10\xE6V[\x82RPPV[`\0` \x82\x01\x90Pa\x11\x1C`\0\x83\x01\x84a\x10\xF8V[\x92\x91PPV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11=\x81a\x11\"V[\x82RPPV[`\0` \x82\x01\x90Pa\x11X`\0\x83\x01\x84a\x114V[\x92\x91PPV[a\x11g\x81a\x0F\x9AV[\x81\x14a\x11rW`\0\x80\xFD[PV[`\0\x815\x90Pa\x11\x84\x81a\x11^V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xA0Wa\x11\x9Fa\x0E\xC1V[[`\0a\x11\xAE\x84\x82\x85\x01a\x11uV[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa\x11\xC6\x81a\x0E\xD0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xE2Wa\x11\xE1a\x0E\xC1V[[`\0a\x11\xF0\x84\x82\x85\x01a\x11\xB7V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x123\x82a\x0E\xC6V[\x91Pa\x12>\x83a\x0E\xC6V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x12VWa\x12Ua\x11\xF9V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[`\0a\x12\xB3a\x12\xAEa\x12\xA9\x84a\x12\x8BV[a\x10\xA8V[a\x0E\xC6V[\x90P\x91\x90PV[a\x12\xC3\x81a\x12\x98V[\x82RPPV[`\0` \x82\x01\x90Pa\x12\xDE`\0\x83\x01\x84a\x12\xBAV[\x92\x91PPV[`\0a\x12\xEF\x82a\x0F<V[\x91Pa\x12\xFA\x83a\x0F<V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x1AWa\x13\x19a\x11\xF9V[[\x92\x91PPV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x13wW\x80\x86\x04\x81\x11\x15a\x13SWa\x13Ra\x11\xF9V[[`\x01\x85\x16\x15a\x13bW\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x13p\x85a\x13 V[\x94Pa\x137V[\x94P\x94\x92PPPV[`\0\x82a\x13\x90W`\x01\x90Pa\x14LV[\x81a\x13\x9EW`\0\x90Pa\x14LV[\x81`\x01\x81\x14a\x13\xB4W`\x02\x81\x14a\x13\xBEWa\x13\xEDV[`\x01\x91PPa\x14LV[`\xFF\x84\x11\x15a\x13\xD0Wa\x13\xCFa\x11\xF9V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x13\xE7Wa\x13\xE6a\x11\xF9V[[Pa\x14LV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x14\"W\x82\x82\n\x90P\x83\x81\x11\x15a\x14\x1DWa\x14\x1Ca\x11\xF9V[[a\x14LV[a\x14/\x84\x84\x84`\x01a\x13-V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x14FWa\x14Ea\x11\xF9V[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\x14^\x82a\x0E\xC6V[\x91Pa\x14i\x83a\x12\x8BV[\x92Pa\x14\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x13\x80V[\x90P\x92\x91PPV[`\0a\x14\xA9\x82a\x0E\xC6V[\x91Pa\x14\xB4\x83a\x0E\xC6V[\x92P\x82\x82\x02a\x14\xC2\x81a\x0E\xC6V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x14\xD9Wa\x14\xD8a\x11\xF9V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a\x15\x1A\x82a\x0E\xC6V[\x91Pa\x15%\x83a\x0E\xC6V[\x92P\x82a\x155Wa\x154a\x14\xE0V[[\x82\x82\x04\x90P\x92\x91PPV[`\0a\x15[a\x15Va\x15Q\x84a\x0F\xD6V[a\x10\xA8V[a\x0E\xC6V[\x90P\x91\x90PV[a\x15k\x81a\x15@V[\x82RPPV[`\0` \x82\x01\x90Pa\x15\x86`\0\x83\x01\x84a\x15bV[\x92\x91PPV[`\0a\x15\x97\x82a\x0F\xD6V[\x91Pa\x15\xA2\x83a\x0F\xD6V[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xBEWa\x15\xBDa\x11\xF9V[[\x92\x91PPV[`\0a\x15\xCF\x82a\x0F\xD6V[\x91Pa\x15\xDA\x83a\x0F\xD6V[\x92P\x82a\x15\xEAWa\x15\xE9a\x14\xE0V[[\x82\x82\x06\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x16\x1Aa\x16\x15a\x16\x10\x84a\x15\xF5V[a\x10\xA8V[a\x12\x8BV[\x90P\x91\x90PV[a\x16*\x81a\x15\xFFV[\x82RPPV[a\x169\x81a\x0E\xC6V[\x82RPPV[`\0`@\x82\x01\x90Pa\x16T`\0\x83\x01\x85a\x16!V[a\x16a` \x83\x01\x84a\x160V[\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x16\x8Da\x16\x88a\x16\x83\x84a\x16hV[a\x10\xA8V[a\x12\x8BV[\x90P\x91\x90PV[a\x16\x9D\x81a\x16rV[\x82RPPV[`\0`@\x82\x01\x90Pa\x16\xB8`\0\x83\x01\x85a\x16\x94V[a\x16\xC5` \x83\x01\x84a\x160V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x90\xE9\x9F%\x10\x9CXp\xE2\xC5\x15\xAB\t\xE9\x7F\x9F\xBCh?\xFF\xE5\x8A\x99 \x13\xCD\xB2\x07\x04\x1A{\x12dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static YIELDORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80cy\xBAP\x97\x11a\0\x97W\x80c\xB5\x1AO$\x11a\0fW\x80c\xB5\x1AO$\x14a\x02*W\x80c\xD0\xB0o]\x14a\x02HW\x80c\xE3\x0C9x\x14a\x02fW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x84Wa\0\xF5V[\x80cy\xBAP\x97\x14a\x01\xB6W\x80c\x81\xCE\x1C#\x14a\x01\xC0W\x80c\x8D\xA5\xCB[\x14a\x01\xF0W\x80c\xB1\xA0\xFE\xEF\x14a\x02\x0EWa\0\xF5V[\x80c&\x98{`\x11a\0\xD3W\x80c&\x98{`\x14a\x01fW\x80c1\xC9IG\x14a\x01\x84W\x80cS\xD7\x86\x93\x14a\x01\xA2W\x80cqP\x18\xA6\x14a\x01\xACWa\0\xF5V[\x80c\x04\xDEG\xE4\x14a\0\xFAW\x80c\x08Oci\x14a\x01*W\x80c#\x12\xC4\xBB\x14a\x01HW[`\0\x80\xFD[a\x01\x14`\x04\x806\x03\x81\x01\x90a\x01\x0F\x91\x90a\x0E\xFCV[a\x02\xA0V[`@Qa\x01!\x91\x90a\x0F_V[`@Q\x80\x91\x03\x90\xF3[a\x012a\x02\xE6V[`@Qa\x01?\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01Pa\x03\nV[`@Qa\x01]\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01na\x03.V[`@Qa\x01{\x91\x90a\x0F\xF5V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ca\x03DV[`@Qa\x01\x99\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01\xAAa\x03hV[\0[a\x01\xB4a\x04\x06V[\0[a\x01\xBEa\x04\x1AV[\0[a\x01\xDA`\x04\x806\x03\x81\x01\x90a\x01\xD5\x91\x90a\x10\x10V[a\x04\xA9V[`@Qa\x01\xE7\x91\x90a\x0F\xF5V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF8a\x04\xD9V[`@Qa\x02\x05\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x02(`\x04\x806\x03\x81\x01\x90a\x02#\x91\x90a\x10{V[a\x05\x02V[\0[a\x022a\x05NV[`@Qa\x02?\x91\x90a\x11\x07V[`@Q\x80\x91\x03\x90\xF3[a\x02Pa\x05tV[`@Qa\x02]\x91\x90a\x11CV[`@Q\x80\x91\x03\x90\xF3[a\x02na\x05\x8CV[`@Qa\x02{\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x02\x9E`\x04\x806\x03\x81\x01\x90a\x02\x99\x91\x90a\x11\x8AV[a\x05\xB6V[\0[`\x03\x82`\x07\x81\x10a\x02\xB0W`\0\x80\xFD[\x01\x81`\x03\x81\x10a\x02\xBFW`\0\x80\xFD[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA6\xAF\xED\x95`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFB\x91\x90a\x11\xCCV[Pa\x04\x04a\x06cV[V[a\x04\x0Ea\t\x83V[a\x04\x18`\0a\n\nV[V[`\0a\x04$a\n;V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04Ea\x05\x8CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x9DW\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x94\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xFD[a\x04\xA6\x81a\n\nV[PV[`\x02\x81`\x03\x81\x10a\x04\xB9W`\0\x80\xFD[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\na\t\x83V[\x80`\n`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[`\n`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\xBEa\t\x83V[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\x1Ea\x04\xD9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[Bb\x01Jx`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\x8F\x91\x90a\x12(V[\x11\x15a\x06\xC7W`@Q\x7F\xE5=\xFD\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x90P`\0`\x03\x82`\x07\x81\x10a\x06\xFAWa\x06\xF9a\x12\\V[[\x01\x90P`\0[`\x03c\xFF\xFF\xFF\xFF\x16\x81`\xFF\x16\x10\x15a\t\x0EW`\0a\x07 \x82`\xFF\x16a\nCV[\x90P`\0\x83\x83`\xFF\x16`\x03\x81\x10a\x07:Wa\x079a\x12\\V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x07\x88WP\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10[\x15a\x07\xCAW\x82`@Q\x7F\xD8\xD9\xAE\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xC1\x91\x90a\x12\xC9V[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x83a\x07\xD8\x91\x90a\x12\xE4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a\x089a\x084`\x07c\xFF\xFF\xFF\xFF\x16`\x08`\na\x08\x02\x91\x90a\x14SV[a\x01ma\x08\x0F\x91\x90a\x14\x9EV[a\x08\x19\x91\x90a\x15\x0FV[\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85a\x0CB\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\rIV[\x90P\x80`\x02\x86`\xFF\x16`\x03\x81\x10a\x08SWa\x08Ra\x12\\V[[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x86\x86`\xFF\x16`\x03\x81\x10a\x08\x93Wa\x08\x92a\x12\\V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\xFF\x16\x7F\xD2\x0CoF\xFFO\x97t2\x15\x1B\xD6,\xF6[\x9F\x90\xDC~\x0CFc\xF8#\xB64\xCC\x94\xC0MU\x9E\x82`@Qa\x08\xF7\x91\x90a\x15qV[`@Q\x80\x91\x03\x90\xA2\x84`\x01\x01\x94PPPPPa\x07\0V[P`\x07`\x01`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\t0\x91\x90a\x15\x8CV[a\t:\x91\x90a\x15\xC4V[`\n`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPB`\n`\x18a\x01\0\n\x81T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\t\x8Ba\n;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xA9a\x04\xD9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x08Wa\t\xCCa\n;V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFF\x91\x90a\x0F\xBBV[`@Q\x80\x91\x03\x90\xFD[V[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\n8\x81a\r\xA1V[PV[`\x003\x90P\x90V[`\0\x80\x82\x03a\n\xF0W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\n\xE8\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x03_\xAF\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xE3\x91\x90a\x11\xCCV[a\x0EeV[\x91PPa\x0C=V[`\x01\x82\x03a\x0B\x9CW`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0B\x94\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE6\xAA!l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x8F\x91\x90a\x11\xCCV[a\x0EeV[\x91PPa\x0C<V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0C8\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\x8B,\xB6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C3\x91\x90a\x11\xCCV[a\x0EeV[\x91PP[[\x91\x90PV[`\0\x80\x83\x85\x02\x90P`\0\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP`\0\x81\x03a\x0C}W\x83\x82\x81a\x0CsWa\x0Cra\x14\xE0V[[\x04\x92PPPa\rBV[\x80\x84\x11a\x0C\xB6W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P`\0\x85`\0\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82`\0\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P`\0`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\r\x99W` \x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x90\x92\x91\x90a\x16?V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\x0E\xB9W`@\x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xB0\x92\x91\x90a\x16\xA3V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x0E\xD9\x81a\x0E\xC6V[\x81\x14a\x0E\xE4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xF6\x81a\x0E\xD0V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x13Wa\x0F\x12a\x0E\xC1V[[`\0a\x0F!\x85\x82\x86\x01a\x0E\xE7V[\x92PP` a\x0F2\x85\x82\x86\x01a\x0E\xE7V[\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0FY\x81a\x0F<V[\x82RPPV[`\0` \x82\x01\x90Pa\x0Ft`\0\x83\x01\x84a\x0FPV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0F\xA5\x82a\x0FzV[\x90P\x91\x90PV[a\x0F\xB5\x81a\x0F\x9AV[\x82RPPV[`\0` \x82\x01\x90Pa\x0F\xD0`\0\x83\x01\x84a\x0F\xACV[\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xEF\x81a\x0F\xD6V[\x82RPPV[`\0` \x82\x01\x90Pa\x10\n`\0\x83\x01\x84a\x0F\xE6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10&Wa\x10%a\x0E\xC1V[[`\0a\x104\x84\x82\x85\x01a\x0E\xE7V[\x91PP\x92\x91PPV[`\0a\x10H\x82a\x0F\x9AV[\x90P\x91\x90PV[a\x10X\x81a\x10=V[\x81\x14a\x10cW`\0\x80\xFD[PV[`\0\x815\x90Pa\x10u\x81a\x10OV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10\x91Wa\x10\x90a\x0E\xC1V[[`\0a\x10\x9F\x84\x82\x85\x01a\x10fV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x10\xCDa\x10\xC8a\x10\xC3\x84a\x0FzV[a\x10\xA8V[a\x0FzV[\x90P\x91\x90PV[`\0a\x10\xDF\x82a\x10\xB2V[\x90P\x91\x90PV[`\0a\x10\xF1\x82a\x10\xD4V[\x90P\x91\x90PV[a\x11\x01\x81a\x10\xE6V[\x82RPPV[`\0` \x82\x01\x90Pa\x11\x1C`\0\x83\x01\x84a\x10\xF8V[\x92\x91PPV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11=\x81a\x11\"V[\x82RPPV[`\0` \x82\x01\x90Pa\x11X`\0\x83\x01\x84a\x114V[\x92\x91PPV[a\x11g\x81a\x0F\x9AV[\x81\x14a\x11rW`\0\x80\xFD[PV[`\0\x815\x90Pa\x11\x84\x81a\x11^V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xA0Wa\x11\x9Fa\x0E\xC1V[[`\0a\x11\xAE\x84\x82\x85\x01a\x11uV[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa\x11\xC6\x81a\x0E\xD0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xE2Wa\x11\xE1a\x0E\xC1V[[`\0a\x11\xF0\x84\x82\x85\x01a\x11\xB7V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x123\x82a\x0E\xC6V[\x91Pa\x12>\x83a\x0E\xC6V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x12VWa\x12Ua\x11\xF9V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[`\0a\x12\xB3a\x12\xAEa\x12\xA9\x84a\x12\x8BV[a\x10\xA8V[a\x0E\xC6V[\x90P\x91\x90PV[a\x12\xC3\x81a\x12\x98V[\x82RPPV[`\0` \x82\x01\x90Pa\x12\xDE`\0\x83\x01\x84a\x12\xBAV[\x92\x91PPV[`\0a\x12\xEF\x82a\x0F<V[\x91Pa\x12\xFA\x83a\x0F<V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x1AWa\x13\x19a\x11\xF9V[[\x92\x91PPV[`\0\x81`\x01\x1C\x90P\x91\x90PV[`\0\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x13wW\x80\x86\x04\x81\x11\x15a\x13SWa\x13Ra\x11\xF9V[[`\x01\x85\x16\x15a\x13bW\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x13p\x85a\x13 V[\x94Pa\x137V[\x94P\x94\x92PPPV[`\0\x82a\x13\x90W`\x01\x90Pa\x14LV[\x81a\x13\x9EW`\0\x90Pa\x14LV[\x81`\x01\x81\x14a\x13\xB4W`\x02\x81\x14a\x13\xBEWa\x13\xEDV[`\x01\x91PPa\x14LV[`\xFF\x84\x11\x15a\x13\xD0Wa\x13\xCFa\x11\xF9V[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x13\xE7Wa\x13\xE6a\x11\xF9V[[Pa\x14LV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x14\"W\x82\x82\n\x90P\x83\x81\x11\x15a\x14\x1DWa\x14\x1Ca\x11\xF9V[[a\x14LV[a\x14/\x84\x84\x84`\x01a\x13-V[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x14FWa\x14Ea\x11\xF9V[[\x81\x81\x02\x90P[\x93\x92PPPV[`\0a\x14^\x82a\x0E\xC6V[\x91Pa\x14i\x83a\x12\x8BV[\x92Pa\x14\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x13\x80V[\x90P\x92\x91PPV[`\0a\x14\xA9\x82a\x0E\xC6V[\x91Pa\x14\xB4\x83a\x0E\xC6V[\x92P\x82\x82\x02a\x14\xC2\x81a\x0E\xC6V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x14\xD9Wa\x14\xD8a\x11\xF9V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a\x15\x1A\x82a\x0E\xC6V[\x91Pa\x15%\x83a\x0E\xC6V[\x92P\x82a\x155Wa\x154a\x14\xE0V[[\x82\x82\x04\x90P\x92\x91PPV[`\0a\x15[a\x15Va\x15Q\x84a\x0F\xD6V[a\x10\xA8V[a\x0E\xC6V[\x90P\x91\x90PV[a\x15k\x81a\x15@V[\x82RPPV[`\0` \x82\x01\x90Pa\x15\x86`\0\x83\x01\x84a\x15bV[\x92\x91PPV[`\0a\x15\x97\x82a\x0F\xD6V[\x91Pa\x15\xA2\x83a\x0F\xD6V[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xBEWa\x15\xBDa\x11\xF9V[[\x92\x91PPV[`\0a\x15\xCF\x82a\x0F\xD6V[\x91Pa\x15\xDA\x83a\x0F\xD6V[\x92P\x82a\x15\xEAWa\x15\xE9a\x14\xE0V[[\x82\x82\x06\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x16\x1Aa\x16\x15a\x16\x10\x84a\x15\xF5V[a\x10\xA8V[a\x12\x8BV[\x90P\x91\x90PV[a\x16*\x81a\x15\xFFV[\x82RPPV[a\x169\x81a\x0E\xC6V[\x82RPPV[`\0`@\x82\x01\x90Pa\x16T`\0\x83\x01\x85a\x16!V[a\x16a` \x83\x01\x84a\x160V[\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x16\x8Da\x16\x88a\x16\x83\x84a\x16hV[a\x10\xA8V[a\x12\x8BV[\x90P\x91\x90PV[a\x16\x9D\x81a\x16rV[\x82RPPV[`\0`@\x82\x01\x90Pa\x16\xB8`\0\x83\x01\x85a\x16\x94V[a\x16\xC5` \x83\x01\x84a\x160V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x90\xE9\x9F%\x10\x9CXp\xE2\xC5\x15\xAB\t\xE9\x7F\x9F\xBCh?\xFF\xE5\x8A\x99 \x13\xCD\xB2\x07\x04\x1A{\x12dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static YIELDORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct YieldOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for YieldOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for YieldOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for YieldOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for YieldOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(YieldOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> YieldOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    YIELDORACLE_ABI.clone(),
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
                YIELDORACLE_ABI.clone(),
                YIELDORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ADDRESS0` (0x2312c4bb) function
        pub fn address0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([35, 18, 196, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ADDRESS1` (0x31c94947) function
        pub fn address1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([49, 201, 73, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ADDRESS2` (0x084f6369) function
        pub fn address2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 79, 99, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `apys` (0x81ce1c23) function
        pub fn apys(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([129, 206, 28, 35], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentIndex` (0x26987b60) function
        pub fn current_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([38, 152, 123, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `historicalExchangeRates` (0x04de47e4) function
        pub fn historical_exchange_rates(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([4, 222, 71, 228], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ionPool` (0xb51a4f24) function
        pub fn ion_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([181, 26, 79, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdated` (0xd0b06f5d) function
        pub fn last_updated(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([208, 176, 111, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingOwner` (0xe30c3978) function
        pub fn pending_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateAll` (0x53d78693) function
        pub fn update_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 215, 134, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateIonPool` (0xb1a0feef) function
        pub fn update_ion_pool(
            &self,
            ion_pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 160, 254, 239], ion_pool)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ApyUpdate` event
        pub fn apy_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApyUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferStarted` event
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferStartedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            YieldOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for YieldOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyUpdated` with signature `AlreadyUpdated()` and selector `0xe53dfd0a`
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
    #[etherror(name = "AlreadyUpdated", abi = "AlreadyUpdated()")]
    pub struct AlreadyUpdated;
    ///Custom Error type `InvalidExchangeRate` with signature `InvalidExchangeRate(uint256)` and selector `0xd8d9aee7`
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
    #[etherror(name = "InvalidExchangeRate", abi = "InvalidExchangeRate(uint256)")]
    pub struct InvalidExchangeRate {
        pub ilk_index: ::ethers::core::types::U256,
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
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
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
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
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
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `SafeCastOverflowedUintDowncast` with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`
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
        name = "SafeCastOverflowedUintDowncast",
        abi = "SafeCastOverflowedUintDowncast(uint8,uint256)"
    )]
    pub struct SafeCastOverflowedUintDowncast {
        pub bits: u8,
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
    pub enum YieldOracleErrors {
        AlreadyUpdated(AlreadyUpdated),
        InvalidExchangeRate(InvalidExchangeRate),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for YieldOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AlreadyUpdated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AlreadyUpdated(decoded));
            }
            if let Ok(decoded) = <InvalidExchangeRate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidExchangeRate(decoded));
            }
            if let Ok(decoded) = <MathOverflowedMulDiv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MathOverflowedMulDiv(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <SafeCastOverflowedUintDowncast as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeCastOverflowedUintDowncast(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for YieldOracleErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidExchangeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastOverflowedUintDowncast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for YieldOracleErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadyUpdated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidExchangeRate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MathOverflowedMulDiv as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeCastOverflowedUintDowncast as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for YieldOracleErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyUpdated(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidExchangeRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MathOverflowedMulDiv(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeCastOverflowedUintDowncast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for YieldOracleErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyUpdated> for YieldOracleErrors {
        fn from(value: AlreadyUpdated) -> Self {
            Self::AlreadyUpdated(value)
        }
    }
    impl ::core::convert::From<InvalidExchangeRate> for YieldOracleErrors {
        fn from(value: InvalidExchangeRate) -> Self {
            Self::InvalidExchangeRate(value)
        }
    }
    impl ::core::convert::From<MathOverflowedMulDiv> for YieldOracleErrors {
        fn from(value: MathOverflowedMulDiv) -> Self {
            Self::MathOverflowedMulDiv(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for YieldOracleErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for YieldOracleErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflowedUintDowncast> for YieldOracleErrors {
        fn from(value: SafeCastOverflowedUintDowncast) -> Self {
            Self::SafeCastOverflowedUintDowncast(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ApyUpdate", abi = "ApyUpdate(uint256,uint256)")]
    pub struct ApyUpdateFilter {
        #[ethevent(indexed)]
        pub ilk_index: ::ethers::core::types::U256,
        pub new_apy: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferStarted",
        abi = "OwnershipTransferStarted(address,address)"
    )]
    pub struct OwnershipTransferStartedFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
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
    pub enum YieldOracleEvents {
        ApyUpdateFilter(ApyUpdateFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for YieldOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApyUpdateFilter::decode_log(log) {
                return Ok(YieldOracleEvents::ApyUpdateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(YieldOracleEvents::OwnershipTransferStartedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(YieldOracleEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for YieldOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApyUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApyUpdateFilter> for YieldOracleEvents {
        fn from(value: ApyUpdateFilter) -> Self {
            Self::ApyUpdateFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter> for YieldOracleEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for YieldOracleEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `ADDRESS0` function with signature `ADDRESS0()` and selector `0x2312c4bb`
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
    #[ethcall(name = "ADDRESS0", abi = "ADDRESS0()")]
    pub struct Address0Call;
    ///Container type for all input parameters for the `ADDRESS1` function with signature `ADDRESS1()` and selector `0x31c94947`
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
    #[ethcall(name = "ADDRESS1", abi = "ADDRESS1()")]
    pub struct Address1Call;
    ///Container type for all input parameters for the `ADDRESS2` function with signature `ADDRESS2()` and selector `0x084f6369`
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
    #[ethcall(name = "ADDRESS2", abi = "ADDRESS2()")]
    pub struct Address2Call;
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `apys` function with signature `apys(uint256)` and selector `0x81ce1c23`
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
    #[ethcall(name = "apys", abi = "apys(uint256)")]
    pub struct ApysCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `currentIndex` function with signature `currentIndex()` and selector `0x26987b60`
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
    #[ethcall(name = "currentIndex", abi = "currentIndex()")]
    pub struct CurrentIndexCall;
    ///Container type for all input parameters for the `historicalExchangeRates` function with signature `historicalExchangeRates(uint256,uint256)` and selector `0x04de47e4`
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
        name = "historicalExchangeRates",
        abi = "historicalExchangeRates(uint256,uint256)"
    )]
    pub struct HistoricalExchangeRatesCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `ionPool` function with signature `ionPool()` and selector `0xb51a4f24`
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
    #[ethcall(name = "ionPool", abi = "ionPool()")]
    pub struct IonPoolCall;
    ///Container type for all input parameters for the `lastUpdated` function with signature `lastUpdated()` and selector `0xd0b06f5d`
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
    #[ethcall(name = "lastUpdated", abi = "lastUpdated()")]
    pub struct LastUpdatedCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateAll` function with signature `updateAll()` and selector `0x53d78693`
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
    #[ethcall(name = "updateAll", abi = "updateAll()")]
    pub struct UpdateAllCall;
    ///Container type for all input parameters for the `updateIonPool` function with signature `updateIonPool(address)` and selector `0xb1a0feef`
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
    #[ethcall(name = "updateIonPool", abi = "updateIonPool(address)")]
    pub struct UpdateIonPoolCall {
        pub ion_pool: ::ethers::core::types::Address,
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
    pub enum YieldOracleCalls {
        Address0(Address0Call),
        Address1(Address1Call),
        Address2(Address2Call),
        AcceptOwnership(AcceptOwnershipCall),
        Apys(ApysCall),
        CurrentIndex(CurrentIndexCall),
        HistoricalExchangeRates(HistoricalExchangeRatesCall),
        IonPool(IonPoolCall),
        LastUpdated(LastUpdatedCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateAll(UpdateAllCall),
        UpdateIonPool(UpdateIonPoolCall),
    }
    impl ::ethers::core::abi::AbiDecode for YieldOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Address0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Address0(decoded));
            }
            if let Ok(decoded) = <Address1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Address1(decoded));
            }
            if let Ok(decoded) = <Address2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Address2(decoded));
            }
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <ApysCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Apys(decoded));
            }
            if let Ok(decoded) = <CurrentIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentIndex(decoded));
            }
            if let Ok(decoded) = <HistoricalExchangeRatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HistoricalExchangeRates(decoded));
            }
            if let Ok(decoded) = <IonPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IonPool(decoded));
            }
            if let Ok(decoded) = <LastUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastUpdated(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PendingOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingOwner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateAll(decoded));
            }
            if let Ok(decoded) = <UpdateIonPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateIonPool(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for YieldOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Address0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Address1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Address2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Apys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CurrentIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HistoricalExchangeRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IonPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateIonPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for YieldOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Address0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Address1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Address2(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Apys(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::HistoricalExchangeRates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IonPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastUpdated(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateIonPool(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Address0Call> for YieldOracleCalls {
        fn from(value: Address0Call) -> Self {
            Self::Address0(value)
        }
    }
    impl ::core::convert::From<Address1Call> for YieldOracleCalls {
        fn from(value: Address1Call) -> Self {
            Self::Address1(value)
        }
    }
    impl ::core::convert::From<Address2Call> for YieldOracleCalls {
        fn from(value: Address2Call) -> Self {
            Self::Address2(value)
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for YieldOracleCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<ApysCall> for YieldOracleCalls {
        fn from(value: ApysCall) -> Self {
            Self::Apys(value)
        }
    }
    impl ::core::convert::From<CurrentIndexCall> for YieldOracleCalls {
        fn from(value: CurrentIndexCall) -> Self {
            Self::CurrentIndex(value)
        }
    }
    impl ::core::convert::From<HistoricalExchangeRatesCall> for YieldOracleCalls {
        fn from(value: HistoricalExchangeRatesCall) -> Self {
            Self::HistoricalExchangeRates(value)
        }
    }
    impl ::core::convert::From<IonPoolCall> for YieldOracleCalls {
        fn from(value: IonPoolCall) -> Self {
            Self::IonPool(value)
        }
    }
    impl ::core::convert::From<LastUpdatedCall> for YieldOracleCalls {
        fn from(value: LastUpdatedCall) -> Self {
            Self::LastUpdated(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for YieldOracleCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for YieldOracleCalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for YieldOracleCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for YieldOracleCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateAllCall> for YieldOracleCalls {
        fn from(value: UpdateAllCall) -> Self {
            Self::UpdateAll(value)
        }
    }
    impl ::core::convert::From<UpdateIonPoolCall> for YieldOracleCalls {
        fn from(value: UpdateIonPoolCall) -> Self {
            Self::UpdateIonPool(value)
        }
    }
    ///Container type for all return fields from the `ADDRESS0` function with signature `ADDRESS0()` and selector `0x2312c4bb`
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
    pub struct Address0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ADDRESS1` function with signature `ADDRESS1()` and selector `0x31c94947`
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
    pub struct Address1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ADDRESS2` function with signature `ADDRESS2()` and selector `0x084f6369`
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
    pub struct Address2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `apys` function with signature `apys(uint256)` and selector `0x81ce1c23`
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
    pub struct ApysReturn(pub u32);
    ///Container type for all return fields from the `currentIndex` function with signature `currentIndex()` and selector `0x26987b60`
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
    pub struct CurrentIndexReturn(pub u32);
    ///Container type for all return fields from the `historicalExchangeRates` function with signature `historicalExchangeRates(uint256,uint256)` and selector `0x04de47e4`
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
    pub struct HistoricalExchangeRatesReturn(pub u64);
    ///Container type for all return fields from the `ionPool` function with signature `ionPool()` and selector `0xb51a4f24`
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
    pub struct IonPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `lastUpdated` function with signature `lastUpdated()` and selector `0xd0b06f5d`
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
    pub struct LastUpdatedReturn(pub u64);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    pub struct PendingOwnerReturn(pub ::ethers::core::types::Address);
}
