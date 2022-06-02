# @magnifique/tools-builder-handler

This is a handler for the magnifique **backend**, which can compress the backend file into a single file.

The handler is written by `JavaScript`, which is based on `node`.

## Realizing Steps

> ### YOU MUST KNOW
>
> This app's compiling has been included in the [action](/.github/workflows/build.yml)

1. Create a folder named `backend` in `dist`;

2. Copy the builded file in `target/release/server.exe` to `dist/backend/server.exe`, while copy the builded file in `/socket.exe` to `dist/backend/socket.exe`;

3. Copy the `README` file to `dist/backend/README.md`;

4. Download `mongodb` client into `dist/backend/mongodb`;

5. Create the compressed file in `dist/backend.zip` and `dist/backend.tar.gz`;

6. Publish it into `GitHub` release.
