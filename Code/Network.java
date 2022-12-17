package Code;

import java.util.*;
import Helpers.NetworkHelper;

public class Network {
    private double [][] output; //output [layer][neuron]
    private double [][][] weight; //weights [layer][neuron][previousNeuron], defined by the two neurons it joins
    private double [][] bias; //bias [layer][neuron], doesnt need preivous neuron bc the prev is always the bias
    
    private double [][] errorSignal;
    private double [][] outputDerivative;

    public final int[] networkLayerSize; //size of each layer, first and last must be the same size as input and target
    public final int inputSize; //number of nuerons in input layer
    public final int outputSize; //number of neurons in output layer
    public final int networkSize; //number of nueron layers
    
    //constructor, it does stuff
    public Network(int[] networkLayerSize) {
        //initiates variables as defined above
        this.networkLayerSize = networkLayerSize;
        this.networkSize = networkLayerSize.length;
        this.inputSize = networkLayerSize[0];
        this.outputSize = networkLayerSize[networkSize - 1];

        this.output = new double[networkSize][1]; //this stores output at all nodes for backpropigation
        this.weight = new double[networkSize][1][1];
        this.bias = new double[networkSize][1];
        
        this.errorSignal = new double[networkSize][1];
        this.outputDerivative = new double[networkSize][1];

        for (int i = 0; i < networkSize; i++) {
            this.output[i] = new double [networkLayerSize[i]];
            
            this.errorSignal[i] = new double [networkLayerSize[i]];
            this.outputDerivative[i] = new double [networkLayerSize[i]];
            
            this.bias[i] = NetworkHelper.createRandomArray(networkLayerSize[i], 0.3, 0.7); //these upper and lower bounds can be anything that is reasonable

            // This means weight[0] never gets initialized
            if (i > 0) {
                weight[i] = NetworkHelper.createRandomArray(networkLayerSize[i], networkLayerSize[i-1], 0.3, 0.7);
            }
        }
    }

    public void train(double[] input, double [] target, double eta) {
        if (input.length != inputSize || target.length != outputSize) { return; }

        calculate(input);
        backpropError(target);
        updateWeight(eta);
    }
    
    //calculates the output of the network
    public double[] calculate(double[] input){
        if (input.length != this.inputSize) {return null;} //check for bad input
        
        this.output[0] = input; //sets first layer to the input as no change is applied on first iteration
        for (int layer = 1; layer < networkSize; layer++) {
            for (int neuron = 0; neuron < networkLayerSize[layer]; neuron++){
                double sum = bias[layer][neuron];
                for (int prevNeuron = 0; prevNeuron < networkLayerSize[layer - 1]; prevNeuron++){
                    sum += output[layer - 1][prevNeuron] * weight[layer][neuron][prevNeuron];
                }
                output[layer][neuron] = NetworkHelper.sigmoid(sum);

                // maximization moment
                outputDerivative[layer][neuron] = output[layer][neuron] * (1 - output[layer][neuron]);
            }
        }
        return output[networkSize - 1];
    }
    
    //calculates the error, used to modify the outputDerivative that modifies the weigth(later)
    public void backpropError(double[] target) {
        for (int neuron = 0; neuron < networkLayerSize[networkSize - 1]; neuron++) {
            errorSignal[networkSize - 1][neuron] = (output[outputSize-1][neuron] - target[neuron]) * outputDerivative[networkSize-1][neuron];
        }
        
        for(int layer = networkSize - 2; layer > 0 ; layer--){ //loops for all hiden layers, back to front
            for (int neuron = 0; neuron < networkLayerSize[layer]; neuron++) {
                double sum = 0;
                for (int nextNeuron = 0; nextNeuron < networkLayerSize[layer+1]; nextNeuron++){
                    sum += weight[layer+1][nextNeuron][neuron]; //called from point of veiw of next neuron
                }

                // this is only based on current weight and input
                this.errorSignal[layer][neuron] = sum * outputDerivative[layer][neuron];
            }
        }
    }
    
    //updates the weight of each weight after each iteration
    public void updateWeight(double eta){
        for (int layer = 1; layer < networkSize - 1; layer++){
            for (int neuron = 0; neuron < networkLayerSize[layer]; neuron++){
                double delta = -eta * errorSignal[layer][neuron];
                bias [layer][neuron] += delta;  
                
                for (int prevNeuron = 0; prevNeuron < networkLayerSize[layer - 1]; prevNeuron++){
                    weight[layer][neuron][prevNeuron] += output[layer-1][prevNeuron] * delta;
                }
            }
        }
    }

}
