"""
Sala do Futuro - Módulo Daemon

Serviços em segundo plano para notificações e sincronização.
"""

from .notification_service import NotificationDaemon, main

__all__ = ['NotificationDaemon', 'main']
__version__ = '1.0.0'
