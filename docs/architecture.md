# GridFlow Architecture

## Overview
GridFlow est un système de calcul distribué basé sur WASM...

## Components
- **Scheduler**: Orchestrateur principal
- **Contracts**: Types de données partagés
- **Network**: Communication réseau
- **Worker Runtime**: Exécution WASM
- **DevTools**: Outils de développement

## Data Flow
1. Soumission de tâche → Scheduler
2. Distribution → Workers
3. Exécution WASM
4. Aggregation des résultats
