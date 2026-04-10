# Client TCP Echo en Rust : version `std` vs Tokio

Ce projet contient deux clients TCP qui se connectent à un serveur echo sur `localhost:1234` et envoient le message `"Hello World! "`.

## 1. `echo-client-std` (version synchrone)

- Utilise le module réseau standard de Rust :  
  - `use std::io::prelude::*;`  
  - `use std::net::TcpStream;`
- Le code est **bloquant** :  
  - `TcpStream::connect(...)` bloque le thread jusqu’à la connexion.  
  - `stream.write(...)` et `stream.read(...)` bloquent jusqu’à la fin de l’opération.
- Adapté pour des petits scripts simples, mais pas efficace avec beaucoup de connexions (car chaque connexion bloque un thread).

## 2. `echo-client-tokio` (version asynchrone)

- Utilise la bibliothèque Tokio pour le réseau asynchrone :  
  - `use tokio::io::{AsyncReadExt, AsyncWriteExt};`  
  - `use tokio::net::TcpStream;`
- Le code est **non‑bloquant** :  
  - `TcpStream::connect(...).await` attend la connexion sans bloquer le thread.  
  - `stream.write_all(...).await` et `stream.read(...).await` suspendent la tâche le temps que le réseau réponde.
- Le runtime Tokio (`#[tokio::main]`) gère une boucle d’événements et permet de gérer **beaucoup de connexions simultanées** sur peu de threads.

## 3. `async` / `await` en Rust (Tokio)

- `async fn` définit une fonction qui retourne une `Future` (une valeur disponible “plus tard”).  
- `await` attend la résolution de cette `Future` sans bloquer le thread : la tâche est mise en pause, et le runtime peut exécuter autre chose.  
- Tokio est un **runtime asynchrone** qui gère :  
  - la boucle d’événements (réveille les tâches quand le réseau est prêt),  
  - le scheduler des tâches asynchrones.

## 4. À retenir

- `std` : simple, bloquant, adapté aux petits cas.  
- Tokio : plus efficace pour du réseau I/O‑bound, idéal pour clients/serveurs qui gèrent beaucoup de connexions concurrentes.
