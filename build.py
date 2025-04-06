#!/usr/bin/env python3

import os
import shutil
import subprocess


def build():
    print("🔧 Starting build...")

    result = subprocess.run("cargo install cross --git https://github.com/cross-rs/cross", shell=True)
    if result.returncode != 0:
        raise RuntimeError("❌ Install cross failed.")

    result = subprocess.run("cross build --release --all --target aarch64-unknown-linux-gnu", shell=True)
    if result.returncode != 0:
        raise RuntimeError("❌ Build failed.")

    print(f"✅ Build done!")


def gather_artifacts():
    src = 'target/aarch64-unknown-linux-gnu/release/tain-cli'
    dst = 'artifacts/tain'

    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copy2(src, dst)


if __name__ == "__main__":
    build()
    gather_artifacts()
