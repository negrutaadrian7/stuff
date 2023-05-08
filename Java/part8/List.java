
interface List <E>  { // interface d'un List de type E  
    boolean add (E element) ;
    void add (int indice, E element);
    boolean addAll(List<? extends E> list);
    boolean addAll (List <? extends E> list, int indice);
    void clear ();
    
    // Objects.equals() - static method of the Objects class that
    // accepts two objects and checks if the objects are equal 
    boolean contains (Object object) ;
    boolean containsAll (List<?> list);
    E get (int index) ; 
    boolean isEmpty ();
    E remove (int index) ; 
    boolean remove (Object object);
    boolean removeAll (List<?> list);
    E set (int index, E element);
    int size (); 

}