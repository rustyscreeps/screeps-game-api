use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::constants::ResourceType;

//TODO: wiarchbe: Need types for general purpose store and specific store.
// (Specific store can return undefined for missing types.)
#[wasm_bindgen]
extern "C" {
    /// An object that represents the cargo within an entity in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store)
    #[wasm_bindgen]
    pub type Store;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &Store, ty: ResourceType) -> Option<u32>;

    /// Get the capacity of the [`Store`] for the specified resource. If the
    /// [`Store`] can contain any resource, passing `None` as the type will get
    /// the general store capacity.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store.getCapacity)
    #[wasm_bindgen(method, js_name = getCapacity)]
    fn get_capacity_internal(this: &Store, ty: Option<ResourceType>) -> Option<u32>;

    /// Return the free capacity of the [`Store`] for the specified resource.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store.getFreeCapacity)
    #[wasm_bindgen(method, js_name = getFreeCapacity)]
    fn get_free_capacity_internal(this: &Store, ty: Option<ResourceType>) -> Option<i32>;

    /// Return the used capacity of the [`Store`] for the specified resource. If
    /// the [`Store`] can contain any resource, passing `None` as the type will
    /// get the total used capacity.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store.getUsedCapacity)
    #[wasm_bindgen(method, js_name = getUsedCapacity)]
    fn get_used_capacity_internal(this: &Store, ty: Option<ResourceType>) -> Option<u32>;
}

impl Store {
    pub fn store_types(&self) -> Vec<ResourceType> {
        Object::keys(self.unchecked_ref())
            .iter()
            .filter_map(|v| ResourceType::from_js_value(&v))
            .collect()
    }

    pub fn get_capacity(&self, ty: Option<ResourceType>) -> u32 {
        self.get_capacity_internal(ty).unwrap_or(0)
    }

    pub fn get_free_capacity(&self, ty: Option<ResourceType>) -> i32 {
        self.get_free_capacity_internal(ty).unwrap_or(0)
    }

    pub fn get_used_capacity(&self, ty: Option<ResourceType>) -> u32 {
        self.get_used_capacity_internal(ty).unwrap_or(0)
    }
}
