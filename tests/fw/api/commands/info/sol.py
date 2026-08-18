import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import InfoMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class SolInfoFleetCmd(BaseCommand):

    fleet_id: str
    sol_mode: InfoMode | type[Absent]
    fleet_mode: InfoMode | type[Absent]
    fit_mode: InfoMode | type[Absent]
    item_mode: InfoMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'sol_info'
        conditional_insert(container=body, path=['sol_mode'], value=self.sol_mode)
        conditional_insert(container=body, path=['fleet_mode'], value=self.fleet_mode)
        conditional_insert(container=body, path=['fit_mode'], value=self.fit_mode)
        conditional_insert(container=body, path=['item_mode'], value=self.item_mode)
        return body
