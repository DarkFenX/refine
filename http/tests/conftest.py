import os

import pytest

from tests.support import consts as eve_consts
from tests.support.client import TestClient
from tests.support.log import LogReader
from tests.support.server import build_server, kill_server, run_server, build_config
from tests.support.util import next_free_port

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))


@pytest.fixture(scope='session')
def reefast_tmp_folder(tmp_path_factory):
    yield tmp_path_factory.mktemp('reefast_test')


@pytest.fixture(scope='session')
def reefast_config(reefast_tmp_folder):  # pylint: disable=W0621
    config_path = reefast_tmp_folder / 'config.toml'
    port = next_free_port(8000)
    yield build_config(config_path=config_path, port=port, log_folder=reefast_tmp_folder)


@pytest.fixture(scope='session', autouse=True)
def reefast_server(reefast_config, log_reader):  # pylint: disable=W0621
    build_server(PROJECT_ROOT)
    with log_reader.get_collector() as log_collector:
        server_info = run_server(proj_root=PROJECT_ROOT, config_path=reefast_config.config_path)
        # Wait for server to confirm it's up before yielding
        log_collector.wait_log_entry(msg='re:listening on.+', timeout=10)
    try:
        yield server_info
    except Exception:
        kill_server(server_info.pid)
        raise
    kill_server(server_info.pid)


@pytest.fixture()
def client(httpserver, reefast_config):  # pylint: disable=W0621
    test_client = TestClient(eve_data_server=httpserver, api_port=reefast_config.port)
    yield test_client
    test_client.cleanup_sols()
    test_client.cleanup_sources()


@pytest.fixture()
def consts():
    yield eve_consts


@pytest.fixture(scope='session')
def log_reader(reefast_tmp_folder):  # pylint: disable=W0621
    log_path = reefast_tmp_folder / 'reefast-http.log'
    reader = LogReader(path=log_path)
    reader.run()
    yield reader
    reader.stop()


@pytest.fixture()
def log(log_reader):  # pylint: disable=W0621
    with log_reader.get_collector() as log_collector:
        yield log_collector
