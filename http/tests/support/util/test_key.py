import inspect
import os.path

from tests import TEST_FOLDER_SPLIT


class TestKey(tuple):
    pass


def frame_to_primitive(*, frame, ignore_local_context=False) -> tuple:
    if ignore_local_context:
        return (
            frame.filename,
            frame.function)
    pos = frame.positions
    return (
        frame.filename,
        frame.lineno,
        frame.function,
        pos.lineno,
        pos.end_lineno,
        pos.col_offset,
        pos.end_col_offset)


def is_path_in_test_folder(*, path: str) -> bool:
    # Not a test path if it's a path outside of tests folder altogether
    split_path = os.path.normpath(os.path.realpath(path)).split(os.sep)
    if split_path[:len(TEST_FOLDER_SPLIT)] != TEST_FOLDER_SPLIT:
        return False
    split_path = split_path[len(TEST_FOLDER_SPLIT):]
    # Test folder itself
    if len(split_path) == 0:
        return True
    # Support folder
    if split_path[0] == 'support':
        return False
    return True


def is_test_run_func(*, path: str, func: str) -> bool:
    split_path = os.path.normpath(os.path.realpath(path)).split(os.sep)
    if not split_path[-1].startswith('test_'):
        return False
    if not func.startswith('test_'):
        return False
    return True


def get_test_key() -> TestKey:
    """
    This function is supposed to give key (= hashable entity) which is unique for each test, and
    stays the same for the duration of the test.
    """
    stack = inspect.stack(context=0)
    # Include only frames from test folder
    stack = [f for f in stack if is_path_in_test_folder(path=f.filename)]
    # Start stack from test function
    test_frame = next((f for f in stack if is_test_run_func(path=f.filename, func=f.function)), None)
    if test_frame is not None:
        test_frame_index = stack.index(test_frame)
        stack = stack[test_frame_index:]
    # For test method, or a method which tried to retrieve data, ignore all its local context, to
    # refer to the same data on different calls. Unsure if this part is used at all after
    key = [frame_to_primitive(frame=stack[0], ignore_local_context=True)]
    key += [frame_to_primitive(frame=f) for f in stack[1:]]
    return TestKey(key)
