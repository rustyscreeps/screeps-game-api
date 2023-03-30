use js_sys::JsString;
use serde::Serialize;

use crate::{
    local::{Position, RoomCoordinate, RoomName},
    objects::{CircleStyle, LineStyle, PolyStyle, RectStyle, TextStyle},
};

#[derive(Clone, Serialize)]
pub struct MapCircleData {
    x: RoomCoordinate,
    y: RoomCoordinate,
    n: RoomName,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    style: Option<CircleStyle>,
}

#[derive(Clone, Serialize)]
pub struct MapLineData {
    x1: RoomCoordinate,
    y1: RoomCoordinate,
    n1: RoomName,
    x2: RoomCoordinate,
    y2: RoomCoordinate,
    n2: RoomName,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    style: Option<LineStyle>,
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
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    style: Option<RectStyle>,
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
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    style: Option<PolyStyle>,
}

#[derive(Clone, Serialize)]
pub struct MapTextData {
    text: String,
    x: RoomCoordinate,
    y: RoomCoordinate,
    n: RoomName,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    style: Option<TextStyle>,
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
    pub fn circle(center: Position, style: Option<CircleStyle>) -> MapVisualShape {
        MapVisualShape::Circle(MapCircleData {
            x: center.x(),
            y: center.y(),
            n: center.room_name(),
            style,
        })
    }

    pub fn line(from: Position, to: Position, style: Option<LineStyle>) -> MapVisualShape {
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

    pub fn rect(
        top_left: Position,
        width: u32,
        height: u32,
        style: Option<RectStyle>,
    ) -> MapVisualShape {
        MapVisualShape::Rect(MapRectData {
            x: top_left.x(),
            y: top_left.y(),
            n: top_left.room_name(),
            width,
            height,
            style,
        })
    }

    pub fn poly(points: Vec<MapPolyPoint>, style: Option<PolyStyle>) -> MapVisualShape {
        MapVisualShape::Poly(MapPolyData { points, style })
    }

    pub fn text(pos: Position, text: String, style: Option<TextStyle>) -> MapVisualShape {
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

    pub fn circle(pos: Position, style: Option<CircleStyle>) {
        Self::draw(&MapVisualShape::circle(pos, style));
    }

    pub fn line(from: Position, to: Position, style: Option<LineStyle>) {
        Self::draw(&MapVisualShape::line(from, to, style));
    }

    pub fn rect(top_left: Position, width: u32, height: u32, style: Option<RectStyle>) {
        Self::draw(&MapVisualShape::rect(top_left, width, height, style));
    }

    pub fn poly(points: Vec<Position>, style: Option<PolyStyle>) {
        let points = points.iter().map(Into::into).collect();
        Self::draw(&MapVisualShape::poly(points, style));
    }

    pub fn text(pos: Position, text: String, style: Option<TextStyle>) {
        Self::draw(&MapVisualShape::text(pos, text, style));
    }
}
