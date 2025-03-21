use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by
/// [ConstructionSite::remove](crate::ConstructionSite::remove).
///
/// Screeps API Docs: [ConstructionSite.remove](https://docs.screeps.com/api/#ConstructionSite.remove).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/construction-sites.js#L50)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ConstructionSiteRemoveErrorCode {
    NotOwner = -1,
}

impl FromReturnCode for ConstructionSiteRemoveErrorCode {
    type Error = Self;

    fn result_from_i8(val: i8) -> Result<(), Self::Error> {
        let maybe_result = Self::try_result_from_i8(val);
        #[cfg(feature = "unsafe-return-conversion")]
        unsafe {
            maybe_result.unwrap_unchecked()
        }
        #[cfg(not(feature = "unsafe-return-conversion"))]
        maybe_result.unwrap()
    }

    fn try_result_from_i8(val: i8) -> Option<Result<(), Self::Error>> {
        match val {
            0 => Some(Ok(())),
            -1 => Some(Err(ConstructionSiteRemoveErrorCode::NotOwner)),
            _ => None,
        }
    }
}

impl fmt::Display for ConstructionSiteRemoveErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ConstructionSiteRemoveErrorCode::NotOwner => {
                "you are not the owner of this construction site, and it's not in your room"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for ConstructionSiteRemoveErrorCode {}
