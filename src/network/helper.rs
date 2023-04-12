package Helpers;

public interface NetworkHelper {
    
    //creates an array with the same value in all spots
    public static double[] createArray(int size, double value){
        assert (size > 0);

        double[] array = new double[size];
        for(int i = 0; i < size; i++) { array[i] = value; }
        return array;
    }
    
    //makes an array within bounds
    public static double[] createRandomArray(int size, double lowerBound, double upperBound) {
        if(size < 1){
            return null;
        }
        double[] array = new double[size];
        for(int i = 0; i < size; i++){
            array[i] = randomValue(lowerBound,upperBound);
        }
        return array;
    }
    
    //makes a 2d randoma array within the bounds
    public static double[][] createRandomArray(int sizeX, int sizeY, double lowerBound, double upperBound) {
        if(sizeX < 1 || sizeY < 1){
            return null;
        }
        double[][] array = new double[sizeX][sizeY];
        for(int i = 0; i < sizeX; i++){
            array[i] = createRandomArray(sizeY, lowerBound, upperBound);
        }
        return array;
    }
    
    //gets a random value within the bounds
    public static double randomValue(double lowerBound, double upperBound) {
        return Math.random()*(upperBound-lowerBound) + lowerBound;
    }
    
    //gets the index of the highest value of array
    public static int indexOfHighestValue(double[] values){
        int index = 0;
        for(int i = 1; i < values.length; i++){
            if(values[i] > values[index]){
                index = i;
            }
        }
        return index;
    }

    public static double sigmoid (double x) {
        return 1.0d / (1 + Math.exp(-x));
    }

}
