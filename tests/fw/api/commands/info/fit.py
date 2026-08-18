import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import InfoMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseInfoFitCmd(BaseCommand):

    fit_mode: InfoMode | type[Absent]
    item_mode: InfoMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_info'
        conditional_insert(container=body, path=['fit_mode'], value=self.fit_mode)
        conditional_insert(container=body, path=['item_mode'], value=self.item_mode)
        return body


@dataclasses.dataclass(kw_only=True)
class FitInfoFitCmd(BaseInfoFitCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolInfoFitCmd(BaseInfoFitCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
