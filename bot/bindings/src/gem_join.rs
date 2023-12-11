pub use gem_join::*;
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
pub mod gem_join {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IonPool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gem"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_ilkIndex"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("GEM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("GEM"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ILK_INDEX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ILK_INDEX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL"),
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
                    ::std::borrow::ToOwned::to_owned("exit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                (
                    ::std::borrow::ToOwned::to_owned("join"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("join"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("totalGem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalGem"),
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
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
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
                    ::std::borrow::ToOwned::to_owned("EnforcedPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EnforcedPause"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExpectedPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExpectedPause"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("Int256Overflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Int256Overflow"),
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
                (
                    ::std::borrow::ToOwned::to_owned("WrongIlkAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("WrongIlkAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gem"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
    pub static GEMJOIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1998\x03\x80b\0\x199\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x04|V[\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\xADW`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0\xA4\x91\x90b\0\x04\xFFV[`@Q\x80\x91\x03\x90\xFD[b\0\0\xBE\x81b\0\x02M` \x1B` \x1CV[P`\0`\x01`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81`\xFF\x16`\xC0\x81`\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xEF\xFF\0_\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x01\xA2\x91\x90b\0\x05iV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xE6\x91\x90b\0\x05\x86V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x02CW\x81\x83`@Q\x7F[\xAD\xBF\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x02:\x92\x91\x90b\0\x06*V[`@Q\x80\x91\x03\x90\xFD[PPPPb\0\x06WV[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ub\0\x02\x83\x81b\0\x02\x86` \x1B` \x1CV[PV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x03|\x82b\0\x03OV[\x90P\x91\x90PV[`\0b\0\x03\x90\x82b\0\x03oV[\x90P\x91\x90PV[b\0\x03\xA2\x81b\0\x03\x83V[\x81\x14b\0\x03\xAEW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\xC2\x81b\0\x03\x97V[\x92\x91PPV[`\0b\0\x03\xD5\x82b\0\x03oV[\x90P\x91\x90PV[b\0\x03\xE7\x81b\0\x03\xC8V[\x81\x14b\0\x03\xF3W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x04\x07\x81b\0\x03\xDCV[\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[b\0\x04%\x81b\0\x04\rV[\x81\x14b\0\x041W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x04E\x81b\0\x04\x1AV[\x92\x91PPV[b\0\x04V\x81b\0\x03oV[\x81\x14b\0\x04bW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x04v\x81b\0\x04KV[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x04\x99Wb\0\x04\x98b\0\x03JV[[`\0b\0\x04\xA9\x87\x82\x88\x01b\0\x03\xB1V[\x94PP` b\0\x04\xBC\x87\x82\x88\x01b\0\x03\xF6V[\x93PP`@b\0\x04\xCF\x87\x82\x88\x01b\0\x044V[\x92PP``b\0\x04\xE2\x87\x82\x88\x01b\0\x04eV[\x91PP\x92\x95\x91\x94P\x92PV[b\0\x04\xF9\x81b\0\x03oV[\x82RPPV[`\0` \x82\x01\x90Pb\0\x05\x16`\0\x83\x01\x84b\0\x04\xEEV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0b\0\x05Qb\0\x05Kb\0\x05E\x84b\0\x04\rV[b\0\x05&V[b\0\x05\x1CV[\x90P\x91\x90PV[b\0\x05c\x81b\0\x050V[\x82RPPV[`\0` \x82\x01\x90Pb\0\x05\x80`\0\x83\x01\x84b\0\x05XV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x05\x9FWb\0\x05\x9Eb\0\x03JV[[`\0b\0\x05\xAF\x84\x82\x85\x01b\0\x04eV[\x91PP\x92\x91PPV[b\0\x05\xC3\x81b\0\x04\rV[\x82RPPV[`\0b\0\x05\xEAb\0\x05\xE4b\0\x05\xDE\x84b\0\x03OV[b\0\x05&V[b\0\x03OV[\x90P\x91\x90PV[`\0b\0\x05\xFE\x82b\0\x05\xC9V[\x90P\x91\x90PV[`\0b\0\x06\x12\x82b\0\x05\xF1V[\x90P\x91\x90PV[b\0\x06$\x81b\0\x06\x05V[\x82RPPV[`\0`@\x82\x01\x90Pb\0\x06A`\0\x83\x01\x85b\0\x05\xB8V[b\0\x06P` \x83\x01\x84b\0\x06\x19V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Qa\x12\x88b\0\x06\xB1`\09`\0\x81\x81a\x02\xD7\x01R\x81\x81a\x05\x1A\x01Ra\x05\xD6\x01R`\0\x81\x81a\x02\x9B\x01R\x81\x81a\x03\xFC\x01Ra\x05\x9A\x01R`\0\x81\x81a\x03P\x01R\x81\x81a\x03\xAD\x01Ra\x06W\x01Ra\x12\x88`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x83\xE8\xD3\xB8\x11a\0\x8CW\x80c\xE3\x0C9x\x11a\0fW\x80c\xE3\x0C9x\x14a\x01\xC9W\x80c\xED\x0C\xEE\x97\x14a\x01\xE7W\x80c\xEFi;\xED\x14a\x02\x05W\x80c\xF2\xFD\xE3\x8B\x14a\x02!Wa\0\xEAV[\x80c\x83\xE8\xD3\xB8\x14a\x01\x83W\x80c\x84V\xCBY\x14a\x01\xA1W\x80c\x8D\xA5\xCB[\x14a\x01\xABWa\0\xEAV[\x80c\\\x97Z\xBB\x11a\0\xC8W\x80c\\\x97Z\xBB\x14a\x013W\x80cqP\x18\xA6\x14a\x01QW\x80cu5\xD2F\x14a\x01[W\x80cy\xBAP\x97\x14a\x01yWa\0\xEAV[\x80c;M\xA6\x9F\x14a\0\xEFW\x80c?K\xA8:\x14a\x01\x0BW\x80cM\xC6T\x11\x14a\x01\x15W[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\r\xFEV[a\x02=V[\0[a\x01\x13a\x03\x99V[\0[a\x01\x1Da\x03\xABV[`@Qa\x01*\x91\x90a\x0E\x9DV[`@Q\x80\x91\x03\x90\xF3[a\x01;a\x03\xCFV[`@Qa\x01H\x91\x90a\x0E\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x01Ya\x03\xE6V[\0[a\x01ca\x03\xFAV[`@Qa\x01p\x91\x90a\x0F\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x01\x81a\x04\x1EV[\0[a\x01\x8Ba\x04\xADV[`@Qa\x01\x98\x91\x90a\x0F9V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA9a\x04\xB3V[\0[a\x01\xB3a\x04\xC5V[`@Qa\x01\xC0\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x01\xD1a\x04\xEEV[`@Qa\x01\xDE\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x01\xEFa\x05\x18V[`@Qa\x01\xFC\x91\x90a\x0F\x9AV[`@Q\x80\x91\x03\x90\xF3[a\x02\x1F`\x04\x806\x03\x81\x01\x90a\x02\x1A\x91\x90a\r\xFEV[a\x05<V[\0[a\x02;`\x04\x806\x03\x81\x01\x90a\x026\x91\x90a\x0F\xB5V[a\x06\x9FV[\0[a\x02Ea\x07LV[`\0\x81\x12\x15a\x02\x80W`@Q\x7F\x17\xAA\x17%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02`\0\x82\x82Ta\x02\x92\x91\x90a\x10\x11V[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA7\x16'(\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x16\x93\x92\x91\x90a\x10^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x030W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03DW=`\0\x80>=`\0\xFD[PPPPa\x03\x9530\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x8D\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPV[a\x03\xA1a\x08\x0FV[a\x03\xA9a\x08\x96V[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[a\x03\xEEa\x08\x0FV[a\x03\xF8`\0a\x08\xF9V[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x04(a\t*V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04Ia\x04\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\xA1W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x98\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[a\x04\xAA\x81a\x08\xF9V[PV[`\x02T\x81V[a\x04\xBBa\x08\x0FV[a\x04\xC3a\t2V[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05Da\x07LV[`\0\x81\x12\x15a\x05\x7FW`@Q\x7F\x17\xAA\x17%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02`\0\x82\x82Ta\x05\x91\x91\x90a\x10\x95V[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA7\x16'(\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x84a\x06\0\x90a\x10\xC9V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\x1E\x93\x92\x91\x90a\x10^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x068W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06LW=`\0\x80>=`\0\xFD[PPPPa\x06\x9B\x82\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\x94\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPV[a\x06\xA7a\x08\x0FV[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x07a\x04\xC5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[a\x07Ta\x03\xCFV[\x15a\x07\x8BW`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x08\t\x84\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a\x07\xC2\x93\x92\x91\x90a\x11\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\n\x13V[PPPPV[a\x08\x17a\t*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x085a\x04\xC5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\x94Wa\x08Xa\t*V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x8B\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[V[a\x08\x9Ea\n\xAAV[`\0`\x01`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa\x08\xE2a\t*V[`@Qa\x08\xEF\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xA1V[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\t'\x81a\n\xEAV[PV[`\x003\x90P\x90V[a\t:a\x07LV[`\x01\x80`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\t}a\t*V[`@Qa\t\x8A\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xA1V[a\n\x0E\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\t\xC7\x92\x91\x90a\x11HV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\n\x13V[PPPV[`\0a\n>\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xAE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\ncWP\x80\x80` \x01\x90Q\x81\x01\x90a\na\x91\x90a\x11\x9DV[\x15[\x15a\n\xA5W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x9C\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[PPPV[a\n\xB2a\x03\xCFV[a\n\xE8W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[``a\x0B\xBC\x83\x83`\0a\x0B\xC4V[\x90P\x92\x91PPV[``\x81G\x10\x15a\x0C\x0BW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x02\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\x0C4\x91\x90a\x12;V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0CqW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CvV[``\x91P[P\x91P\x91Pa\x0C\x86\x86\x83\x83a\x0C\x91V[\x92PPP\x93\x92PPPV[``\x82a\x0C\xA6Wa\x0C\xA1\x82a\r V[a\r\x18V[`\0\x82Q\x14\x80\x15a\x0C\xCEWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\r\x10W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x07\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\r\x19V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\r3W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\r\x95\x82a\rjV[\x90P\x91\x90PV[a\r\xA5\x81a\r\x8AV[\x81\x14a\r\xB0W`\0\x80\xFD[PV[`\0\x815\x90Pa\r\xC2\x81a\r\x9CV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\r\xDB\x81a\r\xC8V[\x81\x14a\r\xE6W`\0\x80\xFD[PV[`\0\x815\x90Pa\r\xF8\x81a\r\xD2V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x15Wa\x0E\x14a\reV[[`\0a\x0E#\x85\x82\x86\x01a\r\xB3V[\x92PP` a\x0E4\x85\x82\x86\x01a\r\xE9V[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x0Eca\x0E^a\x0EY\x84a\rjV[a\x0E>V[a\rjV[\x90P\x91\x90PV[`\0a\x0Eu\x82a\x0EHV[\x90P\x91\x90PV[`\0a\x0E\x87\x82a\x0EjV[\x90P\x91\x90PV[a\x0E\x97\x81a\x0E|V[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xB2`\0\x83\x01\x84a\x0E\x8EV[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0E\xCD\x81a\x0E\xB8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xE8`\0\x83\x01\x84a\x0E\xC4V[\x92\x91PPV[`\0a\x0E\xF9\x82a\x0EjV[\x90P\x91\x90PV[a\x0F\t\x81a\x0E\xEEV[\x82RPPV[`\0` \x82\x01\x90Pa\x0F$`\0\x83\x01\x84a\x0F\0V[\x92\x91PPV[a\x0F3\x81a\r\xC8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0FN`\0\x83\x01\x84a\x0F*V[\x92\x91PPV[a\x0F]\x81a\r\x8AV[\x82RPPV[`\0` \x82\x01\x90Pa\x0Fx`\0\x83\x01\x84a\x0FTV[\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x0F\x94\x81a\x0F~V[\x82RPPV[`\0` \x82\x01\x90Pa\x0F\xAF`\0\x83\x01\x84a\x0F\x8BV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\xCBWa\x0F\xCAa\reV[[`\0a\x0F\xD9\x84\x82\x85\x01a\r\xB3V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x10\x1C\x82a\r\xC8V[\x91Pa\x10'\x83a\r\xC8V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x10?Wa\x10>a\x0F\xE2V[[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x10X\x81a\x10EV[\x82RPPV[`\0``\x82\x01\x90Pa\x10s`\0\x83\x01\x86a\x0F\x8BV[a\x10\x80` \x83\x01\x85a\x0FTV[a\x10\x8D`@\x83\x01\x84a\x10OV[\x94\x93PPPPV[`\0a\x10\xA0\x82a\r\xC8V[\x91Pa\x10\xAB\x83a\r\xC8V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x10\xC3Wa\x10\xC2a\x0F\xE2V[[\x92\x91PPV[`\0a\x10\xD4\x82a\x10EV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x11\x06Wa\x11\x05a\x0F\xE2V[[\x81`\0\x03\x90P\x91\x90PV[`\0``\x82\x01\x90Pa\x11&`\0\x83\x01\x86a\x0FTV[a\x113` \x83\x01\x85a\x0FTV[a\x11@`@\x83\x01\x84a\x0F*V[\x94\x93PPPPV[`\0`@\x82\x01\x90Pa\x11]`\0\x83\x01\x85a\x0FTV[a\x11j` \x83\x01\x84a\x0F*V[\x93\x92PPPV[a\x11z\x81a\x0E\xB8V[\x81\x14a\x11\x85W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x11\x97\x81a\x11qV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xB3Wa\x11\xB2a\reV[[`\0a\x11\xC1\x84\x82\x85\x01a\x11\x88V[\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11\xFEW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x11\xE3V[`\0\x84\x84\x01RPPPPV[`\0a\x12\x15\x82a\x11\xCAV[a\x12\x1F\x81\x85a\x11\xD5V[\x93Pa\x12/\x81\x85` \x86\x01a\x11\xE0V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x12G\x82\x84a\x12\nV[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 /\x14(\xFD&P\xB8p\xDBX\xABD\x06l\x16\xD60K\x16\x0F5zm\xF1\x17h\x1E\xACu\xB7\xADpdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static GEMJOIN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x83\xE8\xD3\xB8\x11a\0\x8CW\x80c\xE3\x0C9x\x11a\0fW\x80c\xE3\x0C9x\x14a\x01\xC9W\x80c\xED\x0C\xEE\x97\x14a\x01\xE7W\x80c\xEFi;\xED\x14a\x02\x05W\x80c\xF2\xFD\xE3\x8B\x14a\x02!Wa\0\xEAV[\x80c\x83\xE8\xD3\xB8\x14a\x01\x83W\x80c\x84V\xCBY\x14a\x01\xA1W\x80c\x8D\xA5\xCB[\x14a\x01\xABWa\0\xEAV[\x80c\\\x97Z\xBB\x11a\0\xC8W\x80c\\\x97Z\xBB\x14a\x013W\x80cqP\x18\xA6\x14a\x01QW\x80cu5\xD2F\x14a\x01[W\x80cy\xBAP\x97\x14a\x01yWa\0\xEAV[\x80c;M\xA6\x9F\x14a\0\xEFW\x80c?K\xA8:\x14a\x01\x0BW\x80cM\xC6T\x11\x14a\x01\x15W[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\r\xFEV[a\x02=V[\0[a\x01\x13a\x03\x99V[\0[a\x01\x1Da\x03\xABV[`@Qa\x01*\x91\x90a\x0E\x9DV[`@Q\x80\x91\x03\x90\xF3[a\x01;a\x03\xCFV[`@Qa\x01H\x91\x90a\x0E\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x01Ya\x03\xE6V[\0[a\x01ca\x03\xFAV[`@Qa\x01p\x91\x90a\x0F\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x01\x81a\x04\x1EV[\0[a\x01\x8Ba\x04\xADV[`@Qa\x01\x98\x91\x90a\x0F9V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA9a\x04\xB3V[\0[a\x01\xB3a\x04\xC5V[`@Qa\x01\xC0\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x01\xD1a\x04\xEEV[`@Qa\x01\xDE\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x01\xEFa\x05\x18V[`@Qa\x01\xFC\x91\x90a\x0F\x9AV[`@Q\x80\x91\x03\x90\xF3[a\x02\x1F`\x04\x806\x03\x81\x01\x90a\x02\x1A\x91\x90a\r\xFEV[a\x05<V[\0[a\x02;`\x04\x806\x03\x81\x01\x90a\x026\x91\x90a\x0F\xB5V[a\x06\x9FV[\0[a\x02Ea\x07LV[`\0\x81\x12\x15a\x02\x80W`@Q\x7F\x17\xAA\x17%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02`\0\x82\x82Ta\x02\x92\x91\x90a\x10\x11V[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA7\x16'(\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x16\x93\x92\x91\x90a\x10^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x030W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03DW=`\0\x80>=`\0\xFD[PPPPa\x03\x9530\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x8D\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPV[a\x03\xA1a\x08\x0FV[a\x03\xA9a\x08\x96V[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[a\x03\xEEa\x08\x0FV[a\x03\xF8`\0a\x08\xF9V[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x04(a\t*V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04Ia\x04\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\xA1W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x98\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[a\x04\xAA\x81a\x08\xF9V[PV[`\x02T\x81V[a\x04\xBBa\x08\x0FV[a\x04\xC3a\t2V[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05Da\x07LV[`\0\x81\x12\x15a\x05\x7FW`@Q\x7F\x17\xAA\x17%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02`\0\x82\x82Ta\x05\x91\x91\x90a\x10\x95V[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA7\x16'(\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x84a\x06\0\x90a\x10\xC9V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\x1E\x93\x92\x91\x90a\x10^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x068W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06LW=`\0\x80>=`\0\xFD[PPPPa\x06\x9B\x82\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\x94\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPV[a\x06\xA7a\x08\x0FV[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\x07a\x04\xC5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[a\x07Ta\x03\xCFV[\x15a\x07\x8BW`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x08\t\x84\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a\x07\xC2\x93\x92\x91\x90a\x11\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\n\x13V[PPPPV[a\x08\x17a\t*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x085a\x04\xC5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\x94Wa\x08Xa\t*V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x8B\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[V[a\x08\x9Ea\n\xAAV[`\0`\x01`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa\x08\xE2a\t*V[`@Qa\x08\xEF\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xA1V[`\x01`\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua\t'\x81a\n\xEAV[PV[`\x003\x90P\x90V[a\t:a\x07LV[`\x01\x80`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\t}a\t*V[`@Qa\t\x8A\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xA1V[a\n\x0E\x83\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x85\x85`@Q`$\x01a\t\xC7\x92\x91\x90a\x11HV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPPa\n\x13V[PPPV[`\0a\n>\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xAE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x81Q\x14\x15\x80\x15a\ncWP\x80\x80` \x01\x90Q\x81\x01\x90a\na\x91\x90a\x11\x9DV[\x15[\x15a\n\xA5W\x82`@Q\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x9C\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[PPPV[a\n\xB2a\x03\xCFV[a\n\xE8W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[``a\x0B\xBC\x83\x83`\0a\x0B\xC4V[\x90P\x92\x91PPV[``\x81G\x10\x15a\x0C\x0BW0`@Q\x7F\xCDx`Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x02\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x86`@Qa\x0C4\x91\x90a\x12;V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0CqW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CvV[``\x91P[P\x91P\x91Pa\x0C\x86\x86\x83\x83a\x0C\x91V[\x92PPP\x93\x92PPPV[``\x82a\x0C\xA6Wa\x0C\xA1\x82a\r V[a\r\x18V[`\0\x82Q\x14\x80\x15a\x0C\xCEWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\r\x10W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x07\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\r\x19V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\r3W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\r\x95\x82a\rjV[\x90P\x91\x90PV[a\r\xA5\x81a\r\x8AV[\x81\x14a\r\xB0W`\0\x80\xFD[PV[`\0\x815\x90Pa\r\xC2\x81a\r\x9CV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\r\xDB\x81a\r\xC8V[\x81\x14a\r\xE6W`\0\x80\xFD[PV[`\0\x815\x90Pa\r\xF8\x81a\r\xD2V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x15Wa\x0E\x14a\reV[[`\0a\x0E#\x85\x82\x86\x01a\r\xB3V[\x92PP` a\x0E4\x85\x82\x86\x01a\r\xE9V[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x0Eca\x0E^a\x0EY\x84a\rjV[a\x0E>V[a\rjV[\x90P\x91\x90PV[`\0a\x0Eu\x82a\x0EHV[\x90P\x91\x90PV[`\0a\x0E\x87\x82a\x0EjV[\x90P\x91\x90PV[a\x0E\x97\x81a\x0E|V[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xB2`\0\x83\x01\x84a\x0E\x8EV[\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0E\xCD\x81a\x0E\xB8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0E\xE8`\0\x83\x01\x84a\x0E\xC4V[\x92\x91PPV[`\0a\x0E\xF9\x82a\x0EjV[\x90P\x91\x90PV[a\x0F\t\x81a\x0E\xEEV[\x82RPPV[`\0` \x82\x01\x90Pa\x0F$`\0\x83\x01\x84a\x0F\0V[\x92\x91PPV[a\x0F3\x81a\r\xC8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0FN`\0\x83\x01\x84a\x0F*V[\x92\x91PPV[a\x0F]\x81a\r\x8AV[\x82RPPV[`\0` \x82\x01\x90Pa\x0Fx`\0\x83\x01\x84a\x0FTV[\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x0F\x94\x81a\x0F~V[\x82RPPV[`\0` \x82\x01\x90Pa\x0F\xAF`\0\x83\x01\x84a\x0F\x8BV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\xCBWa\x0F\xCAa\reV[[`\0a\x0F\xD9\x84\x82\x85\x01a\r\xB3V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x10\x1C\x82a\r\xC8V[\x91Pa\x10'\x83a\r\xC8V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x10?Wa\x10>a\x0F\xE2V[[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x10X\x81a\x10EV[\x82RPPV[`\0``\x82\x01\x90Pa\x10s`\0\x83\x01\x86a\x0F\x8BV[a\x10\x80` \x83\x01\x85a\x0FTV[a\x10\x8D`@\x83\x01\x84a\x10OV[\x94\x93PPPPV[`\0a\x10\xA0\x82a\r\xC8V[\x91Pa\x10\xAB\x83a\r\xC8V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x10\xC3Wa\x10\xC2a\x0F\xE2V[[\x92\x91PPV[`\0a\x10\xD4\x82a\x10EV[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x11\x06Wa\x11\x05a\x0F\xE2V[[\x81`\0\x03\x90P\x91\x90PV[`\0``\x82\x01\x90Pa\x11&`\0\x83\x01\x86a\x0FTV[a\x113` \x83\x01\x85a\x0FTV[a\x11@`@\x83\x01\x84a\x0F*V[\x94\x93PPPPV[`\0`@\x82\x01\x90Pa\x11]`\0\x83\x01\x85a\x0FTV[a\x11j` \x83\x01\x84a\x0F*V[\x93\x92PPPV[a\x11z\x81a\x0E\xB8V[\x81\x14a\x11\x85W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x11\x97\x81a\x11qV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xB3Wa\x11\xB2a\reV[[`\0a\x11\xC1\x84\x82\x85\x01a\x11\x88V[\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11\xFEW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x11\xE3V[`\0\x84\x84\x01RPPPPV[`\0a\x12\x15\x82a\x11\xCAV[a\x12\x1F\x81\x85a\x11\xD5V[\x93Pa\x12/\x81\x85` \x86\x01a\x11\xE0V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x12G\x82\x84a\x12\nV[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 /\x14(\xFD&P\xB8p\xDBX\xABD\x06l\x16\xD60K\x16\x0F5zm\xF1\x17h\x1E\xACu\xB7\xADpdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static GEMJOIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GemJoin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GemJoin<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GemJoin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GemJoin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GemJoin<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GemJoin)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GemJoin<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GEMJOIN_ABI.clone(),
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
                GEMJOIN_ABI.clone(),
                GEMJOIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `GEM` (0x4dc65411) function
        pub fn gem(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([77, 198, 84, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ILK_INDEX` (0xed0cee97) function
        pub fn ilk_index(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([237, 12, 238, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL` (0x7535d246) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 53, 210, 70], ())
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
        ///Calls the contract's `exit` (0xef693bed) function
        pub fn exit(
            &self,
            user: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 105, 59, 237], (user, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `join` (0x3b4da69f) function
        pub fn join(
            &self,
            user: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 77, 166, 159], (user, amount))
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
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
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
        ///Calls the contract's `totalGem` (0x83e8d3b8) function
        pub fn total_gem(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 232, 211, 184], ())
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
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
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
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GemJoinEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GemJoin<M> {
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
    ///Custom Error type `EnforcedPause` with signature `EnforcedPause()` and selector `0xd93c0665`
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
    #[etherror(name = "EnforcedPause", abi = "EnforcedPause()")]
    pub struct EnforcedPause;
    ///Custom Error type `ExpectedPause` with signature `ExpectedPause()` and selector `0x8dfc202b`
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
    #[etherror(name = "ExpectedPause", abi = "ExpectedPause()")]
    pub struct ExpectedPause;
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
    ///Custom Error type `Int256Overflow` with signature `Int256Overflow()` and selector `0x17aa1725`
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
    #[etherror(name = "Int256Overflow", abi = "Int256Overflow()")]
    pub struct Int256Overflow;
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
    ///Custom Error type `WrongIlkAddress` with signature `WrongIlkAddress(uint8,address)` and selector `0x5badbfd8`
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
    #[etherror(name = "WrongIlkAddress", abi = "WrongIlkAddress(uint8,address)")]
    pub struct WrongIlkAddress {
        pub ilk_index: u8,
        pub gem: ::ethers::core::types::Address,
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
    pub enum GemJoinErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        EnforcedPause(EnforcedPause),
        ExpectedPause(ExpectedPause),
        FailedInnerCall(FailedInnerCall),
        Int256Overflow(Int256Overflow),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        WrongIlkAddress(WrongIlkAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GemJoinErrors {
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
            if let Ok(decoded) = <EnforcedPause as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnforcedPause(decoded));
            }
            if let Ok(decoded) = <ExpectedPause as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectedPause(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <Int256Overflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Int256Overflow(decoded));
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
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <WrongIlkAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WrongIlkAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GemJoinErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnforcedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Int256Overflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrongIlkAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for GemJoinErrors {
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
                    == <EnforcedPause as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExpectedPause as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Int256Overflow as ::ethers::contract::EthError>::selector() => {
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
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WrongIlkAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GemJoinErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnforcedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Int256Overflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WrongIlkAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for GemJoinErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for GemJoinErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for GemJoinErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<EnforcedPause> for GemJoinErrors {
        fn from(value: EnforcedPause) -> Self {
            Self::EnforcedPause(value)
        }
    }
    impl ::core::convert::From<ExpectedPause> for GemJoinErrors {
        fn from(value: ExpectedPause) -> Self {
            Self::ExpectedPause(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for GemJoinErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<Int256Overflow> for GemJoinErrors {
        fn from(value: Int256Overflow) -> Self {
            Self::Int256Overflow(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for GemJoinErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for GemJoinErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for GemJoinErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<WrongIlkAddress> for GemJoinErrors {
        fn from(value: WrongIlkAddress) -> Self {
            Self::WrongIlkAddress(value)
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
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
    pub enum GemJoinEvents {
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GemJoinEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(GemJoinEvents::OwnershipTransferStartedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(GemJoinEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GemJoinEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(GemJoinEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GemJoinEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter> for GemJoinEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for GemJoinEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for GemJoinEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for GemJoinEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `GEM` function with signature `GEM()` and selector `0x4dc65411`
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
    #[ethcall(name = "GEM", abi = "GEM()")]
    pub struct GemCall;
    ///Container type for all input parameters for the `ILK_INDEX` function with signature `ILK_INDEX()` and selector `0xed0cee97`
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
    #[ethcall(name = "ILK_INDEX", abi = "ILK_INDEX()")]
    pub struct IlkIndexCall;
    ///Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
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
    ///Container type for all input parameters for the `exit` function with signature `exit(address,uint256)` and selector `0xef693bed`
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
    #[ethcall(name = "exit", abi = "exit(address,uint256)")]
    pub struct ExitCall {
        pub user: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `join` function with signature `join(address,uint256)` and selector `0x3b4da69f`
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
    #[ethcall(name = "join", abi = "join(address,uint256)")]
    pub struct JoinCall {
        pub user: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
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
    ///Container type for all input parameters for the `totalGem` function with signature `totalGem()` and selector `0x83e8d3b8`
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
    #[ethcall(name = "totalGem", abi = "totalGem()")]
    pub struct TotalGemCall;
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
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
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
    pub enum GemJoinCalls {
        Gem(GemCall),
        IlkIndex(IlkIndexCall),
        Pool(PoolCall),
        AcceptOwnership(AcceptOwnershipCall),
        Exit(ExitCall),
        Join(JoinCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Paused(PausedCall),
        PendingOwner(PendingOwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TotalGem(TotalGemCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
    }
    impl ::ethers::core::abi::AbiDecode for GemJoinCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Gem(decoded));
            }
            if let Ok(decoded) = <IlkIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IlkIndex(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <ExitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Exit(decoded));
            }
            if let Ok(decoded) = <JoinCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Join(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
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
            if let Ok(decoded) = <TotalGemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalGem(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GemJoinCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Gem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IlkIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Exit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Join(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalGem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GemJoinCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Gem(element) => ::core::fmt::Display::fmt(element, f),
                Self::IlkIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Join(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalGem(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GemCall> for GemJoinCalls {
        fn from(value: GemCall) -> Self {
            Self::Gem(value)
        }
    }
    impl ::core::convert::From<IlkIndexCall> for GemJoinCalls {
        fn from(value: IlkIndexCall) -> Self {
            Self::IlkIndex(value)
        }
    }
    impl ::core::convert::From<PoolCall> for GemJoinCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for GemJoinCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<ExitCall> for GemJoinCalls {
        fn from(value: ExitCall) -> Self {
            Self::Exit(value)
        }
    }
    impl ::core::convert::From<JoinCall> for GemJoinCalls {
        fn from(value: JoinCall) -> Self {
            Self::Join(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for GemJoinCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for GemJoinCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for GemJoinCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for GemJoinCalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for GemJoinCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TotalGemCall> for GemJoinCalls {
        fn from(value: TotalGemCall) -> Self {
            Self::TotalGem(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for GemJoinCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for GemJoinCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    ///Container type for all return fields from the `GEM` function with signature `GEM()` and selector `0x4dc65411`
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
    pub struct GemReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ILK_INDEX` function with signature `ILK_INDEX()` and selector `0xed0cee97`
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
    pub struct IlkIndexReturn(pub u8);
    ///Container type for all return fields from the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    pub struct PoolReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub bool);
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
    ///Container type for all return fields from the `totalGem` function with signature `totalGem()` and selector `0x83e8d3b8`
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
    pub struct TotalGemReturn(pub ::ethers::core::types::U256);
}
