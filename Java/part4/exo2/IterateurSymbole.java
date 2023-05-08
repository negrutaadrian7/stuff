package tp04.exo2;

public class IterateurSymbole {

  private final String code;
  /* position courante dans la chaîne */
  private int position = 0;

  public IterateurSymbole(String code) {
    this.code = code;
  }

  public boolean hasNext() {
    return position < code.length();
  }

  public Symbole next() {
    /* après voir déterminé les éléments nécessaires à la création du
     * symbole suivant, on avance dans la chaîne puis on crée et renvoie
     * le symbole
     */
    Epaisseur epaisseur = determineEpaisseur();
    Nature nature = determineNature();
    position++;
    return new Symbole(epaisseur, nature);
  }

  private Epaisseur determineEpaisseur() {
    switch (code.charAt(position)) {
      case '0':
        return Epaisseur.ETROIT;
      case '1':
        return Epaisseur.LARGE;
      default:
        throw new UnsupportedOperationException("Caractère illégale rencontré!");
    }
  }

  private Nature determineNature() {
    /* on alterne barres et espaces en commençant par une barre. */
    if (position % 2 == 0) {
      return Nature.BARRE;
    } else {
      return Nature.ESPACE;
    }
  }
}
