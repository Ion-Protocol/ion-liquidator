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
    const __BYTECODE: &[u8] = b"`\xA0`@R`@Qb\0\x1A\xA08\x03\x80b\0\x1A\xA0\x839\x81\x81\x01`@R\x81\x01\x90b\0\0)\x91\x90b\0\x07\x83V[\x82\x81b\0\0=\x82\x82b\0\0\xD1` \x1B` \x1CV[PP\x81`@Qb\0\0N\x90b\0\x05\x8FV[b\0\0Z\x91\x90b\0\x08\x0CV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15b\0\0tW=_\x80>=_\xFD[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPb\0\0\xC8b\0\0\xBCb\0\x01]` \x1B` \x1CV[b\0\x01f` \x1B` \x1CV[PPPb\0\x08\xB4V[b\0\0\xE2\x82b\0\x01\xC4` \x1B` \x1CV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15b\0\x01HWb\0\x01A\x82\x82b\0\x02\x98` \x1B` \x1CV[Pb\0\x01YV[b\0\x01Xb\0\x03$` \x1B` \x1CV[[PPV[_`\x80Q\x90P\x90V[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fb\0\x01\x97b\0\x03a` \x1B` \x1CV[\x82`@Qb\0\x01\xA8\x92\x91\x90b\0\x08'V[`@Q\x80\x91\x03\x90\xA1b\0\x01\xC1\x81b\0\x03\xBC` \x1B` \x1CV[PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03b\0\x02\"W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x02\x19\x91\x90b\0\x08\x0CV[`@Q\x80\x91\x03\x90\xFD[\x80b\0\x02V\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Bb\0\x04\xA5` \x1B` \x1CV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qb\0\x02\xC3\x91\x90b\0\x08\x9CV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14b\0\x02\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>b\0\x03\x02V[``\x91P[P\x91P\x91Pb\0\x03\x1A\x85\x83\x83b\0\x04\xAE` \x1B` \x1CV[\x92PPP\x92\x91PPV[_4\x11\x15b\0\x03_W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_b\0\x03\x95\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03_\x1Bb\0\x04\xA5` \x1B` \x1CV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\x04/W_`@Q\x7Fb\xE7{\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x04&\x91\x90b\0\x08\x0CV[`@Q\x80\x91\x03\x90\xFD[\x80b\0\x04c\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03_\x1Bb\0\x04\xA5` \x1B` \x1CV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[_\x81\x90P\x91\x90PV[``\x82b\0\x04\xCDWb\0\x04\xC7\x82b\0\x05J` \x1B` \x1CV[b\0\x05BV[_\x82Q\x14\x80\x15b\0\x04\xF4WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15b\0\x059W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x050\x91\x90b\0\x08\x0CV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pb\0\x05CV[[\x93\x92PPPV[_\x81Q\x11\x15b\0\x05]W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xE0\x80b\0\x10\xC0\x839\x01\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_b\0\x05\xD9\x82b\0\x05\xAEV[\x90P\x91\x90PV[b\0\x05\xEB\x81b\0\x05\xCDV[\x81\x14b\0\x05\xF6W_\x80\xFD[PV[_\x81Q\x90Pb\0\x06\t\x81b\0\x05\xE0V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[b\0\x06_\x82b\0\x06\x17V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x06\x81Wb\0\x06\x80b\0\x06'V[[\x80`@RPPPV[_b\0\x06\x95b\0\x05\x9DV[\x90Pb\0\x06\xA3\x82\x82b\0\x06TV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x06\xC5Wb\0\x06\xC4b\0\x06'V[[b\0\x06\xD0\x82b\0\x06\x17V[\x90P` \x81\x01\x90P\x91\x90PV[_[\x83\x81\x10\x15b\0\x06\xFCW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pb\0\x06\xDFV[_\x84\x84\x01RPPPPV[_b\0\x07\x1Db\0\x07\x17\x84b\0\x06\xA8V[b\0\x06\x8AV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15b\0\x07<Wb\0\x07;b\0\x06\x13V[[b\0\x07I\x84\x82\x85b\0\x06\xDDV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12b\0\x07hWb\0\x07gb\0\x06\x0FV[[\x81Qb\0\x07z\x84\x82` \x86\x01b\0\x07\x07V[\x91PP\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15b\0\x07\x9DWb\0\x07\x9Cb\0\x05\xA6V[[_b\0\x07\xAC\x86\x82\x87\x01b\0\x05\xF9V[\x93PP` b\0\x07\xBF\x86\x82\x87\x01b\0\x05\xF9V[\x92PP`@\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x07\xE3Wb\0\x07\xE2b\0\x05\xAAV[[b\0\x07\xF1\x86\x82\x87\x01b\0\x07QV[\x91PP\x92P\x92P\x92V[b\0\x08\x06\x81b\0\x05\xCDV[\x82RPPV[_` \x82\x01\x90Pb\0\x08!_\x83\x01\x84b\0\x07\xFBV[\x92\x91PPV[_`@\x82\x01\x90Pb\0\x08<_\x83\x01\x85b\0\x07\xFBV[b\0\x08K` \x83\x01\x84b\0\x07\xFBV[\x93\x92PPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_b\0\x08r\x82b\0\x08RV[b\0\x08~\x81\x85b\0\x08\\V[\x93Pb\0\x08\x90\x81\x85` \x86\x01b\0\x06\xDDV[\x80\x84\x01\x91PP\x92\x91PPV[_b\0\x08\xA9\x82\x84b\0\x08fV[\x91P\x81\x90P\x92\x91PPV[`\x80Qa\x07\xF3b\0\x08\xCD_9_a\x01\x04\x01Ra\x07\xF3_\xF3\xFE`\x80`@Ra\0\x0Ca\0\x0EV[\0[a\0\x16a\x01\x01V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xF6WcO\x1E\xF2\x86`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16_5\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\0\xE9W`@Q\x7F\xD2\xB5v\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xF1a\x01(V[a\0\xFFV[a\0\xFEa\x01\\V[[V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P\x90V[_\x80_6`\x04\x90\x80\x92a\x01=\x93\x92\x91\x90a\x04\xD0V[\x81\x01\x90a\x01J\x91\x90a\x06\xA8V[\x91P\x91Pa\x01X\x82\x82a\x01nV[PPV[a\x01la\x01ga\x01\xE0V[a\x01\xEEV[V[a\x01w\x82a\x02\rV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a\x01\xD3Wa\x01\xCD\x82\x82a\x02\xD6V[Pa\x01\xDCV[a\x01\xDBa\x03VV[[PPV[_a\x01\xE9a\x03\x92V[\x90P\x90V[6_\x807_\x806_\x84Z\xF4=_\x80>\x80_\x81\x14a\x02\tW=_\xF3[=_\xFD[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x02hW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02_\x91\x90a\x07\"V[`@Q\x80\x91\x03\x90\xFD[\x80a\x02\x94\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\x03\xE5V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa\x02\xFF\x91\x90a\x07\xA7V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x037W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03<V[``\x91P[P\x91P\x91Pa\x03L\x85\x83\x83a\x03\xEEV[\x92PPP\x92\x91PPV[_4\x11\x15a\x03\x90W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\x03\xBE\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\x03\xE5V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x81\x90P\x91\x90PV[``\x82a\x04\x03Wa\x03\xFE\x82a\x04{V[a\x04sV[_\x82Q\x14\x80\x15a\x04)WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x04kW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04b\x91\x90a\x07\"V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x04tV[[\x93\x92PPPV[_\x81Q\x11\x15a\x04\x8DW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\x85\x85\x11\x15a\x04\xE3Wa\x04\xE2a\x04\xC8V[[\x83\x86\x11\x15a\x04\xF4Wa\x04\xF3a\x04\xCCV[[`\x01\x85\x02\x83\x01\x91P\x84\x86\x03\x90P\x94P\x94\x92PPPV[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x05;\x82a\x05\x12V[\x90P\x91\x90PV[a\x05K\x81a\x051V[\x81\x14a\x05UW_\x80\xFD[PV[_\x815\x90Pa\x05f\x81a\x05BV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x05\xBA\x82a\x05tV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05\xD9Wa\x05\xD8a\x05\x84V[[\x80`@RPPPV[_a\x05\xEBa\x04\xBFV[\x90Pa\x05\xF7\x82\x82a\x05\xB1V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\x16Wa\x06\x15a\x05\x84V[[a\x06\x1F\x82a\x05tV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x06La\x06G\x84a\x05\xFCV[a\x05\xE2V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x06hWa\x06ga\x05pV[[a\x06s\x84\x82\x85a\x06,V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x06\x8FWa\x06\x8Ea\x05lV[[\x815a\x06\x9F\x84\x82` \x86\x01a\x06:V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x06\xBEWa\x06\xBDa\x05\nV[[_a\x06\xCB\x85\x82\x86\x01a\x05XV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xECWa\x06\xEBa\x05\x0EV[[a\x06\xF8\x85\x82\x86\x01a\x06{V[\x91PP\x92P\x92\x90PV[_a\x07\x0C\x82a\x05\x12V[\x90P\x91\x90PV[a\x07\x1C\x81a\x07\x02V[\x82RPPV[_` \x82\x01\x90Pa\x075_\x83\x01\x84a\x07\x13V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x07lW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x07QV[_\x84\x84\x01RPPPPV[_a\x07\x81\x82a\x07;V[a\x07\x8B\x81\x85a\x07EV[\x93Pa\x07\x9B\x81\x85` \x86\x01a\x07OV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x07\xB2\x82\x84a\x07wV[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 Z\xEFk\x16\t\x8B\xDF\xCD\x9D\xBF\x80d\xAD\x81B>'C\xA4~\xDBx\xD6\xDA\xF0\x1FH,l\x12\xB1\x9FdsolcC\0\x08\x15\x003`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\t\xE08\x03\x80a\t\xE0\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x01\xD7V[\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xA2W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\x99\x91\x90a\x02\x11V[`@Q\x80\x91\x03\x90\xFD[a\0\xB1\x81a\0\xB8` \x1B` \x1CV[PPa\x02*V[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01\xA6\x82a\x01}V[\x90P\x91\x90PV[a\x01\xB6\x81a\x01\x9CV[\x81\x14a\x01\xC0W_\x80\xFD[PV[_\x81Q\x90Pa\x01\xD1\x81a\x01\xADV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x01\xECWa\x01\xEBa\x01yV[[_a\x01\xF9\x84\x82\x85\x01a\x01\xC3V[\x91PP\x92\x91PPV[a\x02\x0B\x81a\x01\x9CV[\x82RPPV[_` \x82\x01\x90Pa\x02$_\x83\x01\x84a\x02\x02V[\x92\x91PPV[a\x07\xA9\x80a\x027_9_\xF3\xFE`\x80`@R`\x046\x10a\0IW_5`\xE0\x1C\x80cqP\x18\xA6\x14a\0MW\x80c\x8D\xA5\xCB[\x14a\0cW\x80c\x96#`\x9D\x14a\0\x8DW\x80c\xAD<\xB1\xCC\x14a\0\xA9W\x80c\xF2\xFD\xE3\x8B\x14a\0\xD3W[_\x80\xFD[4\x80\x15a\0XW_\x80\xFD[Pa\0aa\0\xFBV[\0[4\x80\x15a\0nW_\x80\xFD[Pa\0wa\x01\x0EV[`@Qa\0\x84\x91\x90a\x03\xF7V[`@Q\x80\x91\x03\x90\xF3[a\0\xA7`\x04\x806\x03\x81\x01\x90a\0\xA2\x91\x90a\x05\xC2V[a\x015V[\0[4\x80\x15a\0\xB4W_\x80\xFD[Pa\0\xBDa\x01\xACV[`@Qa\0\xCA\x91\x90a\x06\xA8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xDEW_\x80\xFD[Pa\0\xF9`\x04\x806\x03\x81\x01\x90a\0\xF4\x91\x90a\x06\xC8V[a\x01\xE5V[\0[a\x01\x03a\x02iV[a\x01\x0C_a\x02\xF0V[V[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x01=a\x02iV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cO\x1E\xF2\x864\x84\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01y\x92\x91\x90a\x07EV[_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x01\x90W_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xA2W=_\x80>=_\xFD[PPPPPPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01\xEDa\x02iV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x02]W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02T\x91\x90a\x03\xF7V[`@Q\x80\x91\x03\x90\xFD[a\x02f\x81a\x02\xF0V[PV[a\x02qa\x03\xB1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\x8Fa\x01\x0EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x02\xEEWa\x02\xB2a\x03\xB1V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xE5\x91\x90a\x03\xF7V[`@Q\x80\x91\x03\x90\xFD[V[_\x80_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81_\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_3\x90P\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x03\xE1\x82a\x03\xB8V[\x90P\x91\x90PV[a\x03\xF1\x81a\x03\xD7V[\x82RPPV[_` \x82\x01\x90Pa\x04\n_\x83\x01\x84a\x03\xE8V[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_a\x04+\x82a\x03\xD7V[\x90P\x91\x90PV[a\x04;\x81a\x04!V[\x81\x14a\x04EW_\x80\xFD[PV[_\x815\x90Pa\x04V\x81a\x042V[\x92\x91PPV[a\x04e\x81a\x03\xD7V[\x81\x14a\x04oW_\x80\xFD[PV[_\x815\x90Pa\x04\x80\x81a\x04\\V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x04\xD4\x82a\x04\x8EV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\xF3Wa\x04\xF2a\x04\x9EV[[\x80`@RPPPV[_a\x05\x05a\x04\x10V[\x90Pa\x05\x11\x82\x82a\x04\xCBV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x050Wa\x05/a\x04\x9EV[[a\x059\x82a\x04\x8EV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x05fa\x05a\x84a\x05\x16V[a\x04\xFCV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x05\x82Wa\x05\x81a\x04\x8AV[[a\x05\x8D\x84\x82\x85a\x05FV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x05\xA9Wa\x05\xA8a\x04\x86V[[\x815a\x05\xB9\x84\x82` \x86\x01a\x05TV[\x91PP\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x05\xD9Wa\x05\xD8a\x04\x19V[[_a\x05\xE6\x86\x82\x87\x01a\x04HV[\x93PP` a\x05\xF7\x86\x82\x87\x01a\x04rV[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x18Wa\x06\x17a\x04\x1DV[[a\x06$\x86\x82\x87\x01a\x05\x95V[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x06eW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x06JV[_\x84\x84\x01RPPPPV[_a\x06z\x82a\x06.V[a\x06\x84\x81\x85a\x068V[\x93Pa\x06\x94\x81\x85` \x86\x01a\x06HV[a\x06\x9D\x81a\x04\x8EV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x06\xC0\x81\x84a\x06pV[\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x06\xDDWa\x06\xDCa\x04\x19V[[_a\x06\xEA\x84\x82\x85\x01a\x04rV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x07\x17\x82a\x06\xF3V[a\x07!\x81\x85a\x06\xFDV[\x93Pa\x071\x81\x85` \x86\x01a\x06HV[a\x07:\x81a\x04\x8EV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\x07X_\x83\x01\x85a\x03\xE8V[\x81\x81\x03` \x83\x01Ra\x07j\x81\x84a\x07\rV[\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 '\x11U?\xC3\xEE\xD7\0d\xDB\x90\x07\xE7*\xE7\xEC\xA8\xED\"F\xE69\x9Dq\xA8\"\x16u\xCFV\x7F\xF5dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@Ra\0\x0Ca\0\x0EV[\0[a\0\x16a\x01\x01V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xF6WcO\x1E\xF2\x86`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16_5\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\0\xE9W`@Q\x7F\xD2\xB5v\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xF1a\x01(V[a\0\xFFV[a\0\xFEa\x01\\V[[V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90P\x90V[_\x80_6`\x04\x90\x80\x92a\x01=\x93\x92\x91\x90a\x04\xD0V[\x81\x01\x90a\x01J\x91\x90a\x06\xA8V[\x91P\x91Pa\x01X\x82\x82a\x01nV[PPV[a\x01la\x01ga\x01\xE0V[a\x01\xEEV[V[a\x01w\x82a\x02\rV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a\x01\xD3Wa\x01\xCD\x82\x82a\x02\xD6V[Pa\x01\xDCV[a\x01\xDBa\x03VV[[PPV[_a\x01\xE9a\x03\x92V[\x90P\x90V[6_\x807_\x806_\x84Z\xF4=_\x80>\x80_\x81\x14a\x02\tW=_\xF3[=_\xFD[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x02hW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02_\x91\x90a\x07\"V[`@Q\x80\x91\x03\x90\xFD[\x80a\x02\x94\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\x03\xE5V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa\x02\xFF\x91\x90a\x07\xA7V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x037W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03<V[``\x91P[P\x91P\x91Pa\x03L\x85\x83\x83a\x03\xEEV[\x92PPP\x92\x91PPV[_4\x11\x15a\x03\x90W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\x03\xBE\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\x03\xE5V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x81\x90P\x91\x90PV[``\x82a\x04\x03Wa\x03\xFE\x82a\x04{V[a\x04sV[_\x82Q\x14\x80\x15a\x04)WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x04kW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04b\x91\x90a\x07\"V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x04tV[[\x93\x92PPPV[_\x81Q\x11\x15a\x04\x8DW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\x85\x85\x11\x15a\x04\xE3Wa\x04\xE2a\x04\xC8V[[\x83\x86\x11\x15a\x04\xF4Wa\x04\xF3a\x04\xCCV[[`\x01\x85\x02\x83\x01\x91P\x84\x86\x03\x90P\x94P\x94\x92PPPV[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x05;\x82a\x05\x12V[\x90P\x91\x90PV[a\x05K\x81a\x051V[\x81\x14a\x05UW_\x80\xFD[PV[_\x815\x90Pa\x05f\x81a\x05BV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x05\xBA\x82a\x05tV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05\xD9Wa\x05\xD8a\x05\x84V[[\x80`@RPPPV[_a\x05\xEBa\x04\xBFV[\x90Pa\x05\xF7\x82\x82a\x05\xB1V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\x16Wa\x06\x15a\x05\x84V[[a\x06\x1F\x82a\x05tV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x06La\x06G\x84a\x05\xFCV[a\x05\xE2V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x06hWa\x06ga\x05pV[[a\x06s\x84\x82\x85a\x06,V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x06\x8FWa\x06\x8Ea\x05lV[[\x815a\x06\x9F\x84\x82` \x86\x01a\x06:V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x06\xBEWa\x06\xBDa\x05\nV[[_a\x06\xCB\x85\x82\x86\x01a\x05XV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xECWa\x06\xEBa\x05\x0EV[[a\x06\xF8\x85\x82\x86\x01a\x06{V[\x91PP\x92P\x92\x90PV[_a\x07\x0C\x82a\x05\x12V[\x90P\x91\x90PV[a\x07\x1C\x81a\x07\x02V[\x82RPPV[_` \x82\x01\x90Pa\x075_\x83\x01\x84a\x07\x13V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_[\x83\x81\x10\x15a\x07lW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x07QV[_\x84\x84\x01RPPPPV[_a\x07\x81\x82a\x07;V[a\x07\x8B\x81\x85a\x07EV[\x93Pa\x07\x9B\x81\x85` \x86\x01a\x07OV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x07\xB2\x82\x84a\x07wV[\x91P\x81\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 Z\xEFk\x16\t\x8B\xDF\xCD\x9D\xBF\x80d\xAD\x81B>'C\xA4~\xDBx\xD6\xDA\xF0\x1FH,l\x12\xB1\x9FdsolcC\0\x08\x15\x003";
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
