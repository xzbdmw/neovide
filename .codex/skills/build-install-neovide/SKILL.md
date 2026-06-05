---
name: build-install-neovide
description: Build, package, install, sign, and verify this Neovide repository as the normal macOS /Applications/Neovide.app. Use when the user asks to compile, build, rebuild, install, reinstall, replace, or verify Neovide from the current repo on macOS, especially when they explicitly want normal Neovide rather than Neovide_notify.
---

# Build Install Neovide

## Workflow

Use this workflow from the Neovide repository root.

1. Check the current worktree first:

```bash
git status --short
```

If there are modified files, assume the user wants those changes included unless they explicitly say otherwise. Do not revert unrelated user changes.

2. Build the release binary:

```bash
cargo build --release
```

Warnings are acceptable if the build exits successfully. Report notable warnings in the final response.

3. Generate the normal macOS app bundle:

```bash
GENERATE_BUNDLE_APP=true ./macos-builder/run
```

Do not set `GENERATE_DMG=true` unless the user asks for a DMG. The normal app bundle is produced at:

```text
target/release/bundle/osx/Neovide.app
```

4. Install the normal app into `/Applications`:

```bash
rm -rf /Applications/Neovide.app
cp -R target/release/bundle/osx/Neovide.app /Applications/Neovide.app
codesign --force --deep --sign - /Applications/Neovide.app
xattr -cr /Applications/Neovide.app
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f /Applications/Neovide.app
```

Do not install or rename `Neovide_notify.app` unless the user explicitly asks for the notify variant.

5. Verify the installed app:

```bash
plutil -p /Applications/Neovide.app/Contents/Info.plist | rg 'CFBundleDisplayName|CFBundleName|CFBundleIdentifier|CFBundleExecutable'
codesign -dv /Applications/Neovide.app 2>&1 | sed -n '1,80p'
/Applications/Neovide.app/Contents/MacOS/neovide --version
```

Expected normal app identity:

```text
CFBundleDisplayName = Neovide
CFBundleName = Neovide
CFBundleIdentifier = com.neovide.neovide
CFBundleExecutable = neovide
Signature = adhoc
```

6. Summarize the result concisely, including the installed path, bundle id, signature type, version output, and any build warnings.
