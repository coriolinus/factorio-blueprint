# factorio-blueprint

Library for reading and writing factorio blueprints. See https://wiki.factorio.com/Blueprint_string_format

The `objects` module contains typed definitions for all of the objects and types defined on that wiki page. All fields are public. While these objects are currently light on helper methods, it's still straightforward to construct arbitary blueprints.

The `Container` enum is the primary entry point to the library: it has variants for each of the top-level blueprint items, and has convenience methods for conversion to and from blueprint string format.

## CLI

When built with `--features codec-cli`, this produces a `factorio-blueprint` executable, which is strictly a codec: it converts from blueprint strings to json, and vice-versa. It can read its inputs from a file, from the command line, or from stdin; it always writes to stdout. This enables some relatively sophisticated manipulations using nothing but the command line. For example, to remove all belts from a blueprint:

```sh
$ factorio-blueprint decode --file tests/examples/super_compact_tileable_mining.txt |\
  jq -c '.blueprint.entities |= map(select((has("name") | not) or (.name | contains("belt") | not)))' |\
  factorio-blueprint encode
0eNq1mttqg0AYhO/7GHttwD3p6mVfo5SSw9IurGswpjQE372atCWUEHCSuQpxo37ujP/OrzmKVdz7bRdSL+qjCOs27UT9chS78J6WcdrWH7Ze1CL0vhGZSMtm+uajX/ddWC+akEJ6X2y6EKMYMhHSxn+JWg7Z3cdQw2smfOpDH/wZ6uaOmdi2u/G3bZrOOO6/kDYTh+kzHw+6Cd2412lUZf+AxqtO58HxPMfhtKHv2vi28h/Lz9B2560nlsNb2jcr3/1c41wkeR2peAySApDcdaL8MUQaIOLKZhDZqKrZ+USKKlqBTJE+I5UUohIgogI5ZIryq0QP8nU1n4jKM3lz9hQVV4kedJ9JoGIbKhBSr39XNcNRTeNIikNk7qhGknLzS4sjWQ5Rga/8hkNUwkSKA+TwNYRkowom4rhI5XAS4ZhISRSI4yGl7sj8hhP5NY6kOER4wiYBWTzyk0Qr4GxkOUAlHtYkh8jhnSMnGqnqjswvKUg6x5Esh0jCsnHio1ZwX0TSTKNAJMUM2hVxSpG2cJfGqUS6wFsix5GsxNd8x5kjB6d9x3nuWMF1iKOZyeEQwpHMwNmao5i5LNSN34R9s/jD2rbR32rzOQ2a0fORyERmNtFPg0YSzc6fIUcFKnAXcZpqU8ImIgE51EOcptpUsIc4QDaHPcS5661ELUTiUaCDOE+JrEYNROIxsH84L6usRf1D4immV/unQ9QXf0DIxKfvdr8nisuVj+P489/4MDx9AxAgLaY=
```
