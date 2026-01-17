# Optimization Notes

This file tracks current O(n) behaviors and the reasoning behind them. These are
not necessarily bugs, but they are important for performance as files and row
counts grow.

## Current O(n) Behaviors

1) `src/view.rs` `flatten_partitions`
- Rebuilds the entire row list every time it is called (O(rows)).
- When partitions are empty and raw lines are shown, it walks every line
  (O(lines)).
- Why it matters: this happens in the main loop and can dominate time as the
  visible set grows.

2) `src/search.rs` `mark_search_matches`
- Scans every line index in every partition when a new search term is applied
  (O(total lines)).
- Why it matters: large logs make search a full scan, even if only a subset of
  rows is visible.

3) `src/ui.rs` `max_row_width`
- Computes width by iterating across all rows (O(rows)) every time it is
  called.
- Why it matters: if called per frame, width recomputation becomes a hotspot.

4) `src/pipeline.rs` `choose_prefix_len`
- Groups lines for every prefix length until the target is met
  (O(n * max_prefix_len)).
- Why it matters: for long lines or large samples, startup cost grows quickly.

5) `src/pipeline.rs` `split_groups_to_target`
- Repeatedly scans groups to find the largest one and splits it, which can be
  O(groups^2) in the worst case.
- Why it matters: partition bootstrapping can slow down for many groups.

6) `src/partition.rs` `split_partition`
- Regroups all indices within a partition when splitting (O(k) per split).
- Why it matters: repeated splits on large partitions can add noticeable time.

## Notes
- These items will be revisited as we introduce streaming loads and incremental
  updates.
- Some O(n) behaviors are acceptable when triggered rarely (e.g. initial
  partition plan computation), but others are problematic if they run every
  frame or on every keypress.
