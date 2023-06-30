import os

import pytest

pytest.register_assert_rewrite(
    'tests.support.client.client',
    'tests.support.api_data.ss',
    'tests.support.api_data.fit',
    'tests.support.api_data.item')

TEST_FOLDER_SPLIT = os.path.dirname(os.path.normpath(os.path.realpath(__file__))).split(os.sep)
