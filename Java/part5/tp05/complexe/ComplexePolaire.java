package tp05.complexe;

public class ComplexePolaire implements Complexe {
    private final double module;
    private final double argument;

    public ComplexePolaire(double module, double argument) {
        this.module = module;
        this.argument = argument;
    }

    @Override
    public double reelle() {
        return module * Math.cos(argument);
    }

    @Override
    public double imaginaire() {
        return module * Math.sin(argument);
    }

    @Override
    public double module() {
        return module;
    }

    @Override
    public double argument() {
        return argument;
    }
}
