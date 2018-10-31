use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ContextMenuAddItemOp;

const DOC : &str = "Must be called inside script_game_context_menu_get_buttons. Adds context menu option for a party and it's respective identifier (will be passed to script_game_event_context_menu_button_clicked).";

pub const OP_CODE: u32 = 980;

pub const IDENT: &str = "context_menu_add_item";

impl Operation for ContextMenuAddItemOp {
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
                make_param_doc("<string_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
