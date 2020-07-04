use crate::render::render_smile;
use crate::Result;
use std::str::FromStr;
use web_sys::CanvasRenderingContext2d;

const WIDTH: f64 = 50.0;

pub struct State {
    width: f64,
    height: f64,

    cur_x: f64,
    cur_y: f64,

    pressed_key: Option<ArrowKey>,

    context: CanvasRenderingContext2d,
}

#[derive(Copy, Clone)]
pub enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for ArrowKey {
    type Err = wasm_bindgen::JsValue;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Right" | "ArrowRight" => Ok(ArrowKey::Right),
            "Left" | "ArrowLeft" => Ok(ArrowKey::Left),
            "Down" | "ArrowDown" => Ok(ArrowKey::Down),
            "Up" | "ArrowUp" => Ok(ArrowKey::Up),
            _ => Err(format!("Can not parse: {}", s).into()),
        }
    }
}

impl State {
    pub fn new(context: CanvasRenderingContext2d, width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            context,
            cur_x: 0.0,
            cur_y: 0.0,
            pressed_key: None,
        }
    }

    pub fn step(&mut self) -> Result<()> {
        if let Some(key) = self.pressed_key {
            match key {
                ArrowKey::Left => {
                    if self.cur_x - 1.0 >= 0.0 {
                        self.cur_x -= 1.0;
                    }
                }
                ArrowKey::Up => {
                    if self.cur_y - 1.0 >= 0.0 {
                        self.cur_y -= 1.0;
                    }
                }
                ArrowKey::Down => {
                    if self.cur_y + 1.0 + WIDTH <= self.height {
                        self.cur_y += 1.0;
                    }
                }
                ArrowKey::Right => {
                    if self.cur_x + 1.0 + WIDTH <= self.width {
                        self.cur_x += 1.0;
                    }
                }
            }
        }
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
        render_smile(&self.context, self.cur_x, self.cur_y, WIDTH)
    }

    pub fn down_key(&mut self, key: ArrowKey) {
        self.pressed_key = Some(key);
    }
    pub fn up_key(&mut self) {
        self.pressed_key = None;
    }
}
