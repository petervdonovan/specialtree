#!/usr/bin/env nu

# assumes you have done "cargo install cargo-depgraph" and have graphviz installed

cargo depgraph --workspace-only --dedup-transitive-deps | dot -Tpng | save -f depgraph.png
