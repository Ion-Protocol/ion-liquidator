pub use yield_oracle_shared_setup::*;
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
pub mod yield_oracle_shared_setup {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
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
                    ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedContracts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("excludedSenders_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failed"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("oracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract YieldOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifactSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "targetArtifactSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifactSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedContracts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetInterfaces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetInterfaces"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedInterfaces_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzInterface[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSelectors"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetedSenders_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("test_SetUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("test_SetUp"),
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
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_int",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_uint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
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
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("logs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static YIELDORACLESHAREDSETUP_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct YieldOracleSharedSetup<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for YieldOracleSharedSetup<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for YieldOracleSharedSetup<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for YieldOracleSharedSetup<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for YieldOracleSharedSetup<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(YieldOracleSharedSetup))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> YieldOracleSharedSetup<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    YIELDORACLESHAREDSETUP_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `IS_TEST` (0xfa7626d4) function
        pub fn is_test(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeArtifacts` (0xb5508aa9) function
        pub fn exclude_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([181, 80, 138, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeContracts` (0xe20c9f71) function
        pub fn exclude_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([226, 12, 159, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeSenders` (0x1ed7831c) function
        pub fn exclude_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([30, 215, 131, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracle` (0x7dc0d1d0) function
        pub fn oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUp` (0x0a9254e4) function
        pub fn set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function
        pub fn target_artifact_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifacts` (0x85226c81) function
        pub fn target_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([133, 34, 108, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetContracts` (0x3f7286f4) function
        pub fn target_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 114, 134, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetInterfaces` (0x2ade3880) function
        pub fn target_interfaces(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzInterface>,
        > {
            self.0
                .method_hash([42, 222, 56, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSelectors` (0x916a17c6) function
        pub fn target_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([145, 106, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSenders` (0x3e5e3c23) function
        pub fn target_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([62, 94, 60, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `test_SetUp` (0x7dc85252) function
        pub fn test_set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 200, 82, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_int` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_named_address` event
        pub fn log_named_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_uint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        ///Gets the contract's `logs` event
        pub fn logs_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            YieldOracleSharedSetupEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for YieldOracleSharedSetup<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ::ethers::core::types::Address);
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
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ::ethers::core::types::Bytes);
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
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
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
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub ::ethers::core::types::I256);
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
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: ::std::string::String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
        pub decimals: ::ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: ::std::string::String,
        pub val: ::std::string::String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ::ethers::core::types::U256);
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
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
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
    pub enum YieldOracleSharedSetupEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ::ethers::contract::EthLogDecode for YieldOracleSharedSetupEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(
                    YieldOracleSharedSetupEvents::LogNamedDecimalIntFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(
                    YieldOracleSharedSetupEvents::LogNamedDecimalUintFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(YieldOracleSharedSetupEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for YieldOracleSharedSetupEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytes32Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalIntFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter>
    for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter>
    for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for YieldOracleSharedSetupEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    #[ethcall(name = "excludeArtifacts", abi = "excludeArtifacts()")]
    pub struct ExcludeArtifactsCall;
    ///Container type for all input parameters for the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
    ///Container type for all input parameters for the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    #[ethcall(name = "excludeSenders", abi = "excludeSenders()")]
    pub struct ExcludeSendersCall;
    ///Container type for all input parameters for the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    ///Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `0x0a9254e4`
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
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    ///Container type for all input parameters for the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    #[ethcall(name = "targetArtifactSelectors", abi = "targetArtifactSelectors()")]
    pub struct TargetArtifactSelectorsCall;
    ///Container type for all input parameters for the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    #[ethcall(name = "targetArtifacts", abi = "targetArtifacts()")]
    pub struct TargetArtifactsCall;
    ///Container type for all input parameters for the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    #[ethcall(name = "targetContracts", abi = "targetContracts()")]
    pub struct TargetContractsCall;
    ///Container type for all input parameters for the `targetInterfaces` function with signature `targetInterfaces()` and selector `0x2ade3880`
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
    #[ethcall(name = "targetInterfaces", abi = "targetInterfaces()")]
    pub struct TargetInterfacesCall;
    ///Container type for all input parameters for the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    #[ethcall(name = "targetSelectors", abi = "targetSelectors()")]
    pub struct TargetSelectorsCall;
    ///Container type for all input parameters for the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
    ///Container type for all input parameters for the `test_SetUp` function with signature `test_SetUp()` and selector `0x7dc85252`
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
    #[ethcall(name = "test_SetUp", abi = "test_SetUp()")]
    pub struct TestSetUpCall;
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
    pub enum YieldOracleSharedSetupCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        Oracle(OracleCall),
        SetUp(SetUpCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetInterfaces(TargetInterfacesCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TestSetUp(TestSetUpCall),
    }
    impl ::ethers::core::abi::AbiDecode for YieldOracleSharedSetupCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) = <ExcludeArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) = <ExcludeContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeContracts(decoded));
            }
            if let Ok(decoded) = <ExcludeSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <OracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Oracle(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUp(decoded));
            }
            if let Ok(decoded) = <TargetArtifactSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) = <TargetArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifacts(decoded));
            }
            if let Ok(decoded) = <TargetContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetContracts(decoded));
            }
            if let Ok(decoded) = <TargetInterfacesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetInterfaces(decoded));
            }
            if let Ok(decoded) = <TargetSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSelectors(decoded));
            }
            if let Ok(decoded) = <TargetSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSenders(decoded));
            }
            if let Ok(decoded) = <TestSetUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TestSetUp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for YieldOracleSharedSetupCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Oracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetInterfaces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestSetUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for YieldOracleSharedSetupCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Oracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetInterfaces(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestSetUp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for YieldOracleSharedSetupCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for YieldOracleSharedSetupCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for YieldOracleSharedSetupCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for YieldOracleSharedSetupCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for YieldOracleSharedSetupCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<OracleCall> for YieldOracleSharedSetupCalls {
        fn from(value: OracleCall) -> Self {
            Self::Oracle(value)
        }
    }
    impl ::core::convert::From<SetUpCall> for YieldOracleSharedSetupCalls {
        fn from(value: SetUpCall) -> Self {
            Self::SetUp(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall>
    for YieldOracleSharedSetupCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for YieldOracleSharedSetupCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for YieldOracleSharedSetupCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetInterfacesCall> for YieldOracleSharedSetupCalls {
        fn from(value: TargetInterfacesCall) -> Self {
            Self::TargetInterfaces(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for YieldOracleSharedSetupCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for YieldOracleSharedSetupCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TestSetUpCall> for YieldOracleSharedSetupCalls {
        fn from(value: TestSetUpCall) -> Self {
            Self::TestSetUp(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    pub struct IsTestReturn(pub bool);
    ///Container type for all return fields from the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    pub struct ExcludeArtifactsReturn {
        pub excluded_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    pub struct ExcludeSendersReturn {
        pub excluded_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    pub struct FailedReturn(pub bool);
    ///Container type for all return fields from the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    pub struct OracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    pub struct TargetArtifactsReturn {
        pub targeted_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    pub struct TargetContractsReturn {
        pub targeted_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `targetInterfaces` function with signature `targetInterfaces()` and selector `0x2ade3880`
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
    pub struct TargetInterfacesReturn {
        pub targeted_interfaces: ::std::vec::Vec<FuzzInterface>,
    }
    ///Container type for all return fields from the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    pub struct TargetSelectorsReturn {
        pub targeted_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
}
