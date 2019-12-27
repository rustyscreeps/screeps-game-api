use std::{marker::PhantomData, mem};

use scoped_tls::scoped_thread_local;
use stdweb::Reference;

use crate::{
    constants::{Direction, ResourceType, ReturnCode},
    local::{Position, RoomName},
    memory::MemoryReference,
    objects::{
        Creep, FindOptions, HasPosition, PowerCreep, Resource, RoomObjectProperties, Step,
        Transferable, Withdrawable,
    },
    pathfinder::{CostMatrix, SearchResults},
    ConversionError,
};

scoped_thread_local!(static COST_CALLBACK: Box<dyn Fn(RoomName, Reference) -> Option<Reference>>);

/// Trait for all wrappers over Screeps JavaScript objects that are creeps or
/// power creeps
///
/// # Contracts
///
/// The reference returned by `AsRef<Reference>::as_ref` must reference a
/// JavaScript object that an instance of `Creep` or `PowerCreep`
pub unsafe trait SharedCreepProperties: RoomObjectProperties {
    fn cancel_order(&self, name: &str) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.cancelOrder(@{name}))
    }

    fn drop(&self, ty: ResourceType, amount: Option<u32>) -> ReturnCode {
        match amount {
            Some(v) => {
                js_unwrap!(@{self.as_ref()}.drop(__resource_type_num_to_str(@{ty as u32}), @{v}))
            }
            None => js_unwrap!(@{self.as_ref()}.drop(__resource_type_num_to_str(@{ty as u32}))),
        }
    }

    fn move_direction(&self, dir: Direction) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.move(@{dir as u32}))
    }

    fn move_to_xy(&self, x: u32, y: u32) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveTo(@{x}, @{y}))
    }

    fn move_to_xy_with_options<'a, F>(
        &self,
        x: u32,
        y: u32,
        move_options: MoveToOptions<'a, F>,
    ) -> ReturnCode
    where
        F: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'a>> + 'a,
    {
        let pos = Position::new(x, y, self.pos().room_name());
        self.move_to_with_options(&pos, move_options)
    }

    fn move_to<T: ?Sized + HasPosition>(&self, target: &T) -> ReturnCode {
        let p = target.pos();
        js_unwrap!(@{self.as_ref()}.moveTo(pos_from_packed(@{p.packed_repr()})))
    }

    fn move_to_with_options<'a, F, T>(
        &self,
        target: &T,
        move_options: MoveToOptions<'a, F>,
    ) -> ReturnCode
    where
        T: ?Sized + HasPosition,
        F: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'a>> + 'a,
    {
        let MoveToOptions {
            reuse_path,
            serialize_memory,
            no_path_finding,
            // visualize_path_style,
            find_options:
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
                },
        } = move_options;

        // This callback is the one actually passed to JavaScript.
        fn callback(room_name: String, cost_matrix: Reference) -> Option<Reference> {
            let room_name = room_name.parse().expect(
                "expected room name passed into Creep.moveTo \
                 callback to be a valid room name",
            );
            COST_CALLBACK.with(|callback| callback(room_name, cost_matrix))
        }

        // User provided callback: rust String, CostMatrix -> Option<CostMatrix>
        let raw_callback = cost_callback;

        // Wrapped user callback: rust String, Reference -> Option<Reference>
        let callback_boxed = move |room_name, cost_matrix_ref| {
            let cmatrix = CostMatrix {
                inner: cost_matrix_ref,
                lifetime: PhantomData,
            };
            raw_callback(room_name, cmatrix).map(|cm| cm.inner)
        };

        // Type erased and boxed callback: no longer a type specific to the closure
        // passed in, now unified as Box<Fn>
        let callback_type_erased: Box<dyn Fn(RoomName, Reference) -> Option<Reference> + 'a> =
            Box::new(callback_boxed);

        // Overwrite lifetime of box inside closure so it can be stuck in
        // scoped_thread_local storage: now pretending to be static data so that
        // it can be stuck in scoped_thread_local. This should be entirely safe
        // because we're only sticking it in scoped storage and we control the only use
        // of it, but it's still necessary because "some lifetime above the current
        // scope but otherwise unknown" is not a valid lifetime to have
        // PF_CALLBACK have.
        let callback_lifetime_erased: Box<
            dyn Fn(RoomName, Reference) -> Option<Reference> + 'static,
        > = unsafe { mem::transmute(callback_type_erased) };

        // Store callback_lifetime_erased in COST_CALLBACK for the duration of the
        // PathFinder call and make the call to PathFinder.
        //
        // See https://docs.rs/scoped-tls/0.1/scoped_tls/
        COST_CALLBACK.set(&callback_lifetime_erased, || {
            let rp = target.pos();
            js_unwrap! {
                @{ self.as_ref() }.moveTo(
                    pos_from_packed(@{rp.packed_repr()}),
                    {
                        reusePath: @{reuse_path},
                        serializeMemory: @{serialize_memory},
                        noPathFinding: @{no_path_finding},
                        visualizePathStyle: undefined,  // todo
                        ignoreCreeps: @{ignore_creeps},
                        ignoreDestructibleStructures: @{ignore_destructible_structures},
                        costCallback: @{callback},
                        maxOps: @{max_ops},
                        heuristicWeight: @{heuristic_weight},
                        serialize: @{serialize},
                        maxRooms: @{max_rooms},
                        range: @{range},
                        plainCost: @{plain_cost},
                        swampCost: @{swamp_cost}
                    }
                )
            }
        })
    }

    fn move_by_path_serialized(&self, path: &str) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveByPath(@{path}))
    }

    fn move_by_path_steps(&self, path: &[Step]) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveByPath(@{path}))
    }

    fn move_by_path_search_result(&self, path: &SearchResults) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveByPath(@{path.opaque_path()}))
    }

    fn memory(&self) -> MemoryReference {
        js_unwrap!(@{self.as_ref()}.memory)
    }

    fn my(&self) -> bool {
        js_unwrap!(@{self.as_ref()}.my)
    }

    fn name(&self) -> String {
        js_unwrap!(@{self.as_ref()}.name)
    }

    fn notify_when_attacked(&self, notify_when_attacked: bool) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.notifyWhenAttacked(@{notify_when_attacked}))
    }

    fn owner_name(&self) -> String {
        js_unwrap!(@{self.as_ref()}.owner.username)
    }

    fn pickup(&self, target: &Resource) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.pickup(@{target.as_ref()}))
    }

    fn say(&self, msg: &str, public: bool) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.say(@{msg}, @{public}))
    }

    fn saying(&self) -> String {
        js_unwrap!(@{self.as_ref()}.saying)
    }

    fn suicide(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.suicide())
    }

    fn ticks_to_live(&self) -> Result<u32, ConversionError> {
        let ttl = crate::traits::TryInto::try_into(js!(
            return @{self.as_ref()}.ticksToLive;
        ))?;

        Ok(ttl)
    }

    fn transfer_amount<T>(&self, target: &T, ty: ResourceType, amount: u32) -> ReturnCode
    where
        T: ?Sized + Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32}),
            @{amount}
        ))
    }

    fn transfer_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: ?Sized + Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32})
        ))
    }

    fn withdraw_amount<T>(&self, target: &T, ty: ResourceType, amount: u32) -> ReturnCode
    where
        T: ?Sized + Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32}),
            @{amount}
        ))
    }

    fn withdraw_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: ?Sized + Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32})
        ))
    }
}

unsafe impl SharedCreepProperties for Creep {}
unsafe impl SharedCreepProperties for PowerCreep {}

pub struct MoveToOptions<'a, F>
where
    F: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'a>>,
{
    pub(crate) reuse_path: u32,
    pub(crate) serialize_memory: bool,
    pub(crate) no_path_finding: bool,
    // pub(crate) visualize_path_style: PolyStyle,
    pub(crate) find_options: FindOptions<'a, F>,
}

impl Default
    for MoveToOptions<'static, fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'static>>>
{
    fn default() -> Self {
        // TODO: should we fall back onto the game's default values, or is
        // it alright to copy them here?
        MoveToOptions {
            reuse_path: 5,
            serialize_memory: true,
            no_path_finding: false,
            // visualize_path_style: None,
            find_options: FindOptions::default(),
        }
    }
}

impl MoveToOptions<'static, fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'static>>> {
    /// Creates default SearchOptions
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, F> MoveToOptions<'a, F>
where
    F: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'a>>,
{
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

    // /// Sets the style to trace the path used by this creep. See doc for default.
    // pub fn visualize_path_style(mut self, style: ) -> Self {
    //     self.visualize_path_style = style;
    //     self
    // }

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

    /// Sets cost callback - default `|_, _| {}`.
    pub fn cost_callback<'b, F2>(self, cost_callback: F2) -> MoveToOptions<'b, F2>
    where
        F2: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'b>>,
    {
        MoveToOptions {
            reuse_path: self.reuse_path,
            serialize_memory: self.serialize_memory,
            no_path_finding: self.no_path_finding,
            // self.visualize_path_style,
            find_options: self.find_options.cost_callback(cost_callback),
        }
    }

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
    pub fn find_options<'b, F2>(self, find_options: FindOptions<'b, F2>) -> MoveToOptions<'b, F2>
    where
        F2: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'b>>,
    {
        MoveToOptions {
            reuse_path: self.reuse_path,
            serialize_memory: self.serialize_memory,
            no_path_finding: self.no_path_finding,
            // self.visualize_path_style,
            find_options,
        }
    }
}
