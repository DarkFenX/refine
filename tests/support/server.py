import os
import subprocess
from signal import SIGKILL


class BuildError(Exception):
    pass


def build_server(proj_root):
    http_path = os.path.join(proj_root, 'http')
    os.chdir(http_path)
    result = subprocess.run(['cargo', 'build', '--profile=release'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if result.returncode != 0:
        raise BuildError(f'expected return code 0, got {result.returncode}')


def build_config(path, port):
    contents = f"[server]\nport = {port}"
    with open(path, 'w') as f:
        f.write(contents)


def run_server(proj_root, config_path):
    binary_path = os.path.join(proj_root, 'target', 'release', 'reefast-http')
    return subprocess.Popen([binary_path, config_path]).pid


def kill_server(pid):
    os.kill(pid, SIGKILL)
