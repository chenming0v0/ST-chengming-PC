use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    pub async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

pub fn tauri_available() -> bool {
    web_sys::window()
        .and_then(|window| js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI__")).ok())
        .map(|value| !value.is_undefined() && !value.is_null())
        .unwrap_or(false)
}

pub async fn tauri_invoke<T: for<'de> Deserialize<'de>>(
    cmd: &str,
    args: &JsValue,
) -> Result<T, String> {
    if !tauri_available() {
        return Err("当前不在 Tauri 运行环境中".to_string());
    }

    let result = invoke(cmd, args.clone()).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| format!("反序列化失败: {}", e))
}

pub async fn tauri_invoke_string(cmd: &str, args: &JsValue) -> Result<String, String> {
    if !tauri_available() {
        return Err("当前不在 Tauri 运行环境中".to_string());
    }

    let result = invoke(cmd, args.clone()).await;
    result.as_string().ok_or_else(|| {
        serde_wasm_bindgen::from_value::<String>(result.clone())
            .unwrap_or_else(|_| format!("调用 {} 失败", cmd))
    })
}

pub fn empty_args() -> JsValue {
    JsValue::from(js_sys::Object::new())
}

pub fn command_args(pairs: &[(&str, JsValue)]) -> JsValue {
    let obj = js_sys::Object::new();
    for (key, value) in pairs {
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(key), value);
    }
    JsValue::from(obj)
}
