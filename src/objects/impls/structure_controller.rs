use stdweb::Value;

use crate::{constants::ReturnCode, objects::StructureController};

simple_accessors! {
    impl StructureController {
        pub fn is_power_enabled() -> bool = isPowerEnabled;
        pub fn level() -> u32 = level;
        pub fn progress() -> Option<u32> = progress;
        pub fn progress_total() -> Option<u32> = progressTotal;
        pub fn safe_mode() -> Option<u32> = safeMode;
        pub fn safe_mode_available() -> u32 = safeModeAvailable;
        pub fn safe_mode_cooldown() -> Option<u32> = safeModeCooldown;
        pub fn ticks_to_downgrade() -> u32 = ticksToDowngrade;
        pub fn upgrade_blocked() -> Option<u32> = upgradeBlocked;
    }
}

#[derive(Debug)]
pub struct Reservation {
    pub username: String,
    pub ticks_to_end: u32,
}

#[derive(Debug)]
pub struct Sign {
    pub username: String,
    pub text: String,
    pub time: u32,
    pub datetime: String, // todo: use real date type
}

impl StructureController {
    pub fn activate_safe_mode(&self) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.activateSafeMode()}
    }

    pub fn reservation(&self) -> Option<Reservation> {
        if let Value::Reference(r) = js!(return @{self.as_ref()}.reservation;) {
            Some(Reservation {
                username: js_unwrap!(@{&r}.username),
                ticks_to_end: js_unwrap!(@{&r}.ticksToEnd),
            })
        } else {
            None
        }
    }

    pub fn sign(&self) -> Option<Sign> {
        if let Value::Reference(r) = js!(return @{self.as_ref()}.sign;) {
            Some(Sign {
                username: js_unwrap!(@{&r}.username),
                text: js_unwrap!(@{&r}.text),
                time: js_unwrap!(@{&r}.time),
                datetime: js_unwrap!(@{&r}.datetime.toString()),
            })
        } else {
            None
        }
    }

    pub fn unclaim(&self) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.unclaim()}
    }
}
