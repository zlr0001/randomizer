import init, { toggle, set_interval } from "./pkg/randomize.js";

init().then(() => {
  console.log("WASM initialisé");
  let enabled = false;

  const btn = document.getElementById("toggle");
  const minInput = document.getElementById("min");
  const maxInput = document.getElementById("max");
  const countdown = document.getElementById("countdown");

  function applyInterval() {
    const min = parseInt(minInput.value);
    const max = parseInt(maxInput.value);
    if (min > 0 && max >= min) {
      set_interval(min, max);
    }
  }

  minInput.addEventListener("change", applyInterval);
  maxInput.addEventListener("change", applyInterval);

  btn.onclick = () => {
    enabled = !enabled;
    toggle(enabled);

    if (enabled) {
      btn.textContent = "Désactiver";
      btn.classList.remove("is-success");
      btn.classList.add("is-danger");
      applyInterval();
      document.getElementById("status").textContent =
        "Rafraîchissement activé";
    } else {
      btn.textContent = "Activer";
      btn.classList.remove("is-danger");
      btn.classList.add("is-success");
      document.getElementById("status").textContent =
        "Rafraîchissement désactivé";
      countdown.textContent = "--:--";
    }
  };

  // Écoute les ticks envoyés par background.js
  chrome.runtime.onMessage.addListener((msg) => {
    if (msg.action === "tick") {
      const totalSec = Math.floor(msg.remainingMs / 1000);
      const min = String(Math.floor(totalSec / 60)).padStart(2, "0");
      const sec = String(totalSec % 60).padStart(2, "0");
      countdown.textContent = `${min}:${sec}`;
    }
  });
});
