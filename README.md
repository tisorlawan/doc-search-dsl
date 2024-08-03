# Doc Search DSL (doc_search_dsl)

A Rust procedural macro for creating complex regex patterns using a Domain-Specific Language (DSL).
This DSL allows you to define hierarchical regex patterns that can be used with a custom Rule enum for advanced text matching and processing.

## Installation

```
cargo add doc_search_dsl lazy_regex
```

## Usage

Here's a basic example of how to use the DSL:

```rust
use doc_search_dsl::{pat, Rule};

fn main() {
    let content = vec![
        "Once upon a midnight dreary, while I pondered, weak and weary,",
        "Over many a quaint and curious volume of forgotten lore—",
        "While I nodded, nearly napping, suddenly there came a tapping,",
        "As of some one gently rapping, rapping at my chamber door.",
        "''Tis some visitor,' I muttered, 'tapping at my chamber door—",
        "Only this and nothing more.",
    ];

    let p = pat! {
        all {
            "nodded",
            "WHILE"i,
            "nothing"
        }
    };

    assert!(matches!(p, Rule::And(_)));
    assert_eq!(p.occurances(&content), 1);
}
```

This example creates a pattern that matches if all of "nodded", "WHILE" (case-insensitive), and "nothing" are present in the content.

## Pattern Types

Let's explore different pattern types, from simple to complex:

### 1. Single Pattern

The simplest pattern matches a single string:

```rust
let p = pat!("dreary");
assert!(matches!(p, Rule::One(_)));
assert_eq!(p.occurances(&content), 1);
```

This matches any line containing "dreary".

### 2. Single Pattern with Flags

You can add flags to modify the pattern behavior:

```rust
let p = pat!("WHILE"i);
assert!(matches!(p, Rule::One(_)));
assert_eq!(p.occurances(&content), 2);
```

The `i` flag makes the pattern case-insensitive, so it matches both "While" and "while".

### 3. Any (OR) Pattern

The `any` pattern matches if any of its sub-patterns match:

```rust
let p = pat! {
    any {
        "dreary",
        "curious",
        "tapping"
    }
};
assert!(matches!(p, Rule::Or(_)));
assert_eq!(p.occurances(&content), 3);
```

This matches lines containing "dreary", "curious", or "tapping".

### 4. All (AND) Pattern

The `all` pattern matches if all of its sub-patterns match:

```rust
let p = pat! {
    all {
        "while",
        "pondered",
        "weary"
    }
};
assert!(matches!(p, Rule::And(_)));
assert_eq!(p.occurances(&content), 1);
```

This matches only if "while", "pondered", and "weary" are all present in the content.

### 5. Sequence Pattern

The `sequence` pattern matches its sub-patterns in order:

```rust
let p = pat! {
    sequence {
        "Once upon",
        "midnight",
        "dreary"
    }
};
assert!(matches!(p, Rule::Sequence(_)));
assert_eq!(p.occurances(&content), 1);
```

This matches the patterns "Once upon", "midnight", and "dreary" in that specific order.

### 6. Nested Patterns

You can nest patterns for more complex matching:

```rust
let p = pat! {
    any {
        all {
            "dreary",
            "weary"
        },
        sequence {
            "tapping",
            "rapping"
        }
    }
};
assert!(matches!(p, Rule::Or(_)));
assert_eq!(p.occurances(&content), 2);
```

This matches either (both "dreary" and "weary") or ("tapping" followed by "rapping").

### 7. Complex Pattern

Here's a more complex example combining various pattern types:

```rust
let p = pat! {
    all {
        any {
            "dreary",
            "curious"
        },
        sequence {
            "while"i,
            pat!("I \\w+")  // Matches "I" followed by any word
        },
        pat!(".*more.$")  // Matches lines ending with "more."
    }
};
assert!(matches!(p, Rule::And(_)));
assert_eq!(p.occurances(&content), 1);
```

This pattern matches content that:

1. Contains either "dreary" or "curious", AND
2. Has "while" (case-insensitive) followed by "I" and any word, AND
3. Has a line ending with "more."

## Conclusion

This DSL provides a flexible and intuitive way to create complex regex patterns in Rust. By combining different pattern types and nesting them, you can create powerful search patterns tailored to your specific needs.

For more detailed information on available methods and advanced usage, please refer to the API documentation.
