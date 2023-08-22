#include <iostream>
#include <windows.h>
#include <string>
#include <chrono>

int main() {
    HANDLE hPipe;

    hPipe = CreateFile(
        "\\\\.\\pipe\\my_bidirectional_pipe", // Nome do pipe
        GENERIC_READ | GENERIC_WRITE,        // Acesso bidirecional
        0,
        NULL,
        OPEN_EXISTING,
        0,
        NULL
    );

    if (hPipe == INVALID_HANDLE_VALUE) {
        std::cerr << "Failed to connect to named pipe." << std::endl;
        return 1;
    }

    while (true) {
        std::string message;
        std::cout << "Enter message: ";
        std::getline(std::cin, message);

        DWORD bytesWritten;
        WriteFile(hPipe, message.c_str(), message.size(), &bytesWritten, NULL);

        char buffer[1024];
        DWORD bytesRead;
        ReadFile(hPipe, buffer, sizeof(buffer), &bytesRead, NULL);
        std::string receivedResponse(buffer, bytesRead);
        std::cout << "Received response: " << receivedResponse << std::endl;
    }

    CloseHandle(hPipe);

    return 0;
}
