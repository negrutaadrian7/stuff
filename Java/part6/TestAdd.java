public class TestAdd {
    public static void main(String[] args) throws IdException, ArithmeticException {
            
        Constante cons = new Constante (2.0);
        Variable var = new Variable("v1", 42.0);
        Constante cons2 = new Constante (1.0);
        
        Division div = new Division (cons, var);
        Add add = new Add (div, cons2);
        
        System.out.println("(" + add + ")");
        
        
        // les classes modifiees: Variable, Constante, Division, Add
        
    }
}