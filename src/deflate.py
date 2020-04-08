#!/usr/bin/env python3

import json
from base64 import b64decode
from pathlib import Path
from zlib import decompress


def unpack(path):
    with open(path) as f:
        data = f.read()
    # clear version byte and trailing whitespace
    if not data.startswith("0"):
        raise Exception("unexpected version byte")
    data = data[1:].rstrip()

    # un-base64
    data = b64decode(data)

    # unzip
    data = decompress(data)

    # load json
    return json.loads(data)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()

    parser.add_argument("path", type=Path)

    args = parser.parse_args()

    print(json.dumps(unpack(args.path), indent=2))
