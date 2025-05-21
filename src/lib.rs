mod utils;

use wasm_bindgen::prelude::*;
use descriptor_encrypt::{encrypt, decrypt, get_template};
use miniscript::descriptor::{Descriptor, DescriptorPublicKey};
use std::str::FromStr;

#[wasm_bindgen]
pub fn encrypt_descriptor(descriptor_string: &str) -> Result<String, JsValue> {
    let desc = Descriptor::<DescriptorPublicKey>::from_str(descriptor_string)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let encrypted = encrypt(desc)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(hex::encode(encrypted))
}

#[wasm_bindgen]
pub fn decrypt_descriptor(hex_data: &str, keys_string: &str) -> Result<String, JsValue> {
    let data = hex::decode(hex_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // Parse keys from string (comma-separated list of keys)
    let mut keys = Vec::new();
    for key_str in keys_string.split(',') {
        if !key_str.trim().is_empty() {
            let key = DescriptorPublicKey::from_str(key_str.trim())
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            keys.push(key);
        }
    }
    
    let descriptor = decrypt(&data, keys)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(descriptor.to_string())
}

#[wasm_bindgen]
pub fn get_descriptor_template(hex_data: &str) -> Result<String, JsValue> {
    let data = hex::decode(hex_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let template = get_template(&data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(template.to_string())
}
