import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.types import ValOptions
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseTryItemsFitCmd(BaseCommand):

    type_ids: list[int]
    val_options: ValOptions | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_try_items'
        body['type_ids'] = self.type_ids
        conditional_insert(container=body, path=['val_options'], value=self.val_options)
        return body


@dataclasses.dataclass(kw_only=True)
class FitTryItemsFitCmd(BaseTryItemsFitCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolTryItemsFitCmd(BaseTryItemsFitCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
