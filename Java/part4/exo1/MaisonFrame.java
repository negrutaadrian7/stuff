package tp04.exo1;

import javax.swing.*;
import java.awt.*;

class MaisonPanel extends JPanel {

  @Override
  protected void paintComponent(Graphics g) {
    int width = 100;
    int height = 100;
    int x1 = 50;
    int y1 = 100;
    int x2 = x1 + width;
    int y2 = y1;
    int x = (x1 + x2) / 2;
    int y = y1 - 50;
    g.drawRect(x1, y1, width, height);
    g.drawLine(x1, y1, x, y);
    g.drawLine(x2, y2, x, y);
  }
}

public class MaisonFrame {

  public static void main(String[] args) {
    JFrame frame = new JFrame();
    frame.setSize(800, 600);
    frame.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
    frame.setLocationRelativeTo(null);
    frame.getContentPane().add(new MaisonPanel());
    frame.setVisible(true);
  }
}
