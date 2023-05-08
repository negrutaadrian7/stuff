package tp04.exo1;

import javax.swing.*;
import java.awt.*;
import java.util.Random;

// Une fenetre graphique constitue de cents disques de rayon
// et de couleur aleatoires 
// Rayon - x > 10 < 100 pixels 
// Centre avec une abscisse entre 0 et 800 
// Ordonnee entre 0 et 600

class CerclePanel extends JPanel {

    private final Cercle[] cercles; // liste avec des cercles comme elements 

    CerclePanel(int nbCercles) { // constructeur du Cercle panel 
        cercles = new Cercle[nbCercles];
        Random random = new Random();
        for (int i = 0; i < nbCercles; i++) {
            int rayon = random.nextInt(90) + 10;
            int x = random.nextInt(800) - rayon;
            int y = random.nextInt(600) - rayon;
            
            // couleur  
            int r = random.nextInt(256);
            int g = random.nextInt(256);
            int b = random.nextInt(256);
            Color couleur = new Color(r, g, b);
            cercles[i] = new Cercle(x, y, rayon, couleur);
        }
    }

    // l'affichage graphique de notre cercle 
    @Override
    protected void paintComponent(Graphics g) {
        for (Cercle cercle : cercles) {
            g.setColor(cercle.getCouleur());
            g.fillOval(cercle.getX(), cercle.getY(), 2 * cercle.getRayon(),
                    2 * cercle.getRayon());
        }
    }
}

public class CercleFrame {

    public static void main(String[] args) {
        JFrame frame = new JFrame();
        frame.setSize(800, 600);
        frame.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
        frame.setLocationRelativeTo(null);
        frame.getContentPane().add(new CerclePanel(100));
        frame.setVisible(true);
    }
}
