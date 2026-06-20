#!/bin/bash
# Script de setup para integração COSMIC
# Instala binários e serviços systemd

set -e

APP_NAME="sala-do-futuro-webapp"
VERSION="1.1.0"
BIN_DIR="$HOME/.local/bin"
SYSTEMD_DIR="$HOME/.config/systemd/user"
SHARE_DIR="$HOME/.local/share/$APP_NAME"

echo "=========================================="
echo "Sala do Futuro - COSMIC Integration Setup"
echo "=========================================="
echo ""

# Verificar se os binários foram compilados
if [ ! -f "target/release/$APP_NAME" ]; then
    echo "❌ Binários não encontrados!"
    echo "Execute: cargo build --release"
    exit 1
fi

# Criar diretórios
echo "📁 Criando diretórios..."
mkdir -p "$BIN_DIR"
mkdir -p "$SYSTEMD_DIR"
mkdir -p "$SHARE_DIR"

# Copiar binários
echo "📦 Instalando binários..."
cp target/release/$APP_NAME "$BIN_DIR/" && echo "  ✓ $APP_NAME"
cp target/release/cosmic-panel-item "$BIN_DIR/" && echo "  ✓ cosmic-panel-item"
cp target/release/sala-do-futuro-notification-daemon "$BIN_DIR/" && echo "  ✓ sala-do-futuro-notification-daemon"

chmod +x "$BIN_DIR"/$APP_NAME
chmod +x "$BIN_DIR"/cosmic-panel-item
chmod +x "$BIN_DIR"/sala-do-futuro-notification-daemon

# Copiar serviços systemd
echo ""
echo "🔧 Instalando serviços systemd..."
cp systemd/sala-do-futuro-notification.service "$SYSTEMD_DIR/" && echo "  ✓ sala-do-futuro-notification.service"
cp systemd/sala-do-futuro-webview.service "$SYSTEMD_DIR/" && echo "  ✓ sala-do-futuro-webview.service"

# Recarregar daemon
echo ""
echo "🔄 Recarregando systemd daemon..."
systemctl --user daemon-reload

# Perguntar se quer ativar autostart
echo ""
echo "⚙️  Configurar autostart?"
read -p "Ativar serviços no login? (s/N): " enable_autostart

if [[ $enable_autostart =~ ^[Ss]$ ]]; then
    echo ""
    echo "✅ Ativando autostart..."
    systemctl --user enable sala-do-futuro-notification.service
    systemctl --user enable sala-do-futuro-webview.service
    
    echo ""
    echo "🚀 Iniciando serviços..."
    systemctl --user start sala-do-futuro-notification.service
    systemctl --user start sala-do-futuro-webview.service
    
    echo ""
    echo "✓ Serviços iniciados!"
    echo ""
    echo "Verificar status:"
    echo "  systemctl --user status sala-do-futuro-notification.service"
    echo "  systemctl --user status sala-do-futuro-webview.service"
else
    echo ""
    echo "Para ativar manualmente:"
    echo "  systemctl --user enable sala-do-futuro-notification.service"
    echo "  systemctl --user enable sala-do-futuro-webview.service"
    echo "  systemctl --user start sala-do-futuro-notification.service"
    echo "  systemctl --user start sala-do-futuro-webview.service"
fi

echo ""
echo "=========================================="
echo "✅ Setup Completo!"
echo "=========================================="
echo ""
echo "Executar aplicação:"
echo "  sala-do-futuro-webapp"
echo ""
echo "Ver logs:"
echo "  journalctl --user -u sala-do-futuro-* -f"
echo ""
echo "Mais informações:"
echo "  cat COSMIC_INTEGRATION.md"
echo ""
