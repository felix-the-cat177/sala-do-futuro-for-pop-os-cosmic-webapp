# Sala do Futuro - COSMIC Integration Architecture

## 🏗️ Visão Geral

A aplicação agora possui uma arquitetura integrada com o COSMIC Desktop com suporte completo a:
- ✅ Navegador Chromium integrado com Vulkan
- ✅ Painel do COSMIC com widget/logo
- ✅ Sistema de notificações via D-Bus
- ✅ Execução em background com systemd
- ✅ Auto-launch quando clicado no painel

## 🔧 Componentes

### 1. **sala-do-futuro-webapp** (App Principal)
- Aplicativo COSMIC com interface web
- Modo background (`--background`)
- Renderização com Vulkan + fallback OpenGL
- Integração com libcosmic

**Binário**: `sala-do-futuro-webapp`

```bash
# Modo normal
sala-do-futuro-webapp

# Modo background (systemd)
sala-do-futuro-webapp --background
```

### 2. **cosmic-panel-item** (Widget no Painel)
- Integra no painel superior do COSMIC
- Logo/ícone da Sala do Futuro
- Indicador de notificações (ponto/cor)
- Click para abrir app (launch behavior)

**Binário**: `cosmic-panel-item`

### 3. **sala-do-futuro-notification-daemon** (Daemon de Notificações)
- Monitora notificações HTML5 do navegador
- Monitora custom events da Sala do Futuro
- Se comunica via D-Bus com COSMIC
- Roda como serviço systemd

**Binário**: `sala-do-futuro-notification-daemon`

**Serviço D-Bus**: `org.saladofuturo.NotificationDaemon`

### 4. **systemd User Services**
Dois serviços de usuário para autostart:

#### `sala-do-futuro-notification.service`
- Inicia o daemon de notificações
- Roda em background automaticamente
- Monitora eventos

#### `sala-do-futuro-webview.service`
- Inicia o app em background
- Configura Vulkan/OpenGL
- Mantém conectado ao D-Bus

## 📦 Instalação

### 1. Compilar
```bash
cd /workspaces/sala-do-futuro-for-pop-os-cosmic-webapp
cargo build --release
```

### 2. Instalar Binários
```bash
mkdir -p ~/.local/bin

# Copiar binários
cp target/release/sala-do-futuro-webapp ~/.local/bin/
cp target/release/cosmic-panel-item ~/.local/bin/
cp target/release/sala-do-futuro-notification-daemon ~/.local/bin/

chmod +x ~/.local/bin/sala-do-futuro-*
```

### 3. Instalar Serviços Systemd
```bash
mkdir -p ~/.config/systemd/user

# Copiar serviços
cp systemd/*.service ~/.config/systemd/user/

# Recarregar daemon
systemctl --user daemon-reload

# Ativar autostart
systemctl --user enable sala-do-futuro-notification.service
systemctl --user enable sala-do-futuro-webview.service

# Iniciar serviços
systemctl --user start sala-do-futuro-notification.service
systemctl --user start sala-do-futuro-webview.service
```

## 🚀 Uso

### Iniciação Automática
Os serviços vão iniciar automaticamente ao fazer login no COSMIC:

```bash
# Verificar status
systemctl --user status sala-do-futuro-notification.service
systemctl --user status sala-do-futuro-webview.service

# Ver logs
journalctl --user -u sala-do-futuro-notification.service -f
journalctl --user -u sala-do-futuro-webview.service -f
```

### Executar Manualmente
```bash
# App com interface
sala-do-futuro-webapp

# Daemon de notificações
sala-do-futuro-notification-daemon

# Widget do painel
cosmic-panel-item
```

## 🎨 Renderização (Vulkan + OpenGL)

### Variáveis de Ambiente
```bash
# Forçar Vulkan
export ENABLE_VULKAN=1
export OZONE_PLATFORM=wayland

# Desabilitar software rendering
export LIBGL_ALWAYS_SOFTWARE=0

# Drivers GPU
export LIBGBM_DRIVERS=nouveau,radeonsi,iris,amdgpu,swrast
```

### Fallback Automático
- Tenta Vulkan primeiro
- Se não disponível, usa OpenGL
- Se nem OpenGL, usa SWRAST (software)

## 🔔 Notificações

### D-Bus Interface
```
Interface: org.saladofuturo.NotificationDaemon
Path: /org/saladofuturo/NotificationDaemon

Methods:
  - notify(app: String, title: String, body: String) -> u32
  - close_notification(id: u32) -> ()

Properties:
  - has_notification: Boolean
```

### Testar Notificação
```bash
# Via dbus-send
dbus-send --session \
  --dest=org.saladofuturo.NotificationDaemon \
  --print-reply \
  /org/saladofuturo/NotificationDaemon \
  org.saladofuturo.NotificationDaemon.notify \
  string:"app" \
  string:"Título" \
  string:"Corpo da mensagem"
```

## 🐛 Debug

### Logs Detalhados
```bash
# Habilitar logs
export RUST_LOG=debug

# Executar com backtrace
export RUST_BACKTRACE=1
sala-do-futuro-webapp

# Ver journalctl com mais detalhes
journalctl --user -u sala-do-futuro-* -n 100 --follow
```

### Verificar Vulkan
```bash
# Checar se Vulkan está disponível
vulkaninfo

# Teste de renderização
glxinfo | grep "OpenGL version"
```

## 📋 Estrutura de Arquivos

```
~/.config/systemd/user/
├── sala-do-futuro-notification.service
└── sala-do-futuro-webview.service

~/.local/bin/
├── sala-do-futuro-webapp
├── cosmic-panel-item
└── sala-do-futuro-notification-daemon

~/.cache/sala-do-futuro/
└── (cache de navegador)

~/.local/share/sala-do-futuro-webapp/
├── config.ron
└── i18n/
    ├── en/
    ├── pt-BR/
    └── ...
```

## 🤝 Integração COSMIC

### Painel Superior
- O widget `cosmic-panel-item` aparece no painel
- Clique: abre Sala do Futuro
- Indicador: mostra notificações ativas

### Notificações
- Integração com D-Bus
- COSMIC mostra notificações da Sala do Futuro
- Sincronização automática

### Temas
- Segue tema do COSMIC (claro/escuro)
- Customização via cosmic-config

## 📝 Troubleshooting

### Serviço não inicia
```bash
# Verificar sintaxe
systemd-analyze verify ~/.config/systemd/user/sala-do-futuro-notification.service

# Ver erro completo
systemctl --user status sala-do-futuro-notification.service
```

### Vulkan não funciona
```bash
# Verificar drivers
vulkaninfo | grep "Device Type\|Device Name"

# Testar com OpenGL fallback
unset ENABLE_VULKAN
sala-do-futuro-webapp
```

### Notificações não funcionam
```bash
# Verificar D-Bus
dbus-send --session --print-reply --dest=org.freedesktop.DBus \
  /org/freedesktop/DBus org.freedesktop.DBus.ListNames

# Ver se daemon está rodando
systemctl --user status sala-do-futuro-notification.service
```

## 🔄 Atualizar

```bash
# Reconstruir
cargo build --release

# Atualizar binários
cp target/release/sala-do-futuro-* ~/.local/bin/

# Reiniciar serviços
systemctl --user restart sala-do-futuro-notification.service
systemctl --user restart sala-do-futuro-webview.service
```

## 📚 Referências

- [COSMIC Desktop](https://system76.com/cosmic)
- [libcosmic](https://github.com/pop-os/libcosmic)
- [CEF (Chromium Embedded Framework)](https://bitbucket.org/chromiumembedded/cef)
- [D-Bus](https://dbus.freedesktop.org/)
- [systemd User Services](https://wiki.archlinux.org/title/Systemd/User)
