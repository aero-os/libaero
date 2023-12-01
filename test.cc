#include <cstdint>
#include <iostream>

extern "C" {
    void libaero_get_own_stackinfo(void **stack_addr, std::size_t *stack_size);
}

int main() {
    void *stack_addr;
    std::size_t stack_size;
    libaero_get_own_stackinfo(&stack_addr, &stack_size);
    std::cout << "stack_addr: " << stack_addr << std::endl;
    return 0;
}