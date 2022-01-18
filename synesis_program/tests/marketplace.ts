import * as anchor from '@project-serum/anchor';
import { Program, web3 } from "@project-serum/anchor";
import { SynesisProgram } from '../target/types/synesis_program';
import { loadKeypairFromFile, hexToBytes, bytesToHex } from './utils';
import assert from "assert";

import { keccak256 } from "js-sha3";
import { MerkleTree } from "merkletreejs";

const { Keypair, PublicKey, SystemProgram, Transaction, SYSVAR_RENT_PUBKEY } =
  web3;

const KANON_GLOBAL_SEED = "SYNESIS_KANON";
const KANON_GLOBAL_SEASON_SEED = "SEASON";

const COLLECTION_STATE_PREFIX = "collection_state";
const COLLECTION_AUTHORITY_PREFIX = "collection_authority";
const USER_MINT_RESERVE_STATE_PREFIX = "user_mint_reserve_state";
const ADMIN_TREASURY_ACCOUNT_PREFIX = "admin_treasury_account";

describe('marketplace test for synesis_program', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SynesisProgram as Program<SynesisProgram>;

  console.log("programID: ", program.programId.toString());

  const globalStateAccountKey = loadKeypairFromFile('tests/keys/globalAcct-kngBcrFsdmLQd3JRfdnDGcXaMKsMi8QJtoedEzXCBLa.json');
  const globalStateAccountWrongKey = loadKeypairFromFile('tests/keys/globalWrongAcct-snSh1MWA6iLfUuWoQQntW4QAmCVVQjMHehehJGPWQ5C.json');
  const adminWalletKey = loadKeypairFromFile('tests/keys/admin-JDS6H6dAKEgpwKZ8bXwj9HTGoNWjxrk2XrnovgLaBAcA.json');
  const wrongAdminWalletKey = loadKeypairFromFile('tests/keys/wrongAdmin-7Go3Ak4zrbuvmCxUpVQJqawysqHNCYq32Pn9hxMeeAeY.json');

  const mintUser = Keypair.generate();

  let adminTreasuryAccountPubkey;
  let adminTreasuryAccountBump;

  let collectionStateAccountPubkey;
  let collectionStateAccountBump;

  let collectionAuthorityAccountPubkey;
  let collectionAuthorityAccountBump;

  let userMintReserveAccountPubkey;
  let userMintReserveAccountBump;


  let seasonNumber = 1;
  let artAmount = 50;
  const merkleSeed = KANON_GLOBAL_SEED + "&" + KANON_GLOBAL_SEASON_SEED + "&" + seasonNumber;

  var collectionMerkleTreeFromDB = [];
  var collectionMerkleRootHexString = "";

  var whitelistMerkleTreeFromDB = [];
  var whitelistMerkleRootHexString = "";

  var nftMerkleTreeFromDB = [];
  var nftMerkleRootHexString = "";

  var airdropNftMerkleTreeFromDB = [];
  var airdropNftMerkleRootHexString = "";


  before(async () => {


    ////////////////////////////////////////////////////////////////
    /// Merkle tree generation process for collection
    ////////////////////////////////////////////////////////////////

    // Hash collection from DB( dont transfer the field 'word' actually in game server, it's top secret)
    for (let i = 0; i < artAmount; i++) {
      const word = "hello" + i.toString();
      collectionMerkleTreeFromDB.push({
        word
      })
    }

    // generate merkle tree
    const collectionLeaves = collectionMerkleTreeFromDB.map(x => keccak256(merkleSeed + x.word));
    const collectionMerkleTree = new MerkleTree(collectionLeaves, keccak256);
    collectionMerkleRootHexString = collectionMerkleTree.getRoot().toString("hex");

    // Update hash and proof
    for (let i = 0; i < artAmount; i++) {
      collectionMerkleTreeFromDB[i].hash = keccak256(merkleSeed + collectionMerkleTreeFromDB[i].word);
      collectionMerkleTreeFromDB[i].proof = collectionMerkleTree.getProof(collectionMerkleTreeFromDB[i].hash);
    }

    // store array to db (omission)

    ////////////////////////////////


    ////////////////////////////////////////////////////////////////
    /// Merkle tree generation process for whitelist
    ////////////////////////////////////////////////////////////////

    // wallet address array from DB
    whitelistMerkleTreeFromDB.push({ wallet_address: "2DHrXdiGFfyjYNcykTaYViHdFT5PC6g3d5kDSpKgQHGx" });
    whitelistMerkleTreeFromDB.push({ wallet_address: "AdU6CTb2JQJA32rTRgUS7ssH4gavR9RtoSDFXaCs8nFt" });
    whitelistMerkleTreeFromDB.push({ wallet_address: "4K1k1DRzohg8hXRvUpHHruEMtJPrzNf8NGx1MbA4nHJd" });
    whitelistMerkleTreeFromDB.push({ wallet_address: "7nG2BHVHwCJ6qgXVvPd6kDJm2UtXabRwb4GXgsLfRfDa" });
    whitelistMerkleTreeFromDB.push({ wallet_address: "7RT9sXJyCrEeqVtquKAeBCHF4fyV1YHb5EcicoFULzPp" });

    // generate merkle tree
    const whitelistLeaves = whitelistMerkleTreeFromDB.map(x => keccak256(merkleSeed + x.wallet_address));
    const whitelistMerkleTree = new MerkleTree(whitelistLeaves, keccak256);
    whitelistMerkleRootHexString = whitelistMerkleTree.getRoot().toString("hex");

    // Update hash and proof
    for (let i = 0; i < whitelistMerkleTreeFromDB.length; i++) {
      whitelistMerkleTreeFromDB[i].hash = keccak256(merkleSeed + whitelistMerkleTreeFromDB[i].wallet_address);
      whitelistMerkleTreeFromDB[i].proof = whitelistMerkleTree.getProof(whitelistMerkleTreeFromDB[i].hash);
    }

    // store array to db (omission)

    ////////////////////////////////

    ////////////////////////////////////////////////////////////////
    /// Merkle tree generation process for NFT (temp code)
    ////////////////////////////////////////////////////////////////

    // nft mint address array from DB
    for (let i = 0; i < artAmount; i++) {
      nftMerkleTreeFromDB.push({
        nft_mint_address: "qBUzzJ8W71wA8J7UWgZ16otGW4Rtgsx9Epmk7MsoR9b"
      })
    }

    // generate merkle tree
    const nftLeaves = nftMerkleTreeFromDB.map(x => keccak256(merkleSeed + x.nft_mint_address));
    const nftMerkleTree = new MerkleTree(nftLeaves, keccak256);
    nftMerkleRootHexString = nftMerkleTree.getRoot().toString("hex");

    // Update hash and proof
    for (let i = 0; i < artAmount; i++) {
      nftMerkleTreeFromDB[i].hash = keccak256(merkleSeed + nftMerkleTreeFromDB[i].nft_mint_address);
      nftMerkleTreeFromDB[i].proof = nftMerkleTree.getProof(nftMerkleTreeFromDB[i].hash);
    }

    // store array to db (omission)

    ////////////////////////////////


    ////////////////////////////////////////////////////////////////
    /// Merkle tree generation process for reserved NFT (temp code)
    ////////////////////////////////////////////////////////////////

    // nft mint address array from DB
    for (let i = 0; i < artAmount; i++) {
      airdropNftMerkleTreeFromDB.push({
        airdrop_nft_mint_address: "qBUzzJ8W71wA8J7UWgZ16otGW4Rtgsx9Epmk7MsoR9b",
        airdrop_user_wallet_address: "7RT9sXJyCrEeqVtquKAeBCHF4fyV1YHb5EcicoFULzPp",
      })
    }

    // generate merkle tree
    const airdropNftLeaves = airdropNftMerkleTreeFromDB.map(x => keccak256(merkleSeed + x.airdrop_user_wallet_address + x.airdrop_nft_mint_address));
    const airdropNftMerkleTree = new MerkleTree(airdropNftLeaves, keccak256);
    airdropNftMerkleRootHexString = airdropNftMerkleTree.getRoot().toString("hex");

    // Update hash and proof
    for (let i = 0; i < artAmount; i++) {
      airdropNftMerkleTreeFromDB[i].hash = keccak256(merkleSeed + airdropNftMerkleTreeFromDB[i].airdrop_user_wallet_address + airdropNftMerkleTreeFromDB[i].airdrop_nft_mint_address);
      airdropNftMerkleTreeFromDB[i].proof = airdropNftMerkleTree.getProof(airdropNftMerkleTreeFromDB[i].hash);
    }

    // store array to db (omission)

    ////////////////////////////////



    ////////////////////////////////
    /// initialize accounts
    ////////////////////////////////

    // Airdrop 1 sol to test wallet
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        adminWalletKey.publicKey,
        1 * web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    [adminTreasuryAccountPubkey, adminTreasuryAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          globalStateAccountKey.publicKey.toBuffer(),
          Buffer.from(anchor.utils.bytes.utf8.encode(ADMIN_TREASURY_ACCOUNT_PREFIX)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEED)),
        ],
        program.programId
      );

    const tx = await program.rpc.initialize(
      {
        treasuryAccountBump: adminTreasuryAccountBump
      },
      {
        accounts: {
          globalState: globalStateAccountKey.publicKey,
          adminTreasuryAccount: adminTreasuryAccountPubkey,
          admin: adminWalletKey.publicKey,
        },
        instructions: [
          await program.account.globalAccount.createInstruction(globalStateAccountKey),
        ],
        signers: [adminWalletKey, globalStateAccountKey],
      }
    );


    ////////////////////////////////
    /// Create Season
    ////////////////////////////////

    // Airdrop 1 sol to test wallet
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        adminWalletKey.publicKey,
        1 * web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    // schedules (hours)
    let countdownDuration = new anchor.BN(7 * 24);
    let promosMintDuration = new anchor.BN(2 * 24);
    let premintDuration = new anchor.BN(1 * 24);
    let premintWave1Duration = new anchor.BN(10);
    let premintBlockingDuration = new anchor.BN(7 * 24);
    let mintWave3Duration = new anchor.BN(1 * 24);

    // limitations
    let promosReservedNftsAmount = 3;
    let premintWave1Amount = 5;
    let premintWave2Amount = 7;
    let whitelistUserMaxPremintQuantity = 1;
    let maxFreelyMintQuantity = 2;
    ////////////////////////////////

    [collectionStateAccountPubkey, collectionStateAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          globalStateAccountKey.publicKey.toBuffer(),
          Buffer.from(anchor.utils.bytes.utf8.encode(COLLECTION_STATE_PREFIX)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEED)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEASON_SEED)),
          new Uint8Array([seasonNumber]),
        ],
        program.programId
      );

    [collectionAuthorityAccountPubkey, collectionAuthorityAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          collectionStateAccountPubkey.toBuffer(),
          Buffer.from(anchor.utils.bytes.utf8.encode(COLLECTION_AUTHORITY_PREFIX)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEED)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEASON_SEED)),
          new Uint8Array([seasonNumber]),
        ],
        program.programId
      );

    await program.rpc.createSeason(
      {
        collectionBump: collectionStateAccountBump,
        authorityBump: collectionAuthorityAccountBump,
        seasonNumber: seasonNumber,
        collectionMerkleRoot: [...Buffer.from(hexToBytes(collectionMerkleRootHexString))],
        whitelistMerkleRoot: [...Buffer.from(hexToBytes(whitelistMerkleRootHexString))],
        // schedules
        countdownDuration,
        promosMintDuration,
        premintDuration,
        premintWave1Duration,
        premintBlockingDuration,
        mintWave3Duration,
        // limitations
        artAmount,
        promosReservedNftsAmount,
        premintWave1Amount,
        premintWave2Amount,
        whitelistUserMaxPremintQuantity,
        maxFreelyMintQuantity,
      },
      {
        accounts: {
          globalState: globalStateAccountKey.publicKey,
          collectionState: collectionStateAccountPubkey,
          collectionAuthority: collectionAuthorityAccountPubkey,
          admin: adminWalletKey.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [adminWalletKey],
      }
    );


    ////////////////////////////////
    /// Open Season
    ////////////////////////////////

    // Airdrop 1 sol to test wallet
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        adminWalletKey.publicKey,
        1 * web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    [collectionStateAccountPubkey, collectionStateAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          globalStateAccountKey.publicKey.toBuffer(),
          Buffer.from(anchor.utils.bytes.utf8.encode(COLLECTION_STATE_PREFIX)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEED)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEASON_SEED)),
          new Uint8Array([seasonNumber]),
        ],
        program.programId
      );

    // correct test
    await program.rpc.openSeason(
      {
        seasonNumber: seasonNumber,
        reservedNftMerkleRoot: [...Buffer.from(hexToBytes(airdropNftMerkleRootHexString))],
        nftMerkleRoot: [...Buffer.from(hexToBytes(nftMerkleRootHexString))],
      },
      {
        accounts: {
          globalState: globalStateAccountKey.publicKey,
          collectionState: collectionStateAccountPubkey,
          admin: adminWalletKey.publicKey,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [adminWalletKey],
      }
    );

    // Airdrop 1 sol to test wallet
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        mintUser.publicKey,
        10 * web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );
  });



  ////////////////////////////////
  /// Test marketplace
  ////////////////////////////////

  it('Initialize user reserved account', async () => {
    [userMintReserveAccountPubkey, userMintReserveAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          collectionStateAccountPubkey.toBuffer(),
          Buffer.from(anchor.utils.bytes.utf8.encode(USER_MINT_RESERVE_STATE_PREFIX)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEED)),
          Buffer.from(anchor.utils.bytes.utf8.encode(KANON_GLOBAL_SEASON_SEED)),
          new Uint8Array([seasonNumber]),
          mintUser.publicKey.toBuffer(),
        ],
        program.programId
      );

    // correct test
    await program.rpc.initializeUserReservedAccount(
      {
        seasonNumber: seasonNumber,
        userMintReserveBump: userMintReserveAccountBump,
      },
      {
        accounts: {
          globalState: globalStateAccountKey.publicKey,
          collectionState: collectionStateAccountPubkey,
          userMintReserveState: userMintReserveAccountPubkey,
          user: mintUser.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [mintUser],
      }
    );

    const userMintReserveAccount = await program.account.userMintReserveAccount.fetch(userMintReserveAccountPubkey);
    assert.ok(userMintReserveAccountBump === userMintReserveAccount.bump, "wrong userMintReserveAccountBump");
    assert.ok(userMintReserveAccount.freelyMintedAmount === 0, "Wrong art amount");
  });
});
