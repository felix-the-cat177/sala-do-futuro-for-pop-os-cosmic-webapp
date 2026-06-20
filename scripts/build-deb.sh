#!/bin/bash
# Script para construir o pacote .deb da Sala do Futuro com suporte COSMIC

set -e

APP_NAME="sala-do-futuro-webapp"
VERSION="1.1.0"
ARCH="amd64"

echo "Construindo $APP_NAME versão $VERSION..."
echo "Incluindo: sala-do-futuro-webapp, cosmic-panel-item, notification-daemon"
echo ""

# Build em release mode
echo "Compilando..."
cargo build --release 2>&1 | tail -5

# Criar estrutura do pacote Debian
DEB_DIR="debian-package"
rm -rf $DEB_DIR
mkdir -p $DEB_DIR/DEBIAN
mkdir -p $DEB_DIR/usr/bin
mkdir -p $DEB_DIR/usr/share/applications
mkdir -p $DEB_DIR/usr/share/metainfo
mkdir -p $DEB_DIR/usr/share/icons/hicolor/scalable/apps
mkdir -p $DEB_DIR/usr/share/sala-do-futuro-webapp/i18n
mkdir -p $DEB_DIR/lib/systemd/user

# Copiar binários principais (versão 1.1.0 com integração COSMIC)
echo "Copiando binários..."
cp target/release/$APP_NAME $DEB_DIR/usr/bin/
cp target/release/cosmic-panel-item $DEB_DIR/usr/bin/
cp target/release/sala-do-futuro-notification-daemon $DEB_DIR/usr/bin/

# Copiar binários auxiliares
cp target/release/sala-do-futuro-webview $DEB_DIR/usr/bin/ 2>/dev/null || true
cp target/release/sala-do-futuro-webview-helper $DEB_DIR/usr/bin/ 2>/dev/null || true

# Copiar serviços systemd
cp systemd/sala-do-futuro-notification.service $DEB_DIR/lib/systemd/user/ 2>/dev/null || true
cp systemd/sala-do-futuro-webview.service $DEB_DIR/lib/systemd/user/ 2>/dev/null || true

# Copiar arquivos de localização
echo "Copiando arquivos de localização..."
cp -r i18n/* $DEB_DIR/usr/share/sala-do-futuro-webapp/i18n/ 2>/dev/null || true

# Copiar arquivos de desktop e metainfo
echo "Copiando recursos..."
cp resources/dev.heppen.webapps.desktop $DEB_DIR/usr/share/applications/sala-do-futuro-webapp.desktop 2>/dev/null || true
cp resources/dev.heppen.webapps.metainfo.xml $DEB_DIR/usr/share/metainfo/sala-do-futuro-webapp.metainfo.xml 2>/dev/null || true

# Copiar ícones
for size in 16x16 24x24 32x32 48x48 64x64 128x128 256x256; do
    mkdir -p $DEB_DIR/usr/share/icons/hicolor/$size/apps
    cp resources/icons/hicolor/$size/apps/sala-do-futuro-webapp.svg $DEB_DIR/usr/share/icons/hicolor/$size/apps/ 2>/dev/null || true
done

# Criar arquivo de controle
cat > $DEB_DIR/DEBIAN/control << EOF
Package: sala-do-futuro-webapp
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Depends: libgtk-3-0, libwebkit2gtk-4.0-37
Recommends: libcosmic0
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
