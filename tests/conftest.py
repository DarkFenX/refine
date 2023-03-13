import os

import pytest

from .support.reefast_env import build_reefast, kill_reefast, run_reefast

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))


@pytest.fixture(scope='session', autouse=True)
def server():
    build_reefast(PROJECT_ROOT)
    pid = run_reefast(PROJECT_ROOT)
    try:
        yield pid
    except Exception:
        kill_reefast(pid)
        raise
    else:
        kill_reefast(pid)


@pytest.fixture(scope='session')
def client():
    yield None
