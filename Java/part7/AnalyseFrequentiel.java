import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.io.PrintWriter;

public class AnalyseFrequentiel {
    
    
    public String filepath; 
    public AnalyseFrequentiel (String filepath) {
        this.filepath = filepath; 
    }
    

    public int[] nbApparitions (String filepath) throws FileNotFoundException, IOException{
        int [] nbOccurenceChar = new int [27];  
        
        File f = new File (filepath);
        FileReader reader = new FileReader(f);
        BufferedReader bReader = new BufferedReader(reader);
        String buff = "" ;
        int nbTotalCarac = 0;

        buff = bReader.readLine();
        while(buff != null){
            for(int i = 0; i < buff.length() - 1; i++){
                if (buff.charAt(i) != '\n') {
                    nbOccurenceChar[(int) buff.charAt(i) - 65] += 1; 
                    nbTotalCarac ++;
                } 
            }
            buff = bReader.readLine(); 
        }
        bReader.close();
        nbOccurenceChar[nbOccurenceChar.length - 1] = nbTotalCarac;
        return nbOccurenceChar;
    }




    public void analyze (String filString) throws FileNotFoundException, IOException { 
        int[] nbAppar = nbApparitions(filString);
        PrintWriter file_to_write = new PrintWriter("res.csv");
        for (int i = 0; i < 26; i++){
            file_to_write.write(((char) (i + 65)) + ";" + nbAppar[i] + (nbAppar[i] / nbAppar[nbAppar.length - 1]));
        }
        file_to_write.close();
    }
}

