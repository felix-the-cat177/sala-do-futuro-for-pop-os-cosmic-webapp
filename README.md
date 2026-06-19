# Sala do Futuro - WebApp para COSMIC e Pop!_OS

Um fork especializado do "WebApps for COSMIC" desenvolvido exclusivamente para a **Sala do Futuro**.

## 🚀 Características Principais

- **Navegador Integrado**: Baseado em Chromium com API Vulkan e aceleração de hardware para máxima performance.
- **Integração Profunda com COSMIC**: Utiliza `cosmic-lib` para integração nativa com o ambiente desktop.
- **Painel de Aplicações**: Logo da Sala do Futuro rodando em segundo plano no painel, recebendo notificações (tarefas, redações, provas).
- **Daemon de Notificações**: Serviço separado para gerenciar alertas em tempo real.
- **Distribuição**: Pacote `.deb` pronto para instalação em sistemas baseados em Debian/Ubuntu.

## 📁 Estrutura do Projeto

```
sala-do-futuro-for-pop-os-cosmic-webapp/
├── src/
│   ├── core/           # Lógica principal do aplicativo
│   ├── daemon/         # Serviço de notificações em segundo plano
│   └── assets/         # Logos, ícones e recursos visuais
├── docs/               # Documentação técnica e de usuário
├── scripts/            # Scripts de build e instalação
├── debian/             # Configuração para empacotamento .deb
└── README.md
```

## 🛠️ Tecnologias

- **Linguagem**: Python 3.10+
- **Framework GUI**: COSMIC Lib (Rust bindings via PyO3 ou GTK4/libadwaita)
- **Navegador**: Chromium customizado com suporte a Vulkan
- **Empacotamento**: DEB (dpkg-deb)

## 📦 Instalação (Futura)

```bash
sudo apt install ./sala-do-futuro-webapp_1.0.0_amd64.deb
```

## 🔧 Desenvolvimento

### Pré-requisitos
- Python 3.10+
- Rust (para compilação de bindings do COSMIC)
- libcosmic-dev
- chromium-browser com suporte a Vulkan

### Rodar localmente
```bash
python -m src.core.main
```

## 📄 Licença

Este projeto é um fork adaptado para necessidades específicas da Sala do Futuro.

---

**Desenvolvido por**: felix-the-cat177  
**Projeto**: Sala do Futuro for Pop!_OS COSMIC
