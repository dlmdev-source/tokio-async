# Client TCP Echo en Rust : version `std` vs Tokio

Ce projet contient deux clients TCP qui se connectent à un serveur echo sur `localhost:1234` et envoient le message `"Hello World!"`.

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

## 4. `sirocco` : serveur TCP basique

- Le dossier `sirocco` contient un petit serveur TCP simple basé sur le module réseau standard de Rust.  
  - Utilise `std::net::TcpListener` pour écouter les connexions entrantes.
- Le serveur démarre sur l’adresse `127.0.0.1:5678` et affiche un message pour chaque nouvelle connexion :
  ```rust
  sirocco starting 127.0.0.1:5678
  sirocco listening 127.0.0.1:5678
  connection established!
  ```
- Exemple minimal de code :
  ```rust
  use std::net::TcpListener;

  const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:5678";

  fn main() {
      println!("sirocco starting {}", SIROCCO_SERVER_ADDRESS);

      let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS).unwrap();

      println!("sirocco listening {}", SIROCCO_SERVER_ADDRESS);

      for stream in listener.incoming() {
          let _stream = stream.unwrap();
          println!("connection established!");
      }
  }
  ```
- Ce serveur est **bloquant**, comme la version `std` du client echo. Il est utile pour tester des connexions TCP locales ou expérimenter la création d’un serveur basique avant de passer à Tokio.

## 5. À retenir

- `std` : simple, bloquant, adapté aux petits cas.  
- Tokio : plus efficace pour du réseau I/O‑bound, idéal pour clients/serveurs qui gèrent beaucoup de connexions concurrentes.  
- `sirocco` : exemple de **serveur TCP minimaliste** basé sur `std`, idéal pour tester la connectivité de base avant d’utiliser la logique asynchrone de Tokio.
