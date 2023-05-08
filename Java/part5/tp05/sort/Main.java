package tp05.sort;

import java.util.Arrays;

public class Main {

    private static final Sort selectionSort = new SelectionSort();
    private static final Sort bubbleSort = new BubbleSort();
    private static final Sort quickSort = new QuickSort();
    private static final Sort quickSortOptimized = new QuickSortOptimized();

    private static void testSort(Sort sort, int[] t) {
        int[] t2 = Arrays.copyOf(t, t.length);
        sort.sort(t2);
        System.out.println(Arrays.toString(t2));
    }

    public static void main(String[] args) {
        int[] t = ArrayUtil.getRandomArray(40);

        testSort(selectionSort, t);
        testSort(bubbleSort, t);
        testSort(quickSort, t);
        testSort(quickSortOptimized, t);

        int n = 6;
        int number = 1000000;

        System.out.printf("Selection, bubble and quick sort (time for %d instances)%n", number);
        for (int i = 0, j = 1; i < n + 1; i++, j *= 2) {
            Instance instance = new Instance(j, number);
            System.out.printf("Length = %3d : %5d ms, %5d ms, %5d ms.%n", j,
                    Benchmark.getTime(selectionSort, instance),
                    Benchmark.getTime(bubbleSort, instance),
                    Benchmark.getTime(quickSort, instance)
            );
        }
    }
}
