# rusty_foundationmodels 0.2.0, modified for Saytrue

This directory is the Source Code Form of `rusty_foundationmodels` as shipped in Saytrue,
published to meet Section 3.2(a) of the Mozilla Public License 2.0 (see `LICENSE`).

- **Upstream:** https://crates.io/crates/rusty_foundationmodels/0.2.0
  (repository https://github.com/undivisible/rusty_foundationmodels, archived)
- **License:** MPL-2.0, unchanged. Only the files below differ from upstream; all other
  files are the published 0.2.0 sources.
- **Shipped in:** Saytrue 0.1.0 and later (macOS and iPhone, Apple Silicon)

## Modifications

Fixed value sets for string properties in structured output (since Saytrue 0.1.0):

| File | Change |
|------|--------|
| `src/lib.rs` | `SchemaProperty` gets an optional `choices: Option<Vec<String>>` field and a `choices()` builder; it is serialised as `choices` when set. |
| `bridge.swift` | `SchemaPropertyDesc` decodes `choices`; a string property with choices is built as `DynamicGenerationSchema(name:anyOf:)`, so the model can only produce one of these values. |

iOS build (since Saytrue 0.1.0 for iPhone):

| File | Change |
|------|--------|
| `build.rs` | Compiles `bridge.swift` for `target_os = "ios"` as well (device `arm64-apple-ios26.0` with SDK `iphoneos`, simulator `arm64-apple-ios26.0-simulator` with `iphonesimulator`). The macOS link arguments apply only on macOS. |

Lists and objects in structured output (from the release after Saytrue 0.1.0):

| File | Change |
|------|--------|
| `src/lib.rs` | `SchemaPropertyType` gets `Array` and `Object`; `SchemaProperty` gets `items`, `max_items` (serialised as `maxItems`) and `properties`, with the builders `array()`, `object()` and `max_items()`. |
| `bridge.swift` | `SchemaPropertyDesc` is a class, because an array element or object field is again a property description. `buildSchema` builds recursively: an array becomes `DynamicGenerationSchema(arrayOf:minimumElements:maximumElements:)`, an object `DynamicGenerationSchema(name:description:properties:)`. An unknown type inside makes the whole property invalid. |

Changed lines in `src/lib.rs` and `bridge.swift` are marked with `Saytrue patch`.
