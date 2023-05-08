public abstract class BinarryExpression implements Expression { // abstract cause we can't implement the interface here 
    
    protected Expression leftOp;
    protected Expression rightOp; 

    public BinarryExpression(Expression e1, Expression e2){
        leftOp = e1;
        rightOp = e2;
    }

}