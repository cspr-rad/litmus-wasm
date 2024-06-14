extern crate alloc;

use alloc::collections::BTreeMap;

use casper_litmus::{
    block::Block, json_compatibility::JsonBlock, kernel::EraInfo, merkle_proof::TrieMerkleProof,
};
use casper_types::{PublicKey, U512};
use serde_json::json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BlockValidator {
    era_info: EraInfo,
}

#[wasm_bindgen]
impl BlockValidator {
    #[wasm_bindgen(constructor)]
    pub fn new(
        era_number: u64,
        validator_weights_js_value: JsValue,
    ) -> Result<BlockValidator, String> {
        let validator_weights: BTreeMap<PublicKey, U512> =
            serde_wasm_bindgen::from_value(validator_weights_js_value)
                .map_err(|err| format!("{err:?}"))?;
        let era_info = EraInfo::new(era_number.into(), validator_weights);
        Ok(BlockValidator { era_info })
    }

    #[wasm_bindgen]
    pub fn validate(&self, json_block_js_value: JsValue) -> Result<(), String> {
        let json_block: JsonBlock = serde_wasm_bindgen::from_value(json_block_js_value)
            .map_err(|err| format!("{err:?}"))?;
        let block = Block::try_from(json_block).map_err(|err| format!("{err:?}"))?;
        self.era_info
            .validate(block.block_header_with_signatures())
            .map_err(|err| format!("{err:?}"))
    }
}

#[wasm_bindgen]
pub fn block_hash(json_block_js_value: JsValue) -> Result<String, String> {
    let json_block: JsonBlock =
        serde_wasm_bindgen::from_value(json_block_js_value).map_err(|err| format!("{err:?}"))?;
    let block = Block::try_from(json_block).map_err(|err| format!("{err:?}"))?;
    let block_hash = block
        .block_header_with_signatures()
        .block_header()
        .block_hash();
    Ok(block_hash.to_hex())
}

#[wasm_bindgen]
pub fn process_query_proofs(
    merkle_proofs_hex_str: &str,
    path: Vec<String>,
) -> Result<JsValue, String> {
    let merkle_proof_bytes =
        base16::decode(merkle_proofs_hex_str).map_err(|err| format!("{err:?}"))?;
    let merkle_proofs: Vec<TrieMerkleProof> =
        casper_types::bytesrepr::deserialize(merkle_proof_bytes)
            .map_err(|err| format!("{err:?}"))?;
    let query_info = casper_litmus::merkle_proof::process_query_proofs(&merkle_proofs, &path)
        .map_err(|err| format!("{err:?}"))?;
    let output = json!({
        "state_root": query_info.state_root(),
        "key": query_info.key(),
        "value": query_info.stored_value()
    });
    serde_wasm_bindgen::to_value(&output).map_err(|err| format!("{err:?}"))
}
