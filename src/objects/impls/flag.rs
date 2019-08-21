use stdweb::Value;

use crate::{
    constants::{Color, ReturnCode},
    macros::*,
    objects::{Flag, HasPosition},
    traits::TryFrom,
};

simple_accessors! {
    impl Flag {
        pub fn color() -> Color = color;
        pub fn name() -> String = name;
        pub fn secondary_color() -> Color = secondaryColor;
    }
}

impl Flag {
    /// Useful method for constructing Flag from the result of
    /// `Position.createFlag` or `Room.createFlag`.
    ///
    /// String names are mapped to Ok(Ok(s)), return codes are mapped to
    /// Ok(Err(e)), other unknown inputs are mapped to Err(e).
    pub(crate) fn interpret_creation_ret_value(
        value: Value,
    ) -> Result<Result<String, ReturnCode>, crate::ConversionError> {
        match value {
            num @ Value::Number(_) => Ok(Err(ReturnCode::try_from(num)?)),
            other => String::try_from(other).map(Ok),
        }
    }

    pub fn remove(&self) {
        js! { @(no_return)
            @{self.as_ref()}.remove();
        }
    }

    pub fn set_color(&self, color: Color, secondary_color: Option<Color>) {
        match secondary_color {
            None => js! { @(no_return)
                @{self.as_ref()}.setColor(@{color as u8});
            },
            Some(sec_color) => js! { @(no_return)
                @{self.as_ref()}.setColor(
                    @{color as u8},
                    @{sec_color as u8},
                );
            },
        };
    }

    pub fn set_position<T: HasPosition>(&self, pos: T) {
        let pos = pos.pos();
        js! { @(no_return)
            @{self.as_ref()}.setPosition(pos_from_packed(@{pos.packed_repr()}));
        }
    }

    pub fn set_position_xy(&self, x: u32, y: u32) {
        js! { @(no_return)
            @{self.as_ref()}.setPosition(@{x}, @{y});
        }
    }
}
