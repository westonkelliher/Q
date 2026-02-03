# Git Account Setup Reference

This repo uses the **westonkelliher** GitHub account, while the work computer defaults to **weston2dots**.

## Quick Commands

### Check which account is active
```bash
gh auth status
```

### Switch to personal account (for this repo)
```bash
gh auth switch --user westonkelliher
```

### Switch back to work account
```bash
gh auth switch --user weston2dots
```

## How It Works

- GitHub CLI (`gh`) manages authentication for both accounts
- Git is configured to use `gh auth git-credential` for HTTPS operations
- Switching accounts with `gh auth switch` changes which credentials are used globally

## This Repo's Local Config

This repo has local git config overrides (stored in `.git/config`):
- `user.name` = Weston Kelliher
- `user.email` = westonkelliher@gmail.com

This ensures commits are attributed correctly regardless of global settings.

## Troubleshooting

**"Permission denied" errors when pushing:**
1. Run `gh auth status` to check active account
2. If wrong account is active, run `gh auth switch --user westonkelliher`
3. Try pushing again

**Check all git config settings:**
```bash
git config --list --show-origin
```

**Verify remote URL:**
```bash
git remote -v
```
Should show: `https://github.com/westonkelliher/Q.git`
