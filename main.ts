"".split('')


function f(index, times, sequence) {
    index < times &&
        (sequence.push(sequence[index - 1] + sequence[index - 2]),
        f(index + 1, times, sequence));
}

let ok = 3;

function fibonacci(index, times){
    const sequence = [0, 1];

    f(index, times, sequence)

    return sequence
}

for(;;) {

}