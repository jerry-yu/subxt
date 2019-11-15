//! Implements support for the paint_contracts module.
use crate::{
    codec::{
        compact,
        Encoded,
    },
    metadata::MetadataError,
    paint::{
        balances::Balances,
        system::System,
        ModuleCalls,
    },
    Valid,
    XtBuilder,
};
use runtime_primitives::traits::{
    IdentifyAccount,
    Verify,
};
use substrate_primitives::Pair;

/// Gas units are chosen to be represented by u64 so that gas metering
/// instructions can operate on them efficiently.
pub type Gas = u64;

/// The subset of the `paint_contracts::Trait` that a client must implement.
pub trait Contracts: System + Balances {}

/// Blanket impl for using existing runtime types
impl<
        T: paint_contracts::Trait
            + paint_system::Trait
            + paint_balances::Trait
            + std::fmt::Debug,
    > Contracts for T
where
    <T as paint_system::Trait>::Header: serde::de::DeserializeOwned,
{
}

/// The Contracts extension trait for the XtBuilder.
pub trait ContractsXt {
    /// Contracts type.
    type Contracts: Contracts;
    /// Key Pair Type
    type Pair: Pair;
    /// Signature type
    type Signature: Verify;

    /// Create a call for the paint contracts module
    fn contracts<F>(
        &self,
        f: F,
    ) -> XtBuilder<Self::Contracts, Self::Pair, Self::Signature, Valid>
    where
        F: FnOnce(
            ModuleCalls<Self::Contracts, Self::Pair>,
        ) -> Result<Encoded, MetadataError>;
}

impl<T: Contracts + 'static, P, S: 'static, V> ContractsXt for XtBuilder<T, P, S, V>
where
    P: Pair,
    S: Verify,
    S::Signer: From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
{
    type Contracts = T;
    type Pair = P;
    type Signature = S;

    fn contracts<F>(&self, f: F) -> XtBuilder<T, P, S, Valid>
    where
        F: FnOnce(
            ModuleCalls<Self::Contracts, Self::Pair>,
        ) -> Result<Encoded, MetadataError>,
    {
        self.set_call("Contracts", f)
    }
}

impl<T: Contracts + 'static, P> ModuleCalls<T, P>
where
    P: Pair,
{
    /// Stores the given binary Wasm code into the chain's storage and returns
    /// its `codehash`.
    /// You can instantiate contracts only with stored code.
    pub fn put_code(
        &self,
        gas_limit: Gas,
        code: Vec<u8>,
    ) -> Result<Encoded, MetadataError> {
        self.module.call("put_code", (compact(gas_limit), code))
    }

    /// Creates a new contract from the `codehash` generated by `put_code`,
    /// optionally transferring some balance.
    ///
    /// Creation is executed as follows:
    ///
    /// - The destination address is computed based on the sender and hash of
    /// the code.
    /// - The smart-contract account is created at the computed address.
    /// - The `ctor_code` is executed in the context of the newly-created
    /// account. Buffer returned after the execution is saved as the `code`
    /// of the account. That code will be invoked upon any call received by
    /// this account.
    /// - The contract is initialized.
    pub fn create(
        &self,
        endowment: <T as Balances>::Balance,
        gas_limit: Gas,
        code_hash: <T as System>::Hash,
        data: Vec<u8>,
    ) -> Result<Encoded, MetadataError> {
        self.module.call(
            "create",
            (compact(endowment), compact(gas_limit), code_hash, data),
        )
    }

    /// Makes a call to an account, optionally transferring some balance.
    ///
    /// * If the account is a smart-contract account, the associated code will
    ///  be executed and any value will be transferred.
    /// * If the account is a regular account, any value will be transferred.
    /// * If no account exists and the call value is not less than
    /// `existential_deposit`, a regular account will be created and any value
    ///  will be transferred.
    pub fn call(
        &self,
        dest: <T as System>::Address,
        value: <T as Balances>::Balance,
        gas_limit: Gas,
        data: Vec<u8>,
    ) -> Result<Encoded, MetadataError> {
        self.module
            .call("call", (dest, compact(value), compact(gas_limit), data))
    }
}

/// Contracts Events
#[derive(parity_scale_codec::Decode)]
pub enum Event<T: System> {
    /// Contract code stored
    CodeStored(T::Hash),
}
