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
                        name: ::std::borrow::ToOwned::to_owned("_weEth"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidIlkIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidIlkIndex"),
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
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x10W_\x80\xFD[P`@Qb\0+\xAF8\x03\x80b\0+\xAF\x839\x81\x81\x01`@R\x81\x01\x90b\0\x006\x91\x90b\0\r\x97V[\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\xAAW_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0\xA1\x91\x90b\0\x0E1V[`@Q\x80\x91\x03\x90\xFD[b\0\0\xBB\x81b\0\x02\xC9` \x1B` \x1CV[P_[`\x07c\xFF\xFF\xFF\xFF\x16\x81\x10\x15b\0\x02\x11W_[`\x03c\xFF\xFF\xFF\xFF\x16\x81\x10\x15b\0\x02\x04W_\x87\x83`\x07\x81\x10b\0\0\xF7Wb\0\0\xF6b\0\x0ELV[[` \x02\x01Q\x82`\x03\x81\x10b\0\x01\x11Wb\0\x01\x10b\0\x0ELV[[` \x02\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\x01eW\x80`@Q\x7F\xD8\xD9\xAE\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x01\\\x91\x90b\0\x0E\x93V[`@Q\x80\x91\x03\x90\xFD[\x86\x82`\x07\x81\x10b\0\x01{Wb\0\x01zb\0\x0ELV[[` \x02\x01Q\x81`\x03\x81\x10b\0\x01\x95Wb\0\x01\x94b\0\x0ELV[[` \x02\x01Q`\x03\x83`\x07\x81\x10b\0\x01\xB1Wb\0\x01\xB0b\0\x0ELV[[\x01\x82`\x03\x81\x10b\0\x01\xC7Wb\0\x01\xC6b\0\x0ELV[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01\x01\x90Pb\0\0\xD0V[P\x80`\x01\x01\x90Pb\0\0\xBEV[P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPb\0\x02\xBEb\0\x03\x01` \x1B` \x1CV[PPPPPb\0\x13\xD9V[`\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ub\0\x02\xFE\x81b\0\x06@` \x1B` \x1CV[PV[Bb\x01Jx`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\x03/\x91\x90b\0\x0E\xDBV[\x11\x15b\0\x03hW`@Q\x7F\xE5=\xFD\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x90P_`\x03\x82`\x07\x81\x10b\0\x03\x9CWb\0\x03\x9Bb\0\x0ELV[[\x01\x90P_[`\x03c\xFF\xFF\xFF\xFF\x16\x81`\xFF\x16\x10\x15b\0\x05\xC7W_b\0\x03\xC9\x82`\xFF\x16b\0\x07\x01` \x1B` \x1CV[\x90P_\x83\x83`\xFF\x16`\x03\x81\x10b\0\x03\xE5Wb\0\x03\xE4b\0\x0ELV[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\x04XW\x82`@Q\x7F\xD8\xD9\xAE\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x04O\x91\x90b\0\x0FbV[`@Q\x80\x91\x03\x90\xFD[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10b\0\x04\xECW_\x82\x84\x03g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pb\0\x04\xE8b\0\x04\xDC`\x07c\xFF\xFF\xFF\xFF\x16`\x08`\nb\0\x04\xA5\x91\x90b\0\x10\xCDV[a\x01mb\0\x04\xB4\x91\x90b\0\x11\x1DV[b\0\x04\xC0\x91\x90b\0\x11\x94V[\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84b\0\t\x0F` \x1B\x90\x92\x91\x90` \x1CV[b\0\n\x14` \x1B` \x1CV[\x91PP[\x80`\x02\x85`\xFF\x16`\x03\x81\x10b\0\x05\x07Wb\0\x05\x06b\0\x0ELV[[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82\x85\x85`\xFF\x16`\x03\x81\x10b\0\x05JWb\0\x05Ib\0\x0ELV[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\xFF\x16\x7F\xD2\x0CoF\xFFO\x97t2\x15\x1B\xD6,\xF6[\x9F\x90\xDC~\x0CFc\xF8#\xB64\xCC\x94\xC0MU\x9E\x82`@Qb\0\x05\xB0\x91\x90b\0\x12\x12V[`@Q\x80\x91\x03\x90\xA2\x83`\x01\x01\x93PPPPb\0\x03\xA1V[P`\x07`\x01`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16b\0\x05\xEB\x91\x90b\0\x12-V[b\0\x05\xF7\x91\x90b\0\x12kV[`\n`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPB`\n`\x18a\x01\0\n\x81T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_\x80\x82\x03b\0\x07\x99W_`\x80Q\x90Pb\0\x07\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cg\x9A\xEF\xCE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x07^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x07\x84\x91\x90b\0\x12\xD1V[b\0\nn` \x1B` \x1CV[\x91PPb\0\t\nV[`\x01\x82\x03b\0\x081W_`\xA0Q\x90Pb\0\x08(\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE6\xAA!l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x07\xF6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x08\x1C\x91\x90b\0\x12\xD1V[b\0\nn` \x1B` \x1CV[\x91PPb\0\t\tV[`\x02\x82\x03b\0\x08\xC9W_`\xC0Q\x90Pb\0\x08\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\x8B,\xB6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x08\x8EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x08\xB4\x91\x90b\0\x12\xD1V[b\0\nn` \x1B` \x1CV[\x91PPb\0\t\x08V[\x81`@Q\x7F\x13\xC3eh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x08\xFF\x91\x90b\0\x0E\x93V[`@Q\x80\x91\x03\x90\xFD[[[\x91\x90PV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03b\0\tLW\x83\x82\x81b\0\tAWb\0\t@b\0\x11gV[[\x04\x92PPPb\0\n\rV[\x80\x84\x11b\0\t\x86W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_c\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15b\0\nfW` \x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\n]\x92\x91\x90b\0\x13BV[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15b\0\n\xC4W`@\x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\n\xBB\x92\x91\x90b\0\x13\xAEV[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[b\0\x0B%\x82b\0\n\xDDV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x0BGWb\0\x0BFb\0\n\xEDV[[\x80`@RPPPV[_b\0\x0B[b\0\n\xCCV[\x90Pb\0\x0Bi\x82\x82b\0\x0B\x1AV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x0B\x8BWb\0\x0B\x8Ab\0\n\xEDV[[` \x82\x02\x90P\x91\x90PV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x0B\xB7Wb\0\x0B\xB6b\0\n\xEDV[[` \x82\x02\x90P\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x0B\xE0\x81b\0\x0B\xC2V[\x81\x14b\0\x0B\xEBW_\x80\xFD[PV[_\x81Q\x90Pb\0\x0B\xFE\x81b\0\x0B\xD5V[\x92\x91PPV[_b\0\x0C\x1Ab\0\x0C\x14\x84b\0\x0B\x9AV[b\0\x0BPV[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x0C7Wb\0\x0C6b\0\x0B\x96V[[\x83[\x81\x81\x10\x15b\0\x0CdW\x80b\0\x0CO\x88\x82b\0\x0B\xEEV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x0C9V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12b\0\x0C\x85Wb\0\x0C\x84b\0\n\xD9V[[`\x03b\0\x0C\x94\x84\x82\x85b\0\x0C\x04V[\x91PP\x92\x91PPV[_b\0\x0C\xB3b\0\x0C\xAD\x84b\0\x0BnV[b\0\x0BPV[\x90P\x80``\x84\x02\x83\x01\x85\x81\x11\x15b\0\x0C\xD0Wb\0\x0C\xCFb\0\x0B\x96V[[\x83[\x81\x81\x10\x15b\0\x0C\xFDW\x80b\0\x0C\xE8\x88\x82b\0\x0CnV[\x84R` \x84\x01\x93PP``\x81\x01\x90Pb\0\x0C\xD2V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12b\0\r\x1EWb\0\r\x1Db\0\n\xD9V[[`\x07b\0\r-\x84\x82\x85b\0\x0C\x9DV[\x91PP\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\ra\x82b\0\r6V[\x90P\x91\x90PV[b\0\rs\x81b\0\rUV[\x81\x14b\0\r~W_\x80\xFD[PV[_\x81Q\x90Pb\0\r\x91\x81b\0\rhV[\x92\x91PPV[_\x80_\x80_a\x03 \x86\x88\x03\x12\x15b\0\r\xB4Wb\0\r\xB3b\0\n\xD5V[[_b\0\r\xC3\x88\x82\x89\x01b\0\r\x07V[\x95PPa\x02\xA0b\0\r\xD7\x88\x82\x89\x01b\0\r\x81V[\x94PPa\x02\xC0b\0\r\xEB\x88\x82\x89\x01b\0\r\x81V[\x93PPa\x02\xE0b\0\r\xFF\x88\x82\x89\x01b\0\r\x81V[\x92PPa\x03\0b\0\x0E\x13\x88\x82\x89\x01b\0\r\x81V[\x91PP\x92\x95P\x92\x95\x90\x93PV[b\0\x0E+\x81b\0\rUV[\x82RPPV[_` \x82\x01\x90Pb\0\x0EF_\x83\x01\x84b\0\x0E V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[b\0\x0E\x8D\x81b\0\x0EyV[\x82RPPV[_` \x82\x01\x90Pb\0\x0E\xA8_\x83\x01\x84b\0\x0E\x82V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_b\0\x0E\xE7\x82b\0\x0EyV[\x91Pb\0\x0E\xF4\x83b\0\x0EyV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15b\0\x0F\x0FWb\0\x0F\x0Eb\0\x0E\xAEV[[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_b\0\x0FJb\0\x0FDb\0\x0F>\x84b\0\x0F\x15V[b\0\x0F!V[b\0\x0EyV[\x90P\x91\x90PV[b\0\x0F\\\x81b\0\x0F*V[\x82RPPV[_` \x82\x01\x90Pb\0\x0Fw_\x83\x01\x84b\0\x0FQV[\x92\x91PPV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15b\0\x0F\xDAW\x80\x86\x04\x81\x11\x15b\0\x0F\xB2Wb\0\x0F\xB1b\0\x0E\xAEV[[`\x01\x85\x16\x15b\0\x0F\xC2W\x80\x82\x02\x91P[\x80\x81\x02\x90Pb\0\x0F\xD2\x85b\0\x0F}V[\x94Pb\0\x0F\x92V[\x94P\x94\x92PPPV[_\x82b\0\x0F\xF4W`\x01\x90Pb\0\x10\xC6V[\x81b\0\x10\x03W_\x90Pb\0\x10\xC6V[\x81`\x01\x81\x14b\0\x10\x1CW`\x02\x81\x14b\0\x10'Wb\0\x10]V[`\x01\x91PPb\0\x10\xC6V[`\xFF\x84\x11\x15b\0\x10<Wb\0\x10;b\0\x0E\xAEV[[\x83`\x02\n\x91P\x84\x82\x11\x15b\0\x10VWb\0\x10Ub\0\x0E\xAEV[[Pb\0\x10\xC6V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\0\x10\x97W\x82\x82\n\x90P\x83\x81\x11\x15b\0\x10\x91Wb\0\x10\x90b\0\x0E\xAEV[[b\0\x10\xC6V[b\0\x10\xA6\x84\x84\x84`\x01b\0\x0F\x89V[\x92P\x90P\x81\x84\x04\x81\x11\x15b\0\x10\xC0Wb\0\x10\xBFb\0\x0E\xAEV[[\x81\x81\x02\x90P[\x93\x92PPPV[_b\0\x10\xD9\x82b\0\x0EyV[\x91Pb\0\x10\xE6\x83b\0\x0F\x15V[\x92Pb\0\x11\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84b\0\x0F\xE3V[\x90P\x92\x91PPV[_b\0\x11)\x82b\0\x0EyV[\x91Pb\0\x116\x83b\0\x0EyV[\x92P\x82\x82\x02b\0\x11F\x81b\0\x0EyV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17b\0\x11`Wb\0\x11_b\0\x0E\xAEV[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_b\0\x11\xA0\x82b\0\x0EyV[\x91Pb\0\x11\xAD\x83b\0\x0EyV[\x92P\x82b\0\x11\xC0Wb\0\x11\xBFb\0\x11gV[[\x82\x82\x04\x90P\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\x11\xFAb\0\x11\xF4b\0\x11\xEE\x84b\0\x11\xCBV[b\0\x0F!V[b\0\x0EyV[\x90P\x91\x90PV[b\0\x12\x0C\x81b\0\x11\xDAV[\x82RPPV[_` \x82\x01\x90Pb\0\x12'_\x83\x01\x84b\0\x12\x01V[\x92\x91PPV[_b\0\x129\x82b\0\x11\xCBV[\x91Pb\0\x12F\x83b\0\x11\xCBV[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x12eWb\0\x12db\0\x0E\xAEV[[\x92\x91PPV[_b\0\x12w\x82b\0\x11\xCBV[\x91Pb\0\x12\x84\x83b\0\x11\xCBV[\x92P\x82b\0\x12\x97Wb\0\x12\x96b\0\x11gV[[\x82\x82\x06\x90P\x92\x91PPV[b\0\x12\xAD\x81b\0\x0EyV[\x81\x14b\0\x12\xB8W_\x80\xFD[PV[_\x81Q\x90Pb\0\x12\xCB\x81b\0\x12\xA2V[\x92\x91PPV[_` \x82\x84\x03\x12\x15b\0\x12\xE9Wb\0\x12\xE8b\0\n\xD5V[[_b\0\x12\xF8\x84\x82\x85\x01b\0\x12\xBBV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_b\0\x13*b\0\x13$b\0\x13\x1E\x84b\0\x13\x01V[b\0\x0F!V[b\0\x0F\x15V[\x90P\x91\x90PV[b\0\x13<\x81b\0\x13\nV[\x82RPPV[_`@\x82\x01\x90Pb\0\x13W_\x83\x01\x85b\0\x131V[b\0\x13f` \x83\x01\x84b\0\x0E\x82V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_b\0\x13\x96b\0\x13\x90b\0\x13\x8A\x84b\0\x13mV[b\0\x0F!V[b\0\x0F\x15V[\x90P\x91\x90PV[b\0\x13\xA8\x81b\0\x13vV[\x82RPPV[_`@\x82\x01\x90Pb\0\x13\xC3_\x83\x01\x85b\0\x13\x9DV[b\0\x13\xD2` \x83\x01\x84b\0\x0E\x82V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Qa\x17\x96b\0\x14\x19_9_\x81\x81a\x02\xE3\x01Ra\x0C\n\x01R_\x81\x81a\x03A\x01Ra\x0Ba\x01R_\x81\x81a\x03\x07\x01Ra\n\xB8\x01Ra\x17\x96_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xF3W_5`\xE0\x1C\x80cy\xBAP\x97\x11a\0\x95W\x80c\xB5\x1AO$\x11a\0dW\x80c\xB5\x1AO$\x14a\x02'W\x80c\xD0\xB0o]\x14a\x02EW\x80c\xE3\x0C9x\x14a\x02cW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x81Wa\0\xF3V[\x80cy\xBAP\x97\x14a\x01\xB3W\x80c\x81\xCE\x1C#\x14a\x01\xBDW\x80c\x8D\xA5\xCB[\x14a\x01\xEDW\x80c\xB1\xA0\xFE\xEF\x14a\x02\x0BWa\0\xF3V[\x80c&\x98{`\x11a\0\xD1W\x80c&\x98{`\x14a\x01cW\x80c1\xC9IG\x14a\x01\x81W\x80cS\xD7\x86\x93\x14a\x01\x9FW\x80cqP\x18\xA6\x14a\x01\xA9Wa\0\xF3V[\x80c\x04\xDEG\xE4\x14a\0\xF7W\x80c\x08Oci\x14a\x01'W\x80c#\x12\xC4\xBB\x14a\x01EW[_\x80\xFD[a\x01\x11`\x04\x806\x03\x81\x01\x90a\x01\x0C\x91\x90a\x0F\x95V[a\x02\x9DV[`@Qa\x01\x1E\x91\x90a\x0F\xF5V[`@Q\x80\x91\x03\x90\xF3[a\x01/a\x02\xE1V[`@Qa\x01<\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x01Ma\x03\x05V[`@Qa\x01Z\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x01ka\x03)V[`@Qa\x01x\x91\x90a\x10\x84V[`@Q\x80\x91\x03\x90\xF3[a\x01\x89a\x03?V[`@Qa\x01\x96\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x01\xA7a\x03cV[\0[a\x01\xB1a\x04\x90V[\0[a\x01\xBBa\x04\xA3V[\0[a\x01\xD7`\x04\x806\x03\x81\x01\x90a\x01\xD2\x91\x90a\x10\x9DV[a\x051V[`@Qa\x01\xE4\x91\x90a\x10\x84V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF5a\x05`V[`@Qa\x02\x02\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x02%`\x04\x806\x03\x81\x01\x90a\x02 \x91\x90a\x11\x03V[a\x05\x87V[\0[a\x02/a\x05\xD2V[`@Qa\x02<\x91\x90a\x11\x89V[`@Q\x80\x91\x03\x90\xF3[a\x02Ma\x05\xF7V[`@Qa\x02Z\x91\x90a\x11\xC2V[`@Q\x80\x91\x03\x90\xF3[a\x02ka\x06\x0FV[`@Qa\x02x\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x02\x9B`\x04\x806\x03\x81\x01\x90a\x02\x96\x91\x90a\x12\x05V[a\x067V[\0[`\x03\x82`\x07\x81\x10a\x02\xACW_\x80\xFD[\x01\x81`\x03\x81\x10a\x02\xBAW_\x80\xFD[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF1\x91\x90a\x12eV[a\x04\x86W`\n_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA6\xAF\xED\x95`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x84\x91\x90a\x12\xA4V[P[a\x04\x8Ea\x06\xE3V[V[a\x04\x98a\t\xEFV[a\x04\xA1_a\nvV[V[_a\x04\xACa\n\xA6V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\xCDa\x06\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05%W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x1C\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xFD[a\x05.\x81a\nvV[PV[`\x02\x81`\x03\x81\x10a\x05@W_\x80\xFD[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x8Fa\t\xEFV[\x80`\n_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[`\n_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x06?a\t\xEFV[\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\x9Ea\x05`V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[Bb\x01Jx`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x0F\x91\x90a\x12\xFCV[\x11\x15a\x07GW`@Q\x7F\xE5=\xFD\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x90P_`\x03\x82`\x07\x81\x10a\x07xWa\x07wa\x13/V[[\x01\x90P_[`\x03c\xFF\xFF\xFF\xFF\x16\x81`\xFF\x16\x10\x15a\tzW_a\x07\x9C\x82`\xFF\x16a\n\xADV[\x90P_\x83\x83`\xFF\x16`\x03\x81\x10a\x07\xB5Wa\x07\xB4a\x13/V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x08%W\x82`@Q\x7F\xD8\xD9\xAE\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x1C\x91\x90a\x13\x98V[`@Q\x80\x91\x03\x90\xFD[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x08\xA8W_\x82\x84\x03g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x08\xA4a\x08\x9F`\x07c\xFF\xFF\xFF\xFF\x16`\x08`\na\x08m\x91\x90a\x14\xE0V[a\x01ma\x08z\x91\x90a\x15*V[a\x08\x84\x91\x90a\x15\x98V[\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84a\x0C\xEC\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xEBV[\x91PP[\x80`\x02\x85`\xFF\x16`\x03\x81\x10a\x08\xC0Wa\x08\xBFa\x13/V[[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82\x85\x85`\xFF\x16`\x03\x81\x10a\t\0Wa\x08\xFFa\x13/V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\xFF\x16\x7F\xD2\x0CoF\xFFO\x97t2\x15\x1B\xD6,\xF6[\x9F\x90\xDC~\x0CFc\xF8#\xB64\xCC\x94\xC0MU\x9E\x82`@Qa\td\x91\x90a\x15\xF8V[`@Q\x80\x91\x03\x90\xA2\x83`\x01\x01\x93PPPPa\x07}V[P`\x07`\x01`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\t\x9C\x91\x90a\x16\x11V[a\t\xA6\x91\x90a\x16HV[`\n`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPB`\n`\x18a\x01\0\n\x81T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\t\xF7a\n\xA6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\x15a\x05`V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\ntWa\n8a\n\xA6V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nk\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xFD[V[`\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\n\xA3\x81a\x0EBV[PV[_3\x90P\x90V[_\x80\x82\x03a\x0BVW_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0BN\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cg\x9A\xEF\xCE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B%W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BI\x91\x90a\x12\xA4V[a\x0F\x03V[\x91PPa\x0C\xE7V[`\x01\x82\x03a\x0B\xFFW_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0B\xF7\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE6\xAA!l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xCEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF2\x91\x90a\x12\xA4V[a\x0F\x03V[\x91PPa\x0C\xE6V[`\x02\x82\x03a\x0C\xA8W_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0C\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\x8B,\xB6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CwW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9B\x91\x90a\x12\xA4V[a\x0F\x03V[\x91PPa\x0C\xE5V[\x81`@Q\x7F\x13\xC3eh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xDC\x91\x90a\x16\x87V[`@Q\x80\x91\x03\x90\xFD[[[\x91\x90PV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\r$W\x83\x82\x81a\r\x1AWa\r\x19a\x15kV[[\x04\x92PPPa\r\xE4V[\x80\x84\x11a\r]W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_c\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\x0E:W` \x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E1\x92\x91\x90a\x16\xD9V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\x0FVW`@\x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0FM\x92\x91\x90a\x179V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x0Ft\x81a\x0FbV[\x81\x14a\x0F~W_\x80\xFD[PV[_\x815\x90Pa\x0F\x8F\x81a\x0FkV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x0F\xABWa\x0F\xAAa\x0F^V[[_a\x0F\xB8\x85\x82\x86\x01a\x0F\x81V[\x92PP` a\x0F\xC9\x85\x82\x86\x01a\x0F\x81V[\x91PP\x92P\x92\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xEF\x81a\x0F\xD3V[\x82RPPV[_` \x82\x01\x90Pa\x10\x08_\x83\x01\x84a\x0F\xE6V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x107\x82a\x10\x0EV[\x90P\x91\x90PV[a\x10G\x81a\x10-V[\x82RPPV[_` \x82\x01\x90Pa\x10`_\x83\x01\x84a\x10>V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10~\x81a\x10fV[\x82RPPV[_` \x82\x01\x90Pa\x10\x97_\x83\x01\x84a\x10uV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10\xB2Wa\x10\xB1a\x0F^V[[_a\x10\xBF\x84\x82\x85\x01a\x0F\x81V[\x91PP\x92\x91PPV[_a\x10\xD2\x82a\x10-V[\x90P\x91\x90PV[a\x10\xE2\x81a\x10\xC8V[\x81\x14a\x10\xECW_\x80\xFD[PV[_\x815\x90Pa\x10\xFD\x81a\x10\xD9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11\x18Wa\x11\x17a\x0F^V[[_a\x11%\x84\x82\x85\x01a\x10\xEFV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x11Qa\x11La\x11G\x84a\x10\x0EV[a\x11.V[a\x10\x0EV[\x90P\x91\x90PV[_a\x11b\x82a\x117V[\x90P\x91\x90PV[_a\x11s\x82a\x11XV[\x90P\x91\x90PV[a\x11\x83\x81a\x11iV[\x82RPPV[_` \x82\x01\x90Pa\x11\x9C_\x83\x01\x84a\x11zV[\x92\x91PPV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11\xBC\x81a\x11\xA2V[\x82RPPV[_` \x82\x01\x90Pa\x11\xD5_\x83\x01\x84a\x11\xB3V[\x92\x91PPV[a\x11\xE4\x81a\x10-V[\x81\x14a\x11\xEEW_\x80\xFD[PV[_\x815\x90Pa\x11\xFF\x81a\x11\xDBV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12\x1AWa\x12\x19a\x0F^V[[_a\x12'\x84\x82\x85\x01a\x11\xF1V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x12D\x81a\x120V[\x81\x14a\x12NW_\x80\xFD[PV[_\x81Q\x90Pa\x12_\x81a\x12;V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12zWa\x12ya\x0F^V[[_a\x12\x87\x84\x82\x85\x01a\x12QV[\x91PP\x92\x91PPV[_\x81Q\x90Pa\x12\x9E\x81a\x0FkV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12\xB9Wa\x12\xB8a\x0F^V[[_a\x12\xC6\x84\x82\x85\x01a\x12\x90V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x13\x06\x82a\x0FbV[\x91Pa\x13\x11\x83a\x0FbV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13)Wa\x13(a\x12\xCFV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_a\x13\x82a\x13}a\x13x\x84a\x13\\V[a\x11.V[a\x0FbV[\x90P\x91\x90PV[a\x13\x92\x81a\x13hV[\x82RPPV[_` \x82\x01\x90Pa\x13\xAB_\x83\x01\x84a\x13\x89V[\x92\x91PPV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x14\x06W\x80\x86\x04\x81\x11\x15a\x13\xE2Wa\x13\xE1a\x12\xCFV[[`\x01\x85\x16\x15a\x13\xF1W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x13\xFF\x85a\x13\xB1V[\x94Pa\x13\xC6V[\x94P\x94\x92PPPV[_\x82a\x14\x1EW`\x01\x90Pa\x14\xD9V[\x81a\x14+W_\x90Pa\x14\xD9V[\x81`\x01\x81\x14a\x14AW`\x02\x81\x14a\x14KWa\x14zV[`\x01\x91PPa\x14\xD9V[`\xFF\x84\x11\x15a\x14]Wa\x14\\a\x12\xCFV[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x14tWa\x14sa\x12\xCFV[[Pa\x14\xD9V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x14\xAFW\x82\x82\n\x90P\x83\x81\x11\x15a\x14\xAAWa\x14\xA9a\x12\xCFV[[a\x14\xD9V[a\x14\xBC\x84\x84\x84`\x01a\x13\xBDV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x14\xD3Wa\x14\xD2a\x12\xCFV[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x14\xEA\x82a\x0FbV[\x91Pa\x14\xF5\x83a\x13\\V[\x92Pa\x15\"\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x14\x0FV[\x90P\x92\x91PPV[_a\x154\x82a\x0FbV[\x91Pa\x15?\x83a\x0FbV[\x92P\x82\x82\x02a\x15M\x81a\x0FbV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x15dWa\x15ca\x12\xCFV[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x15\xA2\x82a\x0FbV[\x91Pa\x15\xAD\x83a\x0FbV[\x92P\x82a\x15\xBDWa\x15\xBCa\x15kV[[\x82\x82\x04\x90P\x92\x91PPV[_a\x15\xE2a\x15\xDDa\x15\xD8\x84a\x10fV[a\x11.V[a\x0FbV[\x90P\x91\x90PV[a\x15\xF2\x81a\x15\xC8V[\x82RPPV[_` \x82\x01\x90Pa\x16\x0B_\x83\x01\x84a\x15\xE9V[\x92\x91PPV[_a\x16\x1B\x82a\x10fV[\x91Pa\x16&\x83a\x10fV[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x16BWa\x16Aa\x12\xCFV[[\x92\x91PPV[_a\x16R\x82a\x10fV[\x91Pa\x16]\x83a\x10fV[\x92P\x82a\x16mWa\x16la\x15kV[[\x82\x82\x06\x90P\x92\x91PPV[a\x16\x81\x81a\x0FbV[\x82RPPV[_` \x82\x01\x90Pa\x16\x9A_\x83\x01\x84a\x16xV[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x16\xC3a\x16\xBEa\x16\xB9\x84a\x16\xA0V[a\x11.V[a\x13\\V[\x90P\x91\x90PV[a\x16\xD3\x81a\x16\xA9V[\x82RPPV[_`@\x82\x01\x90Pa\x16\xEC_\x83\x01\x85a\x16\xCAV[a\x16\xF9` \x83\x01\x84a\x16xV[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x17#a\x17\x1Ea\x17\x19\x84a\x17\0V[a\x11.V[a\x13\\V[\x90P\x91\x90PV[a\x173\x81a\x17\tV[\x82RPPV[_`@\x82\x01\x90Pa\x17L_\x83\x01\x85a\x17*V[a\x17Y` \x83\x01\x84a\x16xV[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 S\xDC`\xB0\x1A\xA9 \xA7Z\xA2\xF5\xF4@\xF0\x0E\xA2\xB1\x05qg\"\xECK\xCC\xE5\r\xD5K|\xD4WWdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static YIELDORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xF3W_5`\xE0\x1C\x80cy\xBAP\x97\x11a\0\x95W\x80c\xB5\x1AO$\x11a\0dW\x80c\xB5\x1AO$\x14a\x02'W\x80c\xD0\xB0o]\x14a\x02EW\x80c\xE3\x0C9x\x14a\x02cW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x81Wa\0\xF3V[\x80cy\xBAP\x97\x14a\x01\xB3W\x80c\x81\xCE\x1C#\x14a\x01\xBDW\x80c\x8D\xA5\xCB[\x14a\x01\xEDW\x80c\xB1\xA0\xFE\xEF\x14a\x02\x0BWa\0\xF3V[\x80c&\x98{`\x11a\0\xD1W\x80c&\x98{`\x14a\x01cW\x80c1\xC9IG\x14a\x01\x81W\x80cS\xD7\x86\x93\x14a\x01\x9FW\x80cqP\x18\xA6\x14a\x01\xA9Wa\0\xF3V[\x80c\x04\xDEG\xE4\x14a\0\xF7W\x80c\x08Oci\x14a\x01'W\x80c#\x12\xC4\xBB\x14a\x01EW[_\x80\xFD[a\x01\x11`\x04\x806\x03\x81\x01\x90a\x01\x0C\x91\x90a\x0F\x95V[a\x02\x9DV[`@Qa\x01\x1E\x91\x90a\x0F\xF5V[`@Q\x80\x91\x03\x90\xF3[a\x01/a\x02\xE1V[`@Qa\x01<\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x01Ma\x03\x05V[`@Qa\x01Z\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x01ka\x03)V[`@Qa\x01x\x91\x90a\x10\x84V[`@Q\x80\x91\x03\x90\xF3[a\x01\x89a\x03?V[`@Qa\x01\x96\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x01\xA7a\x03cV[\0[a\x01\xB1a\x04\x90V[\0[a\x01\xBBa\x04\xA3V[\0[a\x01\xD7`\x04\x806\x03\x81\x01\x90a\x01\xD2\x91\x90a\x10\x9DV[a\x051V[`@Qa\x01\xE4\x91\x90a\x10\x84V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF5a\x05`V[`@Qa\x02\x02\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x02%`\x04\x806\x03\x81\x01\x90a\x02 \x91\x90a\x11\x03V[a\x05\x87V[\0[a\x02/a\x05\xD2V[`@Qa\x02<\x91\x90a\x11\x89V[`@Q\x80\x91\x03\x90\xF3[a\x02Ma\x05\xF7V[`@Qa\x02Z\x91\x90a\x11\xC2V[`@Q\x80\x91\x03\x90\xF3[a\x02ka\x06\x0FV[`@Qa\x02x\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xF3[a\x02\x9B`\x04\x806\x03\x81\x01\x90a\x02\x96\x91\x90a\x12\x05V[a\x067V[\0[`\x03\x82`\x07\x81\x10a\x02\xACW_\x80\xFD[\x01\x81`\x03\x81\x10a\x02\xBAW_\x80\xFD[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\n_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF1\x91\x90a\x12eV[a\x04\x86W`\n_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA6\xAF\xED\x95`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x84\x91\x90a\x12\xA4V[P[a\x04\x8Ea\x06\xE3V[V[a\x04\x98a\t\xEFV[a\x04\xA1_a\nvV[V[_a\x04\xACa\n\xA6V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\xCDa\x06\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05%W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x1C\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xFD[a\x05.\x81a\nvV[PV[`\x02\x81`\x03\x81\x10a\x05@W_\x80\xFD[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x8Fa\t\xEFV[\x80`\n_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[`\n_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x06?a\t\xEFV[\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\x9Ea\x05`V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[Bb\x01Jx`\n`\x18\x90T\x90a\x01\0\n\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x0F\x91\x90a\x12\xFCV[\x11\x15a\x07GW`@Q\x7F\xE5=\xFD\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x90P_`\x03\x82`\x07\x81\x10a\x07xWa\x07wa\x13/V[[\x01\x90P_[`\x03c\xFF\xFF\xFF\xFF\x16\x81`\xFF\x16\x10\x15a\tzW_a\x07\x9C\x82`\xFF\x16a\n\xADV[\x90P_\x83\x83`\xFF\x16`\x03\x81\x10a\x07\xB5Wa\x07\xB4a\x13/V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x08%W\x82`@Q\x7F\xD8\xD9\xAE\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x1C\x91\x90a\x13\x98V[`@Q\x80\x91\x03\x90\xFD[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10a\x08\xA8W_\x82\x84\x03g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x08\xA4a\x08\x9F`\x07c\xFF\xFF\xFF\xFF\x16`\x08`\na\x08m\x91\x90a\x14\xE0V[a\x01ma\x08z\x91\x90a\x15*V[a\x08\x84\x91\x90a\x15\x98V[\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84a\x0C\xEC\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\xEBV[\x91PP[\x80`\x02\x85`\xFF\x16`\x03\x81\x10a\x08\xC0Wa\x08\xBFa\x13/V[[`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82\x85\x85`\xFF\x16`\x03\x81\x10a\t\0Wa\x08\xFFa\x13/V[[`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\xFF\x16\x7F\xD2\x0CoF\xFFO\x97t2\x15\x1B\xD6,\xF6[\x9F\x90\xDC~\x0CFc\xF8#\xB64\xCC\x94\xC0MU\x9E\x82`@Qa\td\x91\x90a\x15\xF8V[`@Q\x80\x91\x03\x90\xA2\x83`\x01\x01\x93PPPPa\x07}V[P`\x07`\x01`\n`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\t\x9C\x91\x90a\x16\x11V[a\t\xA6\x91\x90a\x16HV[`\n`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPB`\n`\x18a\x01\0\n\x81T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\t\xF7a\n\xA6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\x15a\x05`V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\ntWa\n8a\n\xA6V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nk\x91\x90a\x10MV[`@Q\x80\x91\x03\x90\xFD[V[`\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\n\xA3\x81a\x0EBV[PV[_3\x90P\x90V[_\x80\x82\x03a\x0BVW_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0BN\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cg\x9A\xEF\xCE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B%W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BI\x91\x90a\x12\xA4V[a\x0F\x03V[\x91PPa\x0C\xE7V[`\x01\x82\x03a\x0B\xFFW_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0B\xF7\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE6\xAA!l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xCEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF2\x91\x90a\x12\xA4V[a\x0F\x03V[\x91PPa\x0C\xE6V[`\x02\x82\x03a\x0C\xA8W_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\x0C\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\x8B,\xB6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CwW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9B\x91\x90a\x12\xA4V[a\x0F\x03V[\x91PPa\x0C\xE5V[\x81`@Q\x7F\x13\xC3eh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xDC\x91\x90a\x16\x87V[`@Q\x80\x91\x03\x90\xFD[[[\x91\x90PV[_\x80\x83\x85\x02\x90P_\x80\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP_\x81\x03a\r$W\x83\x82\x81a\r\x1AWa\r\x19a\x15kV[[\x04\x92PPPa\r\xE4V[\x80\x84\x11a\r]W`@Q\x7F\"{\xC1S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t\x90P\x82\x81\x11\x82\x03\x91P\x80\x83\x03\x92P_\x85_\x03\x86\x16\x90P\x80\x86\x04\x95P\x80\x84\x04\x93P`\x01\x81\x82_\x03\x04\x01\x90P\x80\x83\x02\x84\x17\x93P_`\x02\x87`\x03\x02\x18\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x87\x02`\x02\x03\x81\x02\x90P\x80\x85\x02\x95PPPPPP[\x93\x92PPPV[_c\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\x0E:W` \x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E1\x92\x91\x90a\x16\xD9V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11\x15a\x0FVW`@\x82`@Q\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0FM\x92\x91\x90a\x179V[`@Q\x80\x91\x03\x90\xFD[\x81\x90P\x91\x90PV[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x0Ft\x81a\x0FbV[\x81\x14a\x0F~W_\x80\xFD[PV[_\x815\x90Pa\x0F\x8F\x81a\x0FkV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x0F\xABWa\x0F\xAAa\x0F^V[[_a\x0F\xB8\x85\x82\x86\x01a\x0F\x81V[\x92PP` a\x0F\xC9\x85\x82\x86\x01a\x0F\x81V[\x91PP\x92P\x92\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xEF\x81a\x0F\xD3V[\x82RPPV[_` \x82\x01\x90Pa\x10\x08_\x83\x01\x84a\x0F\xE6V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x107\x82a\x10\x0EV[\x90P\x91\x90PV[a\x10G\x81a\x10-V[\x82RPPV[_` \x82\x01\x90Pa\x10`_\x83\x01\x84a\x10>V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10~\x81a\x10fV[\x82RPPV[_` \x82\x01\x90Pa\x10\x97_\x83\x01\x84a\x10uV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10\xB2Wa\x10\xB1a\x0F^V[[_a\x10\xBF\x84\x82\x85\x01a\x0F\x81V[\x91PP\x92\x91PPV[_a\x10\xD2\x82a\x10-V[\x90P\x91\x90PV[a\x10\xE2\x81a\x10\xC8V[\x81\x14a\x10\xECW_\x80\xFD[PV[_\x815\x90Pa\x10\xFD\x81a\x10\xD9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11\x18Wa\x11\x17a\x0F^V[[_a\x11%\x84\x82\x85\x01a\x10\xEFV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x11Qa\x11La\x11G\x84a\x10\x0EV[a\x11.V[a\x10\x0EV[\x90P\x91\x90PV[_a\x11b\x82a\x117V[\x90P\x91\x90PV[_a\x11s\x82a\x11XV[\x90P\x91\x90PV[a\x11\x83\x81a\x11iV[\x82RPPV[_` \x82\x01\x90Pa\x11\x9C_\x83\x01\x84a\x11zV[\x92\x91PPV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11\xBC\x81a\x11\xA2V[\x82RPPV[_` \x82\x01\x90Pa\x11\xD5_\x83\x01\x84a\x11\xB3V[\x92\x91PPV[a\x11\xE4\x81a\x10-V[\x81\x14a\x11\xEEW_\x80\xFD[PV[_\x815\x90Pa\x11\xFF\x81a\x11\xDBV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12\x1AWa\x12\x19a\x0F^V[[_a\x12'\x84\x82\x85\x01a\x11\xF1V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x12D\x81a\x120V[\x81\x14a\x12NW_\x80\xFD[PV[_\x81Q\x90Pa\x12_\x81a\x12;V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12zWa\x12ya\x0F^V[[_a\x12\x87\x84\x82\x85\x01a\x12QV[\x91PP\x92\x91PPV[_\x81Q\x90Pa\x12\x9E\x81a\x0FkV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12\xB9Wa\x12\xB8a\x0F^V[[_a\x12\xC6\x84\x82\x85\x01a\x12\x90V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x13\x06\x82a\x0FbV[\x91Pa\x13\x11\x83a\x0FbV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13)Wa\x13(a\x12\xCFV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_a\x13\x82a\x13}a\x13x\x84a\x13\\V[a\x11.V[a\x0FbV[\x90P\x91\x90PV[a\x13\x92\x81a\x13hV[\x82RPPV[_` \x82\x01\x90Pa\x13\xAB_\x83\x01\x84a\x13\x89V[\x92\x91PPV[_\x81`\x01\x1C\x90P\x91\x90PV[_\x80\x82\x91P\x83\x90P[`\x01\x85\x11\x15a\x14\x06W\x80\x86\x04\x81\x11\x15a\x13\xE2Wa\x13\xE1a\x12\xCFV[[`\x01\x85\x16\x15a\x13\xF1W\x80\x82\x02\x91P[\x80\x81\x02\x90Pa\x13\xFF\x85a\x13\xB1V[\x94Pa\x13\xC6V[\x94P\x94\x92PPPV[_\x82a\x14\x1EW`\x01\x90Pa\x14\xD9V[\x81a\x14+W_\x90Pa\x14\xD9V[\x81`\x01\x81\x14a\x14AW`\x02\x81\x14a\x14KWa\x14zV[`\x01\x91PPa\x14\xD9V[`\xFF\x84\x11\x15a\x14]Wa\x14\\a\x12\xCFV[[\x83`\x02\n\x91P\x84\x82\x11\x15a\x14tWa\x14sa\x12\xCFV[[Pa\x14\xD9V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x14\xAFW\x82\x82\n\x90P\x83\x81\x11\x15a\x14\xAAWa\x14\xA9a\x12\xCFV[[a\x14\xD9V[a\x14\xBC\x84\x84\x84`\x01a\x13\xBDV[\x92P\x90P\x81\x84\x04\x81\x11\x15a\x14\xD3Wa\x14\xD2a\x12\xCFV[[\x81\x81\x02\x90P[\x93\x92PPPV[_a\x14\xEA\x82a\x0FbV[\x91Pa\x14\xF5\x83a\x13\\V[\x92Pa\x15\"\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x84a\x14\x0FV[\x90P\x92\x91PPV[_a\x154\x82a\x0FbV[\x91Pa\x15?\x83a\x0FbV[\x92P\x82\x82\x02a\x15M\x81a\x0FbV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x15dWa\x15ca\x12\xCFV[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x15\xA2\x82a\x0FbV[\x91Pa\x15\xAD\x83a\x0FbV[\x92P\x82a\x15\xBDWa\x15\xBCa\x15kV[[\x82\x82\x04\x90P\x92\x91PPV[_a\x15\xE2a\x15\xDDa\x15\xD8\x84a\x10fV[a\x11.V[a\x0FbV[\x90P\x91\x90PV[a\x15\xF2\x81a\x15\xC8V[\x82RPPV[_` \x82\x01\x90Pa\x16\x0B_\x83\x01\x84a\x15\xE9V[\x92\x91PPV[_a\x16\x1B\x82a\x10fV[\x91Pa\x16&\x83a\x10fV[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x16BWa\x16Aa\x12\xCFV[[\x92\x91PPV[_a\x16R\x82a\x10fV[\x91Pa\x16]\x83a\x10fV[\x92P\x82a\x16mWa\x16la\x15kV[[\x82\x82\x06\x90P\x92\x91PPV[a\x16\x81\x81a\x0FbV[\x82RPPV[_` \x82\x01\x90Pa\x16\x9A_\x83\x01\x84a\x16xV[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x16\xC3a\x16\xBEa\x16\xB9\x84a\x16\xA0V[a\x11.V[a\x13\\V[\x90P\x91\x90PV[a\x16\xD3\x81a\x16\xA9V[\x82RPPV[_`@\x82\x01\x90Pa\x16\xEC_\x83\x01\x85a\x16\xCAV[a\x16\xF9` \x83\x01\x84a\x16xV[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x17#a\x17\x1Ea\x17\x19\x84a\x17\0V[a\x11.V[a\x13\\V[\x90P\x91\x90PV[a\x173\x81a\x17\tV[\x82RPPV[_`@\x82\x01\x90Pa\x17L_\x83\x01\x85a\x17*V[a\x17Y` \x83\x01\x84a\x16xV[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 S\xDC`\xB0\x1A\xA9 \xA7Z\xA2\xF5\xF4@\xF0\x0E\xA2\xB1\x05qg\"\xECK\xCC\xE5\r\xD5K|\xD4WWdsolcC\0\x08\x15\x003";
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
    ///Custom Error type `InvalidIlkIndex` with signature `InvalidIlkIndex(uint256)` and selector `0x13c36568`
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
    #[etherror(name = "InvalidIlkIndex", abi = "InvalidIlkIndex(uint256)")]
    pub struct InvalidIlkIndex {
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
        InvalidIlkIndex(InvalidIlkIndex),
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
            if let Ok(decoded) = <InvalidIlkIndex as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidIlkIndex(decoded));
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
                Self::InvalidIlkIndex(element) => {
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
                    == <InvalidIlkIndex as ::ethers::contract::EthError>::selector() => {
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
                Self::InvalidIlkIndex(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<InvalidIlkIndex> for YieldOracleErrors {
        fn from(value: InvalidIlkIndex) -> Self {
            Self::InvalidIlkIndex(value)
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
