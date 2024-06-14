use casper_types::{
    account::Account,
    system::auction::{Bid, EraInfo, UnbondingPurse, WithdrawPurse},
    CLValue, Contract, ContractPackage, ContractWasm, DeployInfo, Transfer,
};
use serde::Serialize;

#[derive(Eq, PartialEq, Clone, Debug, Serialize)]
pub enum JSCompatibleStoredValue<'a> {
    CLValue(&'a CLValue),
    Account(&'a Account),
    ContractWasm(&'a ContractWasm),
    Contract(&'a Contract),
    ContractPackage(&'a ContractPackage),
    Transfer(&'a Transfer),
    DeployInfo(&'a DeployInfo),
    EraInfo(&'a EraInfo),
    Bid(&'a Bid),
    Withdraw(&'a Vec<WithdrawPurse>),
    Unbonding(&'a Vec<UnbondingPurse>),
}

impl<'a> From<&'a casper_types::StoredValue> for JSCompatibleStoredValue<'a> {
    fn from(stored_value: &'a casper_types::StoredValue) -> Self {
        use JSCompatibleStoredValue::*;
        match stored_value {
            casper_types::StoredValue::CLValue(cl_value) => CLValue(cl_value),
            casper_types::StoredValue::Account(account) => Account(account),
            casper_types::StoredValue::ContractWasm(contract_wasm) => ContractWasm(contract_wasm),
            casper_types::StoredValue::Contract(contract) => Contract(contract),
            casper_types::StoredValue::ContractPackage(contract_package) => {
                ContractPackage(contract_package)
            }
            casper_types::StoredValue::Transfer(transfer) => Transfer(transfer),
            casper_types::StoredValue::DeployInfo(deploy_info) => DeployInfo(deploy_info),
            casper_types::StoredValue::EraInfo(era_info) => EraInfo(era_info),
            casper_types::StoredValue::Bid(bid) => Bid(bid),
            casper_types::StoredValue::Withdraw(withdraw) => Withdraw(withdraw),
            casper_types::StoredValue::Unbonding(unbonding) => Unbonding(unbonding),
        }
    }
}
