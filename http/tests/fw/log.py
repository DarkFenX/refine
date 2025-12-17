import contextlib
import os
import queue
import re
import time
import typing
from dataclasses import dataclass
from enum import StrEnum, unique
from threading import Thread

from fw.util import Timer

if typing.TYPE_CHECKING:
    from collections.abc import Generator
    from pathlib import Path


class ParseError(Exception):
    pass


class LogEntryNotFoundError(Exception):
    pass


@unique
class Level(StrEnum):
    error = 'ERROR'
    warning = 'WARN'
    info = 'INFO'
    debug = 'DEBUG'
    trace = 'TRACE'


@dataclass(kw_only=True)
class LogEntry:

    timestamp: str
    level: Level
    span: str
    msg: str

    def check(
            self, *,
            msg: str,
            level: Level | str | None = None,
            span: str | None = None,
    ) -> bool:
        # Span of None just means no span specified
        if span is not None and not self.span.endswith(span):
            return False
        # Level of None means we do not check level
        if level is not None and level != self.level:
            return False
        # Regex matching based on "re:" prefix
        if msg[:3] == 're:':
            pattern = msg[3:]
            if not re.match(pattern, self.msg):
                return False
        elif msg != self.msg:
            return False
        return True


class LogReader:

    TIMESTAMP_PATTERN = r'\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{3}'
    LEVEL_PATTERN = '|'.join(Level)
    LOG_MATCHER = re.compile(
        fr'^\[(?P<timestamp>{TIMESTAMP_PATTERN})]\s+'
        fr'(?P<level>{LEVEL_PATTERN})\s'
        fr'((?P<span>\S+): )?'
        fr'(?P<msg>.*)\n$')

    def __init__(self, *, path: Path) -> None:
        self.__path: Path = path
        self.__collectors: list[LogCollector] = []
        self.__execute_flag: bool = False

    def __add_collector(self, *, collector: LogCollector) -> None:
        self.__collectors.append(collector)

    def __remove_collector(self, *, collector: LogCollector) -> None:
        self.__collectors.remove(collector)

    def run(self) -> None:
        self.__execute_flag = True
        t = Thread(target=self.__execute)
        t.start()

    def stop(self) -> None:
        self.__execute_flag = False

    @contextlib.contextmanager
    def get_collector(self) -> Generator[LogCollector]:
        collector = LogCollector()
        self.__add_collector(collector=collector)
        try:
            yield collector
        finally:
            self.__remove_collector(collector=collector)

    def __follow(self) -> Generator[str]:
        self.__path.touch(mode=0o644, exist_ok=True)
        with self.__path.open() as f:
            f.seek(0, os.SEEK_END)
            while self.__execute_flag:
                try:
                    line = f.readline()
                # Can sometimes happen if multibyte symbol got read mid-write, or if seek() put
                # read position mid-multibyte-symbol
                except UnicodeDecodeError:
                    time.sleep(0.1)
                    continue
                if not line:
                    time.sleep(0.1)
                    continue
                yield line

    def __parse(self, *, line: str) -> LogEntry:
        m = re.match(self.LOG_MATCHER, line)
        if not m:
            raise ParseError(line)
        return LogEntry(
            timestamp=m.group('timestamp'),
            level=Level(m.group('level')),
            span=m.group('span'),
            msg=m.group('msg'))

    def __execute(self) -> None:
        for line in self.__follow():
            # Should happen only if we were asked to stop following
            if line is None:
                return
            # Don't waste time on parsing when nobody is going to take it anyway
            if not self.__collectors:
                continue
            try:
                entry = self.__parse(line=line)
            except ParseError as e:
                for target in self.__collectors:
                    target.append_error(error=e)
                continue
            for target in self.__collectors:
                target.append_entry(entry=entry)


class LogCollector:

    def __init__(self) -> None:
        self.__buffer: queue.SimpleQueue[LogEntry] = queue.SimpleQueue()
        self.__errors: list[ParseError] = []
        self.__collecting: bool = True

    def append_error(self, *, error: ParseError) -> None:
        if self.__collecting:
            self.__errors.append(error)

    def append_entry(self, *, entry: LogEntry) -> None:
        if self.__collecting:
            self.__buffer.put(entry)

    def wait_log_entry(
            self, *,
            msg: str,
            level: Level | str | None = None,
            span: str | None = None,
            timeout: float = 1,
    ) -> None:
        timer = Timer(timeout=timeout)
        while True:
            try:
                entry = self.__buffer.get(timeout=timer.remainder)
            except queue.Empty as e:
                e_msg = f'cannot find log entry with level {level}, span {span}, message "{msg}"'
                raise LogEntryNotFoundError(e_msg) from e
            if entry.check(msg=msg, level=level, span=span):
                return
            # Prevent more entries getting into queue after timeout while checking remaining ones
            if timer.remainder == 0:
                self.__collecting = False

    @property
    def buffer(self) -> queue.SimpleQueue[LogEntry]:
        return self.__buffer

    @property
    def errors(self) -> list[ParseError]:
        return self.__errors
