

function f(index, times, sequence) {
    index < times &&
        (sequence.push(sequence[index - 1] + sequence[index - 2]),
            f(index + 1, times, sequence));
}

function fibonacci(index, times) {
    const sequence = [0, 1];

    f(index, times, sequence)

    return sequence
}

/** INJECTED */
function arraysEqual(arr1, arr2) { return arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]); }

for (let i = 0; i < 30; i++) { let res = fibonacci(2, i); if (i === 29) { console.log(arraysEqual(res, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811]) ? '__PASS55__' : ''); } }
/** i know this is not efficient :d */