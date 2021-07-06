# Bachelor_prototyping
Web Content Rendering at the Edge

Edge platform: fastly
Programming language: rust
Runtime: webassembly
CLI: https://developer.fastly.com/reference/cli/
Starter: https://developer.fastly.com/solutions/starters/compute-starter-kit-rust-default/
Deployment: https://developer.fastly.com/learning/compute/rust/

## TODO:

* [X] Acheter un nom de domaine
* [X] Limite de budget cloud CHF 200.-
* [X] Configurer le CLI fastly en local
* [X] Créer un service de test
* [X] Partager les accès à ce service
* [X] Installer un environnement de développement rust
* [X] Installer le plugin rust IntelliJ (license education avec email heig-vd)
* [ ] Déployer le rust starter kit
* [ ] Adapter le rust starter kit au scenario d'aggrégation de contenu.

Cloudflare
* [X] Acheter un nom de domaine
* [X] Limite de budget cloud CHF 200.-
* [X] Configurer le CLI Cloudflare en local
* [X] Créer un service de test
* [ ] Partager les accès à ce service
* [X] Installer un environnement de développement rust
* [X] Installer le plugin rust IntelliJ (license education avec email heig-vd)
* [X] Déployer le rust starter kit
* [ ] Adapter le rust starter kit au scenario d'aggrégation de contenu.

## Content aggregation

L'abstraction fournie par Fastly pour les requêtes et les Réponses HTTP est la suivante:

```
Request -> Result
```

En partant d'une requête HTTP contenant un chemin (`/path`) et des paramètres GET (`?content=a+b+c`) telle que décrite ci-dessous, l'objectif est de créer une programme de concatenation simple.

```
GET /path/?concatenation=a+b+c
```

Ici, le chemin correspond à un dossier disponible sur le cloud contenant des fichiers textes.

```
/path/
/path/a
/path/b
/path/c
```

Une requête sur le dossier `/path/`avec les paramêtres `concatenation=a+b+c` doit permettre de concactener le contenu des fichiers textes `a`, `b` et `c` stockés dans ce dossier.

Par exemple, si chacun des fichiers a son nom pour contenu, la requête GET décrite ci-dessus devrait retourner une réponse contenant la chaine de caractère `abc`.
