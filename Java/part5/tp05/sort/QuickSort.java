package tp05.sort;

public class QuickSort implements Sort {
    public int partition(int[] t, int from, int to) {
        int i = from + 1; // urmatorul element de la start 
        for (int j = from + 1; j < to; j++) {
            if (t[j] < t[from]) { 
                ArrayUtil.swap(t, i, j);
                i += 1;
            }
        }
        ArrayUtil.swap(t, from, i - 1);
        return i - 1;
    }

    @Override
    public void sort(int[] t, int from, int to) {
        if (to > from + 1) {
            int i = partition(t, from, to);
            sort(t, from, i);
            sort(t, i + 1, to);
        }
    }
}
