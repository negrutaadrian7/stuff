package tp05.sort;

import java.util.Arrays;

public class Instance {

    private final int[][] arrays;

    public Instance(int length, int number) {
        arrays = new int[number][];
        for (int i = 0; i < number; i++) {
            arrays[i] = ArrayUtil.getRandomArray(length);
        }
    }

    public int[][] get() {
        int number = arrays.length;
        int[][] copy = new int[number][];
        for (int i = 0; i < number; i++) {
            copy[i] = Arrays.copyOf(arrays[i], arrays[i].length);
        }
        return copy;
    }
}
