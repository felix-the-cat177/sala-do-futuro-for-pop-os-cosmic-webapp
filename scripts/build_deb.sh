#!/bin/bash
# Script de build para empacotamento .deb do Sala do Futuro

set -e

echo "🔨 Iniciando build do Sala do Futuro..."

# Variáveis
APP_NAME="sala-do-futuro-webapp"
VERSION="1.0.0"
ARCH="amd64"

# Criar estrutura do pacote Debian
DEB_DIR="debian_package"
rm -rf $DEB_DIR
mkdir -p $DEB_DIR/DEBIAN
mkdir -p $DEB_DIR/opt/$APP_NAME
mkdir -p $DEB_DIR/usr/share/applications
mkdir -p $DEB_DIR/usr/share/icons/hicolor/scalable/apps
mkdir -p $DEB_DIR/usr/bin

# Copiar arquivos da aplicação
cp -r src/* $DEB_DIR/opt/$APP_NAME/
cp ola_mundo.py $DEB_DIR/opt/$APP_NAME/ 2>/dev/null || true

# Criar script de controle do Debian
cat > $DEB_DIR/DEBIAN/control << EOF
Package: $APP_NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Depends: python3, chromium-browser, libcosmic-dev
Maintainer: felix-the-cat177
Description: Sala do Futuro WebApp for COSMIC and Pop!_OS
 Aplicativo web nativo para integração com o ambiente COSMIC.
 Inclui navegador Chromium com Vulkan e sistema de notificações.
EOF

# Criar script de instalação
cat > $DEB_DIR/DEBIAN/postinst << EOF
#!/bin/bash
echo "Configurando Sala do Futuro..."
update-desktop-database
echo "Instalação concluída!"
EOF
chmod +x $DEB_DIR/DEBIAN/postinst

# Criar desktop entry
cat > $DEB_DIR/usr/share/applications/sala-do-futuro.desktop << EOF
[Desktop Entry]
Name=Sala do Futuro
Comment=WebApp especializado para COSMIC e Pop!_OS
Exec=/opt/$APP_NAME/main.py
Icon=sala-do-futuro
Terminal=false
Type=Application
Categories=Education;Network;
StartupNotify=true
EOF

# Criar wrapper executável
cat > $DEB_DIR/usr/bin/sala-do-futuro << EOF
#!/bin/bash
python3 /opt/$APP_NAME/core/main.py "\$@"
EOF
chmod +x $DEB_DIR/usr/bin/sala-do-futuro

# Construir o pacote .deb
echo "📦 Construindo pacote .deb..."
cd $DEB_DIR
dpkg-deb --build . ../${APP_NAME}_${VERSION}_${ARCH}.deb
cd ..

echo "✅ Build concluído!"
echo "📁 Pacote disponível: ${APP_NAME}_${VERSION}_${ARCH}.deb"
echo ""
echo "Para instalar:"
echo "  sudo apt install ./${APP_NAME}_${VERSION}_${ARCH}.deb"
