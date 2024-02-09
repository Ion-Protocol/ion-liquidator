pub use fetch_parameters::*;
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
pub mod fetch_parameters {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("ionPool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IonPool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("liquidationContract"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Liquidation"),
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
    pub static FETCHPARAMETERS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x04\xFF8\x03\x80a\x04\xFF\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x03mV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xA8\xAE\x9B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0{W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\x9F\x91\x90a\x03\xDEV[\x90P_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3IUi`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xEBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x0F\x91\x90a\x03\xDEV[\x90P_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B\xA7e\x07_`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01K\x91\x90a\x04WV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01fW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x8A\x91\x90a\x03\xDEV[\x90P_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cI\xEDd4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90a\x03\xDEV[\x90P_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAES\x953`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90a\x03\xDEV[\x90P_`@Q\x80`\xA0\x01`@R\x80\x87\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x83\x81RP\x90P_\x81`@Q` \x01a\x02\xA8\x91\x90a\x04\xE5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x02\xF0\x82a\x02\xC7V[\x90P\x91\x90PV[_a\x03\x01\x82a\x02\xE6V[\x90P\x91\x90PV[a\x03\x11\x81a\x02\xF7V[\x81\x14a\x03\x1BW_\x80\xFD[PV[_\x81Q\x90Pa\x03,\x81a\x03\x08V[\x92\x91PPV[_a\x03<\x82a\x02\xE6V[\x90P\x91\x90PV[a\x03L\x81a\x032V[\x81\x14a\x03VW_\x80\xFD[PV[_\x81Q\x90Pa\x03g\x81a\x03CV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x03\x83Wa\x03\x82a\x02\xC3V[[_a\x03\x90\x85\x82\x86\x01a\x03\x1EV[\x92PP` a\x03\xA1\x85\x82\x86\x01a\x03YV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a\x03\xBD\x81a\x03\xABV[\x81\x14a\x03\xC7W_\x80\xFD[PV[_\x81Q\x90Pa\x03\xD8\x81a\x03\xB4V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x03\xF3Wa\x03\xF2a\x02\xC3V[[_a\x04\0\x84\x82\x85\x01a\x03\xCAV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x04Aa\x04<a\x047\x84a\x04\tV[a\x04\x1EV[a\x04\x12V[\x90P\x91\x90PV[a\x04Q\x81a\x04'V[\x82RPPV[_` \x82\x01\x90Pa\x04j_\x83\x01\x84a\x04HV[\x92\x91PPV[a\x04y\x81a\x03\xABV[\x82RPPV[`\xA0\x82\x01_\x82\x01Qa\x04\x93_\x85\x01\x82a\x04pV[P` \x82\x01Qa\x04\xA6` \x85\x01\x82a\x04pV[P`@\x82\x01Qa\x04\xB9`@\x85\x01\x82a\x04pV[P``\x82\x01Qa\x04\xCC``\x85\x01\x82a\x04pV[P`\x80\x82\x01Qa\x04\xDF`\x80\x85\x01\x82a\x04pV[PPPPV[_`\xA0\x82\x01\x90Pa\x04\xF8_\x83\x01\x84a\x04\x7FV[\x92\x91PPV\xFE";
    /// The bytecode of the contract.
    pub static FETCHPARAMETERS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \x91\x935t\x9BB\xC6y\xEAa\xE1\xF1\x1C\x87\xB9.4(\xC5\xDC\xBA\xD0\xEF\x1A\xCCV\xDC\xCB\x11\\\xA1\\dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static FETCHPARAMETERS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FetchParameters<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FetchParameters<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FetchParameters<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FetchParameters<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FetchParameters<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FetchParameters))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FetchParameters<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FETCHPARAMETERS_ABI.clone(),
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
                FETCHPARAMETERS_ABI.clone(),
                FETCHPARAMETERS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FetchParameters<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
