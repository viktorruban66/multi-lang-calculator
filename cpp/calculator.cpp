#include <iostream>
using namespace std;

int main() {
    double a, b;
    char op;
    cout << "Enter expression (e.g., 3 + 5): ";
    cin >> a >> op >> b;
    switch(op) {
        case '+': cout << a + b; break;
        case '-': cout << a - b; break;
        case '*': cout << a * b; break;
        case '/': cout << (b != 0 ? a / b : 0); break;
        default: cout << "Invalid operator";
    }
    return 0;
}
