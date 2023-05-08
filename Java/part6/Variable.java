import java.util.ArrayList;

// Array and arraylist, once we have an array, the lenght cannot be changed otherwise the arraylist

public class Variable implements Expression {

    protected String id;
    protected double value;

    // the difference between araylist and simply a list, is that we can extend the
    // arraylist what we can't do with a list
    protected static ArrayList<String> usedID = new ArrayList<>(); // the table where we hold all the id's

    public Variable(String identificateur, double nombre) throws IdException {
        if (usedID.indexOf(identificateur) != -1)
            throw new IdException(identificateur); // exception in constructor
        usedID.add(identificateur);
        this.id = identificateur;
        this.value = nombre;
    }

    public void setV(double value) {
        this.value = value;
    }

    public double getV() {
        return this.value;
    }

    @Override
    public double evalue() {
        return this.value;
    }

    @Override
    public String toString() {
        return this.id;
    }
}

// noeuds internes - des operations
// les feuilles - constantes et des variables.

// O(n^2)
