# Smart pointer e dereferenziazione

> Le riflessioni di un principiante di Rust.
>
> Capitolo coinvolto: 15


## Smart pointer

- `Box<T>` vs reference: entrambi rappresentano, in un certo senso, il concetto di "puntatore". `Box<T>` memorizza il valore `T` nello heap e ne possiede l'ownership, mentre una reference prende solamente in prestito (`borrow`) un valore e può riferirsi a dati che si trovano sia nello stack sia nello heap.

  Tuttavia, secondo me la differenza principale è proprio questa: `Box<T>` rappresenta un **possesso**, mentre una reference rappresenta un **prestito**. Di conseguenza, usando le reference entra spesso in gioco il concetto di lifetime. In realtà ogni reference possiede sempre un lifetime, anche quando viene dedotto implicitamente dal compilatore. Un `Box<T>`, invece, non introduce di per sé un lifetime di borrowing, a meno che naturalmente il tipo `T` non contenga a sua volta delle reference.

- `RefCell<T>`: si usa quando abbiamo bisogno della cosiddetta **interior mutability**, cioè quando vogliamo poter mutare lo stato interno di un valore anche se abbiamo accesso al contenitore soltanto attraverso una reference condivisa/immutabile.

  `RefCell<T>` non elimina le regole di borrowing di Rust: semplicemente le controlla a runtime invece che a compile time. Con `borrow()` possiamo ottenere un `Ref<T>`, mentre con `borrow_mut()` possiamo ottenere un `RefMut<T>`. Se violiamo le normali regole di borrowing, per esempio cercando di ottenere due borrow mutabili contemporaneamente, il programma va in panic a runtime.

  - Nel contesto del testing, possiamo avere per esempio un trait la cui signature richiede, per la logica del progetto, una reference immutabile come `&self`. Supponiamo però di voler creare un oggetto mock che implementa tale trait e di avere bisogno, esclusivamente per il testing, di modificare lo stato interno del mock, per esempio per registrare quante volte è stato chiamato un determinato metodo.

    Non avrebbe senso modificare la signature del trait soltanto per adattarla alle necessità del test: siamo noi che dobbiamo adattare il mock all'interfaccia richiesta dal progetto. In questo caso possiamo utilizzare `RefCell<T>` per mantenere `&self` nell'implementazione del trait e, allo stesso tempo, modificare lo stato interno del mock attraverso `borrow_mut()`.

  - Più in generale, `RefCell<T>` è utile nelle situazioni in cui abbiamo bisogno di mutare lo stato interno attraverso un accesso esternamente immutabile e sappiamo che le regole di borrowing verranno rispettate, anche se il compilatore non è in grado di verificarlo staticamente.

## Dereferenziazione in Rust: `Deref`, deref coercion, autoderef e autoref

In Rust abbiamo diversi tipi che possono comportarsi, in qualche modo, come dei puntatori: reference, `Box<T>`, `Rc<T>`, `Ref<T>`, `RefMut<T>` e altri.

Nasce quindi la necessità di rendere il loro utilizzo il più possibile uniforme, in modo da alleggerire il modello mentale del programmatore.

Partendo dal costrutto nativo delle reference, Rust permette di utilizzare l'operatore di dereferenziazione `*` per seguire una reference e accedere al valore a cui essa punta.

Gli smart pointer, però, non sono semplicemente dei wrapper di reference: sono tipi che si comportano come puntatori e che possono aggiungere funzionalità ulteriori, come ownership, reference counting e così via.

Per permettere a questi tipi di offrire un comportamento di dereferenziazione simile a quello delle reference, Rust mette a disposizione il trait `Deref`.

Quando utilizziamo `*` su un tipo che implementa `Deref`, Rust utilizza il metodo `deref()` per ottenere una reference al valore target e successivamente applica la normale dereferenziazione.

Per esempio:

```rust
use std::ops::Deref;

let b = Box::new(5);

let x = *b;

// Concettualmente equivalente a:
let x = *Deref::deref(&b);
```

`Deref::deref(&b)` restituisce in questo caso una `&i32`; applicando successivamente `*`, arriviamo al valore `i32`.

Il fatto che `deref()` restituisca una reference è importante: se restituisse direttamente il valore interno, in molti casi finiremmo per spostare (`move`) il valore fuori dallo smart pointer.

Esiste poi il corrispondente trait `DerefMut`, che permette la dereferenziazione mutabile. In questo caso il metodo principale è concettualmente:

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

e permette di ottenere una reference mutabile al target quando abbiamo accesso mutabile al tipo che implementa `DerefMut`.

### Deref coercion

Abbiamo poi la **deref coercion**.

La deref coercion permette a Rust di convertire automaticamente una reference a un tipo che implementa `Deref` in una reference al suo `Target`.

Per esempio, se una funzione accetta:

```rust
fn hello(name: &str) {
    println!("Hello, {name}");
}
```

possiamo passarle una `&String`, perché `String` implementa:

```text
Deref<Target = str>
```

e Rust può quindi effettuare automaticamente:

```text
&String -> &str
```

La deref coercion può anche avvenire più volte consecutivamente.

Per esempio, con:

```rust
let name = Box::new(String::from("Rust"));

hello(&name);
```

il compilatore può effettuare concettualmente questa catena:

```text
&Box<String> -> &String -> &str
```

chiamando automaticamente le necessarie implementazioni di `Deref`.

L'obiettivo è permettere ai diversi tipi "pointer-like" di presentare un'interfaccia più uniforme al programmatore, evitando di dover scrivere manualmente tutte le chiamate a `deref()`.

C'è però una precisazione importante per `RefCell<T>`.

`RefCell<T>` **non implementa direttamente `Deref<Target = T>`**, quindi una cosa come:

```text
&RefCell<Box<T>> -> &Box<T> -> &T
```

non avviene automaticamente.

Dobbiamo prima ottenere un borrow:

```rust
use std::cell::RefCell;

fn use_value(value: &i32) {
    println!("{value}");
}

let cell = RefCell::new(Box::new(5));

let borrowed = cell.borrow();
// borrowed: Ref<'_, Box<i32>>

use_value(&borrowed);
```

`borrow()` restituisce un `Ref<Box<i32>>`, e `Ref<T>` implementa `Deref`. A quel punto Rust può seguire una catena concettualmente simile a:

```text
&Ref<Box<i32>> -> &Box<i32> -> &i32
```

Questa distinzione è importante: è `Ref<T>` (e analogamente `RefMut<T>`) a comportarsi come uno smart pointer dereferenziabile verso `T`, non direttamente `RefCell<T>`.

### Autoderef e autoref

Autoderef e autoref entrano in gioco soprattutto durante la risoluzione delle chiamate ai metodi.

Supponiamo di avere:

```rust
struct Person;

impl Person {
    fn hello(&self) {
        println!("Hello!");
    }
}
```

e:

```rust
let person = Person;

person.hello();
```

Il metodo `hello` richiede `&self`, mentre noi abbiamo un valore di tipo `Person`.

Durante la ricerca del metodo corretto, Rust considera automaticamente anche una reference al receiver. Possiamo quindi pensare, come modello mentale semplificato, che il compilatore sia in grado di trasformare concettualmente:

```rust
person.hello();
```

in qualcosa di simile a:

```rust
(&person).hello();
```

Questo comportamento viene chiamato **autoref**.

Durante la ricerca di un metodo Rust può anche dereferenziare automaticamente il receiver più volte. Questo comportamento viene chiamato **autoderef**.

Per esempio, una chiamata può funzionare anche se partiamo da:

```rust
&Person
```

oppure:

```rust
&&Person
```

perché durante la risoluzione del metodo Rust considera progressivamente i tipi ottenuti dereferenziando il receiver.

Lo stesso meccanismo può utilizzare anche il trait `Deref`. Se abbiamo quindi uno smart pointer che implementa `Deref<Target = Person>`, Rust può arrivare a considerare `Person` durante la ricerca del metodo.

Perciò **deref coercion** e **autoderef/autoref** sono meccanismi collegati, ma non sono esattamente la stessa cosa.

La **deref coercion** è una coercizione di tipo che permette, in determinati contesti, di trasformare automaticamente una reference come:

```text
&Box<String> -> &String -> &str
```

L'**autoderef/autoref**, invece, fa parte soprattutto del processo con cui Rust cerca il metodo da chiamare sul receiver: il compilatore prova varie dereferenziazioni del receiver e considera automaticamente anche le relative forme `&T` e `&mut T`.

Entrambi i meccanismi contribuiscono allo stesso obiettivo generale: rendere reference e tipi che implementano `Deref` molto più naturali da utilizzare, evitando al programmatore di scrivere continuamente dereferenziazioni e borrow espliciti.

