
= Come ottenere ETH nella testnet di sepolia

Per ottenere ETH nella testnet di sepolia basta usare il faucet di polygon che da 0.05 ETH ogni 24 ore. Basta collegarsi su X e anche su GitHub. Momentanemanete sembra rotto il link per collegarsi su GitHub, ma ci si può sempre connettere ad X creando un email usa e getta.

Link per il faucet: #link("https://faucet.polygon.technology/").

Link del RPC: #link("https://ethereum-sepolia-rpc.publicnode.com")

= Liquidity Pool

Per la liquidity pool esiste l'indirizzo per la liquidity factory: 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f e il liquidity router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D.

L'idea è quella di creare una liquidity pool con il nostro token SapiCoin e WETH.

Però nonostante uniswap sia state-of-art utilizziamo un contratto più semplice.

In una liquidity pool l'obbiettivo è scambiare token tra utenti della blockchain.

Quindi l'address del contratto avrà l'ownership di etherium e SapiCoin da scambiare.

Poi viene deciso un tasso di cambio. Nelle valute FIAT le banche centrali (BCE) decidono il tasso di cambio.

In un contratto il tasso di cambio viene deciso da un "oracolo", che non necessariamente deve essere un individuo, ma codice all'interno del contratto stesso che la usa come regola per calcolare il tasso di cambio.

La regola che noi imponiamo è la Constant Product Formula: $x dot y=k$

La costante $k$ è l'invariante del nostro sistema. $x$ e $y$ posso mutare, ma sempre rispettando il vincolo imposto dall'oracolo.

Però qua viene un problema... chi verserà i soldi nella liquidity pool?

Nessuno metterebbe soldi dentro una liquidity pool senza averne un ricavato.

Quindi ai scambi di valuta avvenuti dentro al contratto una percentuale viene trattenuta per gli investitori.

Nella nostra liquidity pool la tassa è del 0.3%.

Quindi se volessi cambiare dei soldi, come si dovrebbe fare?

Sia $x$ la quantità totale di soldi della valuta x.

Sia $y$ la quantità totale di soldi della valuta y.

$x_"in"$ è la quantità di soldi che vogliamo scambiare, quindi quelli immessi nella liquidity pool di netto.

Dobbiamo calcolare il loro, che sarà la quantità effettiva che viene inserito nel valore totale della liquidty pool.

Questo perché gli investori quando decidono di fare il withdraw del contratto la quantità delle due valute che ottengono in base al tasso di cambio di quel monento. // TODO: controllare

Quindi $x_"taxed" = x_"in" - x_"in" dot 0.003$ 

il tasso di cambio è : $x/y$.

Quindi $y_"out"$, ovvero la quantità della valuta y che viene data in cambio degli $x_"taxed"$.

$y_"out" = (x_"taxed" dot y)/(x_"taxed" + x)$

Questo ci assicura che $k$ rimane invariato (e anche il tasso di cambio).

L'unico modo in cui cambiare $k$ è con l'arrivo di più investitori.

Come si investe?

Chi investe sta aggiungendo liquidità (quindi valore) alla liquidity pool.

Ogni investirore ci si deve salvare la quantità di share nella liquidity pool, ovvero quanto è investito.

$x_"invt"^i$ sono i soldi investiti dall'investitore $i$.

La quota di proprietà viene calcolato con i LP token.

Quando viene inizializata la pool il primo investore detiene l'intera quota.

Il valore totale della pool è proporzionale a $sqrt(x dot y)$., quindi la liquidità totale è $L = sqrt(x dot y)$

Quindi quando entra un investitori viene calcolato quanto valore stanno inserendo.

$"LP" = sqrt(x_"invt"^i dot y)$

Questo valore viene aggiunto alla liquidità totale e salvato per ogni investitore.

Quando un investitore vuole fare i withdraw deve bruciare i suoi LP.

Riceve indietro ETH e SapiCoin in proporzione alla sua quota della pool attuale.

Se la pool nel frattempo è cresciuta grazie alle fee, allora la sua quota vale di più.


= UML


#figure(
  image("analisi.png")
)

= Auction

Il contratto dell'asta funziona che ognuno che piazza un bid approva all'indirizzo del contratto il trasferimento di TOT token (il valore del bid). Il contratto effettua la trasazione al proprietario dell'asta solo quando il proprietario decide che sia finito.

Ovviamente il proprietario dell'asta deve anche essere il proprietario dell'NFT in palio.

