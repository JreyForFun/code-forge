"""Build the Android app: dist/CodeForge-<version>-android.apk

Uses the Android SDK/NDK and JDK already on this PC (no extra tools):
  1. cargo builds the game for arm64 Android with the NDK's clang
  2. aapt2 packages the manifest and launcher icon
  3. the native libraries are added, then zipalign + apksigner sign the APK

usage:  python scripts/build_android.py
"""
import os, re, shutil, subprocess, sys, zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SDK = Path(os.environ.get("ANDROID_HOME") or os.environ.get("ANDROID_SDK_ROOT") or Path.home() / "AppData/Local/Android/Sdk")
APP_ID = "com.codeforge.game"
MIN_SDK, TARGET_SDK = 26, 34
# phones are arm64; x86_64 runs on emulators and some Chromebooks
TARGETS = [("aarch64-linux-android", "arm64-v8a"), ("x86_64-linux-android", "x86_64")]

def newest(d: Path) -> Path:
    vs = sorted([p for p in d.iterdir() if p.is_dir()], key=lambda p: [int(x) for x in re.findall(r"\d+", p.name)])
    if not vs: sys.exit(f"nothing in {d}")
    return vs[-1]

def run(cmd, **kw):
    print(">", " ".join(str(c) for c in cmd)[:200])
    subprocess.run([str(c) for c in cmd], check=True, **kw)

def main():
    version = re.search(r'^version\s*=\s*"([^"]+)"', (ROOT / "Cargo.toml").read_text(), re.M).group(1)
    ndk = newest(SDK / "ndk")
    bt = newest(SDK / "build-tools")
    platform_jar = SDK / "platforms" / f"android-{TARGET_SDK}" / "android.jar"
    if not platform_jar.exists(): platform_jar = newest(SDK / "platforms") / "android.jar"
    llvm = ndk / "toolchains/llvm/prebuilt/windows-x86_64"
    jdk = next((p for p in sorted(Path("C:/Program Files/Java").glob("jdk-*"), reverse=True) if (p / "bin/keytool.exe").exists()), None)
    if not jdk: sys.exit("no JDK with keytool found under C:/Program Files/Java")
    print(f"NDK {ndk.name}, build-tools {bt.name}, {platform_jar.parent.name}, {jdk.name}")

    # 1. native libraries
    clang, clangxx = llvm / "bin/clang.exe", llvm / "bin/clang++.exe"
    libs = []
    for TARGET, abi in TARGETS:
      env = dict(os.environ)
      tgt = f"--target={TARGET}{MIN_SDK}"
      key = TARGET.replace("-", "_")
      env.update({
        f"CARGO_TARGET_{key.upper()}_LINKER": str(clang),
        f"CC_{key}": str(clang), f"CXX_{key}": str(clangxx), f"AR_{key}": str(llvm / "bin/llvm-ar.exe"),
        f"CFLAGS_{key}": tgt, f"CXXFLAGS_{key}": tgt,
        "CARGO_TARGET_DIR": str(ROOT / "android/target"),
        "ANDROID_NDK_HOME": str(ndk), "ANDROID_NDK_ROOT": str(ndk), "ANDROID_HOME": str(SDK),
    })
      env["RUSTFLAGS"] = (env.get("RUSTFLAGS", "") + f" -C link-arg={tgt}").strip()
      run(["cargo", "build", "--release", "--target", TARGET, "--manifest-path", ROOT / "android/Cargo.toml"], env=env)
      libs.append((abi, ROOT / f"android/target/{TARGET}/release/libcode_forge.so", llvm / f"sysroot/usr/lib/{TARGET}/libc++_shared.so"))

    # 2. manifest + icon
    build = ROOT / "android/build"
    shutil.rmtree(build, ignore_errors=True)
    (build / "res/mipmap-xxxhdpi").mkdir(parents=True)
    from PIL import Image
    Image.open(ROOT / "assets/icon.png").convert("RGBA").resize((192, 192), Image.NEAREST).save(build / "res/mipmap-xxxhdpi/ic_launcher.png")
    # adaptive icon (Android 8+): stone-wall background + the hammer inside the safe zone, so the
    # launcher's circle/squircle mask is filled instead of showing a small square in a white circle
    S = 432                                               # 108dp at xxxhdpi
    wall = Image.open(ROOT / "assets/ui/wall.png").convert("RGBA").resize((48, 48), Image.NEAREST)
    bg = Image.new("RGBA", (S, S), (22, 19, 34, 255))
    for y in range(0, S, 48):
        for x in range(0, S, 48): bg.alpha_composite(wall, (x, y))
    shade = Image.new("RGBA", (S, S), (16, 12, 28, 150))
    bg.alpha_composite(shade)
    bg.save(build / "res/mipmap-xxxhdpi/ic_launcher_background.png")
    mark = Image.open(ROOT / "assets/art/logo_mark.png").convert("RGBA")
    k = 264 // mark.width                                  # whole-number scale inside the 66dp safe zone
    mark = mark.resize((mark.width * k, mark.height * k), Image.NEAREST)
    fg = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    fg.alpha_composite(mark, ((S - mark.width) // 2, (S - mark.height) // 2))
    fg.save(build / "res/mipmap-xxxhdpi/ic_launcher_foreground.png")
    (build / "res/mipmap-anydpi-v26").mkdir(parents=True)
    adaptive = """<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
  <background android:drawable="@mipmap/ic_launcher_background"/>
  <foreground android:drawable="@mipmap/ic_launcher_foreground"/>
</adaptive-icon>
"""
    for n in ["ic_launcher", "ic_launcher_round"]: (build / f"res/mipmap-anydpi-v26/{n}.xml").write_text(adaptive, encoding="utf-8")
    vcode = int("".join(f"{int(x):02d}" for x in version.split(".")))
    (build / "AndroidManifest.xml").write_text(f'''<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="{APP_ID}"
    android:versionCode="{vcode}" android:versionName="{version}">
  <uses-sdk android:minSdkVersion="{MIN_SDK}" android:targetSdkVersion="{TARGET_SDK}"/>
  <uses-feature android:glEsVersion="0x00030000" android:required="true"/>
  <application android:label="Code Forge" android:icon="@mipmap/ic_launcher" android:roundIcon="@mipmap/ic_launcher_round" android:hasCode="false"
      android:extractNativeLibs="true" android:allowBackup="true">
    <activity android:name="android.app.NativeActivity" android:exported="true"
        android:configChanges="orientation|screenSize|screenLayout|keyboardHidden|keyboard|uiMode|density"
        android:windowSoftInputMode="adjustResize"
        android:theme="@android:style/Theme.DeviceDefault.NoActionBar.Fullscreen">
      <meta-data android:name="android.app.lib_name" android:value="code_forge"/>
      <intent-filter>
        <action android:name="android.intent.action.MAIN"/>
        <category android:name="android.intent.category.LAUNCHER"/>
      </intent-filter>
    </activity>
  </application>
</manifest>
''', encoding="utf-8")
    aapt2 = bt / "aapt2.exe"
    run([aapt2, "compile", "--dir", build / "res", "-o", build / "res.zip"])
    run([aapt2, "link", "-o", build / "base.apk", "-I", platform_jar, "--manifest", build / "AndroidManifest.xml", build / "res.zip", "--min-sdk-version", MIN_SDK, "--target-sdk-version", TARGET_SDK])

    # 3. add the native libraries, align, sign
    with zipfile.ZipFile(build / "base.apk", "a", zipfile.ZIP_DEFLATED) as z:
        for abi, so, cxx_shared in libs:
            z.write(so, f"lib/{abi}/libcode_forge.so")
            if cxx_shared.exists(): z.write(cxx_shared, f"lib/{abi}/libc++_shared.so")
    run([bt / "zipalign.exe", "-f", "-p", "4", build / "base.apk", build / "aligned.apk"])
    ks = ROOT / "android/codeforge.keystore"
    if not ks.exists():
        run([jdk / "bin/keytool.exe", "-genkeypair", "-keystore", ks, "-alias", "codeforge", "-keyalg", "RSA", "-keysize", "2048", "-validity", "10000",
             "-storepass", "codeforge", "-keypass", "codeforge", "-dname", "CN=Code Forge, O=Code Forge, C=PH"])
    out = ROOT / f"dist/CodeForge-{version}-android.apk"
    out.parent.mkdir(exist_ok=True)
    apksigner = bt / "lib/apksigner.jar"
    run([jdk / "bin/java.exe", "-jar", apksigner, "sign", "--ks", ks, "--ks-pass", "pass:codeforge", "--key-pass", "pass:codeforge", "--out", out, build / "aligned.apk"])
    print(f"\nDone: {out} ({out.stat().st_size / 1e6:.1f} MB)")

if __name__ == "__main__":
    main()
