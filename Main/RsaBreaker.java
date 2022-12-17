package Main;

import DataTypes.JsonMock;
import Helpers.DataSetHelper;

public class RsaBreaker {

    public static void main(String[] args) {
        JsonMock trainingData = DataSetHelper.makeTrainingData();

        boolean run = true;
        while (run) {
            trainingData = null;
            run = false;
        }

        System.out.println(trainingData);
    }

    /*
    //the main method, it does the computor things
    public static void main (String [] args){
        int[] networkNodes = new int[]{7,5,5,3};
        Network net = new Network(networkNodes);

        double[] input = new double[]{0.5,0.7,0.5,0.1,0.3,0.5,0.9};
        double[] target = new double[]{0.1,0.3,0.5};

        for (int i = 0; i < 1000; i++)
            net.train(input, target, 0.05);

        double[] output = net.calculate(input);
        System.out.println(Arrays.toString(output));
    }
    */
}
