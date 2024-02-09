pub use whitelist::*;
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
pub mod whitelist {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_borrowersRoots"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes32[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lendersRoot"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                            32usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes32"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("approveProtocolWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "approveProtocolWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
                    ::std::borrow::ToOwned::to_owned("borrowersRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowersRoot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isWhitelistedBorrower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isWhitelistedBorrower",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolCaller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isWhitelistedLender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isWhitelistedLender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolCaller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lendersRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lendersRoot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("protocolWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolWhitelist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "protocolControlledAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("revokeProtocolWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokeProtocolWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
                    ::std::borrow::ToOwned::to_owned("updateBorrowersRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateBorrowersRoot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_borrowersRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("updateLendersRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateLendersRoot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lendersRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("NotWhitelistedBorrower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotWhitelistedBorrower",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
                    ::std::borrow::ToOwned::to_owned("NotWhitelistedLender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotWhitelistedLender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static WHITELIST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x10W_\x80\xFD[P`@Qb\0\x14g8\x03\x80b\0\x14g\x839\x81\x81\x01`@R\x81\x01\x90b\0\x006\x91\x90b\0\x03\xDBV[3_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\xAAW_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0\xA1\x91\x90b\0\x04\x82V[`@Q\x80\x91\x03\x90\xFD[b\0\0\xBB\x81b\0\x01-` \x1B` \x1CV[P_[\x82Q\x81`\xFF\x16\x10\x15b\0\x01\x1DW\x82\x81`\xFF\x16\x81Q\x81\x10b\0\0\xE4Wb\0\0\xE3b\0\x04\x9DV[[` \x02` \x01\x01Q`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x80\x80b\0\x01\x14\x90b\0\x05\x03V[\x91PPb\0\0\xBEV[P\x80`\x04\x81\x90UPPPb\0\x050V[`\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ub\0\x01b\x81b\0\x01e` \x1B` \x1CV[PV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[b\0\x02\x83\x82b\0\x02;V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x02\xA5Wb\0\x02\xA4b\0\x02KV[[\x80`@RPPPV[_b\0\x02\xB9b\0\x02&V[\x90Pb\0\x02\xC7\x82\x82b\0\x02xV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x02\xE9Wb\0\x02\xE8b\0\x02KV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_\x81\x90P\x91\x90PV[b\0\x03\x12\x81b\0\x02\xFEV[\x81\x14b\0\x03\x1DW_\x80\xFD[PV[_\x81Q\x90Pb\0\x030\x81b\0\x03\x07V[\x92\x91PPV[_b\0\x03Lb\0\x03F\x84b\0\x02\xCCV[b\0\x02\xAEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x03rWb\0\x03qb\0\x02\xFAV[[\x83[\x81\x81\x10\x15b\0\x03\x9FW\x80b\0\x03\x8A\x88\x82b\0\x03 V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x03tV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12b\0\x03\xC0Wb\0\x03\xBFb\0\x027V[[\x81Qb\0\x03\xD2\x84\x82` \x86\x01b\0\x036V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15b\0\x03\xF4Wb\0\x03\xF3b\0\x02/V[[_\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x04\x14Wb\0\x04\x13b\0\x023V[[b\0\x04\"\x85\x82\x86\x01b\0\x03\xA9V[\x92PP` b\0\x045\x85\x82\x86\x01b\0\x03 V[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\x04j\x82b\0\x04?V[\x90P\x91\x90PV[b\0\x04|\x81b\0\x04^V[\x82RPPV[_` \x82\x01\x90Pb\0\x04\x97_\x83\x01\x84b\0\x04qV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_b\0\x05\x0F\x82b\0\x04\xF7V[\x91P`\xFF\x82\x03b\0\x05%Wb\0\x05$b\0\x04\xCAV[[`\x01\x82\x01\x90P\x91\x90PV[a\x0F)\x80b\0\x05>_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xE8W_5`\xE0\x1C\x80c\x8Cz\x0C\xBD\x11a\0\x8AW\x80c\xB5@k=\x11a\0dW\x80c\xB5@k=\x14a\x02\x0CW\x80c\xBD\x81\x8AY\x14a\x02<W\x80c\xE3\x0C9x\x14a\x02lW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x8AWa\0\xE8V[\x80c\x8Cz\x0C\xBD\x14a\x01\xB6W\x80c\x8D\xA5\xCB[\x14a\x01\xD2W\x80c\x96\xFF\xA8\xD5\x14a\x01\xF0Wa\0\xE8V[\x80c`g&\x8C\x11a\0\xC6W\x80c`g&\x8C\x14a\x01TW\x80ca,\xB2\xB5\x14a\x01rW\x80cqP\x18\xA6\x14a\x01\xA2W\x80cy\xBAP\x97\x14a\x01\xACWa\0\xE8V[\x80c\x1D\xB4\x86e\x14a\0\xECW\x80c=\xFD\x14\x0B\x14a\x01\x1CW\x80cX'\x8A\xB6\x14a\x018W[_\x80\xFD[a\x01\x06`\x04\x806\x03\x81\x01\x90a\x01\x01\x91\x90a\x0B9V[a\x02\xA6V[`@Qa\x01\x13\x91\x90a\x0B\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x016`\x04\x806\x03\x81\x01\x90a\x011\x91\x90a\x0C\x10V[a\x04\x08V[\0[a\x01R`\x04\x806\x03\x81\x01\x90a\x01M\x91\x90a\x0C;V[a\x04\x1AV[\0[a\x01\\a\x04zV[`@Qa\x01i\x91\x90a\x0CuV[`@Q\x80\x91\x03\x90\xF3[a\x01\x8C`\x04\x806\x03\x81\x01\x90a\x01\x87\x91\x90a\x0C;V[a\x04\x80V[`@Qa\x01\x99\x91\x90a\x0B\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x01\xAAa\x04\x9DV[\0[a\x01\xB4a\x04\xB0V[\0[a\x01\xD0`\x04\x806\x03\x81\x01\x90a\x01\xCB\x91\x90a\x0C\xC4V[a\x05>V[\0[a\x01\xDAa\x05fV[`@Qa\x01\xE7\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\n`\x04\x806\x03\x81\x01\x90a\x02\x05\x91\x90a\x0C;V[a\x05\x8DV[\0[a\x02&`\x04\x806\x03\x81\x01\x90a\x02!\x91\x90a\r*V[a\x05\xECV[`@Qa\x023\x91\x90a\x0B\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02V`\x04\x806\x03\x81\x01\x90a\x02Q\x91\x90a\r\xAEV[a\x07fV[`@Qa\x02c\x91\x90a\x0CuV[`@Q\x80\x91\x03\x90\xF3[a\x02ta\x07{V[`@Qa\x02\x81\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA4`\x04\x806\x03\x81\x01\x90a\x02\x9F\x91\x90a\x0C;V[a\x07\xA3V[\0[_`\x02_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x02\xFFW`\x01\x90Pa\x04\0V[_`\x04T\x90P_\x80\x1B\x81\x03a\x03\x18W`\x01\x91PPa\x04\0V[_\x85`@Q` \x01a\x03*\x91\x90a\r\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x03P\x91\x90a\r\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x03\xB3\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08OV[\x15a\x03\xC3W`\x01\x92PPPa\x04\0V[\x85`@Q\x7F\xA2\xA9?\xD3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xF7\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[a\x04\x10a\x08eV[\x80`\x04\x81\x90UPPV[a\x04\"a\x08eV[`\x01`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x04T\x81V[`\x02` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[a\x04\xA5a\x08eV[a\x04\xAE_a\x08\xECV[V[_a\x04\xB9a\t\x1CV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\xDAa\x07{V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x052W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05)\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xFD[a\x05;\x81a\x08\xECV[PV[a\x05Fa\x08eV[\x80`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UPPPV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x95a\x08eV[_`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[_`\x02_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x06EW`\x01\x90Pa\x07]V[_`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a\x06sW`\x01\x91PPa\x07]V[_\x85`@Q` \x01a\x06\x85\x91\x90a\r\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x06\xAB\x91\x90a\r\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x07\x0E\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08OV[\x15a\x07\x1EW`\x01\x92PPPa\x07]V[\x87\x86`@Q\x7F\x7F\xBDtD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07T\x92\x91\x90a\x0E\"V[`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[`\x03` R\x80_R`@_ _\x91P\x90PT\x81V[_`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x07\xABa\x08eV[\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\na\x05fV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[_\x82a\x08[\x85\x84a\t#V[\x14\x90P\x93\x92PPPV[a\x08ma\t\x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\x8Ba\x05fV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\xEAWa\x08\xAEa\t\x1CV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xE1\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xFD[V[`\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\t\x19\x81a\twV[PV[_3\x90P\x90V[_\x80\x82\x90P_[\x84Q\x81\x10\x15a\tlWa\tW\x82\x86\x83\x81Q\x81\x10a\tJWa\tIa\x0EIV[[` \x02` \x01\x01Qa\n8V[\x91P\x80\x80a\td\x90a\x0E\xACV[\x91PPa\t*V[P\x80\x91PP\x92\x91PPV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_\x81\x83\x10a\nOWa\nJ\x82\x84a\nbV[a\nZV[a\nY\x83\x83a\nbV[[\x90P\x92\x91PPV[_\x82_R\x81` R`@_ \x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\n\xA7\x82a\n~V[\x90P\x91\x90PV[a\n\xB7\x81a\n\x9DV[\x81\x14a\n\xC1W_\x80\xFD[PV[_\x815\x90Pa\n\xD2\x81a\n\xAEV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a\n\xF9Wa\n\xF8a\n\xD8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x16Wa\x0B\x15a\n\xDCV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x0B2Wa\x0B1a\n\xE0V[[\x92P\x92\x90PV[_\x80_\x80``\x85\x87\x03\x12\x15a\x0BQWa\x0BPa\nvV[[_a\x0B^\x87\x82\x88\x01a\n\xC4V[\x94PP` a\x0Bo\x87\x82\x88\x01a\n\xC4V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x90Wa\x0B\x8Fa\nzV[[a\x0B\x9C\x87\x82\x88\x01a\n\xE4V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[a\x0B\xBE\x81a\x0B\xAAV[\x82RPPV[_` \x82\x01\x90Pa\x0B\xD7_\x83\x01\x84a\x0B\xB5V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0B\xEF\x81a\x0B\xDDV[\x81\x14a\x0B\xF9W_\x80\xFD[PV[_\x815\x90Pa\x0C\n\x81a\x0B\xE6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0C%Wa\x0C$a\nvV[[_a\x0C2\x84\x82\x85\x01a\x0B\xFCV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0CPWa\x0COa\nvV[[_a\x0C]\x84\x82\x85\x01a\n\xC4V[\x91PP\x92\x91PPV[a\x0Co\x81a\x0B\xDDV[\x82RPPV[_` \x82\x01\x90Pa\x0C\x88_\x83\x01\x84a\x0CfV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0C\xA3\x81a\x0C\x8EV[\x81\x14a\x0C\xADW_\x80\xFD[PV[_\x815\x90Pa\x0C\xBE\x81a\x0C\x9AV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x0C\xDAWa\x0C\xD9a\nvV[[_a\x0C\xE7\x85\x82\x86\x01a\x0C\xB0V[\x92PP` a\x0C\xF8\x85\x82\x86\x01a\x0B\xFCV[\x91PP\x92P\x92\x90PV[a\r\x0B\x81a\n\x9DV[\x82RPPV[_` \x82\x01\x90Pa\r$_\x83\x01\x84a\r\x02V[\x92\x91PPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a\rCWa\rBa\nvV[[_a\rP\x88\x82\x89\x01a\x0C\xB0V[\x95PP` a\ra\x88\x82\x89\x01a\n\xC4V[\x94PP`@a\rr\x88\x82\x89\x01a\n\xC4V[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x93Wa\r\x92a\nzV[[a\r\x9F\x88\x82\x89\x01a\n\xE4V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\r\xC3Wa\r\xC2a\nvV[[_a\r\xD0\x84\x82\x85\x01a\x0C\xB0V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\xF3a\r\xEE\x82a\x0B\xDDV[a\r\xD9V[\x82RPPV[_a\x0E\x04\x82\x84a\r\xE2V[` \x82\x01\x91P\x81\x90P\x92\x91PPV[a\x0E\x1C\x81a\x0C\x8EV[\x82RPPV[_`@\x82\x01\x90Pa\x0E5_\x83\x01\x85a\x0E\x13V[a\x0EB` \x83\x01\x84a\r\x02V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[_a\x0E\xB6\x82a\x0E\xA3V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\xE8Wa\x0E\xE7a\x0EvV[[`\x01\x82\x01\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 w\"\x84\\\xADy\"\x91y\x91\x1E\x97$\x88F\x9D\x8D\x16\xA0{\x02\xF2\x84\xFB\x8E\xC5$\xE78\x03\xE3\xEEdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static WHITELIST_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xE8W_5`\xE0\x1C\x80c\x8Cz\x0C\xBD\x11a\0\x8AW\x80c\xB5@k=\x11a\0dW\x80c\xB5@k=\x14a\x02\x0CW\x80c\xBD\x81\x8AY\x14a\x02<W\x80c\xE3\x0C9x\x14a\x02lW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x8AWa\0\xE8V[\x80c\x8Cz\x0C\xBD\x14a\x01\xB6W\x80c\x8D\xA5\xCB[\x14a\x01\xD2W\x80c\x96\xFF\xA8\xD5\x14a\x01\xF0Wa\0\xE8V[\x80c`g&\x8C\x11a\0\xC6W\x80c`g&\x8C\x14a\x01TW\x80ca,\xB2\xB5\x14a\x01rW\x80cqP\x18\xA6\x14a\x01\xA2W\x80cy\xBAP\x97\x14a\x01\xACWa\0\xE8V[\x80c\x1D\xB4\x86e\x14a\0\xECW\x80c=\xFD\x14\x0B\x14a\x01\x1CW\x80cX'\x8A\xB6\x14a\x018W[_\x80\xFD[a\x01\x06`\x04\x806\x03\x81\x01\x90a\x01\x01\x91\x90a\x0B9V[a\x02\xA6V[`@Qa\x01\x13\x91\x90a\x0B\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x016`\x04\x806\x03\x81\x01\x90a\x011\x91\x90a\x0C\x10V[a\x04\x08V[\0[a\x01R`\x04\x806\x03\x81\x01\x90a\x01M\x91\x90a\x0C;V[a\x04\x1AV[\0[a\x01\\a\x04zV[`@Qa\x01i\x91\x90a\x0CuV[`@Q\x80\x91\x03\x90\xF3[a\x01\x8C`\x04\x806\x03\x81\x01\x90a\x01\x87\x91\x90a\x0C;V[a\x04\x80V[`@Qa\x01\x99\x91\x90a\x0B\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x01\xAAa\x04\x9DV[\0[a\x01\xB4a\x04\xB0V[\0[a\x01\xD0`\x04\x806\x03\x81\x01\x90a\x01\xCB\x91\x90a\x0C\xC4V[a\x05>V[\0[a\x01\xDAa\x05fV[`@Qa\x01\xE7\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\n`\x04\x806\x03\x81\x01\x90a\x02\x05\x91\x90a\x0C;V[a\x05\x8DV[\0[a\x02&`\x04\x806\x03\x81\x01\x90a\x02!\x91\x90a\r*V[a\x05\xECV[`@Qa\x023\x91\x90a\x0B\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02V`\x04\x806\x03\x81\x01\x90a\x02Q\x91\x90a\r\xAEV[a\x07fV[`@Qa\x02c\x91\x90a\x0CuV[`@Q\x80\x91\x03\x90\xF3[a\x02ta\x07{V[`@Qa\x02\x81\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA4`\x04\x806\x03\x81\x01\x90a\x02\x9F\x91\x90a\x0C;V[a\x07\xA3V[\0[_`\x02_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x02\xFFW`\x01\x90Pa\x04\0V[_`\x04T\x90P_\x80\x1B\x81\x03a\x03\x18W`\x01\x91PPa\x04\0V[_\x85`@Q` \x01a\x03*\x91\x90a\r\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x03P\x91\x90a\r\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x03\xB3\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08OV[\x15a\x03\xC3W`\x01\x92PPPa\x04\0V[\x85`@Q\x7F\xA2\xA9?\xD3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xF7\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[a\x04\x10a\x08eV[\x80`\x04\x81\x90UPPV[a\x04\"a\x08eV[`\x01`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x04T\x81V[`\x02` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[a\x04\xA5a\x08eV[a\x04\xAE_a\x08\xECV[V[_a\x04\xB9a\t\x1CV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\xDAa\x07{V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x052W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05)\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xFD[a\x05;\x81a\x08\xECV[PV[a\x05Fa\x08eV[\x80`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UPPPV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x95a\x08eV[_`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[_`\x02_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x06EW`\x01\x90Pa\x07]V[_`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a\x06sW`\x01\x91PPa\x07]V[_\x85`@Q` \x01a\x06\x85\x91\x90a\r\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x06\xAB\x91\x90a\r\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x07\x0E\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08OV[\x15a\x07\x1EW`\x01\x92PPPa\x07]V[\x87\x86`@Q\x7F\x7F\xBDtD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07T\x92\x91\x90a\x0E\"V[`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[`\x03` R\x80_R`@_ _\x91P\x90PT\x81V[_`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x07\xABa\x08eV[\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\na\x05fV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[_\x82a\x08[\x85\x84a\t#V[\x14\x90P\x93\x92PPPV[a\x08ma\t\x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\x8Ba\x05fV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\xEAWa\x08\xAEa\t\x1CV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xE1\x91\x90a\r\x11V[`@Q\x80\x91\x03\x90\xFD[V[`\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\t\x19\x81a\twV[PV[_3\x90P\x90V[_\x80\x82\x90P_[\x84Q\x81\x10\x15a\tlWa\tW\x82\x86\x83\x81Q\x81\x10a\tJWa\tIa\x0EIV[[` \x02` \x01\x01Qa\n8V[\x91P\x80\x80a\td\x90a\x0E\xACV[\x91PPa\t*V[P\x80\x91PP\x92\x91PPV[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_\x81\x83\x10a\nOWa\nJ\x82\x84a\nbV[a\nZV[a\nY\x83\x83a\nbV[[\x90P\x92\x91PPV[_\x82_R\x81` R`@_ \x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\n\xA7\x82a\n~V[\x90P\x91\x90PV[a\n\xB7\x81a\n\x9DV[\x81\x14a\n\xC1W_\x80\xFD[PV[_\x815\x90Pa\n\xD2\x81a\n\xAEV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a\n\xF9Wa\n\xF8a\n\xD8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x16Wa\x0B\x15a\n\xDCV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x0B2Wa\x0B1a\n\xE0V[[\x92P\x92\x90PV[_\x80_\x80``\x85\x87\x03\x12\x15a\x0BQWa\x0BPa\nvV[[_a\x0B^\x87\x82\x88\x01a\n\xC4V[\x94PP` a\x0Bo\x87\x82\x88\x01a\n\xC4V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x90Wa\x0B\x8Fa\nzV[[a\x0B\x9C\x87\x82\x88\x01a\n\xE4V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[a\x0B\xBE\x81a\x0B\xAAV[\x82RPPV[_` \x82\x01\x90Pa\x0B\xD7_\x83\x01\x84a\x0B\xB5V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0B\xEF\x81a\x0B\xDDV[\x81\x14a\x0B\xF9W_\x80\xFD[PV[_\x815\x90Pa\x0C\n\x81a\x0B\xE6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0C%Wa\x0C$a\nvV[[_a\x0C2\x84\x82\x85\x01a\x0B\xFCV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0CPWa\x0COa\nvV[[_a\x0C]\x84\x82\x85\x01a\n\xC4V[\x91PP\x92\x91PPV[a\x0Co\x81a\x0B\xDDV[\x82RPPV[_` \x82\x01\x90Pa\x0C\x88_\x83\x01\x84a\x0CfV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0C\xA3\x81a\x0C\x8EV[\x81\x14a\x0C\xADW_\x80\xFD[PV[_\x815\x90Pa\x0C\xBE\x81a\x0C\x9AV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x0C\xDAWa\x0C\xD9a\nvV[[_a\x0C\xE7\x85\x82\x86\x01a\x0C\xB0V[\x92PP` a\x0C\xF8\x85\x82\x86\x01a\x0B\xFCV[\x91PP\x92P\x92\x90PV[a\r\x0B\x81a\n\x9DV[\x82RPPV[_` \x82\x01\x90Pa\r$_\x83\x01\x84a\r\x02V[\x92\x91PPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a\rCWa\rBa\nvV[[_a\rP\x88\x82\x89\x01a\x0C\xB0V[\x95PP` a\ra\x88\x82\x89\x01a\n\xC4V[\x94PP`@a\rr\x88\x82\x89\x01a\n\xC4V[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x93Wa\r\x92a\nzV[[a\r\x9F\x88\x82\x89\x01a\n\xE4V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\r\xC3Wa\r\xC2a\nvV[[_a\r\xD0\x84\x82\x85\x01a\x0C\xB0V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\xF3a\r\xEE\x82a\x0B\xDDV[a\r\xD9V[\x82RPPV[_a\x0E\x04\x82\x84a\r\xE2V[` \x82\x01\x91P\x81\x90P\x92\x91PPV[a\x0E\x1C\x81a\x0C\x8EV[\x82RPPV[_`@\x82\x01\x90Pa\x0E5_\x83\x01\x85a\x0E\x13V[a\x0EB` \x83\x01\x84a\r\x02V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[_a\x0E\xB6\x82a\x0E\xA3V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\xE8Wa\x0E\xE7a\x0EvV[[`\x01\x82\x01\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 w\"\x84\\\xADy\"\x91y\x91\x1E\x97$\x88F\x9D\x8D\x16\xA0{\x02\xF2\x84\xFB\x8E\xC5$\xE78\x03\xE3\xEEdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static WHITELIST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Whitelist<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Whitelist<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Whitelist<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Whitelist<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Whitelist<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Whitelist)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Whitelist<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    WHITELIST_ABI.clone(),
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
                WHITELIST_ABI.clone(),
                WHITELIST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveProtocolWhitelist` (0x58278ab6) function
        pub fn approve_protocol_whitelist(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 39, 138, 182], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrowersRoot` (0xbd818a59) function
        pub fn borrowers_root(
            &self,
            ilk_index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([189, 129, 138, 89], ilk_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWhitelistedBorrower` (0xb5406b3d) function
        pub fn is_whitelisted_borrower(
            &self,
            ilk_index: u8,
            pool_caller: ::ethers::core::types::Address,
            addr: ::ethers::core::types::Address,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 64, 107, 61], (ilk_index, pool_caller, addr, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWhitelistedLender` (0x1db48665) function
        pub fn is_whitelisted_lender(
            &self,
            pool_caller: ::ethers::core::types::Address,
            addr: ::ethers::core::types::Address,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 180, 134, 101], (pool_caller, addr, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lendersRoot` (0x6067268c) function
        pub fn lenders_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([96, 103, 38, 140], ())
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
        ///Calls the contract's `protocolWhitelist` (0x612cb2b5) function
        pub fn protocol_whitelist(
            &self,
            protocol_controlled_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([97, 44, 178, 181], protocol_controlled_address)
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
        ///Calls the contract's `revokeProtocolWhitelist` (0x96ffa8d5) function
        pub fn revoke_protocol_whitelist(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 255, 168, 213], addr)
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
        ///Calls the contract's `updateBorrowersRoot` (0x8c7a0cbd) function
        pub fn update_borrowers_root(
            &self,
            ilk_index: u8,
            borrowers_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 122, 12, 189], (ilk_index, borrowers_root))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateLendersRoot` (0x3dfd140b) function
        pub fn update_lenders_root(
            &self,
            lenders_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 253, 20, 11], lenders_root)
                .expect("method not found (this should never happen)")
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
            WhitelistEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Whitelist<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotWhitelistedBorrower` with signature `NotWhitelistedBorrower(uint8,address)` and selector `0x7fbd7444`
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
        name = "NotWhitelistedBorrower",
        abi = "NotWhitelistedBorrower(uint8,address)"
    )]
    pub struct NotWhitelistedBorrower {
        pub ilk_index: u8,
        pub addr: ::ethers::core::types::Address,
    }
    ///Custom Error type `NotWhitelistedLender` with signature `NotWhitelistedLender(address)` and selector `0xa2a93fd3`
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
    #[etherror(name = "NotWhitelistedLender", abi = "NotWhitelistedLender(address)")]
    pub struct NotWhitelistedLender {
        pub addr: ::ethers::core::types::Address,
    }
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
    pub enum WhitelistErrors {
        NotWhitelistedBorrower(NotWhitelistedBorrower),
        NotWhitelistedLender(NotWhitelistedLender),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for WhitelistErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <NotWhitelistedBorrower as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotWhitelistedBorrower(decoded));
            }
            if let Ok(decoded) = <NotWhitelistedLender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotWhitelistedLender(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WhitelistErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NotWhitelistedBorrower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotWhitelistedLender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for WhitelistErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <NotWhitelistedBorrower as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotWhitelistedLender as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for WhitelistErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NotWhitelistedBorrower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotWhitelistedLender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for WhitelistErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<NotWhitelistedBorrower> for WhitelistErrors {
        fn from(value: NotWhitelistedBorrower) -> Self {
            Self::NotWhitelistedBorrower(value)
        }
    }
    impl ::core::convert::From<NotWhitelistedLender> for WhitelistErrors {
        fn from(value: NotWhitelistedLender) -> Self {
            Self::NotWhitelistedLender(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for WhitelistErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for WhitelistErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
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
    pub enum WhitelistEvents {
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for WhitelistEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(WhitelistEvents::OwnershipTransferStartedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(WhitelistEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WhitelistEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter> for WhitelistEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for WhitelistEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `approveProtocolWhitelist` function with signature `approveProtocolWhitelist(address)` and selector `0x58278ab6`
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
        name = "approveProtocolWhitelist",
        abi = "approveProtocolWhitelist(address)"
    )]
    pub struct ApproveProtocolWhitelistCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `borrowersRoot` function with signature `borrowersRoot(uint8)` and selector `0xbd818a59`
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
    #[ethcall(name = "borrowersRoot", abi = "borrowersRoot(uint8)")]
    pub struct BorrowersRootCall {
        pub ilk_index: u8,
    }
    ///Container type for all input parameters for the `isWhitelistedBorrower` function with signature `isWhitelistedBorrower(uint8,address,address,bytes32[])` and selector `0xb5406b3d`
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
        name = "isWhitelistedBorrower",
        abi = "isWhitelistedBorrower(uint8,address,address,bytes32[])"
    )]
    pub struct IsWhitelistedBorrowerCall {
        pub ilk_index: u8,
        pub pool_caller: ::ethers::core::types::Address,
        pub addr: ::ethers::core::types::Address,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `isWhitelistedLender` function with signature `isWhitelistedLender(address,address,bytes32[])` and selector `0x1db48665`
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
        name = "isWhitelistedLender",
        abi = "isWhitelistedLender(address,address,bytes32[])"
    )]
    pub struct IsWhitelistedLenderCall {
        pub pool_caller: ::ethers::core::types::Address,
        pub addr: ::ethers::core::types::Address,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `lendersRoot` function with signature `lendersRoot()` and selector `0x6067268c`
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
    #[ethcall(name = "lendersRoot", abi = "lendersRoot()")]
    pub struct LendersRootCall;
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
    ///Container type for all input parameters for the `protocolWhitelist` function with signature `protocolWhitelist(address)` and selector `0x612cb2b5`
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
    #[ethcall(name = "protocolWhitelist", abi = "protocolWhitelist(address)")]
    pub struct ProtocolWhitelistCall {
        pub protocol_controlled_address: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `revokeProtocolWhitelist` function with signature `revokeProtocolWhitelist(address)` and selector `0x96ffa8d5`
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
        name = "revokeProtocolWhitelist",
        abi = "revokeProtocolWhitelist(address)"
    )]
    pub struct RevokeProtocolWhitelistCall {
        pub addr: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `updateBorrowersRoot` function with signature `updateBorrowersRoot(uint8,bytes32)` and selector `0x8c7a0cbd`
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
    #[ethcall(name = "updateBorrowersRoot", abi = "updateBorrowersRoot(uint8,bytes32)")]
    pub struct UpdateBorrowersRootCall {
        pub ilk_index: u8,
        pub borrowers_root: [u8; 32],
    }
    ///Container type for all input parameters for the `updateLendersRoot` function with signature `updateLendersRoot(bytes32)` and selector `0x3dfd140b`
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
    #[ethcall(name = "updateLendersRoot", abi = "updateLendersRoot(bytes32)")]
    pub struct UpdateLendersRootCall {
        pub lenders_root: [u8; 32],
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
    pub enum WhitelistCalls {
        AcceptOwnership(AcceptOwnershipCall),
        ApproveProtocolWhitelist(ApproveProtocolWhitelistCall),
        BorrowersRoot(BorrowersRootCall),
        IsWhitelistedBorrower(IsWhitelistedBorrowerCall),
        IsWhitelistedLender(IsWhitelistedLenderCall),
        LendersRoot(LendersRootCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        ProtocolWhitelist(ProtocolWhitelistCall),
        RenounceOwnership(RenounceOwnershipCall),
        RevokeProtocolWhitelist(RevokeProtocolWhitelistCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateBorrowersRoot(UpdateBorrowersRootCall),
        UpdateLendersRoot(UpdateLendersRootCall),
    }
    impl ::ethers::core::abi::AbiDecode for WhitelistCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <ApproveProtocolWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproveProtocolWhitelist(decoded));
            }
            if let Ok(decoded) = <BorrowersRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BorrowersRoot(decoded));
            }
            if let Ok(decoded) = <IsWhitelistedBorrowerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsWhitelistedBorrower(decoded));
            }
            if let Ok(decoded) = <IsWhitelistedLenderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsWhitelistedLender(decoded));
            }
            if let Ok(decoded) = <LendersRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LendersRoot(decoded));
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
            if let Ok(decoded) = <ProtocolWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProtocolWhitelist(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RevokeProtocolWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeProtocolWhitelist(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateBorrowersRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateBorrowersRoot(decoded));
            }
            if let Ok(decoded) = <UpdateLendersRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateLendersRoot(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WhitelistCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveProtocolWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowersRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsWhitelistedBorrower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsWhitelistedLender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LendersRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeProtocolWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateBorrowersRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateLendersRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for WhitelistCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveProtocolWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BorrowersRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsWhitelistedBorrower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsWhitelistedLender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LendersRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeProtocolWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBorrowersRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateLendersRoot(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for WhitelistCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<ApproveProtocolWhitelistCall> for WhitelistCalls {
        fn from(value: ApproveProtocolWhitelistCall) -> Self {
            Self::ApproveProtocolWhitelist(value)
        }
    }
    impl ::core::convert::From<BorrowersRootCall> for WhitelistCalls {
        fn from(value: BorrowersRootCall) -> Self {
            Self::BorrowersRoot(value)
        }
    }
    impl ::core::convert::From<IsWhitelistedBorrowerCall> for WhitelistCalls {
        fn from(value: IsWhitelistedBorrowerCall) -> Self {
            Self::IsWhitelistedBorrower(value)
        }
    }
    impl ::core::convert::From<IsWhitelistedLenderCall> for WhitelistCalls {
        fn from(value: IsWhitelistedLenderCall) -> Self {
            Self::IsWhitelistedLender(value)
        }
    }
    impl ::core::convert::From<LendersRootCall> for WhitelistCalls {
        fn from(value: LendersRootCall) -> Self {
            Self::LendersRoot(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for WhitelistCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for WhitelistCalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<ProtocolWhitelistCall> for WhitelistCalls {
        fn from(value: ProtocolWhitelistCall) -> Self {
            Self::ProtocolWhitelist(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for WhitelistCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RevokeProtocolWhitelistCall> for WhitelistCalls {
        fn from(value: RevokeProtocolWhitelistCall) -> Self {
            Self::RevokeProtocolWhitelist(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for WhitelistCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateBorrowersRootCall> for WhitelistCalls {
        fn from(value: UpdateBorrowersRootCall) -> Self {
            Self::UpdateBorrowersRoot(value)
        }
    }
    impl ::core::convert::From<UpdateLendersRootCall> for WhitelistCalls {
        fn from(value: UpdateLendersRootCall) -> Self {
            Self::UpdateLendersRoot(value)
        }
    }
    ///Container type for all return fields from the `borrowersRoot` function with signature `borrowersRoot(uint8)` and selector `0xbd818a59`
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
    pub struct BorrowersRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isWhitelistedBorrower` function with signature `isWhitelistedBorrower(uint8,address,address,bytes32[])` and selector `0xb5406b3d`
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
    pub struct IsWhitelistedBorrowerReturn(pub bool);
    ///Container type for all return fields from the `isWhitelistedLender` function with signature `isWhitelistedLender(address,address,bytes32[])` and selector `0x1db48665`
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
    pub struct IsWhitelistedLenderReturn(pub bool);
    ///Container type for all return fields from the `lendersRoot` function with signature `lendersRoot()` and selector `0x6067268c`
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
    pub struct LendersRootReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `protocolWhitelist` function with signature `protocolWhitelist(address)` and selector `0x612cb2b5`
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
    pub struct ProtocolWhitelistReturn(pub bool);
}
