public class Cesar {
    private int key;

    public Cesar(int cle) throws KeyException {

        if (cle > 0 || cle < 25) {
            this.key = cle;
        } else {
            throw new KeyException(cle);
        }
    }

    private char codeChar(char c) {
        int char_final = (int) c + this.key;
        if (char_final >= 65 || char_final <= 90)
            return (char) char_final;
        else {
            return c;
        }
    }

    public char decodeChar(char c) {
        int char_final = (int) c - this.key;
        return (char) char_final;
    }

    // Strings
    public String codeString(String text) {
        String caracter = "";

        for (int i = 0; i < text.length(); i++) {
            caracter = caracter + codeChar((text.charAt(i)));
        }

        return caracter;
    }

    public String decodeString(String s) {
        String caracter = "";

        for (int i = 0; i < s.length(); i++) {
            caracter = caracter + decodeChar(s.charAt(i));
        }

        return caracter;
    }

}
