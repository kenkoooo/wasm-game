use crate::state::{ArrowKey, State};
use crate::web_sys_utils::{get_document, get_element_by_id, read_text};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod render;
mod state;
mod web_sys_utils;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let canvas: web_sys::HtmlCanvasElement = get_element_by_id("canvas");
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let buffer: Rc<Cell<Option<String>>> = Rc::new(Cell::new(None));
    let state = Rc::new(State::new(context, width, height));
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    {
        let state = state.clone();
        let buffer = buffer.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let buffer = buffer.clone();
            if let Some(a) = buffer.take() {
                console_log!("{}", a);
            }
            state.step().unwrap();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    {
        let state = state.clone();
        let keydown_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if let Ok(key) = e.key().parse::<ArrowKey>() {
                state.down_key(key);
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

        get_document().set_onkeydown(Some(keydown_handler.as_ref().unchecked_ref()));
        keydown_handler.forget();
    }

    {
        let state = state.clone();
        let keyup_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key().parse::<ArrowKey>().is_ok() {
                state.up_key();
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

        get_document().set_onkeyup(Some(keyup_handler.as_ref().unchecked_ref()));
        keyup_handler.forget();
    }

    {
        let buffer = buffer.clone();
        let input_change_handler = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let input1: web_sys::HtmlInputElement = get_element_by_id("input1");
            if let Some(file) = input1.files().unwrap().get(0) {
                read_text(&file, buffer.clone());
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        get_element_by_id::<web_sys::HtmlInputElement>("input1")
            .set_onchange(Some(input_change_handler.as_ref().unchecked_ref()));
        input_change_handler.forget();
    }
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
