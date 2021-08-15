#!/bin/bash
set -ex

cd dart_project
docker build -t my/dart_project .
cd ..


cd rust_project
docker build -t my/rust_project .
cd ..

cd CSharpProject
docker build -t my/csharp_project .
cd ..
