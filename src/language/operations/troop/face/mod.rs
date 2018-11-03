use language::operations::{Operation};
pub mod face_keys_get_age;
pub mod face_keys_get_beard;
pub mod face_keys_get_face_texture;
pub mod face_keys_get_hair;
pub mod face_keys_get_hair_color;
pub mod face_keys_get_hair_texture;
pub mod face_keys_get_morph_key;
pub mod face_keys_get_skin_color;
pub mod face_keys_set_age;
pub mod face_keys_set_beard;
pub mod face_keys_set_face_texture;
pub mod face_keys_set_hair;
pub mod face_keys_set_hair_color;
pub mod face_keys_set_hair_texture;
pub mod face_keys_set_morph_key;
pub mod face_keys_set_skin_color;
pub mod player_set_face_keys;
pub mod str_store_player_face_keys;
pub mod str_store_troop_face_keys;
pub mod troop_set_face_keys;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(face_keys_get_age::FaceKeysGetAgeOp {}));
    result.push(Box::new(face_keys_get_beard::FaceKeysGetBeardOp {}));
    result.push(Box::new(
        face_keys_get_face_texture::FaceKeysGetFaceTextureOp {},
    ));
    result.push(Box::new(face_keys_get_hair::FaceKeysGetHairOp {}));
    result.push(Box::new(
        face_keys_get_hair_color::FaceKeysGetHairColorOp {},
    ));
    result.push(Box::new(
        face_keys_get_hair_texture::FaceKeysGetHairTextureOp {},
    ));
    result.push(Box::new(face_keys_get_morph_key::FaceKeysGetMorphKeyOp {}));
    result.push(Box::new(
        face_keys_get_skin_color::FaceKeysGetSkinColorOp {},
    ));
    result.push(Box::new(face_keys_set_age::FaceKeysSetAgeOp {}));
    result.push(Box::new(face_keys_set_beard::FaceKeysSetBeardOp {}));
    result.push(Box::new(
        face_keys_set_face_texture::FaceKeysSetFaceTextureOp {},
    ));
    result.push(Box::new(face_keys_set_hair::FaceKeysSetHairOp {}));
    result.push(Box::new(
        face_keys_set_hair_color::FaceKeysSetHairColorOp {},
    ));
    result.push(Box::new(
        face_keys_set_hair_texture::FaceKeysSetHairTextureOp {},
    ));
    result.push(Box::new(face_keys_set_morph_key::FaceKeysSetMorphKeyOp {}));
    result.push(Box::new(
        face_keys_set_skin_color::FaceKeysSetSkinColorOp {},
    ));
    result.push(Box::new(player_set_face_keys::PlayerSetFaceKeysOp {}));
    result.push(Box::new(
        str_store_player_face_keys::StrStorePlayerFaceKeysOp {},
    ));
    result.push(Box::new(
        str_store_troop_face_keys::StrStoreTroopFaceKeysOp {},
    ));
    result.push(Box::new(troop_set_face_keys::TroopSetFaceKeysOp {}));
    result
}
