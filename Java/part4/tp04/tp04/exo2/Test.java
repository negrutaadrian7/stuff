package tp04.exo2;

import javax.swing.*;
import java.awt.*;

class MessagePanel extends JPanel {

  private final String message;

  public MessagePanel(String message) {
    this.message = message;
  }

  @Override
  protected void paintComponent(Graphics g) {
    int x = 20;
    int y = 20;
    Code39.draw(g, x, y, message);
  }
}

public class Test {

  public static void main(String[] args) {
    JFrame frame = new JFrame();
    frame.setSize(800, 600);
    frame.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
    frame.setLocationRelativeTo(null);
    frame.getContentPane().add(new MessagePanel("2020 ROCKS"));
    frame.setVisible(true);
  }
}
