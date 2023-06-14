import os
import subprocess
from signal import SIGKILL


def build_server(proj_root):
    http_path = os.path.join(proj_root, 'http')
    os.chdir(http_path)
    subprocess.run(
        ['cargo', 'build', '--package=reefast-http', '--profile=release'],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=True)


def build_config(config_path, port, log_folder):
    contents = [
        '[server]',
        f'port = {port}',
        'nsolsys_lifetime = 30',
        'solsys_cleanup_interval = 5',
        '[log]',
        f'folder = "{log_folder}"',
        'level = "debug"',
        'rotate = false']
    with open(config_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(contents))


def run_server(proj_root, config_path):
    binary_path = os.path.join(proj_root, 'target', 'release', 'reefast-http')
    return subprocess.Popen(
        [binary_path, config_path],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL).pid


def kill_server(pid):
    os.kill(pid, SIGKILL)
