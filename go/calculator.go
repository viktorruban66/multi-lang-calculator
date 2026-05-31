package main
import "fmt"

func main() {
    var a, b float64
    var op string
    fmt.Print("Enter expression: ")
    fmt.Scan(&a, &op, &b)
    switch op {
    case "+": fmt.Println(a + b)
    case "-": fmt.Println(a - b)
    case "*": fmt.Println(a * b)
    case "/": 
        if b != 0 { fmt.Println(a / b) } else { fmt.Println(0) }
    default: fmt.Println("Invalid operator")
    }
}
