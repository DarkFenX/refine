import contextlib
import math
import typing

import pytest

if typing.TYPE_CHECKING:
    from collections.abc import Generator


# Wrapper around pytest approx function, to override default parameters
def approx(expected: float, accuracy: int = 7):  # noqa: ANN201
    # 6 digits after dot for numbers more than 1 and less than -1
    if abs(expected) >= 1:
        tolerance = 10 ** -(accuracy - 1)
        return pytest.approx(expected=expected, abs=tolerance)
    # 7 digits after dot when we expect 0
    if expected == 0:
        tolerance = 10 ** -accuracy
        return pytest.approx(expected=expected, abs=tolerance)
    # 7 significant digits for numbers between 0 and 1/-1
    highest_magnitude = math.floor(math.log10(abs(expected)))
    tolerance = 10 ** (highest_magnitude - accuracy + 1)
    return pytest.approx(expected=expected, abs=tolerance)


@contextlib.contextmanager
def check_no_field():  # noqa: ANN201
    with pytest.raises(AttributeError):
        yield
