pub use i_access_control_default_admin_rules::*;
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
pub mod i_access_control_default_admin_rules {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptDefaultAdminTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "acceptDefaultAdminTransfer",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beginDefaultAdminTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beginDefaultAdminTransfer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("cancelDefaultAdminTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cancelDefaultAdminTransfer",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeDefaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeDefaultAdminDelay",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
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
                    ::std::borrow::ToOwned::to_owned("defaultAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("defaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultAdminDelay"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultAdminDelayIncreaseWait"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultAdminDelayIncreaseWait",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
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
                    ::std::borrow::ToOwned::to_owned("pendingDefaultAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pendingDefaultAdmin",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("acceptSchedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingDefaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pendingDefaultAdminDelay",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("effectSchedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
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
                    ::std::borrow::ToOwned::to_owned("rollbackDefaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rollbackDefaultAdminDelay",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("DefaultAdminDelayChangeCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminDelayChangeCanceled",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DefaultAdminDelayChangeScheduled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminDelayChangeScheduled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("effectSchedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DefaultAdminTransferCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminTransferCanceled",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DefaultAdminTransferScheduled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminTransferScheduled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("acceptSchedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
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
                    ::std::borrow::ToOwned::to_owned(
                        "AccessControlEnforcedDefaultAdminDelay",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlEnforcedDefaultAdminDelay",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("schedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AccessControlEnforcedDefaultAdminRules",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlEnforcedDefaultAdminRules",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlInvalidDefaultAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlInvalidDefaultAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultAdmin"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IACCESSCONTROLDEFAULTADMINRULES_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IAccessControlDefaultAdminRules<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAccessControlDefaultAdminRules<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAccessControlDefaultAdminRules<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAccessControlDefaultAdminRules<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAccessControlDefaultAdminRules<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IAccessControlDefaultAdminRules))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAccessControlDefaultAdminRules<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IACCESSCONTROLDEFAULTADMINRULES_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `acceptDefaultAdminTransfer` (0xcefc1429) function
        pub fn accept_default_admin_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 252, 20, 41], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beginDefaultAdminTransfer` (0x634e93da) function
        pub fn begin_default_admin_transfer(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 78, 147, 218], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelDefaultAdminTransfer` (0xd602b9fd) function
        pub fn cancel_default_admin_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 2, 185, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeDefaultAdminDelay` (0x649a5ec7) function
        pub fn change_default_admin_delay(
            &self,
            new_delay: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 154, 94, 199], new_delay)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdmin` (0x84ef8ffc) function
        pub fn default_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([132, 239, 143, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdminDelay` (0xcc8463c8) function
        pub fn default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([204, 132, 99, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdminDelayIncreaseWait` (0x022d63fb) function
        pub fn default_admin_delay_increase_wait(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([2, 45, 99, 251], ())
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
        ///Calls the contract's `pendingDefaultAdmin` (0xcf6eefb7) function
        pub fn pending_default_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u64),
        > {
            self.0
                .method_hash([207, 110, 239, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingDefaultAdminDelay` (0xa1eda53c) function
        pub fn pending_default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([161, 237, 165, 60], ())
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
        ///Calls the contract's `rollbackDefaultAdminDelay` (0x0aa6220b) function
        pub fn rollback_default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 166, 34, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DefaultAdminDelayChangeCanceled` event
        pub fn default_admin_delay_change_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminDelayChangeCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminDelayChangeScheduled` event
        pub fn default_admin_delay_change_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminDelayChangeScheduledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminTransferCanceled` event
        pub fn default_admin_transfer_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminTransferScheduled` event
        pub fn default_admin_transfer_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferScheduledFilter,
        > {
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IAccessControlDefaultAdminRulesEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IAccessControlDefaultAdminRules<M> {
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
    ///Custom Error type `AccessControlEnforcedDefaultAdminDelay` with signature `AccessControlEnforcedDefaultAdminDelay(uint48)` and selector `0x19ca5ebb`
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
        name = "AccessControlEnforcedDefaultAdminDelay",
        abi = "AccessControlEnforcedDefaultAdminDelay(uint48)"
    )]
    pub struct AccessControlEnforcedDefaultAdminDelay {
        pub schedule: u64,
    }
    ///Custom Error type `AccessControlEnforcedDefaultAdminRules` with signature `AccessControlEnforcedDefaultAdminRules()` and selector `0x3fc3c27a`
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
        name = "AccessControlEnforcedDefaultAdminRules",
        abi = "AccessControlEnforcedDefaultAdminRules()"
    )]
    pub struct AccessControlEnforcedDefaultAdminRules;
    ///Custom Error type `AccessControlInvalidDefaultAdmin` with signature `AccessControlInvalidDefaultAdmin(address)` and selector `0xc22c8022`
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
        name = "AccessControlInvalidDefaultAdmin",
        abi = "AccessControlInvalidDefaultAdmin(address)"
    )]
    pub struct AccessControlInvalidDefaultAdmin {
        pub default_admin: ::ethers::core::types::Address,
    }
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
    pub enum IAccessControlDefaultAdminRulesErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlEnforcedDefaultAdminDelay(AccessControlEnforcedDefaultAdminDelay),
        AccessControlEnforcedDefaultAdminRules(AccessControlEnforcedDefaultAdminRules),
        AccessControlInvalidDefaultAdmin(AccessControlInvalidDefaultAdmin),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IAccessControlDefaultAdminRulesErrors {
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
            if let Ok(decoded) = <AccessControlEnforcedDefaultAdminDelay as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlEnforcedDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <AccessControlEnforcedDefaultAdminRules as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlEnforcedDefaultAdminRules(decoded));
            }
            if let Ok(decoded) = <AccessControlInvalidDefaultAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlInvalidDefaultAdmin(decoded));
            }
            if let Ok(decoded) = <AccessControlUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlUnauthorizedAccount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAccessControlDefaultAdminRulesErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlEnforcedDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlEnforcedDefaultAdminRules(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlInvalidDefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IAccessControlDefaultAdminRulesErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessControlBadConfirmation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlEnforcedDefaultAdminDelay as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlEnforcedDefaultAdminRules as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlInvalidDefaultAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IAccessControlDefaultAdminRulesErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlEnforcedDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlEnforcedDefaultAdminRules(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlInvalidDefaultAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for IAccessControlDefaultAdminRulesErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation>
    for IAccessControlDefaultAdminRulesErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlEnforcedDefaultAdminDelay>
    for IAccessControlDefaultAdminRulesErrors {
        fn from(value: AccessControlEnforcedDefaultAdminDelay) -> Self {
            Self::AccessControlEnforcedDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<AccessControlEnforcedDefaultAdminRules>
    for IAccessControlDefaultAdminRulesErrors {
        fn from(value: AccessControlEnforcedDefaultAdminRules) -> Self {
            Self::AccessControlEnforcedDefaultAdminRules(value)
        }
    }
    impl ::core::convert::From<AccessControlInvalidDefaultAdmin>
    for IAccessControlDefaultAdminRulesErrors {
        fn from(value: AccessControlInvalidDefaultAdmin) -> Self {
            Self::AccessControlInvalidDefaultAdmin(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount>
    for IAccessControlDefaultAdminRulesErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
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
        name = "DefaultAdminDelayChangeCanceled",
        abi = "DefaultAdminDelayChangeCanceled()"
    )]
    pub struct DefaultAdminDelayChangeCanceledFilter;
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
        name = "DefaultAdminDelayChangeScheduled",
        abi = "DefaultAdminDelayChangeScheduled(uint48,uint48)"
    )]
    pub struct DefaultAdminDelayChangeScheduledFilter {
        pub new_delay: u64,
        pub effect_schedule: u64,
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
        name = "DefaultAdminTransferCanceled",
        abi = "DefaultAdminTransferCanceled()"
    )]
    pub struct DefaultAdminTransferCanceledFilter;
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
        name = "DefaultAdminTransferScheduled",
        abi = "DefaultAdminTransferScheduled(address,uint48)"
    )]
    pub struct DefaultAdminTransferScheduledFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers::core::types::Address,
        pub accept_schedule: u64,
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
    pub enum IAccessControlDefaultAdminRulesEvents {
        DefaultAdminDelayChangeCanceledFilter(DefaultAdminDelayChangeCanceledFilter),
        DefaultAdminDelayChangeScheduledFilter(DefaultAdminDelayChangeScheduledFilter),
        DefaultAdminTransferCanceledFilter(DefaultAdminTransferCanceledFilter),
        DefaultAdminTransferScheduledFilter(DefaultAdminTransferScheduledFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IAccessControlDefaultAdminRulesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DefaultAdminDelayChangeCanceledFilter::decode_log(log) {
                return Ok(
                    IAccessControlDefaultAdminRulesEvents::DefaultAdminDelayChangeCanceledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DefaultAdminDelayChangeScheduledFilter::decode_log(
                log,
            ) {
                return Ok(
                    IAccessControlDefaultAdminRulesEvents::DefaultAdminDelayChangeScheduledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DefaultAdminTransferCanceledFilter::decode_log(log) {
                return Ok(
                    IAccessControlDefaultAdminRulesEvents::DefaultAdminTransferCanceledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DefaultAdminTransferScheduledFilter::decode_log(log) {
                return Ok(
                    IAccessControlDefaultAdminRulesEvents::DefaultAdminTransferScheduledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(
                    IAccessControlDefaultAdminRulesEvents::RoleAdminChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(
                    IAccessControlDefaultAdminRulesEvents::RoleGrantedFilter(decoded),
                );
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(
                    IAccessControlDefaultAdminRulesEvents::RoleRevokedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAccessControlDefaultAdminRulesEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminDelayChangeCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminDelayChangeScheduledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminTransferCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminTransferScheduledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeCanceledFilter>
    for IAccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminDelayChangeCanceledFilter) -> Self {
            Self::DefaultAdminDelayChangeCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeScheduledFilter>
    for IAccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminDelayChangeScheduledFilter) -> Self {
            Self::DefaultAdminDelayChangeScheduledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferCanceledFilter>
    for IAccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminTransferCanceledFilter) -> Self {
            Self::DefaultAdminTransferCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferScheduledFilter>
    for IAccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminTransferScheduledFilter) -> Self {
            Self::DefaultAdminTransferScheduledFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter>
    for IAccessControlDefaultAdminRulesEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter>
    for IAccessControlDefaultAdminRulesEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter>
    for IAccessControlDefaultAdminRulesEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptDefaultAdminTransfer` function with signature `acceptDefaultAdminTransfer()` and selector `0xcefc1429`
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
    #[ethcall(name = "acceptDefaultAdminTransfer", abi = "acceptDefaultAdminTransfer()")]
    pub struct AcceptDefaultAdminTransferCall;
    ///Container type for all input parameters for the `beginDefaultAdminTransfer` function with signature `beginDefaultAdminTransfer(address)` and selector `0x634e93da`
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
        name = "beginDefaultAdminTransfer",
        abi = "beginDefaultAdminTransfer(address)"
    )]
    pub struct BeginDefaultAdminTransferCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `cancelDefaultAdminTransfer` function with signature `cancelDefaultAdminTransfer()` and selector `0xd602b9fd`
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
    #[ethcall(name = "cancelDefaultAdminTransfer", abi = "cancelDefaultAdminTransfer()")]
    pub struct CancelDefaultAdminTransferCall;
    ///Container type for all input parameters for the `changeDefaultAdminDelay` function with signature `changeDefaultAdminDelay(uint48)` and selector `0x649a5ec7`
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
    #[ethcall(name = "changeDefaultAdminDelay", abi = "changeDefaultAdminDelay(uint48)")]
    pub struct ChangeDefaultAdminDelayCall {
        pub new_delay: u64,
    }
    ///Container type for all input parameters for the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
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
    #[ethcall(name = "defaultAdmin", abi = "defaultAdmin()")]
    pub struct DefaultAdminCall;
    ///Container type for all input parameters for the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
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
    #[ethcall(name = "defaultAdminDelay", abi = "defaultAdminDelay()")]
    pub struct DefaultAdminDelayCall;
    ///Container type for all input parameters for the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
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
        name = "defaultAdminDelayIncreaseWait",
        abi = "defaultAdminDelayIncreaseWait()"
    )]
    pub struct DefaultAdminDelayIncreaseWaitCall;
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
    ///Container type for all input parameters for the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
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
    #[ethcall(name = "pendingDefaultAdmin", abi = "pendingDefaultAdmin()")]
    pub struct PendingDefaultAdminCall;
    ///Container type for all input parameters for the `pendingDefaultAdminDelay` function with signature `pendingDefaultAdminDelay()` and selector `0xa1eda53c`
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
    #[ethcall(name = "pendingDefaultAdminDelay", abi = "pendingDefaultAdminDelay()")]
    pub struct PendingDefaultAdminDelayCall;
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
    ///Container type for all input parameters for the `rollbackDefaultAdminDelay` function with signature `rollbackDefaultAdminDelay()` and selector `0x0aa6220b`
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
    #[ethcall(name = "rollbackDefaultAdminDelay", abi = "rollbackDefaultAdminDelay()")]
    pub struct RollbackDefaultAdminDelayCall;
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
    pub enum IAccessControlDefaultAdminRulesCalls {
        AcceptDefaultAdminTransfer(AcceptDefaultAdminTransferCall),
        BeginDefaultAdminTransfer(BeginDefaultAdminTransferCall),
        CancelDefaultAdminTransfer(CancelDefaultAdminTransferCall),
        ChangeDefaultAdminDelay(ChangeDefaultAdminDelayCall),
        DefaultAdmin(DefaultAdminCall),
        DefaultAdminDelay(DefaultAdminDelayCall),
        DefaultAdminDelayIncreaseWait(DefaultAdminDelayIncreaseWaitCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        PendingDefaultAdmin(PendingDefaultAdminCall),
        PendingDefaultAdminDelay(PendingDefaultAdminDelayCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RollbackDefaultAdminDelay(RollbackDefaultAdminDelayCall),
    }
    impl ::ethers::core::abi::AbiDecode for IAccessControlDefaultAdminRulesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) = <BeginDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeginDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) = <CancelDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) = <ChangeDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <DefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdmin(decoded));
            }
            if let Ok(decoded) = <DefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <DefaultAdminDelayIncreaseWaitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminDelayIncreaseWait(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
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
            if let Ok(decoded) = <PendingDefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingDefaultAdmin(decoded));
            }
            if let Ok(decoded) = <PendingDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingDefaultAdminDelay(decoded));
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
            if let Ok(decoded) = <RollbackDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollbackDefaultAdminDelay(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAccessControlDefaultAdminRulesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeginDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingDefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RollbackDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAccessControlDefaultAdminRulesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeginDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingDefaultAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollbackDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AcceptDefaultAdminTransferCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: AcceptDefaultAdminTransferCall) -> Self {
            Self::AcceptDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<BeginDefaultAdminTransferCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: BeginDefaultAdminTransferCall) -> Self {
            Self::BeginDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<CancelDefaultAdminTransferCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: CancelDefaultAdminTransferCall) -> Self {
            Self::CancelDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<ChangeDefaultAdminDelayCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: ChangeDefaultAdminDelayCall) -> Self {
            Self::ChangeDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: DefaultAdminCall) -> Self {
            Self::DefaultAdmin(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: DefaultAdminDelayCall) -> Self {
            Self::DefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayIncreaseWaitCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: DefaultAdminDelayIncreaseWaitCall) -> Self {
            Self::DefaultAdminDelayIncreaseWait(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for IAccessControlDefaultAdminRulesCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for IAccessControlDefaultAdminRulesCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: PendingDefaultAdminCall) -> Self {
            Self::PendingDefaultAdmin(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminDelayCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: PendingDefaultAdminDelayCall) -> Self {
            Self::PendingDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for IAccessControlDefaultAdminRulesCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RollbackDefaultAdminDelayCall>
    for IAccessControlDefaultAdminRulesCalls {
        fn from(value: RollbackDefaultAdminDelayCall) -> Self {
            Self::RollbackDefaultAdminDelay(value)
        }
    }
    ///Container type for all return fields from the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
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
    pub struct DefaultAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
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
    pub struct DefaultAdminDelayReturn(pub u64);
    ///Container type for all return fields from the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
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
    pub struct DefaultAdminDelayIncreaseWaitReturn(pub u64);
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
    ///Container type for all return fields from the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
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
    pub struct PendingDefaultAdminReturn {
        pub new_admin: ::ethers::core::types::Address,
        pub accept_schedule: u64,
    }
    ///Container type for all return fields from the `pendingDefaultAdminDelay` function with signature `pendingDefaultAdminDelay()` and selector `0xa1eda53c`
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
    pub struct PendingDefaultAdminDelayReturn {
        pub new_delay: u64,
        pub effect_schedule: u64,
    }
}
