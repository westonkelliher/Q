# Combat Subsystem

> **Last Updated**: 2026-02-01  
> **Previous Commit**: `3325792`  
> Check this commit hash against the previous commit to verify documentation is up-to-date.

---

## Notes for LLMs

When modifying this subsystem:

1. **Update Tests**: Add tests for new features
2. **Update This README**: Keep documentation current with changes
3. **Update Commit Hash Before Committing**: When asked to commit, update the "Previous Commit" hash at the top of this file to reference the commit that existed BEFORE the changes being committed

### Commit Workflow

When the user says "commit":
1. Check `git log --oneline -1` for current commit hash
2. Update "Previous Commit" in this README to that hash
3. Update "Last Updated" date if significant changes
4. Stage changes and commit with descriptive message

---

## File Structure

```
src/
├── main.rs    # Entry point
└── lib.rs     # Module exports and re-exports (when created)
```
