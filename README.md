# mdbook-include

A [mdbook](https://github.com/rust-lang/mdBook) preprocessor that include raw github files as code blocks in your book.

## Installation

Grab the binary from the release page and place it somewher in your `$PATH`.

Add it as a preprocessor to your `book.toml`:

```toml
[book]
title = "Example book"
author = "John Doe"

[preprocessor.include]
```

Finally, build your book as normal:

```
mdbook build path/to/book/
```

## Usage

Place the link for raw github file in any of the markdown files int the book after `!!!!include`:

```
!!!!include https://raw.githubusercontent.com/threefoldtech/mdbook-include/development/src/lib.rs
```

Then build the book using mdbook.