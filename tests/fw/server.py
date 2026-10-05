import os
import subprocess
import typing
from dataclasses import dataclass

import psutil

if typing.TYPE_CHECKING:
    from pathlib import Path

    from fw.log import LogReader


@dataclass(kw_only=True)
class ConfigInfo:
    config_path: Path
    max_request_body_size: int


@dataclass(kw_only=True)
class ServerInfo:
    popen: subprocess.Popen
    api_url: str


def build_server(*, proj_root: Path, optimized: bool) -> None:
    http_path = proj_root / 'refine-http'
    os.chdir(http_path)
    subprocess.run(
        ['cargo', 'build', '--package=refine-http', f'--profile={get_profile_name(optimized=optimized)}'],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=True)


def build_config(
        *,
        config_path: Path,
        log_dir: Path,
        max_request_body_size: int = 1024 * 1024,
) -> ConfigInfo:
    contents = [
        '[app]',
        f'max_request_body_size = {max_request_body_size}',
        'sol_lifetime = 30',
        'sol_cleanup_interval = 5',
        'standard_threads = 2',
        'heavy_threads = 4',
        '[network]',
        'address = "localhost"',
        'port = 0',  # Let OS allocate a port, which one is allocated is reported via logs
        '[log]',
        f'dir = "{log_dir}"',
        'level = "debug"',
        'bodies = true',
        'rotate = false']
    with config_path.open(mode='w', encoding='utf-8') as f:
        f.write('\n'.join(contents))
    return ConfigInfo(config_path=config_path, max_request_body_size=max_request_body_size)


def run_server(
        *,
        proj_root: Path,
        config_path: Path,
        optimized: bool,
        cpu_affinity: list[int],
        log_reader: LogReader,
) -> ServerInfo:
    binary_path = proj_root / 'target' / get_profile_name(optimized=optimized) / 'refine-http'
    with log_reader.get_collector() as log_collector:
        popen = subprocess.Popen(
            [binary_path, config_path],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL)
        if cpu_affinity:
            psutil.Process(pid=popen.pid).cpu_affinity(cpus=cpu_affinity)
        try:
            # Wait for server to confirm it's up
            log_entry = log_collector.wait_log_entry(msg='re:listening on .+', timeout=10)
        except Exception:
            _terminate_server(popen=popen)
            raise
    addr = log_entry.msg.removeprefix('listening on ')
    return ServerInfo(popen=popen, api_url=f'http://{addr}')


def get_profile_name(*, optimized: bool) -> str:
    return 'release-opt' if optimized else 'release'


def kill_server(*, server_info: ServerInfo) -> None:
    _terminate_server(popen=server_info.popen)
    _kill_server(popen=server_info.popen)


def _terminate_server(*, popen: subprocess.Popen) -> None:
    popen.terminate()
    popen.wait(timeout=2)


def _kill_server(*, popen: subprocess.Popen) -> None:
    popen.kill()
