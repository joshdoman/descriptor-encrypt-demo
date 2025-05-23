// bootstrap.js
import("./pkg/descriptor_encrypt_demo.js")
  .then(module => {
    window.wasm = {
      encrypt_descriptor: module.encrypt_descriptor,
      encrypt_descriptor_with_full_secrecy: module.encrypt_descriptor_with_full_secrecy,
      decrypt_descriptor: module.decrypt_descriptor,
      get_descriptor_template: module.get_descriptor_template,
      get_origin_derivation_paths: module.get_origin_derivation_paths
    };
    
    // Initialize the WASM module
    module.default();
    
    console.log("WebAssembly module loaded successfully!");
  })
  .catch(e => console.error("Error importing WASM module:", e));