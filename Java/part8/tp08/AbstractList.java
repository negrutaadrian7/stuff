package tp08;

import java.util.Objects;

public abstract class AbstractList<E> implements List<E> {
    public boolean add(E element) {
        add(size(), element);
        return true;
    }

    public boolean addAll(List<? extends E> list) {
        return addAll(size(), list);
    }

    public boolean addAll(int index, List<? extends E> list) {
        for (int i = 0; i < list.size(); i++) {
            add(index + i, list.get(i));
        }
        return !list.isEmpty();
    }

    public void clear() {
        int size = size();
        while (size-- > 0) {
            remove(0);
        }
    }

    public boolean contains(Object object) {
        for (int i = 0; i < size(); i++) {
            if (Objects.equals(object, get(i))) {
                return true;
            }
        }
        return false;
    }

    public boolean containsAll(List<?> list) {
        for (int i = 0; i < list.size(); i++) {
            if (!contains(list.get(i))) {
                return false;
            }
        }
        return true;
    }

    public boolean isEmpty() {
        return size() == 0;
    }

    public boolean remove(Object object) {
        for (int i = 0; i < size(); i++) {
            if (object.equals(get(i))) {
                remove(i);
                return true;
            }
        }
        return false;
    }

    public boolean removeAll(List<?> list) {
        boolean hasChanged = false;
        for (int i = 0; i < size();) {
            if (list.contains(get(i))) {
                remove(i);
                hasChanged = true;
            } else {
                i++;
            }
        }
        return hasChanged;
    }

    public E set(int index, E element) {
        E old = remove(index);
        add(index, element);
        return old;
    }

    @Override
    public String toString() {
        if (isEmpty()) {
            return "[ ]";
        }
        StringBuilder builder = new StringBuilder();
        builder.append('[');
        builder.append(get(0));
        for (int i = 1; i < size(); i++) {
            builder.append(", ");
            builder.append(get(i));
        }
        builder.append(']');
        return builder.toString();
    }
}
