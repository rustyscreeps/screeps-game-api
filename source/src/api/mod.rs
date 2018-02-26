macro_rules! get_from_js {
    ($name:ident -> { $js_side:expr } -> $rust_ret_type:ty) => (
        get_from_js!($name() -> { $js_side } -> $rust_ret_type);
    );
    (
        $name:ident(
            $($param_ident:ident: $param_ty:ty),*
        ) -> {
            $($js_side:tt)*
        } -> $rust_ret_type:ty
    ) => (
        pub fn $name(
            $($param_ident: $param_ty),*
        ) -> $rust_ret_type {
            use stdweb::unstable::TryInto;
            (js! { return $($js_side)*; }).try_into().unwrap()
        }
    )
}

pub mod game;
pub mod objects;

pub use self::objects::*;
pub use self::utils::*;

mod utils {
    use stdweb::Value;
    use stdweb::unstable::{TryFrom, TryInto};

    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Part {
        Move = 0,
        Work = 1,
        Carry = 2,
        Attack = 3,
        RangedAttack = 4,
        Tough = 5,
        Heal = 6,
        Claim = 7,
    }

    impl TryFrom<Value> for Part {
        type Error = <Value as TryInto<u32>>::Error;
        fn try_from(v: Value) -> Result<Self, Self::Error> {
            let x: u32 = (js!(
                switch (@{v}) {
                    case MOVE: return 0;
                    case WORK: return 1;
                    case CARRY: return 2;
                    case ATTACK: return 3;
                    case RANGED_ATTACK: return 4;
                    case TOUGH: return 5;
                    case HEAL: return 6;
                    case CLAIM: return 7;
                    default: return -1;
                }
            )).try_into()?;
            let res = match x {
                0 => Part::Move,
                1 => Part::Work,
                2 => Part::Carry,
                3 => Part::Attack,
                4 => Part::RangedAttack,
                5 => Part::Tough,
                6 => Part::Heal,
                7 => Part::Claim,
                _ => unreachable!(),
            };
            Ok(res)
        }
    }

    enum_from_primitive! {
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ReturnCode {
        Ok = 0,
        NotOwner = -1,
        NameExists = -3,
        Busy = -4,
        NotFound = -5,
        NotEnough = -6,
        InvalidTarget = -7,
        Full = -8,
        NotInRange = -9,
        InvalidArgs = -10,
        Tired = -11,
        NoBodypart = -12,
        RclNotEnough = -14,
        GclNotEnough = -15,
    }
    }

    impl TryFrom<Value> for ReturnCode {
        type Error = <Value as TryInto<i32>>::Error;
        fn try_from(v: Value) -> Result<Self, Self::Error> {
            use num_traits::FromPrimitive;
            let x: i32 = v.try_into()?;
            Ok(Self::from_i32(x).unwrap())
        }
    }

    enum_from_primitive! {
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Direction {
        Top = 1,
        TopRight = 2,
        Right = 3,
        BottomRight = 4,
        Bottom = 5,
        BottomLeft = 6,
        Left = 7,
        TopLeft = 8,
    }
    }
}
