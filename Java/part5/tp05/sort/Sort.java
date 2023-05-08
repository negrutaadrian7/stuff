package tp05.sort;

// nu este obligatoriu sa efectuam un override la functiile definite cu keyword "default";
public interface Sort {
    
    default void sort(int[] t) {
        sort(t, 0, t.length); // if we pass just an argument to the function so the default method will be with 0 as from and the last element of the array t.length
    }

    void sort(int[] t, int from, int to);
}
