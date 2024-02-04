/**
 * Roman integers (string) to number (int)
 * @requires Object
 * @requires IfStatement
 */
const symbols = {
    "I": 1,
    "V": 5,
    "X": 10,
    "L": 50,
    "C": 100,
    "D": 500,
    "M": 1000
};

function romanToInt(s, index = 0, value = 0) {
    if (index >= s.length) {
        return value;
    }

    const currentSymbolValue = symbols[s[index]];
    const nextSymbolValue = symbols[s[index + 1]];

    if (nextSymbolValue !== undefined && nextSymbolValue > currentSymbolValue) {
        value -= currentSymbolValue;
    } 
    if (nextSymbolValue === undefined || nextSymbolValue <= currentSymbolValue) {
        value += currentSymbolValue;
    }

    return romanToInt(s, index + 1, value);
};



// console.log(romanToInt("CMXCIX"))