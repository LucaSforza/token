
= Come ottenere ETH nella testnet di sepolia

Per ottenere ETH nella testnet di sepolia basta usare il faucet di polygon che da 0.05 ETH ogni 24 ore. Basta collegarsi su X e anche su GitHub. Momentanemanete sembra rotto il link per collegarsi su GitHub, ma ci si può sempre connettere ad X creando un email usa e getta.

Link per il faucet: #link("https://faucet.polygon.technology/").

Link del RPC: #link("https://ethereum-sepolia-rpc.publicnode.com")

= Liquidity Pool

Per la liquidity pool esiste l'indirizzo per la liquidity factory: 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f e il liquidity router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D.

L'idea è quella di creare una liquidity pool con il nostro token SapiCoin e WETH.

= UML


#figure(
  image("analisi.png")
)

= Auction

Il contratto dell'asta funziona che ognuno che piazza un bid approva all'indirizzo del contratto il trasferimento di TOT token (il valore del bid). Il contratto effettua la trasazione al proprietario dell'asta solo quando il proprietario decide che sia finito.

Ovviamente il proprietario dell'asta deve anche essere il proprietario dell'NFT in palio.

