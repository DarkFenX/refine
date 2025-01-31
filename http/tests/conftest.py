from __future__ import annotations

import os
import typing

import pytest

from tests.fw import consts as eve_consts
from tests.fw.client import TestClient
from tests.fw.log import LogReader
from tests.fw.server import build_config, build_server, kill_server, run_server
from tests.fw.util import next_free_port

if typing.TYPE_CHECKING:
    from collections.abc import Iterator
    from pathlib import Path
    from types import ModuleType

    import pytest_httpserver

    from tests.fw.log import LogCollector
    from tests.fw.server import ConfigInfo, ServerInfo

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))


def pytest_addoption(parser: pytest.Parser) -> None:
    parser.addoption(
        "--fast-cleanup-check",
        action="store_true",
        help="make log check during source creation faster, but unreliable")


@pytest.fixture(scope='session')
def reefast_tmp_folder(tmp_path_factory: pytest.TempPathFactory) -> Iterator[Path]:
    yield tmp_path_factory.mktemp('reefast_test')


@pytest.fixture(scope='session')
def reefast_config(reefast_tmp_folder: Path) -> Iterator[ConfigInfo]:
    config_path = reefast_tmp_folder / 'config.toml'
    port = next_free_port(start_port=8000)
    yield build_config(config_path=config_path, port=port, log_folder=reefast_tmp_folder)


@pytest.fixture(scope='session', autouse=True)
def reefast_server(reefast_config: ConfigInfo, log_reader: LogReader) -> Iterator[ServerInfo]:
    build_server(proj_root=PROJECT_ROOT)
    with log_reader.get_collector() as log_collector:
        server_info = run_server(proj_root=PROJECT_ROOT, config_path=reefast_config.config_path)
        # Wait for server to confirm it's up before yielding
        log_collector.wait_log_entry(msg='re:listening on.+', timeout=10)
    try:
        yield server_info
    except Exception:
        kill_server(pid=server_info.pid)
        raise
    kill_server(pid=server_info.pid)


@pytest.fixture()
def client(
        pytestconfig: pytest.Config,
        httpserver: pytest_httpserver.HTTPServer,
        reefast_config: ConfigInfo,
        log_reader: LogReader,
) -> Iterator[TestClient]:
    test_client = TestClient(
        eve_data_server=httpserver,
        api_port=reefast_config.port,
        log_reader=log_reader,
        fast_cleanup_check=pytestconfig.getoption("fast_cleanup_check"))
    yield test_client
    test_client.cleanup_sols()
    test_client.cleanup_sources()


@pytest.fixture()
def consts() -> Iterator[ModuleType]:
    yield eve_consts


@pytest.fixture(scope='session')
def log_reader(reefast_tmp_folder: Path) -> Iterator[LogReader]:
    log_path = reefast_tmp_folder / 'reefast-http.log'
    reader = LogReader(path=log_path)
    reader.run()
    yield reader
    reader.stop()


@pytest.fixture()
def log(log_reader: LogReader) -> Iterator[LogCollector]:
    with log_reader.get_collector() as log_collector:
        yield log_collector
