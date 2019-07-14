use {
    objects::{HasId, RoomObject, SizedRoomObject},
    traits::TryInto,
    ConversionError,
};

// TODO: split these out into separate files once we add documentation.
//
// Right now, they can all fit in here because they're pretty small.
/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub mod cpu {
    use std::collections;

    use constants::ReturnCode;

    /// See [`v8_getheapstatistics`]
    ///
    /// [`v8_getheapstatistics`]: https://nodejs.org/dist/latest-v8.x/docs/api/v8.html#v8_v8_getheapstatistics
    #[derive(Default, Serialize, Deserialize)]
    pub struct HeapStatistics {
        pub total_heap_size: u32,
        pub total_heap_size_executable: u32,
        pub total_physical_size: u32,
        pub used_heap_size: u32,
        pub heap_size_limit: u32,
        pub malloced_memory: u32,
        pub peak_malloced_memory: u32,
        pub does_zap_garbage: u32,
        pub externally_allocated_size: u32,
    }

    js_serializable!(HeapStatistics);
    js_deserializable!(HeapStatistics);

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn limit() -> f64 {
        js_unwrap!(Game.cpu.limit)
    }

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn tick_limit() -> f64 {
        js_unwrap!(Game.cpu.tickLimit)
    }

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn bucket() -> f64 {
        js_unwrap!(Game.cpu.bucket)
    }

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn shard_limits() -> collections::HashMap<String, f64> {
        js_unwrap!(Game.cpu.shardLimits)
    }

    /// See [http://docs.screeps.com/api/#Game.getHeapStatistics]
    ///
    /// [http://docs.screeps.com/api/#Game.getHeapStatistics]: http://docs.screeps.com/api/#Game.getHeapStatistics
    ///
    /// Returns object with all 0 values if heap statistics are not available.
    pub fn get_heap_statistics() -> HeapStatistics {
        use stdweb::unstable::TryInto;
        use stdweb::Value;

        let heap_stats: Value =
            js_unwrap!(Game.cpu.getHeapStatistics && Game.cpu.getHeapStatistics());

        match heap_stats {
            Value::Null | Value::Undefined | Value::Bool(false) => HeapStatistics::default(),
            other => other.try_into().expect(
                "expected Game.cpu.getHeapStatistics() to return an object with a known format",
            ),
        }
    }

    /// See [http://docs.screeps.com/api/#Game.getUsed]
    ///
    /// [http://docs.screeps.com/api/#Game.getUsed]: http://docs.screeps.com/api/#Game.getUsed
    pub fn get_used() -> f64 {
        js_unwrap!(Game.cpu.getUsed())
    }

    /// See [http://docs.screeps.com/api/#Game.setShardLimits]
    ///
    /// [http://docs.screeps.com/api/#Game.setShardLimits]: http://docs.screeps.com/api/#Game.setShardLimits
    pub fn set_shard_limits(limits: collections::HashMap<String, f64>) -> ReturnCode {
        js_unwrap!(Game.cpu.setShardLimits(@{limits}))
    }
}

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub mod gcl {
    /// See [http://docs.screeps.com/api/#Game.gcl]
    ///
    /// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
    pub fn level() -> u32 {
        js_unwrap!(Game.gcl.level)
    }

    /// See [http://docs.screeps.com/api/#Game.gcl]
    ///
    /// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
    pub fn progress() -> f64 {
        js_unwrap!(Game.gcl.progress)
    }

    /// See [http://docs.screeps.com/api/#Game.gcl]
    ///
    /// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
    pub fn progress_total() -> f64 {
        js_unwrap!(Game.gcl.progressTotal)
    }
}

/// See [http://docs.screeps.com/api/#Game.map]
///
/// [http://docs.screeps.com/api/#Game.map]: http://docs.screeps.com/api/#Game.map
pub mod map {
    use std::{collections, mem};

    use stdweb::Value;

    use {
        constants::{find::Exit, Direction, ReturnCode},
        objects::RoomTerrain,
        traits::{TryFrom, TryInto},
    };

    /// See [http://docs.screeps.com/api/#Game.map.describeExits]
    ///
    /// [http://docs.screeps.com/api/#Game.map.describeExits]: http://docs.screeps.com/api/#Game.map.describeExits
    pub fn describe_exits(room_name: &str) -> collections::HashMap<Direction, String> {
        use num_traits::FromPrimitive;

        let orig: collections::HashMap<String, String> =
            js_unwrap!(Game.map.describeExits(@{room_name}));

        orig.into_iter()
            .map(|(key, value)| {
                let key: u32 = key.parse().expect(
                    "expected all directions returned from Game.map.describeExits to be integers",
                );
                (
                Direction::from_u32(key).expect("expected all directions returned from Game.map.describeExits to be directions"),
                value,
            )
            })
            .collect()
    }

    /// See [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]
    ///
    /// [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]: http://docs.screeps.com/api/#Game.map.getRoomLinearDistance
    pub fn get_room_linear_distance(room1: &str, room2: &str, continuous: bool) -> u32 {
        js_unwrap!(Game.map.getRoomLinearDistance(@{room1}, @{room2}, @{continuous}))
    }

    pub fn get_room_terrain(room_name: &str) -> RoomTerrain {
        js_unwrap!(Game.map.getRoomTerrain(@{room_name}))
    }

    /// See [http://docs.screeps.com/api/#Game.map.getWorldSize]
    ///
    /// [http://docs.screeps.com/api/#Game.map.getWorldSize]: http://docs.screeps.com/api/#Game.map.getWorldSize
    pub fn get_world_size() -> u32 {
        js_unwrap!(Game.map.getWorldSize())
    }

    /// See [http://docs.screeps.com/api/#Game.map.isRoomAvailable]
    ///
    /// [http://docs.screeps.com/api/#Game.map.isRoomAvailable]: http://docs.screeps.com/api/#Game.map.isRoomAvailable
    pub fn is_room_available(room_name: &str) -> bool {
        js_unwrap!(Game.map.isRoomAvailable(@{room_name}))
    }

    /// Implements `Game.map.findExit`.
    pub fn find_exit(from_room: &str, to_room: &str) -> Result<Exit, ReturnCode> {
        let code: i32 = js_unwrap! {Game.map.findExit(@{from_room}, @{to_room})};
        Exit::try_from(code)
            .map_err(|v| v.try_into().expect("find_exit: Error code not recognized."))
    }

    pub fn find_exit_with_callback(
        from_room: &str,
        to_room: &str,
        route_callback: impl Fn(String, String) -> f64,
    ) -> Result<Exit, ReturnCode> {
        // Actual callback
        fn callback(room_name: String, from_room_name: String) -> f64 {
            FR_CALLBACK.with(|callback| callback(room_name, from_room_name))
        }

        let callback_type_erased: Box<dyn Fn(String, String) -> f64> = Box::new(route_callback);

        let callback_lifetime_erased: Box<dyn Fn(String, String) -> f64 + 'static> =
            unsafe { mem::transmute(callback_type_erased) };

        FR_CALLBACK.set(&callback_lifetime_erased, || {
            let code: i32 = js_unwrap! {Game.map.findExit(@{from_room}, @{to_room}, @{callback})};
            Exit::try_from(code)
                .map_err(|v| v.try_into().expect("find_exit: Error code not recognized."))
        })
    }

    pub fn find_route(from_room: &str, to_room: &str) -> Result<Vec<RoomRouteStep>, ReturnCode> {
        let v = js!(return Game.map.findRoute(@{from_room}, @{to_room}););
        parse_find_route_returned_value(v)
    }

    scoped_thread_local!(static FR_CALLBACK: Box<(dyn Fn(String, String) -> f64)>);

    pub fn find_route_with_callback(
        from_room: &str,
        to_room: &str,
        route_callback: impl Fn(String, String) -> f64,
    ) -> Result<Vec<RoomRouteStep>, ReturnCode> {
        // Actual callback
        fn callback(room_name: String, from_room_name: String) -> f64 {
            FR_CALLBACK.with(|callback| callback(room_name, from_room_name))
        }

        let callback_type_erased: Box<dyn Fn(String, String) -> f64> = Box::new(route_callback);

        let callback_lifetime_erased: Box<dyn Fn(String, String) -> f64 + 'static> =
            unsafe { mem::transmute(callback_type_erased) };

        FR_CALLBACK.set(&callback_lifetime_erased, || {
            let v = js!(return Game.map.findRoute(@{from_room}, @{to_room}, @{callback}););
            parse_find_route_returned_value(v)
        })
    }

    fn parse_find_route_returned_value(v: Value) -> Result<Vec<RoomRouteStep>, ReturnCode> {
        match v {
            Value::Number(x) => {
                let i: i32 = x.try_into().unwrap();
                Err(i
                    .try_into()
                    .unwrap_or_else(|val| panic!("Unexpected return code: {}", val)))
            }
            Value::Reference(_) => Ok(v.try_into().expect("Error on parsing exit directions.")),
            _ => panic!(
                "Game.map.findRoute expected Number or Reference, found {:?}.",
                v
            ),
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RoomRouteStep {
        exit: Exit,
        room: String,
    }
    js_deserializable!(RoomRouteStep);
}

pub mod market {
    use std::collections::HashMap;

    use stdweb::unstable::TryInto;

    use constants::{ResourceType, ReturnCode};
    use Room;

    #[repr(u32)]
    #[derive(Clone, Debug)]
    pub enum OrderType {
        Sell = 0,
        Buy = 1,
    }

    // impl OrderType {
    //     fn as_string(&self) -> String {
    //         match self {
    //             OrderType::Sell => String::from("sell"),
    //             OrderType::Buy => String::from("buy")
    //         }
    //     }
    // }

    #[derive(Deserialize, Debug)]
    pub struct Player {
        username: String,
    }
    js_deserializable!(Player);

    #[derive(Deserialize, Debug)]
    pub struct TransactionOrder {
        id: String,
        #[serde(rename = "type")]
        order_type: String,
        price: f64,
    }
    js_deserializable!(TransactionOrder);

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Transaction {
        transaction_id: String,
        time: u32,
        sender: Player,
        recipient: Player,
        resource_type: String,
        amount: u32,
        from: String,
        to: String,
        description: String,
        order: Option<TransactionOrder>,
    }
    js_deserializable!(Transaction);

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Order {
        id: String,
        created: u32,
        #[serde(rename = "type")]
        order_type: String,
        resource_type: String,
        room_name: String,
        amount: u32,
        remaining_amount: u32,
        price: f64,
    }
    js_deserializable!(Order);

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MyOrder {
        id: String,
        created: u32,
        active: bool,
        #[serde(rename = "type")]
        order_type: String,
        resource_type: String,
        room_name: String,
        amount: u32,
        remaining_amount: u32,
        total_amount: u32,
        price: f64,
    }
    js_deserializable!(MyOrder);

    pub fn credits() -> u32 {
        js_unwrap!(Game.market.credits)
    }

    pub fn incoming_transactions() -> Vec<Transaction> {
        let arr_transaction_value = js! {
            return Game.market.incomingTransactions;
        };
        arr_transaction_value.try_into().unwrap()
    }

    pub fn outgoing_transactions() -> Vec<Transaction> {
        let arr_transaction_value = js! {
            return Game.market.outgoingTransactions;
        };
        arr_transaction_value.try_into().unwrap()
    }

    pub fn orders() -> HashMap<String, MyOrder> {
        let order_book_value = js! {
            return Game.market.orders;
        };
        order_book_value.try_into().unwrap()
    }

    pub fn calc_transaction_cost(amount: u32, room1: &Room, room2: &Room) -> u32 {
        js_unwrap!(Game.market.calcTransactionCost(@{amount}, @{room1.name()}, @{room2.name()}))
    }

    pub fn cancel_order(order_id: &str) -> ReturnCode {
        js_unwrap!(Game.market.cancelOrder(@{order_id}))
    }

    pub fn change_order_price(order_id: &str, new_price: u32) -> ReturnCode {
        js_unwrap!(Game.market.changeOrderPrice(@{order_id}, @{new_price}))
    }

    pub fn create_order(
        order_type: OrderType,
        resource_type: ResourceType,
        price: f64,
        total_amount: u32,
        room: &Room,
    ) -> ReturnCode {
        js_unwrap! {
            Game.market.createOrder(__order_type_num_to_str(@{order_type as u32}),
                                    __resource_type_num_to_str(@{resource_type as u32}),
                                    @{price},
                                    @{total_amount},
                                    @{room.name()})
        }
    }

    pub fn deal(order_id: &str, amount: u32, target_room: &Room) -> ReturnCode {
        js_unwrap! {Game.market.deal(@{order_id}, @{amount}, @{target_room.name()})}
    }

    pub fn extend_order(order_id: &str, add_amount: u32) -> ReturnCode {
        js_unwrap! {Game.market.extendOrder(@{order_id}, @{add_amount})}
    }

    /// Get all orders from the market
    ///
    /// Contrary to the JS version, filtering should be done afterwards.
    pub fn get_all_orders() -> Vec<Order> {
        let all_order = js! {
            return Game.market.getAllOrders();
        };
        all_order.try_into().unwrap()
    }

    pub fn get_order(id: &str) -> Option<Order> {
        let order = js! {
            return Game.marget.getOrder(@{id});
        };
        order.try_into().ok()
    }
}

/// See [http://docs.screeps.com/api/#Game.shard]
///
/// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
pub mod shard {
    /// See [http://docs.screeps.com/api/#Game.shard]
    ///
    /// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
    pub fn name() -> String {
        js_unwrap!(Game.shard.name)
    }

    /// See [http://docs.screeps.com/api/#Game.shard]
    ///
    /// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
    pub fn shard_type() -> String {
        js_unwrap!(Game.shard.type)
    }

    /// See [http://docs.screeps.com/api/#Game.shard]
    ///
    /// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
    pub fn ptr() -> bool {
        js_unwrap!(Game.shard.ptr)
    }
}

game_map_access! {
    /// See [http://docs.screeps.com/api/#Game.constructionSites]
    ///
    /// [http://docs.screeps.com/api/#Game.constructionSites]: http://docs.screeps.com/api/#Game.constructionSites
    (construction_sites, objects::ConstructionSite, Game.constructionSites),
    /// See [http://docs.screeps.com/api/#Game.creeps]
    ///
    /// [http://docs.screeps.com/api/#Game.creeps]: http://docs.screeps.com/api/#Game.creeps
    (creeps, objects::Creep, Game.creeps),
    /// See [http://docs.screeps.com/api/#Game.flags]
    ///
    /// [http://docs.screeps.com/api/#Game.flags]: http://docs.screeps.com/api/#Game.flags
    (flags, objects::Flag, Game.flags),
    // TODO: See [http://docs.screeps.com/api/#Game.resources]
    ///
    /// [http://docs.screeps.com/api/#Game.resources]: http://docs.screeps.com/api/#Game.resources
    /// See [http://docs.screeps.com/api/#Game.rooms]
    ///
    /// [http://docs.screeps.com/api/#Game.rooms]: http://docs.screeps.com/api/#Game.rooms
    (rooms, objects::Room, Game.rooms),
    /// See [http://docs.screeps.com/api/#Game.spawns]
    ///
    /// [http://docs.screeps.com/api/#Game.spawns]: http://docs.screeps.com/api/#Game.spawns
    (spawns, objects::StructureSpawn, Game.spawns),
    /// See [http://docs.screeps.com/api/#Game.structures]
    ///
    /// [http://docs.screeps.com/api/#Game.structures]: http://docs.screeps.com/api/#Game.structures
    (structures, objects::Structure, Game.structures)
}

/// See [http://docs.screeps.com/api/#Game.time]
///
/// [http://docs.screeps.com/api/#Game.time]: http://docs.screeps.com/api/#Game.time
pub fn time() -> u32 {
    js_unwrap!(Game.time)
}

/// See [http://docs.screeps.com/api/#Game.getObjectById]
///
/// This gets an object expecting a specific type and will return a `ConversionError` if the type
/// does not match.
///
/// If all you want to assume is that something has an ID, use [`get_object_erased`].
/// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
pub fn get_object_typed<T>(id: &str) -> Result<Option<T>, ConversionError>
where
    T: HasId + SizedRoomObject,
{
    js!(return Game.getObjectById(@{id});).try_into()
}

/// See [http://docs.screeps.com/api/#Game.getObjectById]
///
/// This gets the object in 'erased' form - all that is known about it is that it's a RoomObject.
///
/// If a more specific type is expected, [`get_object_typed`] can be used.
/// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
pub fn get_object_erased(id: &str) -> Option<RoomObject> {
    js_unwrap_ref!(Game.getObjectById(@{id}))
}

pub fn notify(message: &str, group_interval: Option<u32>) {
    js! { @(no_return)
        Game.notify(@{message}, @{group_interval.unwrap_or(0)});
    }
}
