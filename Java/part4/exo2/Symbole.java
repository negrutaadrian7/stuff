package tp04.exo2;

import java.awt.*;

public class Symbole {

  private final Epaisseur epaisseur;
  private final Nature nature;

  public Symbole(Epaisseur epaisseur, Nature nature) {
    this.epaisseur = epaisseur;
    this.nature = nature;
  }

  public int draw(Graphics g, int x, int y) {
    nature.setColor(g);
    return epaisseur.draw(g, x, y);
  }
}
