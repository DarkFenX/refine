import os

from support import reefast_running

SCRIPT_DIR = os.path.realpath(os.path.dirname(__file__))
PROJECT_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, '..'))


class BuildError(Exception):
    pass


if __name__ == '__main__':
    with reefast_running(PROJECT_ROOT):
        import time
        time.sleep(10)
