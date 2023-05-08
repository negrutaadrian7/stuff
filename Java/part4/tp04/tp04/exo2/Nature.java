package tp04.exo2;

import java.awt.*;

public enum Nature {

  BARRE(Color.BLACK),
  ESPACE(Color.WHITE);
  private final Color color;

  private Nature(Color color) {
    this.color = color;
  }

  public void setColor(Graphics g) {
    g.setColor(color);
  }
}
