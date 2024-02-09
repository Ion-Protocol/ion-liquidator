pub use mock_swell::*;
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
pub mod mock_swell {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRate"),
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
                    ::std::borrow::ToOwned::to_owned("setNewRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setNewRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newRate"),
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
                    ::std::borrow::ToOwned::to_owned("swETHToETHRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swETHToETHRate"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKSWELL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@Rg\x0F\xF5\x9E\xE83\xB3\0\0_U4\x80\x15a\0\x1AW_\x80\xFD[Pa\x01t\x80a\0(_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80c-H;\xEC\x14a\0CW\x80cg\x9A\xEF\xCE\x14a\0_W\x80c\xD6\x8B,\xB6\x14a\0}W[_\x80\xFD[a\0]`\x04\x806\x03\x81\x01\x90a\0X\x91\x90a\0\xEBV[a\0\x9BV[\0[a\0ga\0\xA4V[`@Qa\0t\x91\x90a\x01%V[`@Q\x80\x91\x03\x90\xF3[a\0\x85a\0\xACV[`@Qa\0\x92\x91\x90a\x01%V[`@Q\x80\x91\x03\x90\xF3[\x80_\x81\x90UPPV[_\x80T\x90P\x90V[_\x80T\x90P\x90V[_\x80\xFD[_\x81\x90P\x91\x90PV[a\0\xCA\x81a\0\xB8V[\x81\x14a\0\xD4W_\x80\xFD[PV[_\x815\x90Pa\0\xE5\x81a\0\xC1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x01\0Wa\0\xFFa\0\xB4V[[_a\x01\r\x84\x82\x85\x01a\0\xD7V[\x91PP\x92\x91PPV[a\x01\x1F\x81a\0\xB8V[\x82RPPV[_` \x82\x01\x90Pa\x018_\x83\x01\x84a\x01\x16V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xBB\xF5\x98\xE5(\x1B\x19r\xC7A\x96\x872\xF4\x1F\x89\x9F\xA3\x82Q\t\xAC\xCF$\x96r%e\x92.AYdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKSWELL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80c-H;\xEC\x14a\0CW\x80cg\x9A\xEF\xCE\x14a\0_W\x80c\xD6\x8B,\xB6\x14a\0}W[_\x80\xFD[a\0]`\x04\x806\x03\x81\x01\x90a\0X\x91\x90a\0\xEBV[a\0\x9BV[\0[a\0ga\0\xA4V[`@Qa\0t\x91\x90a\x01%V[`@Q\x80\x91\x03\x90\xF3[a\0\x85a\0\xACV[`@Qa\0\x92\x91\x90a\x01%V[`@Q\x80\x91\x03\x90\xF3[\x80_\x81\x90UPPV[_\x80T\x90P\x90V[_\x80T\x90P\x90V[_\x80\xFD[_\x81\x90P\x91\x90PV[a\0\xCA\x81a\0\xB8V[\x81\x14a\0\xD4W_\x80\xFD[PV[_\x815\x90Pa\0\xE5\x81a\0\xC1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x01\0Wa\0\xFFa\0\xB4V[[_a\x01\r\x84\x82\x85\x01a\0\xD7V[\x91PP\x92\x91PPV[a\x01\x1F\x81a\0\xB8V[\x82RPPV[_` \x82\x01\x90Pa\x018_\x83\x01\x84a\x01\x16V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xBB\xF5\x98\xE5(\x1B\x19r\xC7A\x96\x872\xF4\x1F\x89\x9F\xA3\x82Q\t\xAC\xCF$\x96r%e\x92.AYdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKSWELL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockSwell<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockSwell<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockSwell<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockSwell<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockSwell<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockSwell)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockSwell<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKSWELL_ABI.clone(),
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
                MOCKSWELL_ABI.clone(),
                MOCKSWELL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getRate` (0x679aefce) function
        pub fn get_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 154, 239, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNewRate` (0x2d483bec) function
        pub fn set_new_rate(
            &self,
            new_rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 72, 59, 236], new_rate)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swETHToETHRate` (0xd68b2cb6) function
        pub fn sw_eth_to_eth_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([214, 139, 44, 182], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockSwell<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getRate` function with signature `getRate()` and selector `0x679aefce`
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
    #[ethcall(name = "getRate", abi = "getRate()")]
    pub struct GetRateCall;
    ///Container type for all input parameters for the `setNewRate` function with signature `setNewRate(uint256)` and selector `0x2d483bec`
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
    #[ethcall(name = "setNewRate", abi = "setNewRate(uint256)")]
    pub struct SetNewRateCall {
        pub new_rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swETHToETHRate` function with signature `swETHToETHRate()` and selector `0xd68b2cb6`
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
    #[ethcall(name = "swETHToETHRate", abi = "swETHToETHRate()")]
    pub struct SwETHToETHRateCall;
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
    pub enum MockSwellCalls {
        GetRate(GetRateCall),
        SetNewRate(SetNewRateCall),
        SwETHToETHRate(SwETHToETHRateCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockSwellCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRate(decoded));
            }
            if let Ok(decoded) = <SetNewRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNewRate(decoded));
            }
            if let Ok(decoded) = <SwETHToETHRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwETHToETHRate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockSwellCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetNewRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwETHToETHRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockSwellCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNewRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwETHToETHRate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetRateCall> for MockSwellCalls {
        fn from(value: GetRateCall) -> Self {
            Self::GetRate(value)
        }
    }
    impl ::core::convert::From<SetNewRateCall> for MockSwellCalls {
        fn from(value: SetNewRateCall) -> Self {
            Self::SetNewRate(value)
        }
    }
    impl ::core::convert::From<SwETHToETHRateCall> for MockSwellCalls {
        fn from(value: SwETHToETHRateCall) -> Self {
            Self::SwETHToETHRate(value)
        }
    }
    ///Container type for all return fields from the `getRate` function with signature `getRate()` and selector `0x679aefce`
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
    pub struct GetRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `swETHToETHRate` function with signature `swETHToETHRate()` and selector `0xd68b2cb6`
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
    pub struct SwETHToETHRateReturn(pub ::ethers::core::types::U256);
}
