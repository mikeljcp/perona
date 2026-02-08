# Documentação das crates (resumo)

Este ficheiro contém um resumo rápido das crates presentes em `crates/` e pontos importantes para cada uma.

## perona-database (crates/database)
- Responsabilidade: conexão com base de dados, execução de queries e implementação de repositórios.
- Código principal: `crates/database/src/` (veja `lib.rs` e `database/` e `repositories/`).

## perona-memory (crates/memory)
- Responsabilidade: ler memória de processos e construir um `Client` para manipular dados em memória.
- Detalhe: função pública `read(pid: u32) -> Result<Client, String>` em `lib.rs` que usa `proc_mem`.

## perona-player-rating (crates/playerRating)
- Responsabilidade: calcular Player Rating (PR) usando constantes (`POINT_DIFFERENCE`, `POINT_MULTIPLIER`).
- Estrutura principal: `PlayerRating` em `src/player_rating.rs` com métodos `get_pr()`.

## perona-process (crates/process)
- Responsabilidade esperada: utilitários e gerência de processos.

## perona-validation (crates/validation)
- Responsabilidade esperada: validações compartilhadas e constantes.

## perona-types (crates/types)
- Responsabilidade esperada: estruturas tipadas do domínio (ex.: `player`, `address`).

---

## Como estender a documentação por crate

1. Abrir `crates/<crate>/src/lib.rs` e adicionar doc comments no topo do arquivo:

```rust
//! Crate `<nome>` — breve descrição
```

2. Para cada função/struct pública, adicionar comentários doc `///` com exemplos de uso.

3. Re-gerar docs com `cargo doc --workspace --no-deps`.
