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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("InvalidConstructorArguments"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidConstructorArguments",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWhitelistMerkleProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidWhitelistMerkleProof",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x14\xCB8\x03\x80b\0\x14\xCB\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x03\xF4V[3`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\xADW`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0\xA4\x91\x90b\0\x04\x9FV[`@Q\x80\x91\x03\x90\xFD[b\0\0\xBE\x81b\0\x013` \x1B` \x1CV[P`\0[\x82Q\x81`\xFF\x16\x10\x15b\0\x01#W\x82\x81`\xFF\x16\x81Q\x81\x10b\0\0\xE8Wb\0\0\xE7b\0\x04\xBCV[[` \x02` \x01\x01Q`\x03`\0\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80\x80b\0\x01\x1A\x90b\0\x05'V[\x91PPb\0\0\xC2V[P\x80`\x04\x81\x90UPPPb\0\x05UV[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ub\0\x01i\x81b\0\x01l` \x1B` \x1CV[PV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x02\x94\x82b\0\x02IV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x02\xB6Wb\0\x02\xB5b\0\x02ZV[[\x80`@RPPPV[`\0b\0\x02\xCBb\0\x020V[\x90Pb\0\x02\xD9\x82\x82b\0\x02\x89V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x02\xFCWb\0\x02\xFBb\0\x02ZV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[b\0\x03'\x81b\0\x03\x12V[\x81\x14b\0\x033W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03G\x81b\0\x03\x1CV[\x92\x91PPV[`\0b\0\x03db\0\x03^\x84b\0\x02\xDEV[b\0\x02\xBFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x03\x8AWb\0\x03\x89b\0\x03\rV[[\x83[\x81\x81\x10\x15b\0\x03\xB7W\x80b\0\x03\xA2\x88\x82b\0\x036V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x03\x8CV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x03\xD9Wb\0\x03\xD8b\0\x02DV[[\x81Qb\0\x03\xEB\x84\x82` \x86\x01b\0\x03MV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x04\x0EWb\0\x04\rb\0\x02:V[[`\0\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x04/Wb\0\x04.b\0\x02?V[[b\0\x04=\x85\x82\x86\x01b\0\x03\xC1V[\x92PP` b\0\x04P\x85\x82\x86\x01b\0\x036V[\x91PP\x92P\x92\x90PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x04\x87\x82b\0\x04ZV[\x90P\x91\x90PV[b\0\x04\x99\x81b\0\x04zV[\x82RPPV[`\0` \x82\x01\x90Pb\0\x04\xB6`\0\x83\x01\x84b\0\x04\x8EV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x054\x82b\0\x05\x1AV[\x91P`\xFF\x82\x03b\0\x05JWb\0\x05Ib\0\x04\xEBV[[`\x01\x82\x01\x90P\x91\x90PV[a\x0Ff\x80b\0\x05e`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80cy\xBAP\x97\x11a\0\x8CW\x80c\x96\xFF\xA8\xD5\x11a\0fW\x80c\x96\xFF\xA8\xD5\x14a\x02#W\x80c\xBD\x81\x8AY\x14a\x02?W\x80c\xE3\x0C9x\x14a\x02oW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x8DWa\0\xEAV[\x80cy\xBAP\x97\x14a\x01\xDFW\x80c\x8Cz\x0C\xBD\x14a\x01\xE9W\x80c\x8D\xA5\xCB[\x14a\x02\x05Wa\0\xEAV[\x80c`g&\x8C\x11a\0\xC8W\x80c`g&\x8C\x14a\x01WW\x80ca,\xB2\xB5\x14a\x01uW\x80cqP\x18\xA6\x14a\x01\xA5W\x80cr%\x85\xD5\x14a\x01\xAFWa\0\xEAV[\x80c\x12]\xDFM\x14a\0\xEFW\x80c=\xFD\x14\x0B\x14a\x01\x1FW\x80cX'\x8A\xB6\x14a\x01;W[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x0B\xB1V[a\x02\xA9V[`@Qa\x01\x16\x91\x90a\x0C@V[`@Q\x80\x91\x03\x90\xF3[a\x019`\x04\x806\x03\x81\x01\x90a\x014\x91\x90a\x0C\x91V[a\x04,V[\0[a\x01U`\x04\x806\x03\x81\x01\x90a\x01P\x91\x90a\x0C\xBEV[a\x04>V[\0[a\x01_a\x04\xA1V[`@Qa\x01l\x91\x90a\x0C\xFAV[`@Q\x80\x91\x03\x90\xF3[a\x01\x8F`\x04\x806\x03\x81\x01\x90a\x01\x8A\x91\x90a\x0C\xBEV[a\x04\xA7V[`@Qa\x01\x9C\x91\x90a\x0C@V[`@Q\x80\x91\x03\x90\xF3[a\x01\xADa\x04\xC7V[\0[a\x01\xC9`\x04\x806\x03\x81\x01\x90a\x01\xC4\x91\x90a\r\x15V[a\x04\xDBV[`@Qa\x01\xD6\x91\x90a\x0C@V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE7a\x06DV[\0[a\x02\x03`\x04\x806\x03\x81\x01\x90a\x01\xFE\x91\x90a\ruV[a\x06\xD3V[\0[a\x02\ra\x06\xFDV[`@Qa\x02\x1A\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02=`\x04\x806\x03\x81\x01\x90a\x028\x91\x90a\x0C\xBEV[a\x07&V[\0[a\x02Y`\x04\x806\x03\x81\x01\x90a\x02T\x91\x90a\r\xDFV[a\x07\x89V[`@Qa\x02f\x91\x90a\x0C\xFAV[`@Q\x80\x91\x03\x90\xF3[a\x02wa\x07\xA1V[`@Qa\x02\x84\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA7`\x04\x806\x03\x81\x01\x90a\x02\xA2\x91\x90a\x0C\xBEV[a\x07\xCBV[\0[`\0`\x02`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x03\x06W`\x01\x90Pa\x04$V[`\0`\x03`\0\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x80\x1B\x81\x03a\x038W`\x01\x91PPa\x04$V[`\0\x85`@Q` \x01a\x03K\x91\x90a\r\xC4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x03q\x91\x90a\x0E-V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x03\xD5\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08xV[\x15a\x03\xE5W`\x01\x92PPPa\x04$V[\x86\x86`@Q\x7F\x7F\xBDtD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x1B\x92\x91\x90a\x0EWV[`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[a\x044a\x08\x8FV[\x80`\x04\x81\x90UPPV[a\x04Fa\x08\x8FV[`\x01`\x02`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x04T\x81V[`\x02` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[a\x04\xCFa\x08\x8FV[a\x04\xD9`\0a\t\x16V[V[`\0`\x02`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x058W`\x01\x90Pa\x06=V[`\0`\x04T\x90P`\0\x80\x1B\x81\x03a\x05SW`\x01\x91PPa\x06=V[`\0\x85`@Q` \x01a\x05f\x91\x90a\r\xC4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x05\x8C\x91\x90a\x0E-V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x05\xF0\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08xV[\x15a\x06\0W`\x01\x92PPPa\x06=V[\x85`@Q\x7F\xA2\xA9?\xD3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x064\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[`\0a\x06Na\tGV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06oa\x07\xA1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xC7W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xBE\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xFD[a\x06\xD0\x81a\t\x16V[PV[a\x06\xDBa\x08\x8FV[\x80`\x03`\0\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPPPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x07.a\x08\x8FV[`\0`\x02`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x03` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x07\xD3a\x08\x8FV[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x083a\x06\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[`\0\x82a\x08\x85\x85\x84a\tOV[\x14\x90P\x93\x92PPPV[a\x08\x97a\tGV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xB5a\x06\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\x14Wa\x08\xD8a\tGV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x0B\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xFD[V[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\tD\x81a\t\xA5V[PV[`\x003\x90P\x90V[`\0\x80\x82\x90P`\0[\x84Q\x81\x10\x15a\t\x9AWa\t\x85\x82\x86\x83\x81Q\x81\x10a\txWa\twa\x0E\x80V[[` \x02` \x01\x01Qa\niV[\x91P\x80\x80a\t\x92\x90a\x0E\xE8V[\x91PPa\tXV[P\x80\x91PP\x92\x91PPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81\x83\x10a\n\x81Wa\n|\x82\x84a\n\x94V[a\n\x8CV[a\n\x8B\x83\x83a\n\x94V[[\x90P\x92\x91PPV[`\0\x82`\0R\x81` R`@`\0 \x90P\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\n\xCB\x81a\n\xB5V[\x81\x14a\n\xD6W`\0\x80\xFD[PV[`\0\x815\x90Pa\n\xE8\x81a\n\xC2V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0B\x19\x82a\n\xEEV[\x90P\x91\x90PV[a\x0B)\x81a\x0B\x0EV[\x81\x14a\x0B4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0BF\x81a\x0B V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x0BqWa\x0Bpa\x0BLV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x8EWa\x0B\x8Da\x0BQV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x0B\xAAWa\x0B\xA9a\x0BVV[[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0B\xCBWa\x0B\xCAa\n\xABV[[`\0a\x0B\xD9\x87\x82\x88\x01a\n\xD9V[\x94PP` a\x0B\xEA\x87\x82\x88\x01a\x0B7V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x0BWa\x0C\na\n\xB0V[[a\x0C\x17\x87\x82\x88\x01a\x0B[V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0C:\x81a\x0C%V[\x82RPPV[`\0` \x82\x01\x90Pa\x0CU`\0\x83\x01\x84a\x0C1V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0Cn\x81a\x0C[V[\x81\x14a\x0CyW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\x8B\x81a\x0CeV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xA7Wa\x0C\xA6a\n\xABV[[`\0a\x0C\xB5\x84\x82\x85\x01a\x0C|V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xD4Wa\x0C\xD3a\n\xABV[[`\0a\x0C\xE2\x84\x82\x85\x01a\x0B7V[\x91PP\x92\x91PPV[a\x0C\xF4\x81a\x0C[V[\x82RPPV[`\0` \x82\x01\x90Pa\r\x0F`\0\x83\x01\x84a\x0C\xEBV[\x92\x91PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\r.Wa\r-a\n\xABV[[`\0a\r<\x86\x82\x87\x01a\x0B7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r]Wa\r\\a\n\xB0V[[a\ri\x86\x82\x87\x01a\x0B[V[\x92P\x92PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\r\x8CWa\r\x8Ba\n\xABV[[`\0a\r\x9A\x85\x82\x86\x01a\n\xD9V[\x92PP` a\r\xAB\x85\x82\x86\x01a\x0C|V[\x91PP\x92P\x92\x90PV[a\r\xBE\x81a\x0B\x0EV[\x82RPPV[`\0` \x82\x01\x90Pa\r\xD9`\0\x83\x01\x84a\r\xB5V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\r\xF5Wa\r\xF4a\n\xABV[[`\0a\x0E\x03\x84\x82\x85\x01a\n\xD9V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0E'a\x0E\"\x82a\x0C[V[a\x0E\x0CV[\x82RPPV[`\0a\x0E9\x82\x84a\x0E\x16V[` \x82\x01\x91P\x81\x90P\x92\x91PPV[a\x0EQ\x81a\n\xB5V[\x82RPPV[`\0`@\x82\x01\x90Pa\x0El`\0\x83\x01\x85a\x0EHV[a\x0Ey` \x83\x01\x84a\r\xB5V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x90P\x91\x90PV[`\0a\x0E\xF3\x82a\x0E\xDEV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0F%Wa\x0F$a\x0E\xAFV[[`\x01\x82\x01\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x0B\xCA\n\xB4\x84\xE4\xB4i)\xC7\xB6\xB1\x137\xA68\xAD\xA1\x80\xC3\xEF\x9F\xF6N6yV\xB4\x95\x8D\xB1\xB7dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static WHITELIST_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80cy\xBAP\x97\x11a\0\x8CW\x80c\x96\xFF\xA8\xD5\x11a\0fW\x80c\x96\xFF\xA8\xD5\x14a\x02#W\x80c\xBD\x81\x8AY\x14a\x02?W\x80c\xE3\x0C9x\x14a\x02oW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x8DWa\0\xEAV[\x80cy\xBAP\x97\x14a\x01\xDFW\x80c\x8Cz\x0C\xBD\x14a\x01\xE9W\x80c\x8D\xA5\xCB[\x14a\x02\x05Wa\0\xEAV[\x80c`g&\x8C\x11a\0\xC8W\x80c`g&\x8C\x14a\x01WW\x80ca,\xB2\xB5\x14a\x01uW\x80cqP\x18\xA6\x14a\x01\xA5W\x80cr%\x85\xD5\x14a\x01\xAFWa\0\xEAV[\x80c\x12]\xDFM\x14a\0\xEFW\x80c=\xFD\x14\x0B\x14a\x01\x1FW\x80cX'\x8A\xB6\x14a\x01;W[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x0B\xB1V[a\x02\xA9V[`@Qa\x01\x16\x91\x90a\x0C@V[`@Q\x80\x91\x03\x90\xF3[a\x019`\x04\x806\x03\x81\x01\x90a\x014\x91\x90a\x0C\x91V[a\x04,V[\0[a\x01U`\x04\x806\x03\x81\x01\x90a\x01P\x91\x90a\x0C\xBEV[a\x04>V[\0[a\x01_a\x04\xA1V[`@Qa\x01l\x91\x90a\x0C\xFAV[`@Q\x80\x91\x03\x90\xF3[a\x01\x8F`\x04\x806\x03\x81\x01\x90a\x01\x8A\x91\x90a\x0C\xBEV[a\x04\xA7V[`@Qa\x01\x9C\x91\x90a\x0C@V[`@Q\x80\x91\x03\x90\xF3[a\x01\xADa\x04\xC7V[\0[a\x01\xC9`\x04\x806\x03\x81\x01\x90a\x01\xC4\x91\x90a\r\x15V[a\x04\xDBV[`@Qa\x01\xD6\x91\x90a\x0C@V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE7a\x06DV[\0[a\x02\x03`\x04\x806\x03\x81\x01\x90a\x01\xFE\x91\x90a\ruV[a\x06\xD3V[\0[a\x02\ra\x06\xFDV[`@Qa\x02\x1A\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02=`\x04\x806\x03\x81\x01\x90a\x028\x91\x90a\x0C\xBEV[a\x07&V[\0[a\x02Y`\x04\x806\x03\x81\x01\x90a\x02T\x91\x90a\r\xDFV[a\x07\x89V[`@Qa\x02f\x91\x90a\x0C\xFAV[`@Q\x80\x91\x03\x90\xF3[a\x02wa\x07\xA1V[`@Qa\x02\x84\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA7`\x04\x806\x03\x81\x01\x90a\x02\xA2\x91\x90a\x0C\xBEV[a\x07\xCBV[\0[`\0`\x02`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x03\x06W`\x01\x90Pa\x04$V[`\0`\x03`\0\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x80\x1B\x81\x03a\x038W`\x01\x91PPa\x04$V[`\0\x85`@Q` \x01a\x03K\x91\x90a\r\xC4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x03q\x91\x90a\x0E-V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x03\xD5\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08xV[\x15a\x03\xE5W`\x01\x92PPPa\x04$V[\x86\x86`@Q\x7F\x7F\xBDtD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x1B\x92\x91\x90a\x0EWV[`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[a\x044a\x08\x8FV[\x80`\x04\x81\x90UPPV[a\x04Fa\x08\x8FV[`\x01`\x02`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x04T\x81V[`\x02` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[a\x04\xCFa\x08\x8FV[a\x04\xD9`\0a\t\x16V[V[`\0`\x02`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x058W`\x01\x90Pa\x06=V[`\0`\x04T\x90P`\0\x80\x1B\x81\x03a\x05SW`\x01\x91PPa\x06=V[`\0\x85`@Q` \x01a\x05f\x91\x90a\r\xC4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x05\x8C\x91\x90a\x0E-V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x05\xF0\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83\x83a\x08xV[\x15a\x06\0W`\x01\x92PPPa\x06=V[\x85`@Q\x7F\xA2\xA9?\xD3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x064\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[`\0a\x06Na\tGV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06oa\x07\xA1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xC7W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xBE\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xFD[a\x06\xD0\x81a\t\x16V[PV[a\x06\xDBa\x08\x8FV[\x80`\x03`\0\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPPPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x07.a\x08\x8FV[`\0`\x02`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x03` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x07\xD3a\x08\x8FV[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x083a\x06\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[`\0\x82a\x08\x85\x85\x84a\tOV[\x14\x90P\x93\x92PPPV[a\x08\x97a\tGV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xB5a\x06\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\x14Wa\x08\xD8a\tGV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x0B\x91\x90a\r\xC4V[`@Q\x80\x91\x03\x90\xFD[V[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\tD\x81a\t\xA5V[PV[`\x003\x90P\x90V[`\0\x80\x82\x90P`\0[\x84Q\x81\x10\x15a\t\x9AWa\t\x85\x82\x86\x83\x81Q\x81\x10a\txWa\twa\x0E\x80V[[` \x02` \x01\x01Qa\niV[\x91P\x80\x80a\t\x92\x90a\x0E\xE8V[\x91PPa\tXV[P\x80\x91PP\x92\x91PPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81\x83\x10a\n\x81Wa\n|\x82\x84a\n\x94V[a\n\x8CV[a\n\x8B\x83\x83a\n\x94V[[\x90P\x92\x91PPV[`\0\x82`\0R\x81` R`@`\0 \x90P\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\n\xCB\x81a\n\xB5V[\x81\x14a\n\xD6W`\0\x80\xFD[PV[`\0\x815\x90Pa\n\xE8\x81a\n\xC2V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0B\x19\x82a\n\xEEV[\x90P\x91\x90PV[a\x0B)\x81a\x0B\x0EV[\x81\x14a\x0B4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0BF\x81a\x0B V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x0BqWa\x0Bpa\x0BLV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x8EWa\x0B\x8Da\x0BQV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x0B\xAAWa\x0B\xA9a\x0BVV[[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0B\xCBWa\x0B\xCAa\n\xABV[[`\0a\x0B\xD9\x87\x82\x88\x01a\n\xD9V[\x94PP` a\x0B\xEA\x87\x82\x88\x01a\x0B7V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x0BWa\x0C\na\n\xB0V[[a\x0C\x17\x87\x82\x88\x01a\x0B[V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0C:\x81a\x0C%V[\x82RPPV[`\0` \x82\x01\x90Pa\x0CU`\0\x83\x01\x84a\x0C1V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0Cn\x81a\x0C[V[\x81\x14a\x0CyW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0C\x8B\x81a\x0CeV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xA7Wa\x0C\xA6a\n\xABV[[`\0a\x0C\xB5\x84\x82\x85\x01a\x0C|V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xD4Wa\x0C\xD3a\n\xABV[[`\0a\x0C\xE2\x84\x82\x85\x01a\x0B7V[\x91PP\x92\x91PPV[a\x0C\xF4\x81a\x0C[V[\x82RPPV[`\0` \x82\x01\x90Pa\r\x0F`\0\x83\x01\x84a\x0C\xEBV[\x92\x91PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\r.Wa\r-a\n\xABV[[`\0a\r<\x86\x82\x87\x01a\x0B7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r]Wa\r\\a\n\xB0V[[a\ri\x86\x82\x87\x01a\x0B[V[\x92P\x92PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\r\x8CWa\r\x8Ba\n\xABV[[`\0a\r\x9A\x85\x82\x86\x01a\n\xD9V[\x92PP` a\r\xAB\x85\x82\x86\x01a\x0C|V[\x91PP\x92P\x92\x90PV[a\r\xBE\x81a\x0B\x0EV[\x82RPPV[`\0` \x82\x01\x90Pa\r\xD9`\0\x83\x01\x84a\r\xB5V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\r\xF5Wa\r\xF4a\n\xABV[[`\0a\x0E\x03\x84\x82\x85\x01a\n\xD9V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0E'a\x0E\"\x82a\x0C[V[a\x0E\x0CV[\x82RPPV[`\0a\x0E9\x82\x84a\x0E\x16V[` \x82\x01\x91P\x81\x90P\x92\x91PPV[a\x0EQ\x81a\n\xB5V[\x82RPPV[`\0`@\x82\x01\x90Pa\x0El`\0\x83\x01\x85a\x0EHV[a\x0Ey` \x83\x01\x84a\r\xB5V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x90P\x91\x90PV[`\0a\x0E\xF3\x82a\x0E\xDEV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0F%Wa\x0F$a\x0E\xAFV[[`\x01\x82\x01\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x0B\xCA\n\xB4\x84\xE4\xB4i)\xC7\xB6\xB1\x137\xA68\xAD\xA1\x80\xC3\xEF\x9F\xF6N6yV\xB4\x95\x8D\xB1\xB7dsolcC\0\x08\x15\x003";
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
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([189, 129, 138, 89], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWhitelistedBorrower` (0x125ddf4d) function
        pub fn is_whitelisted_borrower(
            &self,
            ilk_index: u8,
            addr: ::ethers::core::types::Address,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([18, 93, 223, 77], (ilk_index, addr, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWhitelistedLender` (0x722585d5) function
        pub fn is_whitelisted_lender(
            &self,
            addr: ::ethers::core::types::Address,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([114, 37, 133, 213], (addr, proof))
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
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([97, 44, 178, 181], p0)
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
    ///Custom Error type `InvalidConstructorArguments` with signature `InvalidConstructorArguments()` and selector `0x0c8f7609`
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
        name = "InvalidConstructorArguments",
        abi = "InvalidConstructorArguments()"
    )]
    pub struct InvalidConstructorArguments;
    ///Custom Error type `InvalidWhitelistMerkleProof` with signature `InvalidWhitelistMerkleProof()` and selector `0x5d33d064`
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
        name = "InvalidWhitelistMerkleProof",
        abi = "InvalidWhitelistMerkleProof()"
    )]
    pub struct InvalidWhitelistMerkleProof;
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
        InvalidConstructorArguments(InvalidConstructorArguments),
        InvalidWhitelistMerkleProof(InvalidWhitelistMerkleProof),
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
            if let Ok(decoded) = <InvalidConstructorArguments as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidConstructorArguments(decoded));
            }
            if let Ok(decoded) = <InvalidWhitelistMerkleProof as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidWhitelistMerkleProof(decoded));
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
                Self::InvalidConstructorArguments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWhitelistMerkleProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                    == <InvalidConstructorArguments as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidWhitelistMerkleProof as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                Self::InvalidConstructorArguments(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidWhitelistMerkleProof(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<InvalidConstructorArguments> for WhitelistErrors {
        fn from(value: InvalidConstructorArguments) -> Self {
            Self::InvalidConstructorArguments(value)
        }
    }
    impl ::core::convert::From<InvalidWhitelistMerkleProof> for WhitelistErrors {
        fn from(value: InvalidWhitelistMerkleProof) -> Self {
            Self::InvalidWhitelistMerkleProof(value)
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
    pub struct BorrowersRootCall(pub u8);
    ///Container type for all input parameters for the `isWhitelistedBorrower` function with signature `isWhitelistedBorrower(uint8,address,bytes32[])` and selector `0x125ddf4d`
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
        abi = "isWhitelistedBorrower(uint8,address,bytes32[])"
    )]
    pub struct IsWhitelistedBorrowerCall {
        pub ilk_index: u8,
        pub addr: ::ethers::core::types::Address,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `isWhitelistedLender` function with signature `isWhitelistedLender(address,bytes32[])` and selector `0x722585d5`
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
        abi = "isWhitelistedLender(address,bytes32[])"
    )]
    pub struct IsWhitelistedLenderCall {
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
    pub struct ProtocolWhitelistCall(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `isWhitelistedBorrower` function with signature `isWhitelistedBorrower(uint8,address,bytes32[])` and selector `0x125ddf4d`
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
    ///Container type for all return fields from the `isWhitelistedLender` function with signature `isWhitelistedLender(address,bytes32[])` and selector `0x722585d5`
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
