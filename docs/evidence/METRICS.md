# Metrics

What Sill measures about itself, how, and the rules for using the numbers.
Snapshots live in [snapshots/](snapshots/), one file per month once there is
motion worth recording (no fake cadence — a quiet month may simply be skipped
with the gap visible).

## Tracked metrics

| Metric                                              | Source (verifiable)       | Notes                                                                |
| --------------------------------------------------- | ------------------------- | -------------------------------------------------------------------- |
| Stars, forks, watchers                              | GitHub repo API           | Vanity-adjacent; recorded because reviewers ask, never optimized for |
| Contributors (total / first-time)                   | GitHub contributors graph | The number that matters most                                         |
| PRs opened/merged; median time-to-first-review      | GitHub API                | Health of the contribution loop                                      |
| Issues opened/closed; median time-to-first-response | GitHub API                |                                                                      |
| Releases + per-asset download counts                | GitHub Releases API       | Only meaningful post-first-release                                   |
| CI reliability (failed runs on main)                | Actions history           |                                                                      |
| Security: advisories published, time-to-fix         | GitHub advisories         | Never includes embargoed detail                                      |
| Performance results                                 | benchmarks/results/       | Only harness-produced numbers                                        |

## Rules

1. Numbers come from the named sources at snapshot time — no estimates, no
   interpolation, no "about".
2. Adoption language stays calibrated: stars are interest, downloads are
   downloads; neither is "users" ([ADOPTION.md](ADOPTION.md)).
3. Machine-readable needs are served by the GitHub API itself; snapshot files
   exist for point-in-time honesty, not as a parallel analytics system.
