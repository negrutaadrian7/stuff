package tp05.complexe;

// toate functiile vor fi implementate ulterior in clasele care vor implementa aceasta interfata

//Une méthode par défaut permet de 
//donner une implémentation dans une 
//interface.
//Cette implémentation peut faire appel à 
//des méthodes qui seront définies plus 
//tard dans les classes.


public interface Complexe {
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
