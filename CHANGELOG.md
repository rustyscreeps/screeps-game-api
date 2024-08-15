Unreleased
==========

### Bugfixes:

- Temporarily pin to wasm-bindgen 0.2.92 due to incompatible generated javascript

0.21.2 (2024-08-14)
===================

### Bugfixes:

- Update for new string enum implementation in wasm-bindgen 0.2.93

### Misc:

- Move crate constant `ROOM_AREA` to extra constants module and make public

0.21.1 (2024-06-28)
===================

### Additions:

- Add function `Direction::iter` which returns an iterator over all the `Direction` enum values
- Add function `RoomXY::neighbors` which returns an iterator over all the valid neighbors of
  a given `RoomXY` position
- Add static function `LocalCostMatrix::new_with_value` which returns a `LocalCostMatrix` with
  every position set to a given `u8` value
- Implement `JsCollectionIntoValue` and `JsCollectionFromValue` for `IntershardResourceType` and
  `u32` to allow `game::resources()` return value to be used as expected
- Add static function `RoomXY::new` to allow creating a new `RoomXY` from `RoomCoordinates`
- Add static function `RoomXY::checked_new` to allow creating a new `RoomXY` from a (u8, u8)
  pair, while checking the validity of the coordinates provided
- Add function `RoomXY::towards` which returns a `RoomXY` between two `RoomXY` positions,
  rounding towards the start position if necessary
- Add function `RoomXY::between` which returns a `RoomXY` between two `RoomXY` positions,
  rounding towards the target position if necessary
- Add function `RoomXY::midpoint_between` which returns the `RoomXY` midpoint between
  two `RoomXY` positions, rounding towards the target position if necessary
- Add function `RoomXY::offset` which modifies a `RoomXY` in-place by a (i8, i8) offset
- Add function `RoomXY::get_direction_to` which returns a `Direction` that is closest to
  a given `RoomXY` position
- Add function `RoomXY::get_range_to` which returns the Chebyshev Distance to a given
  `RoomXY` position
- Add function `RoomXY::in_range_to` which returns whether the Chebyshev Distance to a given
  `RoomXY` position is less-than-or-equal-to a given distance
- Add function `RoomXY::is_near_to` which returns whether a given `RoomXY` position is adjacent
- Add function `RoomXY::is_equal_to` which returns whether a given `RoomXY` position is
  the same position
- Implement the `PartialOrd` and `Ord` traits for `RoomXY`
- Implement the `Add<(i8, i8)>`, `Add<Direction>`, `Sub<(i8, i8)>`, `Sub<Direction>`,
  and `Sub<RoomXY>` traits for `RoomXY`

0.21.0 (2024-05-14)
===================

### Breaking:

- Change return type of `Flag::name` and `StructureSpawn::name` to `String` from `JsString`
- Change name of `LocalRoomTerrain::get` to `get_xy` for consistency with `RoomTerrain`
- Change name of `constants::extra::CONSTRUCTION_SITE_STOMP_RATIO` to
  `CONSTRUCTION_SITE_DROP_RATIO`
- Remove features `generate-pixel` and `inter-shard-memory`, use the `mmo` feature instead
- Place `game::cpu::{shard_limits, unlocked, unlocked_time, set_shard_limits, unlock}` functions
  behind the `mmo` feature
- Change return type of `game::{construction_sites, structures}` to `JsHashMap<ObjectId<_>, _>`
  instead of `JsHashMap<RawObjectId, _>`

### Additions:

- Add functions `game::creeps_jsstring`, `game::flags_jsstring`, `game::power_creeps_jsstring`,
  and `game::spawns_jsstring` to get the object as `JsHashMap<JsString, _>` instead of `String`
- Add function `game::market::orders_jsstring` to get the object as `JsHashMap<JsString, MyOrder>`
- Add function `name_jsstring` to `SharedCreepProperties` trait, `Flag`, and `StructureSpawn`
- Add function `raw_memory::segments_jsstring` to get the object as `JsHashMap<u8, JsString>`
- Add `RoomTerrain::get_xy` which takes a `RoomXY` instead of a coordinate pair
- Add `RoomCoordinate::is_room_edge`, `RoomXY::is_room_edge`, and `Position::is_room_edge`
- Add new extra constant `constants::extra::CONTROLLER_DOWNGRADE_PROGRESS_RATIO`, which is
  the percentage toward the previous level that a downgrading controller's progress is set to
- Add new extra constants `constants::extra::{MOVE_COST_PLAIN, MOVE_COST_ROAD, MOVE_COST_SWAMP}`
- Add constant `screeps::constants::resources::RESOURCES_ALL` (also accessible as 
  `screeps::RESOURCES_ALL`) that contains every resource type
- Add function `StoreObject::resource_types` to get the types of resources that a store can hold
    - This can be used with a `StructureObject` as `StoreObject::try_from(structure_object).map(|store| store.resource_types())`
      which returns a `Result<&'static [ResourceType], StoreObjectConversionError>`
- Add missing `StoreObject::Reactor` to the `seasonal-season-5` feature
- Implement `Serialize` and `Deserialize` for `RoomStatus`
- Add function `JsHashMap::entries`

### Bugfixes:

- Implement `JsCollectionFromValue` for `Direction`, `ObjectId<_>`
- Implement `Debug` for `RouteStep`
- Made the `AttackController` event deserialize correctly

### Misc:

- Change `PhantomData` in `screeps::local::ObjectId` to better model `ObjectId`'s relationship with
  the wrapped type.
    - This allows `ObjectId` to be `Send + Sync` regardless of the wrapped type
- Update `enum-iterator` to 2.0

0.20.1 (2024-01-09)
===================

### Additions:

- Add `StructureObject::as_repairable` matching related functions for other object traits
- Add implementations of `TryFrom<AccountPowerCreep>` and `AsRef<AccountPowerCreep>` for
  `PowerCreep`
- Add implementation of `From<PowerCreep>` for `AccountPowerCreep`
- Add `constants::extra::NOTIFY_MAX_LENGTH` with the length of allowed input to `game::notify`

0.20.0 (2024-01-08)
===================

### Breaking:

- Remove `HasNativeId`, `MaybeHasNativeId`, `HasTypedId`, and `MaybeHasTypedId` traits, adding
  their functions to the `HasId` and `MaybeHasId` traits
  - Renamed `native_id`/`try_native_id` to `js_raw_id`/`try_js_raw_id` for consistency with the
    other trait functions
- Remove `Resolvable` trait, moving its functionality to `MaybeHasId`
- Remove `ObjectWithId` and `ObjectWithMaybeId` enums
- Remove `Option<_>` for style options on `MapVisual`, fixes all visuals failing to render if
  one without style was used
- Remove `score`, `symbols`, and `thorium` features, moving all items they enabled to their
  respective `seasonal-season-1`, `seasonal-season-2`, and `seasonal-season-5` features
- Add `Repairable` trait, and change target type for `Creep::repair` and `StructureTower::repair`
  to `?Sized + Repairable` to match ergonomics of similar functions

### Bugfixes:

- Fix `raw_memory::set_public_segments` and `set_default_public_segment` argument conversion

0.19.0 (2023-12-20)
===================

### Breaking:

- A number of functions on `StructureController` now return `Option<u32>` to account for cases 
  where they may be undefined: `progress`, `progress_total`, `ticks_to_downgrade`, and
  `upgrade_blocked`

### Bugfixes:

- An undefined `hits` or `hitsMax` value on an invulnerable wall or certain controllers will no
  longer cause a panic when building in dev mode
- Fixed incorrect JavaScript field name on `StructurePortal::destination()` getter

0.18.0 (2023-11-27)
===================

### Breaking:

- Simplification of cost matrix types and traits:
  - Remove `SparseCostMatrix`, to be moved to `screeps-game-utils` crate
  - Remove `LocalCostMatrix::merge_from_dense` / `merge_from_sparse`
  - Remove `CostMatrixSet::set_multi` and replace `CostMatrixSet::set` with `set_xy`
  - Add `CostMatrixGet` trait with `get_xy` function
  - Remove `HasLocalPosition` trait with no implementations

0.17.0 (2023-11-27)
===================

### Breaking:

- Change `TOWER_OPTIMAL_RANGE` and `TOWER_FALLOFF_RANGE` types to `u8` and `TOWER_FALLOFF` type
  to `f64`
- Changed `RoomTerrain::new` and `game::map::get_room_terrain` return type to
  `Option<RoomTerrain>`, returning `None` when the specified room is outside the server's
  map
- Changed `game::map::get_room_status` return type to `Option<RoomStatusResult>`, returning
  `None` instead of the previous behavior of returning an artificial 'normal' status for rooms
  outside the server's map
- Remove `constants::math::control_points_for_gcl` and `power_for_gpl` - moved to new
  `screeps-game-utils` crate

### Bugfixes:

- Implement `JsCollectionIntoValue` for `Direction`, making the `JsHashMap` returned by
  `game::map::describe_exits` able to be used
- Handle object return properly from `RoomTerrain::get_raw_buffer_to_array` when built in dev mode

0.16.1 (2023-10-11)
===================

### Additions:

- Add new geometry helper functions to `Direction`: `is_diagonal`, `is_orthogonal`, `multi_rot`,
  `rot_cw`, and `rot_ccw`
- Add `checked_add` and `saturating_add` functions to `RoomCoordinate` and `RoomXY`, as well as
  `checked_add_direction` and `saturating_add_direction` to `RoomXY`

### Bugfixes:

- Fix incorrect setter name on `visualize_path_style` causing the setting to not work
- `OwnedStructure`, `OwnedStructureObject`, and `OwnedStructureProperties`'s `my` method now
  correctly handles the value being undefined.
  - This fixes a panic on checking `my` for unowned controllers. (again)

0.16.0 (2023-09-14)
===================

### Breaking:

- Use constant values compatible with the game for serializing `PowerCreepClass`, and
  `IntershardResourceType`, and `Part` as string - note that if you've stored any of these values,
  they will fail to parse after updating!
- Removed `FromStr` impl on `Part` and replace with automatically-generated implementations for all
  string-represented constant enums, as well as adding `Display` implementation
- `MapVisualShape::text` and `MapVisual::text` `style` arguments changed to be type
  `Option<MapTextStyle>`
  - The map visual APIs use a different set of options than room visuals, so they need to be a
    different type to express those options
  - Note that all color settings for map visuals are much more restrictive: they only accept colors
    of the form `#FF22DD`, no web-style color names
- Change `Room::find_exit_to` input type from `&JsString` to `RoomName`

### Additions:

- Add `local::serde_position_packed` module, for use with the `with` serde attribute, allowing
  serialized positions to be stored as packed even with human-readable serializers
- New types `MapFontStyle`, `MapFontVariant`, `MapTextStyle` for use in the changes to map visuals
- Newly public functions `RoomName::x_coord` and `RoomName::y_coord` to get the position of a room
  in the world map
- Add `LocalRoomTerrain`, a wrapper in wasm memory of the data in a `RoomTerrain` object.

### Bugfixes:

- Fix incorrect return values in `StructureType::initial_hits` and `ResourceType::boost` constant
  functions
- Use `std::Cow` in custom deserialization process for `StructureType` and `ResourceType` to fix
  failures when deserializing in some cases, like from `serde_json::Value`

0.15.0 (2023-08-03)
===================

### Breaking:

- Move `game::gcl::total_for_level` to `constants::math::control_points_for_gcl` and move 
  `game::gpl::total_for_level` to `constants::math::power_for_gpl`
- Change `constants::math::power_for_gpl` to return `u128` to allow for valid values to be
  calculated for all possible input `u32` values
- Rename `RoomObject::pos` to `RoomObject::js_pos` to avoid confusion with `HasPosition::pos`
  and remove the possibiliy for differing behavior based on whether the trait was imported
- Change `Source::ticks_to_regeneration` and `Mineral::ticks_to_regeneration` return types to
  `Option<u32>`, returning `None` when the timer isn't active instead of panic
- Change `RoomTerrain::new` room name argument type from `&JsString` to `RoomName`

### Additions:

- Add `RoomName::checked_add` to allow a math to be done on the position of the room on the map
  without the potential to panic that the `ops::Add` implementation has
- Add `const` to most functions representing constants, so they can be evaluated during compile
- Fix incorrect value of `constants::extras::FLAG_NAME_MAX_LENGTH` - now 100, previously 60
- Add new extra constant `constants::extras::POWER_CREEP_CARRY_CAPACITY_PER_LEVEL`

### Bugfixes:

- Fix potential for panic in store functions when called with resource types that the store
  isn't currently valid for

0.14.0 (2023-07-03)
===================

### Breaking:

- Revert `RoomTerrain::get_raw_buffer` return type from `Result<Uint8Array, ErrorCode>` back to
  pre-0.13 `Uint8Array`, since it can't error when called with no destination
- Add `sim` feature, which enables the sim-related special case name of `sim` for a room at the
  coordinates of W127N127 (allowing for bots to be built to not include that support)

### Additions:

- Add `?Sized` to `SharedCreepProperties::withdraw` and `transfer` methods to allow dynamic use

0.13.0 (2023-06-27)
===================

### Breaking:

- Remove `ReturnCode` and replace with `Result<(), ErrorCode>`
- Change `RoomPosition` and `Position` methods `look` and `look_for` to return a `Result` instead
  of panicking when used in room not visible in the current tick
- Remove re-exports of `constants::find::Find` and `constants::look::Look` enums and mark them as
  hidden from docs, since they're likely to cause confusion and not generally needed

### Additions:

- Add feature `unsafe-return-conversion` that allows skipping bounds checks on return codes for all
  game functions using integer return codes, risking undefined behavior for values outside the
  expected range
- Add `Position::checked_from_world_coords` allowing an error return instead of a panic for out of
  bounds coordinates
- Add `Position::checked_add` and `Position::checked_add_direction`, alternatives to `Add<_>` which
  can error

### Bugfixes:

- Fix panic when accessing the store of a creep while spawning
- Fix js property mapping for `crate::raw_memory::ForeignSegment::id`

0.12.2 (2023-06-17)
===================

### Additions:

- Add custom implementation of `Debug` for `RoomName` showing the non-packed name
- Add implementation of `From<ExitDirection>` for `Exit`
- Added extra constants for CPU cost per intent, `INTENT_CPU_COST`, and the range of creeps' ranged
  actions, `CREEP_RANGED_ACTION_RANGE`

0.12.1 (2023-06-10)
===================

### Additions:

- Add undocumented `LOOK_REACTORS` season 5 constant

### Bugfixes:

- Add `HasNativeId` and `Transferable` traits to `Reactor`
- Add `Transferable` and `Withdrawable` traits to season 1 and 2 object types

0.12.0 (2023-06-07)
===================

### Breaking:

- Remove `Density::iter_values`, update documentation to indicate `enum_iterator::all` should be
  used instead
- Move `CostMatrixSet` and `HasLocalPosition` traits from `crate::objects` to `crate::traits`

### Additions:

- Add `Creep::claim_reactor` function for season 5
- Add `Density::thorium_amount` function with additional season 5 constants
- Implement `From<Direction>` for `(i32, i32)`, as well as `Add<Direction>` and `Sub<Direction>`
  for `Position`, to support using directions for position math

### Bugfixes:

- Fix `SearchOptions` not mapping to camel-cased field names when converting to js object. Fixes
  `pathfinder::search` not using the specified settings for `max_ops`, `plain_cost`, etc.

### Misc:

- Reorganize `crate::objects` module with some new public sub-modules to group them logically, with
  all of their contents re-exported to maintain compatibiltiy with existing imports

0.11.0 (2023-05-29)
===================

### Breaking:

- Move `crate::inter_shard_memory::InterShardMemory::*` to `crate::inter_shard_memory::*` and move
  `crate::raw_memory::RawMemory::*` to `crate::raw_memory::*` for consistency
- Update `enum-iterator` to 1.4 (`IntoEnumIterator` trait replaced with `Sequence`)
- Remove re-exports of `game::*`, `pathfinder::*`, and `raw_memory::*` to resolve name conflict
  and simplify crate namespace

### Additions:

- Implement `TryFrom<JsString>` for `RawObjectId`
- Implement `FromStr` for `JsObjectId`
- Implement `BODYPARTS_ALL`, `RESOURCES_ALL`, and `COLORS_ALL` constants using `enum-iterator`
- Implement `std::error::Error` for `OutOfBoundsError`, to make it more ergonomic to use with
  other error types
- Added `Default` derivation for `RoomCoordinate` and `RoomXY`
- Added `Thorium` resource, `Reactor` room object, and relevant constants and formulas for season
  5; added `thorium` feature which enables `Reactor` and `Thorium`, and the `seasonal-season-5`
  feature which enables the seasonal constants as well as the `thorium` feature

### Bugfixes:

- Fix `TextStyle::stroke_width` and `TextStyle::background_padding` functions setting incorrect
  values

### Misc:

- Update `serde-wasm-bindgen` to 0.5

0.10.0 (2023-03-13)
===================

### Notably breaking:

- Convert from stdweb to wasm-bindgen as underlying framework. While extensive effort was put into
  keeping the API as similar as possible to prior versions, breaking changes are present
  throughout the API as well as associated representations, such as resource types' serialized
  values; bots still using stdweb APIs should target version 0.9.

0.9.1 (2022-09-08)
==================

- Fixed `Room::serialize_path` and `Room::deserialize_path`, which are static methods and don't
  exist on instances of `Room` objects themselves
- Changed `BuildEvent` to match what's returned by the game, which doesn't match the documentation
- Add the `generate-pixel`, `inter-shard-memory`, and `score` features which enable features not
  present in all server environments
- Add the `mmo` feature which activates the `generate-pixel` and `inter-shard-memory` because
  these interfaces don't exist on private servers
- Add the `seasonal-season-1` feature for season 1, which activates the `score` feature
- Add the `symbols` feature to support season 2
- Add the `seasonal-season-2` feature for season 2, which activates the `symbols` feature

0.9.0 (2021-01-23)
==================

### Notably breaking:

- Change `game::inter_shard_memory::get_local` and `get_remote` to return `Option<String>`,
  accounting for cases where they have not been set (breaking)
- Remove `constants::INVADER_CORE_EXPAND_TIME`, replaced by per-level
  `constants::invader_core_expand_time`
- Add the ability to mark a room as impassable when using the pathfinder. Converts callback
  functions for room cost to use `SingleRoomCostResult` and `MultiRoomCostResult` as appropriate

### Additions:

- Add `AccountPowerCreep::cancel_delete`, which allows cancelling pending deletion of a power
  creep
- Add `StructureType::controller_structures`, which translates the `CONTROLLER_STRUCTURES`
  constant, the number of each structure allowed at a given RCL
- Add missing `constants::MARKET_MAX_ORDERS`, `constants::MARKET_ORDER_LIFE_TIME`, and
  `constants::HARVEST_DEPOSIT_POWER`
- Add the `parse_display::Display` trait to type constant enums which currently implement
  `parse_display::FromStr`, allowing reversal of the conversion from native to constant string
- Add new `IntershardResourceType::CPUUnlock`, `IntershardResourceType::Pixel`, and
  `IntershardResourceType::AccessKey` resources
- Add `game::cpu::generate_pixel` and `constants::PIXEL_CPU_COST`
- Update `constants::PIXEL_CPU_COST` to match game balance change

### Bugfixes:

- Change `game::inter_shard_memory` functions to avoid panicking on private servers where the
  interface doesn't exist
- Update `constants::stronghold_rampart_hits` function with updated values from rebalancing
- Corrected value of `constants::RAMPART_HITS_MAX_RCL5` and `constants::POWER_SPAWN_HITS`
- Fixed `Position::get_direction_to` which always returned the opposite of what it should
- Fixed deserialization of `EVENT_POWER` and `EVENT_TRANSFER` events

0.8.0 (2020-05-30)
==================

### Notably breaking:

- Remove deprecated `game::map::is_room_available`, use new `get_room_status` instead
- Move creep functions which are implemented identically on power creeps to `SharedCreepProperties`
  trait
- Update integer representation of `IntershardResource::SubscriptionToken` to move out of conflict
  with normal resources to allow parsing market orders which might have either
- Update `game::market` functions to be able to work with intershard orders and transactions for
  them, making `RoomName` optional in many cases as it's not used for intershard transactions
- Update field visibility on `game::market` structs used as return values to public, update to
  native types for `ResourceType` and `RoomName` values, and make a number of fields optional for
  compatibility with intershard orders
- Update `game::market::create_order` to use the currently documented object syntax and new
  `MarketResourceType` to specify resource
- Update `game::market::calc_transaction_cost` to work with `RoomName` instead of `&Room` to
  avoid requiring visibility of both rooms
- Change `game::map::describe_exits` to use `RoomName` instead of `String` for values
- Remove `StructurePowerSpawn::power` and `power_capacity` (replaced with `HasStore` functions)
- Remove explicitly implemented `Creep::energy` function which used deprecated `.carry`, now
  using the `energy` implementation from `HasStore`
- Change `RoomObjectProperties::room` to return `Option<Room>` to handle the cases that the base
  game API leaves it undefined: for construction sites and flags in non-visible rooms
- Add `MarketResourceType` enum, which can wrap either a `ResourceType` or `IntershardResourceType`
  and switch to using it for `game::market` endpoints which accept either type
- Change `StructureTerminal::send` to take the destination room name as `RoomName` instead of
  `&str`
- Change `game::market::get_all_orders` to accept an `Option<MarketResourceType>` as a filter
  since this is optimized in the server code
- Change `HasStore::store_free_capacity` to return `i32`, handling potential negative values due
  to expiration of `OPERATE_STORAGE`
- Change `constants::GCL_POW` to f64 from f32 due to slightly incorrect calculations when using
  this from f32 to calculate GCL levels
- Remove explicit `ticks_to_decay` implementations on `StructureContainer` and `Tombstone`, use
  the implementation on `CanDecay` instead
- Change `game::cpu::limit`, `tick_limit`, `bucket`, `shard_limits`, and `set_shard_limits` to
  use `u32` from `f64`

### Additions:

- Add `RoomVisual`, rendering primitives (`Circle`, `Line`, `Rect`, `Poly`, `Text`).
- Add Visual rendering primitive enum for storage and batching.
- Add `MoveToOptions::visualize_path_style`to allow for path visualization of movement system.
- Add `ResourceType::reaction_components` function translating the `REACTIONS` constant
- Add `ResourceType::commodity_recipe` function and `FactoryRecipe` struct translating the
  `COMMODITIES` constant
- Add `ResourceType::boost` function translating the `BOOSTS` constant
- Add `game::map::get_room_status` as interface to new `Game.map.getRoomStatus` function
- Add `StructureLab::reverse_reaction` as interface to new `reverseReaction`
- Add `effects` to room objects, allowing access to the effects applied on room objects which
  are used by both strongholds and power creeps.  New `EffectType` enum returned by this call
  represents the `NaturalEffectType` (for stronghold effects) or `PowerType` (for power creeps)
- Add `game::gpl::level`, `game::gpl::progress` and `game::gpl::progress_total`
- Add `StructureController::is_power_enabled`
- Add `game::power_creeps` access, which returns a special `AccountPowerCreep` reference due
  to the fact that these power creeps may not be spawned on the current shard and allow spawning.
  Use `AccountPowerCreep::get_power_creep` which returns `Option<PowerCreep>` to get the living
  power creep, if spawned on the current shard.
- Add `PowerCreepClass` enum to represent power creep classes, currently only `Operator`
- Add `game::market::get_history` and `game::market::OrderHistoryRecord` exposing new
  `getHistory` API function
- Add `Creep::move_pulled_by` which allows a creep to accept another creep's attempt to `pull`
- Add `SearchOptions::max_cost` to limit the maximum path cost for pathfinder searches
- Add `RoomTerrain::get_raw_buffer_to_array` to load a room's terrain into an existing `[u8; 2500]`
- Add `game::gcl::total_for_level` and `game::gpl::total_for_level` which calculate the total
  lifetime points required for a given level of GCL or GPL
- Add `CostMatrixSet` trait to allow applying costs to a cost matrix generically

### Bugfixes:

- Fix `Room::find_path` function call to underlying javascript
- Fix typo in `Position::create_named_construction_site` and work around screeps bug in
  `Room::create_named_construction_site` by passing x and y instead of position object
- Fix javascript associated object name for `StructureSpawn::spawning`
- Correct swapped return types for `Mineral::density` and `Mineral::mineral_amount` and add
  a workaround for some private servers returning floating point `mineralAmount` values
- Fix typo in `StructureController::reservation` ticks_to_end return value
- Fix reversed conversion of `TOUGH` and `HEAL` parts
- Fix `OwnedStructureProperties::has_owner` to correctly return false for unowned structures
- Work around a case where `map::describe_exits` would panic when a private server returns null
  for an unavailable room
- Change `Source` and `Mineral` `ticks_to_regeneration` functions to return 0, preventing panics
  in cases where the game API returns negative or undefined values
- Fix visibility of struct fields on `MapRoomStatus` and `RoomRouteStep`
- Add `total_available_size` field to `game::cpu::HeapStatistics`
- Add missed `StructureFactory::level` function to determine a factory's level (or `None` if a
  power creep has not yet used `OPERATE_FACTORY`)
- Change `pathfinder::search_many` to return an incomplete result when called with no goals to
  prevent a panic due to unexpected return data from javascript.
- Change `MemoryReference::get` to return a generic error type
- Change `StructureLab::mineral_type` to return `Option<ResourceType>`, avoiding panic when labs
  are empty

### Misc:

- Update `StructureTower::attack` and `heal` to allow targeting power creeps, and update
  `repair` to accept `StructureProperties` matching `Creep::repair`
- Update `Creep::heal` and `ranged_heal` to target anything with the `SharedCreepProperties`
  trait to allow use on power creeps

0.7.0 (2019-10-19)
==================

### Notably breaking:

- Remove `CanStoreEnergy` trait, moving all structures and creeps to `HasStore`, migrating from
  deprecated Screeps API endpoints to new `.store` API (breaking)
    - Remove `Creep::carry_total`, `Creep::carry_types`, `Creep::carry_of`
    - Remove `StructureLab::mineral_amount`, `StructureLab::mineral_capacity`
    - Remove `StructureNuker::ghodium`, `StructureNuker::ghodium_capacity`
    - Change `HasStore::store_capacity` to use new API and now takes `Option<ResourceType>`
    - Add `HasStore::store_free_capacity` and `HasStore::store_used_capacity`, which both
    take `Option<ResourceType>`
- Change return type of `game::rooms::keys` from `Vec<String>` to `Vec<RoomName>`
- Change `HasCooldown` trait to apply to objects with `RoomObjectProperties` instead of
  `StructureProperties` due to addition of `Deposit` objects
- Fix `Position::in_range_to` to be inclusive, to match documentation and JS behavior

### Additions:

- Add new `StructureFactory` and `StructureInvaderCore` structure types
- Add a number of new constants related to factory operation and strongholds
- Add new resource types for factory commodities
- Add `Deposit` objects and related find/look constants
- Add `Ruin` objects and related find/look constants
- Change `Creep.harvest` to work with any harvestable object type; `Deposit`, `Mineral`, and
  `Source`
- Add `ObjectId<T>`, a typed binary object ID, and `RawObjectId`, an untyped binary object ID
  - untyped ids can be converted to typed freely - the type is purely for type inference
  - `RoomObject::id` now returns `ObjectId<Self>`, and an `untyped_id` function is added for
    retrieving an untyped id
  - `game::get_object_typed` now takes `ObjectId<T>`, and `game::get_object_erased` is now generic,
    taking either id type
  - both types are 12 bytes large, and can represent all hex-formatted object IDs from the official
    server, the default private servers backend and the private server with ags131's mongodb mod
- Add support for accessing intershard resource amounts, which currently only includes subscription
  tokens, under `game::resources`.
- Implement `PartialOrd` and `Ord` for `Position`, `RoomName`, `RawObjectId` and `ObjectId`. See
  documentation for ordering specifications.

### Bugfixes:

- Fix typos in JavaScript code for `game::market::get_order` and `Nuke::launch_room_name`
- Fix `Creep::body` crashing if body contained non-boosted parts
- Fix JS syntax error in `Creep::move_to_with_options`
- Remove usage of internal `stdweb` macros, which break with stdweb version changes
- Rename incorrectly spelled `Density::probabilitiy` to `probability`.
- Rename incorrectly spelled `Nuke::lauch_room_name` to `launch_room_name`.
- Rename constants with typo `SPAWN_RENEW_RATION` and  `LINK_LOSS_RATION` to `SPAWN_RENEW_RATIO`
  and `LINK_LOSS_RATIO` respectively

### Misc:

- Add a number of modules grouping constants together, such as `constants::creep` for creep related
  constants or `constants::minerals` for mineral related constants.
- Remove remaining usages of internal `get_from_js!` macro, as it was minimally useful
- Improve syntax and consistency of some internal macros

0.6.0 (2019-08-15)
==================

- Change `LocalRoomName`'s orientation so that north is negative and south is positive.
  This is a breaking change for `LocalRoomName::from_coords`, fields and add/sub implementations
  (#200)
- Rework `LocalRoomPosition` to use a single `u32` as its representation, matching the Screeps'
  server's optimizations, and implement math utilities for it. (#203)
  - `LocalRoomPosition`, now `Position`, implements `Add<(i32, i32)>` for altering world
    coordinates, and has methods like `from_world_coords` and `world_coords`. See [`Position`
    documentation][pos-doc].
- Merge `LocalRoomPosition` and `RoomPosition` into one efficient, local, `Position` type. (#206)
  - Game methods dealing with coordinates now take something which can produce a local
    `Position`, and will only have to upload a single `u32` to JavaScript to call it.
  - `Position` methods dealing with math between positions are now implemented in pure-Rust code
  - An alias `type RoomPosition = Position;` has been added to reflect the JS API
- Rename `LocalRoomName` to `RoomName`, use in APIs, and optimize representation:
  - It is now a 16-bit sized structure, one very efficient to get from a `Position` (#209)
  - It's now used in all API bindings referencing room names (#217)
- Make `StructureSpawn::spawning` an `Option<Spawning>` to reflect reality
- Fix prices returned from `game::market` APIs being integers rather than floats (breaking) (#179)
- Use `OrderType` rather than `String` for `order_type` fields of `TransactionOrder`, `Order` and
  `MyOrder`. (breaking) (#213)
- Work around bug where IntelliJ-Rust didn't understand `screeps::game::*` modules created by a
  macro, even with experimental engine enabled (#197)
- `OwnedStructureProperties`'s `my` method now correctly handles the value being undefined.
  This fixes a panic on checking `my` for unowned controllers (#189)
- `StructurePortal` no longer implements `OwnedStructure` and `Attackable`. (#190)
- Collections provided by `Game` now implement the `hashmap` function to retrieve both keys
  and values at once. (#194)
- Allow accessing fields of the `Bodypart` struct. (#215)
- Implement `Clone` for `Structure`
- Update `screeps::game::market::OrderType` derives to match other constants changed in the
  constants overhaul last update (#213)
- Split [cargo-screeps](https://github.com/rustyscreeps/cargo-screeps/) out into a separate
  repository
- Mark most constant functions as inlinable (#221)
- Misc. documentation improvements.
- Add `game::cpu::halt` binding for [`Game.cpu.halt`](https://docs.screeps.com/api/#Game.halt)
  (#210)
- Add `Creep::pull` binding for [`Creep.pull`](https://docs.screeps.com/api/#Creep.pull) (#210)

[pos-doc]: https://docs.rs/screeps-game-api/0.6.0/screeps/local/struct.Position.html

0.5.0 (2019-07-19)
==================

- `Serialize` and `Deserialize` format changed for most constants and for `LocalRoomPosition`
  (breaking)
  - constants now always serialize as integers (see
    [the module doc](https://docs.rs/screeps-game-api/0.5/screeps/constants/index.html)) (#164)
  - `LocalRoomPosition` will now serialize differently depending on whether the format is readable
    (see the [`LocalRoomPosition`
    doc](https://docs.rs/screeps-game-api/0.5/screeps/struct.LocalRoomPosition.html)) (#171)
- `constants-serde` feature removed in favor of always implementing Serialize and Deserialize for
  constants. (#164)
- Add methods for retrieving coords from `RoomPosition` (#145)
- Implement `Hash` for most constants (#148)
- Add equality checking between strings and `LocalRoomName` (#149)
- Add `raw_memory::drop_segment` (#160)
- Add `inter_shard_memory` (#163)
- Change `MemoryReference::get` and `get_path` to return `Option<T>` (breaking) (#169)
- Add `Display` implementations for `Direction` and `LocalRoomPosition`
- Fix visibility of `LocalRoomNameParseError`, `Bodypart`, `MoveToOptions` and `PortalDestination`
  (#134)
- Fix `Creep::get_active_bodyparts` (#147)
- Fix `CostMatrix` upload method (#150)
- Fix typos in `StructureController` getters (#151)
- Fix `RoomPosition::try_from` conversion (#152)
- Fix errors on successful flag creation (#157)
- Fix `ConstructionSite::structure_type`, `Mineral::mineral_type` and `StructureLab::mineral_type`
  (#173)
- Change many `StructureController` properties to `Option`s to reflect reality (breaking) (#162)
- Migrate to Rust 2018 (#161)
- Improve code reuse and reduce total amount of code (#164, #166, #175)

Thanks to [@ASalvail], [@Dessix] and [@mettke] for contributing to this release.

0.4.0 (2019-02-15)
==================

- Allow simulation room when using local room names (#106)
- Add `Room::look_for_at_xy` (#104)
- Take `&T` in `Room::create_construction_site` and `look_for_at` rather than `T` for
  `T: HasPosition` (breaking) (#105)
- Remove unnecessary allocation from PathFinder and Room.find calls (#112)
- Add `Neg` implementation for `Direction` allowing unary minus to reverse direction (#113)
- Add `JsVec` structure for transparently wrapping typed JavaScript arrays without immediately
  unwrapping them. (#114)
- Switch to `num-derive` for deriving number->enum transitions (previously `enum-primitive`)
  (#116)
- Fix inability to decode `Structure` at all (#119)
- Remove `Sized` bound on `RoomObjectProperties` and add `SizedRoomObject` trait
- Add utility methods for turning `Structure` into `Option<&SomeTrait>` for a variety of traits -
  for instance, see `as_attackable()` (#122)
- Fix constant name `DROPPED_RESOURCES` (previously: `DROPPED_RESOUCES`) (#126)
- Remove `Attackable` implementation for `Structure` (`StructureController` and `StructurePortal`
  are _not_ attackable). Use `as_attackable()` instead. (#129)

0.3.0 (2018-11-12)
==================

- Implement Serialize and Deserialize for constants under `constants-serde` feature flag (#69)
- Fix Tombstone::get_position_at (#70)
- Remove duplicate Tombstone::id property (#70)
- Implement September 2018 update (#71)
  - Replace old terrain APIs with game::map::get_room_terrain
  - Add event API
  - Add support for named construction sites on RoomPosition
- Add support for `findRoute` and `findExit` (#76)
- Add full support for Creep::body (#78)
- Add support for `moveTo` options (#80)
- Replace all usages of u8 for in-room positions with u32 (#80)
- Remove duplicate properties on StructureSpawn (#82)
- Use u32 for everything non-negative (#83)
- Add StructureSpawn::spawning property (#86)
- Replace use-once builder with more standard options struct in `StructureSpawn::spawn` (#86)
- Add look, lookAt, lookAtArea support (#87)
- Add MemoryReference::get<T>, get_path<T> for retrieving arbitrary types from memory (#96)
- Fix unconditional error in `find_in_range`, `find_closest_by_range` bindings (#99)
- Unbreak `mem_get!` macro broken in last release (#102)
- Rename `Density` enum variants to remove `Density` prefix (#101)
- Make various API functions take `Copy` types by value rather than by reference (#101)

0.2.0 (2018-08-28)
==================

- Remove `Reservation` and `Sign` structures from prelude (breaking)
- Add `Eq`, `PartialEq` implementations to everything which has an ID, as well as `Room`
- Implement `RawMemory` `get` and `set` support
- Create CONTRIBUTING document
- Change all `TryFrom` implementations to perform type checks rather than making assumptions.
- Implement `ReferenceType` for all reference wrapping structures
- Add traits `ExpectedTypeInto` and `ExpectedTypeFrom` for converting `stdweb::Value`s to typed
  structures with or without type checks depending on `screeps-game-api`'s feature flags
  - Add feature `check-all-casts` which is off by default and when enabled adds type checks to
    `ExpectedTypeFrom` conversions
- Create enum `Density` rather than having `DENSITY_LOW`, `_MODERATE`, `_HIGH` and `_ULTRA`
  constants (breaking)
- Implement support for APIs:
  - `RawMemory.get`, `set`
  - `Game.map.findExit`
  - `Game.market.*`
  - `Game.notify`
  - `Creep.drop`, `owner.username`, `moveByPath`, `notifyWhenAttacked`, `saying`
  - `Flag.remove`, `setColor`, `setPosition`
  - `Mineral.density`, `mineralAmount`, `mineralType`, `ticksToRegeneration`
  - `Nuke.launchRoomName`, `timeToLand`
  - `Room.serializePath`, `deserializePath`, `findExitTo`, `getPositionAt`, `findPath`, `memory`,
    `findPathTo`
  - `RoomPosition.isEqualTo` accepting `x`/`y` parameters
  - `StructureController.activateSafeMode`, `unclaim`
  - `StructureKeeperLair.ticksToSpawn`
  - `StructureLab.mineralAmount`, `mineralCapacity`, `mineralType`, `boostCreep`, `runReaction`
  - `StructureLink.transferEnergy`
  - `StructureNuker.ghodium`, `ghodiumCapacity`, `launchNuke`
  - `StructureObserver.observeRoom`
  - `StructurePortal.destination`
  - `StructurePowerBank.power`
  - `StructurePowerSpawn.processPower`
  - `StructureRampart.setPublic`
  - `StructureSpawn.memory`, `recycleCreep`, `renewCreep`
  - `StructureTerminal.send`
  - `StructureTower.attack`, `heal`, `repair`
- Rename `get_object` to `get_object_erased` and add support for `get_object_typed` directly
  fetching a room object knowing its type (breaking)
- Add support for fetching and setting memory keys by path using lodash
- Rename memory `num` method to `f64` and `int` to `i32`
- Add `mem_get!` and `mem_set!` macros for compile-time memory path interpretation.
- Rename `owner` methods to `owner_name`
- Fix `Creep.signController` type signature
- Fix `Room.findClosestByRange` type signature
- Add trait `HasId` representing things which have ids (all `RoomObject`s except `Flag`s)
  - Move `id` methods from individual structures to this structure (breaking)
- Add traits `CanStoreEnergy`, `HasCooldown`, `CanDecay`, `Withdrawable`, `Attackable`
- Move `hits` and `hitsMax` bindings from `StructureProperties` trait to `Attackable` trait
  (breaking)
- Re-export `stdweb::unstable::{TryFrom, TryInto}` in `traits` module

Thanks to [@ASalvail] for planning and implementing the vast majority of this release!

0.1.1 (2018-09-04)
==================

- Add support for StructureController (thanks [@ASalvail]!)

0.1.0 (2018-08-16)
==================

- Work around Rust bug appearing in latest nightly
  (https://github.com/rust-lang/rust/issues/53140)
- Bump to 0.1.0 so we can create bugfix releases such as this one which
  do not contain breaking changes.

0.0.10 (2018-06-16)
===================

- Add ReturnCode::as_result
- Add `MemoryReference::from_reference_unchecked`
- Support ConstructionSite properties
- Support `Room::look_for_at_area`
- Make 'Structure' type an enum rather than opaque reference
- Add `LocalRoomPosition::remote` shorthand

0.0.9 (2018-03-20)
==================

- Add string access API to Memory

0.0.8 (2018-03-12)
==================

- Implement PathFinder interface
- Implement Serialize/Deserialize for LocalRoomPosition
- Add LocalRoomName structure
- Change time to unsigned integers
- Add `StructureSpawn::is_spawning`
- Add `RoomPosition::local` utility
- Fix HeapStatistics field names
- Fix console.error hack (see cargo-screeps changes)

0.0.7 (2018-03-06)
==================

- Hotfix for code so it compiles

0.0.6 (2018-03-06)
==================

- Add tombstone support
- Add reaction_time constant calculation method
- Support `v8_getheapstatistics`
- Replace some `get_from_js!()` calls with manual functions
- Switch from using .0 to .as_ref() to refer to inner Reference of wrapping objects
- Add support for creating construction sites and flags
- Add Source support
- Add support for spawning creeps
- Make methods of traits have default impls, instead of using macros to implement them

0.0.5 (2018-03-01)
==================

- Fix crates.io distribution to include javascript files

0.0.4 (2018-03-01)
==================

- Fix crates.io distribution to include Web.toml

0.0.3 (2018-03-01)
==================

- No notable changes

0.0.2 (2018-03-01)
==================

- No notable changes

0.0.1 (2018-02-28)
==================

- Initial release


[@ASalvail]: https://github.com/ASalvail
[@Dessix]: https://github.com/Dessix
[@mettke]: https://github.com/mettke
