import os

import pytest

from .support import consts as eve_consts
from .support.client import TestClient
from .support.server import build_reefast, kill_reefast, run_reefast

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))


@pytest.fixture(scope='session', autouse=True)
def reefast_server():
    build_reefast(PROJECT_ROOT)
    pid = run_reefast(PROJECT_ROOT)
    try:
        yield pid
    except Exception:
        kill_reefast(pid)
        raise
    else:
        kill_reefast(pid)


@pytest.fixture()
def client(httpserver):
    yield TestClient(httpserver)


@pytest.fixture()
def consts():
    yield eve_consts
