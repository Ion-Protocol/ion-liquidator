pub use ion_registry::*;
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
pub mod ion_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gemJoins"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract GemJoin[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_depositContracts"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("depositContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositContracts"),
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
                    ::std::borrow::ToOwned::to_owned("gemJoins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gemJoins"),
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
                                        ::std::borrow::ToOwned::to_owned("contract GemJoin"),
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
                    ::std::borrow::ToOwned::to_owned("setDepositContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDepositContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("depositContract"),
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
                    ::std::borrow::ToOwned::to_owned("setGemJoin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGemJoin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ilkIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gemJoin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GemJoin"),
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
            ]),
            events: ::core::convert::From::from([
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
    pub static IONREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x0E\xDD8\x03\x80b\0\x0E\xDD\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x06\x19V[\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\xADW`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\0\xA4\x91\x90b\0\x06\xC4V[`@Q\x80\x91\x03\x90\xFD[b\0\0\xBE\x81b\0\x01\x0F` \x1B` \x1CV[P\x81Q\x83Q\x14b\0\0\xD4Wb\0\0\xD3b\0\x06\xE1V[[\x82`\x01\x90\x80Q\x90` \x01\x90b\0\0\xEC\x92\x91\x90b\0\x01\xD3V[P\x81`\x02\x90\x80Q\x90` \x01\x90b\0\x01\x05\x92\x91\x90b\0\x02bV[PPPPb\0\x07\x10V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x02OW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x02NW\x82Q\x82`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x91` \x01\x91\x90`\x01\x01\x90b\0\x01\xF4V[[P\x90Pb\0\x02^\x91\x90b\0\x02\xF1V[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x02\xDEW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x02\xDDW\x82Q\x82`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x91` \x01\x91\x90`\x01\x01\x90b\0\x02\x83V[[P\x90Pb\0\x02\xED\x91\x90b\0\x02\xF1V[P\x90V[[\x80\x82\x11\x15b\0\x03\x0CW`\0\x81`\0\x90UP`\x01\x01b\0\x02\xF2V[P\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x03t\x82b\0\x03)V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x03\x96Wb\0\x03\x95b\0\x03:V[[\x80`@RPPPV[`\0b\0\x03\xABb\0\x03\x10V[\x90Pb\0\x03\xB9\x82\x82b\0\x03iV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x03\xDCWb\0\x03\xDBb\0\x03:V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x04\x1F\x82b\0\x03\xF2V[\x90P\x91\x90PV[`\0b\0\x043\x82b\0\x04\x12V[\x90P\x91\x90PV[b\0\x04E\x81b\0\x04&V[\x81\x14b\0\x04QW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x04e\x81b\0\x04:V[\x92\x91PPV[`\0b\0\x04\x82b\0\x04|\x84b\0\x03\xBEV[b\0\x03\x9FV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x04\xA8Wb\0\x04\xA7b\0\x03\xEDV[[\x83[\x81\x81\x10\x15b\0\x04\xD5W\x80b\0\x04\xC0\x88\x82b\0\x04TV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x04\xAAV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x04\xF7Wb\0\x04\xF6b\0\x03$V[[\x81Qb\0\x05\t\x84\x82` \x86\x01b\0\x04kV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x050Wb\0\x05/b\0\x03:V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[b\0\x05L\x81b\0\x04\x12V[\x81\x14b\0\x05XW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x05l\x81b\0\x05AV[\x92\x91PPV[`\0b\0\x05\x89b\0\x05\x83\x84b\0\x05\x12V[b\0\x03\x9FV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15b\0\x05\xAFWb\0\x05\xAEb\0\x03\xEDV[[\x83[\x81\x81\x10\x15b\0\x05\xDCW\x80b\0\x05\xC7\x88\x82b\0\x05[V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pb\0\x05\xB1V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x05\xFEWb\0\x05\xFDb\0\x03$V[[\x81Qb\0\x06\x10\x84\x82` \x86\x01b\0\x05rV[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x065Wb\0\x064b\0\x03\x1AV[[`\0\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x06VWb\0\x06Ub\0\x03\x1FV[[b\0\x06d\x86\x82\x87\x01b\0\x04\xDFV[\x93PP` \x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x06\x88Wb\0\x06\x87b\0\x03\x1FV[[b\0\x06\x96\x86\x82\x87\x01b\0\x05\xE6V[\x92PP`@b\0\x06\xA9\x86\x82\x87\x01b\0\x05[V[\x91PP\x92P\x92P\x92V[b\0\x06\xBE\x81b\0\x04\x12V[\x82RPPV[`\0` \x82\x01\x90Pb\0\x06\xDB`\0\x83\x01\x84b\0\x06\xB3V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD[a\x07\xBD\x80b\0\x07 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xD8W\x80c\x9B\xC1\xE3\x85\x14a\0\xF6W\x80c\xF1\xAD\xC9\xE1\x14a\x01&W\x80c\xF2\xFD\xE3\x8B\x14a\x01BWa\0}V[\x80c\x087\x0Cq\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\x9EW\x80c~\xB4\xFAs\x14a\0\xA8W[`\0\x80\xFD[a\0\x9C`\x04\x806\x03\x81\x01\x90a\0\x97\x91\x90a\x05fV[a\x01^V[\0[a\0\xA6a\x01\xCAV[\0[a\0\xC2`\x04\x806\x03\x81\x01\x90a\0\xBD\x91\x90a\x05\xDCV[a\x01\xDEV[`@Qa\0\xCF\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xF3[a\0\xE0a\x02\x1DV[`@Qa\0\xED\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xF3[a\x01\x10`\x04\x806\x03\x81\x01\x90a\x01\x0B\x91\x90a\x05\xDCV[a\x02FV[`@Qa\x01\x1D\x91\x90a\x06\x92V[`@Q\x80\x91\x03\x90\xF3[a\x01@`\x04\x806\x03\x81\x01\x90a\x01;\x91\x90a\x06\xEBV[a\x02\x85V[\0[a\x01\\`\x04\x806\x03\x81\x01\x90a\x01W\x91\x90a\x07+V[a\x02\xF1V[\0[a\x01fa\x03wV[\x80`\x02\x83`\xFF\x16\x81T\x81\x10a\x01~Wa\x01}a\x07XV[[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\x01\xD2a\x03wV[a\x01\xDC`\0a\x03\xFEV[V[`\x02\x81\x81T\x81\x10a\x01\xEEW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\x01\x81\x81T\x81\x10a\x02VW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x8Da\x03wV[\x80`\x01\x83`\xFF\x16\x81T\x81\x10a\x02\xA5Wa\x02\xA4a\x07XV[[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\x02\xF9a\x03wV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x03kW`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03b\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xFD[a\x03t\x81a\x03\xFEV[PV[a\x03\x7Fa\x04\xC2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\x9Da\x02\x1DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03\xFCWa\x03\xC0a\x04\xC2V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xF3\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x04\xE5\x81a\x04\xCFV[\x81\x14a\x04\xF0W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\x02\x81a\x04\xDCV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x053\x82a\x05\x08V[\x90P\x91\x90PV[a\x05C\x81a\x05(V[\x81\x14a\x05NW`\0\x80\xFD[PV[`\0\x815\x90Pa\x05`\x81a\x05:V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05}Wa\x05|a\x04\xCAV[[`\0a\x05\x8B\x85\x82\x86\x01a\x04\xF3V[\x92PP` a\x05\x9C\x85\x82\x86\x01a\x05QV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[a\x05\xB9\x81a\x05\xA6V[\x81\x14a\x05\xC4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\xD6\x81a\x05\xB0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x05\xF2Wa\x05\xF1a\x04\xCAV[[`\0a\x06\0\x84\x82\x85\x01a\x05\xC7V[\x91PP\x92\x91PPV[a\x06\x12\x81a\x05(V[\x82RPPV[`\0` \x82\x01\x90Pa\x06-`\0\x83\x01\x84a\x06\tV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x06Xa\x06Sa\x06N\x84a\x05\x08V[a\x063V[a\x05\x08V[\x90P\x91\x90PV[`\0a\x06j\x82a\x06=V[\x90P\x91\x90PV[`\0a\x06|\x82a\x06_V[\x90P\x91\x90PV[a\x06\x8C\x81a\x06qV[\x82RPPV[`\0` \x82\x01\x90Pa\x06\xA7`\0\x83\x01\x84a\x06\x83V[\x92\x91PPV[`\0a\x06\xB8\x82a\x05(V[\x90P\x91\x90PV[a\x06\xC8\x81a\x06\xADV[\x81\x14a\x06\xD3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x06\xE5\x81a\x06\xBFV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\x02Wa\x07\x01a\x04\xCAV[[`\0a\x07\x10\x85\x82\x86\x01a\x04\xF3V[\x92PP` a\x07!\x85\x82\x86\x01a\x06\xD6V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x07AWa\x07@a\x04\xCAV[[`\0a\x07O\x84\x82\x85\x01a\x05QV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xB5\x96\xE0\x99c\x1F\x02\x91\x14G\x17\xCC\xF2T\xEB\x95\xB7\xAC\xB6\xBB\xA6\0Gw\xE7\x19Z\xAE(3\xDB\xB0dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static IONREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xD8W\x80c\x9B\xC1\xE3\x85\x14a\0\xF6W\x80c\xF1\xAD\xC9\xE1\x14a\x01&W\x80c\xF2\xFD\xE3\x8B\x14a\x01BWa\0}V[\x80c\x087\x0Cq\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\x9EW\x80c~\xB4\xFAs\x14a\0\xA8W[`\0\x80\xFD[a\0\x9C`\x04\x806\x03\x81\x01\x90a\0\x97\x91\x90a\x05fV[a\x01^V[\0[a\0\xA6a\x01\xCAV[\0[a\0\xC2`\x04\x806\x03\x81\x01\x90a\0\xBD\x91\x90a\x05\xDCV[a\x01\xDEV[`@Qa\0\xCF\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xF3[a\0\xE0a\x02\x1DV[`@Qa\0\xED\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xF3[a\x01\x10`\x04\x806\x03\x81\x01\x90a\x01\x0B\x91\x90a\x05\xDCV[a\x02FV[`@Qa\x01\x1D\x91\x90a\x06\x92V[`@Q\x80\x91\x03\x90\xF3[a\x01@`\x04\x806\x03\x81\x01\x90a\x01;\x91\x90a\x06\xEBV[a\x02\x85V[\0[a\x01\\`\x04\x806\x03\x81\x01\x90a\x01W\x91\x90a\x07+V[a\x02\xF1V[\0[a\x01fa\x03wV[\x80`\x02\x83`\xFF\x16\x81T\x81\x10a\x01~Wa\x01}a\x07XV[[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\x01\xD2a\x03wV[a\x01\xDC`\0a\x03\xFEV[V[`\x02\x81\x81T\x81\x10a\x01\xEEW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\x01\x81\x81T\x81\x10a\x02VW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x8Da\x03wV[\x80`\x01\x83`\xFF\x16\x81T\x81\x10a\x02\xA5Wa\x02\xA4a\x07XV[[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[a\x02\xF9a\x03wV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x03kW`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03b\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xFD[a\x03t\x81a\x03\xFEV[PV[a\x03\x7Fa\x04\xC2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\x9Da\x02\x1DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03\xFCWa\x03\xC0a\x04\xC2V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xF3\x91\x90a\x06\x18V[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x04\xE5\x81a\x04\xCFV[\x81\x14a\x04\xF0W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\x02\x81a\x04\xDCV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x053\x82a\x05\x08V[\x90P\x91\x90PV[a\x05C\x81a\x05(V[\x81\x14a\x05NW`\0\x80\xFD[PV[`\0\x815\x90Pa\x05`\x81a\x05:V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05}Wa\x05|a\x04\xCAV[[`\0a\x05\x8B\x85\x82\x86\x01a\x04\xF3V[\x92PP` a\x05\x9C\x85\x82\x86\x01a\x05QV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[a\x05\xB9\x81a\x05\xA6V[\x81\x14a\x05\xC4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\xD6\x81a\x05\xB0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x05\xF2Wa\x05\xF1a\x04\xCAV[[`\0a\x06\0\x84\x82\x85\x01a\x05\xC7V[\x91PP\x92\x91PPV[a\x06\x12\x81a\x05(V[\x82RPPV[`\0` \x82\x01\x90Pa\x06-`\0\x83\x01\x84a\x06\tV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x06Xa\x06Sa\x06N\x84a\x05\x08V[a\x063V[a\x05\x08V[\x90P\x91\x90PV[`\0a\x06j\x82a\x06=V[\x90P\x91\x90PV[`\0a\x06|\x82a\x06_V[\x90P\x91\x90PV[a\x06\x8C\x81a\x06qV[\x82RPPV[`\0` \x82\x01\x90Pa\x06\xA7`\0\x83\x01\x84a\x06\x83V[\x92\x91PPV[`\0a\x06\xB8\x82a\x05(V[\x90P\x91\x90PV[a\x06\xC8\x81a\x06\xADV[\x81\x14a\x06\xD3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x06\xE5\x81a\x06\xBFV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\x02Wa\x07\x01a\x04\xCAV[[`\0a\x07\x10\x85\x82\x86\x01a\x04\xF3V[\x92PP` a\x07!\x85\x82\x86\x01a\x06\xD6V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x07AWa\x07@a\x04\xCAV[[`\0a\x07O\x84\x82\x85\x01a\x05QV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xB5\x96\xE0\x99c\x1F\x02\x91\x14G\x17\xCC\xF2T\xEB\x95\xB7\xAC\xB6\xBB\xA6\0Gw\xE7\x19Z\xAE(3\xDB\xB0dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static IONREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct IonRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IonRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IonRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IonRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IonRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IonRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IonRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IONREGISTRY_ABI.clone(),
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
                IONREGISTRY_ABI.clone(),
                IONREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `depositContracts` (0x7eb4fa73) function
        pub fn deposit_contracts(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([126, 180, 250, 115], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gemJoins` (0x9bc1e385) function
        pub fn gem_joins(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([155, 193, 227, 133], p0)
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDepositContract` (0x08370c71) function
        pub fn set_deposit_contract(
            &self,
            ilk_index: u8,
            deposit_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 55, 12, 113], (ilk_index, deposit_contract))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGemJoin` (0xf1adc9e1) function
        pub fn set_gem_join(
            &self,
            ilk_index: u8,
            gem_join: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 173, 201, 225], (ilk_index, gem_join))
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
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IonRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub enum IonRegistryErrors {
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IonRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
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
    impl ::ethers::core::abi::AbiEncode for IonRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
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
    impl ::ethers::contract::ContractRevert for IonRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
    impl ::core::fmt::Display for IonRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
    impl ::core::convert::From<::std::string::String> for IonRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for IonRegistryErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for IonRegistryErrors {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `depositContracts` function with signature `depositContracts(uint256)` and selector `0x7eb4fa73`
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
    #[ethcall(name = "depositContracts", abi = "depositContracts(uint256)")]
    pub struct DepositContractsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `gemJoins` function with signature `gemJoins(uint256)` and selector `0x9bc1e385`
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
    #[ethcall(name = "gemJoins", abi = "gemJoins(uint256)")]
    pub struct GemJoinsCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `setDepositContract` function with signature `setDepositContract(uint8,address)` and selector `0x08370c71`
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
    #[ethcall(name = "setDepositContract", abi = "setDepositContract(uint8,address)")]
    pub struct SetDepositContractCall {
        pub ilk_index: u8,
        pub deposit_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGemJoin` function with signature `setGemJoin(uint8,address)` and selector `0xf1adc9e1`
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
    #[ethcall(name = "setGemJoin", abi = "setGemJoin(uint8,address)")]
    pub struct SetGemJoinCall {
        pub ilk_index: u8,
        pub gem_join: ::ethers::core::types::Address,
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
    pub enum IonRegistryCalls {
        DepositContracts(DepositContractsCall),
        GemJoins(GemJoinsCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDepositContract(SetDepositContractCall),
        SetGemJoin(SetGemJoinCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for IonRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositContracts(decoded));
            }
            if let Ok(decoded) = <GemJoinsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GemJoins(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetDepositContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDepositContract(decoded));
            }
            if let Ok(decoded) = <SetGemJoinCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetGemJoin(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IonRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GemJoins(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDepositContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGemJoin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IonRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GemJoins(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGemJoin(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositContractsCall> for IonRegistryCalls {
        fn from(value: DepositContractsCall) -> Self {
            Self::DepositContracts(value)
        }
    }
    impl ::core::convert::From<GemJoinsCall> for IonRegistryCalls {
        fn from(value: GemJoinsCall) -> Self {
            Self::GemJoins(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for IonRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for IonRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetDepositContractCall> for IonRegistryCalls {
        fn from(value: SetDepositContractCall) -> Self {
            Self::SetDepositContract(value)
        }
    }
    impl ::core::convert::From<SetGemJoinCall> for IonRegistryCalls {
        fn from(value: SetGemJoinCall) -> Self {
            Self::SetGemJoin(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for IonRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `depositContracts` function with signature `depositContracts(uint256)` and selector `0x7eb4fa73`
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
    pub struct DepositContractsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `gemJoins` function with signature `gemJoins(uint256)` and selector `0x9bc1e385`
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
    pub struct GemJoinsReturn(pub ::ethers::core::types::Address);
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
}
