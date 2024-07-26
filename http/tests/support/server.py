import os
import subprocess
from collections import namedtuple
from pathlib import Path
from signal import SIGKILL


ConfigInfo = namedtuple('ConfigInfo', (['config_path', 'port']))
ServerInfo = namedtuple('ServerInfo', ['pid'])


def build_server(proj_root: str):
    http_path = os.path.join(proj_root, 'http')
    os.chdir(http_path)
    subprocess.run(
        ['cargo', 'build', '--package=reefast-http', '--profile=release'],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=True)


def build_config(config_path: Path, port: int, log_folder: Path) -> ConfigInfo:
    contents = [
        '[server]',
        f'port = {port}',
        'solsys_lifetime = 30',
        'solsys_cleanup_interval = 5',
        '[log]',
        f'folder = "{log_folder}"',
        'level = "debug"',
        'rotate = false']
    with open(config_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(contents))
    return ConfigInfo(config_path=config_path, port=port)


def run_server(proj_root: str, config_path: str) -> ServerInfo:
    binary_path = os.path.join(proj_root, 'target', 'release', 'reefast-http')
    return ServerInfo(pid=subprocess.Popen(
        [binary_path, config_path],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL).pid)


def kill_server(pid: int) -> None:
    os.kill(pid, SIGKILL)
