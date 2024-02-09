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
                        name: ::std::borrow::ToOwned::to_owned("liquidationContract"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Liquidation"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x04!8\x03\x80a\x04!\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x02sV[_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1FQU\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0{W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\x9F\x91\x90a\x02\xDBV[\x90P_`@Q\x80`@\x01`@R\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA3hIw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xF6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x1A\x91\x90a\x039V[\x81R` \x01\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x04\xB5G_`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01X\x91\x90a\x03\xB2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01sW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x97\x91\x90a\x039V[\x81RP\x90P_\x81`@Q` \x01a\x01\xAE\x91\x90a\x04\x07V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01\xF6\x82a\x01\xCDV[\x90P\x91\x90PV[_a\x02\x07\x82a\x01\xECV[\x90P\x91\x90PV[a\x02\x17\x81a\x01\xFDV[\x81\x14a\x02!W_\x80\xFD[PV[_\x81Q\x90Pa\x022\x81a\x02\x0EV[\x92\x91PPV[_a\x02B\x82a\x01\xECV[\x90P\x91\x90PV[a\x02R\x81a\x028V[\x81\x14a\x02\\W_\x80\xFD[PV[_\x81Q\x90Pa\x02m\x81a\x02IV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x02\x89Wa\x02\x88a\x01\xC9V[[_a\x02\x96\x85\x82\x86\x01a\x02$V[\x92PP` a\x02\xA7\x85\x82\x86\x01a\x02_V[\x91PP\x92P\x92\x90PV[a\x02\xBA\x81a\x01\xECV[\x81\x14a\x02\xC4W_\x80\xFD[PV[_\x81Q\x90Pa\x02\xD5\x81a\x02\xB1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x02\xF0Wa\x02\xEFa\x01\xC9V[[_a\x02\xFD\x84\x82\x85\x01a\x02\xC7V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x03\x18\x81a\x03\x06V[\x81\x14a\x03\"W_\x80\xFD[PV[_\x81Q\x90Pa\x033\x81a\x03\x0FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x03NWa\x03Ma\x01\xC9V[[_a\x03[\x84\x82\x85\x01a\x03%V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x03\x9Ca\x03\x97a\x03\x92\x84a\x03dV[a\x03yV[a\x03mV[\x90P\x91\x90PV[a\x03\xAC\x81a\x03\x82V[\x82RPPV[_` \x82\x01\x90Pa\x03\xC5_\x83\x01\x84a\x03\xA3V[\x92\x91PPV[a\x03\xD4\x81a\x03\x06V[\x82RPPV[`@\x82\x01_\x82\x01Qa\x03\xEE_\x85\x01\x82a\x03\xCBV[P` \x82\x01Qa\x04\x01` \x85\x01\x82a\x03\xCBV[PPPPV[_`@\x82\x01\x90Pa\x04\x1A_\x83\x01\x84a\x03\xDAV[\x92\x91PPV\xFE";
    /// The bytecode of the contract.
    pub static FETCHRATESANDEXCHANGERATES_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 :\xC1PBe6\x8F\x8EA]\xFE\xAD`\xD8'\x8F%Fl\x13\x15|\x88\xB8ZT\xCC\0\xC7\xB3T\x91dsolcC\0\x08\x15\x003";
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
