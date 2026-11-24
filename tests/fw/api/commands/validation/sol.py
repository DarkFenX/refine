import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_val_options_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.types import ValOptions
    from fw.consts import ApiValResultMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class SolValSolCmd(BaseCommand):

    options: ValOptions | type[Absent]
    fit_ids: list[str] | type[Absent]
    result_mode: ApiValResultMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'sol_validate'
        conditional_insert(container=body, path=['options'], value=process_val_options_request(options=self.options))
        conditional_insert(container=body, path=['fit_ids'], value=self.fit_ids)
        conditional_insert(container=body, path=['result_mode'], value=self.result_mode)
        return body
