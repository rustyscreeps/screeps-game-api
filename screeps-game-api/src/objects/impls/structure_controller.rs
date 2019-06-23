use stdweb::Value;

use {constants::ReturnCode, objects::StructureController};

simple_accessors! {
    StructureController;
    (level -> level -> u32),
    (progress -> progress -> u32),
    (progress_total -> progressTotal -> u32),
    (safe_mode -> safeMode -> u32),
    (safe_mode_available -> safeModeAvailable -> u32),
    (safe_mode_cooldown -> safeModeCooldown -> u32),
    (ticks_to_downgrade -> ticksToDowngrade -> u32),
    (upgrade_blocked -> upgradeBlocked -> u32)
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
                ticks_to_end: js_unwrap!(@{&r}.ticks_to_end),
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
