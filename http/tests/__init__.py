import contextlib
import math
import os

import pytest


pytest.register_assert_rewrite(
    'tests.support.api.client',
    'tests.support.api.types.sol',
    'tests.support.api.types.fit',
    'tests.support.api.types.item',
    'tests.support.response',)

TEST_FOLDER_SPLIT = os.path.dirname(os.path.normpath(os.path.realpath(__file__))).split(os.sep)


# Wrapper around pytest approx function, to override default parameters
def approx(expected):
    if abs(expected) >= 1:
        return pytest.approx(expected=expected, abs=1e-6)
    if expected == 0:
        return pytest.approx(expected=expected, abs=1e-7)
    # 7 significant digits for numbers between 0 and 1/-1
    highest_magnitude = math.floor(math.log10(abs(expected)))
    tolerance = 10 ** (highest_magnitude - 6)
    return pytest.approx(expected=expected, abs=tolerance)


@contextlib.contextmanager
def check_no_field():
    with pytest.raises(AttributeError):
        yield
