import inspect
import os.path

from tests import TEST_FOLDER_SPLIT


class StackKey(tuple):
    pass


def frame_to_primitive(*, frame, ignore_local_context=False):
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


def is_test_path(*, path: str) -> bool:
    # Not test path if it's a path outside of tests folder altogether
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


def get_stack_key() -> StackKey:
    """
    This function is supposed to give key (= hashable entity) which is unique for each test, and
    stays the same for the duration of the test.

    Current implementation assumes that all calls which eventually use this function have to be done
    from the same function. This translates to need to have all the EVE data set up in one function,
    otherwise you have to explicitly specify data source to use.
    """
    stack = inspect.stack(context=0)
    # Include only frames from test folder
    stack = [f for f in stack if is_test_path(path=f.filename)]
    # For method which tried to retrieve data, ignore all its local context, to refer to the same
    # data on different calls
    key = [frame_to_primitive(frame=stack[0], ignore_local_context=True)]
    key += [frame_to_primitive(frame=f) for f in stack[1:]]
    return StackKey(key)
