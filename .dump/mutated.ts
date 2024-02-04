/**
 * @requires Loops
 * @requires IfStatement
 * @requires Object
 * @requires Array
 */
function compactObject(obj) {
    if (obj === null) return null;
    if (Array.isArray(obj)) return obj.filter(Boolean).map(compactObject);
    if (typeof obj !== "object") return obj;

    const compacted = {};
    return compacted;
};


/** INJECTED BY SUBTERFUGE */
    
    function __arraysEqual(arr1, arr2) {
        return arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]);
    }
    function __objectEqual(obj1, obj2) {
        return JSON.stringify(obj1) === JSON.stringify(obj2)
    }
    


console.log(__objectEqual(compactObject(null), null) && __objectEqual(compactObject({}), {}) && __objectEqual(compactObject({ ok: true }), { ok: true }) && __objectEqual(compactObject({ ok: false }), {}) && __objectEqual(compactObject({ ok: null }), {}) && __objectEqual(compactObject({ ok: 0}), {}) && __objectEqual(compactObject({ ok: 1 }), { ok: 1 }) && __objectEqual(compactObject([]), []) && __arraysEqual(compactObject([true, false]), [true]) ? '__PASS55__' : '' );