import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_stats_options_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import StatsOptions
    from fw.api.types import FitStatsOptions, ItemStatsOptions
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseStatsFitCmd(BaseCommand):

    fit_options: StatsOptions[FitStatsOptions] | type[Absent]
    item_options: StatsOptions[ItemStatsOptions] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_stats'
        conditional_insert(
            container=body,
            path=['fit_options'],
            value=process_stats_options_request(options=self.fit_options))
        conditional_insert(
            container=body,
            path=['item_options'],
            value=process_stats_options_request(options=self.item_options))
        return body


@dataclasses.dataclass(kw_only=True)
class FitStatsFitCmd(BaseStatsFitCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolStatsFitCmd(BaseStatsFitCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
