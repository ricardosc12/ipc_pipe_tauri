#include <iostream>
#include <chrono>

// Função recursiva para calcular o n-ésimo número de Fibonacci
int fibonacci(int n) {
    if (n <= 1)
        return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
    int n = 40;  // Você pode ajustar o valor de 'n' para controlar a complexidade do cálculo

    auto start = std::chrono::high_resolution_clock::now();
    int result = fibonacci(n);
    auto end = std::chrono::high_resolution_clock::now();

    std::chrono::duration<double> duration = end - start;
    std::cout << "Fibonacci(" << n << ") = " << result << std::endl;
    std::cout << "Tempo de execução: " << duration.count() << " segundos" << std::endl;

    return 0;
}