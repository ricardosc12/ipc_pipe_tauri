#include <iostream>
#include <string>
#include <windows.h>

struct PipeThreadArgs
{
    int *num;
};

DWORD WINAPI message_thread(LPVOID lpParameter)
{
    PipeThreadArgs *args = (PipeThreadArgs *)lpParameter;
    *(args->num) = *(args->num) + 1;
}

int main()
{

    int num = 0;
    PipeThreadArgs *args = new PipeThreadArgs{&num};
    CreateThread(NULL, 0, message_thread, args, 0, NULL);
    CreateThread(NULL, 0, message_thread, args, 0, NULL);

    while (true)
    {
        printf("NUM: %d", num);
        Sleep(1000);
    }
}