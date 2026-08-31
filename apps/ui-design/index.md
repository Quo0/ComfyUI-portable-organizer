# ComfyUI Portable Organizer — design

The style guide and the screen mock-ups in one site, under separate menu items:

- **[Style guide](/styleguide/)** — palette, typography and component samples
  in both themes.
- **[Screens](/screens/)** — screen mock-ups by end-to-end scenario, built from
  those same components.

The source of truth is not here: colours and metrics live in
`apps/desktop/src/styles/tokens.css`, components in
`apps/desktop/src/styles/components.css`. Both are read directly, with no
copies; `pnpm ui-design:tokens` projects the tokens onto `.t-light`/`.t-dark`
(for the panels below) — automatically when `pnpm ui-design:dev` runs.
