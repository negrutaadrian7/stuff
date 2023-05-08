public class InvalidArg extends Exception {
    private String codeOuDecode ;
    
    public InvalidArg(String s) {
        super("Nombre des arguments invalide " + s); 
        this.codeOuDecode = s;
    }
    
}
