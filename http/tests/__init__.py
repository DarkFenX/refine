from __future__ import annotations

import contextlib
import math
import typing
from pathlib import Path

import pytest

if typing.TYPE_CHECKING:
    from collections.abc import Iterator

pytest.register_assert_rewrite(
    'tests.fw.api.client.sol',
    'tests.fw.api.client.src',
    'tests.fw.api.types.fit',
    'tests.fw.response')

TEST_FOLDER_SPLIT = Path(__file__).resolve().absolute().parent.parts


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
def check_no_field() -> Iterator[None]:
    with pytest.raises(AttributeError):
        yield


# Effect ID converters
def effect_dogma_to_api(*, dogma_effect_id: int) -> str:
    return f'd{dogma_effect_id}'


def effect_scse_to_api(*, type_id: int) -> str:
    return f'scse{type_id}'


def effect_scpe_to_api(*, type_id: int) -> str:
    return f'scpe{type_id}'


def effect_scpt_to_api(*, type_id: int) -> str:
    return f'scpt{type_id}'


def effect_scsl_to_api(*, type_id: int) -> str:
    return f'scsl{type_id}'
