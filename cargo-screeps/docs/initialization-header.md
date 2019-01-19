Writing an initialization header
================================

The initialization header is a piece of code controlling how the WASM environment starts up. As
screeps is a resource constrained environment, and your rust code may want to interop with other
code, this initialization is completely controllable.

The default initialization header is as follows:

```js
"use strict";
// Create a global variable to store the wasm module. This is the compiled uninstantiated code.
// If `new WebAssembly.Instance` times out, this will helpfully remain here to be reused next tick.
let wasm_module = null;

// This function does the initialization. It's separate from `module.exports.loop` so JS code can
// run `module.exports.loop = wasm_initialize` to reset the WASM vm next tick.
function wasm_initialize() {
    // If we haven't compiled the code, let's do that.
    if (wasm_module == null) {
        // Fetch the source byte array. `wasm_fetch_module_bytes` just returns
        // require('wasm_module_name'), but it's useful to have as a separate function
        // so we don't hardcode the module name.
        let wasm_bytes = wasm_fetch_module_bytes();
        // Compile the module. I've found that this can time out, but because of however it works,
        // `wasm_module` will still be initialized.
        wasm_module = new WebAssembly.Module(wasm_bytes);
    }
    // Initialize `stdweb`. This returns an object with an `initialize` function and `imports`
    // object, meant to be used as below:
    let stdweb_vars = wasm_create_stdweb_vars();
    // Create the WebAssembly instance.
    let wasm_instance = new WebAssembly.Instance(wasm_module, stdweb_vars.imports);
    // Run the remaining stdweb initialization. This is fairly short, just some things to make sure
    // callbacks work right and such. This will also run the rust `main` function.
    stdweb_vars.initialize(wasm_instance);
    // Now we run the actual game loop - this assumes that the rust `main` function overwrites
    // `module.exports.loop`.
    module.exports.loop();
}

module.exports.loop = wasm_initialize;
```

For reference, see [WebAssembly.Module] and [WebAssembly.Instance] docs.

The two utility functions, `wasm_fetch_module_bytes` and `wasm_create_stdweb_vars` will always be
generated, regardless of initialization header. Their purpose is to provide an interface to stdweb's
internal initialization, and to remain stable between stdweb and cargo-web updates.

They are documented as follows:

```js
/**
 * Fetches WASM bytes for the wasm module.
 *
 * These should be given to `new WebAssembly.Module` to instantiate a WebAssembly module.
 */
function wasm_fetch_module_bytes() {
    ...
}

/**
 * Creates the stdweb wrapper for a module instance.
 *
 * It has two properties, `imports` and `initialize`. `imports` should be passed as the second
 * argument to `new WebAssembly.Instance` to provide the WASM module with imports, and `initialize`
 * should be called on the resulting WebAssembly instance.
 *
 * Calling `initialize` will finish associating the imports with the wasm module, and will call the
 * rust module's main function.
 */
function wasm_create_stdweb_vars() {
    ...
}
```

## Making your own `initialization_header`

To fully initialize the WASM instance, you will at minimum need to do the following things:

- call `wasm_create_stdweb_vars()`
- initialize the instance using `stdweb_vars.imports` as the import section
  - I would recommend calling `new WebAssembly.Module` then `new WebAssembly.Instance` like the
    default header, but there are other ways to do this, like `WebAssembly.instantiate()`
- Call `stdweb_vars.initialize(the_instance_you_created)` to finish stdweb initialization, linking
  all the necessary WASM and JS functions together.

I would recommend using `wasm_fetch_module_bytes()` to fetch the source bytes to feed to the
`WebAssembly.Module` constructor, but if you get them some other way that works too.

The default initialization header does all these and not much else. It tries to remain utilitarian
and do the absolute minimum to get you running.

### Example: handle low bucket CPU

The following will try to prevent WASM initialization from timing out. Exiting out early like this
is a fairly standard thing to do, even in JS codebases. But at what point you want to exit out will
depend on your codebase. 500 bucket is quite a generous amount.

```js
"use strict";
let wasm_module = null;

function wasm_initialize() {
    if (Game.cpu.bucket < 500) {
        return;
    }
    if (wasm_module == null) {
        let wasm_bytes = wasm_fetch_module_bytes();
        wasm_module = new WebAssembly.Module(wasm_bytes);
    }
    // The biggest CPU users will be the call to `new WebAssembly.Module` and
    // `new WebAssembly.Instance`, so having two checks will be useful.
    if (Game.cpu.bucket < 500) {
        return;
    }
    let stdweb_vars = wasm_create_stdweb_vars();
    let wasm_instance = new WebAssembly.Instance(wasm_module, stdweb_vars.imports);
    stdweb_vars.initialize(wasm_instance);
    module.exports.loop();
}

module.exports.loop = wasm_initialize;
```

### Example: measuring CPU usage loading WASM.

Exporting to statistics like Graphana will probably be more useful than logging to console, but you
can see the idea.

```js
"use strict";
let wasm_module = null;

function wasm_initialize() {
    let initial_cpu = Game.cpu.getUsed();
    if (wasm_module == null) {
        let wasm_bytes = wasm_fetch_module_bytes();
        wasm_module = new WebAssembly.Module(wasm_bytes);
    }
    let cpu_post_compiling = Game.cpu.getUsed();
    let stdweb_vars = wasm_create_stdweb_vars();
    let cpu_post_stdweb_vars_creation = Game.cpu.getUsed();
    let wasm_instance = new WebAssembly.Instance(wasm_module, stdweb_vars.imports);
    let cpu_post_instantiation = Game.cpu.getUsed();
    stdweb_vars.initialize(wasm_instance);
    let cpu_post_initialization = Game.cpu.getUsed();
    console.log(`Initialized WASM.
Module Compilation: ${cpu_post_compiling - initial_cpu} CPU
stdweb vars creation: ${cpu_post_stdweb_vars_creation - cpu_post_compiling} CPU
WebAssembly.Instance creation: ${cpu_post_instantiation - cpu_post_stdweb_vars_creation} CPU
stdweb initialization finish: ${cpu_post_initialization - cpu_post_instantiation} CPU
total: ${cpu_post_initialization - initial_cpu} CPU`);
    module.exports.loop();
}

module.exports.loop = wasm_initialize;
```

### Beyond that

I don't have many more recommendations from here on out. You can inovate how you'd like, but if
you'd rather focus on other areas of the game then using the example which exits on low CPU usage
as is is more than reasonable.

If you have more examples you think would be useful here, however, I'd welcome them! Pull requests
will be accepted.

[WebAssembly.Module]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Module#Constructor_Syntax
[WebAssembly.Instance]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/WebAssembly/Instance#Constructor_Syntax
