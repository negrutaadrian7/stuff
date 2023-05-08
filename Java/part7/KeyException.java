public class KeyException extends Exception {
    private int key;

    public KeyException(int s) {
        super("Cle invalide " + s); // call the parent constructor (Exception);
        this.key = s;
    }
}