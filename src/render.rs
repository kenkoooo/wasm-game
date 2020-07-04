use crate::UnwrapOrLog;
use std::f64;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub fn render_smile(
    context: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
) -> Result<(), JsValue> {
    let cx = x + width / 2.0;
    let cy = y + width / 2.0;

    context.begin_path();

    let face_radius = width / 3.0;

    // Draw the outer circle.
    context
        .arc(cx, cy, face_radius, 0.0, f64::consts::PI * 2.0)
        .unwrap_or_log();

    let mouth_radius = face_radius * 0.7;
    // Draw the mouth.
    context.move_to(cx + mouth_radius, cy);
    context
        .arc(cx, cy, mouth_radius, 0.0, f64::consts::PI)
        .unwrap_or_log();

    let eye_radius = face_radius / 10.0;
    let eye_y = cy - eye_radius * 2.0;
    let left_eye_start_x = cx - eye_radius * 2.0;
    // Draw the left eye.
    context.move_to(left_eye_start_x, eye_y);
    context
        .arc(
            left_eye_start_x - eye_radius,
            eye_y,
            eye_radius,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap_or_log();

    let right_eye_start_x = cx + eye_radius * 2.0 + eye_radius * 2.0;
    // Draw the right eye.
    context.move_to(right_eye_start_x, eye_y);
    context
        .arc(
            right_eye_start_x - eye_radius,
            eye_y,
            eye_radius,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap_or_log();

    context.stroke();
    Ok(())
}
