use casper_litmus::{
    block::Block, block_header::BlockHash, json_compatibility::JsonBlock, kernel::EraInfo,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BlockValidator {
    era_info: EraInfo,
}

#[wasm_bindgen]
impl BlockValidator {
    #[wasm_bindgen(constructor)]
    pub fn new(json_block_str: &str) -> Result<BlockValidator, String> {
        let json_block: JsonBlock =
            serde_json::from_str(json_block_str).map_err(|err| format!("{err:?}"))?;
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
    pub fn validate_json_block(&self, json_block_str: &str) -> Result<(), String> {
        let json_block: JsonBlock =
            serde_json::from_str(json_block_str).map_err(|err| format!("{err:?}"))?;
        let block = Block::try_from(json_block).map_err(|err| format!("{err:?}"))?;
        self.era_info
            .validate(block.block_header_with_signatures())
            .map_err(|err| format!("{err:?}"))
    }
}

#[wasm_bindgen]
pub fn validate_block_hash(expected_hash_str: &str, json_block_str: &str) -> Result<(), String> {
    let expected_block_hash: BlockHash =
        serde_json::from_str(expected_hash_str).map_err(|err| format!("{err:?}"))?;
    let json_block: JsonBlock =
        serde_json::from_str(json_block_str).map_err(|err| format!("{err:?}"))?;
    let block = Block::try_from(json_block).map_err(|err| format!("{err:?}"))?;
    let actual_block_hash = block
        .block_header_with_signatures()
        .block_header()
        .block_hash();
    if expected_block_hash != actual_block_hash {
        let expected_block_hash_hex = expected_block_hash.to_hex();
        let actual_block_hash_hex = actual_block_hash.to_hex();
        return Err(format!(
            "Block hashes do no match. Expected block hash: {expected_block_hash_hex} Actual block hash: {actual_block_hash_hex}"
        ));
    }
    Ok(())
}
