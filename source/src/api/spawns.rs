use stdweb::Value;
use stdweb::unstable::{TryFrom, TryInto};

use api::{Part, ReturnCode};

use num_traits::FromPrimitive;

pub fn spawn_names() -> Vec<String> {
    (js! {
        return Object.keys(Game.spawns);
    }).try_into()
        .unwrap()
}

pub fn pos(name: &str) -> (String, i32, i32) {
    let list: Vec<Value> = (js! {
        let pos = Game.spawns[@{name}].pos;
        return [pos.x, pos.y, pos.roomName];
    }).try_into()
        .unwrap();

    let mut it = list.into_iter();
    let i0 = it.next().unwrap();
    let i1 = it.next().unwrap();
    let i2 = it.next().unwrap();

    (
        i0.try_into().unwrap(),
        TryInto::<f64>::try_into(i1).unwrap() as i32,
        TryInto::<f64>::try_into(i2).unwrap() as i32,
    )
}

pub fn spawn_creep(spawn_name: &str, body: &[Part], name: &str) -> ReturnCode {
    let int_array: Vec<i32> = body.iter()
        .map(|p| match *p {
            Part::Work => 0,
            Part::Move => 1,
            Part::Carry => 2,
            Part::Attack => 3,
            Part::RangedAttack => 4,
            Part::Heal => 5,
            Part::Tough => 6,
            Part::Claim => 7,
        })
        .collect();

    let res = (js! {
        let arr = @{int_array}.map((num) => {
            switch (num) {
                case 0: return WORK;
                case 1: return MOVE;
                case 2: return CARRY;
                case 3: return ATTACK;
                case 4: return RANGED_ATTACK;
                case 5: return HEAL;
                case 6: return TOUGH;
                case 7: return CLAIM;
            }
        });
        return Game.spawns[@{spawn_name}].spawnCreep(arr, @{name});
    }).try_into()
        .unwrap();

    ReturnCode::from_i32(res).unwrap()
}
