import java.util.Objects;

/* 
Dans la classe abstraite on override les methodes qui sont plus generales 
c-a-d a partir desquelles on peut deduire des methodes plus particulieres
Par ex: addAll(int, List <E>) peut etre deduite de add(int, E) donc on utilise
add(int, E) pour construire add(int, List<E>)
*/


abstract class AbstractList<E> implements List<E>
{
    public boolean add (E element)
    {
        add(size(), element);
        return true;
    }



    public boolean addAll(List<? extends E> maListe)
    {
        return addAll(maListe, size()); // implemented later in ArrayList
    }
    
    
    public boolean addAll (List <? extends E> list, int indice)
    {
        for (int i = 0; i < list.size(); i++) {
            add(indice + i, list.get(i));
        }
        return !list.isEmpty(); // on sait que si la liste contient des elements, inevitablement on les ajoute 

    }
    
    
    
    public void clear ()
    {
        int size = size ();
        while (size-- > 0) {
            remove(0); // on decale a gauche, donc on efface chaque fois l'indice 0 
        }
    }

    
    
    public boolean contains(Object object) 
    {
        for (int i = 0; i < size(); i++) {
            if (Objects.equals(object, get(i))) {
                return true;
            }
        }
        return false;
    }
    
    public boolean containsAll (List<?> list) 
    {
        for (int i = 0; i < list.size(); i++){
            if (!contains(list.get(i))){
                return false;
            }
        }
        return true;

    }

    public boolean isEmpty ()
    {
        return size() == 0; 
    }

    public boolean remove (Object object)
    {
        boolean hasModified = false;
        for (int i = 0; i < size(); i++){
            if (object.equals(get(i))){ // work the same as (==) 
                remove(i);
                hasModified = true;
            }
        }

        return hasModified;
    }


    // supprime toutes les occurences des elements d'une liste passe en parametre

    public boolean removeAll (List<?> list) 
    {
        boolean hasChanged = false;
        for (int i = 0; i < size(); i++) {
            if (list.contains(get(i))) {
                remove(i);
                hasChanged = true;
            } 
        }
        return hasChanged;
    }

    public E set(int index, E element) 
    {
        E old = remove(index);
        add(index, element);
        return old;
    }

    @Override
    public String toString() 
    {
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
