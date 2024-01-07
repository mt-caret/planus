#!/bin/bash
cargo run -q -- ocaml ./arrow-format/*.fbs |
ocamlformat --impl --enable-outside-detected-project --profile=janestreet -
