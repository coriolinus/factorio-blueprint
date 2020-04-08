# factorio-blueprint

Library for reading and writing factorio blueprints. See https://wiki.factorio.com/Blueprint_string_format

Includes a binary which can convert from JSON to a blueprint string and back. Strictly speaking, this binary will be built in all cases, but to minimize compilation time and dependency load, it only does anything on an opt-in basis. To opt into building the binary such that it actually does anything, build with `--features codec-cli`.
