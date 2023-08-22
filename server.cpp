#include <iostream>
#include <windows.h>
#include <string>
#include <chrono>

struct PipeThreadArgs
{
    HANDLE pipe;
    int *numClients;
};

DWORD WINAPI message_thread(LPVOID lpParameter)
{
    char buffer[1024];
    DWORD bytesRead, bytesWritten;
    PipeThreadArgs *args = (PipeThreadArgs *)lpParameter;
    HANDLE hPipe = args->pipe;
    while (true)
    {
        if (ReadFile(hPipe, buffer, sizeof(buffer), &bytesRead, NULL) != FALSE)
        {
            std::string receivedMessage(buffer, bytesRead);
            std::cout << "Received: " << receivedMessage << std::endl;

            if (receivedMessage == "exit")
            {
                break;
            }

            // Respond back to the client
            std::string response = "Server received: " + receivedMessage;
            WriteFile(hPipe, response.c_str(), response.size(), &bytesWritten, NULL);
        }
        else
            break;
    }

    printf("Desconectando pipe !\n");
    *(args->numClients) = *(args->numClients) - 1;
    DisconnectNamedPipe(hPipe);
    CloseHandle(hPipe);
}

int main()
{
    HANDLE hPipe;
    char buffer[1024];
    DWORD bytesRead, bytesWritten;
    int maxClients = 2;
    int numClients = 0;
    while (true)
    {   
        if (numClients < maxClients)
        {
            hPipe = CreateNamedPipe(
                "\\\\.\\pipe\\my_bidirectional_pipe", // Nome do pipe
                PIPE_ACCESS_DUPLEX,                   // Acesso bidirecional
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                maxClients,
                sizeof(buffer),
                sizeof(buffer),
                0,
                NULL);

            if (hPipe == INVALID_HANDLE_VALUE)
            {
                std::cerr << "Failed to create named pipe." << std::endl;
                return 1;
            }

            if (ConnectNamedPipe(hPipe, NULL) != FALSE)
            {
                printf("CONECTOU\n");
                numClients++;
                PipeThreadArgs *args = new PipeThreadArgs{hPipe, &numClients};
                CreateThread(NULL, 0, message_thread, args, 0, NULL);
            }
        }
    }

    DisconnectNamedPipe(hPipe);
    CloseHandle(hPipe);

    return 0;
}
