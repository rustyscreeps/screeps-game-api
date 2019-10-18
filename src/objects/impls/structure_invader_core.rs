use crate::objects::StructureInvaderCore;

simple_accessors! {
    impl StructureInvaderCore {
        pub fn level() -> u32 = level;
        pub fn ticks_to_deploy() -> Option<u32> = ticksToDeploy;
    }
}
