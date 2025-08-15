import typing

from tests.fw.util import get_test_key
from .containers import EveObjects

if typing.TYPE_CHECKING:
    from tests.fw.util import TestKey

data_id: int = 10000


class EveDataManager:

    def __init__(self, **kwargs) -> None:
        super().__init__(**kwargs)
        self.__datas: dict[str, EveObjects] = {}
        self.__defsrc_stack_alias_map: dict[TestKey, str] = {}

    def mk_eve_data(self) -> EveObjects:
        global data_id  # noqa: PLW0603
        alias = str(data_id)
        data = self.__datas[alias] = EveObjects(alias=alias)
        data_id += 1
        return data

    def _get_default_eve_data(self) -> EveObjects:
        key = get_test_key()
        if key in self.__defsrc_stack_alias_map:
            alias = self.__defsrc_stack_alias_map[key]
            return self.__datas[alias]
        data = self.mk_eve_data()
        self.__defsrc_stack_alias_map[key] = data.alias
        return data

    @property
    def _eve_datas(self) -> dict[str, EveObjects]:
        return self.__datas
