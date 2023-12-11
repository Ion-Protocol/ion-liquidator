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
                        name: ::std::borrow::ToOwned::to_owned("ilkAmounts"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r\x828\x03\x80a\r\x82\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x08qV[`\0\x83`\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0QWa\0Pa\x08\xC4V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x8AW\x81` \x01[a\0wa\x07dV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0oW\x90P[P\x90P`\0[\x84`\xFF\x16\x81`\xFF\x16\x10\x15a\x05iW`\0a\x01@`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7F()\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x012a\0\xEE\x85`\xFF\x16a\x06\x99` \x1B` \x1CV[`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7FLIQUIDATION_THRESHOLD_\0\0\0\0\0\0\0\0\0\0\x81RPa\x06\xEA` \x1B\x90\x91\x90` \x1CV[a\x06\xEA` \x1B\x90\x91\x90` \x1CV[\x90P`\0\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`@Qa\x01\x80\x91\x90a\tdV[`@Q\x80\x91\x03\x90 {\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x01\xE2\x91\x90a\t\xC2V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x02\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\"V[``\x91P[P\x91P\x91P\x81a\x02gW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02^\x90a\n\\V[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x02}\x91\x90a\n\xB2V[\x90P`\0a\x03!`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7F()\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x03\x13a\x02\xCF\x89`\xFF\x16a\x06\x99` \x1B` \x1CV[`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FMAX_DISCOUNT_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x06\xEA` \x1B\x90\x91\x90` \x1CV[a\x06\xEA` \x1B\x90\x91\x90` \x1CV[\x90P`\0\x80\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`@Qa\x03a\x91\x90a\tdV[`@Q\x80\x91\x03\x90 {\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03\xC3\x91\x90a\t\xC2V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x03\xFEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\x03V[``\x91P[P\x91P\x91P\x81a\x04HW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04?\x90a\x0BQV[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x04^\x91\x90a\n\xB2V[\x90P\x84\x8A\x8A`\xFF\x16\x81Q\x81\x10a\x04wWa\x04va\x0BqV[[` \x02` \x01\x01Q`\0\x01\x81\x81RPP\x80\x8A\x8A`\xFF\x16\x81Q\x81\x10a\x04\x9EWa\x04\x9Da\x0BqV[[` \x02` \x01\x01Q` \x01\x81\x81RPP\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B\xA7e\x07\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xE7\x91\x90a\x0B\xAFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05(\x91\x90a\n\xB2V[\x8A\x8A`\xFF\x16\x81Q\x81\x10a\x05>Wa\x05=a\x0BqV[[` \x02` \x01\x01Q`@\x01\x81\x81RPPPPPPPPPP\x80\x80a\x05a\x90a\x0B\xF9V[\x91PPa\0\x90V[P`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cI\xEDd4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xDB\x91\x90a\n\xB2V[\x90P`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAES\x953`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06N\x91\x90a\n\xB2V[\x90P`\0`@Q\x80`@\x01`@R\x80\x84\x81R` \x01\x83\x81RP\x90P`\0\x84\x82`@Q` \x01a\x06~\x92\x91\x90a\rQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[```\x80`@Q\x01\x90P` \x81\x01`@R`\0\x81R\x80`\0\x19\x83[`\x01\x15a\x06\xD5W\x81\x84\x01\x93P`\n\x81\x06`0\x01\x84S`\n\x81\x04\x90P\x80a\x06\xB4W[P\x82\x82\x03` \x84\x03\x93P\x80\x84RPPP\x91\x90PV[```\x1F\x19`@Q\x91P\x83Q\x81` \x82\x01\x16[`\x01\x15a\x07\x17W\x80\x86\x01Q\x81\x85\x01R\x82\x81\x01\x90P\x80a\x06\xFDW[P\x83Q\x81\x84\x01\x83` \x83\x01\x16[`\x01\x15a\x07>W\x80\x87\x01Q\x81\x83\x01R\x84\x81\x01\x90P\x80a\x07$W[P\x81\x83\x01\x80` \x87\x01\x01`\0\x81R\x81\x87R\x85`\x1F\x82\x01\x16`@RPPPPPP\x92\x91PPV[`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80\xFD[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x07\xA0\x81a\x07\x8AV[\x81\x14a\x07\xABW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x07\xBD\x81a\x07\x97V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x07\xEE\x82a\x07\xC3V[\x90P\x91\x90PV[`\0a\x08\0\x82a\x07\xE3V[\x90P\x91\x90PV[a\x08\x10\x81a\x07\xF5V[\x81\x14a\x08\x1BW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x08-\x81a\x08\x07V[\x92\x91PPV[`\0a\x08>\x82a\x07\xE3V[\x90P\x91\x90PV[a\x08N\x81a\x083V[\x81\x14a\x08YW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x08k\x81a\x08EV[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x08\x8AWa\x08\x89a\x07\x85V[[`\0a\x08\x98\x86\x82\x87\x01a\x07\xAEV[\x93PP` a\x08\xA9\x86\x82\x87\x01a\x08\x1EV[\x92PP`@a\x08\xBA\x86\x82\x87\x01a\x08\\V[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\t'W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\t\x0CV[`\0\x84\x84\x01RPPPPV[`\0a\t>\x82a\x08\xF3V[a\tH\x81\x85a\x08\xFEV[\x93Pa\tX\x81\x85` \x86\x01a\t\tV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\tp\x82\x84a\t3V[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0a\t\x9C\x82a\t{V[a\t\xA6\x81\x85a\t\x86V[\x93Pa\t\xB6\x81\x85` \x86\x01a\t\tV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\t\xCE\x82\x84a\t\x91V[\x91P\x81\x90P\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FFetchParameters: failed to fetch`\0\x82\x01R\x7F liquidation threshold\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\nF`6\x83a\t\xD9V[\x91Pa\nQ\x82a\t\xEAV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\nu\x81a\n9V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\n\x8F\x81a\n|V[\x81\x14a\n\x9AW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\n\xAC\x81a\n\x86V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\xC8Wa\n\xC7a\x07\x85V[[`\0a\n\xD6\x84\x82\x85\x01a\n\x9DV[\x91PP\x92\x91PPV[\x7FFetchParameters: failed to fetch`\0\x82\x01R\x7F max discount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x0B;`-\x83a\t\xD9V[\x91Pa\x0BF\x82a\n\xDFV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0Bj\x81a\x0B.V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x0B\xA9\x81a\x07\x8AV[\x82RPPV[`\0` \x82\x01\x90Pa\x0B\xC4`\0\x83\x01\x84a\x0B\xA0V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x0C\x04\x82a\x07\x8AV[\x91P`\xFF\x82\x03a\x0C\x17Wa\x0C\x16a\x0B\xCAV[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x0CW\x81a\n|V[\x82RPPV[``\x82\x01`\0\x82\x01Qa\x0Cs`\0\x85\x01\x82a\x0CNV[P` \x82\x01Qa\x0C\x86` \x85\x01\x82a\x0CNV[P`@\x82\x01Qa\x0C\x99`@\x85\x01\x82a\x0CNV[PPPPV[`\0a\x0C\xAB\x83\x83a\x0C]V[``\x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x0C\xCF\x82a\x0C\"V[a\x0C\xD9\x81\x85a\x0C-V[\x93Pa\x0C\xE4\x83a\x0C>V[\x80`\0[\x83\x81\x10\x15a\r\x15W\x81Qa\x0C\xFC\x88\x82a\x0C\x9FV[\x97Pa\r\x07\x83a\x0C\xB7V[\x92PP`\x01\x81\x01\x90Pa\x0C\xE8V[P\x85\x93PPPP\x92\x91PPV[`@\x82\x01`\0\x82\x01Qa\r8`\0\x85\x01\x82a\x0CNV[P` \x82\x01Qa\rK` \x85\x01\x82a\x0CNV[PPPPV[`\0``\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\rk\x81\x85a\x0C\xC4V[\x90Pa\rz` \x83\x01\x84a\r\"V[\x93\x92PPPV\xFE";
    /// The bytecode of the contract.
    pub static FETCHPARAMETERS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 y\x8E\x85\x81\x8A,\xEF\x11\x1B\xA6d n\xA5\xC5\xE6>\"\x13o\x99F\xF7\x9F\xAACp\xC7c\x06\xC9\x7FdsolcC\0\x08\x15\x003";
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
