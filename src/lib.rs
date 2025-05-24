mod utils;

use wasm_bindgen::prelude::*;
use descriptor_encrypt::{encrypt, encrypt_with_full_secrecy, decrypt, get_template};
use bitcoin::secp256k1::Secp256k1;
use miniscript::descriptor::{Descriptor, DescriptorPublicKey, DescriptorSecretKey};
use std::str::FromStr;

#[wasm_bindgen]
pub fn encrypt_descriptor(descriptor: String) -> Result<String, JsValue> {
    let secp = Secp256k1::new();
    let (desc, _) = Descriptor::<DescriptorPublicKey>::parse_descriptor(&secp, &descriptor)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let encrypted = encrypt(desc)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(hex::encode(encrypted))
}

#[wasm_bindgen]
pub fn encrypt_descriptor_with_full_secrecy(descriptor: String) -> Result<String, JsValue> {
    let secp = Secp256k1::new();
    let (desc, _) = Descriptor::<DescriptorPublicKey>::parse_descriptor(&secp, &descriptor)
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
        let trimmed_key = key.trim();
        if !trimmed_key.is_empty() {
            let key = if let Ok(key) = DescriptorSecretKey::from_str(trimmed_key) {
                let secp = Secp256k1::new();
                key.to_public(&secp)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?
            } else {
                DescriptorPublicKey::from_str(trimmed_key)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?
            };
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