pub use mock_stader::*;
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
pub mod mock_stader {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getExchangeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getExchangeRate"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKSTADER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@Rg\x0FC\xFC,\x04\xEE\0\0_U4\x80\x15a\0\x1AW_\x80\xFD[Pa\x01C\x80a\0(_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80c-H;\xEC\x14a\08W\x80c\xE6\xAA!l\x14a\0TW[_\x80\xFD[a\0R`\x04\x806\x03\x81\x01\x90a\0M\x91\x90a\0\xBAV[a\0rV[\0[a\0\\a\0{V[`@Qa\0i\x91\x90a\0\xF4V[`@Q\x80\x91\x03\x90\xF3[\x80_\x81\x90UPPV[_\x80T\x90P\x90V[_\x80\xFD[_\x81\x90P\x91\x90PV[a\0\x99\x81a\0\x87V[\x81\x14a\0\xA3W_\x80\xFD[PV[_\x815\x90Pa\0\xB4\x81a\0\x90V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\0\xCFWa\0\xCEa\0\x83V[[_a\0\xDC\x84\x82\x85\x01a\0\xA6V[\x91PP\x92\x91PPV[a\0\xEE\x81a\0\x87V[\x82RPPV[_` \x82\x01\x90Pa\x01\x07_\x83\x01\x84a\0\xE5V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 I\xAD\xBE\xA9Y\x10\xF2\xADs\xC4\xE6\xF2\xE8M\xDBb\x84\xDC\x05\xE9r\xEB\x1D\xA5O\xDF?\xB1\x96\xF8l\x15dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKSTADER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80c-H;\xEC\x14a\08W\x80c\xE6\xAA!l\x14a\0TW[_\x80\xFD[a\0R`\x04\x806\x03\x81\x01\x90a\0M\x91\x90a\0\xBAV[a\0rV[\0[a\0\\a\0{V[`@Qa\0i\x91\x90a\0\xF4V[`@Q\x80\x91\x03\x90\xF3[\x80_\x81\x90UPPV[_\x80T\x90P\x90V[_\x80\xFD[_\x81\x90P\x91\x90PV[a\0\x99\x81a\0\x87V[\x81\x14a\0\xA3W_\x80\xFD[PV[_\x815\x90Pa\0\xB4\x81a\0\x90V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\0\xCFWa\0\xCEa\0\x83V[[_a\0\xDC\x84\x82\x85\x01a\0\xA6V[\x91PP\x92\x91PPV[a\0\xEE\x81a\0\x87V[\x82RPPV[_` \x82\x01\x90Pa\x01\x07_\x83\x01\x84a\0\xE5V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 I\xAD\xBE\xA9Y\x10\xF2\xADs\xC4\xE6\xF2\xE8M\xDBb\x84\xDC\x05\xE9r\xEB\x1D\xA5O\xDF?\xB1\x96\xF8l\x15dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKSTADER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockStader<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockStader<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockStader<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockStader<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockStader<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockStader)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockStader<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKSTADER_ABI.clone(),
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
                MOCKSTADER_ABI.clone(),
                MOCKSTADER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getExchangeRate` (0xe6aa216c) function
        pub fn get_exchange_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([230, 170, 33, 108], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockStader<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getExchangeRate` function with signature `getExchangeRate()` and selector `0xe6aa216c`
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
    #[ethcall(name = "getExchangeRate", abi = "getExchangeRate()")]
    pub struct GetExchangeRateCall;
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
    pub enum MockStaderCalls {
        GetExchangeRate(GetExchangeRateCall),
        SetNewRate(SetNewRateCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockStaderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetExchangeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetExchangeRate(decoded));
            }
            if let Ok(decoded) = <SetNewRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNewRate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockStaderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetExchangeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNewRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockStaderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetExchangeRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNewRate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetExchangeRateCall> for MockStaderCalls {
        fn from(value: GetExchangeRateCall) -> Self {
            Self::GetExchangeRate(value)
        }
    }
    impl ::core::convert::From<SetNewRateCall> for MockStaderCalls {
        fn from(value: SetNewRateCall) -> Self {
            Self::SetNewRate(value)
        }
    }
    ///Container type for all return fields from the `getExchangeRate` function with signature `getExchangeRate()` and selector `0xe6aa216c`
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
    pub struct GetExchangeRateReturn(pub ::ethers::core::types::U256);
}
