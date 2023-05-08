package tp08;

import java.util.Arrays;

public class ArrayList<E> extends AbstractList<E> {

    private static final int INITIAL_CAPACITY = 8;
    private Object[] array;

    private int size;

    public ArrayList() {
        array = new Object[INITIAL_CAPACITY];
    }

    private int getCapacity() {
        return array.length;
    }

    public void ensureCapacity(int minCapacity) {
        if (getCapacity() < minCapacity) {
            increaseCapacityTo(minCapacity);
        }
    }

    private void ensureCapacity() {
        if (getCapacity() == size()) {
            increaseCapacity();
        }
    }

    private void increaseCapacity() {
        increaseCapacityTo(2 * getCapacity());
    }

    private void increaseCapacityTo(int newCapacity) {
        array = Arrays.copyOf(array, newCapacity);
    }

    private void checkIndexForAdd(int index) {
        if (index < 0 || index > size()) {
            throw new IndexOutOfBoundsException();
        }
    }

    @Override
    public void add(int index, E element) {
        checkIndexForAdd(index);
        ensureCapacity();
        System.arraycopy(array, index, array, index + 1, size() - index);
        array[index] = element;
        size++;
    }

    private void checkIndex(int index) {
        if (index < 0 || index >= size()) {
            throw new IndexOutOfBoundsException();
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public E get(int index) {
        checkIndex(index);
        return (E) array[index];
    }

    @Override
    public E remove(int index) {
        E element = get(index);
        System.arraycopy(array, index + 1, array, index, size() - index - 1);
        size--;
        return element;
    }

    @Override
    public E set(int index, E element) {
        E oldElement = get(index);
        array[index] = element;
        return oldElement;
    }

    @Override
    public int size() {
        return size;
    }

    /* Optimisation des implémentations par défaut */

    public boolean addAll(int index, List<? extends E> list) {
        checkIndexForAdd(index);
        int deltaSize = list.size();
        ensureCapacity(size() + deltaSize);
        System.arraycopy(array, index, array, index + deltaSize, size() - index);
        for (int i = 0; i < deltaSize; i++) {
            array[index + i] = list.get(i);
        }
        size += list.size();
        return deltaSize != 0;
    }

    @Override
    public void clear() {
        size = 0;
    }

    /* map et flatMap, surtout intéressant pour les signatures */

    <R> List<R> map(Function<? super E, ? extends R> f) {
        ArrayList<R> mappedList = new ArrayList<>();
        mappedList.ensureCapacity(getCapacity());
        for (int i = 0; i < size(); i++) {
            mappedList.add(f.apply(get(i)));
        }
        return mappedList;
    }

    <R> List<R> flatMap(Function<? super E, ? extends List<? extends R>> f) {
        ArrayList<R> mappedList = new ArrayList<>();
        for (int i = 0; i < size(); i++) {
            mappedList.addAll(f.apply(get(i)));
        }
        return mappedList;
    }
}
