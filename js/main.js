
// __initialize defined above when files are concatenated
// it's signature is 'function __initialize( __wasm_module, __load_asynchronously ) {'
__initialize(new WebAssembly.Module(require('compiled')), false);
