package DataTypes;

import java.io.File;
import java.util.ArrayList;

public class JsonMock {

    String fileName;
    ArrayList<KeySet> data;

    public JsonMock() {
        fileName = "jankyJson.cringe";
        this.data = new ArrayList<KeySet>();
    }

    public boolean updateFile() {
        return true;
    }

    public boolean fillData(File fileName) {
        return true;
    }

    public boolean add(KeySet added) {
        return data.add(added);
    }

}
