from collections.abc import Callable

from fw.eve.containers import EvePrimitives, EveStrings

type DataPrimHook = Callable[[EvePrimitives], None]
type DataStrHook = Callable[[EveStrings], None]

type SwBuffs = dict[int, float] | tuple[dict[int, float], int]
type SeBuffs = dict[int, float]
type PeBuffs = dict[int, float]
type PtBuffs = dict[int, float] | tuple[dict[int, float], int]
type SlBuffs = dict[int, float] | tuple[dict[int, float], int]
