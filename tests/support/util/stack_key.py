import inspect
import os.path

from tests import TEST_FOLDER_SPLIT


def frame_to_primitive(frame, ignore_local_context=False):
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


def is_support_path(path):
    split_path = os.path.normpath(os.path.realpath(path)).split(os.sep)
    # Not support path if it's a path outside of tests folder altogether
    if split_path[:len(TEST_FOLDER_SPLIT)] != TEST_FOLDER_SPLIT:
        return False
    split_path = split_path[len(TEST_FOLDER_SPLIT):]
    # Test folder itself also isn't support path
    if len(split_path) == 0:
        return False
    return split_path[0] == 'support'


def get_stack_key():
    stack = inspect.stack(context=0)
    # Filter out stack entries for entities in support folder
    stack = [f for f in stack if not is_support_path(f.filename)]
    # For method which tried to retrieve data, ignore all its local context,
    # to refer to the same data on different calls
    key = [frame_to_primitive(stack[0], ignore_local_context=True)]
    key += [frame_to_primitive(f) for f in stack[1:]]
    return tuple(key)
