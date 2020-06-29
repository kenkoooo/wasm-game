use crate::render::render_smile;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

const WIDTH: f64 = 50.0;

pub struct State {
    width: f64,
    height: f64,

    cur_x: Rc<Cell<f64>>,
    cur_y: Rc<Cell<f64>>,

    pressed_key: Rc<Cell<Option<ArrowKey>>>,

    context: Rc<CanvasRenderingContext2d>,
}

#[derive(Copy, Clone)]
pub enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
}

impl State {
    pub fn new(context: CanvasRenderingContext2d, width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            context: Rc::new(context),
            cur_x: Rc::new(Cell::new(0.0)),
            cur_y: Rc::new(Cell::new(0.0)),
            pressed_key: Rc::new(Cell::new(None)),
        }
    }

    fn go_left(&self) {
        let cur_x = self.cur_x.clone();
        if cur_x.get() - 1.0 >= 0.0 {
            cur_x.set(cur_x.get() - 1.0);
        }
    }
    fn go_right(&self) {
        let cur_x = self.cur_x.clone();
        if cur_x.get() + 1.0 + WIDTH <= self.width {
            cur_x.set(cur_x.get() + 1.0);
        }
    }
    fn go_up(&self) {
        let cur_y = self.cur_y.clone();
        if cur_y.get() - 1.0 >= 0.0 {
            cur_y.set(cur_y.get() - 1.0);
        }
    }
    fn go_down(&self) {
        let cur_y = self.cur_y.clone();
        if cur_y.get() + 1.0 + WIDTH <= self.height {
            cur_y.set(cur_y.get() + 1.0);
        }
    }

    pub fn step(&self) -> Result<(), JsValue> {
        let pressed_key = self.pressed_key.clone();
        if let Some(key) = pressed_key.get() {
            match key {
                ArrowKey::Left => self.go_left(),
                ArrowKey::Up => self.go_up(),
                ArrowKey::Down => self.go_down(),
                ArrowKey::Right => self.go_right(),
            }
        }
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
        render_smile(&self.context, self.cur_x.get(), self.cur_y.get(), WIDTH)
    }

    pub fn down_key(&self, key: ArrowKey) {
        let pressed_key = self.pressed_key.clone();
        pressed_key.set(Some(key));
    }
    pub fn up_key(&self) {
        let pressed_key = self.pressed_key.clone();
        pressed_key.set(None);
    }
}
