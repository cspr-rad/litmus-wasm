use casper_litmus::{block::Block, json_compatibility::JsonBlock, kernel::EraInfo};
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
