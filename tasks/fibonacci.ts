/**
 * @requires Array      This task requires arrays.
 * @requires MutableVar This task requires mutable variables (let).
*/

const sequence = [0, 1];

function fibonacci(index, times) {
    index < times &&
        (sequence.push(sequence[index - 1] + sequence[index - 2]),
        fibonacci(index + 1));
}