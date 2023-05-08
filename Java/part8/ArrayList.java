import java.util.Arrays;

// private methodes pour gerer les cas de capacite dans la classe
public class ArrayList<E> extends AbstractList<E> {
    private static final int INITIAL_CAPACITY = 8;
    private Object[] array; // tableau interne qui stocke les elements; tableau de type object qu'on passe
                            // en parametre
    private int size;

    public ArrayList() {
        array = new Object[INITIAL_CAPACITY];
    }

    private int getCapacity() {
        return array.length;
    }

    // Redimensionne le tableau interne si necessaire pour atteindre la capacite
    // demandee
    public void ensureCapacity(int minCapacity) {
        if (getCapacity() < minCapacity) {
            increaseCapacityTo(minCapacity); // on double la dimension maximale a 2
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
        array = Arrays.copyOf(array, newCapacity); // truncating with nulls, for the rest of the elements
    }

    private void checkIndexForAdd(int index) {
        if (index < 0 || index > size()) {
            throw new IndexOutOfBoundsException();
        }
    }

    private void checkIndex(int index) {
        if (index < 0 || index >= size()) {
            throw new IndexOutOfBoundsException();
        }
    }

    @Override
    public void add(int index, E element) { // ajoute un element a un index donne

        checkIndexForAdd(index);
        ensureCapacity();
        // on touche pas les elements apres le index
        System.arraycopy(array, index, array, index + 1, size() - index); // on deplace le tableau d'unindice

        array[index] = element;
        size++;
    }

    // insertion des elements et pas juste des modifications aux indices
    // en gros il s'agit de la concatenation d'un element avec une liste, sous ocaml
    // : element :: tailListe
    @Override
    public boolean addAll(List<? extends E> list, int index) {
        checkIndexForAdd(index);
        int deltaSize = list.size(); // longueur pour la liste qu'on veut ajouter
        ensureCapacity(size() + deltaSize);
        System.arraycopy(array, index, array, index + deltaSize, size() - index); // les elements sont decales
        for (int i = 0; i < deltaSize; i++) {
            array[index + 1] = list.get(i);
        }
        size += list.size();
        return deltaSize != 0;
    }

    @Override
    public void clear() {
        size = 0;
    }

    @Override
    @SuppressWarnings("unchecked") // we can suppress the warning associated with our raw type usage
    public E get(int index) {
        checkIndex(index);
        return (E) array[index];
    }

    @Override
    public E remove(int index) {
        checkIndex(index);
        E old = get(index);

        System.arraycopy(array, index + 1, array, index, size() - index - 1);

        size--;
        return old;
    }

    @Override
    public int size() {
        return size;
    }

}
