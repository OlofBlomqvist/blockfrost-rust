use serde::{Deserialize, Serialize};

use crate::*;

impl BlockFrostApi {
    endpoints! {
        /// Return the information about the latest, therefore current, epoch.
        epochs_latest() -> Epoch => "/epochs/latest";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1latest/get"),

        /// Return the protocol parameters for the latest epoch.
        epochs_latest_parameters() -> EpochParameters => "/epochs/latest/parameters";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1latest~1parameters/get"),

        /// Return the content of the requested epoch.
        epochs_by_number(number: Integer) -> Epoch => "/epochs/{number}";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}/get"),

        /// Return the protocol parameters for the epoch specified
        epochs_parameters(number: Integer) -> EpochParameters => "/epochs/{number}/parameters";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1parameters/get"),
    }
    paged_endpoints! {
        /// Return the list of epochs following a specific epoch.
        epochs_next(number: Integer) -> Vec<Epoch> => "/epochs/{number}/next";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1next/get"),

        /// Return the list of epochs preceding a specific epoch.
        epochs_previous(number: Integer) -> Vec<Epoch> => "/epochs/{number}/previous";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1previous/get"),

        /// Return the active stake distribution for the specified epoch.
        epochs_stakes(number: Integer) -> Vec<AddressStakePool> => "/epochs/{number}/stakes";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1stakes/get"),

        /// Return the active stake distribution for the epoch specified by stake pool.
        epochs_stakes_by_pool(number: Integer, pool_id: &str) -> Vec<AddressStake> => "/epochs/{number}/stakes/{pool_id}";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1stakes~1{pool_id}/get"),

        /// Return the blocks minted for the epoch specified.
        epochs_blocks(number: Integer) -> Vec<String> => "/epochs/{number}/blocks";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1blocks/get"),

        /// Return the block minted for the epoch specified by stake pool.
        epochs_blocks_by_pool(number: Integer, pool_id: &str) -> Vec<String> => "/epochs/{number}/blocks/{pool_id}";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1blocks~1{pool_id}/get"),
    }
}

/// Created by [`epochs_latest`](BlockFrostApi::epochs_latest) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Epoch {
    /// Epoch number.
    pub epoch: Integer,
    /// Unix time of the start of the epoch.
    pub start_time: Integer,
    /// Unix time of the end of the epoch.
    pub end_time: Integer,
    /// Unix time of the first block of the epoch.
    pub first_block_time: Integer,
    /// Unix time of the last block of the epoch.
    pub last_block_time: Integer,
    /// Number of blocks within the epoch.
    pub block_count: Integer,
    /// Number of transactions within the epoch.
    pub tx_count: Integer,
    /// Sum of all the transactions within the epoch in Lovelaces.
    pub output: String,
    /// Sum of all the fees within the epoch in Lovelaces.
    pub fees: String,
    /// Sum of all the active stakes within the epoch in Lovelaces.
    pub active_stake: Option<String>,
}

/// Created by [`epochs_latest_parameters`](BlockFrostApi::epochs_latest_parameters) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochParameters {
    pub epoch: i64,
    #[serde(rename = "min_fee_a")]
    pub min_fee_a: i64,
    #[serde(rename = "min_fee_b")]
    pub min_fee_b: i64,
    #[serde(rename = "max_block_size")]
    pub max_block_size: i64,
    #[serde(rename = "max_tx_size")]
    pub max_tx_size: i64,
    #[serde(rename = "max_block_header_size")]
    pub max_block_header_size: i64,
    #[serde(rename = "key_deposit")]
    pub key_deposit: String,
    #[serde(rename = "pool_deposit")]
    pub pool_deposit: String,
    #[serde(rename = "e_max")]
    pub e_max: i64,
    #[serde(rename = "n_opt")]
    pub n_opt: i64,
    pub a0: f64,
    pub rho: f64,
    pub tau: f64,
    #[serde(rename = "decentralisation_param")]
    pub decentralisation_param: f64,
    #[serde(rename = "extra_entropy")]
    pub extra_entropy: serde_json::Value,
    #[serde(rename = "protocol_major_ver")]
    pub protocol_major_ver: i64,
    #[serde(rename = "protocol_minor_ver")]
    pub protocol_minor_ver: i64,
    #[serde(rename = "min_utxo")]
    pub min_utxo: String,
    #[serde(rename = "min_pool_cost")]
    pub min_pool_cost: String,
    pub nonce: String,
    #[serde(rename = "cost_models")]
    pub cost_models: CostModels,
    #[serde(rename = "price_mem")]
    pub price_mem: f64,
    #[serde(rename = "price_step")]
    pub price_step: f64,
    #[serde(rename = "max_tx_ex_mem")]
    pub max_tx_ex_mem: String,
    #[serde(rename = "max_tx_ex_steps")]
    pub max_tx_ex_steps: String,
    #[serde(rename = "max_block_ex_mem")]
    pub max_block_ex_mem: String,
    #[serde(rename = "max_block_ex_steps")]
    pub max_block_ex_steps: String,
    #[serde(rename = "max_val_size")]
    pub max_val_size: String,
    #[serde(rename = "collateral_percent")]
    pub collateral_percent: i64,
    #[serde(rename = "max_collateral_inputs")]
    pub max_collateral_inputs: i64,
    #[serde(rename = "coins_per_utxo_size")]
    pub coins_per_utxo_size: String,
    #[serde(rename = "coins_per_utxo_word")]
    pub coins_per_utxo_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostModels {
    #[serde(rename = "PlutusV1")]
    pub plutus_v1: PlutusV1,
    #[serde(rename = "PlutusV2")]
    pub plutus_v2: PlutusV2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlutusV2 {
    #[serde(rename = "addInteger-cpu-arguments-intercept")]
    pub add_integer_cpu_arguments_intercept: i64,
    #[serde(rename = "addInteger-cpu-arguments-slope")]
    pub add_integer_cpu_arguments_slope: i64,
    #[serde(rename = "addInteger-memory-arguments-intercept")]
    pub add_integer_memory_arguments_intercept: i64,
    #[serde(rename = "addInteger-memory-arguments-slope")]
    pub add_integer_memory_arguments_slope: i64,
    #[serde(rename = "appendByteString-cpu-arguments-intercept")]
    pub append_byte_string_cpu_arguments_intercept: i64,
    #[serde(rename = "appendByteString-cpu-arguments-slope")]
    pub append_byte_string_cpu_arguments_slope: i64,
    #[serde(rename = "appendByteString-memory-arguments-intercept")]
    pub append_byte_string_memory_arguments_intercept: i64,
    #[serde(rename = "appendByteString-memory-arguments-slope")]
    pub append_byte_string_memory_arguments_slope: i64,
    #[serde(rename = "appendString-cpu-arguments-intercept")]
    pub append_string_cpu_arguments_intercept: i64,
    #[serde(rename = "appendString-cpu-arguments-slope")]
    pub append_string_cpu_arguments_slope: i64,
    #[serde(rename = "appendString-memory-arguments-intercept")]
    pub append_string_memory_arguments_intercept: i64,
    #[serde(rename = "appendString-memory-arguments-slope")]
    pub append_string_memory_arguments_slope: i64,
    #[serde(rename = "bData-cpu-arguments")]
    pub b_data_cpu_arguments: i64,
    #[serde(rename = "bData-memory-arguments")]
    pub b_data_memory_arguments: i64,
    #[serde(rename = "blake2b_256-cpu-arguments-intercept")]
    pub blake2b_256_cpu_arguments_intercept: i64,
    #[serde(rename = "blake2b_256-cpu-arguments-slope")]
    pub blake2b_256_cpu_arguments_slope: i64,
    #[serde(rename = "blake2b_256-memory-arguments")]
    pub blake2b_256_memory_arguments: i64,
    #[serde(rename = "cekApplyCost-exBudgetCPU")]
    pub cek_apply_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekApplyCost-exBudgetMemory")]
    pub cek_apply_cost_ex_budget_memory: i64,
    #[serde(rename = "cekBuiltinCost-exBudgetCPU")]
    pub cek_builtin_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekBuiltinCost-exBudgetMemory")]
    pub cek_builtin_cost_ex_budget_memory: i64,
    #[serde(rename = "cekConstCost-exBudgetCPU")]
    pub cek_const_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekConstCost-exBudgetMemory")]
    pub cek_const_cost_ex_budget_memory: i64,
    #[serde(rename = "cekDelayCost-exBudgetCPU")]
    pub cek_delay_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekDelayCost-exBudgetMemory")]
    pub cek_delay_cost_ex_budget_memory: i64,
    #[serde(rename = "cekForceCost-exBudgetCPU")]
    pub cek_force_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekForceCost-exBudgetMemory")]
    pub cek_force_cost_ex_budget_memory: i64,
    #[serde(rename = "cekLamCost-exBudgetCPU")]
    pub cek_lam_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekLamCost-exBudgetMemory")]
    pub cek_lam_cost_ex_budget_memory: i64,
    #[serde(rename = "cekStartupCost-exBudgetCPU")]
    pub cek_startup_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekStartupCost-exBudgetMemory")]
    pub cek_startup_cost_ex_budget_memory: i64,
    #[serde(rename = "cekVarCost-exBudgetCPU")]
    pub cek_var_cost_ex_budget_cpu: i64,
    #[serde(rename = "cekVarCost-exBudgetMemory")]
    pub cek_var_cost_ex_budget_memory: i64,
    #[serde(rename = "chooseData-cpu-arguments")]
    pub choose_data_cpu_arguments: i64,
    #[serde(rename = "chooseData-memory-arguments")]
    pub choose_data_memory_arguments: i64,
    #[serde(rename = "chooseList-cpu-arguments")]
    pub choose_list_cpu_arguments: i64,
    #[serde(rename = "chooseList-memory-arguments")]
    pub choose_list_memory_arguments: i64,
    #[serde(rename = "chooseUnit-cpu-arguments")]
    pub choose_unit_cpu_arguments: i64,
    #[serde(rename = "chooseUnit-memory-arguments")]
    pub choose_unit_memory_arguments: i64,
    #[serde(rename = "consByteString-cpu-arguments-intercept")]
    pub cons_byte_string_cpu_arguments_intercept: i64,
    #[serde(rename = "consByteString-cpu-arguments-slope")]
    pub cons_byte_string_cpu_arguments_slope: i64,
    #[serde(rename = "consByteString-memory-arguments-intercept")]
    pub cons_byte_string_memory_arguments_intercept: i64,
    #[serde(rename = "consByteString-memory-arguments-slope")]
    pub cons_byte_string_memory_arguments_slope: i64,
    #[serde(rename = "constrData-cpu-arguments")]
    pub constr_data_cpu_arguments: i64,
    #[serde(rename = "constrData-memory-arguments")]
    pub constr_data_memory_arguments: i64,
    #[serde(rename = "decodeUtf8-cpu-arguments-intercept")]
    pub decode_utf8_cpu_arguments_intercept: i64,
    #[serde(rename = "decodeUtf8-cpu-arguments-slope")]
    pub decode_utf8_cpu_arguments_slope: i64,
    #[serde(rename = "decodeUtf8-memory-arguments-intercept")]
    pub decode_utf8_memory_arguments_intercept: i64,
    #[serde(rename = "decodeUtf8-memory-arguments-slope")]
    pub decode_utf8_memory_arguments_slope: i64,
    #[serde(rename = "divideInteger-cpu-arguments-constant")]
    pub divide_integer_cpu_arguments_constant: i64,
    #[serde(rename = "divideInteger-cpu-arguments-model-arguments-intercept")]
    pub divide_integer_cpu_arguments_model_arguments_intercept: i64,
    #[serde(rename = "divideInteger-cpu-arguments-model-arguments-slope")]
    pub divide_integer_cpu_arguments_model_arguments_slope: i64,
    #[serde(rename = "divideInteger-memory-arguments-intercept")]
    pub divide_integer_memory_arguments_intercept: i64,
    #[serde(rename = "divideInteger-memory-arguments-minimum")]
    pub divide_integer_memory_arguments_minimum: i64,
    #[serde(rename = "divideInteger-memory-arguments-slope")]
    pub divide_integer_memory_arguments_slope: i64,
    #[serde(rename = "encodeUtf8-cpu-arguments-intercept")]
    pub encode_utf8_cpu_arguments_intercept: i64,
    #[serde(rename = "encodeUtf8-cpu-arguments-slope")]
    pub encode_utf8_cpu_arguments_slope: i64,
    #[serde(rename = "encodeUtf8-memory-arguments-intercept")]
    pub encode_utf8_memory_arguments_intercept: i64,
    #[serde(rename = "encodeUtf8-memory-arguments-slope")]
    pub encode_utf8_memory_arguments_slope: i64,
    #[serde(rename = "equalsByteString-cpu-arguments-constant")]
    pub equals_byte_string_cpu_arguments_constant: i64,
    #[serde(rename = "equalsByteString-cpu-arguments-intercept")]
    pub equals_byte_string_cpu_arguments_intercept: i64,
    #[serde(rename = "equalsByteString-cpu-arguments-slope")]
    pub equals_byte_string_cpu_arguments_slope: i64,
    #[serde(rename = "equalsByteString-memory-arguments")]
    pub equals_byte_string_memory_arguments: i64,
    #[serde(rename = "equalsData-cpu-arguments-intercept")]
    pub equals_data_cpu_arguments_intercept: i64,
    #[serde(rename = "equalsData-cpu-arguments-slope")]
    pub equals_data_cpu_arguments_slope: i64,
    #[serde(rename = "equalsData-memory-arguments")]
    pub equals_data_memory_arguments: i64,
    #[serde(rename = "equalsInteger-cpu-arguments-intercept")]
    pub equals_integer_cpu_arguments_intercept: i64,
    #[serde(rename = "equalsInteger-cpu-arguments-slope")]
    pub equals_integer_cpu_arguments_slope: i64,
    #[serde(rename = "equalsInteger-memory-arguments")]
    pub equals_integer_memory_arguments: i64,
    #[serde(rename = "equalsString-cpu-arguments-constant")]
    pub equals_string_cpu_arguments_constant: i64,
    #[serde(rename = "equalsString-cpu-arguments-intercept")]
    pub equals_string_cpu_arguments_intercept: i64,
    #[serde(rename = "equalsString-cpu-arguments-slope")]
    pub equals_string_cpu_arguments_slope: i64,
    #[serde(rename = "equalsString-memory-arguments")]
    pub equals_string_memory_arguments: i64,
    #[serde(rename = "fstPair-cpu-arguments")]
    pub fst_pair_cpu_arguments: i64,
    #[serde(rename = "fstPair-memory-arguments")]
    pub fst_pair_memory_arguments: i64,
    #[serde(rename = "headList-cpu-arguments")]
    pub head_list_cpu_arguments: i64,
    #[serde(rename = "headList-memory-arguments")]
    pub head_list_memory_arguments: i64,
    #[serde(rename = "iData-cpu-arguments")]
    pub i_data_cpu_arguments: i64,
    #[serde(rename = "iData-memory-arguments")]
    pub i_data_memory_arguments: i64,
    #[serde(rename = "ifThenElse-cpu-arguments")]
    pub if_then_else_cpu_arguments: i64,
    #[serde(rename = "ifThenElse-memory-arguments")]
    pub if_then_else_memory_arguments: i64,
    #[serde(rename = "indexByteString-cpu-arguments")]
    pub index_byte_string_cpu_arguments: i64,
    #[serde(rename = "indexByteString-memory-arguments")]
    pub index_byte_string_memory_arguments: i64,
    #[serde(rename = "lengthOfByteString-cpu-arguments")]
    pub length_of_byte_string_cpu_arguments: i64,
    #[serde(rename = "lengthOfByteString-memory-arguments")]
    pub length_of_byte_string_memory_arguments: i64,
    #[serde(rename = "lessThanByteString-cpu-arguments-intercept")]
    pub less_than_byte_string_cpu_arguments_intercept: i64,
    #[serde(rename = "lessThanByteString-cpu-arguments-slope")]
    pub less_than_byte_string_cpu_arguments_slope: i64,
    #[serde(rename = "lessThanByteString-memory-arguments")]
    pub less_than_byte_string_memory_arguments: i64,
    #[serde(rename = "lessThanEqualsByteString-cpu-arguments-intercept")]
    pub less_than_equals_byte_string_cpu_arguments_intercept: i64,
    #[serde(rename = "lessThanEqualsByteString-cpu-arguments-slope")]
    pub less_than_equals_byte_string_cpu_arguments_slope: i64,
    #[serde(rename = "lessThanEqualsByteString-memory-arguments")]
    pub less_than_equals_byte_string_memory_arguments: i64,
    #[serde(rename = "lessThanEqualsInteger-cpu-arguments-intercept")]
    pub less_than_equals_integer_cpu_arguments_intercept: i64,
    #[serde(rename = "lessThanEqualsInteger-cpu-arguments-slope")]
    pub less_than_equals_integer_cpu_arguments_slope: i64,
    #[serde(rename = "lessThanEqualsInteger-memory-arguments")]
    pub less_than_equals_integer_memory_arguments: i64,
    #[serde(rename = "lessThanInteger-cpu-arguments-intercept")]
    pub less_than_integer_cpu_arguments_intercept: i64,
    #[serde(rename = "lessThanInteger-cpu-arguments-slope")]
    pub less_than_integer_cpu_arguments_slope: i64,
    #[serde(rename = "lessThanInteger-memory-arguments")]
    pub less_than_integer_memory_arguments: i64,
    #[serde(rename = "listData-cpu-arguments")]
    pub list_data_cpu_arguments: i64,
    #[serde(rename = "listData-memory-arguments")]
    pub list_data_memory_arguments: i64,
    #[serde(rename = "mapData-cpu-arguments")]
    pub map_data_cpu_arguments: i64,
    #[serde(rename = "mapData-memory-arguments")]
    pub map_data_memory_arguments: i64,
    #[serde(rename = "mkCons-cpu-arguments")]
    pub mk_cons_cpu_arguments: i64,
    #[serde(rename = "mkCons-memory-arguments")]
    pub mk_cons_memory_arguments: i64,
    #[serde(rename = "mkNilData-cpu-arguments")]
    pub mk_nil_data_cpu_arguments: i64,
    #[serde(rename = "mkNilData-memory-arguments")]
    pub mk_nil_data_memory_arguments: i64,
    #[serde(rename = "mkNilPairData-cpu-arguments")]
    pub mk_nil_pair_data_cpu_arguments: i64,
    #[serde(rename = "mkNilPairData-memory-arguments")]
    pub mk_nil_pair_data_memory_arguments: i64,
    #[serde(rename = "mkPairData-cpu-arguments")]
    pub mk_pair_data_cpu_arguments: i64,
    #[serde(rename = "mkPairData-memory-arguments")]
    pub mk_pair_data_memory_arguments: i64,
    #[serde(rename = "modInteger-cpu-arguments-constant")]
    pub mod_integer_cpu_arguments_constant: i64,
    #[serde(rename = "modInteger-cpu-arguments-model-arguments-intercept")]
    pub mod_integer_cpu_arguments_model_arguments_intercept: i64,
    #[serde(rename = "modInteger-cpu-arguments-model-arguments-slope")]
    pub mod_integer_cpu_arguments_model_arguments_slope: i64,
    #[serde(rename = "modInteger-memory-arguments-intercept")]
    pub mod_integer_memory_arguments_intercept: i64,
    #[serde(rename = "modInteger-memory-arguments-minimum")]
    pub mod_integer_memory_arguments_minimum: i64,
    #[serde(rename = "modInteger-memory-arguments-slope")]
    pub mod_integer_memory_arguments_slope: i64,
    #[serde(rename = "multiplyInteger-cpu-arguments-intercept")]
    pub multiply_integer_cpu_arguments_intercept: i64,
    #[serde(rename = "multiplyInteger-cpu-arguments-slope")]
    pub multiply_integer_cpu_arguments_slope: i64,
    #[serde(rename = "multiplyInteger-memory-arguments-intercept")]
    pub multiply_integer_memory_arguments_intercept: i64,
    #[serde(rename = "multiplyInteger-memory-arguments-slope")]
    pub multiply_integer_memory_arguments_slope: i64,
    #[serde(rename = "nullList-cpu-arguments")]
    pub null_list_cpu_arguments: i64,
    #[serde(rename = "nullList-memory-arguments")]
    pub null_list_memory_arguments: i64,
    #[serde(rename = "quotientInteger-cpu-arguments-constant")]
    pub quotient_integer_cpu_arguments_constant: i64,
    #[serde(rename = "quotientInteger-cpu-arguments-model-arguments-intercept")]
    pub quotient_integer_cpu_arguments_model_arguments_intercept: i64,
    #[serde(rename = "quotientInteger-cpu-arguments-model-arguments-slope")]
    pub quotient_integer_cpu_arguments_model_arguments_slope: i64,
    #[serde(rename = "quotientInteger-memory-arguments-intercept")]
    pub quotient_integer_memory_arguments_intercept: i64,
    #[serde(rename = "quotientInteger-memory-arguments-minimum")]
    pub quotient_integer_memory_arguments_minimum: i64,
    #[serde(rename = "quotientInteger-memory-arguments-slope")]
    pub quotient_integer_memory_arguments_slope: i64,
    #[serde(rename = "remainderInteger-cpu-arguments-constant")]
    pub remainder_integer_cpu_arguments_constant: i64,
    #[serde(rename = "remainderInteger-cpu-arguments-model-arguments-intercept")]
    pub remainder_integer_cpu_arguments_model_arguments_intercept: i64,
    #[serde(rename = "remainderInteger-cpu-arguments-model-arguments-slope")]
    pub remainder_integer_cpu_arguments_model_arguments_slope: i64,
    #[serde(rename = "remainderInteger-memory-arguments-intercept")]
    pub remainder_integer_memory_arguments_intercept: i64,
    #[serde(rename = "remainderInteger-memory-arguments-minimum")]
    pub remainder_integer_memory_arguments_minimum: i64,
    #[serde(rename = "remainderInteger-memory-arguments-slope")]
    pub remainder_integer_memory_arguments_slope: i64,
    #[serde(rename = "serialiseData-cpu-arguments-intercept")]
    pub serialise_data_cpu_arguments_intercept: i64,
    #[serde(rename = "serialiseData-cpu-arguments-slope")]
    pub serialise_data_cpu_arguments_slope: i64,
    #[serde(rename = "serialiseData-memory-arguments-intercept")]
    pub serialise_data_memory_arguments_intercept: i64,
    #[serde(rename = "serialiseData-memory-arguments-slope")]
    pub serialise_data_memory_arguments_slope: i64,
    #[serde(rename = "sha2_256-cpu-arguments-intercept")]
    pub sha2_256_cpu_arguments_intercept: i64,
    #[serde(rename = "sha2_256-cpu-arguments-slope")]
    pub sha2_256_cpu_arguments_slope: i64,
    #[serde(rename = "sha2_256-memory-arguments")]
    pub sha2_256_memory_arguments: i64,
    #[serde(rename = "sha3_256-cpu-arguments-intercept")]
    pub sha3_256_cpu_arguments_intercept: i64,
    #[serde(rename = "sha3_256-cpu-arguments-slope")]
    pub sha3_256_cpu_arguments_slope: i64,
    #[serde(rename = "sha3_256-memory-arguments")]
    pub sha3_256_memory_arguments: i64,
    #[serde(rename = "sliceByteString-cpu-arguments-intercept")]
    pub slice_byte_string_cpu_arguments_intercept: i64,
    #[serde(rename = "sliceByteString-cpu-arguments-slope")]
    pub slice_byte_string_cpu_arguments_slope: i64,
    #[serde(rename = "sliceByteString-memory-arguments-intercept")]
    pub slice_byte_string_memory_arguments_intercept: i64,
    #[serde(rename = "sliceByteString-memory-arguments-slope")]
    pub slice_byte_string_memory_arguments_slope: i64,
    #[serde(rename = "sndPair-cpu-arguments")]
    pub snd_pair_cpu_arguments: i64,
    #[serde(rename = "sndPair-memory-arguments")]
    pub snd_pair_memory_arguments: i64,
    #[serde(rename = "subtractInteger-cpu-arguments-intercept")]
    pub subtract_integer_cpu_arguments_intercept: i64,
    #[serde(rename = "subtractInteger-cpu-arguments-slope")]
    pub subtract_integer_cpu_arguments_slope: i64,
    #[serde(rename = "subtractInteger-memory-arguments-intercept")]
    pub subtract_integer_memory_arguments_intercept: i64,
    #[serde(rename = "subtractInteger-memory-arguments-slope")]
    pub subtract_integer_memory_arguments_slope: i64,
    #[serde(rename = "tailList-cpu-arguments")]
    pub tail_list_cpu_arguments: i64,
    #[serde(rename = "tailList-memory-arguments")]
    pub tail_list_memory_arguments: i64,
    #[serde(rename = "trace-cpu-arguments")]
    pub trace_cpu_arguments: i64,
    #[serde(rename = "trace-memory-arguments")]
    pub trace_memory_arguments: i64,
    #[serde(rename = "unBData-cpu-arguments")]
    pub un_bdata_cpu_arguments: i64,
    #[serde(rename = "unBData-memory-arguments")]
    pub un_bdata_memory_arguments: i64,
    #[serde(rename = "unConstrData-cpu-arguments")]
    pub un_constr_data_cpu_arguments: i64,
    #[serde(rename = "unConstrData-memory-arguments")]
    pub un_constr_data_memory_arguments: i64,
    #[serde(rename = "unIData-cpu-arguments")]
    pub un_idata_cpu_arguments: i64,
    #[serde(rename = "unIData-memory-arguments")]
    pub un_idata_memory_arguments: i64,
    #[serde(rename = "unListData-cpu-arguments")]
    pub un_list_data_cpu_arguments: i64,
    #[serde(rename = "unListData-memory-arguments")]
    pub un_list_data_memory_arguments: i64,
    #[serde(rename = "unMapData-cpu-arguments")]
    pub un_map_data_cpu_arguments: i64,
    #[serde(rename = "unMapData-memory-arguments")]
    pub un_map_data_memory_arguments: i64,
    #[serde(rename = "verifyEcdsaSecp256k1Signature-cpu-arguments")]
    pub verify_ecdsa_secp256k1_signature_cpu_arguments: i64,
    #[serde(rename = "verifyEcdsaSecp256k1Signature-memory-arguments")]
    pub verify_ecdsa_secp256k1_signature_memory_arguments: i64,
    #[serde(rename = "verifyEd25519Signature-cpu-arguments-intercept")]
    pub verify_ed25519_signature_cpu_arguments_intercept: i64,
    #[serde(rename = "verifyEd25519Signature-cpu-arguments-slope")]
    pub verify_ed25519_signature_cpu_arguments_slope: i64,
    #[serde(rename = "verifyEd25519Signature-memory-arguments")]
    pub verify_ed25519_signature_memory_arguments: i64,
    #[serde(rename = "verifySchnorrSecp256k1Signature-cpu-arguments-intercept")]
    pub verify_schnorr_secp256k1_signature_cpu_arguments_intercept: i64,
    #[serde(rename = "verifySchnorrSecp256k1Signature-cpu-arguments-slope")]
    pub verify_schnorr_secp256k1_signature_cpu_arguments_slope: i64,
    #[serde(rename = "verifySchnorrSecp256k1Signature-memory-arguments")]
    pub verify_schnorr_secp256k1_signature_memory_arguments: i64
    
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlutusV1 {
   pub who_cares : Option<String>
}

/// Created by [`epochs_stakes`](BlockFrostApi::epochs_stakes) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressStakePool {
    /// Stake address.
    pub stake_address: String,
    /// Bech32 prefix of the pool delegated to.
    pub pool_id: String,
    /// Amount of active delegated stake in Lovelaces.
    pub amount: String,
}

/// Created by [`epochs_stakes_by_pool`](BlockFrostApi::epochs_stakes_by_pool) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressStake {
    /// Stake address.
    pub stake_address: String,
    /// Amount of active delegated stake in Lovelaces.
    pub amount: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_epochs_latest, Epoch, r#"
    {
        "epoch": 225,
        "start_time": 1603403091,
        "end_time": 1603835086,
        "first_block_time": 1603403092,
        "last_block_time": 1603835084,
        "block_count": 21298,
        "tx_count": 17856,
        "output": "7849943934049314",
        "fees": "4203312194",
        "active_stake": "784953934049314"
    }
    "# }

    test_example! { test_epochs_latest_parameters, EpochParameters, r#"
    {
        "epoch": 225,
        "min_fee_a": 44,
        "min_fee_b": 155381,
        "max_block_size": 65536,
        "max_tx_size": 16384,
        "max_block_header_size": 1100,
        "key_deposit": "2000000",
        "pool_deposit": "500000000",
        "e_max": 18,
        "n_opt": 150,
        "a0": 0.3,
        "rho": 0.003,
        "tau": 0.2,
        "decentralisation_param": 0.5,
        "extra_entropy": null,
        "protocol_major_ver": 2,
        "protocol_minor_ver": 0,
        "min_utxo": "1000000",
        "min_pool_cost": "340000000",
        "nonce": "1a3be38bcbb7911969283716ad7aa550250226b76a61fc51cc9a9a35d9276d81",
        "price_mem": 0.001,
        "price_step": 0.01,
        "max_tx_ex_mem": "11000000000",
        "max_tx_ex_steps": "11000000000",
        "max_block_ex_mem": "110000000000",
        "max_block_ex_steps": "110000000000",
        "max_val_size": "5000",
        "collateral_percent": 1.5,
        "max_collateral_inputs": 6,
        "coins_per_utxo_word": "34482"
    }
    "# }

    test_example! { test_epochs_next, Vec<Epoch>, r#"
    [
        {
            "epoch": 225,
            "start_time": 1603403091,
            "end_time": 1603835086,
            "first_block_time": 1603403092,
            "last_block_time": 1603835084,
            "block_count": 21298,
            "tx_count": 17856,
            "output": "7849943934049314",
            "fees": "4203312194",
            "active_stake": "784953934049314"
        }
    ]"#}

    test_example! { test_epochs_stakes, Vec<AddressStakePool>, r#"
    [
        {
            "stake_address": "stake1u9l5q5jwgelgagzyt6nuaasefgmn8pd25c8e9qpeprq0tdcp0e3uk",
            "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
            "amount": "4440295078"
        }
    ]
    "#}

    test_example! { test_epochs_stakes_by_pool, Vec<AddressStake>, r#"
    [
        {
            "stake_address": "stake1u9l5q5jwgelgagzyt6nuaasefgmn8pd25c8e9qpeprq0tdcp0e3uk",
            "amount": "4440295078"
        }
    ]
    "#}

    test_example! { test_epochs_blocks, Vec<String>, r#"
    [
        "d0fa315687e99ccdc96b14cc2ea74a767405d64427b648c470731a9b69e4606e",
        "38bc6efb92a830a0ed22a64f979d120d26483fd3c811f6622a8c62175f530878",
        "f3258fcd8b975c061b4fcdcfcbb438807134d6961ec278c200151274893b6b7d"
    ]
    "#}
}
