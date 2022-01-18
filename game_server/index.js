//const web3 = require("@solana/web3.js");
// const {
//     getPhantomWallet,
//     getSlopeWallet,
//     getSolflareWallet,
// } = require("@solana/wallet-adapter-wallets");
// const { SystemProgram } = anchor.web3;

const restify = require("restify");
const anchor = require("@project-serum/anchor");
const { MerkleTree } = require("merkletreejs");
// const SHA256 = require("crypto-js/sha256");
const keccak256 = require('js-sha3').keccak256;
const { bytes } = require("@project-serum/anchor/dist/cjs/utils");
// const { Keccak } = require("keccak");

const PORT = 5000
const app = restify.createServer();
app.get('/', (req, res) => res.send({ hello: "world" }));
app.listen(PORT);
console.log("PORT- 5000");


function hexToBytes(hex) {
    for (var bytes = [], c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
}
// const ab = keccak256.arrayBuffer("hello!");
// const st = keccak256("hello!");

// console.log("compare:\n", Buffer.from(hexToBytes(st)), "\n", ab);

app.get('/get-wallet-info/:walletAddr', async (req, res) => {
    const walletAddr = new anchor.web3.PublicKey(req.params.walletAddr);
    var connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl('devnet'), 'confirmed');


    // var wallet = web3.keypair.generate();
    let account = await connection.getAccountInfo(walletAddr);

    console.log(account.data.toString());


    var collection = [];

    const collection_number = "1";
    const synesis_collection_key = "Season";
    const nft_amount = 10;
    const nonce = synesis_collection_key + "&" + collection_number;
    var merkle_hash = "";

    ////////////////////////////////
    // Hash collection
    ////////////////////////////////
    for (let i = 0; i < nft_amount; i++) {
        const word = "hello" + i.toString();
        collection.push({
            word,
        })
    }

    ////////////////////NFT MINT TEST////////////
    // collection.push({
    //     word: "www",
    //     hash: "qBUzzJ8W71wA8J7UWgZ16otGW4Rtgsx9Epmk7MsoR9b",
    //     proof: ""
    // });
    ////////////////////////////////



    ////////////////////////////////
    // Merkle Tree
    ////////////////////////////////
    function keccak(data) {
        return keccak256.arrayBuffer(data);
    }

    const leaves = collection.map(x => keccak256(nonce + x.word));
    const tree = new MerkleTree(leaves, keccak256);
    merkle_hash = tree.getRoot().toString("hex");


    ////////////////////////////////
    // Update hash and proof
    ////////////////////////////////
    for (let i = 0; i < nft_amount; i++) {
        collection[i].hash = keccak256(nonce + collection[i].word);
        collection[i].proof = tree.getProof(collection[i].hash);
    }
    console.log(collection);

    ////////////////////////////////
    // test verification
    ////////////////////////////////
    const badHash = "c6c0c6bca3571efc11caa4bc9537c355568d4175350739ca5ec625aac808482g";
    const rightHash = collection[2].hash;
    const proof = collection[2].proof;
    const verifyTree = new MerkleTree([], keccak256);
    console.log("proof", proof);
    console.log("right hash verification: ", verifyTree.verify(proof, rightHash, merkle_hash));
    console.log("bad hash verification: ", verifyTree.verify(proof, badHash, merkle_hash));

    res.send("balance is " + account.lamports);
})