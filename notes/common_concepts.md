# Common programming concepts

> Le riflessioni di un principiante di Rust.
>
> Capitoli coinvolti: 03

## Unit Type

Rust è un linguaggio prevalentemente orientato alle espressioni (*mostly expression-oriented*): quasi tutti i costrutti sono espressioni e quindi producono un valore. Il punto e virgola non "restituisce" niente: trasforma un'espressione in uno **statement** e ne **scarta** il valore. Di conseguenza un blocco `{ ... }` vale il valore della sua ultima espressione, oppure `()` se il suo ultimo elemento è uno statement (cioè se termina con `;`).

`()` è una tupla di zero elementi, e in Rust questa 0-tupla ha un nome particolare: `unit type`. Il tipo si scrive `()` e il suo unico valore si scrive `()`. Essendo l'unico valore possibile, non serve memorizzarlo: è un tipo *zero-sized*, occupa 0 byte.

```rust
let a = if c { 1 } else { 2 };    // ok, a: i32
let b = if c { 1; } else { 2; };  // ok, b: () — il `;` scarta il valore dei due rami

// ok, b: () perché il blocco termina con uno statement (`f();`),
// quindi il valore restituito da f() viene scartato
let b: () = { f(); };

// invece
let c: () = (let y = 5);          // ERRORE: `let` è uno statement, non un'espressione
let d = (fn g() {});              // ERRORE: anche una dichiarazione `fn` è uno statement

// però il *nome* di una funzione è un valore, quindi questo va bene:
fn g() {}
let d = g;                        // ok, d è lo zero-sized fn item di `g`
```

Nota: una funzione senza tipo di ritorno restituisce implicitamente `()`;
`fn f() {}` e `fn f() -> () {}` sono la stessa cosa.

- Grazie a questa proprietà delle espressioni, in Rust non c'è bisogno dell'operatore ternario (sarebbe ridondante), come invece succede in C, C++, Java, ecc. In quei linguaggi `if` è uno statement, non restituisce un valore, e l'operatore ternario esiste proprio per colmare questa lacuna. In Rust si può scrivere direttamente:
  ```rust
  let x = if cond { 1 } else { 2 };
  ```
  Lo stesso vale per `match`, `loop` e per i blocchi `{ ... }`, che sono anch'essi espressioni.

- In C# il tipo `void` non è un tipo utilizzabile: esiste nel CLR come `System.Void`, ma il linguaggio ne vieta l'uso come tipo di una variabile o come argomento generico. Da qui nasce il bisogno di avere `Action` e `Func` per colmare questo buco. In Rust invece il tipo unit è un tipo a tutti gli effetti: può essere il tipo di una variabile e può essere usato come parametro generico, quindi non serve nessuna distinzione tra `Action` e `Func`, tra `Task` e `Task<T>`, ecc. Esiste solo `Fn() -> ()` e solo `Result<T, E>` (dove `T` può benissimo essere `()`).

- L'"opposto" del tipo unit è il never type `!`: il tipo delle espressioni che non restituiscono mai il controllo, come `panic!()`, `return`, `break`, `continue`, `loop {}`. Mentre `()` è un tipo con un solo valore, `!` è un tipo con zero valori, e per questo può essere coercito in qualsiasi altro tipo. È il motivo per cui questo codice compila:

  ```rust
  fn f(cond: bool) {
      let x: i32 = if cond { 1 } else { return };  // `return` ha tipo `!`
      println!("{x}");
  }
  ```