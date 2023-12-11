pub use mock_whitelist::*;
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
pub mod mock_whitelist {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("isWhitelistedBorrower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isWhitelistedBorrower",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                    ::std::borrow::ToOwned::to_owned("isWhitelistedLender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isWhitelistedLender",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKWHITELIST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\xFC\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x12]\xDFM\x14a\0;W\x80cr%\x85\xD5\x14a\0kW[`\0\x80\xFD[a\0U`\x04\x806\x03\x81\x01\x90a\0P\x91\x90a\x01\xBCV[a\0\x9BV[`@Qa\0b\x91\x90a\x02KV[`@Q\x80\x91\x03\x90\xF3[a\0\x85`\x04\x806\x03\x81\x01\x90a\0\x80\x91\x90a\x02fV[a\0\xA9V[`@Qa\0\x92\x91\x90a\x02KV[`@Q\x80\x91\x03\x90\xF3[`\0`\x01\x90P\x94\x93PPPPV[`\0`\x01\x90P\x93\x92PPPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\0\xD6\x81a\0\xC0V[\x81\x14a\0\xE1W`\0\x80\xFD[PV[`\0\x815\x90Pa\0\xF3\x81a\0\xCDV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x01$\x82a\0\xF9V[\x90P\x91\x90PV[a\x014\x81a\x01\x19V[\x81\x14a\x01?W`\0\x80\xFD[PV[`\0\x815\x90Pa\x01Q\x81a\x01+V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x01|Wa\x01{a\x01WV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x99Wa\x01\x98a\x01\\V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x01\xB5Wa\x01\xB4a\x01aV[[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x01\xD6Wa\x01\xD5a\0\xB6V[[`\0a\x01\xE4\x87\x82\x88\x01a\0\xE4V[\x94PP` a\x01\xF5\x87\x82\x88\x01a\x01BV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x16Wa\x02\x15a\0\xBBV[[a\x02\"\x87\x82\x88\x01a\x01fV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x02E\x81a\x020V[\x82RPPV[`\0` \x82\x01\x90Pa\x02``\0\x83\x01\x84a\x02<V[\x92\x91PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x02\x7FWa\x02~a\0\xB6V[[`\0a\x02\x8D\x86\x82\x87\x01a\x01BV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xAEWa\x02\xADa\0\xBBV[[a\x02\xBA\x86\x82\x87\x01a\x01fV[\x92P\x92PP\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 Ps=E\x08\xDF-\x1D^~p\x15\x80\x9B_\xF4\xC7:V\x9C\xDD\x0B\xDD\rdh\x07\x90#\x93\x0F\xF0dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKWHITELIST_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x12]\xDFM\x14a\0;W\x80cr%\x85\xD5\x14a\0kW[`\0\x80\xFD[a\0U`\x04\x806\x03\x81\x01\x90a\0P\x91\x90a\x01\xBCV[a\0\x9BV[`@Qa\0b\x91\x90a\x02KV[`@Q\x80\x91\x03\x90\xF3[a\0\x85`\x04\x806\x03\x81\x01\x90a\0\x80\x91\x90a\x02fV[a\0\xA9V[`@Qa\0\x92\x91\x90a\x02KV[`@Q\x80\x91\x03\x90\xF3[`\0`\x01\x90P\x94\x93PPPPV[`\0`\x01\x90P\x93\x92PPPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\0\xD6\x81a\0\xC0V[\x81\x14a\0\xE1W`\0\x80\xFD[PV[`\0\x815\x90Pa\0\xF3\x81a\0\xCDV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x01$\x82a\0\xF9V[\x90P\x91\x90PV[a\x014\x81a\x01\x19V[\x81\x14a\x01?W`\0\x80\xFD[PV[`\0\x815\x90Pa\x01Q\x81a\x01+V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x01|Wa\x01{a\x01WV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x99Wa\x01\x98a\x01\\V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x01\xB5Wa\x01\xB4a\x01aV[[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x01\xD6Wa\x01\xD5a\0\xB6V[[`\0a\x01\xE4\x87\x82\x88\x01a\0\xE4V[\x94PP` a\x01\xF5\x87\x82\x88\x01a\x01BV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x16Wa\x02\x15a\0\xBBV[[a\x02\"\x87\x82\x88\x01a\x01fV[\x92P\x92PP\x92\x95\x91\x94P\x92PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x02E\x81a\x020V[\x82RPPV[`\0` \x82\x01\x90Pa\x02``\0\x83\x01\x84a\x02<V[\x92\x91PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x02\x7FWa\x02~a\0\xB6V[[`\0a\x02\x8D\x86\x82\x87\x01a\x01BV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xAEWa\x02\xADa\0\xBBV[[a\x02\xBA\x86\x82\x87\x01a\x01fV[\x92P\x92PP\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 Ps=E\x08\xDF-\x1D^~p\x15\x80\x9B_\xF4\xC7:V\x9C\xDD\x0B\xDD\rdh\x07\x90#\x93\x0F\xF0dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKWHITELIST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockWhitelist<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockWhitelist<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockWhitelist<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockWhitelist<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockWhitelist<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockWhitelist))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockWhitelist<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKWHITELIST_ABI.clone(),
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
                MOCKWHITELIST_ABI.clone(),
                MOCKWHITELIST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `isWhitelistedBorrower` (0x125ddf4d) function
        pub fn is_whitelisted_borrower(
            &self,
            p0: u8,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([18, 93, 223, 77], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWhitelistedLender` (0x722585d5) function
        pub fn is_whitelisted_lender(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([114, 37, 133, 213], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockWhitelist<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `isWhitelistedBorrower` function with signature `isWhitelistedBorrower(uint8,address,bytes32[])` and selector `0x125ddf4d`
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
        name = "isWhitelistedBorrower",
        abi = "isWhitelistedBorrower(uint8,address,bytes32[])"
    )]
    pub struct IsWhitelistedBorrowerCall(
        pub u8,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<[u8; 32]>,
    );
    ///Container type for all input parameters for the `isWhitelistedLender` function with signature `isWhitelistedLender(address,bytes32[])` and selector `0x722585d5`
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
        name = "isWhitelistedLender",
        abi = "isWhitelistedLender(address,bytes32[])"
    )]
    pub struct IsWhitelistedLenderCall(
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<[u8; 32]>,
    );
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
    pub enum MockWhitelistCalls {
        IsWhitelistedBorrower(IsWhitelistedBorrowerCall),
        IsWhitelistedLender(IsWhitelistedLenderCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockWhitelistCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsWhitelistedBorrowerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsWhitelistedBorrower(decoded));
            }
            if let Ok(decoded) = <IsWhitelistedLenderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsWhitelistedLender(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockWhitelistCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsWhitelistedBorrower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsWhitelistedLender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockWhitelistCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsWhitelistedBorrower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsWhitelistedLender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IsWhitelistedBorrowerCall> for MockWhitelistCalls {
        fn from(value: IsWhitelistedBorrowerCall) -> Self {
            Self::IsWhitelistedBorrower(value)
        }
    }
    impl ::core::convert::From<IsWhitelistedLenderCall> for MockWhitelistCalls {
        fn from(value: IsWhitelistedLenderCall) -> Self {
            Self::IsWhitelistedLender(value)
        }
    }
    ///Container type for all return fields from the `isWhitelistedBorrower` function with signature `isWhitelistedBorrower(uint8,address,bytes32[])` and selector `0x125ddf4d`
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
    pub struct IsWhitelistedBorrowerReturn(pub bool);
    ///Container type for all return fields from the `isWhitelistedLender` function with signature `isWhitelistedLender(address,bytes32[])` and selector `0x722585d5`
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
    pub struct IsWhitelistedLenderReturn(pub bool);
}
