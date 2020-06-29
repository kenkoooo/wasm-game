use crate::state::{ArrowKey, State};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod render;
mod state;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let state = Rc::new(State::new(context, width, height));
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    {
        let state = state.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            state.step().unwrap();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    {
        let state = state.clone();
        let keydown_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Right" || e.key() == "ArrowRight" {
                state.down_key(ArrowKey::Right);
            } else if e.key() == "Left" || e.key() == "ArrowLeft" {
                state.down_key(ArrowKey::Left);
            } else if e.key() == "Up" || e.key() == "ArrowUp" {
                state.down_key(ArrowKey::Up);
            } else if e.key() == "Down" || e.key() == "ArrowDown" {
                state.down_key(ArrowKey::Down);
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

        document.set_onkeydown(Some(keydown_handler.as_ref().unchecked_ref()));
        keydown_handler.forget();
    }

    {
        let state = state.clone();
        let keyup_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Right"
                || e.key() == "ArrowRight"
                || e.key() == "Left"
                || e.key() == "ArrowLeft"
            {
                state.up_key();
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        document.set_onkeyup(Some(keyup_handler.as_ref().unchecked_ref()));
        keyup_handler.forget();
    }
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
