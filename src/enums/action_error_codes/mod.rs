//! This module contains error code enums for individual API actions.

mod constructionsite_error_codes;
mod creep_error_codes;
mod flag_error_codes;
#[cfg(feature = "mmo")]
mod game_cpu_error_codes;
mod game_map_error_codes;
mod game_market_error_codes;
mod powercreep_error_codes;
mod room_error_codes;
mod roomposition_error_codes;
mod sharedcreep_error_codes;
mod spawning_error_codes;
mod structure_error_codes;
mod structurecontroller_error_codes;
mod structurefactory_error_codes;
mod structurelab_error_codes;
mod structurelink_error_codes;
mod structurenuker_error_codes;
mod structureobserver_error_codes;
mod structurepowerspawn_error_codes;
mod structurerampart_error_codes;
mod structurespawn_error_codes;
mod structureterminal_error_codes;
mod structuretower_error_codes;

pub mod construction_site {
    pub use super::constructionsite_error_codes::ConstructionSiteRemoveErrorCode;
}

pub mod creep {
    pub use super::creep_error_codes::{
        AttackControllerErrorCode, BuildErrorCode, ClaimControllerErrorCode, CreepAttackErrorCode,
        CreepCancelOrderErrorCode, CreepHealErrorCode, CreepMoveByPathErrorCode,
        CreepMoveDirectionErrorCode, CreepMovePulledByErrorCode, CreepMoveToErrorCode,
        CreepRepairErrorCode, DismantleErrorCode, GenerateSafeModeErrorCode, HarvestErrorCode,
        PullErrorCode, RangedAttackErrorCode, RangedHealErrorCode, RangedMassAttackErrorCode,
        ReserveControllerErrorCode, SignControllerErrorCode, UpgradeControllerErrorCode,
    };
}

pub mod flag {
    pub use super::flag_error_codes::{
        FlagRemoveErrorCode, SetColorErrorCode, SetPositionErrorCode,
    };
}

pub mod game {
    use super::{game_map_error_codes, game_market_error_codes};

    #[cfg(feature = "mmo")]
    use super::game_cpu_error_codes;

    #[cfg(feature = "mmo")]
    pub mod cpu {
        pub use super::game_cpu_error_codes::{
            GeneratePixelErrorCode, SetShardLimitsErrorCode, UnlockErrorCode,
        };
    }

    pub mod map {
        pub use super::game_map_error_codes::{FindExitErrorCode, FindRouteErrorCode};
    }

    pub mod market {
        pub use super::game_market_error_codes::{
            ChangeOrderPriceErrorCode, CreateOrderErrorCode, DealErrorCode, ExtendOrderErrorCode,
            MarketCancelOrderErrorCode,
        };
    }
}

pub mod powercreep {
    pub use super::powercreep_error_codes::{
        DeleteErrorCode, EnableRoomErrorCode, PowerCreepCancelOrderErrorCode,
        PowerCreepCreateErrorCode, PowerCreepMoveByPathErrorCode, PowerCreepMoveDirectionErrorCode,
        PowerCreepMoveToErrorCode, RenameErrorCode, RenewErrorCode, SpawnErrorCode,
        UpgradeErrorCode, UsePowerErrorCode,
    };
}

pub mod room {
    pub use super::room_error_codes::{
        FindExitToErrorCode, RoomCreateConstructionSiteErrorCode, RoomCreateFlagErrorCode,
    };
}

pub mod room_position {
    pub use super::roomposition_error_codes::{
        RoomPositionCreateConstructionSiteErrorCode, RoomPositionCreateFlagErrorCode,
    };
}

pub mod shared {
    pub use super::sharedcreep_error_codes::{
        DropErrorCode, NotifyWhenAttackedErrorCode, PickupErrorCode, SayErrorCode,
        SuicideErrorCode, TransferErrorCode, WithdrawErrorCode,
    };
}

pub mod spawning {
    pub use super::spawning_error_codes::{CancelErrorCode, SetDirectionsErrorCode};
}

pub mod structure {
    pub use super::structure_error_codes::{
        DestroyErrorCode, StructureNotifyWhenAttackedErrorCode,
    };
}

pub mod structure_controller {
    pub use super::structurecontroller_error_codes::{ActivateSafeModeErrorCode, UnclaimErrorCode};
}

pub mod structure_factory {
    pub use super::structurefactory_error_codes::ProduceErrorCode;
}

pub mod structure_lab {
    pub use super::structurelab_error_codes::{
        BoostCreepErrorCode, ReverseReactionErrorCode, RunReactionErrorCode, UnboostCreepErrorCode,
    };
}

pub mod structure_link {
    pub use super::structurelink_error_codes::TransferEnergyErrorCode;
}

pub mod structure_nuker {
    pub use super::structurenuker_error_codes::LaunchNukeErrorCode;
}

pub mod structure_observer {
    pub use super::structureobserver_error_codes::ObserveRoomErrorCode;
}

pub mod structure_powerspawn {
    pub use super::structurepowerspawn_error_codes::ProcessPowerErrorCode;
}

pub mod structure_rampart {
    pub use super::structurerampart_error_codes::SetPublicErrorCode;
}

pub mod structure_spawn {
    pub use super::structurespawn_error_codes::{
        RecycleCreepErrorCode, RenewCreepErrorCode, SpawnCreepErrorCode,
    };
}

pub mod structure_terminal {
    pub use super::structureterminal_error_codes::SendErrorCode;
}

pub mod structure_tower {
    pub use super::structuretower_error_codes::{
        TowerAttackErrorCode, TowerHealErrorCode, TowerRepairErrorCode,
    };
}

pub use self::{
    construction_site::*, creep::*, flag::*, game::*, powercreep::*, room::*, room_position::*,
    shared::*, spawning::*, structure::*, structure_controller::*, structure_factory::*,
    structure_lab::*, structure_link::*, structure_nuker::*, structure_observer::*,
    structure_powerspawn::*, structure_rampart::*, structure_spawn::*, structure_terminal::*,
    structure_tower::*,
};
