from __future__ import annotations

import dataclasses
import typing

if typing.TYPE_CHECKING:
    from tests.fw.consts import ApiEffMode


@dataclasses.dataclass(kw_only=True)
class EffectInfo:

    running: bool
    mode: ApiEffMode
