pub use i_create_x::*;
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
pub mod i_create_x {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeCreate2Address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreate2Address",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCodeHash"),
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
                                    name: ::std::borrow::ToOwned::to_owned("computedAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreate2Address",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCodeHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deployer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("computedAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeCreate3Address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreate3Address",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deployer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("computedAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreate3Address",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
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
                                    name: ::std::borrow::ToOwned::to_owned("computedAddress"),
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
                    ::std::borrow::ToOwned::to_owned("computeCreateAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreateAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                                    name: ::std::borrow::ToOwned::to_owned("computedAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreateAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deployer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                                    name: ::std::borrow::ToOwned::to_owned("computedAddress"),
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
                    ::std::borrow::ToOwned::to_owned("deployCreate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployCreate2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreate2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreate2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployCreate2AndInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate2AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate2AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate2AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate2AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployCreate2Clone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreate2Clone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreate2Clone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployCreate3"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreate3"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreate3"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployCreate3AndInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate3AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate3AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate3AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreate3AndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployCreateAndInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreateAndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deployCreateAndInit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ICreateX.Values"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployCreateClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployCreateClone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ContractCreation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ContractCreation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ContractCreation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Create3ProxyContractCreation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Create3ProxyContractCreation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FailedContractCreation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FailedContractCreation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emitter"),
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
                    ::std::borrow::ToOwned::to_owned("FailedContractInitialisation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FailedContractInitialisation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emitter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revertData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedEtherTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FailedEtherTransfer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emitter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revertData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNonceValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidNonceValue"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emitter"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidSalt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSalt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emitter"),
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
    pub static ICREATEX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ICreateX<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICreateX<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICreateX<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICreateX<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICreateX<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ICreateX)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ICreateX<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ICREATEX_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `computeCreate2Address` (0x890c283b) function
        pub fn compute_create_2_address(
            &self,
            salt: [u8; 32],
            init_code_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([137, 12, 40, 59], (salt, init_code_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreate2Address` (0xd323826a) function
        pub fn compute_create_2_address_with_salt_and_init_code_hash(
            &self,
            salt: [u8; 32],
            init_code_hash: [u8; 32],
            deployer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([211, 35, 130, 106], (salt, init_code_hash, deployer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreate3Address` (0x42d654fc) function
        pub fn compute_create_3_address_with_deployer(
            &self,
            salt: [u8; 32],
            deployer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([66, 214, 84, 252], (salt, deployer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreate3Address` (0x6cec2536) function
        pub fn compute_create_3_address(
            &self,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([108, 236, 37, 54], salt)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreateAddress` (0x28ddd046) function
        pub fn compute_create_address(
            &self,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([40, 221, 208, 70], nonce)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreateAddress` (0x74637a7a) function
        pub fn compute_create_address_with_deployer(
            &self,
            deployer: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([116, 99, 122, 122], (deployer, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate` (0x27fe1822) function
        pub fn deploy_create(
            &self,
            init_code: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([39, 254, 24, 34], init_code)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2` (0x26307668) function
        pub fn deploy_create_2_with_salt(
            &self,
            salt: [u8; 32],
            init_code: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([38, 48, 118, 104], (salt, init_code))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2` (0x26a32fc7) function
        pub fn deploy_create_2(
            &self,
            init_code: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([38, 163, 47, 199], init_code)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2AndInit` (0xa7db93f2) function
        pub fn deploy_create_2_and_init_3(
            &self,
            salt: [u8; 32],
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [167, 219, 147, 242],
                    (salt, init_code, data, values, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2AndInit` (0xc3fe107b) function
        pub fn deploy_create_2_and_init_0(
            &self,
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([195, 254, 16, 123], (init_code, data, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2AndInit` (0xe437252a) function
        pub fn deploy_create_2_and_init_1(
            &self,
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [228, 55, 37, 42],
                    (init_code, data, values, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2AndInit` (0xe96deee4) function
        pub fn deploy_create_2_and_init_2(
            &self,
            salt: [u8; 32],
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([233, 109, 238, 228], (salt, init_code, data, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2Clone` (0x2852527a) function
        pub fn deploy_create_2_clone_with_salt_and_implementation(
            &self,
            salt: [u8; 32],
            implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([40, 82, 82, 122], (salt, implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate2Clone` (0x81503da1) function
        pub fn deploy_create_2_clone(
            &self,
            implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([129, 80, 61, 161], (implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate3` (0x7f565360) function
        pub fn deploy_create_3(
            &self,
            init_code: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([127, 86, 83, 96], init_code)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate3` (0x9c36a286) function
        pub fn deploy_create_3_with_salt(
            &self,
            salt: [u8; 32],
            init_code: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([156, 54, 162, 134], (salt, init_code))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate3AndInit` (0x00d84acb) function
        pub fn deploy_create_3_and_init_1(
            &self,
            salt: [u8; 32],
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([0, 216, 74, 203], (salt, init_code, data, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate3AndInit` (0x2f990e3f) function
        pub fn deploy_create_3_and_init_0(
            &self,
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([47, 153, 14, 63], (init_code, data, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate3AndInit` (0xddda0acb) function
        pub fn deploy_create_3_and_init_3(
            &self,
            salt: [u8; 32],
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [221, 218, 10, 203],
                    (salt, init_code, data, values, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreate3AndInit` (0xf5745aba) function
        pub fn deploy_create_3_and_init_2(
            &self,
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [245, 116, 90, 186],
                    (init_code, data, values, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreateAndInit` (0x31a7c8c8) function
        pub fn deploy_create_and_init(
            &self,
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([49, 167, 200, 200], (init_code, data, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreateAndInit` (0x98e81077) function
        pub fn deploy_create_and_init_with_init_code_and_data_and_refund_address(
            &self,
            init_code: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
            values: Values,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [152, 232, 16, 119],
                    (init_code, data, values, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployCreateClone` (0xf9664498) function
        pub fn deploy_create_clone(
            &self,
            implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([249, 102, 68, 152], (implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ContractCreation` event
        pub fn contract_creation_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCreation1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCreation` event
        pub fn contract_creation_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCreation2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Create3ProxyContractCreation` event
        pub fn create_3_proxy_contract_creation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Create3ProxyContractCreationFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ICreateXEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ICreateX<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `FailedContractCreation` with signature `FailedContractCreation(address)` and selector `0xc05cee7a`
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
    #[etherror(name = "FailedContractCreation", abi = "FailedContractCreation(address)")]
    pub struct FailedContractCreation {
        pub emitter: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedContractInitialisation` with signature `FailedContractInitialisation(address,bytes)` and selector `0xa57ca239`
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
        name = "FailedContractInitialisation",
        abi = "FailedContractInitialisation(address,bytes)"
    )]
    pub struct FailedContractInitialisation {
        pub emitter: ::ethers::core::types::Address,
        pub revert_data: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `FailedEtherTransfer` with signature `FailedEtherTransfer(address,bytes)` and selector `0xc2b3f445`
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
    #[etherror(name = "FailedEtherTransfer", abi = "FailedEtherTransfer(address,bytes)")]
    pub struct FailedEtherTransfer {
        pub emitter: ::ethers::core::types::Address,
        pub revert_data: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `InvalidNonceValue` with signature `InvalidNonceValue(address)` and selector `0x3c55ab3b`
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
    #[etherror(name = "InvalidNonceValue", abi = "InvalidNonceValue(address)")]
    pub struct InvalidNonceValue {
        pub emitter: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidSalt` with signature `InvalidSalt(address)` and selector `0x13b3a2a1`
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
    #[etherror(name = "InvalidSalt", abi = "InvalidSalt(address)")]
    pub struct InvalidSalt {
        pub emitter: ::ethers::core::types::Address,
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
    pub enum ICreateXErrors {
        FailedContractCreation(FailedContractCreation),
        FailedContractInitialisation(FailedContractInitialisation),
        FailedEtherTransfer(FailedEtherTransfer),
        InvalidNonceValue(InvalidNonceValue),
        InvalidSalt(InvalidSalt),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ICreateXErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <FailedContractCreation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedContractCreation(decoded));
            }
            if let Ok(decoded) = <FailedContractInitialisation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedContractInitialisation(decoded));
            }
            if let Ok(decoded) = <FailedEtherTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedEtherTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidNonceValue as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidNonceValue(decoded));
            }
            if let Ok(decoded) = <InvalidSalt as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSalt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ICreateXErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::FailedContractCreation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedContractInitialisation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedEtherTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNonceValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ICreateXErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <FailedContractCreation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedContractInitialisation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedEtherTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidNonceValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSalt as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ICreateXErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FailedContractCreation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedContractInitialisation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedEtherTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidNonceValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSalt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ICreateXErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<FailedContractCreation> for ICreateXErrors {
        fn from(value: FailedContractCreation) -> Self {
            Self::FailedContractCreation(value)
        }
    }
    impl ::core::convert::From<FailedContractInitialisation> for ICreateXErrors {
        fn from(value: FailedContractInitialisation) -> Self {
            Self::FailedContractInitialisation(value)
        }
    }
    impl ::core::convert::From<FailedEtherTransfer> for ICreateXErrors {
        fn from(value: FailedEtherTransfer) -> Self {
            Self::FailedEtherTransfer(value)
        }
    }
    impl ::core::convert::From<InvalidNonceValue> for ICreateXErrors {
        fn from(value: InvalidNonceValue) -> Self {
            Self::InvalidNonceValue(value)
        }
    }
    impl ::core::convert::From<InvalidSalt> for ICreateXErrors {
        fn from(value: InvalidSalt) -> Self {
            Self::InvalidSalt(value)
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
    #[ethevent(name = "ContractCreation", abi = "ContractCreation(address,bytes32)")]
    pub struct ContractCreation1Filter {
        #[ethevent(indexed)]
        pub new_contract: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub salt: [u8; 32],
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
    #[ethevent(name = "ContractCreation", abi = "ContractCreation(address)")]
    pub struct ContractCreation2Filter {
        #[ethevent(indexed)]
        pub new_contract: ::ethers::core::types::Address,
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
        name = "Create3ProxyContractCreation",
        abi = "Create3ProxyContractCreation(address,bytes32)"
    )]
    pub struct Create3ProxyContractCreationFilter {
        #[ethevent(indexed)]
        pub new_contract: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub salt: [u8; 32],
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
    pub enum ICreateXEvents {
        ContractCreation1Filter(ContractCreation1Filter),
        ContractCreation2Filter(ContractCreation2Filter),
        Create3ProxyContractCreationFilter(Create3ProxyContractCreationFilter),
    }
    impl ::ethers::contract::EthLogDecode for ICreateXEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ContractCreation1Filter::decode_log(log) {
                return Ok(ICreateXEvents::ContractCreation1Filter(decoded));
            }
            if let Ok(decoded) = ContractCreation2Filter::decode_log(log) {
                return Ok(ICreateXEvents::ContractCreation2Filter(decoded));
            }
            if let Ok(decoded) = Create3ProxyContractCreationFilter::decode_log(log) {
                return Ok(ICreateXEvents::Create3ProxyContractCreationFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ICreateXEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ContractCreation1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCreation2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Create3ProxyContractCreationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ContractCreation1Filter> for ICreateXEvents {
        fn from(value: ContractCreation1Filter) -> Self {
            Self::ContractCreation1Filter(value)
        }
    }
    impl ::core::convert::From<ContractCreation2Filter> for ICreateXEvents {
        fn from(value: ContractCreation2Filter) -> Self {
            Self::ContractCreation2Filter(value)
        }
    }
    impl ::core::convert::From<Create3ProxyContractCreationFilter> for ICreateXEvents {
        fn from(value: Create3ProxyContractCreationFilter) -> Self {
            Self::Create3ProxyContractCreationFilter(value)
        }
    }
    ///Container type for all input parameters for the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32)` and selector `0x890c283b`
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
        name = "computeCreate2Address",
        abi = "computeCreate2Address(bytes32,bytes32)"
    )]
    pub struct ComputeCreate2AddressCall {
        pub salt: [u8; 32],
        pub init_code_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32,address)` and selector `0xd323826a`
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
        name = "computeCreate2Address",
        abi = "computeCreate2Address(bytes32,bytes32,address)"
    )]
    pub struct ComputeCreate2AddressWithSaltAndInitCodeHashCall {
        pub salt: [u8; 32],
        pub init_code_hash: [u8; 32],
        pub deployer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `computeCreate3Address` function with signature `computeCreate3Address(bytes32,address)` and selector `0x42d654fc`
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
        name = "computeCreate3Address",
        abi = "computeCreate3Address(bytes32,address)"
    )]
    pub struct ComputeCreate3AddressWithDeployerCall {
        pub salt: [u8; 32],
        pub deployer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `computeCreate3Address` function with signature `computeCreate3Address(bytes32)` and selector `0x6cec2536`
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
    #[ethcall(name = "computeCreate3Address", abi = "computeCreate3Address(bytes32)")]
    pub struct ComputeCreate3AddressCall {
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `computeCreateAddress` function with signature `computeCreateAddress(uint256)` and selector `0x28ddd046`
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
    #[ethcall(name = "computeCreateAddress", abi = "computeCreateAddress(uint256)")]
    pub struct ComputeCreateAddressCall {
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `computeCreateAddress` function with signature `computeCreateAddress(address,uint256)` and selector `0x74637a7a`
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
        name = "computeCreateAddress",
        abi = "computeCreateAddress(address,uint256)"
    )]
    pub struct ComputeCreateAddressWithDeployerCall {
        pub deployer: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deployCreate` function with signature `deployCreate(bytes)` and selector `0x27fe1822`
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
    #[ethcall(name = "deployCreate", abi = "deployCreate(bytes)")]
    pub struct DeployCreateCall {
        pub init_code: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deployCreate2` function with signature `deployCreate2(bytes32,bytes)` and selector `0x26307668`
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
    #[ethcall(name = "deployCreate2", abi = "deployCreate2(bytes32,bytes)")]
    pub struct DeployCreate2WithSaltCall {
        pub salt: [u8; 32],
        pub init_code: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deployCreate2` function with signature `deployCreate2(bytes)` and selector `0x26a32fc7`
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
    #[ethcall(name = "deployCreate2", abi = "deployCreate2(bytes)")]
    pub struct DeployCreate2Call {
        pub init_code: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes32,bytes,bytes,(uint256,uint256),address)` and selector `0xa7db93f2`
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
        name = "deployCreate2AndInit",
        abi = "deployCreate2AndInit(bytes32,bytes,bytes,(uint256,uint256),address)"
    )]
    pub struct DeployCreate2AndInit3Call {
        pub salt: [u8; 32],
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes,bytes,(uint256,uint256))` and selector `0xc3fe107b`
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
        name = "deployCreate2AndInit",
        abi = "deployCreate2AndInit(bytes,bytes,(uint256,uint256))"
    )]
    pub struct DeployCreate2AndInit0Call {
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
    }
    ///Container type for all input parameters for the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes,bytes,(uint256,uint256),address)` and selector `0xe437252a`
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
        name = "deployCreate2AndInit",
        abi = "deployCreate2AndInit(bytes,bytes,(uint256,uint256),address)"
    )]
    pub struct DeployCreate2AndInit1Call {
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes32,bytes,bytes,(uint256,uint256))` and selector `0xe96deee4`
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
        name = "deployCreate2AndInit",
        abi = "deployCreate2AndInit(bytes32,bytes,bytes,(uint256,uint256))"
    )]
    pub struct DeployCreate2AndInit2Call {
        pub salt: [u8; 32],
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
    }
    ///Container type for all input parameters for the `deployCreate2Clone` function with signature `deployCreate2Clone(bytes32,address,bytes)` and selector `0x2852527a`
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
        name = "deployCreate2Clone",
        abi = "deployCreate2Clone(bytes32,address,bytes)"
    )]
    pub struct DeployCreate2CloneWithSaltAndImplementationCall {
        pub salt: [u8; 32],
        pub implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deployCreate2Clone` function with signature `deployCreate2Clone(address,bytes)` and selector `0x81503da1`
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
    #[ethcall(name = "deployCreate2Clone", abi = "deployCreate2Clone(address,bytes)")]
    pub struct DeployCreate2CloneCall {
        pub implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deployCreate3` function with signature `deployCreate3(bytes)` and selector `0x7f565360`
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
    #[ethcall(name = "deployCreate3", abi = "deployCreate3(bytes)")]
    pub struct DeployCreate3Call {
        pub init_code: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deployCreate3` function with signature `deployCreate3(bytes32,bytes)` and selector `0x9c36a286`
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
    #[ethcall(name = "deployCreate3", abi = "deployCreate3(bytes32,bytes)")]
    pub struct DeployCreate3WithSaltCall {
        pub salt: [u8; 32],
        pub init_code: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes32,bytes,bytes,(uint256,uint256))` and selector `0x00d84acb`
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
        name = "deployCreate3AndInit",
        abi = "deployCreate3AndInit(bytes32,bytes,bytes,(uint256,uint256))"
    )]
    pub struct DeployCreate3AndInit1Call {
        pub salt: [u8; 32],
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
    }
    ///Container type for all input parameters for the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes,bytes,(uint256,uint256))` and selector `0x2f990e3f`
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
        name = "deployCreate3AndInit",
        abi = "deployCreate3AndInit(bytes,bytes,(uint256,uint256))"
    )]
    pub struct DeployCreate3AndInit0Call {
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
    }
    ///Container type for all input parameters for the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes32,bytes,bytes,(uint256,uint256),address)` and selector `0xddda0acb`
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
        name = "deployCreate3AndInit",
        abi = "deployCreate3AndInit(bytes32,bytes,bytes,(uint256,uint256),address)"
    )]
    pub struct DeployCreate3AndInit3Call {
        pub salt: [u8; 32],
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes,bytes,(uint256,uint256),address)` and selector `0xf5745aba`
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
        name = "deployCreate3AndInit",
        abi = "deployCreate3AndInit(bytes,bytes,(uint256,uint256),address)"
    )]
    pub struct DeployCreate3AndInit2Call {
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deployCreateAndInit` function with signature `deployCreateAndInit(bytes,bytes,(uint256,uint256))` and selector `0x31a7c8c8`
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
        name = "deployCreateAndInit",
        abi = "deployCreateAndInit(bytes,bytes,(uint256,uint256))"
    )]
    pub struct DeployCreateAndInitCall {
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
    }
    ///Container type for all input parameters for the `deployCreateAndInit` function with signature `deployCreateAndInit(bytes,bytes,(uint256,uint256),address)` and selector `0x98e81077`
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
        name = "deployCreateAndInit",
        abi = "deployCreateAndInit(bytes,bytes,(uint256,uint256),address)"
    )]
    pub struct DeployCreateAndInitWithInitCodeAndDataAndRefundAddressCall {
        pub init_code: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
        pub values: Values,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deployCreateClone` function with signature `deployCreateClone(address,bytes)` and selector `0xf9664498`
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
    #[ethcall(name = "deployCreateClone", abi = "deployCreateClone(address,bytes)")]
    pub struct DeployCreateCloneCall {
        pub implementation: ::ethers::core::types::Address,
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
    pub enum ICreateXCalls {
        ComputeCreate2Address(ComputeCreate2AddressCall),
        ComputeCreate2AddressWithSaltAndInitCodeHash(
            ComputeCreate2AddressWithSaltAndInitCodeHashCall,
        ),
        ComputeCreate3AddressWithDeployer(ComputeCreate3AddressWithDeployerCall),
        ComputeCreate3Address(ComputeCreate3AddressCall),
        ComputeCreateAddress(ComputeCreateAddressCall),
        ComputeCreateAddressWithDeployer(ComputeCreateAddressWithDeployerCall),
        DeployCreate(DeployCreateCall),
        DeployCreate2WithSalt(DeployCreate2WithSaltCall),
        DeployCreate2(DeployCreate2Call),
        DeployCreate2AndInit3(DeployCreate2AndInit3Call),
        DeployCreate2AndInit0(DeployCreate2AndInit0Call),
        DeployCreate2AndInit1(DeployCreate2AndInit1Call),
        DeployCreate2AndInit2(DeployCreate2AndInit2Call),
        DeployCreate2CloneWithSaltAndImplementation(
            DeployCreate2CloneWithSaltAndImplementationCall,
        ),
        DeployCreate2Clone(DeployCreate2CloneCall),
        DeployCreate3(DeployCreate3Call),
        DeployCreate3WithSalt(DeployCreate3WithSaltCall),
        DeployCreate3AndInit1(DeployCreate3AndInit1Call),
        DeployCreate3AndInit0(DeployCreate3AndInit0Call),
        DeployCreate3AndInit3(DeployCreate3AndInit3Call),
        DeployCreate3AndInit2(DeployCreate3AndInit2Call),
        DeployCreateAndInit(DeployCreateAndInitCall),
        DeployCreateAndInitWithInitCodeAndDataAndRefundAddress(
            DeployCreateAndInitWithInitCodeAndDataAndRefundAddressCall,
        ),
        DeployCreateClone(DeployCreateCloneCall),
    }
    impl ::ethers::core::abi::AbiDecode for ICreateXCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ComputeCreate2AddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate2Address(decoded));
            }
            if let Ok(decoded) = <ComputeCreate2AddressWithSaltAndInitCodeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate2AddressWithSaltAndInitCodeHash(decoded));
            }
            if let Ok(decoded) = <ComputeCreate3AddressWithDeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate3AddressWithDeployer(decoded));
            }
            if let Ok(decoded) = <ComputeCreate3AddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate3Address(decoded));
            }
            if let Ok(decoded) = <ComputeCreateAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreateAddress(decoded));
            }
            if let Ok(decoded) = <ComputeCreateAddressWithDeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreateAddressWithDeployer(decoded));
            }
            if let Ok(decoded) = <DeployCreateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate(decoded));
            }
            if let Ok(decoded) = <DeployCreate2WithSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2WithSalt(decoded));
            }
            if let Ok(decoded) = <DeployCreate2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2(decoded));
            }
            if let Ok(decoded) = <DeployCreate2AndInit3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2AndInit3(decoded));
            }
            if let Ok(decoded) = <DeployCreate2AndInit0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2AndInit0(decoded));
            }
            if let Ok(decoded) = <DeployCreate2AndInit1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2AndInit1(decoded));
            }
            if let Ok(decoded) = <DeployCreate2AndInit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2AndInit2(decoded));
            }
            if let Ok(decoded) = <DeployCreate2CloneWithSaltAndImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2CloneWithSaltAndImplementation(decoded));
            }
            if let Ok(decoded) = <DeployCreate2CloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate2Clone(decoded));
            }
            if let Ok(decoded) = <DeployCreate3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate3(decoded));
            }
            if let Ok(decoded) = <DeployCreate3WithSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate3WithSalt(decoded));
            }
            if let Ok(decoded) = <DeployCreate3AndInit1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate3AndInit1(decoded));
            }
            if let Ok(decoded) = <DeployCreate3AndInit0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate3AndInit0(decoded));
            }
            if let Ok(decoded) = <DeployCreate3AndInit3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate3AndInit3(decoded));
            }
            if let Ok(decoded) = <DeployCreate3AndInit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreate3AndInit2(decoded));
            }
            if let Ok(decoded) = <DeployCreateAndInitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreateAndInit(decoded));
            }
            if let Ok(decoded) = <DeployCreateAndInitWithInitCodeAndDataAndRefundAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::DeployCreateAndInitWithInitCodeAndDataAndRefundAddress(decoded),
                );
            }
            if let Ok(decoded) = <DeployCreateCloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployCreateClone(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ICreateXCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeCreate2Address(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreate2AddressWithSaltAndInitCodeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreate3AddressWithDeployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreate3Address(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreateAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreateAddressWithDeployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2WithSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2AndInit3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2AndInit0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2AndInit1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2AndInit2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2CloneWithSaltAndImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate2Clone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate3WithSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate3AndInit1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate3AndInit0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate3AndInit3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreate3AndInit2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreateAndInit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreateAndInitWithInitCodeAndDataAndRefundAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployCreateClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ICreateXCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeCreate2Address(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreate2AddressWithSaltAndInitCodeHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreate3AddressWithDeployer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreate3Address(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreateAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreateAddressWithDeployer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployCreate2WithSalt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate2(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployCreate2AndInit3(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate2AndInit0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate2AndInit1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate2AndInit2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate2CloneWithSaltAndImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate2Clone(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate3(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployCreate3WithSalt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate3AndInit1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate3AndInit0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate3AndInit3(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreate3AndInit2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreateAndInit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreateAndInitWithInitCodeAndDataAndRefundAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployCreateClone(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeCreate2AddressCall> for ICreateXCalls {
        fn from(value: ComputeCreate2AddressCall) -> Self {
            Self::ComputeCreate2Address(value)
        }
    }
    impl ::core::convert::From<ComputeCreate2AddressWithSaltAndInitCodeHashCall>
    for ICreateXCalls {
        fn from(value: ComputeCreate2AddressWithSaltAndInitCodeHashCall) -> Self {
            Self::ComputeCreate2AddressWithSaltAndInitCodeHash(value)
        }
    }
    impl ::core::convert::From<ComputeCreate3AddressWithDeployerCall> for ICreateXCalls {
        fn from(value: ComputeCreate3AddressWithDeployerCall) -> Self {
            Self::ComputeCreate3AddressWithDeployer(value)
        }
    }
    impl ::core::convert::From<ComputeCreate3AddressCall> for ICreateXCalls {
        fn from(value: ComputeCreate3AddressCall) -> Self {
            Self::ComputeCreate3Address(value)
        }
    }
    impl ::core::convert::From<ComputeCreateAddressCall> for ICreateXCalls {
        fn from(value: ComputeCreateAddressCall) -> Self {
            Self::ComputeCreateAddress(value)
        }
    }
    impl ::core::convert::From<ComputeCreateAddressWithDeployerCall> for ICreateXCalls {
        fn from(value: ComputeCreateAddressWithDeployerCall) -> Self {
            Self::ComputeCreateAddressWithDeployer(value)
        }
    }
    impl ::core::convert::From<DeployCreateCall> for ICreateXCalls {
        fn from(value: DeployCreateCall) -> Self {
            Self::DeployCreate(value)
        }
    }
    impl ::core::convert::From<DeployCreate2WithSaltCall> for ICreateXCalls {
        fn from(value: DeployCreate2WithSaltCall) -> Self {
            Self::DeployCreate2WithSalt(value)
        }
    }
    impl ::core::convert::From<DeployCreate2Call> for ICreateXCalls {
        fn from(value: DeployCreate2Call) -> Self {
            Self::DeployCreate2(value)
        }
    }
    impl ::core::convert::From<DeployCreate2AndInit3Call> for ICreateXCalls {
        fn from(value: DeployCreate2AndInit3Call) -> Self {
            Self::DeployCreate2AndInit3(value)
        }
    }
    impl ::core::convert::From<DeployCreate2AndInit0Call> for ICreateXCalls {
        fn from(value: DeployCreate2AndInit0Call) -> Self {
            Self::DeployCreate2AndInit0(value)
        }
    }
    impl ::core::convert::From<DeployCreate2AndInit1Call> for ICreateXCalls {
        fn from(value: DeployCreate2AndInit1Call) -> Self {
            Self::DeployCreate2AndInit1(value)
        }
    }
    impl ::core::convert::From<DeployCreate2AndInit2Call> for ICreateXCalls {
        fn from(value: DeployCreate2AndInit2Call) -> Self {
            Self::DeployCreate2AndInit2(value)
        }
    }
    impl ::core::convert::From<DeployCreate2CloneWithSaltAndImplementationCall>
    for ICreateXCalls {
        fn from(value: DeployCreate2CloneWithSaltAndImplementationCall) -> Self {
            Self::DeployCreate2CloneWithSaltAndImplementation(value)
        }
    }
    impl ::core::convert::From<DeployCreate2CloneCall> for ICreateXCalls {
        fn from(value: DeployCreate2CloneCall) -> Self {
            Self::DeployCreate2Clone(value)
        }
    }
    impl ::core::convert::From<DeployCreate3Call> for ICreateXCalls {
        fn from(value: DeployCreate3Call) -> Self {
            Self::DeployCreate3(value)
        }
    }
    impl ::core::convert::From<DeployCreate3WithSaltCall> for ICreateXCalls {
        fn from(value: DeployCreate3WithSaltCall) -> Self {
            Self::DeployCreate3WithSalt(value)
        }
    }
    impl ::core::convert::From<DeployCreate3AndInit1Call> for ICreateXCalls {
        fn from(value: DeployCreate3AndInit1Call) -> Self {
            Self::DeployCreate3AndInit1(value)
        }
    }
    impl ::core::convert::From<DeployCreate3AndInit0Call> for ICreateXCalls {
        fn from(value: DeployCreate3AndInit0Call) -> Self {
            Self::DeployCreate3AndInit0(value)
        }
    }
    impl ::core::convert::From<DeployCreate3AndInit3Call> for ICreateXCalls {
        fn from(value: DeployCreate3AndInit3Call) -> Self {
            Self::DeployCreate3AndInit3(value)
        }
    }
    impl ::core::convert::From<DeployCreate3AndInit2Call> for ICreateXCalls {
        fn from(value: DeployCreate3AndInit2Call) -> Self {
            Self::DeployCreate3AndInit2(value)
        }
    }
    impl ::core::convert::From<DeployCreateAndInitCall> for ICreateXCalls {
        fn from(value: DeployCreateAndInitCall) -> Self {
            Self::DeployCreateAndInit(value)
        }
    }
    impl ::core::convert::From<
        DeployCreateAndInitWithInitCodeAndDataAndRefundAddressCall,
    > for ICreateXCalls {
        fn from(
            value: DeployCreateAndInitWithInitCodeAndDataAndRefundAddressCall,
        ) -> Self {
            Self::DeployCreateAndInitWithInitCodeAndDataAndRefundAddress(value)
        }
    }
    impl ::core::convert::From<DeployCreateCloneCall> for ICreateXCalls {
        fn from(value: DeployCreateCloneCall) -> Self {
            Self::DeployCreateClone(value)
        }
    }
    ///Container type for all return fields from the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32)` and selector `0x890c283b`
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
    pub struct ComputeCreate2AddressReturn {
        pub computed_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32,address)` and selector `0xd323826a`
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
    pub struct ComputeCreate2AddressWithSaltAndInitCodeHashReturn {
        pub computed_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `computeCreate3Address` function with signature `computeCreate3Address(bytes32,address)` and selector `0x42d654fc`
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
    pub struct ComputeCreate3AddressWithDeployerReturn {
        pub computed_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `computeCreate3Address` function with signature `computeCreate3Address(bytes32)` and selector `0x6cec2536`
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
    pub struct ComputeCreate3AddressReturn {
        pub computed_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `computeCreateAddress` function with signature `computeCreateAddress(uint256)` and selector `0x28ddd046`
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
    pub struct ComputeCreateAddressReturn {
        pub computed_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `computeCreateAddress` function with signature `computeCreateAddress(address,uint256)` and selector `0x74637a7a`
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
    pub struct ComputeCreateAddressWithDeployerReturn {
        pub computed_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate` function with signature `deployCreate(bytes)` and selector `0x27fe1822`
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
    pub struct DeployCreateReturn {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2` function with signature `deployCreate2(bytes32,bytes)` and selector `0x26307668`
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
    pub struct DeployCreate2WithSaltReturn {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2` function with signature `deployCreate2(bytes)` and selector `0x26a32fc7`
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
    pub struct DeployCreate2Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes32,bytes,bytes,(uint256,uint256),address)` and selector `0xa7db93f2`
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
    pub struct DeployCreate2AndInit3Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes,bytes,(uint256,uint256))` and selector `0xc3fe107b`
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
    pub struct DeployCreate2AndInit0Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes,bytes,(uint256,uint256),address)` and selector `0xe437252a`
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
    pub struct DeployCreate2AndInit1Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2AndInit` function with signature `deployCreate2AndInit(bytes32,bytes,bytes,(uint256,uint256))` and selector `0xe96deee4`
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
    pub struct DeployCreate2AndInit2Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2Clone` function with signature `deployCreate2Clone(bytes32,address,bytes)` and selector `0x2852527a`
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
    pub struct DeployCreate2CloneWithSaltAndImplementationReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate2Clone` function with signature `deployCreate2Clone(address,bytes)` and selector `0x81503da1`
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
    pub struct DeployCreate2CloneReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate3` function with signature `deployCreate3(bytes)` and selector `0x7f565360`
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
    pub struct DeployCreate3Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate3` function with signature `deployCreate3(bytes32,bytes)` and selector `0x9c36a286`
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
    pub struct DeployCreate3WithSaltReturn {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes32,bytes,bytes,(uint256,uint256))` and selector `0x00d84acb`
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
    pub struct DeployCreate3AndInit1Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes,bytes,(uint256,uint256))` and selector `0x2f990e3f`
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
    pub struct DeployCreate3AndInit0Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes32,bytes,bytes,(uint256,uint256),address)` and selector `0xddda0acb`
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
    pub struct DeployCreate3AndInit3Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreate3AndInit` function with signature `deployCreate3AndInit(bytes,bytes,(uint256,uint256),address)` and selector `0xf5745aba`
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
    pub struct DeployCreate3AndInit2Return {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreateAndInit` function with signature `deployCreateAndInit(bytes,bytes,(uint256,uint256))` and selector `0x31a7c8c8`
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
    pub struct DeployCreateAndInitReturn {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreateAndInit` function with signature `deployCreateAndInit(bytes,bytes,(uint256,uint256),address)` and selector `0x98e81077`
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
    pub struct DeployCreateAndInitWithInitCodeAndDataAndRefundAddressReturn {
        pub new_contract: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployCreateClone` function with signature `deployCreateClone(address,bytes)` and selector `0xf9664498`
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
    pub struct DeployCreateCloneReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///`Values(uint256,uint256)`
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
    pub struct Values {
        pub constructor_amount: ::ethers::core::types::U256,
        pub init_call_amount: ::ethers::core::types::U256,
    }
}
