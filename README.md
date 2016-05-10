mlml - multi-line markup language
=================================

Adds line continuation for strings to HTML/XML files.

This was partially to learn more about programming in rust and partially
because long URLs in HTML keep ruining my 80 character width limit.

# Downloading and compiling

Download source and compile manually:

```bash
git clone https://github.com/briansteffens/mlml
cd mlml
cargo build
```

Run tests:

```bash
cargo test
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
