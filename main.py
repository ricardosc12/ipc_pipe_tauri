import win32pipe
import win32file

print(2//2.3)

pipe = win32file.CreateFile(
    r'\\.\pipe\my_bidirectional_pipe',  # Nome do pipe
    win32file.GENERIC_READ | win32file.GENERIC_WRITE,
    0, None, win32file.OPEN_EXISTING, 0, None
)

try:
    while True:
        message = input("Enter message: ")
        win32file.WriteFile(pipe, message.encode())
        buffer = win32file.ReadFile(pipe, 1024)[1]
        response = buffer.decode()
        print("Received response:", response)
except KeyboardInterrupt:
    pass

win32file.CloseHandle(pipe)
