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
        return this.mergeSortHelper(pairs, 0, pairs.length -1);
    }

    mergeSortHelper(pairs: Pair[], s: number, e: number) {
        if (e - s + 1 <= 1) {
            return pairs;
        }

        const m = Math.floor((e + s) / 2);

        this.mergeSortHelper(pairs, s, m);

        this.mergeSortHelper(pairs, m + 1, e);

        this.merge(pairs, s, m, e);

        return pairs
    }

    merge(pairs: Pair[], s: number, m: number, e: number) {
        const l = pairs.slice(s, m + 1);
        const r = pairs.slice(m + 1, e + 1);

        let i = 0;
        let j = 0;
        let k = s;

        while (i < l.length && j < r.length) {
            if (l[i].key <= r[j].key) {
                pairs[k] = l[i];
                i += 1;
            } else {
                pairs[k] = r[j];
                j += 1;
            }
            k += 1;
        }

        while (i < l.length) {
            pairs[k] = l[i];
            i += 1;
            k += 1;
        }

        while (j < r.length) {
            pairs[k] = r[j];
            j += 1;
            k += 1;
        }

    }
}
