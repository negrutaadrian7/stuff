package tp05.complexe;

// in orice caz toate functiile dintr-o interfata trebuie sa fie implementate

public class ComplexeCartesien implements Complexe {
    private final double reelle;
    private final double imaginaire;

    public ComplexeCartesien(double reelle, double imaginaire) {
        this.reelle = reelle;
        this.imaginaire = imaginaire;
    }

    @Override
    public double reelle() {
        return reelle;
    }

    @Override
    public double imaginaire() {
        return imaginaire;
    }

    @Override
    public double module() {
        return Math.sqrt(reelle * reelle + imaginaire * imaginaire);
    }

    @Override
    public double argument() {
        return Math.atan2(imaginaire, reelle);
    }
}
