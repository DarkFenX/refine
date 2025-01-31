from __future__ import annotations

import os
import subprocess
import typing
from dataclasses import dataclass
from signal import SIGKILL

if typing.TYPE_CHECKING:
    from pathlib import Path


@dataclass(kw_only=True)
class ConfigInfo:
    config_path: Path
    port: int


@dataclass(kw_only=True)
class ServerInfo:
    pid: int


def build_server(*, proj_root: Path) -> None:
    http_path = proj_root / 'http'
    os.chdir(http_path)
    subprocess.run(
        ['cargo', 'build', '--package=reefast-http', '--profile=release'],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=True)


def build_config(*, config_path: Path, port: int, log_folder: Path) -> ConfigInfo:
    contents = [
        '[server]',
        f'port = {port}',
        'solsys_lifetime = 30',
        'solsys_cleanup_interval = 5',
        '[log]',
        f'folder = "{log_folder}"',
        'level = "debug"',
        'rotate = false']
    with config_path.open(mode='w') as f:
        f.write('\n'.join(contents))
    return ConfigInfo(config_path=config_path, port=port)


def run_server(*, proj_root: Path, config_path: Path) -> ServerInfo:
    binary_path = proj_root / 'target' / 'release' / 'reefast-http'
    return ServerInfo(pid=subprocess.Popen(
        [binary_path, config_path],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL).pid)


def kill_server(*, pid: int) -> None:
    os.kill(pid, SIGKILL)
