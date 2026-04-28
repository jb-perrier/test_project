# test_project Fixture Notes

`test_project/` is the local sample workspace for Story Agent development.

Fixture ID policy:

- The checked-in sample stories intentionally keep numeric-looking IDs such as `ST-00001` through `ST-00007`.
- New stories created by the extension now use the current hex-style format such as `ST-A1B2C`.
- Existing numeric IDs remain valid because Story Agent treats story IDs as `ST-[0-9A-F]{5}` and normalizes them case-insensitively.

This fixture therefore keeps the older numeric sample set on purpose as compatibility coverage instead of rewriting the sample history.

When you create new stories in this workspace, Story Agent will create managed story worktrees outside the tracked repository root under the sibling `test_project.story-agent-worktrees/<storyId>/` path.