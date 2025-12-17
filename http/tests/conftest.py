import typing

import pytest

from fw import consts as eve_consts
from fw.client import TestClient
from fw.log import LogReader
from fw.server import build_config, build_server, kill_server, run_server
from fw.util import PROJECT_ROOT, next_free_port

if typing.TYPE_CHECKING:
    from collections.abc import Generator
    from pathlib import Path

    import pytest_httpserver

    from fw.log import LogCollector
    from fw.server import ConfigInfo, ServerInfo


def pytest_addoption(parser: pytest.Parser) -> None:
    parser.addoption(
        '--fast-cleanup-check',
        action='store_true',
        help='make log check during source creation faster, but unreliable')
    parser.addoption(
        '--optimized',
        action='store_true',
        help='build server using the release-opt profile')


@pytest.fixture(scope='session')
def run_tmp_folder(tmp_path_factory: pytest.TempPathFactory) -> Path:
    return tmp_path_factory.mktemp('refine_test')


@pytest.fixture(scope='session')
def run_config(run_tmp_folder: Path) -> ConfigInfo:
    config_path = run_tmp_folder / 'config.toml'
    port = next_free_port(start_port=8000)
    return build_config(config_path=config_path, port=port, log_folder=run_tmp_folder)


@pytest.fixture(scope='session', autouse=True)
def refine_server(
        pytestconfig: pytest.Config,
        run_config: ConfigInfo,
        log_reader: LogReader,
) -> Generator[ServerInfo]:
    optimized = pytestconfig.getoption('optimized')
    build_server(proj_root=PROJECT_ROOT, optimized=optimized)
    with log_reader.get_collector() as log_collector:
        server_info = run_server(proj_root=PROJECT_ROOT, config_path=run_config.config_path, optimized=optimized)
        try:
            # Wait for server to confirm it's up before yielding
            log_collector.wait_log_entry(msg='re:listening on.+', timeout=10)
        except Exception:
            kill_server(server_info=server_info)
            raise
    try:
        yield server_info
    except Exception:
        kill_server(server_info=server_info)
        raise
    kill_server(server_info=server_info)


@pytest.fixture
def client(
        pytestconfig: pytest.Config,
        httpserver: pytest_httpserver.HTTPServer,
        run_config: ConfigInfo,
        log_reader: LogReader,
) -> Generator[TestClient]:
    test_client = TestClient(
        eve_data_server=httpserver,
        api_port=run_config.port,
        log_reader=log_reader,
        fast_cleanup_check=pytestconfig.getoption('fast_cleanup_check'))
    yield test_client
    test_client.cleanup_sols()
    test_client.cleanup_sources()


@pytest.fixture
def consts():  # noqa: ANN201
    return eve_consts


@pytest.fixture(scope='session')
def log_reader(run_tmp_folder: Path) -> Generator[LogReader]:
    log_path = run_tmp_folder / 'refine-http.log'
    reader = LogReader(path=log_path)
    reader.run()
    yield reader
    reader.stop()


@pytest.fixture
def log(log_reader: LogReader) -> Generator[LogCollector]:
    with log_reader.get_collector() as log_collector:
        yield log_collector
