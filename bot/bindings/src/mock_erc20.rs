pub use mock_erc20::*;
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
pub mod mock_erc20 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKERC20_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\x1C\x98\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xCDW_5`\xE0\x1C\x80c6D\xE5\x15\x11a\0\x8AW\x80c\x95\xD8\x9BA\x11a\0dW\x80c\x95\xD8\x9BA\x14a\x02%W\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xD5\x05\xAC\xCF\x14a\x02sW\x80c\xDDb\xED>\x14a\x02\x8FWa\0\xCDV[\x80c6D\xE5\x15\x14a\x01\xA7W\x80cp\xA0\x821\x14a\x01\xC5W\x80c~\xCE\xBE\0\x14a\x01\xF5Wa\0\xCDV[\x80c\x06\xFD\xDE\x03\x14a\0\xD1W\x80c\t^\xA7\xB3\x14a\0\xEFW\x80c\x16$\xF6\xC6\x14a\x01\x1FW\x80c\x18\x16\r\xDD\x14a\x01;W\x80c#\xB8r\xDD\x14a\x01YW\x80c1<\xE5g\x14a\x01\x89W[_\x80\xFD[a\0\xD9a\x02\xBFV[`@Qa\0\xE6\x91\x90a\x0F\x12V[`@Q\x80\x91\x03\x90\xF3[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x0F\xD0V[a\x03JV[`@Qa\x01\x16\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[a\x019`\x04\x806\x03\x81\x01\x90a\x014\x91\x90a\x11\xA3V[a\x047V[\0[a\x01Ca\x04\xFAV[`@Qa\x01P\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[a\x01s`\x04\x806\x03\x81\x01\x90a\x01n\x91\x90a\x12SV[a\x05\0V[`@Qa\x01\x80\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[a\x01\x91a\x07\x8DV[`@Qa\x01\x9E\x91\x90a\x12\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xAFa\x07\x9FV[`@Qa\x01\xBC\x91\x90a\x12\xE3V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDF`\x04\x806\x03\x81\x01\x90a\x01\xDA\x91\x90a\x12\xFCV[a\x07\xC6V[`@Qa\x01\xEC\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[a\x02\x0F`\x04\x806\x03\x81\x01\x90a\x02\n\x91\x90a\x12\xFCV[a\x07\xDBV[`@Qa\x02\x1C\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[a\x02-a\x07\xF0V[`@Qa\x02:\x91\x90a\x0F\x12V[`@Q\x80\x91\x03\x90\xF3[a\x02]`\x04\x806\x03\x81\x01\x90a\x02X\x91\x90a\x0F\xD0V[a\x08|V[`@Qa\x02j\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[a\x02\x8D`\x04\x806\x03\x81\x01\x90a\x02\x88\x91\x90a\x13QV[a\t\xFCV[\0[a\x02\xA9`\x04\x806\x03\x81\x01\x90a\x02\xA4\x91\x90a\x13\xEEV[a\x0C\xEFV[`@Qa\x02\xB6\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[_\x80Ta\x02\xCB\x90a\x14YV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xF7\x90a\x14YV[\x80\x15a\x03BW\x80`\x1F\x10a\x03\x19Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_\x81`\x05_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x04%\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[`\t_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x04\x86W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04}\x90a\x14\xD3V[`@Q\x80\x91\x03\x90\xFD[\x82_\x90\x81a\x04\x94\x91\x90a\x16\x8EV[P\x81`\x01\x90\x81a\x04\xA4\x91\x90a\x16\x8EV[P\x80`\x02_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa\x04\xC7a\r\x0FV[`\x06\x81\x90UPa\x04\xD5a\r1V[`\x07\x81\x90UP`\x01`\t_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[`\x03T\x81V[_\x80`\x05_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P_\x19\x81\x14a\x06\x0CWa\x05\x8F\x81\x84a\r\xC2V[`\x05_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[a\x06S`\x04_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x84a\r\xC2V[`\x04_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UPa\x06\xDB`\x04_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x84a\x0E\x1AV[`\x04_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x85`@Qa\x07y\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3`\x01\x91PP\x93\x92PPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[_`\x06Ta\x07\xABa\r\x0FV[\x14a\x07\xBDWa\x07\xB8a\r1V[a\x07\xC1V[`\x07T[\x90P\x90V[`\x04` R\x80_R`@_ _\x91P\x90PT\x81V[`\x08` R\x80_R`@_ _\x91P\x90PT\x81V[`\x01\x80Ta\x07\xFD\x90a\x14YV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08)\x90a\x14YV[\x80\x15a\x08tW\x80`\x1F\x10a\x08KWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_a\x08\xC4`\x04_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x83a\r\xC2V[`\x04_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UPa\tL`\x04_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x83a\x0E\x1AV[`\x04_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\t\xEA\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[B\x84\x10\x15a\n?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n6\x90a\x17\xA7V[`@Q\x80\x91\x03\x90\xFD[_`\x01a\nJa\x07\x9FV[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x8A\x8A\x8A`\x08_\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x80\x92\x91\x90a\n\xBB\x90a\x17\xF2V[\x91\x90PU\x8B`@Q` \x01a\n\xD5\x96\x95\x94\x93\x92\x91\x90a\x18HV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xFC\x92\x91\x90a\x19\x1BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85\x85`@Q_\x81R` \x01`@R`@Qa\x0B1\x94\x93\x92\x91\x90a\x19QV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0BQW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0B\xC4WP\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x0C\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xFA\x90a\x19\xDEV[`@Q\x80\x91\x03\x90\xFD[\x85`\x05_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x88`@Qa\x0C\xDD\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\x05` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91PPT\x81V[_a\x0E~a\x0Ew\x90Pa\x0E~\x81\x90Pa\r*\x81c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x90V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F_`@Qa\ra\x91\x90a\x1A\x98V[`@Q\x80\x91\x03\x90 \x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6a\r\x92a\r\x0FV[0`@Q` \x01a\r\xA7\x95\x94\x93\x92\x91\x90a\x1A\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81\x83\x10\x15a\x0E\x06W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xFD\x90a\x1BIV[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x0E\x12\x91\x90a\x1BgV[\x90P\x92\x91PPV[_\x80\x82\x84a\x0E(\x91\x90a\x1B\x9AV[\x90P\x83\x81\x10\x15a\x0EmW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ed\x90a\x1C\x17V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[_F\x90P\x90V[a\x0E\x86a\x1C5V[V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x0E\xBFW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x0E\xA4V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a\x0E\xE4\x82a\x0E\x88V[a\x0E\xEE\x81\x85a\x0E\x92V[\x93Pa\x0E\xFE\x81\x85` \x86\x01a\x0E\xA2V[a\x0F\x07\x81a\x0E\xCAV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F*\x81\x84a\x0E\xDAV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0Fl\x82a\x0FCV[\x90P\x91\x90PV[a\x0F|\x81a\x0FbV[\x81\x14a\x0F\x86W_\x80\xFD[PV[_\x815\x90Pa\x0F\x97\x81a\x0FsV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0F\xAF\x81a\x0F\x9DV[\x81\x14a\x0F\xB9W_\x80\xFD[PV[_\x815\x90Pa\x0F\xCA\x81a\x0F\xA6V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x0F\xE6Wa\x0F\xE5a\x0F;V[[_a\x0F\xF3\x85\x82\x86\x01a\x0F\x89V[\x92PP` a\x10\x04\x85\x82\x86\x01a\x0F\xBCV[\x91PP\x92P\x92\x90PV[_\x81\x15\x15\x90P\x91\x90PV[a\x10\"\x81a\x10\x0EV[\x82RPPV[_` \x82\x01\x90Pa\x10;_\x83\x01\x84a\x10\x19V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x10\x7F\x82a\x0E\xCAV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10\x9EWa\x10\x9Da\x10IV[[\x80`@RPPPV[_a\x10\xB0a\x0F2V[\x90Pa\x10\xBC\x82\x82a\x10vV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xDBWa\x10\xDAa\x10IV[[a\x10\xE4\x82a\x0E\xCAV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x11\x11a\x11\x0C\x84a\x10\xC1V[a\x10\xA7V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x11-Wa\x11,a\x10EV[[a\x118\x84\x82\x85a\x10\xF1V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x11TWa\x11Sa\x10AV[[\x815a\x11d\x84\x82` \x86\x01a\x10\xFFV[\x91PP\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x11\x82\x81a\x11mV[\x81\x14a\x11\x8CW_\x80\xFD[PV[_\x815\x90Pa\x11\x9D\x81a\x11yV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x11\xBAWa\x11\xB9a\x0F;V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xD7Wa\x11\xD6a\x0F?V[[a\x11\xE3\x86\x82\x87\x01a\x11@V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x04Wa\x12\x03a\x0F?V[[a\x12\x10\x86\x82\x87\x01a\x11@V[\x92PP`@a\x12!\x86\x82\x87\x01a\x11\x8FV[\x91PP\x92P\x92P\x92V[a\x124\x81a\x0F\x9DV[\x82RPPV[_` \x82\x01\x90Pa\x12M_\x83\x01\x84a\x12+V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x12jWa\x12ia\x0F;V[[_a\x12w\x86\x82\x87\x01a\x0F\x89V[\x93PP` a\x12\x88\x86\x82\x87\x01a\x0F\x89V[\x92PP`@a\x12\x99\x86\x82\x87\x01a\x0F\xBCV[\x91PP\x92P\x92P\x92V[a\x12\xAC\x81a\x11mV[\x82RPPV[_` \x82\x01\x90Pa\x12\xC5_\x83\x01\x84a\x12\xA3V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x12\xDD\x81a\x12\xCBV[\x82RPPV[_` \x82\x01\x90Pa\x12\xF6_\x83\x01\x84a\x12\xD4V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\x11Wa\x13\x10a\x0F;V[[_a\x13\x1E\x84\x82\x85\x01a\x0F\x89V[\x91PP\x92\x91PPV[a\x130\x81a\x12\xCBV[\x81\x14a\x13:W_\x80\xFD[PV[_\x815\x90Pa\x13K\x81a\x13'V[\x92\x91PPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a\x13lWa\x13ka\x0F;V[[_a\x13y\x8A\x82\x8B\x01a\x0F\x89V[\x97PP` a\x13\x8A\x8A\x82\x8B\x01a\x0F\x89V[\x96PP`@a\x13\x9B\x8A\x82\x8B\x01a\x0F\xBCV[\x95PP``a\x13\xAC\x8A\x82\x8B\x01a\x0F\xBCV[\x94PP`\x80a\x13\xBD\x8A\x82\x8B\x01a\x11\x8FV[\x93PP`\xA0a\x13\xCE\x8A\x82\x8B\x01a\x13=V[\x92PP`\xC0a\x13\xDF\x8A\x82\x8B\x01a\x13=V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_\x80`@\x83\x85\x03\x12\x15a\x14\x04Wa\x14\x03a\x0F;V[[_a\x14\x11\x85\x82\x86\x01a\x0F\x89V[\x92PP` a\x14\"\x85\x82\x86\x01a\x0F\x89V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x14pW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14\x83Wa\x14\x82a\x14,V[[P\x91\x90PV[\x7FALREADY_INITIALIZED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x14\xBD`\x13\x83a\x0E\x92V[\x91Pa\x14\xC8\x82a\x14\x89V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x14\xEA\x81a\x14\xB1V[\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a\x15M\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x15\x12V[a\x15W\x86\x83a\x15\x12V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x15\x92a\x15\x8Da\x15\x88\x84a\x0F\x9DV[a\x15oV[a\x0F\x9DV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x15\xAB\x83a\x15xV[a\x15\xBFa\x15\xB7\x82a\x15\x99V[\x84\x84Ta\x15\x1EV[\x82UPPPPV[_\x90V[a\x15\xD3a\x15\xC7V[a\x15\xDE\x81\x84\x84a\x15\xA2V[PPPV[[\x81\x81\x10\x15a\x16\x01Wa\x15\xF6_\x82a\x15\xCBV[`\x01\x81\x01\x90Pa\x15\xE4V[PPV[`\x1F\x82\x11\x15a\x16FWa\x16\x17\x81a\x14\xF1V[a\x16 \x84a\x15\x03V[\x81\x01` \x85\x10\x15a\x16/W\x81\x90P[a\x16Ca\x16;\x85a\x15\x03V[\x83\x01\x82a\x15\xE3V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a\x16f_\x19\x84`\x08\x02a\x16KV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a\x16~\x83\x83a\x16WV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x16\x97\x82a\x0E\x88V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xB0Wa\x16\xAFa\x10IV[[a\x16\xBA\x82Ta\x14YV[a\x16\xC5\x82\x82\x85a\x16\x05V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a\x16\xF6W_\x84\x15a\x16\xE4W\x82\x87\x01Q\x90P[a\x16\xEE\x85\x82a\x16sV[\x86UPa\x17UV[`\x1F\x19\x84\x16a\x17\x04\x86a\x14\xF1V[_[\x82\x81\x10\x15a\x17+W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x17\x06V[\x86\x83\x10\x15a\x17HW\x84\x89\x01Qa\x17D`\x1F\x89\x16\x82a\x16WV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x17\x91`\x17\x83a\x0E\x92V[\x91Pa\x17\x9C\x82a\x17]V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x17\xBE\x81a\x17\x85V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x17\xFC\x82a\x0F\x9DV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x18.Wa\x18-a\x17\xC5V[[`\x01\x82\x01\x90P\x91\x90PV[a\x18B\x81a\x0FbV[\x82RPPV[_`\xC0\x82\x01\x90Pa\x18[_\x83\x01\x89a\x12\xD4V[a\x18h` \x83\x01\x88a\x189V[a\x18u`@\x83\x01\x87a\x189V[a\x18\x82``\x83\x01\x86a\x12+V[a\x18\x8F`\x80\x83\x01\x85a\x12+V[a\x18\x9C`\xA0\x83\x01\x84a\x12+V[\x97\x96PPPPPPPV[_\x81\x90P\x92\x91PPV[\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x18\xE5`\x02\x83a\x18\xA7V[\x91Pa\x18\xF0\x82a\x18\xB1V[`\x02\x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x19\x15a\x19\x10\x82a\x12\xCBV[a\x18\xFBV[\x82RPPV[_a\x19%\x82a\x18\xD9V[\x91Pa\x191\x82\x85a\x19\x04V[` \x82\x01\x91Pa\x19A\x82\x84a\x19\x04V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pa\x19d_\x83\x01\x87a\x12\xD4V[a\x19q` \x83\x01\x86a\x12\xA3V[a\x19~`@\x83\x01\x85a\x12\xD4V[a\x19\x8B``\x83\x01\x84a\x12\xD4V[\x95\x94PPPPPV[\x7FINVALID_SIGNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x19\xC8`\x0E\x83a\x0E\x92V[\x91Pa\x19\xD3\x82a\x19\x94V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19\xF5\x81a\x19\xBCV[\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x81Ta\x1A$\x81a\x14YV[a\x1A.\x81\x86a\x19\xFCV[\x94P`\x01\x82\x16_\x81\x14a\x1AHW`\x01\x81\x14a\x1A]Wa\x1A\x8FV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x1A\x8FV[a\x1Af\x85a\x1A\x06V[_[\x83\x81\x10\x15a\x1A\x87W\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x1AhV[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[_a\x1A\xA3\x82\x84a\x1A\x18V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pa\x1A\xC1_\x83\x01\x88a\x12\xD4V[a\x1A\xCE` \x83\x01\x87a\x12\xD4V[a\x1A\xDB`@\x83\x01\x86a\x12\xD4V[a\x1A\xE8``\x83\x01\x85a\x12+V[a\x1A\xF5`\x80\x83\x01\x84a\x189V[\x96\x95PPPPPPV[\x7FERC20: subtraction underflow\0\0\0\0_\x82\x01RPV[_a\x1B3`\x1C\x83a\x0E\x92V[\x91Pa\x1B>\x82a\x1A\xFFV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1B`\x81a\x1B'V[\x90P\x91\x90PV[_a\x1Bq\x82a\x0F\x9DV[\x91Pa\x1B|\x83a\x0F\x9DV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1B\x94Wa\x1B\x93a\x17\xC5V[[\x92\x91PPV[_a\x1B\xA4\x82a\x0F\x9DV[\x91Pa\x1B\xAF\x83a\x0F\x9DV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1B\xC7Wa\x1B\xC6a\x17\xC5V[[\x92\x91PPV[\x7FERC20: addition overflow\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x1C\x01`\x18\x83a\x0E\x92V[\x91Pa\x1C\x0C\x82a\x1B\xCDV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C.\x81a\x1B\xF5V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`Q`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 j\x84%6\x0F}\x18\x97u\x85$0\x82{\xF5\xC7\xE6\xE3\xF3r\xA0\xAB\x0C\x18\x0E\x89\xCA(\x08XS\x08dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKERC20_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xCDW_5`\xE0\x1C\x80c6D\xE5\x15\x11a\0\x8AW\x80c\x95\xD8\x9BA\x11a\0dW\x80c\x95\xD8\x9BA\x14a\x02%W\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xD5\x05\xAC\xCF\x14a\x02sW\x80c\xDDb\xED>\x14a\x02\x8FWa\0\xCDV[\x80c6D\xE5\x15\x14a\x01\xA7W\x80cp\xA0\x821\x14a\x01\xC5W\x80c~\xCE\xBE\0\x14a\x01\xF5Wa\0\xCDV[\x80c\x06\xFD\xDE\x03\x14a\0\xD1W\x80c\t^\xA7\xB3\x14a\0\xEFW\x80c\x16$\xF6\xC6\x14a\x01\x1FW\x80c\x18\x16\r\xDD\x14a\x01;W\x80c#\xB8r\xDD\x14a\x01YW\x80c1<\xE5g\x14a\x01\x89W[_\x80\xFD[a\0\xD9a\x02\xBFV[`@Qa\0\xE6\x91\x90a\x0F\x12V[`@Q\x80\x91\x03\x90\xF3[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x0F\xD0V[a\x03JV[`@Qa\x01\x16\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[a\x019`\x04\x806\x03\x81\x01\x90a\x014\x91\x90a\x11\xA3V[a\x047V[\0[a\x01Ca\x04\xFAV[`@Qa\x01P\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[a\x01s`\x04\x806\x03\x81\x01\x90a\x01n\x91\x90a\x12SV[a\x05\0V[`@Qa\x01\x80\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[a\x01\x91a\x07\x8DV[`@Qa\x01\x9E\x91\x90a\x12\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xAFa\x07\x9FV[`@Qa\x01\xBC\x91\x90a\x12\xE3V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDF`\x04\x806\x03\x81\x01\x90a\x01\xDA\x91\x90a\x12\xFCV[a\x07\xC6V[`@Qa\x01\xEC\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[a\x02\x0F`\x04\x806\x03\x81\x01\x90a\x02\n\x91\x90a\x12\xFCV[a\x07\xDBV[`@Qa\x02\x1C\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[a\x02-a\x07\xF0V[`@Qa\x02:\x91\x90a\x0F\x12V[`@Q\x80\x91\x03\x90\xF3[a\x02]`\x04\x806\x03\x81\x01\x90a\x02X\x91\x90a\x0F\xD0V[a\x08|V[`@Qa\x02j\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[a\x02\x8D`\x04\x806\x03\x81\x01\x90a\x02\x88\x91\x90a\x13QV[a\t\xFCV[\0[a\x02\xA9`\x04\x806\x03\x81\x01\x90a\x02\xA4\x91\x90a\x13\xEEV[a\x0C\xEFV[`@Qa\x02\xB6\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xF3[_\x80Ta\x02\xCB\x90a\x14YV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xF7\x90a\x14YV[\x80\x15a\x03BW\x80`\x1F\x10a\x03\x19Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_\x81`\x05_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x04%\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[`\t_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x04\x86W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04}\x90a\x14\xD3V[`@Q\x80\x91\x03\x90\xFD[\x82_\x90\x81a\x04\x94\x91\x90a\x16\x8EV[P\x81`\x01\x90\x81a\x04\xA4\x91\x90a\x16\x8EV[P\x80`\x02_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa\x04\xC7a\r\x0FV[`\x06\x81\x90UPa\x04\xD5a\r1V[`\x07\x81\x90UP`\x01`\t_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[`\x03T\x81V[_\x80`\x05_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P_\x19\x81\x14a\x06\x0CWa\x05\x8F\x81\x84a\r\xC2V[`\x05_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[a\x06S`\x04_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x84a\r\xC2V[`\x04_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UPa\x06\xDB`\x04_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x84a\x0E\x1AV[`\x04_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x85`@Qa\x07y\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3`\x01\x91PP\x93\x92PPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[_`\x06Ta\x07\xABa\r\x0FV[\x14a\x07\xBDWa\x07\xB8a\r1V[a\x07\xC1V[`\x07T[\x90P\x90V[`\x04` R\x80_R`@_ _\x91P\x90PT\x81V[`\x08` R\x80_R`@_ _\x91P\x90PT\x81V[`\x01\x80Ta\x07\xFD\x90a\x14YV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08)\x90a\x14YV[\x80\x15a\x08tW\x80`\x1F\x10a\x08KWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_a\x08\xC4`\x04_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x83a\r\xC2V[`\x04_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UPa\tL`\x04_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x83a\x0E\x1AV[`\x04_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\t\xEA\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[B\x84\x10\x15a\n?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n6\x90a\x17\xA7V[`@Q\x80\x91\x03\x90\xFD[_`\x01a\nJa\x07\x9FV[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x8A\x8A\x8A`\x08_\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x80\x92\x91\x90a\n\xBB\x90a\x17\xF2V[\x91\x90PU\x8B`@Q` \x01a\n\xD5\x96\x95\x94\x93\x92\x91\x90a\x18HV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xFC\x92\x91\x90a\x19\x1BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85\x85`@Q_\x81R` \x01`@R`@Qa\x0B1\x94\x93\x92\x91\x90a\x19QV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0BQW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0B\xC4WP\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x0C\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xFA\x90a\x19\xDEV[`@Q\x80\x91\x03\x90\xFD[\x85`\x05_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x88`@Qa\x0C\xDD\x91\x90a\x12:V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\x05` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91PPT\x81V[_a\x0E~a\x0Ew\x90Pa\x0E~\x81\x90Pa\r*\x81c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x90V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F_`@Qa\ra\x91\x90a\x1A\x98V[`@Q\x80\x91\x03\x90 \x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6a\r\x92a\r\x0FV[0`@Q` \x01a\r\xA7\x95\x94\x93\x92\x91\x90a\x1A\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81\x83\x10\x15a\x0E\x06W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xFD\x90a\x1BIV[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x0E\x12\x91\x90a\x1BgV[\x90P\x92\x91PPV[_\x80\x82\x84a\x0E(\x91\x90a\x1B\x9AV[\x90P\x83\x81\x10\x15a\x0EmW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ed\x90a\x1C\x17V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[_F\x90P\x90V[a\x0E\x86a\x1C5V[V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x0E\xBFW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x0E\xA4V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a\x0E\xE4\x82a\x0E\x88V[a\x0E\xEE\x81\x85a\x0E\x92V[\x93Pa\x0E\xFE\x81\x85` \x86\x01a\x0E\xA2V[a\x0F\x07\x81a\x0E\xCAV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F*\x81\x84a\x0E\xDAV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0Fl\x82a\x0FCV[\x90P\x91\x90PV[a\x0F|\x81a\x0FbV[\x81\x14a\x0F\x86W_\x80\xFD[PV[_\x815\x90Pa\x0F\x97\x81a\x0FsV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0F\xAF\x81a\x0F\x9DV[\x81\x14a\x0F\xB9W_\x80\xFD[PV[_\x815\x90Pa\x0F\xCA\x81a\x0F\xA6V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x0F\xE6Wa\x0F\xE5a\x0F;V[[_a\x0F\xF3\x85\x82\x86\x01a\x0F\x89V[\x92PP` a\x10\x04\x85\x82\x86\x01a\x0F\xBCV[\x91PP\x92P\x92\x90PV[_\x81\x15\x15\x90P\x91\x90PV[a\x10\"\x81a\x10\x0EV[\x82RPPV[_` \x82\x01\x90Pa\x10;_\x83\x01\x84a\x10\x19V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x10\x7F\x82a\x0E\xCAV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10\x9EWa\x10\x9Da\x10IV[[\x80`@RPPPV[_a\x10\xB0a\x0F2V[\x90Pa\x10\xBC\x82\x82a\x10vV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xDBWa\x10\xDAa\x10IV[[a\x10\xE4\x82a\x0E\xCAV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x11\x11a\x11\x0C\x84a\x10\xC1V[a\x10\xA7V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x11-Wa\x11,a\x10EV[[a\x118\x84\x82\x85a\x10\xF1V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x11TWa\x11Sa\x10AV[[\x815a\x11d\x84\x82` \x86\x01a\x10\xFFV[\x91PP\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x11\x82\x81a\x11mV[\x81\x14a\x11\x8CW_\x80\xFD[PV[_\x815\x90Pa\x11\x9D\x81a\x11yV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x11\xBAWa\x11\xB9a\x0F;V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xD7Wa\x11\xD6a\x0F?V[[a\x11\xE3\x86\x82\x87\x01a\x11@V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x04Wa\x12\x03a\x0F?V[[a\x12\x10\x86\x82\x87\x01a\x11@V[\x92PP`@a\x12!\x86\x82\x87\x01a\x11\x8FV[\x91PP\x92P\x92P\x92V[a\x124\x81a\x0F\x9DV[\x82RPPV[_` \x82\x01\x90Pa\x12M_\x83\x01\x84a\x12+V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x12jWa\x12ia\x0F;V[[_a\x12w\x86\x82\x87\x01a\x0F\x89V[\x93PP` a\x12\x88\x86\x82\x87\x01a\x0F\x89V[\x92PP`@a\x12\x99\x86\x82\x87\x01a\x0F\xBCV[\x91PP\x92P\x92P\x92V[a\x12\xAC\x81a\x11mV[\x82RPPV[_` \x82\x01\x90Pa\x12\xC5_\x83\x01\x84a\x12\xA3V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x12\xDD\x81a\x12\xCBV[\x82RPPV[_` \x82\x01\x90Pa\x12\xF6_\x83\x01\x84a\x12\xD4V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\x11Wa\x13\x10a\x0F;V[[_a\x13\x1E\x84\x82\x85\x01a\x0F\x89V[\x91PP\x92\x91PPV[a\x130\x81a\x12\xCBV[\x81\x14a\x13:W_\x80\xFD[PV[_\x815\x90Pa\x13K\x81a\x13'V[\x92\x91PPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a\x13lWa\x13ka\x0F;V[[_a\x13y\x8A\x82\x8B\x01a\x0F\x89V[\x97PP` a\x13\x8A\x8A\x82\x8B\x01a\x0F\x89V[\x96PP`@a\x13\x9B\x8A\x82\x8B\x01a\x0F\xBCV[\x95PP``a\x13\xAC\x8A\x82\x8B\x01a\x0F\xBCV[\x94PP`\x80a\x13\xBD\x8A\x82\x8B\x01a\x11\x8FV[\x93PP`\xA0a\x13\xCE\x8A\x82\x8B\x01a\x13=V[\x92PP`\xC0a\x13\xDF\x8A\x82\x8B\x01a\x13=V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_\x80`@\x83\x85\x03\x12\x15a\x14\x04Wa\x14\x03a\x0F;V[[_a\x14\x11\x85\x82\x86\x01a\x0F\x89V[\x92PP` a\x14\"\x85\x82\x86\x01a\x0F\x89V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x14pW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14\x83Wa\x14\x82a\x14,V[[P\x91\x90PV[\x7FALREADY_INITIALIZED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x14\xBD`\x13\x83a\x0E\x92V[\x91Pa\x14\xC8\x82a\x14\x89V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x14\xEA\x81a\x14\xB1V[\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a\x15M\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x15\x12V[a\x15W\x86\x83a\x15\x12V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x15\x92a\x15\x8Da\x15\x88\x84a\x0F\x9DV[a\x15oV[a\x0F\x9DV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x15\xAB\x83a\x15xV[a\x15\xBFa\x15\xB7\x82a\x15\x99V[\x84\x84Ta\x15\x1EV[\x82UPPPPV[_\x90V[a\x15\xD3a\x15\xC7V[a\x15\xDE\x81\x84\x84a\x15\xA2V[PPPV[[\x81\x81\x10\x15a\x16\x01Wa\x15\xF6_\x82a\x15\xCBV[`\x01\x81\x01\x90Pa\x15\xE4V[PPV[`\x1F\x82\x11\x15a\x16FWa\x16\x17\x81a\x14\xF1V[a\x16 \x84a\x15\x03V[\x81\x01` \x85\x10\x15a\x16/W\x81\x90P[a\x16Ca\x16;\x85a\x15\x03V[\x83\x01\x82a\x15\xE3V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a\x16f_\x19\x84`\x08\x02a\x16KV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a\x16~\x83\x83a\x16WV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x16\x97\x82a\x0E\x88V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xB0Wa\x16\xAFa\x10IV[[a\x16\xBA\x82Ta\x14YV[a\x16\xC5\x82\x82\x85a\x16\x05V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a\x16\xF6W_\x84\x15a\x16\xE4W\x82\x87\x01Q\x90P[a\x16\xEE\x85\x82a\x16sV[\x86UPa\x17UV[`\x1F\x19\x84\x16a\x17\x04\x86a\x14\xF1V[_[\x82\x81\x10\x15a\x17+W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x17\x06V[\x86\x83\x10\x15a\x17HW\x84\x89\x01Qa\x17D`\x1F\x89\x16\x82a\x16WV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x17\x91`\x17\x83a\x0E\x92V[\x91Pa\x17\x9C\x82a\x17]V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x17\xBE\x81a\x17\x85V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x17\xFC\x82a\x0F\x9DV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x18.Wa\x18-a\x17\xC5V[[`\x01\x82\x01\x90P\x91\x90PV[a\x18B\x81a\x0FbV[\x82RPPV[_`\xC0\x82\x01\x90Pa\x18[_\x83\x01\x89a\x12\xD4V[a\x18h` \x83\x01\x88a\x189V[a\x18u`@\x83\x01\x87a\x189V[a\x18\x82``\x83\x01\x86a\x12+V[a\x18\x8F`\x80\x83\x01\x85a\x12+V[a\x18\x9C`\xA0\x83\x01\x84a\x12+V[\x97\x96PPPPPPPV[_\x81\x90P\x92\x91PPV[\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x18\xE5`\x02\x83a\x18\xA7V[\x91Pa\x18\xF0\x82a\x18\xB1V[`\x02\x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x19\x15a\x19\x10\x82a\x12\xCBV[a\x18\xFBV[\x82RPPV[_a\x19%\x82a\x18\xD9V[\x91Pa\x191\x82\x85a\x19\x04V[` \x82\x01\x91Pa\x19A\x82\x84a\x19\x04V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pa\x19d_\x83\x01\x87a\x12\xD4V[a\x19q` \x83\x01\x86a\x12\xA3V[a\x19~`@\x83\x01\x85a\x12\xD4V[a\x19\x8B``\x83\x01\x84a\x12\xD4V[\x95\x94PPPPPV[\x7FINVALID_SIGNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x19\xC8`\x0E\x83a\x0E\x92V[\x91Pa\x19\xD3\x82a\x19\x94V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19\xF5\x81a\x19\xBCV[\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x81Ta\x1A$\x81a\x14YV[a\x1A.\x81\x86a\x19\xFCV[\x94P`\x01\x82\x16_\x81\x14a\x1AHW`\x01\x81\x14a\x1A]Wa\x1A\x8FV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x1A\x8FV[a\x1Af\x85a\x1A\x06V[_[\x83\x81\x10\x15a\x1A\x87W\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x1AhV[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[_a\x1A\xA3\x82\x84a\x1A\x18V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pa\x1A\xC1_\x83\x01\x88a\x12\xD4V[a\x1A\xCE` \x83\x01\x87a\x12\xD4V[a\x1A\xDB`@\x83\x01\x86a\x12\xD4V[a\x1A\xE8``\x83\x01\x85a\x12+V[a\x1A\xF5`\x80\x83\x01\x84a\x189V[\x96\x95PPPPPPV[\x7FERC20: subtraction underflow\0\0\0\0_\x82\x01RPV[_a\x1B3`\x1C\x83a\x0E\x92V[\x91Pa\x1B>\x82a\x1A\xFFV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1B`\x81a\x1B'V[\x90P\x91\x90PV[_a\x1Bq\x82a\x0F\x9DV[\x91Pa\x1B|\x83a\x0F\x9DV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1B\x94Wa\x1B\x93a\x17\xC5V[[\x92\x91PPV[_a\x1B\xA4\x82a\x0F\x9DV[\x91Pa\x1B\xAF\x83a\x0F\x9DV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1B\xC7Wa\x1B\xC6a\x17\xC5V[[\x92\x91PPV[\x7FERC20: addition overflow\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x1C\x01`\x18\x83a\x0E\x92V[\x91Pa\x1C\x0C\x82a\x1B\xCDV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C.\x81a\x1B\xF5V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`Q`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 j\x84%6\x0F}\x18\x97u\x85$0\x82{\xF5\xC7\xE6\xE3\xF3r\xA0\xAB\x0C\x18\x0E\x89\xCA(\x08XS\x08dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKERC20_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockERC20<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockERC20<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockERC20)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKERC20_ABI.clone(),
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
                MOCKERC20_ABI.clone(),
                MOCKERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1624f6c6) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            symbol: ::std::string::String,
            decimals: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 36, 246, 198], (name, symbol, decimals))
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
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
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
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
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
            MockERC20Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockERC20<M> {
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum MockERC20Events {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockERC20Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockERC20Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockERC20Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockERC20Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockERC20Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
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
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
        pub amount: ::ethers::core::types::U256,
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
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string,string,uint8)` and selector `0x1624f6c6`
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
    #[ethcall(name = "initialize", abi = "initialize(string,string,uint8)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
        pub decimals: u8,
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
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
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
        pub amount: ::ethers::core::types::U256,
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
        pub amount: ::ethers::core::types::U256,
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
    pub enum MockERC20Calls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Initialize(InitializeCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
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
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
            }
        }
    }
    impl ::core::fmt::Display for MockERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for MockERC20Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockERC20Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockERC20Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockERC20Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockERC20Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockERC20Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockERC20Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for MockERC20Calls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockERC20Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockERC20Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockERC20Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockERC20Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockERC20Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
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
