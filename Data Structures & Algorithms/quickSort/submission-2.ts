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
        this.quickSortHelper(pairs, 0, pairs.length -1);
        return pairs;
    }

    quickSortHelper(pairs: Pair[], s: number, e: number) {
        if (e - s + 1 <= 1) {
            return;
        }

        const pivot = pairs[e];
        let left = s;

        for (let i = s; i < e; i++) {
            if (pairs[i].key < pivot.key) {
                const temp = pairs[left];
                pairs[left] = pairs[i];
                pairs[i] = temp;
                left += 1;
            }
        }

        pairs[e] = pairs[left];
        pairs[left] = pivot;

        this.quickSortHelper(pairs, s, left - 1);

        this.quickSortHelper(pairs, left + 1, e);
    }
}
