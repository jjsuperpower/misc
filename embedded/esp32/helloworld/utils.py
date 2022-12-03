__all__ = ['logging']

import time

class logging:

    DEBUG = 0
    INFO = 1
    WARN = 2
    ERROR = 3
    CRITICAL = 4

    group = 'SYSTEM'
    stdout = 'serial'
    log_level = INFO

    @classmethod
    def _print(cls, level, *args, **kwargs):
        if cls.stdout == 'serial':
            print(str(time.ticks_ms()) + '::' + cls.group + '::' + str(*args), **kwargs)
        else:
            with open(cls.stdout, 'a') as f:
                f.write(str(time.ticks_ms()) + '::' + cls.group + '::' + str(*args), **kwargs)

    @classmethod
    def debug(cls, *args, **kwargs):
        if cls.log_level <= cls.DEBUG:
            cls._print('DEBUG', *args, **kwargs)
    
    @classmethod
    def info(cls, *args, **kwargs):
        if cls.log_level <= cls.INFO:
            cls._print('INFO', *args, **kwargs)

    @classmethod
    def warn(cls, *args, **kwargs):
        if cls.log_level <= cls.WARN:
            cls._print('WARN', *args, **kwargs)

    @classmethod
    def error(cls, *args, **kwargs):
        if cls.log_level <= cls.ERROR:
            cls._print('ERROR', *args, **kwargs)

    @classmethod
    def critical(cls, *args, **kwargs):
        if cls.log_level <= cls.CRITICAL:
            cls._print('CRITICAL', *args, **kwargs)