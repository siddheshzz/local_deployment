contract Voting {
    using ECDSA for bytes32;

    address public signer;
    uint32 public Value;
    address[] public voters;
    mapping(bytes32 => uint8) public Praposals;
    constructor(address[] memory v){
        Value = 24;
        for (uint i = 0; i < v.length; i++) {
            voters.push(v[i]);
        }
    }

    function getVoters() public view returns(address[] memory){
        return voters;
    }

    function getPraposals() public view returns(uint8[] memory){
        return Praposals;
    }
    function _verify(address voter) internal view returns(bool){
        for(uint i=0;i<voters.length;i++){
            if (voter==voters[i]){
                return true;
            }
        }
        return false;
    }
    function splitSignature(bytes memory sig)
        internal
        pure
        returns (uint8, bytes32, bytes32)
    {
        require(sig.length == 65);

        bytes32 r;
        bytes32 s;
        uint8 v;

        assembly {
            // first 32 bytes, after the length prefix
            r := mload(add(sig, 32))
            // second 32 bytes
            s := mload(add(sig, 64))
            // final byte (first byte of the next 32 bytes)
            v := byte(0, mload(add(sig, 96)))
        }

        return (v, r, s);
    }

    function recoverSigner(bytes32 message, bytes memory sig)
        internal
        pure
        returns (address)
    {
        uint8 v;
        bytes32 r;
        bytes32 s;

        (v, r, s) = splitSignature(sig);

        return ecrecover(message, v, r, s);
    }

    function check(string memory message1, bytes memory sig) public {
        bytes32 message = keccak256(abi.encodePacked(message1));
        address signer = recoverSigner(message, sig);

        
        bool isVoter = _verify(signer);
        if(isVoter){
            Praposals[message]++; 
        }
        
        if(Praposals[message]>=2){
            _updateValue(2);
        }
    }
    function _updateValue(uint32 num) internal{
        Value = num;

    }

}