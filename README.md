# TermGloss

This software and everything else in this repository is fully human-made. No LLM
was used for any of it.

-----

TermGloss is a simple Rust program that collects all
[`/-terms`](https://typst.app/docs/reference/model/terms/) from a given set of
typst documents into an alphabetically sorted glossary in a separate file.

## Usage

Install via cargo:

```bash
cargo install --git https://www.github.com/speicherwerk/termgloss
```

And then use as follows:

```bash
Usage: termgloss --output <OUTPUT> <INPUTS>...

Arguments:
  <INPUTS>...  Where to find the /-terms

Options:
  -o, --output <OUTPUT>  Where to write the glossary to. NOTE: This file will get created or truncated (all previous contents deleted)!
  -h, --help             Print help
  -V, --version          Print version

```

## Recommended Workflow

Have a `Makefile` build the glossary before your main document and `#include`
the glossary, for example:

```Makefile
index.pdf: index.typ
    termgloss -o glossary.typ index.typ
    typst compile index.typ
```
