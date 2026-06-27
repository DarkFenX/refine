from collections.abc import Callable

from fw.eve.containers import EvePrimitives

type DataPrimHook = Callable[[EvePrimitives], None]
