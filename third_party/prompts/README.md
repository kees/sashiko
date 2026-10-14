# Vendored review prompts

This directory holds the part of Chris Mason's
[review-prompts](https://github.com/masoncl/review-prompts) that Sashiko's
Linux patch review uses, as of the review-prompts commit named in
`REVISION`. `LICENSE` is review-prompts' own.

## What is kept

Only the files a Linux review can reach:

- Loaded by the review stages: `kernel/false-positive-guide.md`,
  `kernel/inline-template.md`, `kernel/callstack.md`,
  `kernel/technical-patterns.md`, `kernel/subsystem/subsystem.md`, and
  `kernel/subsystem/locking.md`.
- Chosen by the pre-screen from that index: the rest of
  `kernel/subsystem/`.
- Named by a kept file, for a stage to fetch with `read_prompt`:
  `kernel/pointer-guards.md`.
- `kernel/severity.md`, which only Sashiko has.

Everything else in review-prompts is left out: the agent prompts, slash
commands, setup scripts, and skills that run review-prompts' own review in
Claude Code and other tools; the kernel guides no stage loads, such as
`review-core.md` and `lore-thread.md`; and the prompts for other projects.
`read_prompt` can open any file under `kernel/`, so a file kept here that
Sashiko never chose is still one a model could read and follow, and every
sync would have to bring it up to date for nothing.

## Sashiko's changes

Sashiko carries changes to some kept files, such as the harness
adaptations in `kernel/inline-template.md`; `git log -- third_party/prompts`
gives the reason for each. Changes that do not depend on Sashiko's harness
belong in review-prompts too.

## Syncing

To sync with a newer review-prompts commit:

1. Copy the kept files from it: the files listed above, every guide under
   `kernel/subsystem/`, including new ones, and none that review-prompts
   has removed.
2. Keep Sashiko's changes on top, each merged against the review-prompts
   version it descends from.
3. Set `REVISION` to that commit.
4. If a kept file now names a file outside this set, either bring that
   file in and list it above, or leave it out and say why here. The bundle
   tests check that every guide `kernel/subsystem/subsystem.md` names is
   present.

When Sashiko starts loading a file, add it to the list above.
