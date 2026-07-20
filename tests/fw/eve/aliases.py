from collections.abc import Callable

from fw.eve.containers import EvePrimitives, EveStrings

type DataPrimHook = Callable[[EvePrimitives], None]
type DataStrHook = Callable[[EveStrings], None]
