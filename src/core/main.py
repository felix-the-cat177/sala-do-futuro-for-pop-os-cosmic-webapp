"""
Sala do Futuro - Aplicativo Principal

Aplicativo web nativo para COSMIC e Pop!_OS com integração profunda,
navegador Chromium com Vulkan e sistema de notificações.
"""

import sys
import logging
from pathlib import Path

# Configuração básica de logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class SalaDoFuturoApp:
    """Classe principal do aplicativo Sala do Futuro."""
    
    def __init__(self):
        self.name = "Sala do Futuro"
        self.version = "1.0.0"
        self.description = "WebApp especializado para COSMIC e Pop!_OS"
        logger.info(f"Inicializando {self.name} v{self.version}")
    
    def run(self):
        """Executa o aplicativo."""
        logger.info("Iniciando aplicação...")
        print(f"🚀 {self.name} - Pronto para uso!")
        print(f"   Versão: {self.version}")
        print(f"   Integração: COSMIC Desktop")
        print(f"   Navegador: Chromium com Vulkan")
        return 0
    
    def stop(self):
        """Para o aplicativo gracefully."""
        logger.info("Finalizando aplicação...")
        print("👋 Até logo!")


def main():
    """Função entry point do aplicativo."""
    app = SalaDoFuturoApp()
    try:
        exit_code = app.run()
        return exit_code
    except KeyboardInterrupt:
        print("\n⚠️  Interrompido pelo usuário")
        app.stop()
        return 130
    except Exception as e:
        logger.error(f"Erro crítico: {e}")
        return 1
    finally:
        app.stop()


if __name__ == "__main__":
    sys.exit(main())
