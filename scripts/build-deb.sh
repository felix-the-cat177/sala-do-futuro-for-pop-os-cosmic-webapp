#!/bin/bash
# Script para construir o pacote .deb da Sala do Futuro com suporte COSMIC

set -e

APP_NAME="sala-do-futuro-webapp"
VERSION="1.2.0"
ARCH="amd64"

echo "Construindo $APP_NAME versão $VERSION..."
echo "Incluindo: sala-do-futuro-webapp, cosmic-panel-item, notification-daemon"
echo ""

# Garantir que o Cargo.lock está sincronizado com o repositório
# Isso previne erros de "local changes would be overwritten" no git pull
if git diff --quiet Cargo.lock 2>/dev/null; then
    echo "Cargo.lock sincronizado ✅"
else
    echo "Restaurando Cargo.lock do repositório para evitar conflitos de dependência..."
    git checkout -- Cargo.lock
fi

# Build em release mode
echo "Compilando..."
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
mkdir -p $DEB_DIR/lib/systemd/user

# Copiar binários principais (versão 1.1.0 com integração COSMIC)
echo "Copiando binários..."
cp target/release/$APP_NAME $DEB_DIR/usr/bin/
cp target/release/cosmic-panel-item $DEB_DIR/usr/bin/
cp target/release/sala-do-futuro-notification-daemon $DEB_DIR/usr/bin/

# Copiar bibliotecas do CEF (necessárias para o navegador Chromium)
echo "Copiando bibliotecas CEF..."
mkdir -p $DEB_DIR/usr/share/cef
CEF_SRC_DIR=$(find target -name "cef_linux_x86_64" -type d | head -n 1)
if [ -n "$CEF_SRC_DIR" ] && [ -d "$CEF_SRC_DIR" ]; then
    cp -r $CEF_SRC_DIR/. $DEB_DIR/usr/share/cef/
else
    echo "⚠️  AVISO: Diretório do CEF (cef_linux_x86_64) não encontrado em target!"
fi

# Copiar os binários do webview e helper para o diretório do CEF com o nome do APP_ID esperado
echo "Instalando webview e helper no diretório do CEF..."
cp target/release/sala-do-futuro-webview $DEB_DIR/usr/share/cef/dev.heppen.webapps.webview 2>/dev/null || true
cp target/release/sala-do-futuro-webview-helper $DEB_DIR/usr/share/cef/dev.heppen.webapps.webview-helper 2>/dev/null || true

# Criar link simbólico para o webview em /usr/bin
echo "Criando link simbólico para o webview..."
mkdir -p $DEB_DIR/usr/bin
ln -sf ../share/cef/dev.heppen.webapps.webview $DEB_DIR/usr/bin/dev.heppen.webapps.webview

# Copiar serviços systemd
cp systemd/sala-do-futuro-notification.service $DEB_DIR/lib/systemd/user/ 2>/dev/null || true
cp systemd/sala-do-futuro-webview.service $DEB_DIR/lib/systemd/user/ 2>/dev/null || true

# Copiar arquivos de localização
echo "Copiando arquivos de localização..."
cp -r i18n/* $DEB_DIR/usr/share/sala-do-futuro-webapp/i18n/ 2>/dev/null || true

# Copiar arquivos de desktop e metainfo
echo "Copiando recursos..."
cp resources/dev.heppen.webapps.desktop $DEB_DIR/usr/share/applications/sala-do-futuro-webapp.desktop 2>/dev/null || true
cp resources/dev.heppen.webapps.applet.desktop $DEB_DIR/usr/share/applications/dev.heppen.webapps.applet.desktop 2>/dev/null || true
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
Depends: libgtk-3-0, libwebkit2gtk-4.1-0
Recommends: libcosmic0
Maintainer: felix-the-cat177
Description: Sala do Futuro - Web application for COSMIC and Pop!_OS
 Aplicativo web educacional para a Sala do Futuro
 Integrado com ambiente COSMIC
 Navegador Chromium com aceleracao de hardware e API Vulkan
EOF

# Ajustar permissões para evitar erros no dpkg-deb
echo "Ajustando permissões dos arquivos..."
find $DEB_DIR -type d -exec chmod 755 {} \;
find $DEB_DIR -type f -exec chmod 644 {} \;
chmod 755 $DEB_DIR/usr/bin/*
chmod 755 $DEB_DIR/usr/share/cef/dev.heppen.webapps.webview* 2>/dev/null || true

# Construir pacote .deb
cd $DEB_DIR
dpkg-deb --build . ../${APP_NAME}_${VERSION}_${ARCH}.deb
cd ..

echo "Pacote criado: ${APP_NAME}_${VERSION}_${ARCH}.deb"
