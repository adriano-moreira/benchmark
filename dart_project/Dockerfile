FROM dart:stable AS build

WORKDIR /app
COPY pubspec.* ./
RUN dart pub get

COPY . .
RUN dart pub get --offline
RUN dart compile exe bin/dart_project.dart -o bin/dart_project

FROM ubuntu
WORKDIR /app
COPY --from=build /app/bin/dart_project /app/bin/

ENTRYPOINT ["./bin/dart_project"]
