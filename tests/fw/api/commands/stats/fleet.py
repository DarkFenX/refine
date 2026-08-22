import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_stats_options_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import StatsOptions
    from fw.api.types import FitStatsOptions, FleetStatsOptions, ItemStatsOptions
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class SolStatsFleetCmd(BaseCommand):

    fleet_id: str
    fleet_options: StatsOptions[FleetStatsOptions] | type[Absent]
    fit_options: StatsOptions[FitStatsOptions] | type[Absent]
    item_options: StatsOptions[ItemStatsOptions] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fleet_stats'
        body['fleet_id'] = self.fleet_id
        conditional_insert(
            container=body,
            path=['fleet_options'],
            value=process_stats_options_request(options=self.fleet_options))
        conditional_insert(
            container=body,
            path=['fit_options'],
            value=process_stats_options_request(options=self.fit_options))
        conditional_insert(
            container=body,
            path=['item_options'],
            value=process_stats_options_request(options=self.item_options))
        return body
