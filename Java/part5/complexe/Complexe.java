package tp05.complexe;

public interface Complexe {
    // all the methods that not contain the default keyword need to be implemented
    // in the regular classes that will implement this interface
    double reelle();

    double imaginaire();

    double module();

    double argument();

    default Complexe add(Complexe c) {
        return new ComplexeCartesien(this.reelle() + c.reelle(), this.imaginaire() + c.imaginaire());
    }

    default Complexe sub(Complexe c) {
        return new ComplexeCartesien(this.reelle() - c.reelle(), this.imaginaire() - c.imaginaire());
    }

    default Complexe mul(Complexe c) {
        return new ComplexePolaire(this.module() * c.module(), this.argument() + c.argument());
    }

    default Complexe div(Complexe c) {
        return new ComplexePolaire(this.module() / c.module(), this.argument() - c.argument());
    }
}
