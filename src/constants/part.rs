use enum_iterator::Sequence;
use js_sys::{Array, JsString, Map};
use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::prelude::*;

use crate::objects::BodyPart;

/// Translates body part type and `BODYPARTS_ALL` constants
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    Sequence,
)]
#[repr(u8)]
pub enum Part {
    Move = 0,
    Work = 1,
    Carry = 2,
    Attack = 3,
    RangedAttack = 4,
    Tough = 5,
    Heal = 6,
    Claim = 7,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = MOVE)]
    static MOVE_JS: JsString;
    #[wasm_bindgen(js_name = WORK)]
    static WORK_JS: JsString;
    #[wasm_bindgen(js_name = CARRY)]
    static CARRY_JS: JsString;
    #[wasm_bindgen(js_name = ATTACK)]
    static ATTACK_JS: JsString;
    #[wasm_bindgen(js_name = RANGED_ATTACK)]
    static RANGED_ATTACK_JS: JsString;
    #[wasm_bindgen(js_name = TOUGH)]
    static TOUGH_JS: JsString;
    #[wasm_bindgen(js_name = HEAL)]
    static HEAL_JS: JsString;
    #[wasm_bindgen(js_name = CLAIM)]
    static CLAIM_JS: JsString;
}

thread_local! {
    static PART_NUM_TO_STR_MAP: js_sys::Map = {
        js_sys::Map::new()
            .set(&JsValue::from(Part::Move as u8), &MOVE_JS)
            .set(&JsValue::from(Part::Work as u8), &WORK_JS)
            .set(&JsValue::from(Part::Carry as u8), &CARRY_JS)
            .set(&JsValue::from(Part::Attack as u8), &ATTACK_JS)
            .set(&JsValue::from(Part::RangedAttack as u8), &RANGED_ATTACK_JS)
            .set(&JsValue::from(Part::Tough as u8), &TOUGH_JS)
            .set(&JsValue::from(Part::Heal as u8), &HEAL_JS)
            .set(&JsValue::from(Part::Claim as u8), &CLAIM_JS)
    };

    static PART_STR_TO_NUM_MAP: js_sys::Map = {
        js_sys::Map::new()
            .set(&MOVE_JS, &JsValue::from(Part::Move as u8))
            .set(&WORK_JS, &JsValue::from(Part::Work as u8))
            .set(&CARRY_JS, &JsValue::from(Part::Carry as u8))
            .set(&ATTACK_JS, &JsValue::from(Part::Attack as u8))
            .set(&RANGED_ATTACK_JS, &JsValue::from(Part::RangedAttack as u8))
            .set(&TOUGH_JS, &JsValue::from(Part::Tough as u8))
            .set(&HEAL_JS, &JsValue::from(Part::Heal as u8))
            .set(&CLAIM_JS, &JsValue::from(Part::Claim as u8))
    };
}

#[cfg(feature = "snippets")]
#[wasm_bindgen(module = "/js/part.js")]
extern "C" {
    fn bodypart_to_part_num(map: &Map, body_part: &BodyPart) -> Part;
    fn part_nums_to_str_array(map: &Map, part_array: &[u8]) -> Array;
}

#[cfg(not(feature = "snippets"))]
fn bodypart_to_part_num(map: &Map, body_part: &BodyPart) -> Part {
    use num_traits::FromPrimitive;

    let n = map.get(&body_part.part_jsvalue()).as_f64().expect("number") as u8;
    Part::from_u8(n).expect("known part")
}

#[cfg(not(feature = "snippets"))]
fn part_nums_to_str_array(map: &Map, part_array: &[u8]) -> Array {
    let array = Array::new();
    for part_num in part_array {
        array.push(&map.get(&JsValue::from(*part_num)));
    }
    array
}

impl Part {
    /// Translates the `BODYPART_COST` constant.
    #[inline]
    pub const fn cost(self) -> u32 {
        match self {
            Part::Move => 50,
            Part::Work => 100,
            Part::Carry => 50,
            Part::Attack => 80,
            Part::RangedAttack => 150,
            Part::Tough => 10,
            Part::Heal => 250,
            Part::Claim => 600,
        }
    }

    pub(crate) fn slice_to_js_array(parts: &[Self]) -> Array {
        PART_NUM_TO_STR_MAP.with(|map| {
            // SAFETY: &[Part] contains u8 values because it's repr(u8), safe to transmute
            // to &[u8]
            part_nums_to_str_array(map, unsafe { std::mem::transmute(parts) })
        })
    }

    pub(crate) fn from_bodypart(body_part: &BodyPart) -> Self {
        PART_STR_TO_NUM_MAP.with(|map| bodypart_to_part_num(map, body_part))
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    use super::{part_nums_to_str_array, Part};

    thread_local! {
        static TEST_PART_NUM_TO_STR_MAP: js_sys::Map = {
            js_sys::Map::new()
                .set(&JsValue::from(Part::Move as u8), &JsValue::from_str("move"))
                .set(&JsValue::from(Part::Work as u8), &JsValue::from_str("work"))
                .set(&JsValue::from(Part::Carry as u8), &JsValue::from_str("carry"))
                .set(&JsValue::from(Part::Attack as u8), &JsValue::from_str("attack"))
                .set(&JsValue::from(Part::RangedAttack as u8), &JsValue::from_str("ranged_attack"))
                .set(&JsValue::from(Part::Tough as u8), &JsValue::from_str("tough"))
                .set(&JsValue::from(Part::Heal as u8), &JsValue::from_str("heal"))
                .set(&JsValue::from(Part::Claim as u8), &JsValue::from_str("claim"))
        };

        static TEST_PART_STR_TO_NUM_MAP: js_sys::Map = {
            js_sys::Map::new()
                .set(&JsValue::from_str("move"), &JsValue::from(Part::Move as u8))
                .set(&JsValue::from_str("work"), &JsValue::from(Part::Work as u8))
                .set(&JsValue::from_str("carry"), &JsValue::from(Part::Carry as u8))
                .set(&JsValue::from_str("attack"), &JsValue::from(Part::Attack as u8))
                .set(&JsValue::from_str("ranged_attack"), &JsValue::from(Part::RangedAttack as u8))
                .set(&JsValue::from_str("tough"), &JsValue::from(Part::Tough as u8))
                .set(&JsValue::from_str("heal"), &JsValue::from(Part::Heal as u8))
                .set(&JsValue::from_str("claim"), &JsValue::from(Part::Claim as u8))
        };
    }

    #[wasm_bindgen_test]
    pub fn parts_to_array() {
        let body = [Part::Work, Part::Carry, Part::Move, Part::Move].as_slice();
        let array = TEST_PART_NUM_TO_STR_MAP.with(|map| {
            // SAFETY: &[Part] contains u8 values because it's repr(u8), safe to transmute
            // to &[u8]
            part_nums_to_str_array(map, unsafe { std::mem::transmute(body) })
        });
        assert_eq!(array.length(), 4);
        assert_eq!(array.get(0), "work");
        assert_eq!(array.get(1), "carry");
        assert_eq!(array.get(2), "move");
        assert_eq!(array.get(3), "move");
    }
}
