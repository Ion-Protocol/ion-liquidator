pub use mock_reserve_oracle::*;
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
pub mod mock_reserve_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_exchangeRate"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("currentExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentExchangeRate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("setExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setExchangeRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_exchangeRate"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKRESERVEORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x01\xD28\x03\x80a\x01\xD2\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\0tV[\x80_\x81\x90UPPa\0\x9FV[_\x80\xFD[_\x81\x90P\x91\x90PV[a\0S\x81a\0AV[\x81\x14a\0]W_\x80\xFD[PV[_\x81Q\x90Pa\0n\x81a\0JV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\0\x89Wa\0\x88a\0=V[[_a\0\x96\x84\x82\x85\x01a\0`V[\x91PP\x92\x91PPV[a\x01&\x80a\0\xAC_9_\xF3\xFE`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`\x046\x10`0W_5`\xE0\x1C\x80c\xA3hIw\x14`4W\x80c\xDB\x06\x8E\x0E\x14`NW[_\x80\xFD[`:`fV[`@Q`E\x91\x90`\x8AV[`@Q\x80\x91\x03\x90\xF3[`d`\x04\x806\x03\x81\x01\x90``\x91\x90`\xCAV[`kV[\0[_T\x81V[\x80_\x81\x90UPPV[_\x81\x90P\x91\x90PV[`\x84\x81`tV[\x82RPPV[_` \x82\x01\x90P`\x9B_\x83\x01\x84`}V[\x92\x91PPV[_\x80\xFD[`\xAC\x81`tV[\x81\x14`\xB5W_\x80\xFD[PV[_\x815\x90P`\xC4\x81`\xA5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15`\xDCW`\xDB`\xA1V[[_`\xE7\x84\x82\x85\x01`\xB8V[\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE3#ql<H\x8BT\x1E\x88\0\x05\t\x01&\xE7\x08\x15\x85\xF6\x18\xC2r\xE9\x8D\xB0k\xC8\x9F\x14\x08AdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKRESERVEORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`\x046\x10`0W_5`\xE0\x1C\x80c\xA3hIw\x14`4W\x80c\xDB\x06\x8E\x0E\x14`NW[_\x80\xFD[`:`fV[`@Q`E\x91\x90`\x8AV[`@Q\x80\x91\x03\x90\xF3[`d`\x04\x806\x03\x81\x01\x90``\x91\x90`\xCAV[`kV[\0[_T\x81V[\x80_\x81\x90UPPV[_\x81\x90P\x91\x90PV[`\x84\x81`tV[\x82RPPV[_` \x82\x01\x90P`\x9B_\x83\x01\x84`}V[\x92\x91PPV[_\x80\xFD[`\xAC\x81`tV[\x81\x14`\xB5W_\x80\xFD[PV[_\x815\x90P`\xC4\x81`\xA5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15`\xDCW`\xDB`\xA1V[[_`\xE7\x84\x82\x85\x01`\xB8V[\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE3#ql<H\x8BT\x1E\x88\0\x05\t\x01&\xE7\x08\x15\x85\xF6\x18\xC2r\xE9\x8D\xB0k\xC8\x9F\x14\x08AdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKRESERVEORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockReserveOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockReserveOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockReserveOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockReserveOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockReserveOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockReserveOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockReserveOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKRESERVEORACLE_ABI.clone(),
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
                MOCKRESERVEORACLE_ABI.clone(),
                MOCKRESERVEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `currentExchangeRate` (0xa3684977) function
        pub fn current_exchange_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([163, 104, 73, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setExchangeRate` (0xdb068e0e) function
        pub fn set_exchange_rate(
            &self,
            exchange_rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 6, 142, 14], exchange_rate)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockReserveOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `currentExchangeRate` function with signature `currentExchangeRate()` and selector `0xa3684977`
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
    #[ethcall(name = "currentExchangeRate", abi = "currentExchangeRate()")]
    pub struct CurrentExchangeRateCall;
    ///Container type for all input parameters for the `setExchangeRate` function with signature `setExchangeRate(uint256)` and selector `0xdb068e0e`
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
    #[ethcall(name = "setExchangeRate", abi = "setExchangeRate(uint256)")]
    pub struct SetExchangeRateCall {
        pub exchange_rate: ::ethers::core::types::U256,
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
    pub enum MockReserveOracleCalls {
        CurrentExchangeRate(CurrentExchangeRateCall),
        SetExchangeRate(SetExchangeRateCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockReserveOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CurrentExchangeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentExchangeRate(decoded));
            }
            if let Ok(decoded) = <SetExchangeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetExchangeRate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockReserveOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CurrentExchangeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetExchangeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockReserveOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CurrentExchangeRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetExchangeRate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CurrentExchangeRateCall> for MockReserveOracleCalls {
        fn from(value: CurrentExchangeRateCall) -> Self {
            Self::CurrentExchangeRate(value)
        }
    }
    impl ::core::convert::From<SetExchangeRateCall> for MockReserveOracleCalls {
        fn from(value: SetExchangeRateCall) -> Self {
            Self::SetExchangeRate(value)
        }
    }
    ///Container type for all return fields from the `currentExchangeRate` function with signature `currentExchangeRate()` and selector `0xa3684977`
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
    pub struct CurrentExchangeRateReturn(pub ::ethers::core::types::U256);
}
