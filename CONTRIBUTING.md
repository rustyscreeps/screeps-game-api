# Contributing to `cargo-screeps` and `screeps-game-api`:

1. Ask questions. We're all in the `#rust-wasm` channel on the [screeps slack].
2. Make PRs early, often. We'll review code thoroughly and help where possible.
3. Issues are your guide for what needs to be done. If you think something needs doing and it isn't
   already an issue, make it one!

# Style

We adhere to the following guidelines:

- Code Formatting: [fmt-rfcs Rust Style Guide]
- API building: [Rust API Guidelines]

In addition, we have the following in-house guidelines:

## Code Formatting

Please run `rustfmt` on the files you edit when submitting to this repository. This will handle all
style issues mentioned in the 'fmt-rfcs' guidelines, so you should be able to simply run `cargo fmt`
and be done with it.

To install `rustfmt`, use `rustup`:

```console
$ rustup component add --toolchain nightly rustfmt-preview
```

Then to format the code in this repository, use the following:

```console
$ cargo fmt
```

## `use` formatting

Private `use` statements should be above all other non-use items within the module or function they
are declared.

Importing at the module level should be preferred to importing within functions, with the
exception of importing enum variants. Any statement like `use self::MyEnum::*;` should be used only
within a function in order to avoid polluting the module namespace.

When importing multiple things, `use` statements should be separated into the following
newline-separated groups:

- imports from `std`
- imports from external crates
- imports from the crate root
- imports from `self` or `super`.

`pub use` statements should all be in one group under any `mod` statements in a file.

All imports should use the use [RFC 2128] "nested groups" style. There should be one top-level
`use` statement for each of `std`, the current root, `self`, `super` and one for each external
crate.

In accordance to the `fmt-rfcs`, top-level `use` statements within a group and items within a `use`
statement should be alphabetically ordered.

Example import section:

```rust
use std::{
    borrow::Cow,
    cmp::{Eq, PartialEq},
    collections::HashMap,
    error, f64, fmt,
    marker::PhantomData,
    ops,
};

use serde::de::{Deserialize, Deserializer, Error, Unexpected, Visitor};
use stdweb::{
    unstable::{TryFrom, TryInto},
    Reference, Value,
};
use void::Void;


use {
    constants::{ResourceType, ReturnCode, StructureType},
    ConversionError,
};
```

Last, when importing from within the current crate, try to import from a more specific module rather
than a less specific one. When the crate re-exports tons of types from inner modules, it can be
tempting to just import everything from the crate root, but this makes it all more confusing. Import
from a more specific module which defines the type rather than having
`use {ResourceType, StructureProperties, LocalRoomPosition};` with each type coming from a different
module. Things should at minimum use the top-level module, but can also use more specific imports
for things from within the same top-level module.

To clarify, each import from within the same crate should be qualified by one level of module
outside of the current module's hierarchy. If in `objects::impl::construction_site`, importing
`objects::impl::room::Step` should be done with `objects::impl::room::Step` or `super::room::Step`,
but if in `constants`, then it can just be done with `objects::Step`.

[screeps slack]: https://chat.screeps.com/
[fmt-rfcs Rust Style Guide]: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md
[Rust API Guidelines]: https://rust-lang-nursery.github.io/api-guidelines/
[RFC 2128]: https://github.com/rust-lang/rfcs/blob/master/text/2128-use-nested-groups.md
