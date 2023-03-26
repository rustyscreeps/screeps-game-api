# Contributing to `cargo-screeps` and `screeps-game-api`:

1. Ask questions. We're all in the `#rust` channel on the [screeps discord].
2. Make PRs early, often. We'll review code thoroughly and help where possible.
3. Issues are your guide for what needs to be done. If you think something needs doing and it isn't
   already an issue, make it one!
4. Whenever possible, include an update to the `CHANGELOG.md` in the Unreleased section briefly
   describing your change; include "(breaking)" at the end of the line if appropriate.

# Style

We adhere to the following guidelines:

- Code Formatting: [fmt-rfcs Rust Style Guide]
- API building: [Rust API Guidelines]

In addition, we have the following in-house guidelines:

## Code Formatting and Linting

Please run `rustfmt` and `clippy` on the files you edit when submitting to this repository. This
will handle all style issues mentioned in the 'fmt-rfcs' guidelines, so you should be able to
simply run `cargo fmt` and `cargo clippy` and be done with it.

To install the required components, use `rustup`:

```console
$ rustup component add rustfmt
$ rustup component add clippy
```

To format the code in this repository, use the following:

```console
$ cargo fmt
```

Also run `cargo clippy` and resolve any style issues it warns about.

```console
$ cargo clippy
```

## `use` formatting

Private `use` statements should all be together: there should be no non-private-`use` statements
between two private `use` statements.

Importing at the module level should be preferred to importing within functions, with the
exception of importing enum variants. Any statement like `use self::MyEnum::*;` should be used only
within a function in order to avoid polluting the module namespace.

When importing multiple things, `use` statements should be separated into the following
newline-separated groups:

- imports from `std`
- imports from external crates
- imports from the crate root
- imports from `super::`
- imports from `self::`

`pub use` statements should be similarly grouped, but should be separate from private `use` as
mentioned in [Ordering within a module].

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

[screeps discord]: https://discord.gg/screeps
[fmt-rfcs Rust Style Guide]: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md
[Rust API Guidelines]: https://rust-lang-nursery.github.io/api-guidelines/
[RFC 2128]: https://github.com/rust-lang/rfcs/blob/master/text/2128-use-nested-groups.md
[items]: https://doc.rust-lang.org/reference/items.html
