mlml - multi-line markup language
=================================

Adds line continuation for strings to HTML/XML files.

This was partially to learn more about programming in rust and partially
because long URLs in HTML keep ruining my 80 character width limit.

# Overview

Take the following example .mlml snippet:

```mlml
<script src="https://example.org/some_really/" +
            "long_url/">
</script>
```

This program will detect the `+` as a line continuation of the src attribute
value and concatenate the two lines into one:

```html
<script src="https://example.org/some_really/long_url/">
</script>
```

It ignores the contents (innerHTML) of `<script>` and `<style>` tags to
prevent concatenating valid line continuations in JavaScript (for example).

# Downloading and compiling

Download source:

```bash
git clone https://github.com/briansteffens/mlml
cd mlml
```

Run tests:

```bash
cargo test
```

Compile and install:

```bash
make
sudo make install
```

# Usage

Process the included example file, which will convert `example.mlml` into
`example.html`.

```bash
mlml example.mlml
```

Pipe a file in over stdio and show the output in stdout:

```bash
cat example.mlml | mlml
```
