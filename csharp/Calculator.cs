using System;
class Calculator {
    static void Main() {
        Console.Write("Enter expression: ");
        string[] input = Console.ReadLine().Split(' ');
        double a = double.Parse(input[0]);
        char op = char.Parse(input[1]);
        double b = double.Parse(input[2]);
        double result = op switch {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => b != 0 ? a / b : 0,
            _ => 0
        };
        Console.WriteLine(result);
    }
}
