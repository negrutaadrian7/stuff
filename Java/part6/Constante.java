public class Constante implements Expression {
    public final double constante; 
    
    public Constante (double cons){
        this.constante = cons; 
    }


    @Override 
    public double evalue() {
        return this.constante; 
    }

    @Override 
    public String toString(){
        return Double.toString (this.constante); 
    }
}