import typing

from fw.api import ApiClient
from fw.eve import EveDataManager, EveTypeFactory

if typing.TYPE_CHECKING:
    from fw.log import LogReader
    from fw.util import StaticHttpServer


class TestClient(ApiClient, EveTypeFactory, EveDataManager):

    def __init__(
            self, *,
            eve_data_server: StaticHttpServer,
            api_url: str,
            log_reader: LogReader,
    ) -> None:
        super().__init__(data_server=eve_data_server, api_url=api_url, log_reader=log_reader)
