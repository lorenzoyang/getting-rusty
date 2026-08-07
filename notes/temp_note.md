## Cargo.lock

- Il `Cargo.lock` funziona in questo modo, quando scrivi la dipendenza `rand` con la versione `0.8.5` nel tuo `Cargo.toml` e quindi fai la compilazione, Cargo risolve le dipendenze e scarica la versione più recente che soddisfa il vincolo specificato, `0.8.5` significa `^0.8.5` il quale vuol dire `>=0.8.5, <0.9.0`. Quindi se esiste una versione `0.8.6`, Cargo la scaricherà e la userà per la compilazione. Dopo la prima risoluzione delle dipendenze, Cargo.lock viene creato e fissa la versione specifica della libreria `rand` che è stata effettivamente utilizzata, in questo caso `0.8.5` (assumendo che quella fosse la versione più recente disponibile al momento della compilazione). Se nel frattempo esce `0.8.6`, `cargo build` di nuovo, poiché il `Cargo.lock` esiste, Cargo non ri-risolve le dipendenze, non guarda nemmeno crates.io per rand e continua a usare la versione `0.8.5` specificata nel lock file che ha già in cache.

- Il lock non fissa solo `rand` in questo caso, ma anche le dipendenze da cui `rand` dipende con le specifiche versioni.

- Per aggiornare davvero c'è `cargo update`: ignora il `Cargo.lock` e ri-risolve le dipendenze partendo dai vincoli del `Cargo.toml`, poi riscrive il lock con le versioni trovate. Resta però dentro i vincoli SemVer: con `rand = "0.8.5"` (`>=0.8.5, <0.9.0`), se esistono `0.8.6` e `0.999.0`, aggiorna a `0.8.6` e ignora `0.999.0` perché fuori range.

## Regression

Una regressione è un bug introdotto in una versione nuova che rompe un comportamento che prima funzionava. Non è un cambio di API dichiarato, ma un errore involontario dell'autore. Per esempio 0.8.6 potrebbe correggere un bug e per sbaglio far sì che gen_range(1..=100) non restituisca mai 100, o introdurre un panic in un caso limite. Il tuo codice continua a compilare perché le firme sono identiche, ma si comporta diversamente. Quindi non viola le regole di SemVer che è una promessa sull'API pubblica intenzionale, non è, e non potrebbe essere, una promessa che il codice sia privo di bug.

## Shadowing

TODO