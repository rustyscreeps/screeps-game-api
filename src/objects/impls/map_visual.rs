use js_sys::JsString;
use serde::Serialize;

use crate::{
    local::{Position, RoomCoordinate, RoomName},
    objects::{CircleStyle, LineStyle, PolyStyle, RectStyle},
    TextAlign,
};

#[derive(Clone, Serialize)]
pub struct MapCircleData {
    x: RoomCoordinate,
    y: RoomCoordinate,
    n: RoomName,
    #[serde(rename = "s")]
    style: CircleStyle,
}

#[derive(Clone, Serialize)]
pub struct MapLineData {
    x1: RoomCoordinate,
    y1: RoomCoordinate,
    n1: RoomName,
    x2: RoomCoordinate,
    y2: RoomCoordinate,
    n2: RoomName,
    #[serde(rename = "s")]
    style: LineStyle,
}

#[derive(Clone, Serialize)]
pub struct MapRectData {
    x: RoomCoordinate,
    y: RoomCoordinate,
    n: RoomName,
    #[serde(rename = "w")]
    width: u32,
    #[serde(rename = "h")]
    height: u32,
    #[serde(rename = "s")]
    style: RectStyle,
}

#[derive(Clone, Serialize)]
pub struct MapPolyPoint {
    x: RoomCoordinate,
    y: RoomCoordinate,
    n: RoomName,
}

impl From<&Position> for MapPolyPoint {
    fn from(pos: &Position) -> MapPolyPoint {
        MapPolyPoint {
            x: pos.x(),
            y: pos.y(),
            n: pos.room_name(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct MapPolyData {
    points: Vec<MapPolyPoint>,
    #[serde(rename = "s")]
    style: PolyStyle,
}

/// The font style for map text visuals.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MapFontStyle {
    /// Use the normal font face.
    #[default]
    Normal,
    /// Use the italic font face.
    Italic,
    /// Use the oblique font face.
    Oblique,
}

impl MapFontStyle {
    pub fn is_normal(&self) -> bool {
        matches!(self, MapFontStyle::Normal)
    }
}

/// The font variant for map text visuals.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MapFontVariant {
    /// No variant for the font, text rendered normally.
    #[default]
    Normal,
    /// Render all lowercase characters in small caps.
    SmallCaps,
}

impl MapFontVariant {
    pub fn is_normal(&self) -> bool {
        matches!(self, MapFontVariant::Normal)
    }
}

/// Settings for text visuals on the map, used with [`MapVisual::text`].
///
/// This is different than [`TextStyle`] which is used with [`RoomVisual::text`]
/// because the two methods take data in a slightly different format. Notably,
/// room visuals accept colors in any web format and take use a shorthond for
/// font data, where map visuals require stricter color formats and have
/// different font options.
///
/// <div class="warning">
/// <b>Warning</b>
///
/// The `backgroundPadding` setting in the Screeps docs does not function in
/// game so it is not present in this API.
/// </div><br/>
///
/// See also: [Screeps docs](https://docs.screeps.com/api/#Game.map-visual.text).
///
/// [`TextStyle`]: crate::objects::visual::TextStyle
/// [`RoomVisual::text`]: crate::objects::visual::RoomVisual::text
#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f32>,
    #[serde(skip_serializing_if = "MapFontStyle::is_normal")]
    font_style: MapFontStyle,
    #[serde(skip_serializing_if = "MapFontVariant::is_normal")]
    font_variant: MapFontVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    // This setting does not do anything, even though it's documented.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // background_padding: Option<f32>,
    #[serde(skip_serializing_if = "TextAlign::is_center")]
    align: TextAlign,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
}

impl MapTextStyle {
    /// Sets the color of the text style.
    ///
    /// **Unlike room visuals, only hex triplets with a leading `#` are valid.**
    ///
    /// The default value is `#FFFFFF`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use screeps::MapTextStyle;
    ///
    /// // Bright pink! We really need to see this text!
    /// let style = MapTextStyle::default().color("#FF00FF");
    /// ```
    pub fn color(mut self, val: impl Into<String>) -> Self {
        self.color = Some(val.into());
        self
    }

    /// Sets the font family of the text style. Font families with spaces in
    /// their name must be quoted. Normally this involves escaping quotes in a
    /// string literal.
    ///
    /// The default value is `sans-serif`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use screeps::MapTextStyle;
    /// let monospace_style = MapTextStyle::default().font_family("monospace");
    ///
    /// let font_family_style = MapTextStyle::default().font_family("\"Comic Sans MS\"");
    ///
    /// let fallback_style = MapTextStyle::default().font_family("\"Comic Sans MS\", Times, serif");
    /// ```
    ///
    /// For more information about font families, see the [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/font-family).
    pub fn font_family(mut self, val: impl Into<String>) -> Self {
        self.font_family = Some(val.into());
        self
    }

    /// Sets the size of the font, in *game coordinates*. This means that a
    /// `font_size` of 1 corresponds to the same size as one coordinate on the
    /// map. This does **not** support any other units for size.
    ///
    /// The default value is `10`.
    ///
    /// # Examples
    /// ```rust
    /// use screeps::MapTextStyle;
    ///
    /// let tiny_text = MapTextStyle::default().font_size(2_f32);
    ///
    /// let room_height = MapTextStyle::default().font_size(50_f32);
    /// ```
    pub fn font_size(mut self, val: f32) -> Self {
        self.font_size = Some(val);
        self
    }

    /// Sets the style of the font. This controls whether a font should be
    /// styled with a normal, italic, or oblique face.
    ///
    /// The default value is [`MapFontStyle::Normal`].
    ///
    /// `oblique <angle>` is not supported.
    ///
    /// # Examples
    /// ```rust
    /// use screeps::{MapFontStyle, MapTextStyle};
    ///
    /// let normal = MapTextStyle::default().font_style(MapFontStyle::Normal);
    /// let italic = MapTextStyle::default().font_style(MapFontStyle::Italic);
    /// ```
    ///
    /// For more information about font styles, see the [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/font-style).
    ///
    /// [`MapFontStyle::Normal`]: self::MapFontStyle#variant.Normal
    pub fn font_style(mut self, val: MapFontStyle) -> Self {
        self.font_style = val;
        self
    }

    /// Sets the variant of the font. This controls whether or not text is
    /// rendered as small caps.
    ///
    /// The default value is [`MapFontVariant::Normal`].
    ///
    /// # Examples
    /// ```rust
    /// use screeps::{MapFontVariant, MapTextStyle};
    ///
    /// let normal = MapTextStyle::default().font_variant(MapFontVariant::Normal);
    /// let small_caps = MapTextStyle::default().font_variant(MapFontVariant::SmallCaps);
    /// ```
    ///
    /// [`MapFontVariant::Normal`]: self::MapFontVariant#variant.Normal
    pub fn font_variant(mut self, val: MapFontVariant) -> Self {
        self.font_variant = val;
        self
    }

    /// **Unlike room visuals, only hex triplets with a leading `#` are valid.**
    pub fn stroke_color(mut self, val: Option<impl Into<String>>) -> Self {
        self.stroke = val.map(Into::into);
        self
    }

    /// Sets the width of the stroke for the text, if the stroke is enabled.
    ///
    /// A stroke width of `0` results in no stroke. Negative values are invalid.
    ///
    /// The default value is `0.15`.
    ///
    /// # Examples
    /// ```rust
    /// use screeps::MapTextStyle;
    ///
    /// let thin = MapTextStyle::default().stroke_width(0.05_f32);
    /// let wide = MapTextStyle::default().stroke_width(2.0_f32);
    /// ```
    pub fn stroke_width(mut self, val: f32) -> Self {
        self.stroke_width = Some(val);
        self
    }

    /// Sets or removes the background color for the text visual. When a
    /// background is enabled, the text's vertical align is set to `middle`
    /// instead of `baseline`.
    ///
    /// **Unlike room visuals, only hex triplets with a leading `#` are valid.**
    ///
    /// The default value is `None`.
    ///
    /// # Examples
    /// ```rust
    /// use screeps::MapTextStyle;
    ///
    /// let lavender = MapTextStyle::default().background_color(Some("#b373de"));
    /// // Even though this is the default, it's possible to unset the background color.
    /// let unset = MapTextStyle::default().background_color(None);
    /// ```
    // This only takes `&str` to avoid issues where passing `None` fails to infer a
    // type.
    pub fn background_color(mut self, val: Option<&str>) -> Self {
        self.background_color = val.map(String::from);
        self
    }

    /// Sets the horizontal alignment of the text.
    ///
    /// The default value is [`TextAlign::Center`].
    ///
    /// # Examples
    /// ```rust
    /// use screeps::{MapTextStyle, TextAlign};
    ///
    /// let left = MapTextStyle::default().align(TextAlign::Left);
    /// ```
    ///
    /// [`TextAlign::Center`]: crate::objects::visual::TextAlign#variant.Center
    pub fn align(mut self, val: TextAlign) -> Self {
        self.align = val;
        self
    }

    /// Sets the opacity of the text visual. Valid values are in the range
    /// `0.0..=1.0`.
    ///
    /// The default value is `0.5`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use screeps::MapTextStyle;
    ///
    /// let opaque = MapTextStyle::default().opacity(1.0_f32);
    /// let ghost = MapTextStyle::default().opacity(0.05_f32);
    /// ```
    pub fn opacity(mut self, val: f32) -> Self {
        self.opacity = Some(val);
        self
    }
}

#[derive(Clone, Serialize)]
pub struct MapTextData {
    text: String,
    x: RoomCoordinate,
    y: RoomCoordinate,
    n: RoomName,
    #[serde(rename = "s")]
    style: MapTextStyle,
}

#[derive(Clone, Serialize)]
#[serde(tag = "t")]
pub enum MapVisualShape {
    #[serde(rename = "c")]
    Circle(MapCircleData),
    #[serde(rename = "l")]
    Line(MapLineData),
    #[serde(rename = "r")]
    Rect(MapRectData),
    #[serde(rename = "p")]
    Poly(MapPolyData),
    #[serde(rename = "t")]
    Text(MapTextData),
}

impl MapVisualShape {
    pub fn circle(center: Position, style: CircleStyle) -> MapVisualShape {
        MapVisualShape::Circle(MapCircleData {
            x: center.x(),
            y: center.y(),
            n: center.room_name(),
            style,
        })
    }

    pub fn line(from: Position, to: Position, style: LineStyle) -> MapVisualShape {
        MapVisualShape::Line(MapLineData {
            x1: from.x(),
            y1: from.y(),
            n1: from.room_name(),
            x2: to.x(),
            y2: to.y(),
            n2: to.room_name(),
            style,
        })
    }

    pub fn rect(top_left: Position, width: u32, height: u32, style: RectStyle) -> MapVisualShape {
        MapVisualShape::Rect(MapRectData {
            x: top_left.x(),
            y: top_left.y(),
            n: top_left.room_name(),
            width,
            height,
            style,
        })
    }

    pub fn poly(points: Vec<MapPolyPoint>, style: PolyStyle) -> MapVisualShape {
        MapVisualShape::Poly(MapPolyData { points, style })
    }

    pub fn text(pos: Position, text: String, style: MapTextStyle) -> MapVisualShape {
        MapVisualShape::Text(MapTextData {
            x: pos.x(),
            y: pos.y(),
            n: pos.room_name(),
            text,
            style,
        })
    }
}

pub struct MapVisual {}

impl MapVisual {
    pub fn draw(visual: &MapVisualShape) {
        let val = serde_wasm_bindgen::to_value(visual).expect("expect convert visual to value");

        crate::console::add_visual(Some(&JsString::from("map")), &val);
    }

    pub fn draw_multi(visuals: &[MapVisualShape]) {
        for visual in visuals {
            let val = serde_wasm_bindgen::to_value(visual).expect("expect convert visual to value");

            crate::console::add_visual(Some(&JsString::from("map")), &val);
        }
    }

    pub fn circle(pos: Position, style: CircleStyle) {
        Self::draw(&MapVisualShape::circle(pos, style));
    }

    pub fn line(from: Position, to: Position, style: LineStyle) {
        Self::draw(&MapVisualShape::line(from, to, style));
    }

    pub fn rect(top_left: Position, width: u32, height: u32, style: RectStyle) {
        Self::draw(&MapVisualShape::rect(top_left, width, height, style));
    }

    pub fn poly(points: Vec<Position>, style: PolyStyle) {
        let points = points.iter().map(Into::into).collect();
        Self::draw(&MapVisualShape::poly(points, style));
    }

    pub fn text(pos: Position, text: String, style: MapTextStyle) {
        Self::draw(&MapVisualShape::text(pos, text, style));
    }
}
