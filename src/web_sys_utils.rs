use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub type Result<T> = std::result::Result<T, wasm_bindgen::JsValue>;

pub fn get_element_by_id<T: JsCast>(id: &str) -> T {
    let element = get_document()
        .get_element_by_id(id)
        .expect(&format!("Failed to get element by id={}", id));
    element
        .dyn_into::<T>()
        .map_err(|_| ())
        .expect(&format!("Failed to cast element (id={})", id))
}

pub fn get_document() -> web_sys::Document {
    web_sys::window()
        .expect("Failed to get window.")
        .document()
        .expect("Failed to get document.")
}

pub fn read_text(file: &web_sys::File, buffer: Rc<Cell<Option<String>>>) {
    let reader = web_sys::FileReader::new().unwrap();
    let reader_c = reader.clone();
    let onloadend_callback = Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
        let text = reader_c.result().unwrap().as_string().unwrap();
        buffer.set(Some(text));
    }) as Box<dyn FnMut(web_sys::ProgressEvent)>);
    reader.set_onloadend(Some(onloadend_callback.as_ref().unchecked_ref()));
    reader.read_as_text(file).expect("File not readable");
    onloadend_callback.forget();
}
