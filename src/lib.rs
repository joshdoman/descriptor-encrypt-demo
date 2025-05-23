mod utils;

use wasm_bindgen::prelude::*;
use descriptor_encrypt::{encrypt, encrypt_with_full_secrecy, decrypt, get_template};
use miniscript::descriptor::{Descriptor, DescriptorPublicKey};
use std::str::FromStr;

#[wasm_bindgen]
pub fn encrypt_descriptor(descriptor: String) -> Result<String, JsValue> {
    let desc = Descriptor::<DescriptorPublicKey>::from_str(&descriptor)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let encrypted = encrypt(desc)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(hex::encode(encrypted))
}

#[wasm_bindgen]
pub fn encrypt_descriptor_with_full_secrecy(descriptor: String) -> Result<String, JsValue> {
    let desc = Descriptor::<DescriptorPublicKey>::from_str(&descriptor)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let encrypted = encrypt_with_full_secrecy(desc)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(hex::encode(encrypted))
}

#[wasm_bindgen]
pub fn decrypt_descriptor(hex_data: String, keys: Vec<String>) -> Result<String, JsValue> {
    let data = hex::decode(&hex_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let mut decoded_keys = Vec::new();
    for key in keys {
        if !key.trim().is_empty() {
            let key = DescriptorPublicKey::from_str(key.trim())
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            decoded_keys.push(key);
        }
    }
    
    let descriptor = decrypt(&data, decoded_keys)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(descriptor.to_string())
}

#[wasm_bindgen]
pub fn get_descriptor_template(hex_data: String) -> Result<String, JsValue> {
    let data = hex::decode(&hex_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let template = get_template(&data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(template.to_string())
}

#[wasm_bindgen]
pub fn get_origin_derivation_paths(hex_data: String) -> Result<Vec<String>, JsValue> {
    let data = hex::decode(&hex_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let paths = descriptor_encrypt::get_origin_derivation_paths(&data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .into_iter()
        .map(|path| path.to_string())
        .collect();
    
    Ok(paths)
}