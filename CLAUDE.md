# Project Overview

**Puppy Life OS (小狗人生)** — an AI-native pet life management platform for modern pet-owning families, positioned as a family "AI pet butler" and a "digital life archive" for the dog. It uses AI and family-collaboration mechanisms to consolidate scattered pet-care information into a visualized, traceable, and collaborative full-lifecycle pet archive. Core domains include: family system with flexible RBAC permissions, pet profiles, health records, cloud album, smart calendar, walk tracking, AI pet translator, daily briefing, and finance manager. (Product requirements: `degisn/产品原型.md`; API design: `api_doc/`.)

## Architecture (DDD layering)

This is a Rust workspace structured with Domain-Driven Design (DDD). Each crate maps to one architectural layer:

- `crates/domain` — **Domain layer**: core business models (entities, value objects, domain errors `AppError` / `AppResult`). Depends on no infrastructure — only serialization and primitive-type libraries (serde, uuid, chrono, thiserror).
- `crates/db` — **Infrastructure layer**: persistence and data access; implements the repository contracts defined by the domain layer.
- `crates/api` — **Application / interface layer**: the entry point (`main.rs`, on tokio), responsible for exposing HTTP endpoints and orchestrating use cases.

Dependencies always point inward (api / db → domain); the domain layer stays pure and never depends back on infrastructure or frameworks.

# Rust coding guidelines

- Prioritize code correctness and clarity. Speed and efficiency are secondary priorities unless otherwise specified.
- Do not write organizational comments that summarize the code. Comments should only be written in order to explain "why" the code is written in some way in the case there is a reason that is tricky / non-obvious.
- Prefer implementing functionality in existing files unless it is a new logical component. Avoid creating many small files.
- Avoid using functions that panic like `unwrap()` or `expect()`, instead use mechanisms like `?` to propagate errors.
- Be careful with operations like indexing which may panic if the indexes are out of bounds.
- Never silently discard errors with `let _ =` on fallible operations. Always handle errors appropriately:
  - Propagate errors with `?` when the calling function should handle them
  - Use explicit error handling with `match` or `if let Err(...)` when you need custom logic
  - Example: avoid `let _ = client.request(...).await?;` - use `client.request(...).await?;` instead
- When implementing async operations that may fail, ensure errors propagate to the superstratum so users get meaningful feedback.
- Never create files with `mod.rs` paths - prefer `src/some_module.rs` instead of `src/some_module/mod.rs`.
- Avoid creative additions unless explicitly requested
- Use full words for variable names (no abbreviations like "q" for "queue")
- Use variable shadowing to scope clones in async contexts for clarity, minimizing the lifetime of borrowed references.
  Example:
  ```rust
  executor.spawn({
      let task_ran = task_ran.clone();
      async move {
          *task_ran.borrow_mut() = true;
      }
  });
  ```
- If you need to add new dependencies, please add them to the workspace.dependencies and import them from the child crates.
- When installing dependencies, use the `node scripts/getCrateVersion.js [crate-name]` command to obtain the latest version.
  Example:
  ```bash
  node scripts/getCrateVersion.js tokio
  ```
- Performance is not the top priority, but it is still high on the list. Avoid cloning whenever possible, and avoid blocking whenever possible.

### Rules

Respond in Chinese
