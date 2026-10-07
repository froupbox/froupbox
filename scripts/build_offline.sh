#!/bin/sh
set -e

cd "$(realpath "$(dirname "$0")"/..)"

ELECTRON_DIST="./electron_dist"
rm -rf "$ELECTRON_DIST"

npm install

OFFLINE_BUILD=1 npm run build

cp website/offline/* package.json dist/

PACKAGER_CMD=(
  npx @electron/packager dist froupbox --out="$ELECTRON_DIST"
)

"${PACKAGER_CMD[@]}" --platform=win32 --arch=x64
"${PACKAGER_CMD[@]}" --platform=linux --arch=x64

cd "$ELECTRON_DIST"

zip -r froupbox-windows-x64.zip froupbox-win32-x64
tar -cvJf froupbox-linux-x64.tar.xz froupbox-linux-x64