use stdweb::Value;

use {
    constants::{Color, ReturnCode},
    objects::{Flag, HasPosition},
    traits::TryFrom,
};

simple_accessors! {
    Flag;
    (color -> color -> Color),
    (name -> name -> String),
    (secondary_color -> secondaryColor -> Color),
}

impl Flag {
    /// Useful method for constructing Flag from the result of `RoomPosition.createFlag`
    /// or `Room.createFlag`.
    ///
    /// String names are mapped to Ok(Ok(s)), return codes are mapped to Ok(Err(e)), other
    /// unknown inputs are mapped to Err(e).
    pub(crate) fn interpret_creation_ret_value(
        value: Value,
    ) -> Result<Result<String, ReturnCode>, crate::ConversionError> {
        match value {
            Value::Number(num) => Ok(Err(ReturnCode::try_from(num)?)),
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
                @{self.as_ref()}.setColor(@{u32::from(color)});
            },
            Some(sec_color) => js! { @(no_return)
                @{self.as_ref()}.setColor(
                    @{u32::from(color)},
                    @{u32::from(sec_color)},
                );
            },
        };
    }

    pub fn set_position<T: HasPosition>(&self, pos: T) {
        let room_pos = pos.pos();
        js! { @(no_return)
            @{self.as_ref()}.setPosition(@{room_pos.as_ref()});
        }
    }

    pub fn set_position_xy(&self, x: u32, y: u32) {
        js! { @(no_return)
            @{self.as_ref()}.setPosition(@{x}, @{y});
        }
    }
}
