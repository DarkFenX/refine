import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiValInfoMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class SolValSolCmd(BaseCommand):

    options: dict | type[Absent]
    fit_ids: list[str] | type[Absent]
    info_mode: ApiValInfoMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'sol_validate'
        conditional_insert(container=body, path=['options'], value=self.options)
        conditional_insert(container=body, path=['fit_ids'], value=self.fit_ids)
        conditional_insert(container=body, path=['info_mode'], value=self.info_mode)
        return body
