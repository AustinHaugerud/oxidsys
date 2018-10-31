use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GameKeyGetMappedKeyNameOp;

const DOC : &str = "Version 1.161+. Stores human-readable key name that's currently assigned to the provided game key. May store \"unknown\" and \"No key assigned\" strings (the latter is defined in languages/en/ui.csv, the former seems to be hardcoded).";

pub const OP_CODE: u32 = 65;

pub const IDENT: &str = "game_key_get_mapped_key_name";

impl Operation for GameKeyGetMappedKeyNameOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<string_register>", ""),
                make_param_doc("<game_key>", ""),
            ],
        }
    }
}
