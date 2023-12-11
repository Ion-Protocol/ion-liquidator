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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x1Da\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c6D\xE5\x15\x11a\0\x8CW\x80c\x95\xD8\x9BA\x11a\0fW\x80c\x95\xD8\x9BA\x14a\x02(W\x80c\xA9\x05\x9C\xBB\x14a\x02FW\x80c\xD5\x05\xAC\xCF\x14a\x02vW\x80c\xDDb\xED>\x14a\x02\x92Wa\0\xCFV[\x80c6D\xE5\x15\x14a\x01\xAAW\x80cp\xA0\x821\x14a\x01\xC8W\x80c~\xCE\xBE\0\x14a\x01\xF8Wa\0\xCFV[\x80c\x06\xFD\xDE\x03\x14a\0\xD4W\x80c\t^\xA7\xB3\x14a\0\xF2W\x80c\x16$\xF6\xC6\x14a\x01\"W\x80c\x18\x16\r\xDD\x14a\x01>W\x80c#\xB8r\xDD\x14a\x01\\W\x80c1<\xE5g\x14a\x01\x8CW[`\0\x80\xFD[a\0\xDCa\x02\xC2V[`@Qa\0\xE9\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x01\x0C`\x04\x806\x03\x81\x01\x90a\x01\x07\x91\x90a\x10-V[a\x03PV[`@Qa\x01\x19\x91\x90a\x10\x88V[`@Q\x80\x91\x03\x90\xF3[a\x01<`\x04\x806\x03\x81\x01\x90a\x017\x91\x90a\x12\x11V[a\x04BV[\0[a\x01Fa\x05\tV[`@Qa\x01S\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[a\x01v`\x04\x806\x03\x81\x01\x90a\x01q\x91\x90a\x12\xC6V[a\x05\x0FV[`@Qa\x01\x83\x91\x90a\x10\x88V[`@Q\x80\x91\x03\x90\xF3[a\x01\x94a\x07\xAEV[`@Qa\x01\xA1\x91\x90a\x13(V[`@Q\x80\x91\x03\x90\xF3[a\x01\xB2a\x07\xC1V[`@Qa\x01\xBF\x91\x90a\x13\\V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE2`\x04\x806\x03\x81\x01\x90a\x01\xDD\x91\x90a\x13wV[a\x07\xE9V[`@Qa\x01\xEF\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[a\x02\x12`\x04\x806\x03\x81\x01\x90a\x02\r\x91\x90a\x13wV[a\x08\x01V[`@Qa\x02\x1F\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[a\x020a\x08\x19V[`@Qa\x02=\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x02``\x04\x806\x03\x81\x01\x90a\x02[\x91\x90a\x10-V[a\x08\xA7V[`@Qa\x02m\x91\x90a\x10\x88V[`@Q\x80\x91\x03\x90\xF3[a\x02\x90`\x04\x806\x03\x81\x01\x90a\x02\x8B\x91\x90a\x13\xD0V[a\n0V[\0[a\x02\xAC`\x04\x806\x03\x81\x01\x90a\x02\xA7\x91\x90a\x14rV[a\r/V[`@Qa\x02\xB9\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[`\0\x80Ta\x02\xCF\x90a\x14\xE1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFB\x90a\x14\xE1V[\x80\x15a\x03HW\x80`\x1F\x10a\x03\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03HV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03+W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x81`\x05`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x040\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x04\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x89\x90a\x15^V[`@Q\x80\x91\x03\x90\xFD[\x82`\0\x90\x81a\x04\xA1\x91\x90a\x17*V[P\x81`\x01\x90\x81a\x04\xB1\x91\x90a\x17*V[P\x80`\x02`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa\x04\xD5a\rTV[`\x06\x81\x90UPa\x04\xE3a\rwV[`\x07\x81\x90UP`\x01`\t`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[`\x03T\x81V[`\0\x80`\x05`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x19\x81\x14a\x06%Wa\x05\xA4\x81\x84a\x0E\nV[`\x05`\0\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP[a\x06n`\x04`\0\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84a\x0E\nV[`\x04`\0\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\x06\xFA`\x04`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84a\x0EcV[`\x04`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x85`@Qa\x07\x9A\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3`\x01\x91PP\x93\x92PPPV[`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x06Ta\x07\xCEa\rTV[\x14a\x07\xE0Wa\x07\xDBa\rwV[a\x07\xE4V[`\x07T[\x90P\x90V[`\x04` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[`\x08` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[`\x01\x80Ta\x08&\x90a\x14\xE1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08R\x90a\x14\xE1V[\x80\x15a\x08\x9FW\x80`\x1F\x10a\x08tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x9FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x08\xF2`\x04`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x83a\x0E\nV[`\x04`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\t~`\x04`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x83a\x0EcV[`\x04`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\n\x1E\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[B\x84\x10\x15a\nsW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nj\x90a\x18HV[`@Q\x80\x91\x03\x90\xFD[`\0`\x01a\n\x7Fa\x07\xC1V[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x8A\x8A\x8A`\x08`\0\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81T\x80\x92\x91\x90a\n\xF3\x90a\x18\x97V[\x91\x90PU\x8B`@Q` \x01a\x0B\r\x96\x95\x94\x93\x92\x91\x90a\x18\xEEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B4\x92\x91\x90a\x19\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85\x85`@Q`\0\x81R` \x01`@R`@Qa\x0Bj\x94\x93\x92\x91\x90a\x19\xFEV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\x8CW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0C\0WP\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x0C?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C6\x90a\x1A\x8FV[`@Q\x80\x91\x03\x90\xFD[\x85`\x05`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x88`@Qa\r\x1D\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\x05` R\x81`\0R`@`\0 ` R\x80`\0R`@`\0 `\0\x91P\x91PPT\x81V[`\0a\x0E\xC9a\x0E\xC1\x90Pa\x0E\xC9\x81\x90Pa\rp\x81c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x90V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\r\xA9\x91\x90a\x1BRV[`@Q\x80\x91\x03\x90 \x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6a\r\xDAa\rTV[0`@Q` \x01a\r\xEF\x95\x94\x93\x92\x91\x90a\x1BiV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81\x83\x10\x15a\x0EOW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EF\x90a\x1C\x08V[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x0E[\x91\x90a\x1C(V[\x90P\x92\x91PPV[`\0\x80\x82\x84a\x0Er\x91\x90a\x1C\\V[\x90P\x83\x81\x10\x15a\x0E\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xAE\x90a\x1C\xDCV[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0F\x90P\x90V[a\x0E\xD1a\x1C\xFCV[V[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x0F\rW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x0E\xF2V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x0F5\x82a\x0E\xD3V[a\x0F?\x81\x85a\x0E\xDEV[\x93Pa\x0FO\x81\x85` \x86\x01a\x0E\xEFV[a\x0FX\x81a\x0F\x19V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0F}\x81\x84a\x0F*V[\x90P\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0F\xC4\x82a\x0F\x99V[\x90P\x91\x90PV[a\x0F\xD4\x81a\x0F\xB9V[\x81\x14a\x0F\xDFW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\xF1\x81a\x0F\xCBV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x10\n\x81a\x0F\xF7V[\x81\x14a\x10\x15W`\0\x80\xFD[PV[`\0\x815\x90Pa\x10'\x81a\x10\x01V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10DWa\x10Ca\x0F\x8FV[[`\0a\x10R\x85\x82\x86\x01a\x0F\xE2V[\x92PP` a\x10c\x85\x82\x86\x01a\x10\x18V[\x91PP\x92P\x92\x90PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x10\x82\x81a\x10mV[\x82RPPV[`\0` \x82\x01\x90Pa\x10\x9D`\0\x83\x01\x84a\x10yV[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x10\xE5\x82a\x0F\x19V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x11\x04Wa\x11\x03a\x10\xADV[[\x80`@RPPPV[`\0a\x11\x17a\x0F\x85V[\x90Pa\x11#\x82\x82a\x10\xDCV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11CWa\x11Ba\x10\xADV[[a\x11L\x82a\x0F\x19V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x11{a\x11v\x84a\x11(V[a\x11\rV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x11\x97Wa\x11\x96a\x10\xA8V[[a\x11\xA2\x84\x82\x85a\x11YV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x11\xBFWa\x11\xBEa\x10\xA3V[[\x815a\x11\xCF\x84\x82` \x86\x01a\x11hV[\x91PP\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x11\xEE\x81a\x11\xD8V[\x81\x14a\x11\xF9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\x0B\x81a\x11\xE5V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12*Wa\x12)a\x0F\x8FV[[`\0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12HWa\x12Ga\x0F\x94V[[a\x12T\x86\x82\x87\x01a\x11\xAAV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12uWa\x12ta\x0F\x94V[[a\x12\x81\x86\x82\x87\x01a\x11\xAAV[\x92PP`@a\x12\x92\x86\x82\x87\x01a\x11\xFCV[\x91PP\x92P\x92P\x92V[a\x12\xA5\x81a\x0F\xF7V[\x82RPPV[`\0` \x82\x01\x90Pa\x12\xC0`\0\x83\x01\x84a\x12\x9CV[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xDFWa\x12\xDEa\x0F\x8FV[[`\0a\x12\xED\x86\x82\x87\x01a\x0F\xE2V[\x93PP` a\x12\xFE\x86\x82\x87\x01a\x0F\xE2V[\x92PP`@a\x13\x0F\x86\x82\x87\x01a\x10\x18V[\x91PP\x92P\x92P\x92V[a\x13\"\x81a\x11\xD8V[\x82RPPV[`\0` \x82\x01\x90Pa\x13=`\0\x83\x01\x84a\x13\x19V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x13V\x81a\x13CV[\x82RPPV[`\0` \x82\x01\x90Pa\x13q`\0\x83\x01\x84a\x13MV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x13\x8DWa\x13\x8Ca\x0F\x8FV[[`\0a\x13\x9B\x84\x82\x85\x01a\x0F\xE2V[\x91PP\x92\x91PPV[a\x13\xAD\x81a\x13CV[\x81\x14a\x13\xB8W`\0\x80\xFD[PV[`\0\x815\x90Pa\x13\xCA\x81a\x13\xA4V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x13\xEFWa\x13\xEEa\x0F\x8FV[[`\0a\x13\xFD\x8A\x82\x8B\x01a\x0F\xE2V[\x97PP` a\x14\x0E\x8A\x82\x8B\x01a\x0F\xE2V[\x96PP`@a\x14\x1F\x8A\x82\x8B\x01a\x10\x18V[\x95PP``a\x140\x8A\x82\x8B\x01a\x10\x18V[\x94PP`\x80a\x14A\x8A\x82\x8B\x01a\x11\xFCV[\x93PP`\xA0a\x14R\x8A\x82\x8B\x01a\x13\xBBV[\x92PP`\xC0a\x14c\x8A\x82\x8B\x01a\x13\xBBV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x89Wa\x14\x88a\x0F\x8FV[[`\0a\x14\x97\x85\x82\x86\x01a\x0F\xE2V[\x92PP` a\x14\xA8\x85\x82\x86\x01a\x0F\xE2V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x14\xF9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\x0CWa\x15\x0Ba\x14\xB2V[[P\x91\x90PV[\x7FALREADY_INITIALIZED\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x15H`\x13\x83a\x0E\xDEV[\x91Pa\x15S\x82a\x15\x12V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x15w\x81a\x15;V[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x15\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x15\xA3V[a\x15\xEA\x86\x83a\x15\xA3V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x16'a\x16\"a\x16\x1D\x84a\x0F\xF7V[a\x16\x02V[a\x0F\xF7V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x16A\x83a\x16\x0CV[a\x16Ua\x16M\x82a\x16.V[\x84\x84Ta\x15\xB0V[\x82UPPPPV[`\0\x90V[a\x16ja\x16]V[a\x16u\x81\x84\x84a\x168V[PPPV[[\x81\x81\x10\x15a\x16\x99Wa\x16\x8E`\0\x82a\x16bV[`\x01\x81\x01\x90Pa\x16{V[PPV[`\x1F\x82\x11\x15a\x16\xDEWa\x16\xAF\x81a\x15~V[a\x16\xB8\x84a\x15\x93V[\x81\x01` \x85\x10\x15a\x16\xC7W\x81\x90P[a\x16\xDBa\x16\xD3\x85a\x15\x93V[\x83\x01\x82a\x16zV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x17\x01`\0\x19\x84`\x08\x02a\x16\xE3V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x17\x1A\x83\x83a\x16\xF0V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x173\x82a\x0E\xD3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17LWa\x17Ka\x10\xADV[[a\x17V\x82Ta\x14\xE1V[a\x17a\x82\x82\x85a\x16\x9DV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x17\x94W`\0\x84\x15a\x17\x82W\x82\x87\x01Q\x90P[a\x17\x8C\x85\x82a\x17\x0EV[\x86UPa\x17\xF4V[`\x1F\x19\x84\x16a\x17\xA2\x86a\x15~V[`\0[\x82\x81\x10\x15a\x17\xCAW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x17\xA5V[\x86\x83\x10\x15a\x17\xE7W\x84\x89\x01Qa\x17\xE3`\x1F\x89\x16\x82a\x16\xF0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x182`\x17\x83a\x0E\xDEV[\x91Pa\x18=\x82a\x17\xFCV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18a\x81a\x18%V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x18\xA2\x82a\x0F\xF7V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x18\xD4Wa\x18\xD3a\x18hV[[`\x01\x82\x01\x90P\x91\x90PV[a\x18\xE8\x81a\x0F\xB9V[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x19\x03`\0\x83\x01\x89a\x13MV[a\x19\x10` \x83\x01\x88a\x18\xDFV[a\x19\x1D`@\x83\x01\x87a\x18\xDFV[a\x19*``\x83\x01\x86a\x12\x9CV[a\x197`\x80\x83\x01\x85a\x12\x9CV[a\x19D`\xA0\x83\x01\x84a\x12\x9CV[\x97\x96PPPPPPPV[`\0\x81\x90P\x92\x91PPV[\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x19\x90`\x02\x83a\x19OV[\x91Pa\x19\x9B\x82a\x19ZV[`\x02\x82\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x19\xC1a\x19\xBC\x82a\x13CV[a\x19\xA6V[\x82RPPV[`\0a\x19\xD2\x82a\x19\x83V[\x91Pa\x19\xDE\x82\x85a\x19\xB0V[` \x82\x01\x91Pa\x19\xEE\x82\x84a\x19\xB0V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[`\0`\x80\x82\x01\x90Pa\x1A\x13`\0\x83\x01\x87a\x13MV[a\x1A ` \x83\x01\x86a\x13\x19V[a\x1A-`@\x83\x01\x85a\x13MV[a\x1A:``\x83\x01\x84a\x13MV[\x95\x94PPPPPV[\x7FINVALID_SIGNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1Ay`\x0E\x83a\x0E\xDEV[\x91Pa\x1A\x84\x82a\x1ACV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\xA8\x81a\x1AlV[\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0\x81Ta\x1A\xDC\x81a\x14\xE1V[a\x1A\xE6\x81\x86a\x1A\xAFV[\x94P`\x01\x82\x16`\0\x81\x14a\x1B\x01W`\x01\x81\x14a\x1B\x16Wa\x1BIV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x1BIV[a\x1B\x1F\x85a\x1A\xBAV[`\0[\x83\x81\x10\x15a\x1BAW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x1B\"V[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[`\0a\x1B^\x82\x84a\x1A\xCFV[\x91P\x81\x90P\x92\x91PPV[`\0`\xA0\x82\x01\x90Pa\x1B~`\0\x83\x01\x88a\x13MV[a\x1B\x8B` \x83\x01\x87a\x13MV[a\x1B\x98`@\x83\x01\x86a\x13MV[a\x1B\xA5``\x83\x01\x85a\x12\x9CV[a\x1B\xB2`\x80\x83\x01\x84a\x18\xDFV[\x96\x95PPPPPPV[\x7FERC20: subtraction underflow\0\0\0\0`\0\x82\x01RPV[`\0a\x1B\xF2`\x1C\x83a\x0E\xDEV[\x91Pa\x1B\xFD\x82a\x1B\xBCV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C!\x81a\x1B\xE5V[\x90P\x91\x90PV[`\0a\x1C3\x82a\x0F\xF7V[\x91Pa\x1C>\x83a\x0F\xF7V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1CVWa\x1CUa\x18hV[[\x92\x91PPV[`\0a\x1Cg\x82a\x0F\xF7V[\x91Pa\x1Cr\x83a\x0F\xF7V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1C\x8AWa\x1C\x89a\x18hV[[\x92\x91PPV[\x7FERC20: addition overflow\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1C\xC6`\x18\x83a\x0E\xDEV[\x91Pa\x1C\xD1\x82a\x1C\x90V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\xF5\x81a\x1C\xB9V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`Q`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x81\xDA#\xC7]\x1F\x91\xBF\xF0{\xA2;\xA7N\x8E?.\xA9\xFC\xA2I\xAF\xDD\xA8:\x01\xE7/\x06Q\x16hdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKERC20_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c6D\xE5\x15\x11a\0\x8CW\x80c\x95\xD8\x9BA\x11a\0fW\x80c\x95\xD8\x9BA\x14a\x02(W\x80c\xA9\x05\x9C\xBB\x14a\x02FW\x80c\xD5\x05\xAC\xCF\x14a\x02vW\x80c\xDDb\xED>\x14a\x02\x92Wa\0\xCFV[\x80c6D\xE5\x15\x14a\x01\xAAW\x80cp\xA0\x821\x14a\x01\xC8W\x80c~\xCE\xBE\0\x14a\x01\xF8Wa\0\xCFV[\x80c\x06\xFD\xDE\x03\x14a\0\xD4W\x80c\t^\xA7\xB3\x14a\0\xF2W\x80c\x16$\xF6\xC6\x14a\x01\"W\x80c\x18\x16\r\xDD\x14a\x01>W\x80c#\xB8r\xDD\x14a\x01\\W\x80c1<\xE5g\x14a\x01\x8CW[`\0\x80\xFD[a\0\xDCa\x02\xC2V[`@Qa\0\xE9\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x01\x0C`\x04\x806\x03\x81\x01\x90a\x01\x07\x91\x90a\x10-V[a\x03PV[`@Qa\x01\x19\x91\x90a\x10\x88V[`@Q\x80\x91\x03\x90\xF3[a\x01<`\x04\x806\x03\x81\x01\x90a\x017\x91\x90a\x12\x11V[a\x04BV[\0[a\x01Fa\x05\tV[`@Qa\x01S\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[a\x01v`\x04\x806\x03\x81\x01\x90a\x01q\x91\x90a\x12\xC6V[a\x05\x0FV[`@Qa\x01\x83\x91\x90a\x10\x88V[`@Q\x80\x91\x03\x90\xF3[a\x01\x94a\x07\xAEV[`@Qa\x01\xA1\x91\x90a\x13(V[`@Q\x80\x91\x03\x90\xF3[a\x01\xB2a\x07\xC1V[`@Qa\x01\xBF\x91\x90a\x13\\V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE2`\x04\x806\x03\x81\x01\x90a\x01\xDD\x91\x90a\x13wV[a\x07\xE9V[`@Qa\x01\xEF\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[a\x02\x12`\x04\x806\x03\x81\x01\x90a\x02\r\x91\x90a\x13wV[a\x08\x01V[`@Qa\x02\x1F\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[a\x020a\x08\x19V[`@Qa\x02=\x91\x90a\x0FcV[`@Q\x80\x91\x03\x90\xF3[a\x02``\x04\x806\x03\x81\x01\x90a\x02[\x91\x90a\x10-V[a\x08\xA7V[`@Qa\x02m\x91\x90a\x10\x88V[`@Q\x80\x91\x03\x90\xF3[a\x02\x90`\x04\x806\x03\x81\x01\x90a\x02\x8B\x91\x90a\x13\xD0V[a\n0V[\0[a\x02\xAC`\x04\x806\x03\x81\x01\x90a\x02\xA7\x91\x90a\x14rV[a\r/V[`@Qa\x02\xB9\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xF3[`\0\x80Ta\x02\xCF\x90a\x14\xE1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFB\x90a\x14\xE1V[\x80\x15a\x03HW\x80`\x1F\x10a\x03\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03HV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03+W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x81`\x05`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x040\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[`\t`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x04\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x89\x90a\x15^V[`@Q\x80\x91\x03\x90\xFD[\x82`\0\x90\x81a\x04\xA1\x91\x90a\x17*V[P\x81`\x01\x90\x81a\x04\xB1\x91\x90a\x17*V[P\x80`\x02`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa\x04\xD5a\rTV[`\x06\x81\x90UPa\x04\xE3a\rwV[`\x07\x81\x90UP`\x01`\t`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[`\x03T\x81V[`\0\x80`\x05`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x19\x81\x14a\x06%Wa\x05\xA4\x81\x84a\x0E\nV[`\x05`\0\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP[a\x06n`\x04`\0\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84a\x0E\nV[`\x04`\0\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\x06\xFA`\x04`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84a\x0EcV[`\x04`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x85`@Qa\x07\x9A\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3`\x01\x91PP\x93\x92PPPV[`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x06Ta\x07\xCEa\rTV[\x14a\x07\xE0Wa\x07\xDBa\rwV[a\x07\xE4V[`\x07T[\x90P\x90V[`\x04` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[`\x08` R\x80`\0R`@`\0 `\0\x91P\x90PT\x81V[`\x01\x80Ta\x08&\x90a\x14\xE1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08R\x90a\x14\xE1V[\x80\x15a\x08\x9FW\x80`\x1F\x10a\x08tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x9FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x08\xF2`\x04`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x83a\x0E\nV[`\x04`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\t~`\x04`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x83a\x0EcV[`\x04`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\n\x1E\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3`\x01\x90P\x92\x91PPV[B\x84\x10\x15a\nsW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nj\x90a\x18HV[`@Q\x80\x91\x03\x90\xFD[`\0`\x01a\n\x7Fa\x07\xC1V[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x8A\x8A\x8A`\x08`\0\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81T\x80\x92\x91\x90a\n\xF3\x90a\x18\x97V[\x91\x90PU\x8B`@Q` \x01a\x0B\r\x96\x95\x94\x93\x92\x91\x90a\x18\xEEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B4\x92\x91\x90a\x19\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85\x85`@Q`\0\x81R` \x01`@R`@Qa\x0Bj\x94\x93\x92\x91\x90a\x19\xFEV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\x8CW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0C\0WP\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x0C?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C6\x90a\x1A\x8FV[`@Q\x80\x91\x03\x90\xFD[\x85`\x05`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x88`@Qa\r\x1D\x91\x90a\x12\xABV[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\x05` R\x81`\0R`@`\0 ` R\x80`\0R`@`\0 `\0\x91P\x91PPT\x81V[`\0a\x0E\xC9a\x0E\xC1\x90Pa\x0E\xC9\x81\x90Pa\rp\x81c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x90V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\r\xA9\x91\x90a\x1BRV[`@Q\x80\x91\x03\x90 \x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6a\r\xDAa\rTV[0`@Q` \x01a\r\xEF\x95\x94\x93\x92\x91\x90a\x1BiV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81\x83\x10\x15a\x0EOW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EF\x90a\x1C\x08V[`@Q\x80\x91\x03\x90\xFD[\x81\x83a\x0E[\x91\x90a\x1C(V[\x90P\x92\x91PPV[`\0\x80\x82\x84a\x0Er\x91\x90a\x1C\\V[\x90P\x83\x81\x10\x15a\x0E\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xAE\x90a\x1C\xDCV[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0F\x90P\x90V[a\x0E\xD1a\x1C\xFCV[V[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x0F\rW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x0E\xF2V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x0F5\x82a\x0E\xD3V[a\x0F?\x81\x85a\x0E\xDEV[\x93Pa\x0FO\x81\x85` \x86\x01a\x0E\xEFV[a\x0FX\x81a\x0F\x19V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0F}\x81\x84a\x0F*V[\x90P\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0F\xC4\x82a\x0F\x99V[\x90P\x91\x90PV[a\x0F\xD4\x81a\x0F\xB9V[\x81\x14a\x0F\xDFW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0F\xF1\x81a\x0F\xCBV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x10\n\x81a\x0F\xF7V[\x81\x14a\x10\x15W`\0\x80\xFD[PV[`\0\x815\x90Pa\x10'\x81a\x10\x01V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10DWa\x10Ca\x0F\x8FV[[`\0a\x10R\x85\x82\x86\x01a\x0F\xE2V[\x92PP` a\x10c\x85\x82\x86\x01a\x10\x18V[\x91PP\x92P\x92\x90PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x10\x82\x81a\x10mV[\x82RPPV[`\0` \x82\x01\x90Pa\x10\x9D`\0\x83\x01\x84a\x10yV[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x10\xE5\x82a\x0F\x19V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x11\x04Wa\x11\x03a\x10\xADV[[\x80`@RPPPV[`\0a\x11\x17a\x0F\x85V[\x90Pa\x11#\x82\x82a\x10\xDCV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11CWa\x11Ba\x10\xADV[[a\x11L\x82a\x0F\x19V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x11{a\x11v\x84a\x11(V[a\x11\rV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x11\x97Wa\x11\x96a\x10\xA8V[[a\x11\xA2\x84\x82\x85a\x11YV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x11\xBFWa\x11\xBEa\x10\xA3V[[\x815a\x11\xCF\x84\x82` \x86\x01a\x11hV[\x91PP\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x11\xEE\x81a\x11\xD8V[\x81\x14a\x11\xF9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\x0B\x81a\x11\xE5V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12*Wa\x12)a\x0F\x8FV[[`\0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12HWa\x12Ga\x0F\x94V[[a\x12T\x86\x82\x87\x01a\x11\xAAV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12uWa\x12ta\x0F\x94V[[a\x12\x81\x86\x82\x87\x01a\x11\xAAV[\x92PP`@a\x12\x92\x86\x82\x87\x01a\x11\xFCV[\x91PP\x92P\x92P\x92V[a\x12\xA5\x81a\x0F\xF7V[\x82RPPV[`\0` \x82\x01\x90Pa\x12\xC0`\0\x83\x01\x84a\x12\x9CV[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xDFWa\x12\xDEa\x0F\x8FV[[`\0a\x12\xED\x86\x82\x87\x01a\x0F\xE2V[\x93PP` a\x12\xFE\x86\x82\x87\x01a\x0F\xE2V[\x92PP`@a\x13\x0F\x86\x82\x87\x01a\x10\x18V[\x91PP\x92P\x92P\x92V[a\x13\"\x81a\x11\xD8V[\x82RPPV[`\0` \x82\x01\x90Pa\x13=`\0\x83\x01\x84a\x13\x19V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x13V\x81a\x13CV[\x82RPPV[`\0` \x82\x01\x90Pa\x13q`\0\x83\x01\x84a\x13MV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x13\x8DWa\x13\x8Ca\x0F\x8FV[[`\0a\x13\x9B\x84\x82\x85\x01a\x0F\xE2V[\x91PP\x92\x91PPV[a\x13\xAD\x81a\x13CV[\x81\x14a\x13\xB8W`\0\x80\xFD[PV[`\0\x815\x90Pa\x13\xCA\x81a\x13\xA4V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x13\xEFWa\x13\xEEa\x0F\x8FV[[`\0a\x13\xFD\x8A\x82\x8B\x01a\x0F\xE2V[\x97PP` a\x14\x0E\x8A\x82\x8B\x01a\x0F\xE2V[\x96PP`@a\x14\x1F\x8A\x82\x8B\x01a\x10\x18V[\x95PP``a\x140\x8A\x82\x8B\x01a\x10\x18V[\x94PP`\x80a\x14A\x8A\x82\x8B\x01a\x11\xFCV[\x93PP`\xA0a\x14R\x8A\x82\x8B\x01a\x13\xBBV[\x92PP`\xC0a\x14c\x8A\x82\x8B\x01a\x13\xBBV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x89Wa\x14\x88a\x0F\x8FV[[`\0a\x14\x97\x85\x82\x86\x01a\x0F\xE2V[\x92PP` a\x14\xA8\x85\x82\x86\x01a\x0F\xE2V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x14\xF9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\x0CWa\x15\x0Ba\x14\xB2V[[P\x91\x90PV[\x7FALREADY_INITIALIZED\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x15H`\x13\x83a\x0E\xDEV[\x91Pa\x15S\x82a\x15\x12V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x15w\x81a\x15;V[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x15\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x15\xA3V[a\x15\xEA\x86\x83a\x15\xA3V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x16'a\x16\"a\x16\x1D\x84a\x0F\xF7V[a\x16\x02V[a\x0F\xF7V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x16A\x83a\x16\x0CV[a\x16Ua\x16M\x82a\x16.V[\x84\x84Ta\x15\xB0V[\x82UPPPPV[`\0\x90V[a\x16ja\x16]V[a\x16u\x81\x84\x84a\x168V[PPPV[[\x81\x81\x10\x15a\x16\x99Wa\x16\x8E`\0\x82a\x16bV[`\x01\x81\x01\x90Pa\x16{V[PPV[`\x1F\x82\x11\x15a\x16\xDEWa\x16\xAF\x81a\x15~V[a\x16\xB8\x84a\x15\x93V[\x81\x01` \x85\x10\x15a\x16\xC7W\x81\x90P[a\x16\xDBa\x16\xD3\x85a\x15\x93V[\x83\x01\x82a\x16zV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x17\x01`\0\x19\x84`\x08\x02a\x16\xE3V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x17\x1A\x83\x83a\x16\xF0V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x173\x82a\x0E\xD3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17LWa\x17Ka\x10\xADV[[a\x17V\x82Ta\x14\xE1V[a\x17a\x82\x82\x85a\x16\x9DV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x17\x94W`\0\x84\x15a\x17\x82W\x82\x87\x01Q\x90P[a\x17\x8C\x85\x82a\x17\x0EV[\x86UPa\x17\xF4V[`\x1F\x19\x84\x16a\x17\xA2\x86a\x15~V[`\0[\x82\x81\x10\x15a\x17\xCAW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x17\xA5V[\x86\x83\x10\x15a\x17\xE7W\x84\x89\x01Qa\x17\xE3`\x1F\x89\x16\x82a\x16\xF0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x182`\x17\x83a\x0E\xDEV[\x91Pa\x18=\x82a\x17\xFCV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18a\x81a\x18%V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x18\xA2\x82a\x0F\xF7V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x18\xD4Wa\x18\xD3a\x18hV[[`\x01\x82\x01\x90P\x91\x90PV[a\x18\xE8\x81a\x0F\xB9V[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\x19\x03`\0\x83\x01\x89a\x13MV[a\x19\x10` \x83\x01\x88a\x18\xDFV[a\x19\x1D`@\x83\x01\x87a\x18\xDFV[a\x19*``\x83\x01\x86a\x12\x9CV[a\x197`\x80\x83\x01\x85a\x12\x9CV[a\x19D`\xA0\x83\x01\x84a\x12\x9CV[\x97\x96PPPPPPPV[`\0\x81\x90P\x92\x91PPV[\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x19\x90`\x02\x83a\x19OV[\x91Pa\x19\x9B\x82a\x19ZV[`\x02\x82\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x19\xC1a\x19\xBC\x82a\x13CV[a\x19\xA6V[\x82RPPV[`\0a\x19\xD2\x82a\x19\x83V[\x91Pa\x19\xDE\x82\x85a\x19\xB0V[` \x82\x01\x91Pa\x19\xEE\x82\x84a\x19\xB0V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[`\0`\x80\x82\x01\x90Pa\x1A\x13`\0\x83\x01\x87a\x13MV[a\x1A ` \x83\x01\x86a\x13\x19V[a\x1A-`@\x83\x01\x85a\x13MV[a\x1A:``\x83\x01\x84a\x13MV[\x95\x94PPPPPV[\x7FINVALID_SIGNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1Ay`\x0E\x83a\x0E\xDEV[\x91Pa\x1A\x84\x82a\x1ACV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\xA8\x81a\x1AlV[\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0\x81Ta\x1A\xDC\x81a\x14\xE1V[a\x1A\xE6\x81\x86a\x1A\xAFV[\x94P`\x01\x82\x16`\0\x81\x14a\x1B\x01W`\x01\x81\x14a\x1B\x16Wa\x1BIV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x1BIV[a\x1B\x1F\x85a\x1A\xBAV[`\0[\x83\x81\x10\x15a\x1BAW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x1B\"V[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[`\0a\x1B^\x82\x84a\x1A\xCFV[\x91P\x81\x90P\x92\x91PPV[`\0`\xA0\x82\x01\x90Pa\x1B~`\0\x83\x01\x88a\x13MV[a\x1B\x8B` \x83\x01\x87a\x13MV[a\x1B\x98`@\x83\x01\x86a\x13MV[a\x1B\xA5``\x83\x01\x85a\x12\x9CV[a\x1B\xB2`\x80\x83\x01\x84a\x18\xDFV[\x96\x95PPPPPPV[\x7FERC20: subtraction underflow\0\0\0\0`\0\x82\x01RPV[`\0a\x1B\xF2`\x1C\x83a\x0E\xDEV[\x91Pa\x1B\xFD\x82a\x1B\xBCV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C!\x81a\x1B\xE5V[\x90P\x91\x90PV[`\0a\x1C3\x82a\x0F\xF7V[\x91Pa\x1C>\x83a\x0F\xF7V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1CVWa\x1CUa\x18hV[[\x92\x91PPV[`\0a\x1Cg\x82a\x0F\xF7V[\x91Pa\x1Cr\x83a\x0F\xF7V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1C\x8AWa\x1C\x89a\x18hV[[\x92\x91PPV[\x7FERC20: addition overflow\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1C\xC6`\x18\x83a\x0E\xDEV[\x91Pa\x1C\xD1\x82a\x1C\x90V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\xF5\x81a\x1C\xB9V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`Q`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x81\xDA#\xC7]\x1F\x91\xBF\xF0{\xA2;\xA7N\x8E?.\xA9\xFC\xA2I\xAF\xDD\xA8:\x01\xE7/\x06Q\x16hdsolcC\0\x08\x15\x003";
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
