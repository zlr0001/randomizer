let enabled = false;
let min = 1;
let max = 3;
let timer = null;
let tickInterval = null;
let nextReloadTime = null;

function startTimer() {
  if (timer) clearTimeout(timer);
  if (tickInterval) clearInterval(tickInterval);
  if (!enabled) return;

  const randomInterval =
    Math.floor(Math.random() * (max - min + 1) + min) * 60000;

  nextReloadTime = Date.now() + randomInterval;

  console.log(
    `Prochain rafraîchissement dans ${randomInterval / 60000} minutes (min=${min}, max=${max})`
  );

  // Envoie un "tick" chaque seconde au popup
  tickInterval = setInterval(() => {
    if (!nextReloadTime) return;
    const remaining = Math.max(0, nextReloadTime - Date.now());
    chrome.runtime.sendMessage({
      action: "tick",
      remainingMs: remaining,
    });
  }, 1000);

  timer = setTimeout(() => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      if (tabs[0]?.id) {
        console.log("Rechargement de l'onglet :", tabs[0].id);
        chrome.scripting.executeScript({
          target: { tabId: tabs[0].id },
          func: () => window.location.reload(),
        });
      }
    });
    startTimer(); // relance le cycle
  }, randomInterval);
}

chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  console.log("Message reçu :", msg);

  if (msg.action === "toggle") {
    enabled = msg.enabled;
    console.log(`Rafraîchissement ${enabled ? "activé" : "désactivé"}`);
    if (enabled) startTimer();
    else {
      if (timer) clearTimeout(timer);
      if (tickInterval) clearInterval(tickInterval);
      nextReloadTime = null;
    }
  }

  if (msg.action === "set_interval") {
    min = msg.min;
    max = msg.max;
    console.log(`Intervalle mis à jour : min=${min}, max=${max}`);
    if (enabled) startTimer();
  }
});
