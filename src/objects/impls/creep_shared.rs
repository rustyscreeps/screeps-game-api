use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    local::RoomName,
    objects::{CostMatrix, FindPathOptions, PolyStyle},
    pathfinder::SingleRoomCostResult,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type JsMoveToOptions;

    #[wasm_bindgen(method, setter = reusePath)]
    pub fn reuse_path(this: &JsMoveToOptions, ticks: u32);

    #[wasm_bindgen(method, setter = serializeMemory)]
    pub fn serialize_memory(this: &JsMoveToOptions, serialize: bool);

    #[wasm_bindgen(method, setter = noPathFinding)]
    pub fn no_path_finding(this: &JsMoveToOptions, require: bool);

    #[wasm_bindgen(method, setter = vizualizePathStyle)]
    pub fn visualize_path_style(this: &JsMoveToOptions, style: &JsValue);

    // todo this is wrong, the additional options are supposed to be added to the
    // same object
    #[wasm_bindgen(method, setter = heuristicWeight)]
    pub fn find_path_options(this: &JsMoveToOptions, options: &JsValue);
}

impl JsMoveToOptions {
    pub fn new() -> JsMoveToOptions {
        Object::new().unchecked_into()
    }
}

impl Default for JsMoveToOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MoveToOptions<F>
where
    F: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult,
{
    pub(crate) reuse_path: Option<u32>,
    pub(crate) serialize_memory: Option<bool>,
    pub(crate) no_path_finding: Option<bool>,
    pub(crate) visualize_path_style: Option<PolyStyle>,
    pub(crate) find_path_options: FindPathOptions<F, SingleRoomCostResult>,
}

impl Default for MoveToOptions<fn(RoomName, CostMatrix) -> SingleRoomCostResult> {
    fn default() -> Self {
        MoveToOptions {
            reuse_path: None,
            serialize_memory: None,
            no_path_finding: None,
            visualize_path_style: None,
            find_path_options: FindPathOptions::default(),
        }
    }
}

impl MoveToOptions<fn(RoomName, CostMatrix) -> SingleRoomCostResult> {
    /// Creates default SearchOptions
    pub fn new() -> Self {
        Self::default()
    }
}

impl<F> MoveToOptions<F>
where
    F: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult,
{
    /// Enables caching of the calculated path. Default: 5 ticks
    pub fn reuse_path(mut self, n_ticks: u32) -> Self {
        self.reuse_path = Some(n_ticks);
        self
    }

    /// Whether to use the short serialized form. Default: True
    pub fn serialize_memory(mut self, serialize: bool) -> Self {
        self.serialize_memory = Some(serialize);
        self
    }

    /// Return an `ERR_NOT_FOUND` if no path is already cached. Default: False
    pub fn no_path_finding(mut self, no_finding: bool) -> Self {
        self.no_path_finding = Some(no_finding);
        self
    }

    /// Sets the style to trace the path used by this creep. See doc for
    /// default.
    pub fn visualize_path_style(mut self, style: PolyStyle) -> Self {
        self.visualize_path_style = Some(style);
        self
    }

    /// Sets whether the algorithm considers creeps as walkable. Default: False.
    pub fn ignore_creeps(mut self, ignore: bool) -> Self {
        self.find_path_options.ignore_creeps = Some(ignore);
        self
    }

    /// Sets whether the algorithm considers destructible structure as
    /// walkable. Default: False.
    pub fn ignore_destructible_structures(mut self, ignore: bool) -> Self {
        self.find_path_options.ignore_destructible_structures = Some(ignore);
        self
    }

    /// Sets cost callback - default `|_, _| {}`.
    pub fn cost_callback<F2>(self, cost_callback: F2) -> MoveToOptions<F2>
    where
        F2: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult,
    {
        let new_options: MoveToOptions<F2> = MoveToOptions {
            reuse_path: self.reuse_path,
            serialize_memory: self.serialize_memory,
            no_path_finding: self.no_path_finding,
            visualize_path_style: self.visualize_path_style,
            find_path_options: self.find_path_options.cost_callback(cost_callback),
        };

        new_options
    }

    /// Sets maximum ops - default `2000`.
    pub fn max_ops(mut self, ops: u32) -> Self {
        self.find_path_options.max_ops = Some(ops);
        self
    }

    /// Sets heuristic weight - default `1.2`.
    pub fn heuristic_weight(mut self, weight: f64) -> Self {
        self.find_path_options.heuristic_weight = Some(weight);
        self
    }

    /// Sets whether the returned path should be passed to `Room.serializePath`.
    pub fn serialize(mut self, s: bool) -> Self {
        self.find_path_options.serialize = Some(s);
        self
    }

    /// Sets maximum rooms - default `16`, max `16`.
    pub fn max_rooms(mut self, rooms: u8) -> Self {
        self.find_path_options.max_rooms = Some(rooms);
        self
    }

    pub fn range(mut self, k: u32) -> Self {
        self.find_path_options.range = Some(k);
        self
    }

    /// Sets plain cost - default `1`.
    pub fn plain_cost(mut self, cost: u8) -> Self {
        self.find_path_options.plain_cost = Some(cost);
        self
    }

    /// Sets swamp cost - default `5`.
    pub fn swamp_cost(mut self, cost: u8) -> Self {
        self.find_path_options.swamp_cost = Some(cost);
        self
    }

    /// Sets options related to FindPathOptions. Defaults to FindPathOptions
    /// default.
    pub fn find_path_options<F2>(
        self,
        find_path_options: FindPathOptions<F2, SingleRoomCostResult>,
    ) -> MoveToOptions<F2>
    where
        F2: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult,
    {
        MoveToOptions {
            reuse_path: self.reuse_path,
            serialize_memory: self.serialize_memory,
            no_path_finding: self.no_path_finding,
            visualize_path_style: self.visualize_path_style,
            find_path_options,
        }
    }

    pub(crate) fn into_js_options<CR>(self, callback: impl Fn(&JsMoveToOptions) -> CR) -> CR {
        //
        // Create JS object and set properties.
        //

        let js_options = JsMoveToOptions::new();

        if let Some(reuse_path) = self.reuse_path {
            js_options.reuse_path(reuse_path);
        }

        if let Some(serialize_memory) = self.serialize_memory {
            js_options.serialize_memory(serialize_memory);
        }

        if let Some(no_path_finding) = self.no_path_finding {
            js_options.no_path_finding(no_path_finding);
        }

        if let Some(visualize_path_style) = self.visualize_path_style {
            let style = serde_wasm_bindgen::to_value(&visualize_path_style)
                .expect("expected to serialize visualize path style");

            js_options.visualize_path_style(&style);
        }

        self.find_path_options.into_js_options(|find_path_options| {
            js_options.find_path_options(find_path_options);

            callback(&js_options)
        })
    }
}
