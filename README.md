# Unified Trading Super-Terminal (TUI)

Interface de terminal unificada estilo Bloomberg Terminal: gerenciamento de risco, execução de ordens e monitoramento de carteira em uma única tela leve e responsiva.

## Stack

- **Rust** + [Ratatui](https://github.com/ratatui-org/ratatui)
- Integração REST (Binance, Bybit) e FIX (opcional)

## Preview

> Adicione `assets/demo.gif` após gravar a sessão do terminal.

```
┌─ Portfolio ─────────────┬─ Order Entry ───────────┐
│ PnL: +2.4%              │ Symbol: BTCUSDT         │
│ Exposure: 12%           │ Side: [B]uy [S]ell      │
├─ Risk ──────────────────┼─ Open Orders ───────────┤
│ VaR(95%): $1,240        │ ...                     │
└─────────────────────────┴─────────────────────────┘
```

## Atalhos de teclado

| Tecla | Ação |
|-------|------|
| `Tab` | Alternar painéis |
| `o` | Nova ordem |
| `x` | Cancelar ordem selecionada |
| `r` | Atualizar carteira |
| `q` | Sair |
| `?` | Ajuda |

Lista completa: [docs/KEYBINDINGS.md](docs/KEYBINDINGS.md)

## Variáveis de ambiente

```bash
cp config/example.env .env
```

| Variável | Descrição |
|----------|-----------|
| `BINANCE_API_KEY` | Chave REST |
| `BINANCE_API_SECRET` | Segredo |
| `FIX_HOST` | Host FIX (opcional) |
| `FIX_SENDER_COMP_ID` | Comp ID |

**Nunca** commite `.env`. Use permissões `600` no arquivo.

## Arquitetura de segurança local

- Chaves apenas em variáveis de ambiente ou keyring do SO (`keyring` crate).
- Assinaturas HMAC calculadas em memória; sem log de secrets.
- TLS obrigatório para todas as APIs.
- Modo somente leitura disponível: `READ_ONLY=true`.

Documentação: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | [docs/SECURITY.md](docs/SECURITY.md)

## Build

```bash
cargo build --release
./target/release/uterm
```

## Estrutura

| Pasta | Conteúdo |
|-------|----------|
| `src/ui/` | Layout Ratatui |
| `src/execution/` | Envio de ordens |
| `src/risk/` | Limites e VaR simplificado |
| `src/portfolio/` | Posições e PnL |
| `src/connectors/` | REST/FIX |
