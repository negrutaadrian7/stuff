package tp08;

public interface List<E> {
    boolean add(E element);

    void add(int index, E element);

    boolean addAll(List<? extends E> list);

    boolean addAll(int index, List<? extends E> list);

    void clear();

    boolean contains(Object object);

    boolean containsAll(List<?> list);

    E get(int index);

    boolean isEmpty();

    E remove(int index);

    boolean remove(Object object);

    boolean removeAll(List<?> list);

    E set(int index, E element);

    int size();
}
