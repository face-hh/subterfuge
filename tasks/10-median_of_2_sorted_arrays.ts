/**
 * @requires Array
 * @requires MutableVar
 * @requires Loops
 * @requires IfStatement
 * @requires ElseStatement
*/
function findMedianSortedArrays(nums1, nums2) {
    const merged = [];
    let i = 0, j = 0;

    while (i < nums1.length && j < nums2.length) {
        if (nums1[i] < nums2[j]) {
            merged.push(nums1[i]);
            i++;
        } else {
            merged.push(nums2[j]);
            j++;
        }
    }

    while (i < nums1.length) {
        merged.push(nums1[i]);
        i++;
    }

    while (j < nums2.length) {
        merged.push(nums2[j]);
        j++;
    }

    const totalLength = merged.length;
    if (totalLength % 2 === 0) {
        const mid = totalLength / 2;
        return (merged[mid - 1] + merged[mid]) / 2;
    } else {
        const mid = Math.floor(totalLength / 2);
        return merged[mid];
    }
}

// console.log(findMedianSortedArrays([0,0.5], [0, 0.5]))