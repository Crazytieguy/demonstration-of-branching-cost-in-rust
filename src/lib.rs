use std::f32::consts::PI;

pub enum ShapeType {
    Square,
    Rectangle,
    Triangle,
    Circle,
}

use ShapeType::*;

pub struct Shape {
    pub shape_type: ShapeType,
    pub width: f32,
    pub height: f32,
}

/// Roughly equivalent to Casey's GetAreaSwitch
pub fn get_area_branching(shape: &Shape) -> f32 {
    match shape.shape_type {
        Square => 1.0 * shape.width * shape.height,
        Rectangle => 1.0 * shape.width * shape.height,
        Triangle => 0.5 * shape.width * shape.height,
        Circle => PI * shape.width * shape.height,
    }
}

/// Roughly equivalent to Casey's GetAreaUnion
pub fn get_area_non_branching(shape: &Shape) -> f32 {
    let coefficient = match shape.shape_type {
        Square => 1.0,
        Rectangle => 1.0,
        Triangle => 0.5,
        Circle => PI,
    };
    coefficient * shape.width * shape.height
}
