use {
    constants::Color,
    objects::{Flag, HasPosition},
};

simple_accessors! {
    Flag;
    (color -> color -> Color),
    (name -> name -> String),
    (secondary_color -> secondaryColor -> Color),
}

impl Flag {
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
