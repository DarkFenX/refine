import os
import subprocess
from contextlib import contextmanager
from signal import SIGKILL


class BuildError(Exception):
    pass


def build_reefast(proj_root):
    http_path = os.path.join(proj_root, 'http')
    os.chdir(http_path)
    result = subprocess.run(['cargo', 'build', '--profile=release'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if result.returncode != 0:
        raise BuildError(f'expected return code 0, got {result.returncode}')


def run_reefast(proj_root):
    binary_path = os.path.join(proj_root, 'target', 'release', 'reefast-http')
    return subprocess.Popen([binary_path]).pid


def kill_reefast(pid):
    os.kill(pid, SIGKILL)


@contextmanager
def reefast_running(proj_root):
    build_reefast(proj_root)
    pid = run_reefast(proj_root)
    try:
        yield
    except Exception:
        kill_reefast(pid)
        raise
    else:
        kill_reefast(pid)
