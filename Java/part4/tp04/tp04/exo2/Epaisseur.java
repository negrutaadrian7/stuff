package tp04.exo2;

import java.awt.*;

public enum Epaisseur {

  ETROIT(1),
  LARGE(2);
  /* largeur par défaut */
  private static final int LARGEUR = 3;
  /* hauteur par défaut */
  private static final int HAUTEUR = 200;
  /* coefficient multiplicatif de la largeur */
  private final int epaisseur;

  private Epaisseur(int epaisseur) {
    this.epaisseur = epaisseur;
  }

  public int draw(Graphics g, int x, int y) {
    g.fillRect(x, y, epaisseur * LARGEUR, HAUTEUR);
    /* renvoie la lageur de la bande dessinée pour se repositionner */
    return epaisseur * LARGEUR;
  }
}
