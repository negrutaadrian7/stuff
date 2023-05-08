package tp04.exo1;

import javax.swing.*;
import java.awt.*;

class MyPanel extends JPanel {
// panneau dans lequel on peut dessiner

  protected void paintComponent(Graphics g) {
    // Toute la logique de dessin est ici
    // g représente la surface de dessin du panneau
    g.drawString("Hello World!", 20, 20);
  }
}

public class HelloWorldFrame {
  public static void main(String[] args) {
    System.out.println("Hello world");
    JFrame frame = new JFrame(); // creation d'une fenetre graphique
    frame.setSize(800, 600); // taille de la fenetre
    frame.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
    // comportement lors de la fermeture
    frame.setLocationRelativeTo(null); // on centre la fenetre
    frame.getContentPane().add(new MyPanel()); // on ajoute le panneau
    frame.setVisible(true); // on rend la fenetre visible
  }
}