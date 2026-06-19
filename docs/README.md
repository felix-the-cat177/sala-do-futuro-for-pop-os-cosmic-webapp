# Documentação do Sala do Futuro

## Visão Geral

O **Sala do Futuro** é um fork especializado do "WebApps for COSMIC", desenvolvido exclusivamente para fornecer uma experiência integrada e otimizada no ambiente Pop!_OS com desktop COSMIC.

## Arquitetura

### Componentes Principais

1. **Aplicativo Principal (`src/core/`)**
   - Interface principal do usuário
   - Gerenciamento de janelas COSMIC
   - Integração com navegador Chromium

2. **Daemon de Notificações (`src/daemon/`)**
   - Serviço em segundo plano
   - Monitoramento de tarefas, redações e provas
   - Integração com sistema de notificações do COSMIC

3. **Assets (`src/assets/`)**
   - Logos e ícones da Sala do Futuro
   - Imagens de fundo para o painel

## Funcionalidades

### Navegador Integrado
- Baseado em Chromium
- API Vulkan para aceleração de hardware
- Otimizado para performance máxima

### Integração COSMIC
- Uso da `cosmic-lib` para integração nativa
- Ícone no painel com logo da Sala do Futuro
- Notificações em tempo real no sistema

### Sistema de Notificações
- Alertas de tarefas pendentes
- Lembretes de prazos de redação
- Avisos de provas e avaliações
- Comunicados importantes

## Estrutura de Diretórios

```
sala-do-futuro-for-pop-os-cosmic-webapp/
├── src/
│   ├── core/              # Aplicativo principal
│   │   ├── __init__.py
│   │   └── main.py
│   ├── daemon/            # Serviços em background
│   │   ├── __init__.py
│   │   └── notification_service.py
│   └── assets/            # Recursos visuais
├── scripts/               # Scripts de build
│   └── build_deb.sh
├── docs/                  # Documentação
├── debian/                # Configuração do pacote .deb
└── README.md
```

## Desenvolvimento

### Pré-requisitos

- Python 3.10+
- Rust (para bindings do COSMIC)
- libcosmic-dev
- chromium-browser com suporte a Vulkan

### Rodando Localmente

```bash
# Aplicativo principal
python -m src.core.main

# Daemon de notificações (em outro terminal)
python -m src.daemon.notification_service
```

### Build do Pacote .deb

```bash
cd scripts
./build_deb.sh
```

O script gerará um arquivo `.deb` pronto para instalação.

## Instalação

Após o build:

```bash
sudo apt install ./sala-do-futuro-webapp_1.0.0_amd64.deb
```

## Próximos Passos

1. [ ] Implementar integração real com API da Sala do Futuro
2. [ ] Adicionar bindings Rust para cosmic-lib
3. [ ] Criar ícones e logos oficiais
4. [ ] Implementar navegador Chromium customizado
5. [ ] Configurar repositório PPA para distribuição fácil

## Licença

Este projeto é um fork adaptado para necessidades específicas da Sala do Futuro.

---

**Desenvolvido por**: felix-the-cat177  
**Versão**: 1.0.0
