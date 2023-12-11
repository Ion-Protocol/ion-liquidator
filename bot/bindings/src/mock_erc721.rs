pub use mock_erc721::*;
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
pub mod mock_erc721 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
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
    pub static MOCKERC721_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x1F\x1D\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80ccR!\x1E\x11a\0\x8CW\x80c\xA2,\xB4e\x11a\0fW\x80c\xA2,\xB4e\x14a\x02[W\x80c\xB8\x8DO\xDE\x14a\x02wW\x80c\xC8{V\xDD\x14a\x02\x93W\x80c\xE9\x85\xE9\xC5\x14a\x02\xC3Wa\0\xEAV[\x80ccR!\x1E\x14a\x01\xDDW\x80cp\xA0\x821\x14a\x02\rW\x80c\x95\xD8\x9BA\x14a\x02=Wa\0\xEAV[\x80c\t^\xA7\xB3\x11a\0\xC8W\x80c\t^\xA7\xB3\x14a\x01mW\x80c#\xB8r\xDD\x14a\x01\x89W\x80cB\x84.\x0E\x14a\x01\xA5W\x80cL\xD8\x8Bv\x14a\x01\xC1Wa\0\xEAV[\x80c\x01\xFF\xC9\xA7\x14a\0\xEFW\x80c\x06\xFD\xDE\x03\x14a\x01\x1FW\x80c\x08\x18\x12\xFC\x14a\x01=W[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x10\xBBV[a\x02\xF3V[`@Qa\x01\x16\x91\x90a\x11\x03V[`@Q\x80\x91\x03\x90\xF3[a\x01'a\x03\x85V[`@Qa\x014\x91\x90a\x11\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x01W`\x04\x806\x03\x81\x01\x90a\x01R\x91\x90a\x12\x06V[a\x04\x13V[`@Qa\x01d\x91\x90a\x12tV[`@Q\x80\x91\x03\x90\xF3[a\x01\x87`\x04\x806\x03\x81\x01\x90a\x01\x82\x91\x90a\x12\xBBV[a\x04FV[\0[a\x01\xA3`\x04\x806\x03\x81\x01\x90a\x01\x9E\x91\x90a\x12\xFBV[a\x06/V[\0[a\x01\xBF`\x04\x806\x03\x81\x01\x90a\x01\xBA\x91\x90a\x12\xFBV[a\n9V[\0[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a\x14\x83V[a\x0B`V[\0[a\x01\xF7`\x04\x806\x03\x81\x01\x90a\x01\xF2\x91\x90a\x12\x06V[a\x0B\xEFV[`@Qa\x02\x04\x91\x90a\x12tV[`@Q\x80\x91\x03\x90\xF3[a\x02'`\x04\x806\x03\x81\x01\x90a\x02\"\x91\x90a\x14\xFBV[a\x0C\x9AV[`@Qa\x024\x91\x90a\x157V[`@Q\x80\x91\x03\x90\xF3[a\x02Ea\rQV[`@Qa\x02R\x91\x90a\x11\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x02u`\x04\x806\x03\x81\x01\x90a\x02p\x91\x90a\x15~V[a\r\xDFV[\0[a\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a\x16_V[a\x0E\xDCV[\0[a\x02\xAD`\x04\x806\x03\x81\x01\x90a\x02\xA8\x91\x90a\x12\x06V[a\x10\x06V[`@Qa\x02\xBA\x91\x90a\x11\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a\x16\xE2V[a\x10\rV[`@Qa\x02\xEA\x91\x90a\x11\x03V[`@Q\x80\x91\x03\x90\xF3[`\0c\x01\xFF\xC9\xA7`\xE0\x1B\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03NWPc\x80\xACX\xCD`\xE0\x1B\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[\x80a\x03~WPc[^\x13\x9F`\xE0\x1B\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[\x90P\x91\x90PV[`\0\x80Ta\x03\x92\x90a\x17QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xBE\x90a\x17QV[\x80\x15a\x04\x0BW\x80`\x1F\x10a\x03\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x0BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xEEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x04` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x05>WP`\x05`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[a\x05}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05t\x90a\x17\xCEV[`@Q\x80\x91\x03\x90\xFD[\x82`\x04`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPPV[`\x02`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xD0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xC7\x90a\x18:V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x07?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x076\x90a\x18\xA6V[`@Q\x80\x91\x03\x90\xFD[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x07\xFFWP`\x05`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[\x80a\x08hWP`\x04`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x08\xA7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x9E\x90a\x17\xCEV[`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81T\x80\x92\x91\x90a\x08\xF7\x90a\x18\xF5V[\x91\x90PUP`\x03`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81T\x80\x92\x91\x90a\tL\x90a\x19\x1EV[\x91\x90PUP\x81`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x04`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[a\nD\x83\x83\x83a\x06/V[a\nM\x82a\x10<V[\x15\x80a\x0B\x1CWPc\x15\x0Bz\x02`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x15\x0Bz\x023\x86\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xB8\x93\x92\x91\x90a\x19\x9DV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xFB\x91\x90a\x19\xFCV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[a\x0B[W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BR\x90a\x1AuV[`@Q\x80\x91\x03\x90\xFD[PPPV[`\x06`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0B\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xA7\x90a\x1A\xE1V[`@Q\x80\x91\x03\x90\xFD[\x81`\0\x90\x81a\x0B\xBF\x91\x90a\x1C\xADV[P\x80`\x01\x90\x81a\x0B\xCF\x91\x90a\x1C\xADV[P`\x01`\x06`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPV[`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x8C\x90a\x1D\xCBV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\r\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x01\x90a\x1E7V[`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x91\x90PV[`\x01\x80Ta\r^\x90a\x17QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\x8A\x90a\x17QV[\x80\x15a\r\xD7W\x80`\x1F\x10a\r\xACWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xD7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xBAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[\x80`\x05`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x83`@Qa\x0E\xD0\x91\x90a\x11\x03V[`@Q\x80\x91\x03\x90\xA3PPV[a\x0E\xE7\x84\x84\x84a\x06/V[a\x0E\xF0\x83a\x10<V[\x15\x80a\x0F\xC1WPc\x15\x0Bz\x02`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x15\x0Bz\x023\x87\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F]\x94\x93\x92\x91\x90a\x1E\x9BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA0\x91\x90a\x19\xFCV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[a\x10\0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xF7\x90a\x1AuV[`@Q\x80\x91\x03\x90\xFD[PPPPV[``\x91\x90PV[`\x05` R\x81`\0R`@`\0 ` R\x80`\0R`@`\0 `\0\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80\x82;\x90P`\0\x81\x11\x91PP\x91\x90PV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x10\x98\x81a\x10cV[\x81\x14a\x10\xA3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x10\xB5\x81a\x10\x8FV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10\xD1Wa\x10\xD0a\x10YV[[`\0a\x10\xDF\x84\x82\x85\x01a\x10\xA6V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x10\xFD\x81a\x10\xE8V[\x82RPPV[`\0` \x82\x01\x90Pa\x11\x18`\0\x83\x01\x84a\x10\xF4V[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11XW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x11=V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x11\x80\x82a\x11\x1EV[a\x11\x8A\x81\x85a\x11)V[\x93Pa\x11\x9A\x81\x85` \x86\x01a\x11:V[a\x11\xA3\x81a\x11dV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11\xC8\x81\x84a\x11uV[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x11\xE3\x81a\x11\xD0V[\x81\x14a\x11\xEEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\0\x81a\x11\xDAV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\x1CWa\x12\x1Ba\x10YV[[`\0a\x12*\x84\x82\x85\x01a\x11\xF1V[\x91PP\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x12^\x82a\x123V[\x90P\x91\x90PV[a\x12n\x81a\x12SV[\x82RPPV[`\0` \x82\x01\x90Pa\x12\x89`\0\x83\x01\x84a\x12eV[\x92\x91PPV[a\x12\x98\x81a\x12SV[\x81\x14a\x12\xA3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\xB5\x81a\x12\x8FV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\xD2Wa\x12\xD1a\x10YV[[`\0a\x12\xE0\x85\x82\x86\x01a\x12\xA6V[\x92PP` a\x12\xF1\x85\x82\x86\x01a\x11\xF1V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x14Wa\x13\x13a\x10YV[[`\0a\x13\"\x86\x82\x87\x01a\x12\xA6V[\x93PP` a\x133\x86\x82\x87\x01a\x12\xA6V[\x92PP`@a\x13D\x86\x82\x87\x01a\x11\xF1V[\x91PP\x92P\x92P\x92V[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x13\x90\x82a\x11dV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13\xAFWa\x13\xAEa\x13XV[[\x80`@RPPPV[`\0a\x13\xC2a\x10OV[\x90Pa\x13\xCE\x82\x82a\x13\x87V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13\xEEWa\x13\xEDa\x13XV[[a\x13\xF7\x82a\x11dV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x14&a\x14!\x84a\x13\xD3V[a\x13\xB8V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x14BWa\x14Aa\x13SV[[a\x14M\x84\x82\x85a\x14\x04V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14jWa\x14ia\x13NV[[\x815a\x14z\x84\x82` \x86\x01a\x14\x13V[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x9AWa\x14\x99a\x10YV[[`\0\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xB8Wa\x14\xB7a\x10^V[[a\x14\xC4\x85\x82\x86\x01a\x14UV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE5Wa\x14\xE4a\x10^V[[a\x14\xF1\x85\x82\x86\x01a\x14UV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\x11Wa\x15\x10a\x10YV[[`\0a\x15\x1F\x84\x82\x85\x01a\x12\xA6V[\x91PP\x92\x91PPV[a\x151\x81a\x11\xD0V[\x82RPPV[`\0` \x82\x01\x90Pa\x15L`\0\x83\x01\x84a\x15(V[\x92\x91PPV[a\x15[\x81a\x10\xE8V[\x81\x14a\x15fW`\0\x80\xFD[PV[`\0\x815\x90Pa\x15x\x81a\x15RV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x95Wa\x15\x94a\x10YV[[`\0a\x15\xA3\x85\x82\x86\x01a\x12\xA6V[\x92PP` a\x15\xB4\x85\x82\x86\x01a\x15iV[\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\xD9Wa\x15\xD8a\x13XV[[a\x15\xE2\x82a\x11dV[\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x16\x02a\x15\xFD\x84a\x15\xBEV[a\x13\xB8V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x16\x1EWa\x16\x1Da\x13SV[[a\x16)\x84\x82\x85a\x14\x04V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x16FWa\x16Ea\x13NV[[\x815a\x16V\x84\x82` \x86\x01a\x15\xEFV[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x16yWa\x16xa\x10YV[[`\0a\x16\x87\x87\x82\x88\x01a\x12\xA6V[\x94PP` a\x16\x98\x87\x82\x88\x01a\x12\xA6V[\x93PP`@a\x16\xA9\x87\x82\x88\x01a\x11\xF1V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xCAWa\x16\xC9a\x10^V[[a\x16\xD6\x87\x82\x88\x01a\x161V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xF9Wa\x16\xF8a\x10YV[[`\0a\x17\x07\x85\x82\x86\x01a\x12\xA6V[\x92PP` a\x17\x18\x85\x82\x86\x01a\x12\xA6V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x17iW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x17|Wa\x17{a\x17\"V[[P\x91\x90PV[\x7FNOT_AUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x17\xB8`\x0E\x83a\x11)V[\x91Pa\x17\xC3\x82a\x17\x82V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x17\xE7\x81a\x17\xABV[\x90P\x91\x90PV[\x7FWRONG_FROM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x18$`\n\x83a\x11)V[\x91Pa\x18/\x82a\x17\xEEV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18S\x81a\x18\x17V[\x90P\x91\x90PV[\x7FINVALID_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x18\x90`\x11\x83a\x11)V[\x91Pa\x18\x9B\x82a\x18ZV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18\xBF\x81a\x18\x83V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x19\0\x82a\x11\xD0V[\x91P`\0\x82\x03a\x19\x13Wa\x19\x12a\x18\xC6V[[`\x01\x82\x03\x90P\x91\x90PV[`\0a\x19)\x82a\x11\xD0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x19[Wa\x19Za\x18\xC6V[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[PV[`\0a\x19\x87`\0\x83a\x19fV[\x91Pa\x19\x92\x82a\x19wV[`\0\x82\x01\x90P\x91\x90PV[`\0`\x80\x82\x01\x90Pa\x19\xB2`\0\x83\x01\x86a\x12eV[a\x19\xBF` \x83\x01\x85a\x12eV[a\x19\xCC`@\x83\x01\x84a\x15(V[\x81\x81\x03``\x83\x01Ra\x19\xDD\x81a\x19zV[\x90P\x94\x93PPPPV[`\0\x81Q\x90Pa\x19\xF6\x81a\x10\x8FV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A\x12Wa\x1A\x11a\x10YV[[`\0a\x1A \x84\x82\x85\x01a\x19\xE7V[\x91PP\x92\x91PPV[\x7FUNSAFE_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A_`\x10\x83a\x11)V[\x91Pa\x1Aj\x82a\x1A)V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\x8E\x81a\x1ARV[\x90P\x91\x90PV[\x7FALREADY_INITIALIZED\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A\xCB`\x13\x83a\x11)V[\x91Pa\x1A\xD6\x82a\x1A\x95V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\xFA\x81a\x1A\xBEV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x1Bc\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x1B&V[a\x1Bm\x86\x83a\x1B&V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x1B\xAAa\x1B\xA5a\x1B\xA0\x84a\x11\xD0V[a\x1B\x85V[a\x11\xD0V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x1B\xC4\x83a\x1B\x8FV[a\x1B\xD8a\x1B\xD0\x82a\x1B\xB1V[\x84\x84Ta\x1B3V[\x82UPPPPV[`\0\x90V[a\x1B\xEDa\x1B\xE0V[a\x1B\xF8\x81\x84\x84a\x1B\xBBV[PPPV[[\x81\x81\x10\x15a\x1C\x1CWa\x1C\x11`\0\x82a\x1B\xE5V[`\x01\x81\x01\x90Pa\x1B\xFEV[PPV[`\x1F\x82\x11\x15a\x1CaWa\x1C2\x81a\x1B\x01V[a\x1C;\x84a\x1B\x16V[\x81\x01` \x85\x10\x15a\x1CJW\x81\x90P[a\x1C^a\x1CV\x85a\x1B\x16V[\x83\x01\x82a\x1B\xFDV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x1C\x84`\0\x19\x84`\x08\x02a\x1CfV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x1C\x9D\x83\x83a\x1CsV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x1C\xB6\x82a\x11\x1EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xCFWa\x1C\xCEa\x13XV[[a\x1C\xD9\x82Ta\x17QV[a\x1C\xE4\x82\x82\x85a\x1C V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x1D\x17W`\0\x84\x15a\x1D\x05W\x82\x87\x01Q\x90P[a\x1D\x0F\x85\x82a\x1C\x91V[\x86UPa\x1DwV[`\x1F\x19\x84\x16a\x1D%\x86a\x1B\x01V[`\0[\x82\x81\x10\x15a\x1DMW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x1D(V[\x86\x83\x10\x15a\x1DjW\x84\x89\x01Qa\x1Df`\x1F\x89\x16\x82a\x1CsV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FNOT_MINTED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1D\xB5`\n\x83a\x11)V[\x91Pa\x1D\xC0\x82a\x1D\x7FV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1D\xE4\x81a\x1D\xA8V[\x90P\x91\x90PV[\x7FZERO_ADDRESS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1E!`\x0C\x83a\x11)V[\x91Pa\x1E,\x82a\x1D\xEBV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1EP\x81a\x1E\x14V[\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0a\x1Em\x82a\x1EWV[a\x1Ew\x81\x85a\x19fV[\x93Pa\x1E\x87\x81\x85` \x86\x01a\x11:V[a\x1E\x90\x81a\x11dV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x1E\xB0`\0\x83\x01\x87a\x12eV[a\x1E\xBD` \x83\x01\x86a\x12eV[a\x1E\xCA`@\x83\x01\x85a\x15(V[\x81\x81\x03``\x83\x01Ra\x1E\xDC\x81\x84a\x1EbV[\x90P\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \rRW\xB6\x93\xBC\x05\xE2}\x91A\nb\xF22\x92\xA9\xA3\x93\xB64b6\x01TS\xD7\xB9^kf[dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKERC721_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80ccR!\x1E\x11a\0\x8CW\x80c\xA2,\xB4e\x11a\0fW\x80c\xA2,\xB4e\x14a\x02[W\x80c\xB8\x8DO\xDE\x14a\x02wW\x80c\xC8{V\xDD\x14a\x02\x93W\x80c\xE9\x85\xE9\xC5\x14a\x02\xC3Wa\0\xEAV[\x80ccR!\x1E\x14a\x01\xDDW\x80cp\xA0\x821\x14a\x02\rW\x80c\x95\xD8\x9BA\x14a\x02=Wa\0\xEAV[\x80c\t^\xA7\xB3\x11a\0\xC8W\x80c\t^\xA7\xB3\x14a\x01mW\x80c#\xB8r\xDD\x14a\x01\x89W\x80cB\x84.\x0E\x14a\x01\xA5W\x80cL\xD8\x8Bv\x14a\x01\xC1Wa\0\xEAV[\x80c\x01\xFF\xC9\xA7\x14a\0\xEFW\x80c\x06\xFD\xDE\x03\x14a\x01\x1FW\x80c\x08\x18\x12\xFC\x14a\x01=W[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x10\xBBV[a\x02\xF3V[`@Qa\x01\x16\x91\x90a\x11\x03V[`@Q\x80\x91\x03\x90\xF3[a\x01'a\x03\x85V[`@Qa\x014\x91\x90a\x11\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x01W`\x04\x806\x03\x81\x01\x90a\x01R\x91\x90a\x12\x06V[a\x04\x13V[`@Qa\x01d\x91\x90a\x12tV[`@Q\x80\x91\x03\x90\xF3[a\x01\x87`\x04\x806\x03\x81\x01\x90a\x01\x82\x91\x90a\x12\xBBV[a\x04FV[\0[a\x01\xA3`\x04\x806\x03\x81\x01\x90a\x01\x9E\x91\x90a\x12\xFBV[a\x06/V[\0[a\x01\xBF`\x04\x806\x03\x81\x01\x90a\x01\xBA\x91\x90a\x12\xFBV[a\n9V[\0[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a\x14\x83V[a\x0B`V[\0[a\x01\xF7`\x04\x806\x03\x81\x01\x90a\x01\xF2\x91\x90a\x12\x06V[a\x0B\xEFV[`@Qa\x02\x04\x91\x90a\x12tV[`@Q\x80\x91\x03\x90\xF3[a\x02'`\x04\x806\x03\x81\x01\x90a\x02\"\x91\x90a\x14\xFBV[a\x0C\x9AV[`@Qa\x024\x91\x90a\x157V[`@Q\x80\x91\x03\x90\xF3[a\x02Ea\rQV[`@Qa\x02R\x91\x90a\x11\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x02u`\x04\x806\x03\x81\x01\x90a\x02p\x91\x90a\x15~V[a\r\xDFV[\0[a\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a\x16_V[a\x0E\xDCV[\0[a\x02\xAD`\x04\x806\x03\x81\x01\x90a\x02\xA8\x91\x90a\x12\x06V[a\x10\x06V[`@Qa\x02\xBA\x91\x90a\x11\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a\x16\xE2V[a\x10\rV[`@Qa\x02\xEA\x91\x90a\x11\x03V[`@Q\x80\x91\x03\x90\xF3[`\0c\x01\xFF\xC9\xA7`\xE0\x1B\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03NWPc\x80\xACX\xCD`\xE0\x1B\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[\x80a\x03~WPc[^\x13\x9F`\xE0\x1B\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[\x90P\x91\x90PV[`\0\x80Ta\x03\x92\x90a\x17QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xBE\x90a\x17QV[\x80\x15a\x04\x0BW\x80`\x1F\x10a\x03\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x0BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xEEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x04` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x05>WP`\x05`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[a\x05}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05t\x90a\x17\xCEV[`@Q\x80\x91\x03\x90\xFD[\x82`\x04`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPPV[`\x02`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xD0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xC7\x90a\x18:V[`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x07?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x076\x90a\x18\xA6V[`@Q\x80\x91\x03\x90\xFD[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x07\xFFWP`\x05`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[\x80a\x08hWP`\x04`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x08\xA7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x9E\x90a\x17\xCEV[`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81T\x80\x92\x91\x90a\x08\xF7\x90a\x18\xF5V[\x91\x90PUP`\x03`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81T\x80\x92\x91\x90a\tL\x90a\x19\x1EV[\x91\x90PUP\x81`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x04`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[a\nD\x83\x83\x83a\x06/V[a\nM\x82a\x10<V[\x15\x80a\x0B\x1CWPc\x15\x0Bz\x02`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x15\x0Bz\x023\x86\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xB8\x93\x92\x91\x90a\x19\x9DV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xFB\x91\x90a\x19\xFCV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[a\x0B[W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BR\x90a\x1AuV[`@Q\x80\x91\x03\x90\xFD[PPPV[`\x06`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0B\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xA7\x90a\x1A\xE1V[`@Q\x80\x91\x03\x90\xFD[\x81`\0\x90\x81a\x0B\xBF\x91\x90a\x1C\xADV[P\x80`\x01\x90\x81a\x0B\xCF\x91\x90a\x1C\xADV[P`\x01`\x06`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPV[`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x8C\x90a\x1D\xCBV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\0\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\r\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x01\x90a\x1E7V[`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x91\x90PV[`\x01\x80Ta\r^\x90a\x17QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\x8A\x90a\x17QV[\x80\x15a\r\xD7W\x80`\x1F\x10a\r\xACWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xD7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xBAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[\x80`\x05`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x83`@Qa\x0E\xD0\x91\x90a\x11\x03V[`@Q\x80\x91\x03\x90\xA3PPV[a\x0E\xE7\x84\x84\x84a\x06/V[a\x0E\xF0\x83a\x10<V[\x15\x80a\x0F\xC1WPc\x15\x0Bz\x02`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x15\x0Bz\x023\x87\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F]\x94\x93\x92\x91\x90a\x1E\x9BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA0\x91\x90a\x19\xFCV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14[a\x10\0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xF7\x90a\x1AuV[`@Q\x80\x91\x03\x90\xFD[PPPPV[``\x91\x90PV[`\x05` R\x81`\0R`@`\0 ` R\x80`\0R`@`\0 `\0\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80\x82;\x90P`\0\x81\x11\x91PP\x91\x90PV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x10\x98\x81a\x10cV[\x81\x14a\x10\xA3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x10\xB5\x81a\x10\x8FV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10\xD1Wa\x10\xD0a\x10YV[[`\0a\x10\xDF\x84\x82\x85\x01a\x10\xA6V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x10\xFD\x81a\x10\xE8V[\x82RPPV[`\0` \x82\x01\x90Pa\x11\x18`\0\x83\x01\x84a\x10\xF4V[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x11XW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x11=V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x11\x80\x82a\x11\x1EV[a\x11\x8A\x81\x85a\x11)V[\x93Pa\x11\x9A\x81\x85` \x86\x01a\x11:V[a\x11\xA3\x81a\x11dV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11\xC8\x81\x84a\x11uV[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x11\xE3\x81a\x11\xD0V[\x81\x14a\x11\xEEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\0\x81a\x11\xDAV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\x1CWa\x12\x1Ba\x10YV[[`\0a\x12*\x84\x82\x85\x01a\x11\xF1V[\x91PP\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x12^\x82a\x123V[\x90P\x91\x90PV[a\x12n\x81a\x12SV[\x82RPPV[`\0` \x82\x01\x90Pa\x12\x89`\0\x83\x01\x84a\x12eV[\x92\x91PPV[a\x12\x98\x81a\x12SV[\x81\x14a\x12\xA3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\xB5\x81a\x12\x8FV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\xD2Wa\x12\xD1a\x10YV[[`\0a\x12\xE0\x85\x82\x86\x01a\x12\xA6V[\x92PP` a\x12\xF1\x85\x82\x86\x01a\x11\xF1V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x14Wa\x13\x13a\x10YV[[`\0a\x13\"\x86\x82\x87\x01a\x12\xA6V[\x93PP` a\x133\x86\x82\x87\x01a\x12\xA6V[\x92PP`@a\x13D\x86\x82\x87\x01a\x11\xF1V[\x91PP\x92P\x92P\x92V[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x13\x90\x82a\x11dV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13\xAFWa\x13\xAEa\x13XV[[\x80`@RPPPV[`\0a\x13\xC2a\x10OV[\x90Pa\x13\xCE\x82\x82a\x13\x87V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13\xEEWa\x13\xEDa\x13XV[[a\x13\xF7\x82a\x11dV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x14&a\x14!\x84a\x13\xD3V[a\x13\xB8V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x14BWa\x14Aa\x13SV[[a\x14M\x84\x82\x85a\x14\x04V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14jWa\x14ia\x13NV[[\x815a\x14z\x84\x82` \x86\x01a\x14\x13V[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x9AWa\x14\x99a\x10YV[[`\0\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xB8Wa\x14\xB7a\x10^V[[a\x14\xC4\x85\x82\x86\x01a\x14UV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE5Wa\x14\xE4a\x10^V[[a\x14\xF1\x85\x82\x86\x01a\x14UV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\x11Wa\x15\x10a\x10YV[[`\0a\x15\x1F\x84\x82\x85\x01a\x12\xA6V[\x91PP\x92\x91PPV[a\x151\x81a\x11\xD0V[\x82RPPV[`\0` \x82\x01\x90Pa\x15L`\0\x83\x01\x84a\x15(V[\x92\x91PPV[a\x15[\x81a\x10\xE8V[\x81\x14a\x15fW`\0\x80\xFD[PV[`\0\x815\x90Pa\x15x\x81a\x15RV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x95Wa\x15\x94a\x10YV[[`\0a\x15\xA3\x85\x82\x86\x01a\x12\xA6V[\x92PP` a\x15\xB4\x85\x82\x86\x01a\x15iV[\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\xD9Wa\x15\xD8a\x13XV[[a\x15\xE2\x82a\x11dV[\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x16\x02a\x15\xFD\x84a\x15\xBEV[a\x13\xB8V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x16\x1EWa\x16\x1Da\x13SV[[a\x16)\x84\x82\x85a\x14\x04V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x16FWa\x16Ea\x13NV[[\x815a\x16V\x84\x82` \x86\x01a\x15\xEFV[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x16yWa\x16xa\x10YV[[`\0a\x16\x87\x87\x82\x88\x01a\x12\xA6V[\x94PP` a\x16\x98\x87\x82\x88\x01a\x12\xA6V[\x93PP`@a\x16\xA9\x87\x82\x88\x01a\x11\xF1V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xCAWa\x16\xC9a\x10^V[[a\x16\xD6\x87\x82\x88\x01a\x161V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xF9Wa\x16\xF8a\x10YV[[`\0a\x17\x07\x85\x82\x86\x01a\x12\xA6V[\x92PP` a\x17\x18\x85\x82\x86\x01a\x12\xA6V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x17iW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x17|Wa\x17{a\x17\"V[[P\x91\x90PV[\x7FNOT_AUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x17\xB8`\x0E\x83a\x11)V[\x91Pa\x17\xC3\x82a\x17\x82V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x17\xE7\x81a\x17\xABV[\x90P\x91\x90PV[\x7FWRONG_FROM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x18$`\n\x83a\x11)V[\x91Pa\x18/\x82a\x17\xEEV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18S\x81a\x18\x17V[\x90P\x91\x90PV[\x7FINVALID_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x18\x90`\x11\x83a\x11)V[\x91Pa\x18\x9B\x82a\x18ZV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18\xBF\x81a\x18\x83V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x19\0\x82a\x11\xD0V[\x91P`\0\x82\x03a\x19\x13Wa\x19\x12a\x18\xC6V[[`\x01\x82\x03\x90P\x91\x90PV[`\0a\x19)\x82a\x11\xD0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x19[Wa\x19Za\x18\xC6V[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[PV[`\0a\x19\x87`\0\x83a\x19fV[\x91Pa\x19\x92\x82a\x19wV[`\0\x82\x01\x90P\x91\x90PV[`\0`\x80\x82\x01\x90Pa\x19\xB2`\0\x83\x01\x86a\x12eV[a\x19\xBF` \x83\x01\x85a\x12eV[a\x19\xCC`@\x83\x01\x84a\x15(V[\x81\x81\x03``\x83\x01Ra\x19\xDD\x81a\x19zV[\x90P\x94\x93PPPPV[`\0\x81Q\x90Pa\x19\xF6\x81a\x10\x8FV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A\x12Wa\x1A\x11a\x10YV[[`\0a\x1A \x84\x82\x85\x01a\x19\xE7V[\x91PP\x92\x91PPV[\x7FUNSAFE_RECIPIENT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A_`\x10\x83a\x11)V[\x91Pa\x1Aj\x82a\x1A)V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\x8E\x81a\x1ARV[\x90P\x91\x90PV[\x7FALREADY_INITIALIZED\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A\xCB`\x13\x83a\x11)V[\x91Pa\x1A\xD6\x82a\x1A\x95V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\xFA\x81a\x1A\xBEV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x1Bc\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x1B&V[a\x1Bm\x86\x83a\x1B&V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x1B\xAAa\x1B\xA5a\x1B\xA0\x84a\x11\xD0V[a\x1B\x85V[a\x11\xD0V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x1B\xC4\x83a\x1B\x8FV[a\x1B\xD8a\x1B\xD0\x82a\x1B\xB1V[\x84\x84Ta\x1B3V[\x82UPPPPV[`\0\x90V[a\x1B\xEDa\x1B\xE0V[a\x1B\xF8\x81\x84\x84a\x1B\xBBV[PPPV[[\x81\x81\x10\x15a\x1C\x1CWa\x1C\x11`\0\x82a\x1B\xE5V[`\x01\x81\x01\x90Pa\x1B\xFEV[PPV[`\x1F\x82\x11\x15a\x1CaWa\x1C2\x81a\x1B\x01V[a\x1C;\x84a\x1B\x16V[\x81\x01` \x85\x10\x15a\x1CJW\x81\x90P[a\x1C^a\x1CV\x85a\x1B\x16V[\x83\x01\x82a\x1B\xFDV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x1C\x84`\0\x19\x84`\x08\x02a\x1CfV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x1C\x9D\x83\x83a\x1CsV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x1C\xB6\x82a\x11\x1EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xCFWa\x1C\xCEa\x13XV[[a\x1C\xD9\x82Ta\x17QV[a\x1C\xE4\x82\x82\x85a\x1C V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x1D\x17W`\0\x84\x15a\x1D\x05W\x82\x87\x01Q\x90P[a\x1D\x0F\x85\x82a\x1C\x91V[\x86UPa\x1DwV[`\x1F\x19\x84\x16a\x1D%\x86a\x1B\x01V[`\0[\x82\x81\x10\x15a\x1DMW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x1D(V[\x86\x83\x10\x15a\x1DjW\x84\x89\x01Qa\x1Df`\x1F\x89\x16\x82a\x1CsV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FNOT_MINTED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1D\xB5`\n\x83a\x11)V[\x91Pa\x1D\xC0\x82a\x1D\x7FV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1D\xE4\x81a\x1D\xA8V[\x90P\x91\x90PV[\x7FZERO_ADDRESS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1E!`\x0C\x83a\x11)V[\x91Pa\x1E,\x82a\x1D\xEBV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1EP\x81a\x1E\x14V[\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0a\x1Em\x82a\x1EWV[a\x1Ew\x81\x85a\x19fV[\x93Pa\x1E\x87\x81\x85` \x86\x01a\x11:V[a\x1E\x90\x81a\x11dV[\x84\x01\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90Pa\x1E\xB0`\0\x83\x01\x87a\x12eV[a\x1E\xBD` \x83\x01\x86a\x12eV[a\x1E\xCA`@\x83\x01\x85a\x15(V[\x81\x81\x03``\x83\x01Ra\x1E\xDC\x81\x84a\x1EbV[\x90P\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \rRW\xB6\x93\xBC\x05\xE2}\x91A\nb\xF22\x92\xA9\xA3\x93\xB64b6\x01TS\xD7\xB9^kf[dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKERC721_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockERC721<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockERC721<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockERC721<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockERC721<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockERC721<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockERC721)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockERC721<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKERC721_ABI.clone(),
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
                MOCKERC721_ABI.clone(),
                MOCKERC721_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x4cd88b76) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 216, 139, 118], (name, symbol))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (p0, p1))
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
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
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
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, id))
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
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockERC721Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockERC721<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    pub enum MockERC721Events {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockERC721Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockERC721Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(MockERC721Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockERC721Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockERC721Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockERC721Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for MockERC721Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockERC721Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
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
        pub id: ::ethers::core::types::U256,
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
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string,string)` and selector `0x4cd88b76`
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
    #[ethcall(name = "initialize", abi = "initialize(string,string)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub id: ::ethers::core::types::U256,
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
        pub id: ::ethers::core::types::U256,
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
    pub enum MockERC721Calls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockERC721Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetApprovalForAll(decoded));
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
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockERC721Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockERC721Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for MockERC721Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockERC721Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for MockERC721Calls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockERC721Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for MockERC721Calls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockERC721Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for MockERC721Calls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for MockERC721Calls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
    for MockERC721Calls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for MockERC721Calls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for MockERC721Calls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockERC721Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for MockERC721Calls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockERC721Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
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
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
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
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn {
        pub owner: ::ethers::core::types::Address,
    }
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
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}
