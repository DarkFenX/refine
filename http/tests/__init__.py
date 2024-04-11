import os

import pytest

pytest.register_assert_rewrite(
    'tests.support.api.client',
    'tests.support.api.types.ss',
    'tests.support.api.types.fit',
    'tests.support.api.types.item',
    'tests.support.response',)

TEST_FOLDER_SPLIT = os.path.dirname(os.path.normpath(os.path.realpath(__file__))).split(os.sep)
