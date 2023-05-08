package tp04.exo1;
// Une fenetre graphique constitue de cents disques de rayon
// et de couleur aleatoires 
// Rayon - x > 10 < 100 pixels 
// Centre avec une abscisse entre 0 et 800 
// Ordonnee entre 0 et 600 


import java.awt.*;

public class Cercle {

  private final int x;
  private final int y;
  private final int rayon;
  private final Color couleur;

  public Cercle(int x, int y, int rayon, Color couleur) {
    this.x = x;
    this.y = y;
    this.rayon = rayon;
    this.couleur = couleur;
  }

  public int getRayon() {
    return rayon;
  }

  public Color getCouleur() {
    return couleur;
  }

  public int getX() {
    return x;
  }

  public int getY() {
    return y;
  }
}
