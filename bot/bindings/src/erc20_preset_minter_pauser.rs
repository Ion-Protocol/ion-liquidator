pub use erc20_preset_minter_pauser::*;
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
pub mod erc20_preset_minter_pauser {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("PAUSER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PAUSER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burnFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMember"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMember"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "callerConfirmation",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("AccessControlBadConfirmation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlBadConfirmation",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("neededRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientAllowance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("needed"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("needed"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidApprover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approver"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidReceiver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidSpender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERC20PRESETMINTERPAUSER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0*\xAF8\x03\x80b\0*\xAF\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x05YV[\x81\x81\x81`\x05\x90\x81b\0\0J\x91\x90b\0\x08)V[P\x80`\x06\x90\x81b\0\0\\\x91\x90b\0\x08)V[PPP`\0`\x07`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPb\0\0\x9E`\0\x80\x1Bb\0\0\x92b\0\x01+` \x1B` \x1CV[b\0\x013` \x1B` \x1CV[Pb\0\0\xE0\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6b\0\0\xD4b\0\x01+` \x1B` \x1CV[b\0\x013` \x1B` \x1CV[Pb\0\x01\"\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*b\0\x01\x16b\0\x01+` \x1B` \x1CV[b\0\x013` \x1B` \x1CV[PPPb\0\t\x10V[`\x003\x90P\x90V[`\0\x80b\0\x01H\x84\x84b\0\x01\x84` \x1B` \x1CV[\x90P\x80\x15b\0\x01zWb\0\x01x\x83`\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 b\0\x02\x87` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[`\0b\0\x01\x98\x83\x83b\0\x02\xBF` \x1B` \x1CV[b\0\x02|W`\x01`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPb\0\x02\x18b\0\x01+` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pb\0\x02\x81V[`\0\x90P[\x92\x91PPV[`\0b\0\x02\xB7\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Bb\0\x03)` \x1B` \x1CV[\x90P\x92\x91PPV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\0b\0\x03=\x83\x83b\0\x03\xA3` \x1B` \x1CV[b\0\x03\x98W\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pb\0\x03\x9DV[`\0\x90P[\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x04/\x82b\0\x03\xE4V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x04QWb\0\x04Pb\0\x03\xF5V[[\x80`@RPPPV[`\0b\0\x04fb\0\x03\xC6V[\x90Pb\0\x04t\x82\x82b\0\x04$V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x04\x97Wb\0\x04\x96b\0\x03\xF5V[[b\0\x04\xA2\x82b\0\x03\xE4V[\x90P` \x81\x01\x90P\x91\x90PV[`\0[\x83\x81\x10\x15b\0\x04\xCFW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pb\0\x04\xB2V[`\0\x84\x84\x01RPPPPV[`\0b\0\x04\xF2b\0\x04\xEC\x84b\0\x04yV[b\0\x04ZV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15b\0\x05\x11Wb\0\x05\x10b\0\x03\xDFV[[b\0\x05\x1E\x84\x82\x85b\0\x04\xAFV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x05>Wb\0\x05=b\0\x03\xDAV[[\x81Qb\0\x05P\x84\x82` \x86\x01b\0\x04\xDBV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x05sWb\0\x05rb\0\x03\xD0V[[`\0\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x05\x94Wb\0\x05\x93b\0\x03\xD5V[[b\0\x05\xA2\x85\x82\x86\x01b\0\x05&V[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x05\xC6Wb\0\x05\xC5b\0\x03\xD5V[[b\0\x05\xD4\x85\x82\x86\x01b\0\x05&V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80b\0\x061W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x06GWb\0\x06Fb\0\x05\xE9V[[P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02b\0\x06\xB1\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82b\0\x06rV[b\0\x06\xBD\x86\x83b\0\x06rV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0b\0\x07\nb\0\x07\x04b\0\x06\xFE\x84b\0\x06\xD5V[b\0\x06\xDFV[b\0\x06\xD5V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[b\0\x07&\x83b\0\x06\xE9V[b\0\x07>b\0\x075\x82b\0\x07\x11V[\x84\x84Tb\0\x06\x7FV[\x82UPPPPV[`\0\x90V[b\0\x07Ub\0\x07FV[b\0\x07b\x81\x84\x84b\0\x07\x1BV[PPPV[[\x81\x81\x10\x15b\0\x07\x8AWb\0\x07~`\0\x82b\0\x07KV[`\x01\x81\x01\x90Pb\0\x07hV[PPV[`\x1F\x82\x11\x15b\0\x07\xD9Wb\0\x07\xA3\x81b\0\x06MV[b\0\x07\xAE\x84b\0\x06bV[\x81\x01` \x85\x10\x15b\0\x07\xBEW\x81\x90P[b\0\x07\xD6b\0\x07\xCD\x85b\0\x06bV[\x83\x01\x82b\0\x07gV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0b\0\x07\xFE`\0\x19\x84`\x08\x02b\0\x07\xDEV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0b\0\x08\x19\x83\x83b\0\x07\xEBV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[b\0\x084\x82b\0\x05\xDEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x08PWb\0\x08Ob\0\x03\xF5V[[b\0\x08\\\x82Tb\0\x06\x18V[b\0\x08i\x82\x82\x85b\0\x07\x8EV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14b\0\x08\xA1W`\0\x84\x15b\0\x08\x8CW\x82\x87\x01Q\x90P[b\0\x08\x98\x85\x82b\0\x08\x0BV[\x86UPb\0\t\x08V[`\x1F\x19\x84\x16b\0\x08\xB1\x86b\0\x06MV[`\0[\x82\x81\x10\x15b\0\x08\xDBW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pb\0\x08\xB4V[\x86\x83\x10\x15b\0\x08\xFBW\x84\x89\x01Qb\0\x08\xF7`\x1F\x89\x16\x82b\0\x07\xEBV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[a!\x8F\x80b\0\t `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\xDEW\x80c\xA2\x17\xFD\xDF\x11a\0\x97W\x80c\xD59\x13\x93\x11a\0qW\x80c\xD59\x13\x93\x14a\x04\x97W\x80c\xD5Gt\x1F\x14a\x04\xB5W\x80c\xDDb\xED>\x14a\x04\xD1W\x80c\xE6:\xB1\xE9\x14a\x05\x01Wa\x01\x8EV[\x80c\xA2\x17\xFD\xDF\x14a\x04\x19W\x80c\xA9\x05\x9C\xBB\x14a\x047W\x80c\xCA\x15\xC8s\x14a\x04gWa\x01\x8EV[\x80cp\xA0\x821\x14a\x03EW\x80cy\xCCg\x90\x14a\x03uW\x80c\x84V\xCBY\x14a\x03\x91W\x80c\x90\x10\xD0|\x14a\x03\x9BW\x80c\x91\xD1HT\x14a\x03\xCBW\x80c\x95\xD8\x9BA\x14a\x03\xFBWa\x01\x8EV[\x80c//\xF1]\x11a\x01KW\x80c?K\xA8:\x11a\x01%W\x80c?K\xA8:\x14a\x02\xE5W\x80c@\xC1\x0F\x19\x14a\x02\xEFW\x80cB\x96lh\x14a\x03\x0BW\x80c\\\x97Z\xBB\x14a\x03'Wa\x01\x8EV[\x80c//\xF1]\x14a\x02\x8FW\x80c1<\xE5g\x14a\x02\xABW\x80c6V\x8A\xBE\x14a\x02\xC9Wa\x01\x8EV[\x80c\x01\xFF\xC9\xA7\x14a\x01\x93W\x80c\x06\xFD\xDE\x03\x14a\x01\xC3W\x80c\t^\xA7\xB3\x14a\x01\xE1W\x80c\x18\x16\r\xDD\x14a\x02\x11W\x80c#\xB8r\xDD\x14a\x02/W\x80c$\x8A\x9C\xA3\x14a\x02_W[`\0\x80\xFD[a\x01\xAD`\x04\x806\x03\x81\x01\x90a\x01\xA8\x91\x90a\x1A\x12V[a\x05\x1FV[`@Qa\x01\xBA\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x01\xCBa\x05\x99V[`@Qa\x01\xD8\x91\x90a\x1B\x05V[`@Q\x80\x91\x03\x90\xF3[a\x01\xFB`\x04\x806\x03\x81\x01\x90a\x01\xF6\x91\x90a\x1B\xBBV[a\x06+V[`@Qa\x02\x08\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x02\x19a\x06NV[`@Qa\x02&\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x02I`\x04\x806\x03\x81\x01\x90a\x02D\x91\x90a\x1C%V[a\x06XV[`@Qa\x02V\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x02y`\x04\x806\x03\x81\x01\x90a\x02t\x91\x90a\x1C\xAEV[a\x06\x87V[`@Qa\x02\x86\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x02\xA9`\x04\x806\x03\x81\x01\x90a\x02\xA4\x91\x90a\x1D\x05V[a\x06\xA6V[\0[a\x02\xB3a\x06\xC8V[`@Qa\x02\xC0\x91\x90a\x1DaV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a\x1D\x05V[a\x06\xD1V[\0[a\x02\xEDa\x07LV[\0[a\x03\t`\x04\x806\x03\x81\x01\x90a\x03\x04\x91\x90a\x1B\xBBV[a\x07\xC6V[\0[a\x03%`\x04\x806\x03\x81\x01\x90a\x03 \x91\x90a\x1D|V[a\x07\xFFV[\0[a\x03/a\x08\x13V[`@Qa\x03<\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x03_`\x04\x806\x03\x81\x01\x90a\x03Z\x91\x90a\x1D\xA9V[a\x08*V[`@Qa\x03l\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x03\x8F`\x04\x806\x03\x81\x01\x90a\x03\x8A\x91\x90a\x1B\xBBV[a\x08sV[\0[a\x03\x99a\x08\x93V[\0[a\x03\xB5`\x04\x806\x03\x81\x01\x90a\x03\xB0\x91\x90a\x1D\xD6V[a\t\rV[`@Qa\x03\xC2\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xF3[a\x03\xE5`\x04\x806\x03\x81\x01\x90a\x03\xE0\x91\x90a\x1D\x05V[a\t<V[`@Qa\x03\xF2\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x04\x03a\t\xA6V[`@Qa\x04\x10\x91\x90a\x1B\x05V[`@Q\x80\x91\x03\x90\xF3[a\x04!a\n8V[`@Qa\x04.\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x04Q`\x04\x806\x03\x81\x01\x90a\x04L\x91\x90a\x1B\xBBV[a\n?V[`@Qa\x04^\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x04\x81`\x04\x806\x03\x81\x01\x90a\x04|\x91\x90a\x1C\xAEV[a\nbV[`@Qa\x04\x8E\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x04\x9Fa\n\x86V[`@Qa\x04\xAC\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x04\xCF`\x04\x806\x03\x81\x01\x90a\x04\xCA\x91\x90a\x1D\x05V[a\n\xAAV[\0[a\x04\xEB`\x04\x806\x03\x81\x01\x90a\x04\xE6\x91\x90a\x1E@V[a\n\xCCV[`@Qa\x04\xF8\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x05\ta\x0BSV[`@Qa\x05\x16\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[`\0\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x05\x92WPa\x05\x91\x82a\x0BwV[[\x90P\x91\x90PV[```\x05\x80Ta\x05\xA8\x90a\x1E\xAFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD4\x90a\x1E\xAFV[\x80\x15a\x06!W\x80`\x1F\x10a\x05\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x04W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\x066a\x0B\xF1V[\x90Pa\x06C\x81\x85\x85a\x0B\xF9V[`\x01\x91PP\x92\x91PPV[`\0`\x04T\x90P\x90V[`\0\x80a\x06ca\x0B\xF1V[\x90Pa\x06p\x85\x82\x85a\x0C\x0BV[a\x06{\x85\x85\x85a\x0C\x9FV[`\x01\x91PP\x93\x92PPPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x06\xAF\x82a\x06\x87V[a\x06\xB8\x81a\r\x93V[a\x06\xC2\x83\x83a\r\xA7V[PPPPV[`\0`\x12\x90P\x90V[a\x06\xD9a\x0B\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07=W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07G\x82\x82a\r\xEDV[PPPV[a\x07}\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x07xa\x0B\xF1V[a\t<V[a\x07\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xB3\x90a\x1FRV[`@Q\x80\x91\x03\x90\xFD[a\x07\xC4a\x0E3V[V[\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6a\x07\xF0\x81a\r\x93V[a\x07\xFA\x83\x83a\x0E\x96V[PPPV[a\x08\x10a\x08\na\x0B\xF1V[\x82a\x0F\x18V[PV[`\0`\x07`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[`\0`\x02`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x91\x90PV[a\x08\x85\x82a\x08\x7Fa\x0B\xF1V[\x83a\x0C\x0BV[a\x08\x8F\x82\x82a\x0F\x18V[PPV[a\x08\xC4\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x08\xBFa\x0B\xF1V[a\t<V[a\t\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xFA\x90a\x1F\xE4V[`@Q\x80\x91\x03\x90\xFD[a\t\x0Ba\x0F\x9AV[V[`\0a\t4\x82`\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 a\x0F\xFD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[```\x06\x80Ta\t\xB5\x90a\x1E\xAFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xE1\x90a\x1E\xAFV[\x80\x15a\n.W\x80`\x1F\x10a\n\x03Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n.V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x11W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80\x1B\x81V[`\0\x80a\nJa\x0B\xF1V[\x90Pa\nW\x81\x85\x85a\x0C\x9FV[`\x01\x91PP\x92\x91PPV[`\0a\n\x7F`\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 a\x10\x17V[\x90P\x91\x90PV[\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\x81V[a\n\xB3\x82a\x06\x87V[a\n\xBC\x81a\r\x93V[a\n\xC6\x83\x83a\r\xEDV[PPPPV[`\0`\x03`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x92\x91PPV[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81V[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x0B\xEAWPa\x0B\xE9\x82a\x10,V[[\x90P\x91\x90PV[`\x003\x90P\x90V[a\x0C\x06\x83\x83\x83`\x01a\x10\x96V[PPPV[`\0a\x0C\x17\x84\x84a\n\xCCV[\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x0C\x99W\x81\x81\x10\x15a\x0C\x89W\x82\x81\x83`@Q\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x80\x93\x92\x91\x90a \x04V[`@Q\x80\x91\x03\x90\xFD[a\x0C\x98\x84\x84\x84\x84\x03`\0a\x10\x96V[[PPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\r\x11W`\0`@Q\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x08\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\r\x83W`\0`@Q\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\rz\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[a\r\x8E\x83\x83\x83a\x12mV[PPPV[a\r\xA4\x81a\r\x9Fa\x0B\xF1V[a\x12}V[PV[`\0\x80a\r\xB4\x84\x84a\x12\xCEV[\x90P\x80\x15a\r\xE3Wa\r\xE1\x83`\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 a\x13\xBF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[`\0\x80a\r\xFA\x84\x84a\x13\xEFV[\x90P\x80\x15a\x0E)Wa\x0E'\x83`\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 a\x14\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[a\x0E;a\x15\x11V[`\0`\x07`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa\x0E\x7Fa\x0B\xF1V[`@Qa\x0E\x8C\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xA1V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0F\x08W`\0`@Q\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xFF\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[a\x0F\x14`\0\x83\x83a\x12mV[PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0F\x8AW`\0`@Q\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x81\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[a\x0F\x96\x82`\0\x83a\x12mV[PPV[a\x0F\xA2a\x15QV[`\x01`\x07`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x0F\xE6a\x0B\xF1V[`@Qa\x0F\xF3\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xA1V[`\0a\x10\x0C\x83`\0\x01\x83a\x15\x92V[`\0\x1C\x90P\x92\x91PPV[`\0a\x10%\x82`\0\x01a\x15\xBDV[\x90P\x91\x90PV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11\x08W`\0`@Q\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xFF\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11zW`\0`@Q\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11q\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[\x81`\x03`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80\x15a\x12gW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x12^\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xA3[PPPPV[a\x12x\x83\x83\x83a\x15\xCEV[PPPV[a\x12\x87\x82\x82a\t<V[a\x12\xCAW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xC1\x92\x91\x90a ;V[`@Q\x80\x91\x03\x90\xFD[PPV[`\0a\x12\xDA\x83\x83a\t<V[a\x13\xB4W`\x01`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x13Qa\x0B\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x13\xB9V[`\0\x90P[\x92\x91PPV[`\0a\x13\xE7\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x15\xE6V[\x90P\x92\x91PPV[`\0a\x13\xFB\x83\x83a\t<V[\x15a\x14\xD6W`\0\x80`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x14sa\x0B\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x14\xDBV[`\0\x90P[\x92\x91PPV[`\0a\x15\t\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x16VV[\x90P\x92\x91PPV[a\x15\x19a\x08\x13V[a\x15OW`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x15Ya\x08\x13V[\x15a\x15\x90W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x15\xAAWa\x15\xA9a dV[[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x81`\0\x01\x80T\x90P\x90P\x91\x90PV[a\x15\xD6a\x15QV[a\x15\xE1\x83\x83\x83a\x17jV[PPPV[`\0a\x15\xF2\x83\x83a\x19\x92V[a\x16KW\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pa\x16PV[`\0\x90P[\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x14a\x17^W`\0`\x01\x82a\x16\x88\x91\x90a \xC2V[\x90P`\0`\x01\x86`\0\x01\x80T\x90Pa\x16\xA0\x91\x90a \xC2V[\x90P\x80\x82\x14a\x17\x0FW`\0\x86`\0\x01\x82\x81T\x81\x10a\x16\xC1Wa\x16\xC0a dV[[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x16\xE5Wa\x16\xE4a dV[[\x90`\0R` `\0 \x01\x81\x90UP\x83\x87`\x01\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[\x85`\0\x01\x80T\x80a\x17#Wa\x17\"a \xF6V[[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x17dV[`\0\x91PP[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xBCW\x80`\x04`\0\x82\x82Ta\x17\xB0\x91\x90a!%V[\x92PP\x81\x90UPa\x18\x91V[`\0`\x02`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x81\x81\x10\x15a\x18IW\x83\x81\x83`@Q\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18@\x93\x92\x91\x90a \x04V[`@Q\x80\x91\x03\x90\xFD[\x81\x81\x03`\x02`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x18\xDAW\x80`\x04`\0\x82\x82T\x03\x92PP\x81\x90UPa\x19(V[\x80`\x02`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82T\x01\x92PP\x81\x90UP[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\x19\x85\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x19\xEF\x81a\x19\xBAV[\x81\x14a\x19\xFAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1A\x0C\x81a\x19\xE6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A(Wa\x1A'a\x19\xB5V[[`\0a\x1A6\x84\x82\x85\x01a\x19\xFDV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x1AT\x81a\x1A?V[\x82RPPV[`\0` \x82\x01\x90Pa\x1Ao`\0\x83\x01\x84a\x1AKV[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1A\xAFW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1A\x94V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x1A\xD7\x82a\x1AuV[a\x1A\xE1\x81\x85a\x1A\x80V[\x93Pa\x1A\xF1\x81\x85` \x86\x01a\x1A\x91V[a\x1A\xFA\x81a\x1A\xBBV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1B\x1F\x81\x84a\x1A\xCCV[\x90P\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1BR\x82a\x1B'V[\x90P\x91\x90PV[a\x1Bb\x81a\x1BGV[\x81\x14a\x1BmW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\x7F\x81a\x1BYV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x1B\x98\x81a\x1B\x85V[\x81\x14a\x1B\xA3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\xB5\x81a\x1B\x8FV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\xD2Wa\x1B\xD1a\x19\xB5V[[`\0a\x1B\xE0\x85\x82\x86\x01a\x1BpV[\x92PP` a\x1B\xF1\x85\x82\x86\x01a\x1B\xA6V[\x91PP\x92P\x92\x90PV[a\x1C\x04\x81a\x1B\x85V[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\x1F`\0\x83\x01\x84a\x1B\xFBV[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C>Wa\x1C=a\x19\xB5V[[`\0a\x1CL\x86\x82\x87\x01a\x1BpV[\x93PP` a\x1C]\x86\x82\x87\x01a\x1BpV[\x92PP`@a\x1Cn\x86\x82\x87\x01a\x1B\xA6V[\x91PP\x92P\x92P\x92V[`\0\x81\x90P\x91\x90PV[a\x1C\x8B\x81a\x1CxV[\x81\x14a\x1C\x96W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1C\xA8\x81a\x1C\x82V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\xC4Wa\x1C\xC3a\x19\xB5V[[`\0a\x1C\xD2\x84\x82\x85\x01a\x1C\x99V[\x91PP\x92\x91PPV[a\x1C\xE4\x81a\x1CxV[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\xFF`\0\x83\x01\x84a\x1C\xDBV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\x1CWa\x1D\x1Ba\x19\xB5V[[`\0a\x1D*\x85\x82\x86\x01a\x1C\x99V[\x92PP` a\x1D;\x85\x82\x86\x01a\x1BpV[\x91PP\x92P\x92\x90PV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x1D[\x81a\x1DEV[\x82RPPV[`\0` \x82\x01\x90Pa\x1Dv`\0\x83\x01\x84a\x1DRV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D\x92Wa\x1D\x91a\x19\xB5V[[`\0a\x1D\xA0\x84\x82\x85\x01a\x1B\xA6V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D\xBFWa\x1D\xBEa\x19\xB5V[[`\0a\x1D\xCD\x84\x82\x85\x01a\x1BpV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xEDWa\x1D\xECa\x19\xB5V[[`\0a\x1D\xFB\x85\x82\x86\x01a\x1C\x99V[\x92PP` a\x1E\x0C\x85\x82\x86\x01a\x1B\xA6V[\x91PP\x92P\x92\x90PV[a\x1E\x1F\x81a\x1BGV[\x82RPPV[`\0` \x82\x01\x90Pa\x1E:`\0\x83\x01\x84a\x1E\x16V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1EWWa\x1EVa\x19\xB5V[[`\0a\x1Ee\x85\x82\x86\x01a\x1BpV[\x92PP` a\x1Ev\x85\x82\x86\x01a\x1BpV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x1E\xC7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\xDAWa\x1E\xD9a\x1E\x80V[[P\x91\x90PV[\x7FERC20PresetMinterPauser: must ha`\0\x82\x01R\x7Fve pauser role to unpause\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1F<`9\x83a\x1A\x80V[\x91Pa\x1FG\x82a\x1E\xE0V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1Fk\x81a\x1F/V[\x90P\x91\x90PV[\x7FERC20PresetMinterPauser: must ha`\0\x82\x01R\x7Fve pauser role to pause\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1F\xCE`7\x83a\x1A\x80V[\x91Pa\x1F\xD9\x82a\x1FrV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1F\xFD\x81a\x1F\xC1V[\x90P\x91\x90PV[`\0``\x82\x01\x90Pa \x19`\0\x83\x01\x86a\x1E\x16V[a &` \x83\x01\x85a\x1B\xFBV[a 3`@\x83\x01\x84a\x1B\xFBV[\x94\x93PPPPV[`\0`@\x82\x01\x90Pa P`\0\x83\x01\x85a\x1E\x16V[a ]` \x83\x01\x84a\x1C\xDBV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a \xCD\x82a\x1B\x85V[\x91Pa \xD8\x83a\x1B\x85V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a \xF0Wa \xEFa \x93V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD[`\0a!0\x82a\x1B\x85V[\x91Pa!;\x83a\x1B\x85V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a!SWa!Ra \x93V[[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x8Dd\xB9\xCA\xE9\xC1{\xF2+\xE4\xC0\x1EO\xE6L\xA1q\xA4@\xD9/}D\x9D\xCC\xED\xB5\xEBT\xB2\xCDFdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static ERC20PRESETMINTERPAUSER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\xDEW\x80c\xA2\x17\xFD\xDF\x11a\0\x97W\x80c\xD59\x13\x93\x11a\0qW\x80c\xD59\x13\x93\x14a\x04\x97W\x80c\xD5Gt\x1F\x14a\x04\xB5W\x80c\xDDb\xED>\x14a\x04\xD1W\x80c\xE6:\xB1\xE9\x14a\x05\x01Wa\x01\x8EV[\x80c\xA2\x17\xFD\xDF\x14a\x04\x19W\x80c\xA9\x05\x9C\xBB\x14a\x047W\x80c\xCA\x15\xC8s\x14a\x04gWa\x01\x8EV[\x80cp\xA0\x821\x14a\x03EW\x80cy\xCCg\x90\x14a\x03uW\x80c\x84V\xCBY\x14a\x03\x91W\x80c\x90\x10\xD0|\x14a\x03\x9BW\x80c\x91\xD1HT\x14a\x03\xCBW\x80c\x95\xD8\x9BA\x14a\x03\xFBWa\x01\x8EV[\x80c//\xF1]\x11a\x01KW\x80c?K\xA8:\x11a\x01%W\x80c?K\xA8:\x14a\x02\xE5W\x80c@\xC1\x0F\x19\x14a\x02\xEFW\x80cB\x96lh\x14a\x03\x0BW\x80c\\\x97Z\xBB\x14a\x03'Wa\x01\x8EV[\x80c//\xF1]\x14a\x02\x8FW\x80c1<\xE5g\x14a\x02\xABW\x80c6V\x8A\xBE\x14a\x02\xC9Wa\x01\x8EV[\x80c\x01\xFF\xC9\xA7\x14a\x01\x93W\x80c\x06\xFD\xDE\x03\x14a\x01\xC3W\x80c\t^\xA7\xB3\x14a\x01\xE1W\x80c\x18\x16\r\xDD\x14a\x02\x11W\x80c#\xB8r\xDD\x14a\x02/W\x80c$\x8A\x9C\xA3\x14a\x02_W[`\0\x80\xFD[a\x01\xAD`\x04\x806\x03\x81\x01\x90a\x01\xA8\x91\x90a\x1A\x12V[a\x05\x1FV[`@Qa\x01\xBA\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x01\xCBa\x05\x99V[`@Qa\x01\xD8\x91\x90a\x1B\x05V[`@Q\x80\x91\x03\x90\xF3[a\x01\xFB`\x04\x806\x03\x81\x01\x90a\x01\xF6\x91\x90a\x1B\xBBV[a\x06+V[`@Qa\x02\x08\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x02\x19a\x06NV[`@Qa\x02&\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x02I`\x04\x806\x03\x81\x01\x90a\x02D\x91\x90a\x1C%V[a\x06XV[`@Qa\x02V\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x02y`\x04\x806\x03\x81\x01\x90a\x02t\x91\x90a\x1C\xAEV[a\x06\x87V[`@Qa\x02\x86\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x02\xA9`\x04\x806\x03\x81\x01\x90a\x02\xA4\x91\x90a\x1D\x05V[a\x06\xA6V[\0[a\x02\xB3a\x06\xC8V[`@Qa\x02\xC0\x91\x90a\x1DaV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a\x1D\x05V[a\x06\xD1V[\0[a\x02\xEDa\x07LV[\0[a\x03\t`\x04\x806\x03\x81\x01\x90a\x03\x04\x91\x90a\x1B\xBBV[a\x07\xC6V[\0[a\x03%`\x04\x806\x03\x81\x01\x90a\x03 \x91\x90a\x1D|V[a\x07\xFFV[\0[a\x03/a\x08\x13V[`@Qa\x03<\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x03_`\x04\x806\x03\x81\x01\x90a\x03Z\x91\x90a\x1D\xA9V[a\x08*V[`@Qa\x03l\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x03\x8F`\x04\x806\x03\x81\x01\x90a\x03\x8A\x91\x90a\x1B\xBBV[a\x08sV[\0[a\x03\x99a\x08\x93V[\0[a\x03\xB5`\x04\x806\x03\x81\x01\x90a\x03\xB0\x91\x90a\x1D\xD6V[a\t\rV[`@Qa\x03\xC2\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xF3[a\x03\xE5`\x04\x806\x03\x81\x01\x90a\x03\xE0\x91\x90a\x1D\x05V[a\t<V[`@Qa\x03\xF2\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x04\x03a\t\xA6V[`@Qa\x04\x10\x91\x90a\x1B\x05V[`@Q\x80\x91\x03\x90\xF3[a\x04!a\n8V[`@Qa\x04.\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x04Q`\x04\x806\x03\x81\x01\x90a\x04L\x91\x90a\x1B\xBBV[a\n?V[`@Qa\x04^\x91\x90a\x1AZV[`@Q\x80\x91\x03\x90\xF3[a\x04\x81`\x04\x806\x03\x81\x01\x90a\x04|\x91\x90a\x1C\xAEV[a\nbV[`@Qa\x04\x8E\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x04\x9Fa\n\x86V[`@Qa\x04\xAC\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x04\xCF`\x04\x806\x03\x81\x01\x90a\x04\xCA\x91\x90a\x1D\x05V[a\n\xAAV[\0[a\x04\xEB`\x04\x806\x03\x81\x01\x90a\x04\xE6\x91\x90a\x1E@V[a\n\xCCV[`@Qa\x04\xF8\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xF3[a\x05\ta\x0BSV[`@Qa\x05\x16\x91\x90a\x1C\xEAV[`@Q\x80\x91\x03\x90\xF3[`\0\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x05\x92WPa\x05\x91\x82a\x0BwV[[\x90P\x91\x90PV[```\x05\x80Ta\x05\xA8\x90a\x1E\xAFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD4\x90a\x1E\xAFV[\x80\x15a\x06!W\x80`\x1F\x10a\x05\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x04W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\x066a\x0B\xF1V[\x90Pa\x06C\x81\x85\x85a\x0B\xF9V[`\x01\x91PP\x92\x91PPV[`\0`\x04T\x90P\x90V[`\0\x80a\x06ca\x0B\xF1V[\x90Pa\x06p\x85\x82\x85a\x0C\x0BV[a\x06{\x85\x85\x85a\x0C\x9FV[`\x01\x91PP\x93\x92PPPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x06\xAF\x82a\x06\x87V[a\x06\xB8\x81a\r\x93V[a\x06\xC2\x83\x83a\r\xA7V[PPPPV[`\0`\x12\x90P\x90V[a\x06\xD9a\x0B\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07=W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07G\x82\x82a\r\xEDV[PPPV[a\x07}\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x07xa\x0B\xF1V[a\t<V[a\x07\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xB3\x90a\x1FRV[`@Q\x80\x91\x03\x90\xFD[a\x07\xC4a\x0E3V[V[\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6a\x07\xF0\x81a\r\x93V[a\x07\xFA\x83\x83a\x0E\x96V[PPPV[a\x08\x10a\x08\na\x0B\xF1V[\x82a\x0F\x18V[PV[`\0`\x07`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[`\0`\x02`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x91\x90PV[a\x08\x85\x82a\x08\x7Fa\x0B\xF1V[\x83a\x0C\x0BV[a\x08\x8F\x82\x82a\x0F\x18V[PPV[a\x08\xC4\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x08\xBFa\x0B\xF1V[a\t<V[a\t\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xFA\x90a\x1F\xE4V[`@Q\x80\x91\x03\x90\xFD[a\t\x0Ba\x0F\x9AV[V[`\0a\t4\x82`\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 a\x0F\xFD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[```\x06\x80Ta\t\xB5\x90a\x1E\xAFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xE1\x90a\x1E\xAFV[\x80\x15a\n.W\x80`\x1F\x10a\n\x03Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n.V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x11W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80\x1B\x81V[`\0\x80a\nJa\x0B\xF1V[\x90Pa\nW\x81\x85\x85a\x0C\x9FV[`\x01\x91PP\x92\x91PPV[`\0a\n\x7F`\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 a\x10\x17V[\x90P\x91\x90PV[\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\x81V[a\n\xB3\x82a\x06\x87V[a\n\xBC\x81a\r\x93V[a\n\xC6\x83\x83a\r\xEDV[PPPPV[`\0`\x03`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x92\x91PPV[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81V[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x0B\xEAWPa\x0B\xE9\x82a\x10,V[[\x90P\x91\x90PV[`\x003\x90P\x90V[a\x0C\x06\x83\x83\x83`\x01a\x10\x96V[PPPV[`\0a\x0C\x17\x84\x84a\n\xCCV[\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x0C\x99W\x81\x81\x10\x15a\x0C\x89W\x82\x81\x83`@Q\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x80\x93\x92\x91\x90a \x04V[`@Q\x80\x91\x03\x90\xFD[a\x0C\x98\x84\x84\x84\x84\x03`\0a\x10\x96V[[PPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\r\x11W`\0`@Q\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x08\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\r\x83W`\0`@Q\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\rz\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[a\r\x8E\x83\x83\x83a\x12mV[PPPV[a\r\xA4\x81a\r\x9Fa\x0B\xF1V[a\x12}V[PV[`\0\x80a\r\xB4\x84\x84a\x12\xCEV[\x90P\x80\x15a\r\xE3Wa\r\xE1\x83`\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 a\x13\xBF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[`\0\x80a\r\xFA\x84\x84a\x13\xEFV[\x90P\x80\x15a\x0E)Wa\x0E'\x83`\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 a\x14\xE1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[a\x0E;a\x15\x11V[`\0`\x07`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa\x0E\x7Fa\x0B\xF1V[`@Qa\x0E\x8C\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xA1V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0F\x08W`\0`@Q\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xFF\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[a\x0F\x14`\0\x83\x83a\x12mV[PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0F\x8AW`\0`@Q\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x81\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[a\x0F\x96\x82`\0\x83a\x12mV[PPV[a\x0F\xA2a\x15QV[`\x01`\x07`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x0F\xE6a\x0B\xF1V[`@Qa\x0F\xF3\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xA1V[`\0a\x10\x0C\x83`\0\x01\x83a\x15\x92V[`\0\x1C\x90P\x92\x91PPV[`\0a\x10%\x82`\0\x01a\x15\xBDV[\x90P\x91\x90PV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11\x08W`\0`@Q\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xFF\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11zW`\0`@Q\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11q\x91\x90a\x1E%V[`@Q\x80\x91\x03\x90\xFD[\x81`\x03`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80\x15a\x12gW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x12^\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xA3[PPPPV[a\x12x\x83\x83\x83a\x15\xCEV[PPPV[a\x12\x87\x82\x82a\t<V[a\x12\xCAW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xC1\x92\x91\x90a ;V[`@Q\x80\x91\x03\x90\xFD[PPV[`\0a\x12\xDA\x83\x83a\t<V[a\x13\xB4W`\x01`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x13Qa\x0B\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x13\xB9V[`\0\x90P[\x92\x91PPV[`\0a\x13\xE7\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x15\xE6V[\x90P\x92\x91PPV[`\0a\x13\xFB\x83\x83a\t<V[\x15a\x14\xD6W`\0\x80`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x14sa\x0B\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x14\xDBV[`\0\x90P[\x92\x91PPV[`\0a\x15\t\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x16VV[\x90P\x92\x91PPV[a\x15\x19a\x08\x13V[a\x15OW`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x15Ya\x08\x13V[\x15a\x15\x90W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x15\xAAWa\x15\xA9a dV[[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x81`\0\x01\x80T\x90P\x90P\x91\x90PV[a\x15\xD6a\x15QV[a\x15\xE1\x83\x83\x83a\x17jV[PPPV[`\0a\x15\xF2\x83\x83a\x19\x92V[a\x16KW\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pa\x16PV[`\0\x90P[\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x14a\x17^W`\0`\x01\x82a\x16\x88\x91\x90a \xC2V[\x90P`\0`\x01\x86`\0\x01\x80T\x90Pa\x16\xA0\x91\x90a \xC2V[\x90P\x80\x82\x14a\x17\x0FW`\0\x86`\0\x01\x82\x81T\x81\x10a\x16\xC1Wa\x16\xC0a dV[[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x16\xE5Wa\x16\xE4a dV[[\x90`\0R` `\0 \x01\x81\x90UP\x83\x87`\x01\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[\x85`\0\x01\x80T\x80a\x17#Wa\x17\"a \xF6V[[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x17dV[`\0\x91PP[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xBCW\x80`\x04`\0\x82\x82Ta\x17\xB0\x91\x90a!%V[\x92PP\x81\x90UPa\x18\x91V[`\0`\x02`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x81\x81\x10\x15a\x18IW\x83\x81\x83`@Q\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18@\x93\x92\x91\x90a \x04V[`@Q\x80\x91\x03\x90\xFD[\x81\x81\x03`\x02`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x18\xDAW\x80`\x04`\0\x82\x82T\x03\x92PP\x81\x90UPa\x19(V[\x80`\x02`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82T\x01\x92PP\x81\x90UP[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\x19\x85\x91\x90a\x1C\nV[`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x19\xEF\x81a\x19\xBAV[\x81\x14a\x19\xFAW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1A\x0C\x81a\x19\xE6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A(Wa\x1A'a\x19\xB5V[[`\0a\x1A6\x84\x82\x85\x01a\x19\xFDV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x1AT\x81a\x1A?V[\x82RPPV[`\0` \x82\x01\x90Pa\x1Ao`\0\x83\x01\x84a\x1AKV[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1A\xAFW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1A\x94V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x1A\xD7\x82a\x1AuV[a\x1A\xE1\x81\x85a\x1A\x80V[\x93Pa\x1A\xF1\x81\x85` \x86\x01a\x1A\x91V[a\x1A\xFA\x81a\x1A\xBBV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1B\x1F\x81\x84a\x1A\xCCV[\x90P\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1BR\x82a\x1B'V[\x90P\x91\x90PV[a\x1Bb\x81a\x1BGV[\x81\x14a\x1BmW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\x7F\x81a\x1BYV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x1B\x98\x81a\x1B\x85V[\x81\x14a\x1B\xA3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\xB5\x81a\x1B\x8FV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\xD2Wa\x1B\xD1a\x19\xB5V[[`\0a\x1B\xE0\x85\x82\x86\x01a\x1BpV[\x92PP` a\x1B\xF1\x85\x82\x86\x01a\x1B\xA6V[\x91PP\x92P\x92\x90PV[a\x1C\x04\x81a\x1B\x85V[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\x1F`\0\x83\x01\x84a\x1B\xFBV[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C>Wa\x1C=a\x19\xB5V[[`\0a\x1CL\x86\x82\x87\x01a\x1BpV[\x93PP` a\x1C]\x86\x82\x87\x01a\x1BpV[\x92PP`@a\x1Cn\x86\x82\x87\x01a\x1B\xA6V[\x91PP\x92P\x92P\x92V[`\0\x81\x90P\x91\x90PV[a\x1C\x8B\x81a\x1CxV[\x81\x14a\x1C\x96W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1C\xA8\x81a\x1C\x82V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\xC4Wa\x1C\xC3a\x19\xB5V[[`\0a\x1C\xD2\x84\x82\x85\x01a\x1C\x99V[\x91PP\x92\x91PPV[a\x1C\xE4\x81a\x1CxV[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\xFF`\0\x83\x01\x84a\x1C\xDBV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\x1CWa\x1D\x1Ba\x19\xB5V[[`\0a\x1D*\x85\x82\x86\x01a\x1C\x99V[\x92PP` a\x1D;\x85\x82\x86\x01a\x1BpV[\x91PP\x92P\x92\x90PV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x1D[\x81a\x1DEV[\x82RPPV[`\0` \x82\x01\x90Pa\x1Dv`\0\x83\x01\x84a\x1DRV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D\x92Wa\x1D\x91a\x19\xB5V[[`\0a\x1D\xA0\x84\x82\x85\x01a\x1B\xA6V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1D\xBFWa\x1D\xBEa\x19\xB5V[[`\0a\x1D\xCD\x84\x82\x85\x01a\x1BpV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xEDWa\x1D\xECa\x19\xB5V[[`\0a\x1D\xFB\x85\x82\x86\x01a\x1C\x99V[\x92PP` a\x1E\x0C\x85\x82\x86\x01a\x1B\xA6V[\x91PP\x92P\x92\x90PV[a\x1E\x1F\x81a\x1BGV[\x82RPPV[`\0` \x82\x01\x90Pa\x1E:`\0\x83\x01\x84a\x1E\x16V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1EWWa\x1EVa\x19\xB5V[[`\0a\x1Ee\x85\x82\x86\x01a\x1BpV[\x92PP` a\x1Ev\x85\x82\x86\x01a\x1BpV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x1E\xC7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\xDAWa\x1E\xD9a\x1E\x80V[[P\x91\x90PV[\x7FERC20PresetMinterPauser: must ha`\0\x82\x01R\x7Fve pauser role to unpause\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1F<`9\x83a\x1A\x80V[\x91Pa\x1FG\x82a\x1E\xE0V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1Fk\x81a\x1F/V[\x90P\x91\x90PV[\x7FERC20PresetMinterPauser: must ha`\0\x82\x01R\x7Fve pauser role to pause\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1F\xCE`7\x83a\x1A\x80V[\x91Pa\x1F\xD9\x82a\x1FrV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1F\xFD\x81a\x1F\xC1V[\x90P\x91\x90PV[`\0``\x82\x01\x90Pa \x19`\0\x83\x01\x86a\x1E\x16V[a &` \x83\x01\x85a\x1B\xFBV[a 3`@\x83\x01\x84a\x1B\xFBV[\x94\x93PPPPV[`\0`@\x82\x01\x90Pa P`\0\x83\x01\x85a\x1E\x16V[a ]` \x83\x01\x84a\x1C\xDBV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a \xCD\x82a\x1B\x85V[\x91Pa \xD8\x83a\x1B\x85V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a \xF0Wa \xEFa \x93V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD[`\0a!0\x82a\x1B\x85V[\x91Pa!;\x83a\x1B\x85V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a!SWa!Ra \x93V[[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x8Dd\xB9\xCA\xE9\xC1{\xF2+\xE4\xC0\x1EO\xE6L\xA1q\xA4@\xD9/}D\x9D\xCC\xED\xB5\xEBT\xB2\xCDFdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static ERC20PRESETMINTERPAUSER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC20PresetMinterPauser<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20PresetMinterPauser<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20PresetMinterPauser<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20PresetMinterPauser<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20PresetMinterPauser<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC20PresetMinterPauser))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20PresetMinterPauser<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20PRESETMINTERPAUSER_ABI.clone(),
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
                ERC20PRESETMINTERPAUSER_ABI.clone(),
                ERC20PRESETMINTERPAUSER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINTER_ROLE` (0xd5391393) function
        pub fn minter_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 57, 19, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PAUSER_ROLE` (0xe63ab1e9) function
        pub fn pauser_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 58, 177, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnFrom` (0x79cc6790) function
        pub fn burn_from(
            &self,
            account: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 204, 103, 144], (account, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMember` (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            caller_confirmation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, caller_confirmation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ERC20PresetMinterPauserEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20PresetMinterPauser<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessControlBadConfirmation` with signature `AccessControlBadConfirmation()` and selector `0x6697b232`
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
        name = "AccessControlBadConfirmation",
        abi = "AccessControlBadConfirmation()"
    )]
    pub struct AccessControlBadConfirmation;
    ///Custom Error type `AccessControlUnauthorizedAccount` with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`
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
        name = "AccessControlUnauthorizedAccount",
        abi = "AccessControlUnauthorizedAccount(address,bytes32)"
    )]
    pub struct AccessControlUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
        pub needed_role: [u8; 32],
    }
    ///Custom Error type `ERC20InsufficientAllowance` with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`
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
        name = "ERC20InsufficientAllowance",
        abi = "ERC20InsufficientAllowance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientAllowance {
        pub spender: ::ethers::core::types::Address,
        pub allowance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InsufficientBalance` with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`
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
        name = "ERC20InsufficientBalance",
        abi = "ERC20InsufficientBalance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InvalidApprover` with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`
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
    #[etherror(name = "ERC20InvalidApprover", abi = "ERC20InvalidApprover(address)")]
    pub struct ERC20InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidReceiver` with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`
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
    #[etherror(name = "ERC20InvalidReceiver", abi = "ERC20InvalidReceiver(address)")]
    pub struct ERC20InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSender` with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`
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
    #[etherror(name = "ERC20InvalidSender", abi = "ERC20InvalidSender(address)")]
    pub struct ERC20InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSpender` with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`
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
    #[etherror(name = "ERC20InvalidSpender", abi = "ERC20InvalidSpender(address)")]
    pub struct ERC20InvalidSpender {
        pub spender: ::ethers::core::types::Address,
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
    pub enum ERC20PresetMinterPauserErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        EnforcedPause(EnforcedPause),
        ExpectedPause(ExpectedPause),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20PresetMinterPauserErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccessControlBadConfirmation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlBadConfirmation(decoded));
            }
            if let Ok(decoded) = <AccessControlUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientAllowance(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidApprover(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidReceiver(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSender(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSpender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSpender(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20PresetMinterPauserErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnforcedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ERC20PresetMinterPauserErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessControlBadConfirmation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSpender as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ERC20PresetMinterPauserErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnforcedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ERC20PresetMinterPauserErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation>
    for ERC20PresetMinterPauserErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount>
    for ERC20PresetMinterPauserErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance>
    for ERC20PresetMinterPauserErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance>
    for ERC20PresetMinterPauserErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for ERC20PresetMinterPauserErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for ERC20PresetMinterPauserErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for ERC20PresetMinterPauserErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for ERC20PresetMinterPauserErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
        }
    }
    impl ::core::convert::From<EnforcedPause> for ERC20PresetMinterPauserErrors {
        fn from(value: EnforcedPause) -> Self {
            Self::EnforcedPause(value)
        }
    }
    impl ::core::convert::From<ExpectedPause> for ERC20PresetMinterPauserErrors {
        fn from(value: ExpectedPause) -> Self {
            Self::ExpectedPause(value)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    pub enum ERC20PresetMinterPauserEvents {
        ApprovalFilter(ApprovalFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC20PresetMinterPauserEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20PresetMinterPauserEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(ERC20PresetMinterPauserEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(
                    ERC20PresetMinterPauserEvents::RoleAdminChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ERC20PresetMinterPauserEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ERC20PresetMinterPauserEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20PresetMinterPauserEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(ERC20PresetMinterPauserEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC20PresetMinterPauserEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC20PresetMinterPauserEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for ERC20PresetMinterPauserEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter>
    for ERC20PresetMinterPauserEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ERC20PresetMinterPauserEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ERC20PresetMinterPauserEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC20PresetMinterPauserEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for ERC20PresetMinterPauserEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`
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
    #[ethcall(name = "MINTER_ROLE", abi = "MINTER_ROLE()")]
    pub struct MinterRoleCall;
    ///Container type for all input parameters for the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
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
    #[ethcall(name = "PAUSER_ROLE", abi = "PAUSER_ROLE()")]
    pub struct PauserRoleCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
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
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `burnFrom` function with signature `burnFrom(address,uint256)` and selector `0x79cc6790`
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
    #[ethcall(name = "burnFrom", abi = "burnFrom(address,uint256)")]
    pub struct BurnFromCall {
        pub account: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub caller_confirmation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    pub enum ERC20PresetMinterPauserCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MinterRole(MinterRoleCall),
        PauserRole(PauserRoleCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnFrom(BurnFromCall),
        Decimals(DecimalsCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Mint(MintCall),
        Name(NameCall),
        Pause(PauseCall),
        Paused(PausedCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Unpause(UnpauseCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20PresetMinterPauserCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <MinterRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinterRole(decoded));
            }
            if let Ok(decoded) = <PauserRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauserRole(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <BurnFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnFrom(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
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
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20PresetMinterPauserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PauserRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BurnFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ERC20PresetMinterPauserCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ERC20PresetMinterPauserCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<MinterRoleCall> for ERC20PresetMinterPauserCalls {
        fn from(value: MinterRoleCall) -> Self {
            Self::MinterRole(value)
        }
    }
    impl ::core::convert::From<PauserRoleCall> for ERC20PresetMinterPauserCalls {
        fn from(value: PauserRoleCall) -> Self {
            Self::PauserRole(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for ERC20PresetMinterPauserCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ERC20PresetMinterPauserCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ERC20PresetMinterPauserCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for ERC20PresetMinterPauserCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<BurnFromCall> for ERC20PresetMinterPauserCalls {
        fn from(value: BurnFromCall) -> Self {
            Self::BurnFrom(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for ERC20PresetMinterPauserCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ERC20PresetMinterPauserCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for ERC20PresetMinterPauserCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for ERC20PresetMinterPauserCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ERC20PresetMinterPauserCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ERC20PresetMinterPauserCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<MintCall> for ERC20PresetMinterPauserCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC20PresetMinterPauserCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<PauseCall> for ERC20PresetMinterPauserCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for ERC20PresetMinterPauserCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ERC20PresetMinterPauserCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ERC20PresetMinterPauserCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ERC20PresetMinterPauserCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ERC20PresetMinterPauserCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ERC20PresetMinterPauserCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for ERC20PresetMinterPauserCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ERC20PresetMinterPauserCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for ERC20PresetMinterPauserCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`
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
    pub struct MinterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
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
    pub struct PauserRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
