public class Division extends BinarryExpression {

    public Division (Expression e1, Expression e2) {
        super(e1, e2); // call the parent constructor, in this case BinarryExpression constructor
    }


    @Override public double evalue() {
        if (this.rightOp.evalue() == 0) throw new ArithmeticException("Division par 0 ");
        return this.leftOp.evalue() / this.rightOp.evalue();
    }

    @Override public String toString(){
        return "(" + this.leftOp + " / " + this.rightOp + ")";
    }


}