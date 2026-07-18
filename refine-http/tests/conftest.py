import re
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
        '--optimized',
        action='store_true',
        help='build server using the release-opt profile')
    parser.addoption(
        '--cpu-affinity',
        type=str,
        default='',
        help='pin server process to passed CPU IDs')


@pytest.fixture(scope='session')
def run_tmp_dir(tmp_path_factory: pytest.TempPathFactory) -> Path:
    return tmp_path_factory.mktemp('refine_test')


@pytest.fixture(scope='session')
def run_config(run_tmp_dir: Path) -> ConfigInfo:
    config_path = run_tmp_dir / 'config.toml'
    port = next_free_port(start_port=8000)
    return build_config(config_path=config_path, port=port, log_dir=run_tmp_dir)


@pytest.fixture(scope='session', autouse=True)
def refine_server(
        pytestconfig: pytest.Config,
        run_config: ConfigInfo,
        log_reader: LogReader,
) -> Generator[ServerInfo]:
    optimized = pytestconfig.getoption('optimized')
    cpu_affinity = [int(i) for i in re.split(r', ?', pytestconfig.getoption('cpu_affinity')) if i]
    build_server(proj_root=PROJECT_ROOT, optimized=optimized)
    with log_reader.get_collector() as log_collector:
        server_info = run_server(
            proj_root=PROJECT_ROOT,
            config_path=run_config.config_path,
            optimized=optimized,
            cpu_affinity=cpu_affinity)
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
        httpserver: pytest_httpserver.HTTPServer,
        run_config: ConfigInfo,
        log_reader: LogReader,
) -> Generator[TestClient]:
    test_client = TestClient(eve_data_server=httpserver, api_port=run_config.port, log_reader=log_reader)
    yield test_client
    test_client.cleanup_sols()
    test_client.cleanup_sources()


@pytest.fixture
def consts():  # noqa: ANN201
    return eve_consts


@pytest.fixture(scope='session')
def log_reader(run_tmp_dir: Path) -> Generator[LogReader]:
    log_path = run_tmp_dir / 'refine-http.log'
    reader = LogReader(path=log_path)
    reader.run()
    yield reader
    reader.stop()


@pytest.fixture
def log(log_reader: LogReader) -> Generator[LogCollector]:
    with log_reader.get_collector() as log_collector:
        yield log_collector
