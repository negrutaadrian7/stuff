package tp05.sort;

public class Benchmark {

    public static long getTime(Sort sort, Instance instance) {
        int[][] arrays = instance.get();
        long start = System.currentTimeMillis();
        for (int[] array : arrays) {
            sort.sort(array);
        }
        long end = System.currentTimeMillis();
        return end - start;
    }
}
