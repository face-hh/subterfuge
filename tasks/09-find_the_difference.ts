/**
 * @requires Loops
 * @requires MutableVar
 */
function findTheDifference(s, t) {
    for (let letter of s)
        t = t.replace(letter, '');
    return t;
};