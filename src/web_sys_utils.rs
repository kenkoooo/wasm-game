use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub type Result<T> = std::result::Result<T, wasm_bindgen::JsValue>;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

pub fn get_element_by_id<T: JsCast>(id: &str) -> T {
    let element = get_document().get_element_by_id(id).unwrap_or_log();
    element
        .dyn_into::<T>()
        .map_err(|_| format!("Failed to cast element (id={})", id).into())
        .unwrap_or_log()
}

pub fn get_document() -> web_sys::Document {
    web_sys::window().unwrap_or_log().document().unwrap_or_log()
}

pub fn read_text(file: &web_sys::File, buffer: Rc<Cell<Option<String>>>) {
    let reader = web_sys::FileReader::new().unwrap_or_log();
    let reader_c = reader.clone();
    let onloadend_callback = Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
        let text = reader_c
            .result()
            .unwrap_or_log()
            .as_string()
            .unwrap_or_log();
        buffer.set(Some(text));
    }) as Box<dyn FnMut(web_sys::ProgressEvent)>);
    reader.set_onloadend(Some(onloadend_callback.as_ref().unchecked_ref()));
    reader.read_as_text(file).unwrap_or_log();
    onloadend_callback.forget();
}

pub trait UnwrapOrLog<T> {
    fn unwrap_or_log(self) -> T;
}

impl<T> UnwrapOrLog<T> for Result<T> {
    fn unwrap_or_log(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                console_log!("{:?}", e);
                panic!("{:?}", e);
            }
        }
    }
}

impl<T> UnwrapOrLog<T> for Option<T> {
    fn unwrap_or_log(self) -> T {
        match self {
            Some(v) => v,
            None => {
                console_log!("called `Option::unwrap()` on a `None` value");
                panic!("called `Option::unwrap()` on a `None` value");
            }
        }
    }
}
