#!/bin/bash
cargo run -q -- ocaml ~/dev/misc/arrow/format/*.fbs |
ocamlformat --impl --enable-outside-detected-project --profile=janestreet -
