# Backend Development Guidelines

> Best practices for backend development in this project.

---

## Overview

This directory contains guidelines for backend development. Fill in each file with your project's specific conventions.

---

## Guidelines Index

| Guide | Description | Status |
|-------|-------------|--------|
| [Directory Structure](./directory-structure.md) | Module organization and file layout | To fill |
| [Database Guidelines](./database-guidelines.md) | ORM patterns, queries, migrations | To fill |
| [Error Handling](./error-handling.md) | Error types, handling strategies | To fill |
| [Quality Guidelines](./quality-guidelines.md) | Code standards, forbidden patterns | To fill |
| [Logging Guidelines](./logging-guidelines.md) | Structured logging, log levels | To fill |
| [Typed IPC and Fixture Policy](./typed-ipc-policy.md) | T06 command/event DTOs, T09 read-only preflight/discovery, T10 explicit fixture routing, T11 paginated tree/note read, T12 backup inventory and fixture restore, T13 Windows runtime prototype, T14 draft persistence and editor session, T15 typed fixture note CRUD, T16 same-target conflict / unknown-result coordination, T17 host drain `begin_shutdown`, T18 editor content safety (textarea/`<pre>`, exact-byte CRLF, `executed=false`), T19 `list_relations` fixture wiki-link relations plus observation classified_as, T20 `expand_graph` one-hop fixture neighborhood plus bounded progressive expansion, fixture boundary, and T07 runtime snapshot projection | Current |
| [Engine Supervisor and Runtime State](./supervisor-state.md) | T07 lifecycle, spawn policy-before-process, failure, and shutdown contract | Current |

---

## How to Fill These Guidelines

For each guideline file:

1. Document your project's **actual conventions** (not ideals)
2. Include **code examples** from your codebase
3. List **forbidden patterns** and why
4. Add **common mistakes** your team has made

The goal is to help AI assistants and new team members understand how YOUR project works.

---

**Language**: All documentation should be written in **English**.
