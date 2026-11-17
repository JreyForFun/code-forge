# Feature 006: Android app

**Status**: Implemented · **Version**: 1.0.0 · Build: `python scripts/build_android.py`

The same game runs as a native Android app next to the Windows app (no web version).

- `android/Cargo.toml` compiles `src/main.rs` as a `cdylib` loaded by Android's NativeActivity
  (`android_main`); the desktop package is unchanged.
- `scripts/build_android.py` uses the local Android SDK/NDK and JDK: cargo (NDK clang) → aapt2
  (manifest, icon) → native libs (arm64-v8a, x86_64, libc++_shared for Oboe audio) → zipalign → apksigner.
- `src/platform.rs` holds every platform difference (clock, saves, export/import, pictures, keyboard).
- Android specifics: fullscreen; saves in the app's private storage; on-screen keyboard opened through
  InputMethodManager (NativeActivity's own call is unreliable) and the layout lifted above it; the Back
  button closes dialogs / leaves screens; idle animations capped at 30 fps for battery.
- Verified on an Android 13 emulator: install, welcome flow with typing, Learn map, a full lesson page,
  Back navigation. Not verified on a physical phone.

Signing: a local keystore (`android/codeforge.keystore`, not in git). Keep it: Android only installs
updates signed with the same key.
