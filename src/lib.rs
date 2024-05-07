use casper_litmus::{
    block::Block, json_compatibility::JsonBlock, kernel::EraInfo, merkle_proof::TrieMerkleProof,
};
use casper_types::{Key, StoredValue};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BlockValidator {
    era_info: EraInfo,
}

#[wasm_bindgen]
impl BlockValidator {
    #[wasm_bindgen(constructor)]
    pub fn new(json_block_js_value: JsValue) -> Result<BlockValidator, String> {
        let json_block: JsonBlock = serde_wasm_bindgen::from_value(json_block_js_value)
            .map_err(|err| format!("{err:?}"))?;
        let block = Block::try_from(json_block).map_err(|err| format!("{err:?}"))?;
        let block_header = block.block_header_with_signatures().block_header();
        let era_end = block_header
            .era_end()
            .ok_or("block is not a switch block")?;
        let era_info = EraInfo::new(
            block_header.era_id().successor(),
            era_end.next_era_validator_weights().to_owned(),
        );
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

#[derive(serde::Serialize)]
struct MerkleProofInfo<'a, 'b> {
    state_root: String,
    key: &'a Key,
    value: &'b StoredValue,
}

#[wasm_bindgen]
pub fn process_merkle_proof(merkle_proof_hex_str: &str) -> Result<JsValue, String> {
    let merkle_proof_bytes =
        base16::decode(merkle_proof_hex_str).map_err(|err| format!("{err:?}"))?;
    let merkle_proof: TrieMerkleProof = casper_types::bytesrepr::deserialize(merkle_proof_bytes)
        .map_err(|err| format!("{err:?}"))?;
    let state_root = merkle_proof
        .compute_state_hash()
        .map_err(|err| format!("{err:?}"))?;
    let merkle_proof_info = MerkleProofInfo {
        state_root: state_root.to_hex(),
        key: merkle_proof.key(),
        value: merkle_proof.value(),
    };
    serde_wasm_bindgen::to_value(&merkle_proof_info).map_err(|err| format!("{err:?}"))
}
