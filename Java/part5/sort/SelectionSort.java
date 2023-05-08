package tp05.sort;

public final class SelectionSort implements Sort {
    @Override
    public void sort(int[] t, int from, int to) {
        for (int i = from; i < to - 1; i++) {
            int j = getIndexMin(t, i, to);
            ArrayUtil.swap(t, i, j);
        }
    }

    private int getIndexMin(int[] t, int from, int to) {
        int index = from;
        for (int i = from + 1; i < to; i++) {
            if (t[i] < t[index]) {
                index = i;
            }
        }
        return index;
    }
}
