package DataTypes;

import java.math.BigInteger;

public class KeySet {

    long publicKey;
    long privateKey;
    BigInteger combinedKey;

    public KeySet(long publicKey, long privateKey) {
        this.publicKey = publicKey;
        this.privateKey = privateKey;
        this.combinedKey = BigInteger.valueOf( publicKey * privateKey );
    }
}
