package Main;

import DataTypes.JsonMock;
import Helpers.JsonMockHelper;
import Code.Network;

public class RsaBreaker {

    public static void main(String[] args) {
        // The bad option
        mainDefer(); //this will go away and so this is here to keep it clean for now

        // The good (not working) options
        // create two threads
        // EventQueue.invokeLater( new Runnable() { public void run() { DataThread(); } } );
        // EventQueue.invokeLater( new Runnable() { public void run() { NetworkThread(); } } );
    }

    /*
    * @dev Temporary location of the main code of this project. Most of its work is just sent to other locations so
    * code should be almost entirely reusable.
    */
    public static void mainDefer() {

        // initialize network
        int[] networkNodes = new int[]{7, 5, 5, 3};
        Network net = new Network(networkNodes);

        // initialize data
        JsonMock data = JsonMockHelper.makeTrainingData();
        // > double[] input = new double[]{0.5,0.7,0.5,0.1,0.3,0.5,0.9};
        // > double[] target = new double[]{0.1,0.3,0.5};


        boolean running = true;
        while (running) {

            // listen for stop signal ^C

            // generate new peice of data
            // pass it to network
            // > net.train(input, target, 0.05);

            // log statistics
            // > double[] output = net.calculate(input);
            // > System.out.println(Arrays.toString(output));

            // if this is %10000 == 0
            // print statistics
            // save weights
            // save training data
            // set running to false
        }
    }

    private static void DataThread() {
        while(true) {
            System.out.println("a");
        }
        //1) makes data and logs to file on ^C
    }

    private static void NetworkThread() {
        while(true) {
            System.out.println("b");
        }
        //2) reads data from first thread and adds as training to network
    }
}
