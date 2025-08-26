# Randomize Auto Refresh

Pour **Brave** / **Chrome**, elle actualise automatiquement l’onglet actif à un **intervalle aléatoire** défini par l’utilisateur.

---

## Installation (Brave / Chrome)

1. **Télécharger**
   - Copie les fichiers nécessaires (`manifest.json`, `background.js`, `index.html`, `popup.js`, le dossier `pkg/`, et `icons/`) dans un dossier `randomizer/`.

2. **Ouvre la page des extensions**
   - Dans Chrome : `chrome://extensions/`
   - Dans Brave : `brave://extensions/`

3. **Active le mode développeur**
   - En haut à droite, active le bouton **Mode développeur**.

4. **Charge l’extension**
   - Clique sur **Charger l’extension non empaquetée**.  
   - Sélectionne le dossier `randomizer/`.

5. L’extension apparaît dans la barre d’outils

---

## Utilisation

1. Clique sur l’icône **Randomize Auto Refresh** dans la barre d’outils.  
2. Dans le popup :
   - **Min (minutes)** : borne minimale de l’intervalle.  
   - **Max (minutes)** : borne maximale de l’intervalle.  
   - **Bouton Activer/Désactiver** : démarre ou arrête le rafraîchissement.  
3. Une fois activé :
   - L’extension choisit un intervalle aléatoire entre `min` et `max`.  
   - Un **compte à rebours** s’affiche dans le popup.  
   - À la fin du compte à rebours, l’onglet actif est actualisé.  
   - Un nouvel intervalle aléatoire est choisi automatiquement.  
4. Tu peux fermer le popup : le rafraîchissement continue en arrière-plan.  
5. Clique à nouveau sur le bouton pour **désactiver** le rafraîchissement.

---

## Notes

- L’extension fonctionne uniquement sur les **onglets normaux** (pas sur les pages système comme `chrome://` ou `brave://`).  
- Les permissions utilisées :
  - `tabs` → pour identifier l’onglet actif.  
  - `scripting` → pour exécuter `window.location.reload()` dans l’onglet.  
- Le timer continue même si le popup est fermé.  

---
