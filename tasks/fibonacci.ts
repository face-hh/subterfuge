/**
 * @requires Array      This task requires arrays.
 * @requires Loop       This task requires loops (for).
 * @requires Function   This task requires functions.
 * @requires MutableVar This task requires mutable variables (let).
*/

const sequence = [0, 1]
const times = 10

for (let i = 2; i < times; i++) {
    sequence.push(sequence[i - 1] + sequence[i - 2])
}

console.log(sequence)