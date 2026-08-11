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
    quickSort(pairs: Pair[]): Pair[] {
        this.quickSortHelper(pairs, 0, pairs.length - 1);

        return pairs;
    }

    quickSortHelper(pairs: Pair[], s: number, e: number) {
        if (e - s + 1 <= 1) {
            return pairs;
        }

        const pivot = pairs[e];
        let l = s;

        for (let i = s; i < e; i++) {
            if (pairs[i].key < pivot.key) {
                const temp = pairs[i];
                pairs[i] = pairs[l];
                pairs[l] = temp;
                l += 1;
            }
        }
        
        pairs[e] = pairs[l];
        pairs[l] = pivot;

        this.quickSortHelper(pairs, s, l - 1);
        this.quickSortHelper(pairs, l + 1, e);

        return pairs;

    }
}
