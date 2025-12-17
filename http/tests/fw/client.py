import typing

from fw.api import ApiClient
from fw.eve import EveDataManager, EveTypeFactory

if typing.TYPE_CHECKING:
    import pytest_httpserver

    from fw.log import LogReader


class TestClient(ApiClient, EveTypeFactory, EveDataManager):

    def __init__(
            self, *,
            eve_data_server: pytest_httpserver.HTTPServer,
            api_port: int,
            log_reader: LogReader,
            fast_cleanup_check: bool,
    ) -> None:
        super().__init__(
            data_server=eve_data_server,
            port=api_port,
            log_reader=log_reader,
            fast_cleanup_check=fast_cleanup_check)
