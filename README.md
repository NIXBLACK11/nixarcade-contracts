Setup

spl-token create-account So11111111111111111111111111111111111111112 --owner 6iTLyzNKRfprK2njGTubCxdWMvJgu4LFH3V6Kht4Tn2a

async function deriveEscrowTokenAccount(gameAccount) {
    const wsolMint = new PublicKey("So11111111111111111111111111111111111111112");
    const gameAccountKey = new PublicKey(gameAccount);

    const escrowTokenAccount = await getAssociatedTokenAddress(
        wsolMint,
        gameAccountKey,  // <- Authority should be the GAME ACCOUNT, NOT the user!
        false
    );

    console.log("Escrow Token Account Address:", escrowTokenAccount.toBase58());
}
