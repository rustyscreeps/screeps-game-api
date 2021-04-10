use crate::{
    find::FindOptions, 
    objects::PolyStyle
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MoveToOptions
{
    pub(crate) reuse_path: u32,
    pub(crate) serialize_memory: bool,
    pub(crate) no_path_finding: bool,
    pub(crate) visualize_path_style: Option<PolyStyle>,
    pub(crate) find_options: FindOptions
}

impl Default for MoveToOptions {
    fn default() -> Self {
        MoveToOptions {
            reuse_path: 5,
            serialize_memory: true,
            no_path_finding: false,
            visualize_path_style: None,
            find_options: FindOptions::default(),
        }
    }
}

impl MoveToOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MoveToOptions {
    /// Enables caching of the calculated path. Default: 5 ticks
    pub fn reuse_path(mut self, n_ticks: u32) -> Self {
        self.reuse_path = n_ticks;
        self
    }

    /// Whether to use the short serialized form. Default: True
    pub fn serialize_memory(mut self, serialize: bool) -> Self {
        self.serialize_memory = serialize;
        self
    }

    /// Return an `ERR_NOT_FOUND` if no path is already cached. Default: False
    pub fn no_path_finding(mut self, no_finding: bool) -> Self {
        self.no_path_finding = no_finding;
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
        self.find_options.ignore_creeps = ignore;
        self
    }

    /// Sets whether the algorithm considers destructible structure as
    /// walkable. Default: False.
    pub fn ignore_destructible_structures(mut self, ignore: bool) -> Self {
        self.find_options.ignore_destructible_structures = ignore;
        self
    }

    //TODO: wiarchbe: Re-enable.
    /*
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
            find_options: self.find_options.cost_callback(cost_callback),
        };

        new_options
    }
    */    

    /// Sets maximum ops - default `2000`.
    pub fn max_ops(mut self, ops: u32) -> Self {
        self.find_options.max_ops = ops;
        self
    }

    /// Sets heuristic weight - default `1.2`.
    pub fn heuristic_weight(mut self, weight: f64) -> Self {
        self.find_options.heuristic_weight = weight;
        self
    }

    /// Sets whether the returned path should be passed to `Room.serializePath`.
    pub fn serialize(mut self, s: bool) -> Self {
        self.find_options.serialize = s;
        self
    }

    /// Sets maximum rooms - default `16`, max `16`.
    pub fn max_rooms(mut self, rooms: u32) -> Self {
        self.find_options.max_rooms = rooms;
        self
    }

    pub fn range(mut self, k: u32) -> Self {
        self.find_options.range = k;
        self
    }

    /// Sets plain cost - default `1`.
    pub fn plain_cost(mut self, cost: u8) -> Self {
        self.find_options.plain_cost = cost;
        self
    }

    /// Sets swamp cost - default `5`.
    pub fn swamp_cost(mut self, cost: u8) -> Self {
        self.find_options.swamp_cost = cost;
        self
    }

    /// Sets options related to FindOptions. Defaults to FindOptions default.
    pub fn find_options(self, find_options: FindOptions) -> MoveToOptions {
        MoveToOptions {
            reuse_path: self.reuse_path,
            serialize_memory: self.serialize_memory,
            no_path_finding: self.no_path_finding,
            visualize_path_style: self.visualize_path_style,
            find_options,
        }
    }
}
