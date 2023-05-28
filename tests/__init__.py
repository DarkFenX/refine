import os

import pytest

pytest.register_assert_rewrite('tests.support.client.client')

TEST_FOLDER_SPLIT = os.path.dirname(os.path.normpath(os.path.realpath(__file__))).split(os.sep)
