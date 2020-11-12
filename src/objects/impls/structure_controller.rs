use crate::{
    objects::{OwnedStructure, RoomObject, Structure},
};
use js_sys::{Date, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureController`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureController;

    /// Whether power is enabled in the room, allowing power creeps to use powers.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.isPowerEnabled)
    #[wasm_bindgen(method, getter = isPowerEnabled)]
    pub fn is_power_enabled(this: &StructureController) -> bool;

    /// The current room control level (RCL) of the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &StructureController) -> u8;

    /// The progress toward upgrading the controller to the next level
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.progress)
    #[wasm_bindgen(method, getter)]
    pub fn progress(this: &StructureController) -> u32;

    /// The total [`StructureController::progress`] needed to upgrade the controller to the next level.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.progressTotal)
    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(this: &StructureController) -> u32;

    /// Information about the reservation of this controller, if it is currently reserved.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.reservation)
    #[wasm_bindgen(method, getter)]
    pub fn reservation(this: &StructureController) -> Option<Reservation>;

    /// The number of ticks remaining in safe mode, or 0 if safe mode isn't currently active.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.safeMode)
    #[wasm_bindgen(method, getter = safeMode)]
    pub fn safe_mode(this: &StructureController) -> u32;

    /// The number of of available safe mode activations, which can be increased by using [`Creep::generate_safe_mode`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.safeModeAvailable)
    ///
    /// [`Creep::generate_safe_mode`]: crate::objects::Creep::generate_safe_mode
    #[wasm_bindgen(method, getter = safeModeAvailable)]
    pub fn safe_mode_available(this: &StructureController) -> u32;

    /// The cooldown remaining until safe mode can be activated again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.safeModeCooldown)
    #[wasm_bindgen(method, getter = safeModeCooldown)]
    pub fn safe_mode_cooldown(this: &StructureController) -> u32;

    /// Information about the sign on this controller, if it has been signed by [`Creep::sign_controller`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.sign)
    ///
    /// [`Creep::sign_controller`]: crate::objects::Creep::sign_controller
    #[wasm_bindgen(method, getter)]
    pub fn sign(this: &StructureController) -> Option<Sign>;

    /// The number of ticks until the level of the controller will be decremented due to a lack of [`Creep::upgrade_controller`] activity.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.ticksToDowngrade)
    #[wasm_bindgen(method, getter = ticksToDowngrade)]
    pub fn ticks_to_downgrade(this: &StructureController) -> u32;

    /// The number of ticks until the controller can be upgraded, or have safe mode activated, due to [`Creep::attack_controller`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.upgradeBlocked)
    ///
    /// [`Creep::attack_controller`]: crate::objects::Creep::attack_controller
    #[wasm_bindgen(method, getter = upgradeBlocked)]
    pub fn upgrade_blocked(this: &StructureController) -> u32;

    /// Activate safe mode for the room, preventing hostile creep actions in the room for 20,000 ticks
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.activateSafeMode)
    #[wasm_bindgen(method, js_name = activateSafeMode)]
    pub fn activate_safe_mode(this: &StructureController) -> i8;

    /// Relinquish ownership of the controller and its room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.unclaim)
    #[wasm_bindgen(method)]
    pub fn unclaim(this: &StructureController) -> i8;
}


#[wasm_bindgen]
extern "C" {
    /// Object with info on who has reserved this [`StructureController`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.reservation)
    #[wasm_bindgen]
    pub type Reservation;

    /// The name of the player that has reserved this controller as a [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Reservation) -> JsString;

    /// The number of ticks until the reservation expires.
    #[wasm_bindgen(method, getter = ticksToEnd)]
    pub fn ticks_to_end(this: &Reservation) -> u32;
}


#[wasm_bindgen]
extern "C" {
    /// Object with info on the sign on a [`StructureController`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.sign)
    #[wasm_bindgen]
    pub type Sign;

    /// The name of the player that has reserved this controller as a [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Sign) -> JsString;

    /// The text of the sign on this controller as a [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn text(this: &Sign) -> JsString;

    /// The tick when this sign was written.
    #[wasm_bindgen(method, getter)]
    pub fn time(this: &Sign) -> u32;

    /// The timestamp of when this sign was written.
    #[wasm_bindgen(method, getter)]
    pub fn datetime(this: &Sign) -> Date;
}


// use stdweb::Value;

// use crate::{constants::ReturnCode, objects::StructureController};

// simple_accessors! {
//     impl StructureController {
//         pub fn is_power_enabled() -> bool = isPowerEnabled;
//         pub fn level() -> u32 = level;
//         pub fn progress() -> Option<u32> = progress;
//         pub fn progress_total() -> Option<u32> = progressTotal;
//         pub fn safe_mode() -> Option<u32> = safeMode;
//         pub fn safe_mode_available() -> u32 = safeModeAvailable;
//         pub fn safe_mode_cooldown() -> Option<u32> = safeModeCooldown;
//         pub fn ticks_to_downgrade() -> u32 = ticksToDowngrade;
//         pub fn upgrade_blocked() -> Option<u32> = upgradeBlocked;
//     }
// }

// #[derive(Debug)]
// pub struct Reservation {
//     pub username: String,
//     pub ticks_to_end: u32,
// }

// #[derive(Debug)]
// pub struct Sign {
//     pub username: String,
//     pub text: String,
//     pub time: u32,
//     pub datetime: String, // todo: use real date type
// }

// impl StructureController {
//     pub fn activate_safe_mode(&self) -> ReturnCode {
//         js_unwrap! {@{self.as_ref()}.activateSafeMode()}
//     }

//     pub fn reservation(&self) -> Option<Reservation> {
//         if let Value::Reference(r) = js!(return @{self.as_ref()}.reservation;) {
//             Some(Reservation {
//                 username: js_unwrap!(@{&r}.username),
//                 ticks_to_end: js_unwrap!(@{&r}.ticksToEnd),
//             })
//         } else {
//             None
//         }
//     }

//     pub fn sign(&self) -> Option<Sign> {
//         if let Value::Reference(r) = js!(return @{self.as_ref()}.sign;) {
//             Some(Sign {
//                 username: js_unwrap!(@{&r}.username),
//                 text: js_unwrap!(@{&r}.text),
//                 time: js_unwrap!(@{&r}.time),
//                 datetime: js_unwrap!(@{&r}.datetime.toString()),
//             })
//         } else {
//             None
//         }
//     }

//     pub fn unclaim(&self) -> ReturnCode {
//         js_unwrap! {@{self.as_ref()}.unclaim()}
//     }
// }
