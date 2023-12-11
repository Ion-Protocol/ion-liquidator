pub use erc1967_proxy::*;
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
pub mod erc1967_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("implementation"),
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
            ]),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERC1967PROXY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`@Qa\x06\xE58\x03\x80a\x06\xE5\x839\x81\x81\x01`@R\x81\x01\x90a\0%\x91\x90a\x05\x12V[a\x005\x82\x82a\0<` \x1B` \x1CV[PPa\x05\xF6V[a\0K\x82a\0\xC1` \x1B` \x1CV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2`\0\x81Q\x11\x15a\0\xAEWa\0\xA8\x82\x82a\x01\x94` \x1B` \x1CV[Pa\0\xBDV[a\0\xBCa\x02\x1E` \x1B` \x1CV[[PPV[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x01\x1DW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\x14\x91\x90a\x05}V[`@Q\x80\x91\x03\x90\xFD[\x80a\x01P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Ba\x02[` \x1B` \x1CV[`\0\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa\x01\xBE\x91\x90a\x05\xDFV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xF9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xFEV[``\x91P[P\x91P\x91Pa\x02\x14\x85\x83\x83a\x02e` \x1B` \x1CV[\x92PPP\x92\x91PPV[`\x004\x11\x15a\x02YW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0\x81\x90P\x91\x90PV[``\x82a\x02\x80Wa\x02{\x82a\x02\xFA` \x1B` \x1CV[a\x02\xF2V[`\0\x82Q\x14\x80\x15a\x02\xA8WP`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a\x02\xEAW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xE1\x91\x90a\x05}V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa\x02\xF3V[[\x93\x92PPPV[`\0\x81Q\x11\x15a\x03\rW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x03~\x82a\x03SV[\x90P\x91\x90PV[a\x03\x8E\x81a\x03sV[\x81\x14a\x03\x99W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x03\xAB\x81a\x03\x85V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x04\x04\x82a\x03\xBBV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04#Wa\x04\"a\x03\xCCV[[\x80`@RPPPV[`\0a\x046a\x03?V[\x90Pa\x04B\x82\x82a\x03\xFBV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04bWa\x04aa\x03\xCCV[[a\x04k\x82a\x03\xBBV[\x90P` \x81\x01\x90P\x91\x90PV[`\0[\x83\x81\x10\x15a\x04\x96W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x04{V[`\0\x84\x84\x01RPPPPV[`\0a\x04\xB5a\x04\xB0\x84a\x04GV[a\x04,V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x04\xD1Wa\x04\xD0a\x03\xB6V[[a\x04\xDC\x84\x82\x85a\x04xV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x04\xF9Wa\x04\xF8a\x03\xB1V[[\x81Qa\x05\t\x84\x82` \x86\x01a\x04\xA2V[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05)Wa\x05(a\x03IV[[`\0a\x057\x85\x82\x86\x01a\x03\x9CV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05XWa\x05Wa\x03NV[[a\x05d\x85\x82\x86\x01a\x04\xE4V[\x91PP\x92P\x92\x90PV[a\x05w\x81a\x03sV[\x82RPPV[`\0` \x82\x01\x90Pa\x05\x92`\0\x83\x01\x84a\x05nV[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0a\x05\xB9\x82a\x05\x98V[a\x05\xC3\x81\x85a\x05\xA3V[\x93Pa\x05\xD3\x81\x85` \x86\x01a\x04xV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x05\xEB\x82\x84a\x05\xAEV[\x91P\x81\x90P\x92\x91PPV[`\xE1\x80a\x06\x04`\09`\0\xF3\xFE`\x80`@R`\n`\x0CV[\0[`\x18`\x14`\x1AV[`'V[V[`\0`\"`LV[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80`\0\x81\x14`GW=`\0\xF3[=`\0\xFD[`\0`x\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1B`\xA1V[`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x81\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xC2\x01\xCF\x8Cd\xD2\xA2\xC4]L\x1B)\xB9\x13\x11W\x9D\x9F@\xC7\xA8@\xE2\x0F{t\xA8G\xAC\x81\xAD(dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static ERC1967PROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\n`\x0CV[\0[`\x18`\x14`\x1AV[`'V[V[`\0`\"`LV[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80`\0\x81\x14`GW=`\0\xF3[=`\0\xFD[`\0`x\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1B`\xA1V[`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x81\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xC2\x01\xCF\x8Cd\xD2\xA2\xC4]L\x1B)\xB9\x13\x11W\x9D\x9F@\xC7\xA8@\xE2\x0F{t\xA8G\xAC\x81\xAD(dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static ERC1967PROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC1967Proxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC1967Proxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC1967Proxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC1967Proxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC1967Proxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC1967Proxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC1967Proxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC1967PROXY_ABI.clone(),
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
                ERC1967PROXY_ABI.clone(),
                ERC1967PROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
            UpgradedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC1967Proxy<M> {
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
    pub enum ERC1967ProxyErrors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedInnerCall(FailedInnerCall),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC1967ProxyErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC1967ProxyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ERC1967ProxyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ERC1967ProxyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ERC1967ProxyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for ERC1967ProxyErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for ERC1967ProxyErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for ERC1967ProxyErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for ERC1967ProxyErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
}
