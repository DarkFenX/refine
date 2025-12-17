import pytest

from fw import eve
from fw.log import LogEntryNotFoundError
from fw.request import Request
from fw.util import Default
from .base import ApiClientBase


class ApiClientSrc(ApiClientBase, eve.EveDataManager, eve.EveDataServer):

    def __init__(self, *, fast_cleanup_check: bool, **kwargs) -> None:
        super().__init__(**kwargs)
        self.__fast_cleanup_check: bool = fast_cleanup_check
        self.__created_data_aliases: set[str] = set()

    def create_source_request(
            self, *,
            data: eve.EveObjects | type[Default],
    ) -> Request:
        if data is Default:
            data = self._get_default_eve_data()
        return Request(
            client=self,
            method='POST',
            url=f'{self._base_url}/src/{data.alias}',
            json={'data_version': '1', 'data_base_url': f'{self._eve_data_server_base_url}/{data.alias}/'})

    def create_source(
            self, *,
            data: eve.EveObjects | type[Default] = Default,
            cleanup_check: bool = True,
    ) -> None:

        def process(*, data: eve.EveObjects) -> None:
            resp = self.create_source_request(data=data).send()
            assert resp.status_code == 201
            self.__created_data_aliases.add(data.alias)

        if data is Default:
            data = self._get_default_eve_data()
        self._setup_eve_data_server(data=data)
        if cleanup_check:
            with self._log_reader.get_collector() as log_collector:
                process(data=data)
                if self.__fast_cleanup_check:
                    # Check if there are any "cleaned" entries in log upon completion w/o any
                    # waiting for a fast way
                    with pytest.raises(LogEntryNotFoundError):
                        log_collector.wait_log_entry(msg='re:cleaned .+', level='INFO', span='src-new:adg', timeout=0)
                else:
                    # Wait for negative report to appear for regular check
                    log_collector.wait_log_entry(
                        msg='no unused data found during cleanup',
                        level='INFO',
                        span='src-new:adg',
                        timeout=3)
        else:
            process(data=data)

    def remove_source_request(self, *, src_alias: str) -> Request:
        return Request(
            client=self,
            method='DELETE',
            url=f'{self._base_url}/src/{src_alias}')

    def remove_source(self, *, src_alias: str) -> None:
        resp = self.remove_source_request(src_alias=src_alias).send()
        assert resp.status_code == 204
        self.__created_data_aliases.remove(src_alias)

    def create_sources(self, *, cleanup_check: bool = True) -> None:

        def process(*, cleanup_check: bool) -> None:
            for data in self._eve_datas.values():
                self.create_source(data=data, cleanup_check=cleanup_check)

        # If no data was created, create default one
        if not self._eve_datas:
            self._get_default_eve_data()
        # Fast cleanup check is done when we create multiple sources if possible, since it becomes
        # more reliable this way; we check if there are any "cleaned" entries in log upon completion
        # w/o any waiting
        if cleanup_check and self.__fast_cleanup_check:
            with self._log_reader.get_collector() as log_collector:
                # No need to have per-source check when we do wider one
                process(cleanup_check=False)
                with pytest.raises(LogEntryNotFoundError):
                    log_collector.wait_log_entry(msg='re:cleaned .+', level='INFO', span='src-new:adg', timeout=0)
        else:
            process(cleanup_check=cleanup_check)

    def cleanup_sources(self) -> None:
        for alias in self.__created_data_aliases.copy():
            self.remove_source(src_alias=alias)
