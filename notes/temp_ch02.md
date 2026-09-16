

---

## Regression

Una regressione è un bug introdotto in una versione nuova che rompe un comportamento che prima funzionava. Non è un cambio di API dichiarato, ma un errore involontario dell'autore. Per esempio 0.8.6 potrebbe correggere un bug e per sbaglio far sì che gen_range(1..=100) non restituisca mai 100, o introdurre un panic in un caso limite. Il tuo codice continua a compilare perché le firme sono identiche, ma si comporta diversamente. Quindi non viola le regole di SemVer che è una promessa sull'API pubblica intenzionale, non è, e non potrebbe essere, una promessa che il codice sia privo di bug.

---

## Shadowing

TODO