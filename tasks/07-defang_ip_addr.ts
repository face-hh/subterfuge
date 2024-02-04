/**
 * @requires RegularExpression
 */
function defangIPaddr(address) {
    return address.replace(/\./g, "[.]")
}