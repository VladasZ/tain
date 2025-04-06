#!/usr/bin/env python3

import os
import shutil
import subprocess
from pathlib import Path

TARGET = "aarch64-unknown-linux-gnu"
DOCKER_IMAGE = "rust:latest"


def build_with_docker():
    print("🔧 Starting Docker build...")

    docker_cmd = (
        f'docker run --rm '
        f'-v "{Path.cwd()}:/usr/src/myapp" '
        f'-w /usr/src/myapp '
        f'{DOCKER_IMAGE} '
        f'bash -c "dpkg --add-architecture arm64 && apt update && '
        f'apt install -y gcc-aarch64-linux-gnu libc6-dev-arm64-cross libssl-dev:arm64 pkg-config && '
        f'rustup target add {TARGET} && '
        f'export PKG_CONFIG_ALLOW_CROSS=1 && '
        f'export OPENSSL_DIR=/usr/aarch64-linux-gnu && '
        f'export OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu && '
        f'export OPENSSL_INCLUDE_DIR=/usr/include/openssl && '
        f'cargo build --all --release --target={TARGET}"'
    )

    result = subprocess.run(docker_cmd, shell=True)
    if result.returncode != 0:
        raise RuntimeError("❌ Build failed.")

    print(f"✅  Done! Check ./target/{TARGET}/release/")


def gather_artifacts():
    src = 'target/aarch64-unknown-linux-gnu/release/tain-cli'
    dst = 'artifacts/tain'

    os.makedirs(os.path.dirname(dst), exist_ok=True)

    shutil.copy2(src, dst)


if __name__ == "__main__":
    build_with_docker()
    gather_artifacts()
