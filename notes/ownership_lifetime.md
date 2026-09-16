# Ownership e lifetime

> Le riflessioni di un principiante di Rust.
>
> Capitoli coinvolti: 04, 10


## References and Borrowing

### Stack vs Heap in generale

Quando un programma viene caricato in memoria diventa un processo, e il processo vede la memoria come uno spazio di indirizzi tutto suo, lineare (è la *memoria virtuale*: saranno poi il sistema operativo e l'hardware a tradurre questi indirizzi in indirizzi fisici). Dentro questo spazio ci sono due aree, lo `stack` e l'`heap`, che non sono fatte di memoria diversa: si differenziano solo per il modo in cui vengono gestite.

- **Stack:** il ciclo di vita delle chiamate di funzione segue proprio il principio LIFO (Last In, First Out): quando una funzione viene chiamata viene allocata la memoria necessaria al suo contesto di esecuzione (variabili locali, parametri, indirizzo di ritorno, ecc.), questo blocco si chiama *stack frame* e quando la funzione termina l'allocazione viene liberata, l'ultima funzione chiamata è la prima a terminare. Nasce così l'idea di applicare il principio LIFO per gestire la memoria, e l'area di memoria gestita in questo modo viene chiamata stack. Allocare e liberare qui costa pochissimo: basta spostare avanti o indietro un puntatore (lo *stack pointer*).

  - La memoria viene quindi allocata quando la funzione viene chiamata e liberata quando la funzione termina, e da qui viene fuori un nuovo problema: e se avessi bisogno di dati che sopravvivono alla funzione che li ha creati, cioè di dati con una lifetime più lunga della chiamata di funzione in questione? In tal caso serve un'area di memoria che non sia gestita in maniera LIFO, ed è proprio questa l'area che viene chiamata heap.

  - Inoltre, quando la funzione viene chiamata, il frame corrispondente viene allocato e pushato in cima allo stack. Supponiamo che durante l'esecuzione ci sia, per esempio, un array di 10 elementi e che tale array debba crescere ancora: ecco che viene fuori un altro problema. Il frame viene allocato alla chiamata della funzione con una dimensione fissa e, poiché si segue il LIFO, sopra di esso potrebbe esserci già un altro frame appena pushato, quindi il frame non può crescere: è bloccato lì dentro. Non posso quindi tenere un array di dimensione dinamica direttamente nel frame. Anche qui viene fuori l'idea di avere un'area di memoria indipendente dallo stack: nel frame metto un valore di dimensione fissa, cioè l'indirizzo di un'area di memoria dinamica, e tale indirizzo è proprio il concetto di puntatore. Così si riesce ad avere un array di dimensione dinamica pur restando dentro un frame di dimensione fissa nell'area stack.

    > Nota: qualche linguaggio permette di far crescere il frame a runtime (per esempio `alloca` o i VLA in C), ma solo finché quel frame è in cima allo stack. In Rust questo non esiste: la dimensione di ogni frame è nota a compile time, ed è per questo che il compilatore vuole conoscere la size di ogni tipo, oppure ci obbliga a metterlo dietro un puntatore come `Box<T>`.

  - Nota: lo stack ha anche una dimensione massima fissata, e ogni thread ha il suo (tipicamente 8 MiB per il thread principale su Linux, 2 MiB per i thread creati con `thread::spawn`). Superarla significa *stack overflow*.

- **Heap:** è l'area di memoria nata proprio per risolvere i problemi suddetti. Qui la memoria non viene allocata e liberata automaticamente con push e pop, ma viene allocata su richiesta, ottenendo un puntatore all'area di memoria allocata, e poi, quando non serve più, viene liberata sempre su richiesta. Gestire quest'area è più costoso rispetto allo stack: l'allocatore deve cercare un blocco libero abbastanza grande, tenere traccia di ciò che è occupato e di ciò che è libero, ed eventualmente chiedere altra memoria al sistema operativo. Anche l'accesso ai dati tende a costare di più, non perché leggere l'heap sia di per sé più lento, ma perché prima bisogna seguire un puntatore e perché i dati sono sparsi, quindi si sfrutta peggio la cache della CPU. In compenso i dati allocati nell'heap sopravvivono alla chiamata della funzione che li ha creati, finché non vengono liberati.

  - E qui arriva il punto di Rust: quel "finché non vengono liberati" in C significherebbe una `free()` scritta a mano (dimenticarla = memory leak, farla due volte = double free), mentre in Rust ogni valore nell'heap ha un *owner*, e la deallocazione viene inserita dal compilatore alla fine dello scope dell'owner. Niente garbage collector e niente `free()` manuale: è esattamente il problema che il concetto di ownership viene a risolvere.

  - Per esempio, un `Vec<T>` non è altro che una struct di dimensione fissa che sta nel frame (puntatore ai dati + length + capacity) e che punta a un buffer nell'heap: quando il `Vec` cresce oltre la sua capacity cambia il buffer nell'heap, mentre la struct nello stack resta sempre della stessa dimensione.


## Lifetimes

- **borrowing** = l'operazione/relazione con cui si prende in prestito un valore, cioè si accede temporaneamente a esso senza prenderne l'ownership;

- **reference** = il valore attraverso cui si accede al dato preso in prestito.

```rust
fn foo<'s>(&'s self)
````

> `&'s self` è quindi un riferimento a `self`; tramite questo riferimento, `self` rimane borrowed per la regione in cui quel riferimento deve essere valido, rappresentata da `'s`.

- I **lifetime parameters** non sono un meccanismo completamente separato dal sistema dei generics. Un lifetime parameter è infatti un parametro generico, come `'a` o `'s`. Non è però un parametro di tipo come `T`: invece di rappresentare un tipo qualsiasi, funziona come un **placeholder per una regione di validità dei riferimenti**.

  Quindi:

  ```rust
  fn foo<T>(x: T)
  ```

  usa un parametro generico di tipo, mentre:

  ```rust
  fn foo<'a>(x: &'a str)
  ```

  usa un parametro generico di lifetime.

  Si può quindi ragionare così:

  ```text
  T   → placeholder per un tipo
  'a  → placeholder per una regione di validità
  ```

  Quando una funzione che usa lifetime parameters viene chiamata, il compilatore determina quali regioni concrete possono soddisfare quei parametri e le relazioni espresse dalla firma, in modo analogo a come un generic type parameter viene istanziato con un tipo concreto.

  È importante però non identificare necessariamente una lifetime con uno scope lessicale: soprattutto con NLL, la regione associata a una lifetime può essere più piccola di un intero blocco.

  Inoltre, a differenza dei generic type parameters, i lifetime parameters non causano la monomorfizzazione del codice: servono al controllo statico effettuato dal borrow checker e non producono versioni runtime differenti della funzione.

- I **lifetime parameters e le relative annotazioni sui riferimenti** non modificano il comportamento runtime del programma e, soprattutto, non fanno vivere più a lungo né i dati né i riferimenti. Servono invece a descrivere al compilatore le relazioni tra la validità di più riferimenti.

  In altre parole: non dicono al compilatore quanto deve far vivere un dato; gli dicono quali relazioni devono esistere tra i riferimenti affinché il programma sia valido.

  Questo permette al borrow checker di garantire che un riferimento non venga utilizzato quando il dato a cui punta non esiste più, evitando quindi i dangling references.

  Quando un valore raggiunge il proprio `drop scope`, Rust esegue il suo distruttore. Se il tipo implementa `Drop`, viene chiamato `Drop::drop`; inoltre il compilatore genera automaticamente la cosiddetta *drop glue* necessaria a distruggere correttamente i valori contenuti.

- È meglio non dire semplicemente che "ogni variabile ha un lifetime". Più precisamente, ogni riferimento ha una lifetime, cioè una regione durante la quale deve essere valido. Si può ragionare informalmente così:
  - valori/variabili normali → hanno un periodo durante il quale esistono e sono accessibili, determinato anche dai loro scope e drop scope;
  - riferimenti → hanno una regione durante la quale devono rimanere validi.

  I lifetime parameters, come `'a`, servono a esprimere genericamente queste relazioni di validità.

  I valori normali non vengono annotati con lifetime parameters come `'a`, ma hanno comunque una regione di esistenza/validità che il compilatore conosce. I riferimenti hanno lifetimes, e il borrow checker verifica che tali lifetimes non oltrepassino la validità dei dati referenziati.

- Il lifetime di un riferimento è la regione del programma durante la quale quel riferimento deve rimanere valido. Con NLL (*Non-Lexical Lifetimes*), questa regione può terminare all'ultimo uso effettivo del riferimento, senza necessariamente arrivare fino alla fine del blocco lessicale.

- Con NLL, Rust non considera automaticamente un borrow attivo fino alla fine del blocco in cui è stato creato. Il borrow checker analizza gli usi effettivi del riferimento e il control flow e mantiene il borrow soltanto nella regione in cui il riferimento deve ancora essere valido. Per esempio:

  ```rust
  let mut data = vec![1, 2, 3];

  let x = &data[0];

  println!("{x}"); // ultimo utilizzo di x

  data.push(4); // OK: il borrow precedente non serve più
  ```

  Anche se la variabile `x` appartiene ancora allo scope, il riferimento contenuto in `x` non viene più utilizzato dopo `println!`, quindi il borrow può terminare prima della fine del blocco.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Si può leggere questa firma in modo intuitivo così:

> Dammi due riferimenti.
>
> Io posso restituire un riferimento proveniente da uno dei due.
>
> Il riferimento restituito sarà considerato valido soltanto per una lifetime `'a` per la quale entrambi gli input possono essere considerati validi.

Attenzione: `'a` **NON significa necessariamente che `x` e `y` abbiano esattamente la stessa lifetime concreta**.

```rust
x: &'a str
y: &'a str
```

significa piuttosto che entrambi i riferimenti devono essere validi almeno per `'a`.

Il compilatore può quindi determinare una `'a` compatibile con entrambi gli input. Intuitivamente, quando i due riferimenti hanno durate diverse, `'a` non può estendersi oltre la regione per la quale entrambi possono essere considerati validi.

Di conseguenza:

> Il risultato non può essere considerato valido più a lungo della lifetime che il compilatore può garantire per entrambi gli input.

Questo è necessario perché la funzione potrebbe restituire `x` oppure `y`, e chi la chiama non può sapere quale dei due riferimenti verrà restituito.


## The Slice Type

- La definizione generale del tipo slice non è `&str` o `&[T]`, come sembra di capire dal capitolo corrispondente del libro: quelli sono i *riferimenti* a uno slice. I tipi slice veri e propri sono `[T]` e `str`. `str` è un po' più speciale, ha una forma sua: in memoria è rappresentato esattamente come uno slice di `u8`, cioè come `[u8]`, ma con la restrizione aggiuntiva di contenere sempre UTF-8 valido. Non è però *lo stesso* tipo di `[u8]`: per passare dall'uno all'altro servono `as_bytes()` (che riesce sempre) e `str::from_utf8()` (che invece può fallire, perché deve verificare l'invariante).

- Si usa quindi quasi sempre il riferimento allo slice, perché lo slice in sé non può essere usato dove è richiesto un tipo `Sized`. Per esempio `let`, per la definizione di una variabile, richiede un tipo `Sized`, perché il compilatore deve sapere quanto spazio allocare per la variabile sullo stack; ma lo slice è un Dynamically Sized Type (DST), e il compilatore non sa di quanto spazio ha bisogno in fase di compilazione. Lo stesso vale per gli argomenti e per i valori di ritorno delle funzioni: per default hanno un vincolo `Sized` implicito. Quindi questo è illegale:

  ```rust
  let s: [i32] = [1, 2, 3];
  ```

  Mentre questo è legale:

  ```rust
  let s: &[i32] = &[1, 2, 3];
  ```

  perché il riferimento allo slice è un tipo `Sized`, e la variabile riceve come valore il puntatore allo slice (più la sua lunghezza). Da notare che `&[1, 2, 3]` ha in realtà tipo `&[i32; 3]`, cioè un riferimento a un array di dimensione nota, e viene convertito in `&[i32]` tramite *unsized coercion*.

- **Attenzione:** `&[T]` è `Sized`, ma la sua dimensione non è quella di un puntatore normale, è il doppio: puntatore + lunghezza, cioè 16 byte su una piattaforma a 64 bit. Si può verificare con `std::mem::size_of::<&[i32]>()`. È `Sized` perché la sua dimensione è nota staticamente, non perché sia grande quanto un puntatore.

- **Fat pointer:** il puntatore che contiene anche informazioni aggiuntive, per esempio l'informazione della lunghezza dei dati a cui punta. `&str` e `&[T]` sono per esempio dei fat pointer: puntano direttamente ai dati, che sono di un tipo dinamicamente dimensionato, quindi il puntatore da solo non basterebbe e deve portarsi dietro anche la lunghezza. Invece `&String` è un puntatore normale (thin pointer), perché le informazioni necessarie (puntatore ai dati, lunghezza, capacità) sono già contenute nel valore di tipo `String`: il puntatore si occupa semplicemente di puntare all'indirizzo di quel valore.

- Due precisazioni sui fat pointer:
  - L'informazione aggiuntiva non è sempre la lunghezza. In `&dyn Trait` la seconda parola è il puntatore alla vtable, cioè alla tabella dei metodi da chiamare.
  - Non sono solo i riferimenti a essere fat pointer: lo sono anche `Box<[T]>`, `Rc<str>` e in generale qualsiasi puntatore verso un DST.