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

`use` statements should be at the top of the file, module or function they are used within.
Importing at the module level should be prefered to importing within functions, with the
exception of importing enum variants (`use self::MyEnum::*;` is useful when dealing with
big enums, but it's best kept only within the function it's used in).

When importing multiple things, `use` statements should be separated into the following
newline-separated groups:

- imports from `std`
- imports from external crates
- imports from the crate root
- imports from `self` or `super`.

In addition, all imports should use the use [RFC 2128] "nested groups" style. All imports inside
each of the groups and for each external crate should be grouped under one `use` statement.


Exampe import section:

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
module.

[screeps slack]: https://chat.screeps.com/
[fmt-rfcs Rust Style Guide]: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md
[Rust API Guidelines]: https://rust-lang-nursery.github.io/api-guidelines/
[RFC 2128]: https://github.com/rust-lang/rfcs/blob/master/text/2128-use-nested-groups.md
