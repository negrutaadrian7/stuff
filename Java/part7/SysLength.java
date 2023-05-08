public class SysLength extends Exception{
    private int nbArguments;
    
    public SysLength(int s) {
        super("Nombre des arguments invalide " + s); 
        this.nbArguments = s;
    }
}
