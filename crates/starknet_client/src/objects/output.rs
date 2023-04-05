pub mod transaction {

    use serde::{Deserialize, Serialize};
    use starknet_api::hash::StarkFelt;
    use starknet_api::transaction::TransactionHash;

    #[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
    #[serde(tag = "fee_estimation")]
    pub struct FeeEstimate {
        pub overall_fee: u128,
        pub gas_price: u128,
        pub gas_usage: u128,
        pub unit: String,
    }

    #[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
    pub struct CommonTransactionFieldsResult {
        pub code: String,
        pub transaction_hash: TransactionHash,
    }

    #[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
    pub struct InvokeTransactionResult {
        #[serde(flatten)]
        pub common_fields: CommonTransactionFieldsResult,
    }

    #[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
    pub struct DeclareTransactionResult {
        #[serde(flatten)]
        pub common_fields: CommonTransactionFieldsResult,
        pub class_hash: StarkFelt,
    }

    #[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
    pub struct DeployTransactionResult {
        #[serde(flatten)]
        pub common_fields: CommonTransactionFieldsResult,
        pub address: StarkFelt,
    }

    #[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
    #[serde(untagged)]
    pub enum AddTransactionResult {
        Declare(DeclareTransactionResult),
        Deploy(DeployTransactionResult),
        Invoke(InvokeTransactionResult),
    }
}
