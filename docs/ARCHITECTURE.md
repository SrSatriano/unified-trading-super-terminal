# Arquitetura

```
┌──────────┐     ┌─────────────┐     ┌────────────┐
│   UI     │────►│  App State  │◄────│ Connectors │
│ Ratatui  │     │  (Arc<RwLock>)│    │ REST / FIX │
└──────────┘     └──────┬──────┘     └────────────┘
                        │
                 ┌──────▼──────┐
                 │ Risk Engine │
                 └─────────────┘
```

Event loop assíncrono com `tokio`. UI renderiza a 30 FPS; dados de mercado atualizam via channels.
