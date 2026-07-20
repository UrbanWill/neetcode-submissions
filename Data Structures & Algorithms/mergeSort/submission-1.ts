/** Pair class to store key-value pairs */
// class Pair {
//   /**
//    * @param {number} key The key to be stored in the pair
//    * @param {string} value The value to be stored in the pair
//    */
//   constructor(key, value) {
//       this.key = key;
//       this.value = value;
//   }
// }
class Solution {
  /**
   * @param {Pair[]} pairs
   * @returns {Pair[]}
   */
  mergeSort(pairs: Pair[]): Pair[] {
    return this.mergeSortHelper(pairs, 0, pairs.length - 1);
  }

  mergeSortHelper(pairs, s, e) {
    if (e - s + 1 <= 1) {
      return pairs;
    }

    const m = Math.floor((s + e) / 2);

    // Sort left subarray
    this.mergeSortHelper(pairs, s, m);

    // Sort the right subarray
    this.mergeSortHelper(pairs, m + 1, e);

    // Merge sorted subarray
    this.merge(pairs, s, m, e);

    return pairs;
  }

  merge(arr, s, m, e) {
    const l = arr.slice(s, m + 1);
    const r = arr.slice(m + 1, e + 1);

    let i = 0;
    let j = 0;
    let k = s;

    // Merge subarrays
    while (i < l.length && j < r.length) {
      if (l[i].key <= r[j].key) {
        arr[k] = l[i];
        i += 1;
      } else {
        arr[k] = r[j];
        j += 1;
      }
      k += 1;
    }

    while (i < l.length) {
      arr[k] = l[i];
      i += 1;
      k += 1;
    }

    while (j < r.length) {
      arr[k] = r[j];
      j += 1;
      k += 1;
    }
  }
}
