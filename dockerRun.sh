#!/bin/bash
set -ex

docker run --rm my/dart_project hello_world

docker run --rm my/rust_project hello_world

docker run --rm my/csharp_project hello_world
