pub use fetch_rates_and_exchange_rates::*;
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
pub mod fetch_rates_and_exchange_rates {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("amountIlks"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("liquidationContract"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("ionPool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IonPool"),
                        ),
                    },
                ],
            }),
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FETCHRATESANDEXCHANGERATES_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\n\xAA8\x03\x80a\n\xAA\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x06\x10V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA6\xAF\xED\x95`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\0\x9CWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\x99\x91\x90a\x06\x99V[`\x01[\x15a\0\xA3WP[`\0\x83`\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xC2Wa\0\xC1a\x06\xC6V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xFBW\x81` \x01[a\0\xE8a\x05\x1CV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xE0W\x90P[P\x90P`\0[`\x02\x81`\xFF\x16\x10\x15a\x04\"W`\0a\x01\xAF`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7F()\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x01\xA1a\x01]\x85`\xFF\x16a\x04Q` \x1B` \x1CV[`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01\x7FRESERVE_ORACLE_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\xA2` \x1B\x90\x91\x90` \x1CV[a\x04\xA2` \x1B\x90\x91\x90` \x1CV[\x90P`\0\x80\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`@Qa\x01\xEF\x91\x90a\x07fV[`@Q\x80\x91\x03\x90 {\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x02Q\x91\x90a\x07\xC4V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x02\x8CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\x91V[``\x91P[P\x91P\x91P\x81a\x02\xD6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xCD\x90a\x08^V[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x02\xEC\x91\x90a\x08\xBCV[\x90P`@Q\x80`@\x01`@R\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03h\x91\x90a\x06\x99V[\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xA6\x91\x90a\x08\xF8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE7\x91\x90a\x06\x99V[\x81RP\x86\x86`\xFF\x16\x81Q\x81\x10a\x04\0Wa\x03\xFFa\t\x13V[[` \x02` \x01\x01\x81\x90RPPPPP\x80\x80a\x04\x1A\x90a\tqV[\x91PPa\x01\x01V[P`\0\x81`@Q` \x01a\x046\x91\x90a\n\x87V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[```\x80`@Q\x01\x90P` \x81\x01`@R`\0\x81R\x80`\0\x19\x83[`\x01\x15a\x04\x8DW\x81\x84\x01\x93P`\n\x81\x06`0\x01\x84S`\n\x81\x04\x90P\x80a\x04lW[P\x82\x82\x03` \x84\x03\x93P\x80\x84RPPP\x91\x90PV[```\x1F\x19`@Q\x91P\x83Q\x81` \x82\x01\x16[`\x01\x15a\x04\xCFW\x80\x86\x01Q\x81\x85\x01R\x82\x81\x01\x90P\x80a\x04\xB5W[P\x83Q\x81\x84\x01\x83` \x83\x01\x16[`\x01\x15a\x04\xF6W\x80\x87\x01Q\x81\x83\x01R\x84\x81\x01\x90P\x80a\x04\xDCW[P\x81\x83\x01\x80` \x87\x01\x01`\0\x81R\x81\x87R\x85`\x1F\x82\x01\x16`@RPPPPPP\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x05Q\x81a\x05;V[\x81\x14a\x05\\W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x05n\x81a\x05HV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x05\x9F\x82a\x05tV[\x90P\x91\x90PV[a\x05\xAF\x81a\x05\x94V[\x81\x14a\x05\xBAW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x05\xCC\x81a\x05\xA6V[\x92\x91PPV[`\0a\x05\xDD\x82a\x05\x94V[\x90P\x91\x90PV[a\x05\xED\x81a\x05\xD2V[\x81\x14a\x05\xF8W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x06\n\x81a\x05\xE4V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06)Wa\x06(a\x056V[[`\0a\x067\x86\x82\x87\x01a\x05_V[\x93PP` a\x06H\x86\x82\x87\x01a\x05\xBDV[\x92PP`@a\x06Y\x86\x82\x87\x01a\x05\xFBV[\x91PP\x92P\x92P\x92V[`\0\x81\x90P\x91\x90PV[a\x06v\x81a\x06cV[\x81\x14a\x06\x81W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x06\x93\x81a\x06mV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x06\xAFWa\x06\xAEa\x056V[[`\0a\x06\xBD\x84\x82\x85\x01a\x06\x84V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x07)W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x07\x0EV[`\0\x84\x84\x01RPPPPV[`\0a\x07@\x82a\x06\xF5V[a\x07J\x81\x85a\x07\0V[\x93Pa\x07Z\x81\x85` \x86\x01a\x07\x0BV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x07r\x82\x84a\x075V[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0a\x07\x9E\x82a\x07}V[a\x07\xA8\x81\x85a\x07\x88V[\x93Pa\x07\xB8\x81\x85` \x86\x01a\x07\x0BV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x07\xD0\x82\x84a\x07\x93V[\x91P\x81\x90P\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FFetchRatesAndExchangeRates: fail`\0\x82\x01R\x7Fed to fetch reserve oracle\0\0\0\0\0\0` \x82\x01RPV[`\0a\x08H`:\x83a\x07\xDBV[\x91Pa\x08S\x82a\x07\xECV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x08w\x81a\x08;V[\x90P\x91\x90PV[`\0a\x08\x89\x82a\x05\x94V[\x90P\x91\x90PV[a\x08\x99\x81a\x08~V[\x81\x14a\x08\xA4W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x08\xB6\x81a\x08\x90V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08\xD2Wa\x08\xD1a\x056V[[`\0a\x08\xE0\x84\x82\x85\x01a\x08\xA7V[\x91PP\x92\x91PPV[a\x08\xF2\x81a\x05;V[\x82RPPV[`\0` \x82\x01\x90Pa\t\r`\0\x83\x01\x84a\x08\xE9V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\t|\x82a\x05;V[\x91P`\xFF\x82\x03a\t\x8FWa\t\x8Ea\tBV[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\t\xCF\x81a\x06cV[\x82RPPV[`@\x82\x01`\0\x82\x01Qa\t\xEB`\0\x85\x01\x82a\t\xC6V[P` \x82\x01Qa\t\xFE` \x85\x01\x82a\t\xC6V[PPPPV[`\0a\n\x10\x83\x83a\t\xD5V[`@\x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\n4\x82a\t\x9AV[a\n>\x81\x85a\t\xA5V[\x93Pa\nI\x83a\t\xB6V[\x80`\0[\x83\x81\x10\x15a\nzW\x81Qa\na\x88\x82a\n\x04V[\x97Pa\nl\x83a\n\x1CV[\x92PP`\x01\x81\x01\x90Pa\nMV[P\x85\x93PPPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\n\xA1\x81\x84a\n)V[\x90P\x92\x91PPV\xFE";
    /// The bytecode of the contract.
    pub static FETCHRATESANDEXCHANGERATES_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 [\xE9\x99/\xA4X5\xFFf\xF8\n\xC9x\0\xD54%\xE8\x8E\xD3h\xCD\xEE\xC6\n\xAF\xF4\x90\xAF\x126\xEAdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static FETCHRATESANDEXCHANGERATES_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FetchRatesAndExchangeRates<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FetchRatesAndExchangeRates<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FetchRatesAndExchangeRates<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FetchRatesAndExchangeRates<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FetchRatesAndExchangeRates<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FetchRatesAndExchangeRates))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FetchRatesAndExchangeRates<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FETCHRATESANDEXCHANGERATES_ABI.clone(),
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
                FETCHRATESANDEXCHANGERATES_ABI.clone(),
                FETCHRATESANDEXCHANGERATES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FetchRatesAndExchangeRates<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
