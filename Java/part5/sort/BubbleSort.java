package tp05.sort;

public class BubbleSort implements Sort {
    @Override
    public void sort(int[] t, int from, int to) {
        boolean hasModification = true;
        for (int i = from; i < to - 1 && hasModification; i++) {
            hasModification = false;
            for (int j = from; j < to - 1 + from - i; j++) { // j will start from the from index everytime
                if (t[j] > t[j + 1]) {
                    ArrayUtil.swap(t, j, j + 1);
                    hasModification = true;
                }
            }
        }
    }
}
