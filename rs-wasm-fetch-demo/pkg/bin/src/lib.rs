use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen(start)]
pub fn init() {
    // Helpful panic messages in console
    console_error_panic_hook::set_once();
}

/// GET a JSON URL and return it as a JS object (Promise<unknown>)
#[wasm_bindgen]
pub async fn fetch_json(url: String) -> Result<JsValue, JsValue> {
    let window = window().ok_or("no global `window`")?;

    let opts = RequestInit::new();
    opts.set_method("GET");
    // Adjust mode as you need: Cors / NoCors / SameOrigin
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "HTTP error: {} {}",
            resp.status(),
            resp.status_text()
        )));
    }

    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

/// POST a JSON string and return JSON (Promise<unknown>)
#[wasm_bindgen]
pub async fn post_json(url: String, body: String) -> Result<JsValue, JsValue> {
    let window = window().ok_or("no global `window`")?;

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set("Accept", "application/json")?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "HTTP error: {} {}",
            resp.status(),
            resp.status_text()
        )));
    }

    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}
