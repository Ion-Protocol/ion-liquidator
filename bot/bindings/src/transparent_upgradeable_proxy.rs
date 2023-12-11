pub use transparent_upgradeable_proxy::*;
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
pub mod transparent_upgradeable_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_logic"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_data"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1967InvalidAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1967InvalidImplementation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
                    ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
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
                    ::std::borrow::ToOwned::to_owned("ProxyDeniedAdminAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProxyDeniedAdminAccess",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R`@Qb\0\x1Bc8\x03\x80b\0\x1Bc\x839\x81\x81\x01`@R\x81\x01\x90b\0\0)\x91\x90b\0\x07\xB0V[\x82\x81b\0\0=\x82\x82b\0\0\xD4` \x1B` \x1CV[PP\x81`@Qb\0\0N\x90b\0\x05\xAAV[b\0\0Z\x91\x90b\0\x08<V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0wW=`\0\x80>=`\0\xFD[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPb\0\0\xCBb\0\0\xBFb\0\x01a` \x1B` \x1CV[b\0\x01k` \x1B` \x1CV[PPPb\0\x08\xECV[b\0\0\xE5\x82b\0\x01\xC9` \x1B` \x1CV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2`\0\x81Q\x11\x15b\0\x01LWb\0\x01E\x82\x82b\0\x02\xA1` \x1B` \x1CV[Pb\0\x01]V[b\0\x01\\b\0\x031` \x1B` \x1CV[[PPV[`\0`\x80Q\x90P\x90V[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fb\0\x01\x9Cb\0\x03o` \x1B` \x1CV[\x82`@Qb\0\x01\xAD\x92\x91\x90b\0\x08YV[`@Q\x80\x91\x03\x90\xA1b\0\x01\xC6\x81b\0\x03\xCE` \x1B` \x1CV[PV[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03b\0\x02(W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x02\x1F\x91\x90b\0\x08<V[`@Q\x80\x91\x03\x90\xFD[\x80b\0\x02]\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Bb\0\x04\xBC` \x1B` \x1CV[`\0\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qb\0\x02\xCD\x91\x90b\0\x08\xD3V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14b\0\x03\nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x03\x0FV[``\x91P[P\x91P\x91Pb\0\x03'\x85\x83\x83b\0\x04\xC6` \x1B` \x1CV[\x92PPP\x92\x91PPV[`\x004\x11\x15b\0\x03mW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0b\0\x03\xA5\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03`\0\x1Bb\0\x04\xBC` \x1B` \x1CV[`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\x04CW`\0`@Q\x7Fb\xE7{\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x04:\x91\x90b\0\x08<V[`@Q\x80\x91\x03\x90\xFD[\x80b\0\x04x\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03`\0\x1Bb\0\x04\xBC` \x1B` \x1CV[`\0\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[`\0\x81\x90P\x91\x90PV[``\x82b\0\x04\xE5Wb\0\x04\xDF\x82b\0\x05d` \x1B` \x1CV[b\0\x05\\V[`\0\x82Q\x14\x80\x15b\0\x05\x0EWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15b\0\x05SW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x05J\x91\x90b\0\x08<V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pb\0\x05]V[[\x93\x92PPPV[`\0\x81Q\x11\x15b\0\x05xW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n+\x80b\0\x118\x839\x01\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x05\xF9\x82b\0\x05\xCCV[\x90P\x91\x90PV[b\0\x06\x0B\x81b\0\x05\xECV[\x81\x14b\0\x06\x17W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x06+\x81b\0\x06\0V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x06\x86\x82b\0\x06;V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x06\xA8Wb\0\x06\xA7b\0\x06LV[[\x80`@RPPPV[`\0b\0\x06\xBDb\0\x05\xB8V[\x90Pb\0\x06\xCB\x82\x82b\0\x06{V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x06\xEEWb\0\x06\xEDb\0\x06LV[[b\0\x06\xF9\x82b\0\x06;V[\x90P` \x81\x01\x90P\x91\x90PV[`\0[\x83\x81\x10\x15b\0\x07&W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pb\0\x07\tV[`\0\x84\x84\x01RPPPPV[`\0b\0\x07Ib\0\x07C\x84b\0\x06\xD0V[b\0\x06\xB1V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15b\0\x07hWb\0\x07gb\0\x066V[[b\0\x07u\x84\x82\x85b\0\x07\x06V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x07\x95Wb\0\x07\x94b\0\x061V[[\x81Qb\0\x07\xA7\x84\x82` \x86\x01b\0\x072V[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x07\xCCWb\0\x07\xCBb\0\x05\xC2V[[`\0b\0\x07\xDC\x86\x82\x87\x01b\0\x06\x1AV[\x93PP` b\0\x07\xEF\x86\x82\x87\x01b\0\x06\x1AV[\x92PP`@\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x08\x13Wb\0\x08\x12b\0\x05\xC7V[[b\0\x08!\x86\x82\x87\x01b\0\x07}V[\x91PP\x92P\x92P\x92V[b\0\x086\x81b\0\x05\xECV[\x82RPPV[`\0` \x82\x01\x90Pb\0\x08S`\0\x83\x01\x84b\0\x08+V[\x92\x91PPV[`\0`@\x82\x01\x90Pb\0\x08p`\0\x83\x01\x85b\0\x08+V[b\0\x08\x7F` \x83\x01\x84b\0\x08+V[\x93\x92PPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0b\0\x08\xA9\x82b\0\x08\x86V[b\0\x08\xB5\x81\x85b\0\x08\x91V[\x93Pb\0\x08\xC7\x81\x85` \x86\x01b\0\x07\x06V[\x80\x84\x01\x91PP\x92\x91PPV[`\0b\0\x08\xE1\x82\x84b\0\x08\x9CV[\x91P\x81\x90P\x92\x91PPV[`\x80Qa\x080b\0\t\x08`\09`\0a\x01\x06\x01Ra\x080`\0\xF3\xFE`\x80`@Ra\0\x0Ca\0\x0EV[\0[a\0\x16a\x01\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xF7WcO\x1E\xF2\x86`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x005\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\0\xEAW`@Q\x7F\xD2\xB5v\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xF2a\x01*V[a\x01\0V[a\0\xFFa\x01`V[[V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P\x90V[`\0\x80`\x006`\x04\x90\x80\x92a\x01A\x93\x92\x91\x90a\x04\xF1V[\x81\x01\x90a\x01N\x91\x90a\x06\xDAV[\x91P\x91Pa\x01\\\x82\x82a\x01rV[PPV[a\x01pa\x01ka\x01\xE5V[a\x01\xF4V[V[a\x01{\x82a\x02\x1AV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2`\0\x81Q\x11\x15a\x01\xD8Wa\x01\xD2\x82\x82a\x02\xE7V[Pa\x01\xE1V[a\x01\xE0a\x03kV[[PPV[`\0a\x01\xEFa\x03\xA8V[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80`\0\x81\x14a\x02\x15W=`\0\xF3[=`\0\xFD[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x02vW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02m\x91\x90a\x07WV[`@Q\x80\x91\x03\x90\xFD[\x80a\x02\xA3\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Ba\x03\xFFV[`\0\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa\x03\x11\x91\x90a\x07\xE3V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03LW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03QV[``\x91P[P\x91P\x91Pa\x03a\x85\x83\x83a\x04\tV[\x92PPP\x92\x91PPV[`\x004\x11\x15a\x03\xA6W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\x03\xD6\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Ba\x03\xFFV[`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x81\x90P\x91\x90PV[``\x82a\x04\x1EWa\x04\x19\x82a\x04\x98V[a\x04\x90V[`\0\x82Q\x14\x80\x15a\x04FWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x04\x88W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x7F\x91\x90a\x07WV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x04\x91V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\x04\xABW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x85\x85\x11\x15a\x05\x05Wa\x05\x04a\x04\xE7V[[\x83\x86\x11\x15a\x05\x16Wa\x05\x15a\x04\xECV[[`\x01\x85\x02\x83\x01\x91P\x84\x86\x03\x90P\x94P\x94\x92PPPV[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x05a\x82a\x056V[\x90P\x91\x90PV[a\x05q\x81a\x05VV[\x81\x14a\x05|W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\x8E\x81a\x05hV[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x05\xE7\x82a\x05\x9EV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\x06Wa\x06\x05a\x05\xAFV[[\x80`@RPPPV[`\0a\x06\x19a\x04\xDDV[\x90Pa\x06%\x82\x82a\x05\xDEV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06EWa\x06Da\x05\xAFV[[a\x06N\x82a\x05\x9EV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x06}a\x06x\x84a\x06*V[a\x06\x0FV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x06\x99Wa\x06\x98a\x05\x99V[[a\x06\xA4\x84\x82\x85a\x06[V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x06\xC1Wa\x06\xC0a\x05\x94V[[\x815a\x06\xD1\x84\x82` \x86\x01a\x06jV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xF1Wa\x06\xF0a\x05,V[[`\0a\x06\xFF\x85\x82\x86\x01a\x05\x7FV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07 Wa\x07\x1Fa\x051V[[a\x07,\x85\x82\x86\x01a\x06\xACV[\x91PP\x92P\x92\x90PV[`\0a\x07A\x82a\x056V[\x90P\x91\x90PV[a\x07Q\x81a\x076V[\x82RPPV[`\0` \x82\x01\x90Pa\x07l`\0\x83\x01\x84a\x07HV[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x07\xA6W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x07\x8BV[`\0\x84\x84\x01RPPPPV[`\0a\x07\xBD\x82a\x07rV[a\x07\xC7\x81\x85a\x07}V[\x93Pa\x07\xD7\x81\x85` \x86\x01a\x07\x88V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x07\xEF\x82\x84a\x07\xB2V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 :\xDCeE\xE68l+yqo|OK\xA5\xEB\x9Clu\x19\xB0\xA2Rn\xEA\xF4\xAA\x952\xF2\x18qdsolcC\0\x08\x15\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\n+8\x03\x80a\n+\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x01\xE2V[\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xA5W`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\x9C\x91\x90a\x02\x1EV[`@Q\x80\x91\x03\x90\xFD[a\0\xB4\x81a\0\xBB` \x1B` \x1CV[PPa\x029V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x01\xAF\x82a\x01\x84V[\x90P\x91\x90PV[a\x01\xBF\x81a\x01\xA4V[\x81\x14a\x01\xCAW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x01\xDC\x81a\x01\xB6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x01\xF8Wa\x01\xF7a\x01\x7FV[[`\0a\x02\x06\x84\x82\x85\x01a\x01\xCDV[\x91PP\x92\x91PPV[a\x02\x18\x81a\x01\xA4V[\x82RPPV[`\0` \x82\x01\x90Pa\x023`\0\x83\x01\x84a\x02\x0FV[\x92\x91PPV[a\x07\xE3\x80a\x02H`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0JW`\x005`\xE0\x1C\x80cqP\x18\xA6\x14a\0OW\x80c\x8D\xA5\xCB[\x14a\0fW\x80c\x96#`\x9D\x14a\0\x91W\x80c\xAD<\xB1\xCC\x14a\0\xADW\x80c\xF2\xFD\xE3\x8B\x14a\0\xD8W[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[Pa\0da\x01\x01V[\0[4\x80\x15a\0rW`\0\x80\xFD[Pa\0{a\x01\x15V[`@Qa\0\x88\x91\x90a\x04\x0CV[`@Q\x80\x91\x03\x90\xF3[a\0\xAB`\x04\x806\x03\x81\x01\x90a\0\xA6\x91\x90a\x05\xEBV[a\x01>V[\0[4\x80\x15a\0\xB9W`\0\x80\xFD[Pa\0\xC2a\x01\xB9V[`@Qa\0\xCF\x91\x90a\x06\xD9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xE4W`\0\x80\xFD[Pa\0\xFF`\x04\x806\x03\x81\x01\x90a\0\xFA\x91\x90a\x06\xFBV[a\x01\xF2V[\0[a\x01\ta\x02xV[a\x01\x13`\0a\x02\xFFV[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x01Fa\x02xV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cO\x1E\xF2\x864\x84\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x82\x92\x91\x90a\x07}V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x01\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xAFW=`\0\x80>=`\0\xFD[PPPPPPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01\xFAa\x02xV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x02lW`\0`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02c\x91\x90a\x04\x0CV[`@Q\x80\x91\x03\x90\xFD[a\x02u\x81a\x02\xFFV[PV[a\x02\x80a\x03\xC3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\x9Ea\x01\x15V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x02\xFDWa\x02\xC1a\x03\xC3V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xF4\x91\x90a\x04\x0CV[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x003\x90P\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x03\xF6\x82a\x03\xCBV[\x90P\x91\x90PV[a\x04\x06\x81a\x03\xEBV[\x82RPPV[`\0` \x82\x01\x90Pa\x04!`\0\x83\x01\x84a\x03\xFDV[\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0a\x04F\x82a\x03\xEBV[\x90P\x91\x90PV[a\x04V\x81a\x04;V[\x81\x14a\x04aW`\0\x80\xFD[PV[`\0\x815\x90Pa\x04s\x81a\x04MV[\x92\x91PPV[a\x04\x82\x81a\x03\xEBV[\x81\x14a\x04\x8DW`\0\x80\xFD[PV[`\0\x815\x90Pa\x04\x9F\x81a\x04yV[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x04\xF8\x82a\x04\xAFV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05\x17Wa\x05\x16a\x04\xC0V[[\x80`@RPPPV[`\0a\x05*a\x04'V[\x90Pa\x056\x82\x82a\x04\xEFV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05VWa\x05Ua\x04\xC0V[[a\x05_\x82a\x04\xAFV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x05\x8Ea\x05\x89\x84a\x05;V[a\x05 V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x05\xAAWa\x05\xA9a\x04\xAAV[[a\x05\xB5\x84\x82\x85a\x05lV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x05\xD2Wa\x05\xD1a\x04\xA5V[[\x815a\x05\xE2\x84\x82` \x86\x01a\x05{V[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06\x04Wa\x06\x03a\x041V[[`\0a\x06\x12\x86\x82\x87\x01a\x04dV[\x93PP` a\x06#\x86\x82\x87\x01a\x04\x90V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06DWa\x06Ca\x046V[[a\x06P\x86\x82\x87\x01a\x05\xBDV[\x91PP\x92P\x92P\x92V[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x06\x94W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x06yV[`\0\x84\x84\x01RPPPPV[`\0a\x06\xAB\x82a\x06ZV[a\x06\xB5\x81\x85a\x06eV[\x93Pa\x06\xC5\x81\x85` \x86\x01a\x06vV[a\x06\xCE\x81a\x04\xAFV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x06\xF3\x81\x84a\x06\xA0V[\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x07\x11Wa\x07\x10a\x041V[[`\0a\x07\x1F\x84\x82\x85\x01a\x04\x90V[\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a\x07O\x82a\x07(V[a\x07Y\x81\x85a\x073V[\x93Pa\x07i\x81\x85` \x86\x01a\x06vV[a\x07r\x81a\x04\xAFV[\x84\x01\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa\x07\x92`\0\x83\x01\x85a\x03\xFDV[\x81\x81\x03` \x83\x01Ra\x07\xA4\x81\x84a\x07DV[\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x10\xB1\xC8}_L\xC2\xD8\xD1\x15\x12o\xBA\xA3\x14\x9C\xE2\xDA1\x87rjh\x05\x8A\x14R\xEES\xB0\xCD\x16dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@Ra\0\x0Ca\0\x0EV[\0[a\0\x16a\x01\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xF7WcO\x1E\xF2\x86`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x005\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\0\xEAW`@Q\x7F\xD2\xB5v\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xF2a\x01*V[a\x01\0V[a\0\xFFa\x01`V[[V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P\x90V[`\0\x80`\x006`\x04\x90\x80\x92a\x01A\x93\x92\x91\x90a\x04\xF1V[\x81\x01\x90a\x01N\x91\x90a\x06\xDAV[\x91P\x91Pa\x01\\\x82\x82a\x01rV[PPV[a\x01pa\x01ka\x01\xE5V[a\x01\xF4V[V[a\x01{\x82a\x02\x1AV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2`\0\x81Q\x11\x15a\x01\xD8Wa\x01\xD2\x82\x82a\x02\xE7V[Pa\x01\xE1V[a\x01\xE0a\x03kV[[PPV[`\0a\x01\xEFa\x03\xA8V[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80`\0\x81\x14a\x02\x15W=`\0\xF3[=`\0\xFD[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x02vW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02m\x91\x90a\x07WV[`@Q\x80\x91\x03\x90\xFD[\x80a\x02\xA3\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Ba\x03\xFFV[`\0\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa\x03\x11\x91\x90a\x07\xE3V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03LW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03QV[``\x91P[P\x91P\x91Pa\x03a\x85\x83\x83a\x04\tV[\x92PPP\x92\x91PPV[`\x004\x11\x15a\x03\xA6W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\x03\xD6\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Ba\x03\xFFV[`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x81\x90P\x91\x90PV[``\x82a\x04\x1EWa\x04\x19\x82a\x04\x98V[a\x04\x90V[`\0\x82Q\x14\x80\x15a\x04FWP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x04\x88W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\x7F\x91\x90a\x07WV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x04\x91V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\x04\xABW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x85\x85\x11\x15a\x05\x05Wa\x05\x04a\x04\xE7V[[\x83\x86\x11\x15a\x05\x16Wa\x05\x15a\x04\xECV[[`\x01\x85\x02\x83\x01\x91P\x84\x86\x03\x90P\x94P\x94\x92PPPV[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x05a\x82a\x056V[\x90P\x91\x90PV[a\x05q\x81a\x05VV[\x81\x14a\x05|W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\x8E\x81a\x05hV[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x05\xE7\x82a\x05\x9EV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\x06Wa\x06\x05a\x05\xAFV[[\x80`@RPPPV[`\0a\x06\x19a\x04\xDDV[\x90Pa\x06%\x82\x82a\x05\xDEV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06EWa\x06Da\x05\xAFV[[a\x06N\x82a\x05\x9EV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x06}a\x06x\x84a\x06*V[a\x06\x0FV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x06\x99Wa\x06\x98a\x05\x99V[[a\x06\xA4\x84\x82\x85a\x06[V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x06\xC1Wa\x06\xC0a\x05\x94V[[\x815a\x06\xD1\x84\x82` \x86\x01a\x06jV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xF1Wa\x06\xF0a\x05,V[[`\0a\x06\xFF\x85\x82\x86\x01a\x05\x7FV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07 Wa\x07\x1Fa\x051V[[a\x07,\x85\x82\x86\x01a\x06\xACV[\x91PP\x92P\x92\x90PV[`\0a\x07A\x82a\x056V[\x90P\x91\x90PV[a\x07Q\x81a\x076V[\x82RPPV[`\0` \x82\x01\x90Pa\x07l`\0\x83\x01\x84a\x07HV[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x07\xA6W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x07\x8BV[`\0\x84\x84\x01RPPPPV[`\0a\x07\xBD\x82a\x07rV[a\x07\xC7\x81\x85a\x07}V[\x93Pa\x07\xD7\x81\x85` \x86\x01a\x07\x88V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x07\xEF\x82\x84a\x07\xB2V[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 :\xDCeE\xE68l+yqo|OK\xA5\xEB\x9Clu\x19\xB0\xA2Rn\xEA\xF4\xAA\x952\xF2\x18qdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TransparentUpgradeableProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TransparentUpgradeableProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TransparentUpgradeableProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TransparentUpgradeableProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TransparentUpgradeableProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TransparentUpgradeableProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TransparentUpgradeableProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
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
                TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
                TRANSPARENTUPGRADEABLEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransparentUpgradeableProxyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TransparentUpgradeableProxy<M> {
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
    ///Custom Error type `ERC1967InvalidAdmin` with signature `ERC1967InvalidAdmin(address)` and selector `0x62e77ba2`
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
    #[etherror(name = "ERC1967InvalidAdmin", abi = "ERC1967InvalidAdmin(address)")]
    pub struct ERC1967InvalidAdmin {
        pub admin: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967InvalidImplementation` with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`
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
        name = "ERC1967InvalidImplementation",
        abi = "ERC1967InvalidImplementation(address)"
    )]
    pub struct ERC1967InvalidImplementation {
        pub implementation: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967NonPayable` with signature `ERC1967NonPayable()` and selector `0xb398979f`
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
    #[etherror(name = "ERC1967NonPayable", abi = "ERC1967NonPayable()")]
    pub struct ERC1967NonPayable;
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
    ///Custom Error type `ProxyDeniedAdminAccess` with signature `ProxyDeniedAdminAccess()` and selector `0xd2b576ec`
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
    #[etherror(name = "ProxyDeniedAdminAccess", abi = "ProxyDeniedAdminAccess()")]
    pub struct ProxyDeniedAdminAccess;
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
    pub enum TransparentUpgradeableProxyErrors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidAdmin(ERC1967InvalidAdmin),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedInnerCall(FailedInnerCall),
        ProxyDeniedAdminAccess(ProxyDeniedAdminAccess),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for TransparentUpgradeableProxyErrors {
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
            if let Ok(decoded) = <ERC1967InvalidAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967InvalidAdmin(decoded));
            }
            if let Ok(decoded) = <ERC1967InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <ERC1967NonPayable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967NonPayable(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <ProxyDeniedAdminAccess as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxyDeniedAdminAccess(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TransparentUpgradeableProxyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967InvalidAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyDeniedAdminAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for TransparentUpgradeableProxyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967NonPayable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ProxyDeniedAdminAccess as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for TransparentUpgradeableProxyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyDeniedAdminAccess(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for TransparentUpgradeableProxyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for TransparentUpgradeableProxyErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidAdmin>
    for TransparentUpgradeableProxyErrors {
        fn from(value: ERC1967InvalidAdmin) -> Self {
            Self::ERC1967InvalidAdmin(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation>
    for TransparentUpgradeableProxyErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for TransparentUpgradeableProxyErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for TransparentUpgradeableProxyErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<ProxyDeniedAdminAccess>
    for TransparentUpgradeableProxyErrors {
        fn from(value: ProxyDeniedAdminAccess) -> Self {
            Self::ProxyDeniedAdminAccess(value)
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
    pub enum TransparentUpgradeableProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for TransparentUpgradeableProxyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(
                    TransparentUpgradeableProxyEvents::AdminChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(TransparentUpgradeableProxyEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TransparentUpgradeableProxyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter>
    for TransparentUpgradeableProxyEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for TransparentUpgradeableProxyEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
}
