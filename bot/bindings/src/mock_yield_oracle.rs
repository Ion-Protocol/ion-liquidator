pub use mock_yield_oracle::*;
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
pub mod mock_yield_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("apys"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("apys"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
    pub static MOCKYIELDORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@Rb4\xA4\x90`\0\x80a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP4\x80\x15a\x003W`\0\x80\xFD[Pa\x01S\x80a\0C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x81\xCE\x1C#\x14a\x000W[`\0\x80\xFD[a\0J`\x04\x806\x03\x81\x01\x90a\0E\x91\x90a\0\xB6V[a\0`V[`@Qa\0W\x91\x90a\x01\x02V[`@Q\x80\x91\x03\x90\xF3[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\0\x93\x81a\0\x80V[\x81\x14a\0\x9EW`\0\x80\xFD[PV[`\0\x815\x90Pa\0\xB0\x81a\0\x8AV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\0\xCCWa\0\xCBa\0{V[[`\0a\0\xDA\x84\x82\x85\x01a\0\xA1V[\x91PP\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\0\xFC\x81a\0\xE3V[\x82RPPV[`\0` \x82\x01\x90Pa\x01\x17`\0\x83\x01\x84a\0\xF3V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xCB0\x0FF@\xB3z\xB9\x15\xA0\xC1\x8A\xD4\x82\xB4\xC8\xD9w\x97\xE6\xCF=\xFA\x89\xCF\xD1\x9D3m\xF7\xC9SdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOCKYIELDORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x81\xCE\x1C#\x14a\x000W[`\0\x80\xFD[a\0J`\x04\x806\x03\x81\x01\x90a\0E\x91\x90a\0\xB6V[a\0`V[`@Qa\0W\x91\x90a\x01\x02V[`@Q\x80\x91\x03\x90\xF3[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\0\x93\x81a\0\x80V[\x81\x14a\0\x9EW`\0\x80\xFD[PV[`\0\x815\x90Pa\0\xB0\x81a\0\x8AV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\0\xCCWa\0\xCBa\0{V[[`\0a\0\xDA\x84\x82\x85\x01a\0\xA1V[\x91PP\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\0\xFC\x81a\0\xE3V[\x82RPPV[`\0` \x82\x01\x90Pa\x01\x17`\0\x83\x01\x84a\0\xF3V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xCB0\x0FF@\xB3z\xB9\x15\xA0\xC1\x8A\xD4\x82\xB4\xC8\xD9w\x97\xE6\xCF=\xFA\x89\xCF\xD1\x9D3m\xF7\xC9SdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKYIELDORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockYieldOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockYieldOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockYieldOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockYieldOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockYieldOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockYieldOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockYieldOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKYIELDORACLE_ABI.clone(),
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
                MOCKYIELDORACLE_ABI.clone(),
                MOCKYIELDORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `apys` (0x81ce1c23) function
        pub fn apys(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([129, 206, 28, 35], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockYieldOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `apys` function with signature `apys(uint256)` and selector `0x81ce1c23`
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
    #[ethcall(name = "apys", abi = "apys(uint256)")]
    pub struct ApysCall(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `apys` function with signature `apys(uint256)` and selector `0x81ce1c23`
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
    pub struct ApysReturn(pub u32);
}
