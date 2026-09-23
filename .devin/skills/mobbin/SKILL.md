---
name: mobbin
description: Find screen and interaction references for improving flows, motion, buttons, empty states, and layout.
triggers: [user]
allowed-tools: [read, grep, edit, mcp_call_tool]
permissions:
  - Read(src/**)
  - Write(src/**)
  - Read(style/**)
  - Write(style/**)
  - Read(style/tokens.css)
---

Use Mobbin for user flows, motion, buttons, empty states, and layout only. Keep Madrid Lim's existing palette and components.

1. Ask which screen or pattern needs references.
2. Call `mcp_call_tool` on server `mobbin` with `search_screens`, `search_flows`, or `search_sections`.
3. Summarize 2 to 4 results by hierarchy, spacing, affordances, motion timing, and states.
4. Map useful ideas onto existing tokens and components.
5. Implement in Leptos components and plain CSS, honoring `prefers-reduced-motion`.
6. Run the validation commands in `AGENTS.md`.
