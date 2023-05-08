package tp05.sort;

import java.util.Random;

public final class ArrayUtil {
    private static final Random gen = new Random();

    private ArrayUtil() {} 

    static void swap(int[] t, int i, int j) {
        int tmp = t[i];
        t[i] = t[j];
        t[j] = tmp;
    }

    public static void shuffle(int[] t) {
        for (int i = t.length - 1; i > 0; i--) {
            swap(t, i, gen.nextInt(i + 1));
        }
    }

    public static int[] getRandomArray(int length) {
        int[] t = new int[length];
        for (int i = 0; i < length; i++) {
            t[i] = i;
        }
        shuffle(t);
        return t;
    }
}
