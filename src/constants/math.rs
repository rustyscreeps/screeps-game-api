//! Functions allowing calculation of the resulting values of formulas used by
//! game mechanics related to constant values.

use crate::constants::*;

/// Provides the total number of control points needed to achieve a given Global
/// Control Level
///
/// Calculates the total number of control points needed to achieve a given
/// Global Control Level. The game's API only exposes current level plus
/// progress toward the next level; this allows you to see much many points
/// you've spent to achieve your current level
///
/// [Code reference](https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L117)
pub fn control_points_for_gcl(level: u32) -> f64 {
    ((level - 1) as f64).powf(GCL_POW) * GCL_MULTIPLY as f64
}

/// Provides the total number of processed power needed to achieve a given
/// Global Power Level
///
/// Calculates the total number of power that need to be processed to achieve a
/// given Global Power Level. The game's API only exposes current level plus
/// progress toward the next level; this allows you to see how much you
/// processed to achieve your current level
///
/// [Code reference](https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L120)
pub const fn power_for_gpl(level: u32) -> u128 {
    (level as u128).pow(POWER_LEVEL_POW) * POWER_LEVEL_MULTIPLY as u128
}

/// Provides the amount of damage done by tower attacks at a given range, after
/// accounting for reduction from [`TOWER_FALLOFF`].
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/towers/attack.js#L33-L38)
pub fn tower_attack_power_at_range(mut range: u8) -> u32 {
    let mut amount = TOWER_POWER_ATTACK as f64;
    if range > TOWER_FALLOFF_RANGE {
        range = TOWER_FALLOFF_RANGE;
    }
    amount -= amount * TOWER_FALLOFF * range.saturating_sub(TOWER_OPTIMAL_RANGE) as f64
        / (TOWER_FALLOFF_RANGE - TOWER_OPTIMAL_RANGE) as f64;
    amount as u32
}

/// Provides the amount of damage healed by tower healing at a given range,
/// after accounting for reduction from [`TOWER_FALLOFF`].
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/towers/heal.js#L24-L30)
pub fn tower_heal_power_at_range(mut range: u8) -> u32 {
    let mut amount = TOWER_POWER_HEAL as f64;
    if range > TOWER_FALLOFF_RANGE {
        range = TOWER_FALLOFF_RANGE;
    }
    amount -= amount * TOWER_FALLOFF * range.saturating_sub(TOWER_OPTIMAL_RANGE) as f64
        / (TOWER_FALLOFF_RANGE - TOWER_OPTIMAL_RANGE) as f64;
    amount as u32
}

/// Provides the amount of damage repaired by towers at a given range, after
/// accounting for reduction from [`TOWER_FALLOFF`].
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/towers/repair.js#L21-L26)
pub fn tower_repair_power_at_range(mut range: u8) -> u32 {
    let mut amount = TOWER_POWER_REPAIR as f64;
    if range > TOWER_FALLOFF_RANGE {
        range = TOWER_FALLOFF_RANGE;
    }
    amount -= amount * TOWER_FALLOFF * range.saturating_sub(TOWER_OPTIMAL_RANGE) as f64
        / (TOWER_FALLOFF_RANGE - TOWER_OPTIMAL_RANGE) as f64;
    amount as u32
}

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn gcl_formula() {
        // the sanity of these values has been validated up to GCL 33
        // on the MMO game server
        assert_approx_eq!(control_points_for_gcl(1), 0.);
        assert_approx_eq!(control_points_for_gcl(2), 1000000.);
        assert_approx_eq!(control_points_for_gcl(3), 5278031.643091577);
        assert_approx_eq!(control_points_for_gcl(4), 13966610.165238237);
        assert_approx_eq!(control_points_for_gcl(5), 27857618.025475968);
        assert_approx_eq!(control_points_for_gcl(6), 47591348.46789695);
        assert_approx_eq!(control_points_for_gcl(7), 73716210.39885189);
        assert_approx_eq!(control_points_for_gcl(8), 106717414.7996562);
        assert_approx_eq!(control_points_for_gcl(9), 147033389.43962047);
        assert_approx_eq!(control_points_for_gcl(10), 195066199.50773603);
        assert_approx_eq!(control_points_for_gcl(11), 251188643.15095797);
        assert_approx_eq!(control_points_for_gcl(12), 315749334.8687436);
        assert_approx_eq!(control_points_for_gcl(13), 389076491.09393656);
        assert_approx_eq!(control_points_for_gcl(14), 471480836.66525537);
        assert_approx_eq!(control_points_for_gcl(15), 563257892.1815147);
        assert_approx_eq!(control_points_for_gcl(16), 664689811.2891247);
        assert_approx_eq!(control_points_for_gcl(17), 776046882.0533236);
        assert_approx_eq!(control_points_for_gcl(18), 897588771.9617443);
        assert_approx_eq!(control_points_for_gcl(19), 1029565573.4994452);
        assert_approx_eq!(control_points_for_gcl(20), 1172218691.9999762);
        assert_approx_eq!(control_points_for_gcl(25), 2053558031.5768352);
        assert_approx_eq!(control_points_for_gcl(30), 3234113036.1951885);
        assert_approx_eq!(control_points_for_gcl(31), 3508253856.824569);
        assert_approx_eq!(control_points_for_gcl(32), 3795491867.4194345);
        assert_approx_eq!(control_points_for_gcl(33), 4095999999.9999986);
        assert_approx_eq!(control_points_for_gcl(34), 4409947870.045006);
        assert_approx_eq!(control_points_for_gcl(35), 4737501940.897796);
        assert_approx_eq!(control_points_for_gcl(40), 6584989046.083984);
        assert_approx_eq!(control_points_for_gcl(45), 8796024362.57156);
        assert_approx_eq!(control_points_for_gcl(50), 11388606621.52188);
        assert_approx_eq!(control_points_for_gcl(100), 61592022749.941284);
        assert_approx_eq!(control_points_for_gcl(1000), 15810921110646.998);
        assert_approx_eq!(control_points_for_gcl(u32::MAX), 1.3155388150906982e29);
    }

    #[test]
    #[should_panic]
    fn bad_gcl_formula_input() {
        // players cannot be GCL 0, and subtracting 1 (as the formula does)
        // overflows the u32 - this should panic.
        control_points_for_gcl(0);
    }

    #[test]
    fn gpl_formula() {
        // the sanity of these values has been validated up to GCL 33
        // on the MMO game server
        assert_eq!(power_for_gpl(0), 0);
        assert_eq!(power_for_gpl(1), 1_000);
        assert_eq!(power_for_gpl(2), 4_000);
        assert_eq!(power_for_gpl(3), 9_000);
        assert_eq!(power_for_gpl(4), 16_000);
        assert_eq!(power_for_gpl(5), 25_000);
        assert_eq!(power_for_gpl(6), 36_000);
        assert_eq!(power_for_gpl(7), 49_000);
        assert_eq!(power_for_gpl(8), 64_000);
        assert_eq!(power_for_gpl(9), 81_000);
        assert_eq!(power_for_gpl(10), 100_000);
        assert_eq!(power_for_gpl(50), 2_500_000);
        assert_eq!(power_for_gpl(100), 10_000_000);
        assert_eq!(power_for_gpl(1_000), 1_000_000_000);
        assert_eq!(power_for_gpl(5_000), 25_000_000_000);
        assert_eq!(power_for_gpl(10_000), 100_000_000_000);
        assert_eq!(power_for_gpl(50_000), 2_500_000_000_000);
        assert_eq!(power_for_gpl(100_000), 10_000_000_000_000);
        assert_eq!(power_for_gpl(1_000_000), 1_000_000_000_000_000);
        assert_eq!(power_for_gpl(5_000_000), 25_000_000_000_000_000);
        assert_eq!(power_for_gpl(10_000_000), 100_000_000_000_000_000);
        assert_eq!(power_for_gpl(100_000_000), 10_000_000_000_000_000_000);
        // beyond this value the return overflows a u64
        assert_eq!(power_for_gpl(135_818_791), 18_446_743_988_701_681_000);
        // must be u128 return to fit this one!
        assert_eq!(power_for_gpl(135_818_792), 18_446_744_260_339_264_000);
        assert_eq!(power_for_gpl(1_000_000_000), 1_000_000_000_000_000_000_000);
        assert_eq!(power_for_gpl(4_000_000_000), 16_000_000_000_000_000_000_000);
        assert_eq!(power_for_gpl(u32::MAX), 18_446_744_065_119_617_025_000);
    }

    #[test]
    fn tower_attack_power_formula() {
        // at optimal range, the damage should be 100% of the attack power
        assert_eq!(
            tower_attack_power_at_range(TOWER_OPTIMAL_RANGE),
            TOWER_POWER_ATTACK
        );
        // at full falloff range, we should have 1 - TOWER_FALLOFF (25%) damage
        assert_eq!(
            tower_attack_power_at_range(TOWER_FALLOFF_RANGE),
            (TOWER_POWER_ATTACK as f64 * (1. - TOWER_FALLOFF)) as u32
        );
        // test values generated in js using the engine's code
        assert_eq!(tower_attack_power_at_range(5), 600);
        assert_eq!(tower_attack_power_at_range(6), 570);
        assert_eq!(tower_attack_power_at_range(7), 540);
        assert_eq!(tower_attack_power_at_range(8), 510);
        assert_eq!(tower_attack_power_at_range(9), 480);
        assert_eq!(tower_attack_power_at_range(10), 450);
        assert_eq!(tower_attack_power_at_range(11), 420);
        assert_eq!(tower_attack_power_at_range(12), 390);
        assert_eq!(tower_attack_power_at_range(13), 360);
        assert_eq!(tower_attack_power_at_range(14), 330);
        assert_eq!(tower_attack_power_at_range(15), 300);
        assert_eq!(tower_attack_power_at_range(16), 270);
        assert_eq!(tower_attack_power_at_range(17), 240);
        assert_eq!(tower_attack_power_at_range(18), 210);
        assert_eq!(tower_attack_power_at_range(19), 180);
        assert_eq!(tower_attack_power_at_range(20), 150);
        // falloff range stops at 20, make sure beyond that stays at 150
        assert_eq!(tower_attack_power_at_range(25), 150);
        // math should work even at range 0
        assert_eq!(tower_attack_power_at_range(0), 600);
    }

    #[test]
    fn tower_heal_power_formula() {
        // at optimal range, the damage should be 100% of the heal power
        assert_eq!(
            tower_heal_power_at_range(TOWER_OPTIMAL_RANGE),
            TOWER_POWER_HEAL
        );
        // at full falloff range, we should have 1 - TOWER_FALLOFF (25%) hits
        assert_eq!(
            tower_heal_power_at_range(TOWER_FALLOFF_RANGE),
            (TOWER_POWER_HEAL as f64 * (1. - TOWER_FALLOFF)) as u32
        );
        // test values generated in js using the engine's code
        assert_eq!(tower_heal_power_at_range(5), 400);
        assert_eq!(tower_heal_power_at_range(6), 380);
        assert_eq!(tower_heal_power_at_range(7), 360);
        assert_eq!(tower_heal_power_at_range(8), 340);
        assert_eq!(tower_heal_power_at_range(9), 320);
        assert_eq!(tower_heal_power_at_range(10), 300);
        assert_eq!(tower_heal_power_at_range(11), 280);
        assert_eq!(tower_heal_power_at_range(12), 260);
        assert_eq!(tower_heal_power_at_range(13), 240);
        assert_eq!(tower_heal_power_at_range(14), 220);
        assert_eq!(tower_heal_power_at_range(15), 200);
        assert_eq!(tower_heal_power_at_range(16), 180);
        assert_eq!(tower_heal_power_at_range(17), 160);
        assert_eq!(tower_heal_power_at_range(18), 140);
        assert_eq!(tower_heal_power_at_range(19), 120);
        assert_eq!(tower_heal_power_at_range(20), 100);
        // falloff range stops at 20, make sure beyond that stays at 100
        assert_eq!(tower_heal_power_at_range(25), 100);
        // math should work even at range 0
        assert_eq!(tower_heal_power_at_range(0), 400);
    }

    #[test]
    fn tower_repair_power_formula() {
        // at optimal range, the damage should be 100% of the repair power
        assert_eq!(
            tower_repair_power_at_range(TOWER_OPTIMAL_RANGE),
            TOWER_POWER_REPAIR
        );
        // at full falloff range, we should have 1 - TOWER_FALLOFF (25%) repair
        assert_eq!(
            tower_repair_power_at_range(TOWER_FALLOFF_RANGE),
            (TOWER_POWER_REPAIR as f64 * (1. - TOWER_FALLOFF)) as u32
        );
        // test values generated in js using the engine's code
        assert_eq!(tower_repair_power_at_range(5), 800);
        assert_eq!(tower_repair_power_at_range(6), 760);
        assert_eq!(tower_repair_power_at_range(7), 720);
        assert_eq!(tower_repair_power_at_range(8), 680);
        assert_eq!(tower_repair_power_at_range(9), 640);
        assert_eq!(tower_repair_power_at_range(10), 600);
        assert_eq!(tower_repair_power_at_range(11), 560);
        assert_eq!(tower_repair_power_at_range(12), 520);
        assert_eq!(tower_repair_power_at_range(13), 480);
        assert_eq!(tower_repair_power_at_range(14), 440);
        assert_eq!(tower_repair_power_at_range(15), 400);
        assert_eq!(tower_repair_power_at_range(16), 360);
        assert_eq!(tower_repair_power_at_range(17), 320);
        assert_eq!(tower_repair_power_at_range(18), 280);
        assert_eq!(tower_repair_power_at_range(19), 240);
        assert_eq!(tower_repair_power_at_range(20), 200);
        // falloff range stops at 20, make sure beyond that stays at 200
        assert_eq!(tower_repair_power_at_range(25), 200);
        // math should work even at range 0
        assert_eq!(tower_repair_power_at_range(0), 800);
    }
}
