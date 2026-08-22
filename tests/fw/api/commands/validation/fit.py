import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiValInfoMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseValFitCmd(BaseCommand):

    options: dict | type[Absent]
    info_mode: ApiValInfoMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_validate'
        conditional_insert(container=body, path=['options'], value=self.options)
        conditional_insert(container=body, path=['info_mode'], value=self.info_mode)
        return body


@dataclasses.dataclass(kw_only=True)
class FitValFitCmd(BaseValFitCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolValFitCmd(BaseValFitCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
