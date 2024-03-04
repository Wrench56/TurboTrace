import utils

class Payload:
    def __init__(self) -> None:
        self._time_elapsed: int = 0
        self._module: int = 0
        self._level: str = ''
        self._message_type: int = 0
        self._message_length: int = 0
        self._message: str = ''
        self._full_length: int = 0
        self._timestamp: int = 0

    def set_level(self, level: str) -> None:
        level = level.upper()
        if (level in ('DEBUG', 'D')):
            self._level = 'D'
        elif (level in ('INFO', 'I')):
            self._level = 'I'
        elif (level in ('WARN', 'W')):
            self._level = 'W'
        elif (level in ('ERROR', 'E')):
            self._level = 'E'
        elif (level in ('CRIT', 'C')):
            self._level = 'C'
        else:
            self._level = 'D'

    def set_message(self, message: str) -> None:
        self._message_length = len(self._message)
        self._message = message

    def set_timestamp(self, timestamp) -> None:
        self._timestamp = timestamp

    def concat_message(self) -> bytearray:
        self._time_elapsed = utils.current_ms_time() - self._timestamp
        message = self._time_elapsed.to_bytes(4) + self._level.encode() + self._module.to_bytes(2) + self._message_type.to_bytes(2)
        if self._message_length > 0:
            message += self._message_length.to_bytes(2) + self._message.encode()
        return message

    def send(self) -> bytearray:
        return self.concat_message()
    
    def full_length(self) -> int:
        return len(self.concat_message)
    
    def __repr__(self) -> str:
        self._message_length = len(self._message)
        self._full_length = len(self.concat_message())
        return f'Payload [{self._full_length} bytes] (\n * Elapsed time: {self._time_elapsed}ms\n * Level: {self._level}\n * Module: {self._module}\n * Message type: {self._message_type}\n * Message length: {self._message_length} byte(s)\n * Message: {self._message}\n)'