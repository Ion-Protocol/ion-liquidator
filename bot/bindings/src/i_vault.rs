pub use i_vault::*;
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
pub mod i_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IWETH"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("batchSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum IVault.SwapKind"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swaps"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.BatchSwapStep[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAsset[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("funds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.FundManagement",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("limits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deregisterTokens"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
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
                    ::std::borrow::ToOwned::to_owned("exitPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exitPool"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("request"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.ExitPoolRequest",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("flashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IFlashLoanRecipient",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                (
                    ::std::borrow::ToOwned::to_owned("getActionId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActionId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("selector"),
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
                    ::std::borrow::ToOwned::to_owned("getAuthorizer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAuthorizer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAuthorizer"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDomainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDomainSeparator"),
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
                    ::std::borrow::ToOwned::to_owned("getInternalBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInternalBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
                    ::std::borrow::ToOwned::to_owned("getPausedState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPausedState"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paused"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "pauseWindowEndTime",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "bufferPeriodEndTime",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IVault.PoolSpecialization",
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
                    ::std::borrow::ToOwned::to_owned("getPoolTokenInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolTokenInfo"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("managed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastChangeBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetManager"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                                    name: ::std::borrow::ToOwned::to_owned("balances"),
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
                                    name: ::std::borrow::ToOwned::to_owned("lastChangeBlock"),
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
                    ::std::borrow::ToOwned::to_owned("getProtocolFeesCollector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProtocolFeesCollector",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IProtocolFeesCollector",
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
                    ::std::borrow::ToOwned::to_owned("hasApprovedRelayer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasApprovedRelayer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("relayer"),
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
                    ::std::borrow::ToOwned::to_owned("joinPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("joinPool"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("request"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.JoinPoolRequest",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("managePoolBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("managePoolBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ops"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.PoolBalanceOp[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("manageUserBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("manageUserBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ops"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.UserBalanceOp[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queryBatchSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queryBatchSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum IVault.SwapKind"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swaps"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.BatchSwapStep[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAsset[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("funds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.FundManagement",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetDeltas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("specialization"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IVault.PoolSpecialization",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerTokens"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                                    name: ::std::borrow::ToOwned::to_owned("assetManagers"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAuthorizer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAuthorizer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAuthorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAuthorizer"),
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
                    ::std::borrow::ToOwned::to_owned("setPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paused"),
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
                    ::std::borrow::ToOwned::to_owned("setRelayerApproval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRelayerApproval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("relayer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("singleSwap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IVault.SingleSwap"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("funds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.FundManagement",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("limit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("AuthorizerChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AuthorizerChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAuthorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExternalBalanceTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExternalBalanceTransfer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("FlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeAmount"),
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
                    ::std::borrow::ToOwned::to_owned("InternalBalanceChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InternalBalanceChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PausedStateChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PausedStateChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paused"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolBalanceChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PoolBalanceChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityProvider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "protocolFeeAmounts",
                                    ),
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolBalanceManaged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PoolBalanceManaged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cashDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("managedDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PoolRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("specialization"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayerApprovalChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayerApprovalChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("relayer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
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
                    ::std::borrow::ToOwned::to_owned("TokensDeregistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokensDeregistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                    ::std::borrow::ToOwned::to_owned("TokensRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokensRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetManagers"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IVAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IVault)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IVAULT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchSwap` (0x945bcec9) function
        pub fn batch_swap(
            &self,
            kind: u8,
            swaps: ::std::vec::Vec<BatchSwapStep>,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            funds: FundManagement,
            limits: ::std::vec::Vec<::ethers::core::types::I256>,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash(
                    [148, 91, 206, 201],
                    (kind, swaps, assets, funds, limits, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterTokens` (0x7d3aeb96) function
        pub fn deregister_tokens(
            &self,
            pool_id: [u8; 32],
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 58, 235, 150], (pool_id, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exitPool` (0x8bdb3913) function
        pub fn exit_pool(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            request: ExitPoolRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 219, 57, 19], (pool_id, sender, recipient, request))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoan` (0x5c38449e) function
        pub fn flash_loan(
            &self,
            recipient: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 56, 68, 158], (recipient, tokens, amounts, user_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActionId` (0x851c1bb3) function
        pub fn get_action_id(
            &self,
            selector: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([133, 28, 27, 179], selector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAuthorizer` (0xaaabadc5) function
        pub fn get_authorizer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([170, 171, 173, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDomainSeparator` (0xed24911d) function
        pub fn get_domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([237, 36, 145, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInternalBalance` (0x0f5a6efa) function
        pub fn get_internal_balance(
            &self,
            user: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([15, 90, 110, 250], (user, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextNonce` (0x90193b7c) function
        pub fn get_next_nonce(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 25, 59, 124], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPausedState` (0x1c0de051) function
        pub fn get_paused_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([28, 13, 224, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0xf6c00927) function
        pub fn get_pool(
            &self,
            pool_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u8),
        > {
            self.0
                .method_hash([246, 192, 9, 39], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolTokenInfo` (0xb05f8e48) function
        pub fn get_pool_token_info(
            &self,
            pool_id: [u8; 32],
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([176, 95, 142, 72], (pool_id, token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolTokens` (0xf94d4668) function
        pub fn get_pool_tokens(
            &self,
            pool_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([249, 77, 70, 104], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProtocolFeesCollector` (0xd2946c2b) function
        pub fn get_protocol_fees_collector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 148, 108, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasApprovedRelayer` (0xfec90d72) function
        pub fn has_approved_relayer(
            &self,
            user: ::ethers::core::types::Address,
            relayer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([254, 201, 13, 114], (user, relayer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `joinPool` (0xb95cac28) function
        pub fn join_pool(
            &self,
            pool_id: [u8; 32],
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            request: JoinPoolRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 92, 172, 40], (pool_id, sender, recipient, request))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `managePoolBalance` (0xe6c46092) function
        pub fn manage_pool_balance(
            &self,
            ops: ::std::vec::Vec<PoolBalanceOp>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 196, 96, 146], ops)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manageUserBalance` (0x0e8e3e84) function
        pub fn manage_user_balance(
            &self,
            ops: ::std::vec::Vec<UserBalanceOp>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 142, 62, 132], ops)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryBatchSwap` (0xf84d066e) function
        pub fn query_batch_swap(
            &self,
            kind: u8,
            swaps: ::std::vec::Vec<BatchSwapStep>,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            funds: FundManagement,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash([248, 77, 6, 110], (kind, swaps, assets, funds))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerPool` (0x09b2760f) function
        pub fn register_pool(
            &self,
            specialization: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([9, 178, 118, 15], specialization)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerTokens` (0x66a9c7d2) function
        pub fn register_tokens(
            &self,
            pool_id: [u8; 32],
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            asset_managers: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 169, 199, 210], (pool_id, tokens, asset_managers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuthorizer` (0x058a628f) function
        pub fn set_authorizer(
            &self,
            new_authorizer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 138, 98, 143], new_authorizer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPaused` (0x16c38b3c) function
        pub fn set_paused(
            &self,
            paused: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 195, 139, 60], paused)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRelayerApproval` (0xfa6e671d) function
        pub fn set_relayer_approval(
            &self,
            sender: ::ethers::core::types::Address,
            relayer: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 110, 103, 29], (sender, relayer, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x52bbbe29) function
        pub fn swap(
            &self,
            single_swap: SingleSwap,
            funds: FundManagement,
            limit: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([82, 187, 190, 41], (single_swap, funds, limit, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AuthorizerChanged` event
        pub fn authorizer_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorizerChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExternalBalanceTransfer` event
        pub fn external_balance_transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExternalBalanceTransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FlashLoan` event
        pub fn flash_loan_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FlashLoanFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InternalBalanceChanged` event
        pub fn internal_balance_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InternalBalanceChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PausedStateChanged` event
        pub fn paused_state_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PausedStateChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PoolBalanceChanged` event
        pub fn pool_balance_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolBalanceChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PoolBalanceManaged` event
        pub fn pool_balance_managed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolBalanceManagedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PoolRegistered` event
        pub fn pool_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayerApprovalChanged` event
        pub fn relayer_approval_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayerApprovalChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokensDeregistered` event
        pub fn tokens_deregistered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokensDeregisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokensRegistered` event
        pub fn tokens_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokensRegisteredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IVaultEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IVault<M> {
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
    #[ethevent(name = "AuthorizerChanged", abi = "AuthorizerChanged(address)")]
    pub struct AuthorizerChangedFilter {
        #[ethevent(indexed)]
        pub new_authorizer: ::ethers::core::types::Address,
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
        name = "ExternalBalanceTransfer",
        abi = "ExternalBalanceTransfer(address,address,address,uint256)"
    )]
    pub struct ExternalBalanceTransferFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "FlashLoan", abi = "FlashLoan(address,address,uint256,uint256)")]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub fee_amount: ::ethers::core::types::U256,
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
        name = "InternalBalanceChanged",
        abi = "InternalBalanceChanged(address,address,int256)"
    )]
    pub struct InternalBalanceChangedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub delta: ::ethers::core::types::I256,
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
    #[ethevent(name = "PausedStateChanged", abi = "PausedStateChanged(bool)")]
    pub struct PausedStateChangedFilter {
        pub paused: bool,
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
        name = "PoolBalanceChanged",
        abi = "PoolBalanceChanged(bytes32,address,address[],int256[],uint256[])"
    )]
    pub struct PoolBalanceChangedFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub liquidity_provider: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub deltas: ::std::vec::Vec<::ethers::core::types::I256>,
        pub protocol_fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
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
        name = "PoolBalanceManaged",
        abi = "PoolBalanceManaged(bytes32,address,address,int256,int256)"
    )]
    pub struct PoolBalanceManagedFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub asset_manager: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub cash_delta: ::ethers::core::types::I256,
        pub managed_delta: ::ethers::core::types::I256,
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
    #[ethevent(name = "PoolRegistered", abi = "PoolRegistered(bytes32,address,uint8)")]
    pub struct PoolRegisteredFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub pool_address: ::ethers::core::types::Address,
        pub specialization: u8,
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
        name = "RelayerApprovalChanged",
        abi = "RelayerApprovalChanged(address,address,bool)"
    )]
    pub struct RelayerApprovalChangedFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "Swap", abi = "Swap(bytes32,address,address,uint256,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
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
        name = "TokensDeregistered",
        abi = "TokensDeregistered(bytes32,address[])"
    )]
    pub struct TokensDeregisteredFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
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
        name = "TokensRegistered",
        abi = "TokensRegistered(bytes32,address[],address[])"
    )]
    pub struct TokensRegisteredFilter {
        #[ethevent(indexed)]
        pub pool_id: [u8; 32],
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub asset_managers: ::std::vec::Vec<::ethers::core::types::Address>,
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
    pub enum IVaultEvents {
        AuthorizerChangedFilter(AuthorizerChangedFilter),
        ExternalBalanceTransferFilter(ExternalBalanceTransferFilter),
        FlashLoanFilter(FlashLoanFilter),
        InternalBalanceChangedFilter(InternalBalanceChangedFilter),
        PausedStateChangedFilter(PausedStateChangedFilter),
        PoolBalanceChangedFilter(PoolBalanceChangedFilter),
        PoolBalanceManagedFilter(PoolBalanceManagedFilter),
        PoolRegisteredFilter(PoolRegisteredFilter),
        RelayerApprovalChangedFilter(RelayerApprovalChangedFilter),
        SwapFilter(SwapFilter),
        TokensDeregisteredFilter(TokensDeregisteredFilter),
        TokensRegisteredFilter(TokensRegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for IVaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AuthorizerChangedFilter::decode_log(log) {
                return Ok(IVaultEvents::AuthorizerChangedFilter(decoded));
            }
            if let Ok(decoded) = ExternalBalanceTransferFilter::decode_log(log) {
                return Ok(IVaultEvents::ExternalBalanceTransferFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(IVaultEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = InternalBalanceChangedFilter::decode_log(log) {
                return Ok(IVaultEvents::InternalBalanceChangedFilter(decoded));
            }
            if let Ok(decoded) = PausedStateChangedFilter::decode_log(log) {
                return Ok(IVaultEvents::PausedStateChangedFilter(decoded));
            }
            if let Ok(decoded) = PoolBalanceChangedFilter::decode_log(log) {
                return Ok(IVaultEvents::PoolBalanceChangedFilter(decoded));
            }
            if let Ok(decoded) = PoolBalanceManagedFilter::decode_log(log) {
                return Ok(IVaultEvents::PoolBalanceManagedFilter(decoded));
            }
            if let Ok(decoded) = PoolRegisteredFilter::decode_log(log) {
                return Ok(IVaultEvents::PoolRegisteredFilter(decoded));
            }
            if let Ok(decoded) = RelayerApprovalChangedFilter::decode_log(log) {
                return Ok(IVaultEvents::RelayerApprovalChangedFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IVaultEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = TokensDeregisteredFilter::decode_log(log) {
                return Ok(IVaultEvents::TokensDeregisteredFilter(decoded));
            }
            if let Ok(decoded) = TokensRegisteredFilter::decode_log(log) {
                return Ok(IVaultEvents::TokensRegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IVaultEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuthorizerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExternalBalanceTransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashLoanFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalBalanceChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedStateChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolBalanceChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolBalanceManagedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayerApprovalChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokensDeregisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokensRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AuthorizerChangedFilter> for IVaultEvents {
        fn from(value: AuthorizerChangedFilter) -> Self {
            Self::AuthorizerChangedFilter(value)
        }
    }
    impl ::core::convert::From<ExternalBalanceTransferFilter> for IVaultEvents {
        fn from(value: ExternalBalanceTransferFilter) -> Self {
            Self::ExternalBalanceTransferFilter(value)
        }
    }
    impl ::core::convert::From<FlashLoanFilter> for IVaultEvents {
        fn from(value: FlashLoanFilter) -> Self {
            Self::FlashLoanFilter(value)
        }
    }
    impl ::core::convert::From<InternalBalanceChangedFilter> for IVaultEvents {
        fn from(value: InternalBalanceChangedFilter) -> Self {
            Self::InternalBalanceChangedFilter(value)
        }
    }
    impl ::core::convert::From<PausedStateChangedFilter> for IVaultEvents {
        fn from(value: PausedStateChangedFilter) -> Self {
            Self::PausedStateChangedFilter(value)
        }
    }
    impl ::core::convert::From<PoolBalanceChangedFilter> for IVaultEvents {
        fn from(value: PoolBalanceChangedFilter) -> Self {
            Self::PoolBalanceChangedFilter(value)
        }
    }
    impl ::core::convert::From<PoolBalanceManagedFilter> for IVaultEvents {
        fn from(value: PoolBalanceManagedFilter) -> Self {
            Self::PoolBalanceManagedFilter(value)
        }
    }
    impl ::core::convert::From<PoolRegisteredFilter> for IVaultEvents {
        fn from(value: PoolRegisteredFilter) -> Self {
            Self::PoolRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<RelayerApprovalChangedFilter> for IVaultEvents {
        fn from(value: RelayerApprovalChangedFilter) -> Self {
            Self::RelayerApprovalChangedFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for IVaultEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<TokensDeregisteredFilter> for IVaultEvents {
        fn from(value: TokensDeregisteredFilter) -> Self {
            Self::TokensDeregisteredFilter(value)
        }
    }
    impl ::core::convert::From<TokensRegisteredFilter> for IVaultEvents {
        fn from(value: TokensRegisteredFilter) -> Self {
            Self::TokensRegisteredFilter(value)
        }
    }
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `batchSwap` function with signature `batchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool),int256[],uint256)` and selector `0x945bcec9`
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
        name = "batchSwap",
        abi = "batchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool),int256[],uint256)"
    )]
    pub struct BatchSwapCall {
        pub kind: u8,
        pub swaps: ::std::vec::Vec<BatchSwapStep>,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub funds: FundManagement,
        pub limits: ::std::vec::Vec<::ethers::core::types::I256>,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deregisterTokens` function with signature `deregisterTokens(bytes32,address[])` and selector `0x7d3aeb96`
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
    #[ethcall(name = "deregisterTokens", abi = "deregisterTokens(bytes32,address[])")]
    pub struct DeregisterTokensCall {
        pub pool_id: [u8; 32],
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `exitPool` function with signature `exitPool(bytes32,address,address,(address[],uint256[],bytes,bool))` and selector `0x8bdb3913`
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
        name = "exitPool",
        abi = "exitPool(bytes32,address,address,(address[],uint256[],bytes,bool))"
    )]
    pub struct ExitPoolCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub request: ExitPoolRequest,
    }
    ///Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address,address[],uint256[],bytes)` and selector `0x5c38449e`
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
    #[ethcall(name = "flashLoan", abi = "flashLoan(address,address[],uint256[],bytes)")]
    pub struct FlashLoanCall {
        pub recipient: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getActionId` function with signature `getActionId(bytes4)` and selector `0x851c1bb3`
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
    #[ethcall(name = "getActionId", abi = "getActionId(bytes4)")]
    pub struct GetActionIdCall {
        pub selector: [u8; 4],
    }
    ///Container type for all input parameters for the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
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
    #[ethcall(name = "getAuthorizer", abi = "getAuthorizer()")]
    pub struct GetAuthorizerCall;
    ///Container type for all input parameters for the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
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
    #[ethcall(name = "getDomainSeparator", abi = "getDomainSeparator()")]
    pub struct GetDomainSeparatorCall;
    ///Container type for all input parameters for the `getInternalBalance` function with signature `getInternalBalance(address,address[])` and selector `0x0f5a6efa`
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
        name = "getInternalBalance",
        abi = "getInternalBalance(address,address[])"
    )]
    pub struct GetInternalBalanceCall {
        pub user: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getNextNonce` function with signature `getNextNonce(address)` and selector `0x90193b7c`
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
    #[ethcall(name = "getNextNonce", abi = "getNextNonce(address)")]
    pub struct GetNextNonceCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPausedState` function with signature `getPausedState()` and selector `0x1c0de051`
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
    #[ethcall(name = "getPausedState", abi = "getPausedState()")]
    pub struct GetPausedStateCall;
    ///Container type for all input parameters for the `getPool` function with signature `getPool(bytes32)` and selector `0xf6c00927`
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
    #[ethcall(name = "getPool", abi = "getPool(bytes32)")]
    pub struct GetPoolCall {
        pub pool_id: [u8; 32],
    }
    ///Container type for all input parameters for the `getPoolTokenInfo` function with signature `getPoolTokenInfo(bytes32,address)` and selector `0xb05f8e48`
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
    #[ethcall(name = "getPoolTokenInfo", abi = "getPoolTokenInfo(bytes32,address)")]
    pub struct GetPoolTokenInfoCall {
        pub pool_id: [u8; 32],
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolTokens` function with signature `getPoolTokens(bytes32)` and selector `0xf94d4668`
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
    #[ethcall(name = "getPoolTokens", abi = "getPoolTokens(bytes32)")]
    pub struct GetPoolTokensCall {
        pub pool_id: [u8; 32],
    }
    ///Container type for all input parameters for the `getProtocolFeesCollector` function with signature `getProtocolFeesCollector()` and selector `0xd2946c2b`
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
    #[ethcall(name = "getProtocolFeesCollector", abi = "getProtocolFeesCollector()")]
    pub struct GetProtocolFeesCollectorCall;
    ///Container type for all input parameters for the `hasApprovedRelayer` function with signature `hasApprovedRelayer(address,address)` and selector `0xfec90d72`
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
    #[ethcall(name = "hasApprovedRelayer", abi = "hasApprovedRelayer(address,address)")]
    pub struct HasApprovedRelayerCall {
        pub user: ::ethers::core::types::Address,
        pub relayer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `joinPool` function with signature `joinPool(bytes32,address,address,(address[],uint256[],bytes,bool))` and selector `0xb95cac28`
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
        name = "joinPool",
        abi = "joinPool(bytes32,address,address,(address[],uint256[],bytes,bool))"
    )]
    pub struct JoinPoolCall {
        pub pool_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub request: JoinPoolRequest,
    }
    ///Container type for all input parameters for the `managePoolBalance` function with signature `managePoolBalance((uint8,bytes32,address,uint256)[])` and selector `0xe6c46092`
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
        name = "managePoolBalance",
        abi = "managePoolBalance((uint8,bytes32,address,uint256)[])"
    )]
    pub struct ManagePoolBalanceCall {
        pub ops: ::std::vec::Vec<PoolBalanceOp>,
    }
    ///Container type for all input parameters for the `manageUserBalance` function with signature `manageUserBalance((uint8,address,uint256,address,address)[])` and selector `0x0e8e3e84`
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
        name = "manageUserBalance",
        abi = "manageUserBalance((uint8,address,uint256,address,address)[])"
    )]
    pub struct ManageUserBalanceCall {
        pub ops: ::std::vec::Vec<UserBalanceOp>,
    }
    ///Container type for all input parameters for the `queryBatchSwap` function with signature `queryBatchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool))` and selector `0xf84d066e`
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
        name = "queryBatchSwap",
        abi = "queryBatchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool))"
    )]
    pub struct QueryBatchSwapCall {
        pub kind: u8,
        pub swaps: ::std::vec::Vec<BatchSwapStep>,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub funds: FundManagement,
    }
    ///Container type for all input parameters for the `registerPool` function with signature `registerPool(uint8)` and selector `0x09b2760f`
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
    #[ethcall(name = "registerPool", abi = "registerPool(uint8)")]
    pub struct RegisterPoolCall {
        pub specialization: u8,
    }
    ///Container type for all input parameters for the `registerTokens` function with signature `registerTokens(bytes32,address[],address[])` and selector `0x66a9c7d2`
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
        name = "registerTokens",
        abi = "registerTokens(bytes32,address[],address[])"
    )]
    pub struct RegisterTokensCall {
        pub pool_id: [u8; 32],
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub asset_managers: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `setAuthorizer` function with signature `setAuthorizer(address)` and selector `0x058a628f`
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
    #[ethcall(name = "setAuthorizer", abi = "setAuthorizer(address)")]
    pub struct SetAuthorizerCall {
        pub new_authorizer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPaused` function with signature `setPaused(bool)` and selector `0x16c38b3c`
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
    #[ethcall(name = "setPaused", abi = "setPaused(bool)")]
    pub struct SetPausedCall {
        pub paused: bool,
    }
    ///Container type for all input parameters for the `setRelayerApproval` function with signature `setRelayerApproval(address,address,bool)` and selector `0xfa6e671d`
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
        name = "setRelayerApproval",
        abi = "setRelayerApproval(address,address,bool)"
    )]
    pub struct SetRelayerApprovalCall {
        pub sender: ::ethers::core::types::Address,
        pub relayer: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap((bytes32,uint8,address,address,uint256,bytes),(address,bool,address,bool),uint256,uint256)` and selector `0x52bbbe29`
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
        name = "swap",
        abi = "swap((bytes32,uint8,address,address,uint256,bytes),(address,bool,address,bool),uint256,uint256)"
    )]
    pub struct SwapCall {
        pub single_swap: SingleSwap,
        pub funds: FundManagement,
        pub limit: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
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
    pub enum IVaultCalls {
        Weth(WethCall),
        BatchSwap(BatchSwapCall),
        DeregisterTokens(DeregisterTokensCall),
        ExitPool(ExitPoolCall),
        FlashLoan(FlashLoanCall),
        GetActionId(GetActionIdCall),
        GetAuthorizer(GetAuthorizerCall),
        GetDomainSeparator(GetDomainSeparatorCall),
        GetInternalBalance(GetInternalBalanceCall),
        GetNextNonce(GetNextNonceCall),
        GetPausedState(GetPausedStateCall),
        GetPool(GetPoolCall),
        GetPoolTokenInfo(GetPoolTokenInfoCall),
        GetPoolTokens(GetPoolTokensCall),
        GetProtocolFeesCollector(GetProtocolFeesCollectorCall),
        HasApprovedRelayer(HasApprovedRelayerCall),
        JoinPool(JoinPoolCall),
        ManagePoolBalance(ManagePoolBalanceCall),
        ManageUserBalance(ManageUserBalanceCall),
        QueryBatchSwap(QueryBatchSwapCall),
        RegisterPool(RegisterPoolCall),
        RegisterTokens(RegisterTokensCall),
        SetAuthorizer(SetAuthorizerCall),
        SetPaused(SetPausedCall),
        SetRelayerApproval(SetRelayerApprovalCall),
        Swap(SwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for IVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded) = <BatchSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchSwap(decoded));
            }
            if let Ok(decoded) = <DeregisterTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterTokens(decoded));
            }
            if let Ok(decoded) = <ExitPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExitPool(decoded));
            }
            if let Ok(decoded) = <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FlashLoan(decoded));
            }
            if let Ok(decoded) = <GetActionIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetActionId(decoded));
            }
            if let Ok(decoded) = <GetAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAuthorizer(decoded));
            }
            if let Ok(decoded) = <GetDomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDomainSeparator(decoded));
            }
            if let Ok(decoded) = <GetInternalBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInternalBalance(decoded));
            }
            if let Ok(decoded) = <GetNextNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextNonce(decoded));
            }
            if let Ok(decoded) = <GetPausedStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPausedState(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetPoolTokenInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolTokenInfo(decoded));
            }
            if let Ok(decoded) = <GetPoolTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolTokens(decoded));
            }
            if let Ok(decoded) = <GetProtocolFeesCollectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProtocolFeesCollector(decoded));
            }
            if let Ok(decoded) = <HasApprovedRelayerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasApprovedRelayer(decoded));
            }
            if let Ok(decoded) = <JoinPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::JoinPool(decoded));
            }
            if let Ok(decoded) = <ManagePoolBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ManagePoolBalance(decoded));
            }
            if let Ok(decoded) = <ManageUserBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ManageUserBalance(decoded));
            }
            if let Ok(decoded) = <QueryBatchSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QueryBatchSwap(decoded));
            }
            if let Ok(decoded) = <RegisterPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterPool(decoded));
            }
            if let Ok(decoded) = <RegisterTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterTokens(decoded));
            }
            if let Ok(decoded) = <SetAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAuthorizer(decoded));
            }
            if let Ok(decoded) = <SetPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPaused(decoded));
            }
            if let Ok(decoded) = <SetRelayerApprovalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRelayerApproval(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BatchSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExitPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActionId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAuthorizer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInternalBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPausedState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolTokenInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProtocolFeesCollector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasApprovedRelayer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::JoinPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ManagePoolBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ManageUserBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QueryBatchSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAuthorizer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRelayerApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActionId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAuthorizer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDomainSeparator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInternalBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNextNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPausedState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolTokenInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProtocolFeesCollector(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasApprovedRelayer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::JoinPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManagePoolBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManageUserBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryBatchSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorizer(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRelayerApproval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<WethCall> for IVaultCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<BatchSwapCall> for IVaultCalls {
        fn from(value: BatchSwapCall) -> Self {
            Self::BatchSwap(value)
        }
    }
    impl ::core::convert::From<DeregisterTokensCall> for IVaultCalls {
        fn from(value: DeregisterTokensCall) -> Self {
            Self::DeregisterTokens(value)
        }
    }
    impl ::core::convert::From<ExitPoolCall> for IVaultCalls {
        fn from(value: ExitPoolCall) -> Self {
            Self::ExitPool(value)
        }
    }
    impl ::core::convert::From<FlashLoanCall> for IVaultCalls {
        fn from(value: FlashLoanCall) -> Self {
            Self::FlashLoan(value)
        }
    }
    impl ::core::convert::From<GetActionIdCall> for IVaultCalls {
        fn from(value: GetActionIdCall) -> Self {
            Self::GetActionId(value)
        }
    }
    impl ::core::convert::From<GetAuthorizerCall> for IVaultCalls {
        fn from(value: GetAuthorizerCall) -> Self {
            Self::GetAuthorizer(value)
        }
    }
    impl ::core::convert::From<GetDomainSeparatorCall> for IVaultCalls {
        fn from(value: GetDomainSeparatorCall) -> Self {
            Self::GetDomainSeparator(value)
        }
    }
    impl ::core::convert::From<GetInternalBalanceCall> for IVaultCalls {
        fn from(value: GetInternalBalanceCall) -> Self {
            Self::GetInternalBalance(value)
        }
    }
    impl ::core::convert::From<GetNextNonceCall> for IVaultCalls {
        fn from(value: GetNextNonceCall) -> Self {
            Self::GetNextNonce(value)
        }
    }
    impl ::core::convert::From<GetPausedStateCall> for IVaultCalls {
        fn from(value: GetPausedStateCall) -> Self {
            Self::GetPausedState(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for IVaultCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetPoolTokenInfoCall> for IVaultCalls {
        fn from(value: GetPoolTokenInfoCall) -> Self {
            Self::GetPoolTokenInfo(value)
        }
    }
    impl ::core::convert::From<GetPoolTokensCall> for IVaultCalls {
        fn from(value: GetPoolTokensCall) -> Self {
            Self::GetPoolTokens(value)
        }
    }
    impl ::core::convert::From<GetProtocolFeesCollectorCall> for IVaultCalls {
        fn from(value: GetProtocolFeesCollectorCall) -> Self {
            Self::GetProtocolFeesCollector(value)
        }
    }
    impl ::core::convert::From<HasApprovedRelayerCall> for IVaultCalls {
        fn from(value: HasApprovedRelayerCall) -> Self {
            Self::HasApprovedRelayer(value)
        }
    }
    impl ::core::convert::From<JoinPoolCall> for IVaultCalls {
        fn from(value: JoinPoolCall) -> Self {
            Self::JoinPool(value)
        }
    }
    impl ::core::convert::From<ManagePoolBalanceCall> for IVaultCalls {
        fn from(value: ManagePoolBalanceCall) -> Self {
            Self::ManagePoolBalance(value)
        }
    }
    impl ::core::convert::From<ManageUserBalanceCall> for IVaultCalls {
        fn from(value: ManageUserBalanceCall) -> Self {
            Self::ManageUserBalance(value)
        }
    }
    impl ::core::convert::From<QueryBatchSwapCall> for IVaultCalls {
        fn from(value: QueryBatchSwapCall) -> Self {
            Self::QueryBatchSwap(value)
        }
    }
    impl ::core::convert::From<RegisterPoolCall> for IVaultCalls {
        fn from(value: RegisterPoolCall) -> Self {
            Self::RegisterPool(value)
        }
    }
    impl ::core::convert::From<RegisterTokensCall> for IVaultCalls {
        fn from(value: RegisterTokensCall) -> Self {
            Self::RegisterTokens(value)
        }
    }
    impl ::core::convert::From<SetAuthorizerCall> for IVaultCalls {
        fn from(value: SetAuthorizerCall) -> Self {
            Self::SetAuthorizer(value)
        }
    }
    impl ::core::convert::From<SetPausedCall> for IVaultCalls {
        fn from(value: SetPausedCall) -> Self {
            Self::SetPaused(value)
        }
    }
    impl ::core::convert::From<SetRelayerApprovalCall> for IVaultCalls {
        fn from(value: SetRelayerApprovalCall) -> Self {
            Self::SetRelayerApproval(value)
        }
    }
    impl ::core::convert::From<SwapCall> for IVaultCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `batchSwap` function with signature `batchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool),int256[],uint256)` and selector `0x945bcec9`
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
    pub struct BatchSwapReturn(pub ::std::vec::Vec<::ethers::core::types::I256>);
    ///Container type for all return fields from the `getActionId` function with signature `getActionId(bytes4)` and selector `0x851c1bb3`
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
    pub struct GetActionIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getAuthorizer` function with signature `getAuthorizer()` and selector `0xaaabadc5`
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
    pub struct GetAuthorizerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
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
    pub struct GetDomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getInternalBalance` function with signature `getInternalBalance(address,address[])` and selector `0x0f5a6efa`
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
    pub struct GetInternalBalanceReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `getNextNonce` function with signature `getNextNonce(address)` and selector `0x90193b7c`
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
    pub struct GetNextNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPausedState` function with signature `getPausedState()` and selector `0x1c0de051`
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
    pub struct GetPausedStateReturn {
        pub paused: bool,
        pub pause_window_end_time: ::ethers::core::types::U256,
        pub buffer_period_end_time: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getPool` function with signature `getPool(bytes32)` and selector `0xf6c00927`
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
    pub struct GetPoolReturn(pub ::ethers::core::types::Address, pub u8);
    ///Container type for all return fields from the `getPoolTokenInfo` function with signature `getPoolTokenInfo(bytes32,address)` and selector `0xb05f8e48`
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
    pub struct GetPoolTokenInfoReturn {
        pub cash: ::ethers::core::types::U256,
        pub managed: ::ethers::core::types::U256,
        pub last_change_block: ::ethers::core::types::U256,
        pub asset_manager: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getPoolTokens` function with signature `getPoolTokens(bytes32)` and selector `0xf94d4668`
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
    pub struct GetPoolTokensReturn {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub last_change_block: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getProtocolFeesCollector` function with signature `getProtocolFeesCollector()` and selector `0xd2946c2b`
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
    pub struct GetProtocolFeesCollectorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasApprovedRelayer` function with signature `hasApprovedRelayer(address,address)` and selector `0xfec90d72`
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
    pub struct HasApprovedRelayerReturn(pub bool);
    ///Container type for all return fields from the `queryBatchSwap` function with signature `queryBatchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool))` and selector `0xf84d066e`
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
    pub struct QueryBatchSwapReturn {
        pub asset_deltas: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///Container type for all return fields from the `registerPool` function with signature `registerPool(uint8)` and selector `0x09b2760f`
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
    pub struct RegisterPoolReturn(pub [u8; 32]);
    ///Container type for all return fields from the `swap` function with signature `swap((bytes32,uint8,address,address,uint256,bytes),(address,bool,address,bool),uint256,uint256)` and selector `0x52bbbe29`
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
    pub struct SwapReturn(pub ::ethers::core::types::U256);
    ///`BatchSwapStep(bytes32,uint256,uint256,uint256,bytes)`
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
    pub struct BatchSwapStep {
        pub pool_id: [u8; 32],
        pub asset_in_index: ::ethers::core::types::U256,
        pub asset_out_index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///`ExitPoolRequest(address[],uint256[],bytes,bool)`
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
    pub struct ExitPoolRequest {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub min_amounts_out: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
        pub to_internal_balance: bool,
    }
    ///`FundManagement(address,bool,address,bool)`
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
    pub struct FundManagement {
        pub sender: ::ethers::core::types::Address,
        pub from_internal_balance: bool,
        pub recipient: ::ethers::core::types::Address,
        pub to_internal_balance: bool,
    }
    ///`JoinPoolRequest(address[],uint256[],bytes,bool)`
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
    pub struct JoinPoolRequest {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub max_amounts_in: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
        pub from_internal_balance: bool,
    }
    ///`PoolBalanceOp(uint8,bytes32,address,uint256)`
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
    pub struct PoolBalanceOp {
        pub kind: u8,
        pub pool_id: [u8; 32],
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///`SingleSwap(bytes32,uint8,address,address,uint256,bytes)`
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
    pub struct SingleSwap {
        pub pool_id: [u8; 32],
        pub kind: u8,
        pub asset_in: ::ethers::core::types::Address,
        pub asset_out: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///`UserBalanceOp(uint8,address,uint256,address,address)`
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
    pub struct UserBalanceOp {
        pub kind: u8,
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
    }
}
