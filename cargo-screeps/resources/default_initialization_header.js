"use strict";
let wasm_module = null;

function wasm_initialize() {
    if (wasm_module == null) {
        let wasm_bytes = wasm_fetch_module_bytes();
        wasm_module = new WebAssembly.Module(wasm_bytes);
    }
    let stdweb_vars = wasm_create_stdweb_vars();
    let wasm_instance = new WebAssembly.Instance(wasm_module, stdweb_vars.imports);
    stdweb_vars.initialize(wasm_instance);
    // assume the WASM main overrides this
    module.exports.loop();
}

module.exports.loop = wasm_initialize;
