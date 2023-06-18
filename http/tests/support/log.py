import contextlib
import os
import pathlib
import queue
import re
import time
from enum import StrEnum, unique
from threading import Thread

from tests.support.util import Timer, make_repr_str


class ParseError(Exception):
    pass


class LogEntryNotFound(Exception):
    pass


# pylint: disable=C0103
@unique
class Level(StrEnum):
    error = 'ERROR'
    warning = 'WARN'
    info = 'INFO'
    debug = 'DEBUG'
    trace = 'TRACE'


class LogEntry:

    def __init__(self, timestamp, level, span, msg):
        self.timestamp = timestamp
        self.level = level
        self.span = span
        self.msg = msg

    def check(self, msg, level=None, span=None):
        # Span of None just means no span specified
        if span != self.span:
            return False
        # Level of None means we do not check level
        if level is not None and level != self.level:
            return False
        # Regex matching based on "re:" prefix
        if msg[:3] == 're:':
            pattern = msg[3:]
            if not re.match(pattern, self.msg):
                return False
        else:
            if msg != self.msg:
                return False
        return True

    def __repr__(self):
        return make_repr_str(self, spec=['timestamp', 'level', 'span', 'msg'])


class LogReader:

    TIMESTAMP_PATTERN = r'\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{3}'
    LEVEL_PATTERN = '|'.join(Level)
    LOG_MATCHER = re.compile(
        fr'^\[(?P<timestamp>{TIMESTAMP_PATTERN})\]\s+'
        fr'(?P<level>{LEVEL_PATTERN})\s'
        fr'((?P<span>\S+): )?'
        fr'(?P<msg>.*)\n$')

    def __init__(self, path: str) -> None:
        self.__path: str = path
        self.__targets: list[LogCollector] = []
        self.__execute_flag: bool = False

    def __add_target(self, target):
        self.__targets.append(target)

    def __remove_target(self, target):
        self.__targets.remove(target)

    def run(self):
        self.__execute_flag = True
        t = Thread(target=self.__execute)
        t.start()

    def stop(self):
        self.__execute_flag = False

    @contextlib.contextmanager
    def get_collector(self):
        collector = LogCollector()
        self.__add_target(collector)
        try:
            yield collector
        finally:
            self.__remove_target(collector)

    def __follow(self) -> str:
        pathlib.Path(self.__path).touch(mode=0o644, exist_ok=True)
        with open(self.__path, 'r', encoding='utf-8') as f:
            f.seek(0, os.SEEK_END)
            while self.__execute_flag:
                line = f.readline()
                if not line:
                    time.sleep(0.1)
                    continue
                yield line

    def __parse(self, line: str) -> LogEntry:
        m = re.match(self.LOG_MATCHER, line)
        if not m:
            raise ParseError(line)
        return LogEntry(
            timestamp=m.group('timestamp'),
            level=Level(m.group('level')),
            span=m.group('span'),
            msg=m.group('msg'))

    def __execute(self):
        for line in  self.__follow():
            # Should happen only if we were asked to stop following
            if line is None:
                return
            # Don't waste time on parsing when nobody is going to take it anyway
            if not self.__targets:
                continue
            try:
                entry = self.__parse(line)
            except ParseError as e:
                for target in self.__targets:
                    target.append_error(e)
                continue
            for target in self.__targets:
                target.append_entry(entry)


class LogCollector:

    def __init__(self):
        self.__buffer: queue.SimpleQueue[LogEntry] = queue.SimpleQueue()
        self.__errors: list[ParseError] = []

    def append_error(self, error: ParseError) -> None:
        self.__errors.append(error)

    def append_entry(self, entry: LogEntry) -> None:
        self.__buffer.put(entry)

    def wait_log_entry(self, msg, level=None, span=None, timeout=1):
        timer = Timer(timeout=timeout)
        while timer.remainder > 0:
            try:
                entry = self.__buffer.get(timeout=timer.remainder)
            except queue.Empty as e:
                raise LogEntryNotFound(f'cannot find log entry with level {level}, span {span}, message "{msg}"') from e
            if entry.check(msg=msg, level=level, span=span):
                return
        raise LogEntryNotFound(f'cannot find log entry with level {level}, span {span}, message "{msg}"')

    @property
    def buffer(self):
        return self.__buffer

    @property
    def errors(self):
        return self.__errors
