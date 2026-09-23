#[cfg(target_arch = "wasm32")]
mod components;
mod content;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    let root = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("app"))
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok());
    if let Some(root) = root {
        leptos::mount::mount_to(root, || leptos::view! { <components::App /> }).forget();
    } else {
        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
            "application root unavailable",
        ));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
