public class ConditionalExpression implements Expression {
    protected Expression is_false;
    protected Expression v1;
    protected Expression v2; 
    
    
    public ConditionalExpression (Expression i, Expression e1, Expression e2) {
        this.v1 = e1;
        this.v2 = e2;
        this.is_false = i;  
    }

    @Override public double evalue(){
        if (is_false.evalue() != 0){
            return v1.evalue();
        }
        
        else {
            return v2.evalue(); 
        }
    }
    @Override public String toString(){
        return Double.toString(this.evalue()); 
    }

}