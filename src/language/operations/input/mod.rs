use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod clear_omitted_keys;
pub mod game_key_clicked;
pub mod game_key_is_down;
pub mod key_clicked;
pub mod key_is_down;
pub mod mouse_get_position;
pub mod omit_key_once;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(clear_omitted_keys::ClearOmittedKeysOp {}));
    result.push(Box::new(game_key_clicked::GameKeyClickedOp {}));
    result.push(Box::new(game_key_is_down::GameKeyIsDownOp {}));
    result.push(Box::new(key_clicked::KeyClickedOp {}));
    result.push(Box::new(key_is_down::KeyIsDownOp {}));
    result.push(Box::new(mouse_get_position::MouseGetPositionOp {}));
    result.push(Box::new(omit_key_once::OmitKeyOnceOp {}));
    result
}
