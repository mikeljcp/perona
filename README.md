# Perona

Monorepo Rust contendo o ecossistema Perona: várias crates relacionadas ao processamento, leitura de memória de processos, cálculo de rating de jogadores e persistência.

## Visão geral

- Raiz: executável binário em `src/main.rs` que usa a crate `perona-memory` para ler dados de um processo.
- Crates principais (em `crates/`):
  - `perona-database` (crates/database): conexão com DB, queries e repositórios.
  - `perona-memory` (crates/memory): leitura de memória de processos e construção do `Client`.
  - `perona-player-rating` (crates/playerRating): cálculo de rating de jogadores (PR).
  - `perona-process` (crates/process): gerenciamento de processos (auxiliares).
  - `perona-validation` (crates/validation): validações e constantes compartilhadas.
  - `perona-types` (crates/types): tipos e estruturas de domínio.

> Nota: os nomes das crates são prefixados com `perona-` e existem dentro da pasta `crates/`.

## Como construir e executar

Abra um terminal na raiz do repositório e execute:

```powershell
cargo build --workspace
cargo run --workspace
```

O binário principal (`perona`) chama internamente `perona-memory::read(pid)` no `main.rs`. Para rodar a leitura de memória, é necessário executar com permissões apropriadas e fornecer um PID válido (o `main.rs` atual usa PID `4684` como exemplo).

## Gerar documentação (HTML)

Para gerar a documentação das crates:

```powershell
cargo doc --workspace --no-deps
cargo doc --workspace --no-deps --open
```

## Testes

Para executar os testes do workspace:

```powershell
cargo test --workspace
```

## Estrutura importante

- [Crates listada](crates/README.md)
- Código do binário: `src/main.rs`
- Documentação adicional localizada em `docs/`.

## Próximos passos sugeridos

- Executar `cargo test` e `cargo doc` para verificar integridade.
- Adicionar `README.md` por crate com exemplos de uso público das APIs.
