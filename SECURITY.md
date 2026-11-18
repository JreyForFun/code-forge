# Security Policy

## Supported versions

Only the latest release gets security fixes.

| Version | Supported |
| --- | --- |
| 1.0.x (latest) | Yes |
| Older builds (Pixel Learn 1.x, Code Forge 1.4–1.6 test builds) | No: please update |

## What counts as a security problem

Code Forge is an offline game: it has no accounts, no servers and no online features, and it never
sends your data anywhere. Problems that **are** security issues:

- A crafted `.codeforge` progress file that crashes the game, runs code, or writes files outside the
  save folder when imported
- Player code in a lesson or the Spell Lab that can reach your files, the network or other programs
  (the Python, JavaScript and SQL runners are meant to be fully sandboxed)
- The Windows `.exe` or Android `.apk` on the Releases page not matching this source code, or asking
  for permissions it doesn't need

**Not** security issues (please open a normal issue instead):

- Editing your own save file to get more gems or XP. The save checksum only discourages casual
  cheating; it is an offline single-player game, so this is by design.
- Windows SmartScreen saying the app is from an unknown publisher (the app is not code-signed yet).

## Reporting a vulnerability

**Please don't open a public issue.** Report it privately instead:

1. Go to the [Security tab](https://github.com/JreyForFun/code-forge/security) of this repository.
2. Click **Report a vulnerability** and describe what you found, how to reproduce it, and which
   version and device you used.

You can expect a first reply within **7 days**. If the problem is confirmed, a fix will be released
as soon as possible and you'll be credited in the release notes (unless you'd rather not be).

## Verifying downloads

Only download Code Forge from this repository's
[Releases page](https://github.com/JreyForFun/code-forge/releases). Each release lists the SHA-256
checksums of its files. To check a download on Windows:

```powershell
Get-FileHash .\CodeForge-1.0.0-windows.zip -Algorithm SHA256
```

The Android app is always signed with the same key. If Android refuses to install an update over
an existing Code Forge "because the signature is different", the file did not come from here.
