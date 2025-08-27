#!/bin/bash

# Script de déploiement pour osu.osef.me-backend
# Compile en release et transfère vers le serveur distant

set -e  # Arrêter le script en cas d'erreur

echo "🚀 Début du déploiement..."

# Variables
REMOTE_HOST="debian@51.38.239.149"
REMOTE_DIR="~/backend/"
BINARY_NAME="osu-backend"  # Nom du binaire basé sur le nom du projet

echo "📦 Compilation en mode release..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilation réussie!"
else
    echo "❌ Erreur lors de la compilation"
    exit 1
fi

echo "📤 Transfert du fichier vers $REMOTE_HOST:$REMOTE_DIR"

# Créer le répertoire distant s'il n'existe pas
ssh $REMOTE_HOST "mkdir -p $REMOTE_DIR"

# Transférer le binaire
scp target/release/$BINARY_NAME $REMOTE_HOST:$REMOTE_DIR

if [ $? -eq 0 ]; then
    echo "✅ Transfert réussi!"
    
    # Rendre le fichier exécutable sur le serveur distant
    ssh $REMOTE_HOST "chmod +x $REMOTE_DIR$BINARY_NAME"
    
    echo "🎉 Déploiement terminé avec succès!"
    echo "📁 Fichier déployé: $REMOTE_HOST:$REMOTE_DIR$BINARY_NAME"
else
    echo "❌ Erreur lors du transfert"
    exit 1
fi
