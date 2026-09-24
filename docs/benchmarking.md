# Benchmarking

Sashiko includes a benchmark tool to evaluate AI review performance
against known bugs. It ingests patches, waits for reviews to complete,
then uses an AI judge to compare findings against ground-truth
descriptions.

## Prerequisites

- A running sashiko daemon with a configured LLM provider
- A clean database (move or remove any existing `sashiko.db`)
- A benchmark JSON file (several are provided in `benchmarks/`)

## Quick start

```bash
# Start with a clean database
mv sashiko.db sashiko.db.bak

# Run the benchmark
cargo run --bin benchmark -- --file benchmarks/benchmark_small.json
```

## Benchmark files

| File | Description |
|------|-------------|
| `benchmarks/benchmark_tiny.json` | Minimal set for quick smoke tests. |
| `benchmarks/benchmark_small.json` | Small set for development iteration. |
| `benchmarks/benchmark.json` | Full benchmark suite. |
| `benchmarks/benchmark_preexisting.json` | Tests detection of pre-existing bugs. |
| `benchmarks/benchmark_smoke.json` | CI smoke test set. |
| `benchmarks/benchmark_testability.json` | 35 entries drawn from the full set whose known defect is itself a testing defect. |

Each file contains entries with a commit hash, a `Fixed-by` reference,
and a `problem_description` that the AI judge uses to evaluate whether
sashiko detected the issue.

An entry also has to be reviewable. A patch above the reviewer's size
limits (`max_lines_changed`, `max_files_touched`) is skipped without a
review and reported as not reviewed, so it can never count toward
detection. Check a new entry against those limits as well as for ground
truth.

### Scoring one axis: benchmark_testability.json

Each entry is scored on whether some finding described that entry's one
specific bug, so a change that improves review along a different axis
scores nothing on the general sets. `benchmark_testability.json` exists
for that reason: every entry's ground truth is a testing defect -- a
flaky KUnit case, a selftest asserting the wrong value, a test reading
uninitialized memory, a selftest that fails to build, a missing test --
so a change aimed at test quality can be seen at all.

It is a best case by construction. Every entry has a test defect to be
caught, so the set measures catching and never inventing; a change that
looks good here still has to be checked against a general set for the
findings it adds to ordinary patches.

## Command-line options

```
cargo run --bin benchmark -- [OPTIONS]
```

| Flag | Description |
|------|-------------|
| `-f, --file <PATH>` | Path to the benchmark JSON file (required). |
| `-p, --port <PORT>` | Override the daemon port (defaults to Settings.toml value). |
| `-r, --repo <URL>` | Override the kernel repository URL. |
| `--analyze-only` | Skip ingestion; only evaluate existing results in the database. |

## Output

The tool prints a summary to the console:

- **Detection rates**: Detected, Missed, Partially Detected
- **Performance metrics**: Average tokens in/out, average turns,
  average time per review
- **Counts**: Total concerns and findings

Detailed results are written to `benchmark_results.json` in the current
directory, including the AI judge's explanation for each finding.

## Comparing two revisions

Reviews are sampled from a model at temperature 1.0 that plans its own
stages, so two runs of *identical code* do not produce identical results.
Expect detection to move by a couple of entries and every aggregate by
around 10% between such runs.

A single before-and-after pair therefore cannot tell a change apart from
that sampling. Run the same revision twice as a null experiment, compare
those two identical sides first, and treat any difference smaller than
the one they show as unmeasured.

Give each side its own daemon, settings file, database, and port, and run
the sides one after another rather than together. A database per side
also separates the response caches, which sit beside the database file,
so neither side can replay the other's answers. Leave `[nntp]` out of a
benchmark settings file: the daemon starts the NNTP ingestor whenever one
is configured, and would spend the run ingesting mail into the benchmark
database.

## Re-evaluating existing results

If you have already run ingestion and reviews but want to re-score with
updated evaluation logic:

```bash
cargo run --bin benchmark -- --file benchmarks/benchmark_small.json --analyze-only
```

This skips patch submission and review, reading results directly from the
database.
