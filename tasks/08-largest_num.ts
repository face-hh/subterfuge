/**
 * @requires Array
 * @requires RegularExpression
 */

function largestNumber(num) {
    return num.sort(function(a, b) {
        return (b + '' + a ) - (a + '' + b);
    }).join('').replace(/^0*/,'') || '0';
}