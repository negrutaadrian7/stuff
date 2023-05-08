public class IdException extends Exception {
    private String id;

    public IdException(String s) {
        super("Id occupe " + s); 
        this.id = s;
    }



}