# Enums

> Le riflessioni di un principiante di Rust.
>
> Capitoli coinvolti: 06


Sum type, product type, union type e discriminated union.

## Sum type

`enum` in Rust non è semplicemente una lista di costanti come nei linguaggi classici (C, C++, Java), ma è un **sum type**, cioè un tipo che esprime una scelta tra più alternative. La scelta deve essere una sola: un valore non può stare in due varianti contemporaneamente.

```rust
enum E { A(bool), B(u8) }  // 2 + 256 = 258 valori
```

Qui `E` è un tipo che può assumere 258 valori diversi: 2 valori possibili nel caso della scelta `A`, che contiene al suo interno un booleano che può essere `true` o `false`, e 256 valori possibili nel caso della scelta `B`, che contiene al suo interno un `u8`, cioè un qualsiasi intero tra 0 e 255. Poiché possiamo avere soltanto una scelta tra `A` e `B`, e non entrambe in contemporanea, il numero totale di valori possibili è 2 + 256 = 258.

Il sum type ha quindi soltanto due forme: `A` oppure `B`.

> **Cardinalità ≠ dimensione.** 258 è il numero di valori che il tipo può assumere, non i byte che occupa. In memoria `E` occupa quanto la variante più grande, più lo spazio per il discriminante, più l'eventuale padding. Il layout preciso di un `enum` senza `#[repr(...)]` non è garantito dal linguaggio; la cardinalità sì.

> **Sull'`enum` degli "altri linguaggi".** In C e C++ l'`enum` è davvero una lista di costanti intere. In Java è già qualcosa di più (ogni costante è un oggetto, con campi e metodi propri), ma resta un insieme fisso di *istanze dello stesso tipo*: le costanti non possono portare payload di tipo diverso l'una dall'altra. È esattamente questo che manca perché sia un sum type.

## Product type

Dall'altra parte, come le classi in Java o le `struct` di Rust, abbiamo i **product type**, cioè un tipo che esprime una combinazione di più valori: tutti i valori devono essere presenti contemporaneamente.

```rust
struct S { a: bool, b: u8 }  // 2 * 256 = 512 valori
```

Con il product type, la `struct` `S` ha 512 valori possibili e una forma sola: quella in cui sono presenti contemporaneamente sia `a` che `b`. Per questo un pattern che destruttura una `struct` è sempre esaustivo: non c'è nessun altro caso da coprire.

## Union type

Lo union type è semplicemente un tipo che rappresenta l'unione insiemistica di due o più tipi, per esempio in TypeScript:

```typescript
type Value = string | number;
```

Non ha un costruttore, non ha wrapping: una `string` è già di per sé un `Value`, quindi non c'è bisogno di fare prima pattern matching per trovare la forma voluta e poi estrarre il valore da quest'ultima.

Lo union type è spesso **strutturale**, come in TypeScript, e non **nominale** come il sum type di Rust: in Rust `enum E { A(String), B(String) }` ha due varianti perfettamente distinte, perché il nome della variante è l'identità. Questo permette di dare significati diversi allo stesso tipo sottostante, come in `enum Id { Utente(String), Ordine(String) }`, cosa che in TypeScript richiede un tag esplicito. Inoltre TypeScript collassa i duplicati (`string | string` diventa `string`), mentre in Rust le due forme `A(String)` e `B(String)` sono distinte e rimangono tali.

## Discriminated union

La discriminated union è un modo diverso di usare lo union type per simulare un sum type.

Viene dalla `union` del C: più campi che condividono la stessa memoria, ma senza che il linguaggio sappia quale sia valido in questo momento. Devi tenertelo a mente tu, di solito con un campo tag a fianco che aggiorni a mano. Se sbagli, leggi byte a caso.

L'`enum` di Rust è esattamente quella coppia (tag + union), ma costruita e sorvegliata dal compilatore. La differenza è che il tag non è materiale su cui posso mettere le mani:

- **non lo posso scrivere**: l'unico modo di produrre un valore è usare un costruttore di variante, quindi tag e contenuto non possono mai andare fuori sincrono;
- **non lo posso usare per arrivare al contenuto**: l'unica strada verso il payload è un pattern (`match`, `if let`, `while let`, `let ... else`) oppure un metodo che al suo interno fa esattamente questo (`unwrap`, `map`, `?`, ...). Il pattern è la prova che il caso lo sto gestendo.

Da notare anche che "tag + union" è il modello concettuale, non necessariamente il layout fisico: grazie alla *niche optimization* il tag a volte non esiste affatto in memoria (vedi `Option<&T>` più avanti).

La union di TypeScript è invece **non taggata**: è l'unione insiemistica di due tipi, come detto prima. Non c'è un costruttore, una stringa è già un `Value` e non va messa dentro niente. A runtime si distingue ispezionando il valore:

```typescript
if (typeof v === "string") { /* qui v: string */ }
```

Per i casi che `typeof` non distingue si usa il pattern discriminated union, dove però il tag me lo scrivo io:

```typescript
type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "square"; side: number };
```

Quindi in Rust l'`enum` è detto anche discriminated union con tag imposto dal compilatore, mentre in TypeScript la discriminated union è un pattern che si può usare per simulare un sum type, ma il tag lo scrivo io.

## In Java

Java storicamente non aveva sum type: aveva soltanto product type (`class`) e l'ereditarietà. Il modo classico di emulare le "forme" era una gerarchia di classi più il **Visitor pattern**:

```java
interface Shape { <R> R accept(Visitor<R> v); }

interface Visitor<R> {
    R visitCircle(Circle c);
    R visitSquare(Square s);
}

class Circle implements Shape {
    double radius;
    public <R> R accept(Visitor<R> v) { return v.visitCircle(this); }
}
```

Il Visitor funziona a metà:

- **Cosa dà:** un pezzo di esaustività. Se aggiungo `visitTriangle` all'interfaccia `Visitor`, tutte le implementazioni esistenti smettono di compilare, che è lo stesso effetto che si ha quando aggiungo una variante a un `enum` di Rust e tutti i `match` si rompono. (Vale se il nuovo metodo è astratto: se lo dichiaro `default`, il controllo salta.)

- **Cosa non dà:** la **chiusura** del tipo. `interface Shape` resta aperta: chiunque, da qualsiasi modulo, può scrivere `class Triangle implements Shape`, e il compilatore non lo segnala da nessuna parte, perché niente lega `Shape` a `Visitor`. La corrispondenza tra "insieme delle sottoclassi" e "insieme dei metodi visit" la mantengo io, a mano, per convenzione.

Si noti anche la direzione del controllo: il compilatore mi obbliga a reagire quando cambio il `Visitor`, ma non ha modo di accorgersi se qualcuno aggiunge una sottoclasse *senza* toccare il `Visitor`.

Quindi il Visitor simula il **dispatch** su un sum type, ma non la chiusura, che è esattamente la condizione necessaria perché l'esaustività sia verificabile. Se il compilatore non sa quali sono tutte le forme, non può dirmi che le ho coperte tutte.

### `sealed`: chiudere l'insieme

Da Java 17 (con il pattern matching su `switch` completo, record pattern inclusi, in Java 21):

```java
sealed interface Shape permits Circle, Square {}

record Circle(double radius) implements Shape {}
record Square(double side) implements Shape {}

double area(Shape s) {
    return switch (s) {
        case Circle(double r) -> Math.PI * r * r;
        case Square(double l) -> l * l;
        // niente default: il compilatore sa che sono tutte
    };
}
```

`sealed` è il pezzo mancante: `permits` **chiude** l'insieme delle sottoclassi. Ora il compilatore conosce tutte le alternative e può verificare l'esaustività. Una volta ottenuto questo, il Visitor diventa obsoleto: stesse garanzie, tre livelli di boilerplate in meno.

Due dettagli pratici: `permits` si può omettere se le implementazioni stanno nello stesso file sorgente (il compilatore le ricava da sé), e ogni sottotipo deve dichiararsi `final`, `sealed` o `non-sealed` — i `record` sono già implicitamente `final`.

Due keyword per due livelli. Si vede bene come Java separi ciò che Rust fonde:

| livello                   | Java                           | Rust                       |
| ------------------------- | ------------------------------ | -------------------------- |
| sum (le forme, chiuse)    | `sealed interface ... permits` | `enum`                     |
| product (i campi insieme) | `record Circle(double r)`      | la sintassi della variante |

`sealed interface` + `record` = somma di prodotti, la stessa struttura di `enum Shape { Circle { r: f64 }, Square { l: f64 } }`. Java ha bisogno di due dichiarazioni perché ogni variante è un tipo indipendente; Rust ne usa una sola.

### Le varianti Java sono tipi veri

Differenza che spiazza venendo dall'OO: in Java `Circle` è un tipo a sé, e posso scrivere `void f(Circle c)`. C'è subtyping: `Circle <: Shape`.

In Rust **`Shape::Circle` non è un tipo**: è solo un costruttore. Non posso scrivere una funzione che accetti soltanto quella variante; se mi serve, devo definire una `struct Circle` separata e metterla dentro la variante. E non esiste alcuna relazione di sottotipo: c'è un tipo solo, `Shape`.

Anche in memoria la differenza è netta: in Java ogni variante è un oggetto sull'heap e la variabile è un riferimento (salvo ottimizzazioni della JVM, come l'escape analysis, che però non sono garantite dal linguaggio); in Rust il valore sta inline, grande quanto la variante più grande più il discriminante, senza allocazione.

**Il buco che resta: `null`.** Uno `Shape` in Java può comunque essere `null`, e il `switch` con pattern lancia `NullPointerException` se non lo gestisco (posso aggiungere `case null ->`, ma resta un caso in più che il tipo non dichiara). L'esaustività copre le forme che ho dichiarato, ma non quella variante invisibile che Java aggiunge a ogni tipo di riferimento. `sealed` recupera la chiusura, ma non toglie `null`: il tipo dichiarato continua a mentire. Ed è esattamente il problema che `Option` risolve in Rust — non perché sia una classe migliore di `Optional`, ma perché in Rust `null` non esiste più come alternativa.

## `Option`

Come Rust toglie il valore `null` tenendo comunque il concetto.

In Java, quando accedi a una variabile di tipo riferimento cioè di tipo non primitivo, i tipi i cui valori sono allocati sull'heap potresti trovarti il valore `null`, un valore speciale accettato dal compilatore per tutti i tipi di riferimento, che rappresenta l'assenza del valore. Di conseguenza, ogni volta che ci fai accesso devi prima controllare se il valore è `null` oppure no, altrimenti rischi un `NullPointerException` a runtime; e poiché `null` è un valore legittimo per il compilatore, quest'ultimo non può farci niente, non può aiutarti a prevenire il problema. Il fatto che i programmatori debbano fare il controllo a mano ogni volta è molto rischioso e soggetto a errori.

In Rust, invece, il valore `null` non esiste proprio. Non è una questione di stack contro heap: i tipi che "puntano" a qualcosa cioè i riferimenti `&T` e `&mut T`, `Box<T>`, e in generale i tipi della libreria standard — sono garantiti non nulli, e non esiste un letterale `null` da assegnare loro. (L'unica eccezione sono i *raw pointer*, `*const T` e `*mut T`, che possono valere `std::ptr::null()`; ma non si possono dereferenziare fuori da un blocco `unsafe`, quindi non sono un buco nel sistema di tipi: sono un'uscita di emergenza che devo dichiarare esplicitamente.)

Ma il concetto di "assenza del valore" esiste comunque, ed è rappresentato dal tipo `Option<T>`, che è un sum type (un `enum`) con due varianti: `Some(T)` e `None`. `Some(T)` rappresenta la presenza di un valore di tipo `T`, `None` la sua assenza. In questo modo, quando trovi una variabile di tipo `Option<T>`, sai che potrebbe non avere un valore; infatti non la puoi usare direttamente:

```rust
let x: Option<i32> = None;
let y: i32 = 10;
let sum = x + y;  // ERRORE: non puoi sommare un Option<i32> con un i32
```

Sono due tipi diversi, e il compilatore in questo caso ci aiuta impedendoci di fare un errore di tipo, obbligandoci a gestire il caso prima di usare il valore. Posso sempre scrivere `x.unwrap()` e farmi esplodere il programma a runtime, ma è una scelta esplicita che devo mettere nero su bianco: il comportamento di default è gestire entrambi i casi.

Quindi in Rust il sistema di tipi, insieme al compilatore, ci aiuta a prevenire il problema dell'accesso al valore nullo.

- **Non è la classe, è la chiusura.** `Optional` in Java è la stessa idea, ma un `Optional` può a sua volta essere `null`, e niente impedisce di continuare a usare riferimenti nudi accanto ad esso. In Rust non c'è alternativa: se il tipo dice `T`, un valore di `T` c'è.

## `if let` e `let ... else`

Entrambi sono zucchero sintattico su un `match` con un pattern solo. La differenza vera sta nel *dove vive* la variabile che ho estratto.

`if let` — scope interno:

```rust
if let Some(x) = opzione {
    println!("{x}");    // x esiste solo qui dentro
} else {
    // qui x NON esiste
}
// e nemmeno qui
```

`let ... else` — scope esterno:

```rust
let Some(x) = opzione else {
    return;             // deve divergere: return, break, continue, panic!...
};
println!("{x}");        // x vive da qui fino a fine scope
```

Il blocco `else` di `let ... else` deve avere tipo `!`, cioè deve uscire di scena: `return`, `break`, `continue`, `panic!`, `std::process::exit(...)`. Non può semplicemente produrre un valore alternativo, per quello esistono `unwrap_or`, `unwrap_or_else` e compagnia.

`let ... else` è quindi la forma giusta per l'*early return*: tiene il caso felice al livello di indentazione principale, invece di annidarlo dentro un `if let`. Nella stessa famiglia c'è anche `while let`, che ripete il pattern finché continua a corrispondere:

```rust
while let Some(top) = stack.pop() {
    println!("{top}");
}
```
