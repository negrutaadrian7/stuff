public class Multiplication extends BinarryExpression {

    public Multiplication (Expression e1, Expression e2) {
        super(e1, e2); // call the parent constructor, in this case BinarryExpression constructor
    }


    @Override public double evalue(){
        return this.leftOp.evalue() * this.rightOp.evalue();
    }


}