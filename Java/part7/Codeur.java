import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.io.PrintWriter;
import java.text.Normalizer;




public class Codeur {
    
    
    
    public int cleValeur (int key) throws KeyException { 
        int final_key;
        if (key < 0 || key > 25 ) {
            throw new KeyException(key);
        }
        final_key = key;
        return final_key;
    }

    public String codeOuDecode (String val) throws InvalidArg{
        if (val != "code" || val != "decode"){
            throw new InvalidArg(val);
        }
        return val;
    }
    
    public void Ecriture (String filePath, String filePath2, String codeOuDecode, int key) throws FileNotFoundException, KeyException, IOException{ 
        File f = new File(filePath);
        FileReader reader = new FileReader(f);
        BufferedReader breader = new BufferedReader(reader);

        PrintWriter file_to_write = new PrintWriter(filePath2);
        Cesar c = new Cesar(key);
        
        if (codeOuDecode == "code") {
            while (breader.readLine() != null ){
                file_to_write.write(c.codeString(breader.readLine()));
            }
        }

        else {
            while (breader.readLine() != null ){
                file_to_write.write(c.decodeString(breader.readLine()));
            }
        }
        
        breader.close();
        file_to_write.close();
    }

    public static String normalize (String str) {
        return Normalizer.normalize(str, Normalizer.Form.NFD).replaceAll("[\u0300-\u036F]", "").toUpperCase();
        
    }
    
    public static void main(String[] args) throws SysLength, KeyException, InvalidArg, FileNotFoundException, IOException {
        if (args.length < 5) {
            throw new SysLength(args.length);
        }
        Codeur c = new Codeur();
        
        
        int verifiedKey = c.cleValeur(Integer.parseInt(args[1]));
        
        // le mot code ou decode 
        String verifiedCodeOuDecode = c.codeOuDecode(args[2]);
        
        String filePath1 = args[3]; // fichier source
        String filePath2 = args[4]; // fichier destination 
        
        c.Ecriture(filePath1, filePath2, verifiedCodeOuDecode, verifiedKey);
        
    
    }
}


// up casting - inutille 
// ouverture d'un fichier 
// late binding 