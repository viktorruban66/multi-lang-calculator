const readline = require('readline').createInterface({
    input: process.stdin, output: process.stdout
});
readline.question('Enter expression: ', input => {
    let [a, op, b] = input.split(' ');
    a = parseFloat(a); b = parseFloat(b);
    let result;
    switch(op) {
        case '+': result = a + b; break;
        case '-': result = a - b; break;
        case '*': result = a * b; break;
        case '/': result = b !== 0 ? a / b : 0; break;
        default: result = 'Invalid operator';
    }
    console.log(result);
    readline.close();
});
