# wp-source-types

Common source-side connector types shared across the WP connector ecosystem.

## Modules

### `tags` — Lightweight Key-Value Tag Collection

```rust
use wp_source_types::Tags;

let mut tags = Tags::new();
tags.set("env", "prod");
tags.set("region", "us-west");

assert_eq!(tags.get("env"), Some("prod"));
assert!(tags.contains_key("region"));
```

- **Zero heap allocation** for up to 16 entries (inline `SmallVec`)
- **Small-string optimization** via `SmolStr` (≤22 bytes)
- Binary search lookup (keys kept sorted on insert)

### `event` — Source Event and Batch

```rust
use std::sync::Arc;
use wp_source_types::{SourceEvent, Tags};
use wp_model_core::raw::RawData;

let tags = Arc::new(Tags::new());
let event = SourceEvent::new(
    42,           // event_id
    "tcp_src",    // src_key
    RawData::from_string("payload"),
    tags,
);
```

- `SourceEvent` — single event with payload, tags, and optional preprocessing hook
- `SourceBatch = Vec<SourceEvent>` — batch of events for bulk delivery
- `EventPreHook` — type alias for preprocessing closures

## Usage in the Connector Ecosystem

```
wp-source-types           ← Tags, SourceEvent (zero trait dependencies)
    ↑
wp-connector-api          ← Re-exports + defines DataSource trait, SinkFactory, etc.
    ↑
wp-connector-utils / wp-core-connectors / wp-connectors
```

`wp-source-types` sits at the bottom layer with no dependencies on any connector API traits, making it safe for use in any crate without risk of circular dependencies.

## License

Apache-2.0
