/**
 * @requires Array      This task requires arrays.
*/

function f(index, times, sequence) {
    index < times &&
        (sequence.push(sequence[index - 1] + sequence[index - 2]),
        f(index + 1, times, sequence));
}

function fibonacci(index, times){
    const sequence = [0, 1];

    f(index, times, sequence)

    return sequence
}