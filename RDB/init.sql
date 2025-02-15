-- Suppression des tables si elles existent (pour la réinitialisation)
DROP TABLE IF EXISTS emprunts;
DROP TABLE IF EXISTS exemplaires;
DROP TABLE IF EXISTS livres;
DROP TABLE IF EXISTS utilisateurs;
DROP TABLE IF EXISTS auteurs;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS editeurs;

-- Création des tables
CREATE TABLE auteurs (
    auteur_id SERIAL PRIMARY KEY,
    nom VARCHAR(100) NOT NULL,
    nationalite VARCHAR(50)
);

CREATE TABLE categories (
    categorie_id SERIAL PRIMARY KEY,
    nom VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE editeurs (
    editeur_id SERIAL PRIMARY KEY,
    nom VARCHAR(100) NOT NULL,
    adresse VARCHAR(150)
);

CREATE TABLE livres (
    livre_id SERIAL PRIMARY KEY,
    titre VARCHAR(150) NOT NULL,
    auteur_id INT NOT NULL,
    date_publication DATE,
    isbn VARCHAR(13) UNIQUE,
    categorie_id INT NOT NULL,
    editeur_id INT NOT NULL,
    FOREIGN KEY (auteur_id) REFERENCES auteurs(auteur_id),
    FOREIGN KEY (categorie_id) REFERENCES categories(categorie_id),
    FOREIGN KEY (editeur_id) REFERENCES editeurs(editeur_id)
);

CREATE TABLE exemplaires (
    exemplaire_id SERIAL PRIMARY KEY,
    livre_id INT NOT NULL,
    statut VARCHAR(20) CHECK (statut IN ('disponible', 'emprunté', 'en réparation')),
    FOREIGN KEY (livre_id) REFERENCES livres(livre_id)
);

CREATE TABLE utilisateurs (
    utilisateur_id SERIAL PRIMARY KEY,
    prenom VARCHAR(50) NOT NULL,
    nom VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    date_inscription DATE DEFAULT CURRENT_DATE
);

CREATE TABLE emprunts (
    emprunt_id SERIAL PRIMARY KEY,
    utilisateur_id INT NOT NULL,
    exemplaire_id INT NOT NULL,
    date_emprunt DATE NOT NULL DEFAULT CURRENT_DATE,
    date_retour DATE,
    FOREIGN KEY (utilisateur_id) REFERENCES utilisateurs(utilisateur_id),
    FOREIGN KEY (exemplaire_id) REFERENCES exemplaires(exemplaire_id),
    CHECK (date_retour IS NULL OR date_retour > date_emprunt)
);