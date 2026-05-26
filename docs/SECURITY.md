# Segurança local

1. **Armazenamento de chaves**: preferir `keyring` em vez de `.env` em produção.
2. **Permissões de API**: criar chaves com IP whitelist e sem permissão de saque.
3. **Audit log**: registrar ordens enviadas (sem secrets) em `~/.uterm/audit.log`.
4. **Kill switch**: atalho `k` persiste flag em disco para sobreviver a restart acidental da UI.
