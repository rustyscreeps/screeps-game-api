use std::{marker::PhantomData, mem, ops::Range};

use stdweb::Reference;

use {
    constants::{
        find::Exit, Color, Direction, FindConstant, LookConstant, ReturnCode, StructureType,
    },
    memory::MemoryReference,
    objects::{
        HasPosition, Room, RoomPosition, StructureController, StructureStorage, StructureTerminal,
    },
    pathfinder::CostMatrix,
    positions::LocalRoomName,
    traits::TryInto,
};

simple_accessors! {
    Room;
    (controller -> controller -> Option<StructureController>),
    (energy_available -> energyAvailable -> i32),
    (energy_capacity_available -> energyCapacityAvailable -> i32),
    (name -> name -> String),
    (storage -> storage -> Option<StructureStorage>),
    (terminal -> terminal -> Option<StructureTerminal>),
    // todo: visual
}

scoped_thread_local!(static COST_CALLBACK: Box<Fn(String, Reference) -> Option<Reference>>);

impl Room {
    pub fn serialize_path(&self, path: &Vec<Step>) -> String {
        js_unwrap!{@{self.as_ref()}.serializePath(@{path})}
    }

    pub fn deserialize_path(&self, path: &str) -> Vec<Step> {
        js_unwrap!{@{self.as_ref()}.deserializePath(@{path})}
    }

    pub fn create_construction_site<T>(&self, at: T, ty: StructureType) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            @{pos.as_ref()},
            __structure_type_num_to_str(@{ty as i32})
        ))
    }

    pub fn create_named_construction_site<T>(
        &self,
        at: T,
        ty: StructureType,
        name: &str,
    ) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            @{pos.as_ref()},
            __structure_type_num_to_str(@{ty as i32}),
            @{name}
        ))
    }

    pub fn create_flag<T>(
        &self,
        at: T,
        name: &str,
        main_color: Color,
        secondary_color: Color,
    ) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createFlag(
            @{pos.as_ref()},
            @{name},
            @{main_color as i32},
            @{secondary_color as i32}
        ))
    }

    pub fn find<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(@{self.as_ref()}.find(@{ty.find_code()}))
    }

    pub fn find_exit_to(&self, room: &Room) -> Result<Exit, ReturnCode> {
        let code_val = js! {return @{self.as_ref()}.findExitTo(@{room.as_ref()});};
        let code_int: i32 = code_val.try_into().unwrap();

        if code_int < 0 {
            Err(code_int.try_into().unwrap())
        } else {
            Ok(code_int.try_into().unwrap())
        }
    }

    pub fn get_position_at(&self, x: u32, y: u32) -> Option<RoomPosition> {
        js_unwrap!{@{self.as_ref()}.get_position_at(@{x}, @{y})}
    }

    // pub fn look_at(&self, x: u32, y: u32) -> ! {
    //     unimplemented!()
    // }

    // pub fn look_at_area(&self, top: u32, left: u32, bottom: u32, right: u32) -> ! {
    //     unimplemented!()
    // }

    pub fn find_path<'a, O, T, F>(&self, from_pos: &O, to_pos: &T, opts: FindOptions<'a, F>) -> Path
    where
        O: HasPosition,
        T: HasPosition,
        F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>> + 'a,
    {
        let from = from_pos.pos();
        let to = to_pos.pos();

        // This callback is the one actually passed to JavaScript.
        fn callback(room_name: String, cost_matrix: Reference) -> Option<Reference> {
            COST_CALLBACK.with(|callback| callback(room_name, cost_matrix))
        }

        // User provided callback: rust String, CostMatrix -> Option<CostMatrix>
        let raw_callback = opts.cost_callback;

        // Wrapped user callback: rust String, Reference -> Option<Reference>
        let callback_boxed = move |room_name, cost_matrix_ref| {
            let cmatrix = CostMatrix {
                inner: cost_matrix_ref,
                lifetime: PhantomData,
            };
            raw_callback(room_name, cmatrix).map(|cm| cm.inner)
        };

        // Type erased and boxed callback: no longer a type specific to the closure passed in,
        // now unified as Box<Fn>
        let callback_type_erased: Box<Fn(String, Reference) -> Option<Reference> + 'a> =
            Box::new(callback_boxed);

        // Overwrite lifetime of box inside closure so it can be stuck in scoped_thread_local storage:
        // now pretending to be static data so that it can be stuck in scoped_thread_local. This should
        // be entirely safe because we're only sticking it in scoped storage and we control the only use
        // of it, but it's still necessary because "some lifetime above the current scope but otherwise
        // unknown" is not a valid lifetime to have PF_CALLBACK have.
        let callback_lifetime_erased: Box<
            Fn(String, Reference) -> Option<Reference> + 'static,
        > = unsafe { mem::transmute(callback_type_erased) };

        let FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
            ..
        } = opts;

        // Store callback_lifetime_erased in PF_CALLBACK for the duration of the PathFinder call and
        // make the call to PathFinder.
        //
        // See https://docs.rs/scoped-tls/0.1/scoped_tls/
        COST_CALLBACK.set(&callback_lifetime_erased, || {
            let v = js!{
                return @{&self.as_ref()}.search(@{from.as_ref()}, @{to.as_ref()}, {
                    ignoreCreeps: @{ignore_creeps},
                    ignoreDestructibleStructures: @{ignore_destructible_structures}
                    costCallback: @{callback},
                    maxOps: @{max_ops},
                    heuristicWeight: @{heuristic_weight},
                    serialize: @{serialize},
                    maxRooms: @{max_rooms},
                    range: @{range},
                    plainCost: @{plain_cost},
                    swampCost: @{swamp_cost}
                });
            };
            if serialize {
                Path::Serialized(v.try_into().unwrap())
            } else {
                Path::Vectorized(v.try_into().unwrap())
            }
        })
    }

    pub fn look_for_at<T, U>(&self, ty: T, target: U) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        let pos = target.pos();
        T::convert_and_check_items(js_unwrap!(@{self.as_ref()}.lookForAt(
            __look_num_to_str(@{ty.look_code() as i32}),
            @{pos.as_ref()}
        )))
    }

    /// Looks for a given thing over a given area of bounds.
    ///
    /// To keep with `Range` convention, the start is inclusive, and the end
    /// is _exclusive_.
    ///
    /// Note: to ease the implementation and efficiency of the rust interface, this is limited to
    /// returning an array of values without their positions. If position data is needed, all room
    /// objects *should* contain positions alongside them. (for terrain data, I would recommend
    /// using a different method?)
    ///
    /// If you really do need more information here, I would recommend making a PR to add it!
    ///
    /// # Panics
    ///
    /// Panics if start>end for either range, or if end>50 for either range.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use screeps::constants::look;
    /// room.look_for_at_area(look::ENERGY, 20..26, 20..26)
    /// ```
    pub fn look_for_at_area<T>(&self, ty: T, horiz: Range<u8>, vert: Range<u8>) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        assert!(horiz.start <= horiz.end);
        assert!(vert.start <= vert.end);
        assert!(horiz.end <= 50);
        assert!(vert.end <= 50);

        T::convert_and_check_items(js_unwrap!{@{self.as_ref()}.lookForAtArea(
            __look_num_to_str(@{ty.look_code() as i32}),
            @{vert.start},
            @{horiz.start},
            @{vert.end},
            @{horiz.end},
            true
        ).map((obj) => obj[__look_num_to_str(@{ty.look_code() as i32})])})
    }

    pub fn memory(&self) -> MemoryReference {
        js_unwrap!(@{self.as_ref()}.memory)
    }

    pub fn name_local(&self) -> LocalRoomName {
        js_unwrap!(@{self.as_ref()}.name)
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Room) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Room {}

pub struct FindOptions<'a, F>
where
    F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>>,
{
    ignore_creeps: bool,
    ignore_destructible_structures: bool,
    cost_callback: F,
    max_ops: u32,
    heuristic_weight: f64,
    serialize: bool,
    max_rooms: u32,
    range: u32,
    plain_cost: u8,
    swamp_cost: u8,
}

impl Default for FindOptions<'static, fn(String, CostMatrix) -> Option<CostMatrix<'static>>> {
    fn default() -> Self {
        fn cost_matrix(_: String, _: CostMatrix) -> Option<CostMatrix<'static>> {
            None
        }

        // TODO: should we fall back onto the game's default values, or is
        // it alright to copy them here?
        FindOptions {
            ignore_creeps: false,
            ignore_destructible_structures: false,
            cost_callback: cost_matrix,
            max_ops: 2000,
            heuristic_weight: 1.2,
            serialize: false,
            max_rooms: 16,
            range: 0,
            plain_cost: 1,
            swamp_cost: 5,
        }
    }
}

impl FindOptions<'static, fn(String, CostMatrix) -> Option<CostMatrix<'static>>> {
    /// Creates default SearchOptions
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, F> FindOptions<'a, F>
where
    F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>>,
{
    /// Sets whether the algorithm considers creeps as walkable. Default: False.
    pub fn ignore_creeps(mut self, ignore: bool) -> Self {
        self.ignore_creeps = ignore;
        self
    }

    /// Sets whether the algorithm considers destructible structure as
    /// walkable. Default: False.
    pub fn ignore_destructible_structures(mut self, ignore: bool) -> Self {
        self.ignore_destructible_structures = ignore;
        self
    }

    /// Sets cost callback - default `|_, _| {}`.
    pub fn cost_callback<'b, F2>(self, cost_callback: F2) -> FindOptions<'b, F2>
    where
        F2: Fn(String, CostMatrix) -> Option<CostMatrix<'b>>,
    {
        let FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            cost_callback: _,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
        } = self;
        FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            cost_callback,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
        }
    }

    /// Sets maximum ops - default `2000`.
    pub fn max_ops(mut self, ops: u32) -> Self {
        self.max_ops = ops;
        self
    }

    /// Sets heuristic weight - default `1.2`.
    pub fn heuristic_weight(mut self, weight: f64) -> Self {
        self.heuristic_weight = weight;
        self
    }

    /// Sets whether the returned path should be passed to `Room.serializePath`.
    pub fn serialize(mut self, s: bool) -> Self {
        self.serialize = s;
        self
    }

    /// Sets maximum rooms - default `16`, max `16`.
    pub fn max_rooms(mut self, rooms: u32) -> Self {
        self.max_rooms = rooms;
        self
    }

    pub fn range(mut self, k: u32) -> Self {
        self.range = k;
        self
    }

    /// Sets plain cost - default `1`.
    pub fn plain_cost(mut self, cost: u8) -> Self {
        self.plain_cost = cost;
        self
    }

    /// Sets swamp cost - default `5`.
    pub fn swamp_cost(mut self, cost: u8) -> Self {
        self.swamp_cost = cost;
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Step {
    x: u32,
    y: u32,
    dx: i32,
    dy: i32,
    direction: Direction,
}

js_deserializable!{Step}
js_serializable!{Step}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Path {
    Vectorized(Vec<Step>),
    Serialized(String),
}

js_deserializable!{Path}
