#!/bin/bash
# Script para construir o pacote .deb da Sala do Futuro

set -e

APP_NAME="sala-do-futuro-webapp"
VERSION="1.0.0"
ARCH="amd64"

echo "Construindo $APP_NAME versao $VERSION..."

# Build em release mode
cargo build --release

# Criar estrutura do pacote Debian
DEB_DIR="debian-package"
rm -rf $DEB_DIR
mkdir -p $DEB_DIR/DEBIAN
mkdir -p $DEB_DIR/usr/bin
mkdir -p $DEB_DIR/usr/share/applications
mkdir -p $DEB_DIR/usr/share/metainfo
mkdir -p $DEB_DIR/usr/share/icons/hicolor/scalable/apps
mkdir -p $DEB_DIR/usr/share/sala-do-futuro-webapp/i18n

# Copiar binario
cp target/release/$APP_NAME $DEB_DIR/usr/bin/ || true
cp target/release/${APP_NAME}.webview $DEB_DIR/usr/bin/ 2>/dev/null || true
cp target/release/${APP_NAME}-webview-helper $DEB_DIR/usr/bin/ 2>/dev/null || true

# Copiar arquivos de localização
cp -r i18n/* $DEB_DIR/usr/share/sala-do-futuro-webapp/i18n/ || true

# Copiar arquivos de desktop e metainfo
cp resources/dev.heppen.webapps.desktop $DEB_DIR/usr/share/applications/sala-do-futuro-webapp.desktop 2>/dev/null || true
cp resources/dev.heppen.webapps.metainfo.xml $DEB_DIR/usr/share/metainfo/sala-do-futuro-webapp.metainfo.xml 2>/dev/null || true

# Copiar icones
for size in 16x16 24x24 32x32 48x48 64x64 128x128 256x256; do
    cp resources/icons/hicolor/$size/apps/sala-do-futuro-webapp.svg $DEB_DIR/usr/share/icons/hicolor/$size/apps/ 2>/dev/null || true
done

# Criar arquivo de controle
cat > $DEB_DIR/DEBIAN/control << EOF
Package: sala-do-futuro-webapp
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Depends: libgtk-3-0, libwebkit2gtk-4.0-37, libcef-dev
Maintainer: felix-the-cat177
Description: Sala do Futuro - Web application for COSMIC and Pop!_OS
 Aplicativo web educacional para a Sala do Futuro
 Integrado com ambiente COSMIC
 Navegador Chromium com aceleracao de hardware e API Vulkan
EOF

# Construir pacote .deb
cd $DEB_DIR
dpkg-deb --build . ../${APP_NAME}_${VERSION}_${ARCH}.deb
cd ..

echo "Pacote criado: ${APP_NAME}_${VERSION}_${ARCH}.deb"
