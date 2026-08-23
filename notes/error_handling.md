# Error handling (Rust)

> Le riflessioni di un principiante di Rust.
>
> Capitoli coinvolti: 09

## 1. Eccezioni vs. errori come valori

- In molti linguaggi orientati agli oggetti molto diffusi, come Java, C# e Python, gli errori sono gestiti con il meccanismo delle eccezioni (`exception`), che usa **lo stesso strumento** sia per gli errori attesi e recuperabili sia per i bug veri e propri, cioè quelli non recuperabili se non modificando il codice stesso. *(Precisazione: Java una distinzione ce l'ha: `Error` per i guasti irrecuperabili come `OutOfMemoryError`, `Exception` per il resto, e checked vs. unchecked, ma il **meccanismo** di lancio e cattura resta identico per tutti, quindi a livello di linguaggio le due categorie non sono realmente separate.)*

- Inoltre le eccezioni sono relativamente costose. Vale la pena essere precisi su *dove* sta il costo: non è nel blocco `try`, perché nelle implementazioni moderne, se non viene lanciata nessuna eccezione, il percorso normale è praticamente a costo zero. Il costo arriva al momento del lancio: costruzione dell'oggetto eccezione, **cattura dello stack trace** (in Java `fillInStackTrace()`, di solito la parte più cara) e srotolamento (*unwinding*) dello stack alla ricerca di un gestore.

- Rust, grazie al suo sistema di tipi molto potente, introduce l'enum `Result` al posto delle eccezioni per gestire gli errori recuperabili. Sono semplici tipi di ritorno delle funzioni, quindi il loro costo è quello di un normale valore di ritorno: trascurabile rispetto al costo di un'eccezione. Da aggiungere: `Result` è annotato con `#[must_use]`, quindi se ignoro il valore restituito il compilatore mi avvisa. È un punto importante: l'errore non può passare inosservato per distrazione.

- Per i bug, invece, Rust introduce `panic!`, concettualmente simile a un'eccezione non catturata: più costoso, si usa per gli errori non recuperabili, dove è inutile restituire un `Result` se il chiamante della funzione non può comunque farci niente una volta ricevuto tale `Result`. In quel caso si preferisce fermare direttamente il programma, per far sapere al programmatore il prima possibile che c'è un errore nel codice e permettergli di correggerlo.


## 2. Checked exception di Java vs. `Result` di Rust

- Il concetto di *checked exception* di Java è molto simile all'enum `Result` di Rust nell'uso quotidiano della gestione degli errori. Con le checked exception, Java integra il concetto di eccezione nel proprio sistema di tipi e nelle firme dei metodi: è il sistema di tipi o diciamo il compilatore a ricordarmi di gestire tutte le possibili eccezioni. Sono quindi costretto o a catturarle (gestione) o a propagarle dichiarandole nella clausola `throws`, rendendole parte della firma del metodo; e chi invoca quel metodo dovrà a sua volta gestirle o ridichiararle.

- In Java, se un metodo può lanciare più checked exception diverse, posso semplicemente dichiararle tutte dopo `throws`. In Rust invece il tipo di ritorno è uno solo: non posso definire una funzione con tanti tipi di ritorno quanti sono i tipi di errore diversi. In Rust la situazione si risolve in due modi:
  1. definendo per conto mio una sorta di **"super errore"** (di solito un `enum` con una variante per ogni errore sottostante) e implementando `From` per convertire i vari tipi di errore in quell'unico tipo. Il bello è che `?` chiama `From::from` **automaticamente**, quindi una volta scritte le `impl From` la conversione diventa invisibile nel codice;
  2. oppure con il polimorfismo dei trait (*non ci sono ancora arrivato*): `Box<dyn Error>`, cioè un **trait object** che rappresenta "un qualunque tipo che implementa `Error`". Più comodo, ma il chiamante perde la possibilità di distinguere i casi con un `match` sulle varianti.

  In pratica: `enum` quando il chiamante deve **reagire diversamente** a seconda dell'errore, `Box<dyn Error>` quando deve solo propagarlo o stamparlo (tipico delle applicazioni, mentre nelle librerie si preferisce l'`enum`).


## 3. `throw` vs `panic!`, `match` e `?`

- In Java il `throw` può comparire ovunque e fa terminare in anticipo il metodo in esecuzione; se non viene mai catturato viene propagato fino a `main` e termina il programma. *(Più precisamente: termina il **thread**; se è un thread secondario, il resto del programma continua, stessa logica del panic in Rust.)*

- Anche in Rust il `panic!` può avvenire ovunque, ma non esiste un meccanismo di cattura paragonabile a quello di Java, quindi in pratica il panic fa terminare il thread e, se è il `main`, il processo.

- `Result`, essendo parte del sistema di tipi, **contiene** un valore, e questa informazione utile va prima estratta. Il modo più esplicito è il pattern matching con `match`, ma non è l'unico: ci sono anche `if let`, `let ... else` e i combinatori (`map`, `map_err`, `and_then`, `unwrap_or`, `unwrap_or_else`, `ok_or`...). E poi, ovviamente, `unwrap` / `expect`, che però trasformano l'errore in un panic.

- Una volta estratta l'informazione, molto spesso il codice si limita a restituire il valore in caso di successo oppure a terminare in anticipo la funzione restituendo l'errore. Proprio per questo schema ripetitivo Rust introduce uno zucchero sintattico: l'operatore `?`. Cosa fa esattamente `?` su un `Result`:
  - se è `Ok(v)`, l'espressione vale `v` e l'esecuzione prosegue;
  - se è `Err(e)`, esegue un **return anticipato** con `Err(From::from(e))`, cioè converte anche
    il tipo di errore.
  Funziona anche su `Option` (dove `None` provoca il return anticipato di `None`) e, più in generale, in qualunque funzione il cui tipo di ritorno implementi il trait `Try`.

## 4. Perché le checked exception di Java sono considerate un errore di progettazione, mentre il `Result` di Rust no?

### 4.1. Java ha due canali, Rust uno solo

Questo è il problema di fondo. In Java convivono checked e unchecked exception, e le seconde non compaiono da nessuna parte nel sistema di tipi. Il risultato è che la garanzia offerta dal compilatore non è mai esaustiva: guardando una firma non so se quel metodo può fallire, so solo se può fallire in un modo che qualcuno ha deciso di dichiarare. E soprattutto esiste sempre una via di fuga: se il compilatore mi dà fastidio, avvolgo tutto in una `RuntimeException` e il problema sparisce dalla firma.

In Rust il canale è uno solo: una funzione che può fallire in modo recuperabile restituisce Result, punto. La "via di fuga" è il panic, che però è una scelta deliberata e visibile nel codice (`unwrap`, `expect`): si vede in review, si può cercare con grep, si può vietare con un lint. Non è un modo per zittire il compilatore, è un cambio di categoria dichiarato.

- Attenzione: non è che Rust tracci tutto. Nemmeno il compilatore di Rust sa se una funzione può andare in panic: `unwrap()`, `a[i]`, una divisione per zero, un'allocazione fallita: niente di tutto ciò compare nella firma. Su questo i due linguaggi sono pari. La differenza sta in **dove passa la linea** tra ciò che è tracciato e ciò che non lo è.

- In Rust la linea coincide con la distinzione semantica: `Result` copre gli errori attesi, il panic copre i bug. Quello che sfugge al compilatore è per definizione la categoria che non voglio gestire caso per caso, quindi una firma senza `Result` significa davvero "questa funzione non ha modi di fallire previsti".

- In Java la linea taglia invece **in mezzo** agli errori attesi. `Integer.parseInt("abc")` lancia `NumberFormatException`, che è unchecked pur essendo il caso normale di un input utente malformato. Una firma senza `throws` quindi non mi garantisce niente: mi dice solo che nessuno ha dichiarato niente. E dove passi la linea dipende da quale classe l'autore della libreria ha scelto di estendere, quindi cambia da libreria a libreria.

- È la stessa ragione per cui `unwrap()` non è l'equivalente Rust di avvolgere in una `RuntimeException`. `unwrap()` **converte** un errore atteso in un bug, e se salta il programma muore rumorosamente. La `RuntimeException` lascia l'errore atteso esattamente com'era recuperabile e gestibile — limitandosi a nasconderlo alla firma.

### 4.2. Gli errori come valori si compongono, le eccezioni no

Un `Result` è un valore ordinario. Posso metterlo in una `struct`, in un `Vec`, restituirlo da una closure, salvarlo per gestirlo dopo, trasformarlo con `map_err`, raccogliere un iteratore di `Result` in un `Result<Vec<_>, _>` con `collect()`. Un'eccezione, invece, è un flusso di controllo *fuori banda*: esiste solo nell'istante in cui viene lanciata e o la gestisco lì, o la lascio passare. Non posso conservarla, non posso metterla in una lista, non posso rimandarla.

### 4.3. Le checked exception non si compongono con i generici

Questo è, secondo me, il difetto tecnico più grave, ed è indipendente dalla sintassi. In Java non esiste il polimorfismo sulle eccezioni: non posso scrivere "questo metodo lancia le stesse eccezioni della funzione che gli passo". Il risultato pratico è che le interfacce funzionali standard (`Function`, `Supplier`, `Consumer`...) non dichiarano `throws`, quindi **dentro una lambda passata a `stream().map()` non posso lanciare una checked exception**. Devo avvolgerla in
una unchecked, cioè devo abbandonare il sistema che dovevo usare.

In Rust il tipo di errore è un normale parametro di tipo: `Result<T, E>` attraversa closure, iteratori, generici e trait senza nessun trattamento speciale, perché non c'è niente di speciale da trattare. È solo un tipo.

### 4.4. Evoluzione delle API

Aggiungere una checked exception alla firma di un metodo pubblico rompe tutti i chiamanti: è un cambiamento incompatibile. Questa è una delle ragioni per cui le librerie hanno smesso di dichiararle. In Rust il problema esiste in forma attenuata: aggiungere una variante a un `enum` di errori può rompere un `match` esaustivo del chiamante, ma ci sono rimedi: `#[non_exhaustive]` sull'enum obbliga i chiamanti a scrivere un braccio `_ =>` fin da subito, e chi si limita a propagare con `?` non deve cambiare nulla.

### 4.6. Anche il modello di Rust ha i suoi critici

Non è che `Result` sia perfetto e la discussione sia chiusa:

- definire i tipi di errore è noioso, tanto che quasi tutti usano i crate `thiserror` (per le librerie) e `anyhow` (per le applicazioni) invece della libreria standard;
- di default si perde il contesto: `?` propaga l'errore ma non dice *dove* è successo, non c'è uno stack trace come in Java. Bisogna aggiungerlo a mano (`.context(...)` di `anyhow`);
- `Box<dyn Error>` vs. `enum` è un compromesso permanente tra comodità e precisione;
- il panic reintroduce comunque un canale non tipizzato, e `unwrap()` nel codice frettoloso è esattamente il `catch (Exception e) {}` di Rust.

