import contextlib
import math
import os

import pytest

pytest.register_assert_rewrite(
    'tests.support.api.client.sol',
    'tests.support.api.client.src',
    'tests.support.api.types.fit',
    'tests.support.response')

TEST_FOLDER_SPLIT = os.path.dirname(os.path.normpath(os.path.realpath(__file__))).split(os.sep)


# Wrapper around pytest approx function, to override default parameters
def approx(expected, accuracy=7):
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
def check_no_field():
    with pytest.raises(AttributeError):
        yield
