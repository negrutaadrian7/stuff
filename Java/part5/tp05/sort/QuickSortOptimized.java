package tp05.sort;

public class QuickSortOptimized extends QuickSort {
    private static final int THRESHOLD = 16;
    private static final Sort SMALL_SIZE_SORT = new SelectionSort();

    @Override
    public void sort(int[] t, int from, int to) {
        int size = to - from + 1;
        if (size < THRESHOLD) {
            SMALL_SIZE_SORT.sort(t, from, to);
        } else {
            super.sort(t, from, to);
        }
    }
}
