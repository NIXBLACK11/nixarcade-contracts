import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GameWager } from "../target/types/game_wager";
import { assert } from "chai";
import fs from "fs";

describe("game_wager", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.GameWager as Program<GameWager>;

  let firstPlayer: anchor.web3.Keypair;
  let secondPlayer: anchor.web3.Keypair;
  let thirdPlayer: anchor.web3.Keypair;
  let fourthPlayer: anchor.web3.Keypair;
  let gameCode: number;
  let gameType: number;
  let wager: anchor.BN;
  let numPlayers: number;
  let gameAccount: anchor.web3.PublicKey;

  before(async () => {
    // Generate fresh keypairs for each test
    firstPlayer = anchor.web3.Keypair.generate();
    secondPlayer = anchor.web3.Keypair.generate();
    thirdPlayer = anchor.web3.Keypair.generate();
    fourthPlayer = anchor.web3.Keypair.generate();

    // Airdrop SOL to all players
    for (const player of [firstPlayer, secondPlayer, thirdPlayer, fourthPlayer]) {
      const sig = await program.provider.connection.requestAirdrop(
        player.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await program.provider.connection.confirmTransaction(sig);
    }

    // Use a simple u8 for gameCode (e.g. 42)
    gameCode = 42; // u8
    gameType = 0;  // u8, 0 = Ludo (max 4), 1 = TicTacToe (max 2), 2 = Snake and Ladder (max 4)
    wager = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
    numPlayers = 4;

    // Derive PDA for game account using [b"game", gameCode, gameType]
    [gameAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("game"),
        Buffer.from([gameCode]), // gameCode first
        Buffer.from([gameType])  // gameType second
      ],
      program.programId
    );
  });

  it("should initialize a game with correct data", async () => {
    const initialBalance = await program.provider.connection.getBalance(firstPlayer.publicKey);

    await program.methods
      .initializeGame(gameCode, gameType, wager, numPlayers) // gameCode first
      .accounts({
        firstPlayer: firstPlayer.publicKey,
        gameAccount: gameAccount,
      })
      .signers([firstPlayer])
      .rpc();

    const finalBalance = await program.provider.connection.getBalance(firstPlayer.publicKey);
    const gameData = await program.account.game.fetch(gameAccount);

    assert(
      initialBalance - finalBalance >= wager.toNumber(),
      "Player's balance should decrease by at least the wager amount"
    );
    assert.equal(gameData.players[0].toBase58(), firstPlayer.publicKey.toBase58(), "First player should be set correctly");
    assert.equal(gameData.playerIndex, 1, "Player index should be 0 after init");
    assert.equal(gameData.numPlayers, numPlayers, "Number of players should match");
    assert.equal(gameData.wager.toString(), wager.toString(), "Wager amount should match");
    assert.equal(gameData.gameType, gameType, "Game type should match");
    assert.equal(gameData.gameCode, gameCode, "Game code should match");
  });

  it("should not allow a player to join twice", async () => {
    await program.methods
      .joinGame(gameCode, gameType) // gameCode first
      .accounts({
        player: secondPlayer.publicKey,
        gameAccount: gameAccount,
      })
      .signers([secondPlayer])
      .rpc();

    try {
      await program.methods
        .joinGame(gameCode, gameType) // gameCode first
        .accounts({
          player: secondPlayer.publicKey,
          gameAccount: gameAccount,
        })
        .signers([secondPlayer])
        .rpc();
      assert.fail("Should have thrown an error - player already joined");
    } catch (err) {
      assert.include(err.toString(), "Player has already joined this game");
    }
  });

  it("should allow players to join until the game is full", async () => {
    await program.methods
      .joinGame(gameCode, gameType) // gameCode first
      .accounts({
        player: thirdPlayer.publicKey,
        gameAccount: gameAccount,
      })
      .signers([thirdPlayer])
      .rpc();

    await program.methods
      .joinGame(gameCode, gameType) // gameCode first
      .accounts({
        player: fourthPlayer.publicKey,
        gameAccount: gameAccount,
      })
      .signers([fourthPlayer])
      .rpc();

    const gameData = await program.account.game.fetch(gameAccount);
    assert.equal(gameData.playerIndex, 4, "Player index should be 4 after all joined");
    assert.equal(gameData.players[1].toBase58(), secondPlayer.publicKey.toBase58(), "Second player should be correct");
    assert.equal(gameData.players[2].toBase58(), thirdPlayer.publicKey.toBase58(), "Third player should be correct");
    assert.equal(gameData.players[3].toBase58(), fourthPlayer.publicKey.toBase58(), "Fourth player should be correct");
  });


  it("should not allow more than max players to join", async () => {
    const fifthPlayer = anchor.web3.Keypair.generate();
    const sig = await program.provider.connection.requestAirdrop(
      fifthPlayer.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig);

    try {
      await program.methods.joinGame(gameCode, gameType).accounts({
        player: fifthPlayer.publicKey,
        gameAccount: gameAccount,
      }).signers([fifthPlayer]).rpc();
      assert.fail("Should have thrown an error - game already full");
    } catch (err) {
      assert.include(err.toString(), "Game is already full");
    }
  });

  it("should enforce correct min/max players for different game types", async () => {
    // TicTacToe (gameType = 1) only allows 2 players
    const tttGameType = 1;
    const tttNumPlayers = 2;
    const tttGameCode = 77;
    const [tttGameAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("game"),
        Buffer.from([tttGameCode]), // gameCode first
        Buffer.from([tttGameType])  // gameType second
      ],
      program.programId
    );

    await program.methods
      .initializeGame(tttGameCode, tttGameType, wager, tttNumPlayers) // gameCode first
      .accounts({
        firstPlayer: firstPlayer.publicKey,
        gameAccount: tttGameAccount,
      })
      .signers([firstPlayer])
      .rpc();

    await program.methods
      .joinGame(tttGameCode, tttGameType) // gameCode first
      .accounts({
        player: secondPlayer.publicKey,
        gameAccount: tttGameAccount,
      })
      .signers([secondPlayer])
      .rpc();

    // Third player tries to join TicTacToe (should fail)
    try {
      await program.methods
        .joinGame(tttGameCode, tttGameType) // gameCode first
        .accounts({
          player: thirdPlayer.publicKey,
          gameAccount: tttGameAccount,
        })
        .signers([thirdPlayer])
        .rpc();
      assert.fail("Should have thrown an error - TicTacToe game should be full");
    } catch (err) {
      assert.include(err.toString(), "Game is already full");
    }

    const tttGameData = await program.account.game.fetch(tttGameAccount);
    assert.equal(tttGameData.playerIndex, 2, "TicTacToe should have exactly 2 players");
  });

  it("should not allow a non-authority (player) to end the game", async () => {
    try {
      await program.methods
        .endGame(gameCode, gameType, secondPlayer.publicKey)
        .accounts({
          signer: secondPlayer.publicKey,
          winner: secondPlayer.publicKey,
          gameAccount: gameAccount,
          firstPlayer: firstPlayer.publicKey,
        })
        .signers([secondPlayer])
        .rpc();
      assert.fail("Should have thrown an error - non-authority tried to end the game");
    } catch (err) {
      assert.include(err.toString(), "NotAuthorized");
    }
  });

  it("should not allow a random pubkey (not a player, not an authority) to end the game", async () => {
    const randomKeypair = anchor.web3.Keypair.generate();
    const sig = await program.provider.connection.requestAirdrop(
      randomKeypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig);

    try {
      await program.methods
        .endGame(gameCode, gameType, secondPlayer.publicKey)
        .accounts({
          signer: randomKeypair.publicKey,
          winner: secondPlayer.publicKey,
          gameAccount: gameAccount,
          firstPlayer: firstPlayer.publicKey,
        })
        .signers([randomKeypair])
        .rpc();
      assert.fail("Should have thrown an error - random pubkey tried to end the game");
    } catch (err) {
      assert.include(err.toString(), "NotAuthorized");
    }
  });
  
  it("should not allow ending the game with an invalid winner", async () => {
    const secret = JSON.parse(fs.readFileSync("authority1.json", "utf-8"));
    const authority1 = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(secret));
    const sig = await program.provider.connection.requestAirdrop(
      authority1.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig);

    const notAPlayer = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .endGame(gameCode, gameType, notAPlayer.publicKey)
        .accounts({
          signer: authority1.publicKey,
          winner: notAPlayer.publicKey,
          gameAccount: gameAccount,
          firstPlayer: firstPlayer.publicKey,
        })
        .signers([authority1])
        .rpc();
      assert.fail("Should have thrown an error - invalid winner");
    } catch (err) {
      assert.include(err.toString(), "InvalidWinner");
    }
  });

  it("should end the game and pay out the winner, closing the game account", async () => {
    const winner = secondPlayer.publicKey;

    const gameDataBefore = await program.account.game.fetch(gameAccount);
    const wager = gameDataBefore.wager;
    const numPlayers = gameDataBefore.numPlayers;

    const winnerBalanceBefore = await program.provider.connection.getBalance(winner);
    const firstPlayerBalanceBefore = await program.provider.connection.getBalance(firstPlayer.publicKey);

    const secret = JSON.parse(fs.readFileSync("authority1.json", "utf-8"));
    const authority1 = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(secret));
    const authorityBalanceBefore = await program.provider.connection.getBalance(authority1.publicKey);

    const sig = await program.provider.connection.requestAirdrop(
      authority1.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig);

    await program.methods
      .endGame(gameCode, gameType, winner)
      .accounts({
        signer: authority1.publicKey,
        winner: winner,
        gameAccount: gameAccount,
        firstPlayer: firstPlayer.publicKey,
      })
      .signers([authority1])
      .rpc();

    const winnerBalanceAfter = await program.provider.connection.getBalance(winner);
    const firstPlayerBalanceAfter = await program.provider.connection.getBalance(firstPlayer.publicKey);
    const authorityBalanceAfter = await program.provider.connection.getBalance(authority1.publicKey);

    const totalPot = wager.toNumber() * numPlayers;
    const authorityFee = Math.floor(totalPot * 0.05);
    const expectedWinnerPayout = totalPot - authorityFee;

    const payout = winnerBalanceAfter - winnerBalanceBefore;
    const authorityPayout = authorityBalanceAfter - authorityBalanceBefore;
    const rentRefund = firstPlayerBalanceAfter - firstPlayerBalanceBefore;

    assert.isAtLeast(
      payout,
      expectedWinnerPayout,
      `Winner payout should be at least 95% of the pot (expected: ${expectedWinnerPayout}, got: ${payout})`
    );
    assert.isAtMost(
      payout,
      expectedWinnerPayout,
      `Winner payout should be at most 95% of the pot (expected: ${expectedWinnerPayout}, got: ${payout})`
    );
    assert.isAtLeast(
      authorityPayout,
      authorityFee,
      `Authority payout should be at least 5% of the pot (expected: ${authorityFee}, got: ${authorityPayout})`
    );
    assert.isAtMost(
      authorityPayout,
      authorityFee,
      `Authority payout should be at most 5% of the pot (expected: ${authorityFee}, got: ${authorityPayout})`
    );
    assert.isTrue(
      rentRefund > 0,
      "First player should have received the rent-exempt lamports from the closed game account"
    );
  });
});