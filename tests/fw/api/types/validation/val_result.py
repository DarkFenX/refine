import typing

from fw.consts import ApiValStatus

if typing.TYPE_CHECKING:
    from collections.abc import Callable


class ValResult:

    def __init__(self, *, data: list | tuple, cls: Callable) -> None:
        self.status: ApiValStatus = ApiValStatus(data[0])
        # Only statuses which have failing items carry failure details
        if self.status == ApiValStatus.passed:
            return
        self.fail = cls(data=data[1])

    def __eq__(self, other: ValResult) -> bool:
        if not isinstance(other, ValResult):
            return NotImplemented
        return vars(self) == vars(other)

    def __repr__(self) -> str:
        return f'{type(self).__name__}({vars(self)!r})'
