import pytest

pytest.register_assert_rewrite(
    'fw.api.client.sol',
    'fw.api.client.src',
    'fw.api.types.fit',
    'fw.api.types.sol',
    'fw.response',
    'fw.util.ntt_list',
)

# ruff: noqa: E402
from unittest.mock import ANY as ANY_VALUE

from .api import (
    Effect,
    Muta,
    Spool,
)
from .util import approx, check_no_field
