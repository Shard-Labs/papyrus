use assert::assert_ok;

use super::super::test_utils::read_resource::read_resource_file;
use super::transaction::{DeclareTransaction, DeployTransaction, InvokeTransaction, Transaction};

#[test]
fn load_deploy_transaction_succeeds() {
    assert_ok!(serde_json::from_str::<DeployTransaction>(&read_resource_file(
        "deploy_transaction.json"
    )));
}

#[test]
fn load_invoke_transaction_succeeds() {
    assert_ok!(serde_json::from_str::<InvokeTransaction>(&read_resource_file(
        "invoke_transaction.json"
    )));
}

#[test]
fn load_declare_transaction_succeeds() {
    assert_ok!(serde_json::from_str::<DeclareTransaction>(&read_resource_file(
        "declare_transaction.json"
    )));
}

#[test]
fn load_transaction_succeeds() {
    for file_name in
        ["deploy_transaction.json", "invoke_transaction.json", "declare_transaction.json"]
    {
        assert_ok!(serde_json::from_str::<Transaction>(&read_resource_file(file_name)));
    }
}